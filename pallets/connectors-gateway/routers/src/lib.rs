// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
#![cfg_attr(not(feature = "std"), no_std)]

use cfg_traits::{connectors::Router, ethereum::EthereumTransactor};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::{DispatchError, DispatchResult, Weight},
	ensure,
	traits::OriginTrait,
};
use pallet_xcm_transactor::{Currency, CurrencyPayment, TransactWeights};
use scale_info::TypeInfo;
use sp_core::{bounded::BoundedVec, ConstU32, H160, H256, U256};
use sp_runtime::traits::{BlakeTwo256, Hash};
use sp_std::{boxed::Box, marker::PhantomData, vec::Vec};
use xcm::{
	latest::{MultiLocation, OriginKind},
	VersionedMultiLocation,
};

use crate::{axelar_evm::AxelarEVMRouter, ethereum_xcm::EthereumXCMRouter};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod routers;

pub use routers::*;

type CurrencyIdOf<T> = <T as pallet_xcm_transactor::Config>::CurrencyId;
type MessageOf<T> = <T as pallet_connectors_gateway::Config>::Message;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

const CONNECTORS_FUNCTION_NAME: &str = "handle";
const CONNECTORS_MESSAGE_PARAM: &str = "message";

const AXELAR_FUNCTION_NAME: &str = "callContract";
const AXELAR_DESTINATION_CHAIN_PARAM: &str = "destinationChain";
const AXELAR_DESTINATION_CONTRACT_ADDRESS_PARAM: &str = "destinationContractAddress";
const AXELAR_PAYLOAD_PARAM: &str = "payload";

/// The routers used for outgoing messages.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum DomainRouter<T>
where
	T: frame_system::Config
		+ pallet_xcm_transactor::Config
		+ pallet_connectors_gateway::Config
		+ pallet_ethereum_transaction::Config
		+ pallet_evm::Config,
	T::AccountId: AsRef<[u8; 32]>,
{
	EthereumXCM(EthereumXCMRouter<T>),
	AxelarEVM(AxelarEVMRouter<T>),
	AxelarXCM(AxelarXCMRouter<T>),
}

impl<T> Router for DomainRouter<T>
where
	T: frame_system::Config
		+ pallet_xcm_transactor::Config
		+ pallet_connectors_gateway::Config
		+ pallet_ethereum_transaction::Config
		+ pallet_evm::Config,
	T::AccountId: AsRef<[u8; 32]>,
{
	type Message = MessageOf<T>;
	type Sender = AccountIdOf<T>;

	fn init(&self) -> DispatchResult {
		match self {
			DomainRouter::EthereumXCM(r) => r.do_init(),
			DomainRouter::AxelarEVM(r) => r.do_init(),
			DomainRouter::AxelarXCM(r) => r.do_init(),
		}
	}

	fn send(&self, sender: Self::Sender, message: Self::Message) -> DispatchResult {
		match self {
			DomainRouter::EthereumXCM(r) => r.do_send(sender, message),
			DomainRouter::AxelarEVM(r) => r.do_send(sender, message),
			DomainRouter::AxelarXCM(r) => r.do_send(sender, message),
		}
	}
}

/// A generic router used for executing EVM calls.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct EVMRouter<T>
where
	T: frame_system::Config + pallet_ethereum_transaction::Config + pallet_evm::Config,
{
	pub evm_domain: EVMDomain,
	pub _marker: PhantomData<T>,
}

impl<T> EVMRouter<T>
where
	T: frame_system::Config + pallet_ethereum_transaction::Config + pallet_evm::Config,
	T::AccountId: AsRef<[u8; 32]>,
{
	/// Performs an extra check to ensure that the actual contract is deployed
	/// at the provided address and that the contract code hash matches.
	pub fn do_init(&self) -> DispatchResult {
		let code = pallet_evm::AccountCodes::<T>::get(self.evm_domain.target_contract_address);

		ensure!(
			BlakeTwo256::hash_of(&code) == self.evm_domain.target_contract_hash,
			DispatchError::Other("Target contract code does not match"),
		);

		Ok(())
	}

	/// NOTE - the sender account ID provided here will be converted to an EVM
	/// address via truncating. When the call is processed by the underlying EVM
	/// pallet, this EVM address will be converted back into a substrate account
	/// which will be charged for the transaction. This converted substrate
	/// account is not the same as the original account.
	pub fn do_send(&self, sender: T::AccountId, msg: Vec<u8>) -> DispatchResult {
		let sender_evm_address = H160::from_slice(&sender.as_ref()[0..20]);

		// TODO(cdamian): This returns a `DispatchResultWithPostInfo`. Should we
		// propagate that to another layer that will eventually charge for the
		// weight in the PostDispatchInfo?
		<pallet_ethereum_transaction::Pallet<T> as EthereumTransactor>::call(
			sender_evm_address,
			self.evm_domain.target_contract_address,
			msg.as_slice(),
			self.evm_domain.fee_values.value,
			self.evm_domain.fee_values.gas_price,
			self.evm_domain.fee_values.gas_limit,
		)
		.map_err(|e| e.error)?;

		Ok(())
	}
}

/// The EVMDomain holds all relevant information for validating and executing
/// the EVM call.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct EVMDomain {
	/// The address of the contract deployed in our EVM.
	pub target_contract_address: H160,

	/// The `BlakeTwo256` hash of the target contract code.
	///
	/// This is used during router initialization to ensure that the correct
	/// contract code is used.
	pub target_contract_hash: H256,

	/// The values used when executing the EVM call.
	pub fee_values: FeeValues,
}

/// The FeeValues holds all information related to the transaction costs.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct FeeValues {
	/// The value used when executing the EVM call.
	pub value: U256,

	/// The gas price used when executing the EVM call.
	pub gas_price: U256,

	/// The gas limit used when executing the EVM call.
	pub gas_limit: U256,
}

/// A generic router used for executing XCM calls.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct XCMRouter<T>
where
	T: frame_system::Config + pallet_xcm_transactor::Config,
{
	pub xcm_domain: XcmDomain<T::CurrencyId>,
	pub _marker: PhantomData<T>,
}

impl<T> XCMRouter<T>
where
	T: frame_system::Config + pallet_xcm_transactor::Config,
{
	/// Sets the weight information for the provided XCM domain location, and
	/// the fee per second for the provided fee asset location.
	pub fn do_init(&self) -> DispatchResult {
		pallet_xcm_transactor::Pallet::<T>::set_transact_info(
			<T as frame_system::Config>::RuntimeOrigin::root(),
			self.xcm_domain.location.clone(),
			self.xcm_domain.transact_info.transact_extra_weight,
			self.xcm_domain.transact_info.max_weight,
			self.xcm_domain.transact_info.transact_extra_weight_signed,
		)?;

		pallet_xcm_transactor::Pallet::<T>::set_fee_per_second(
			<T as frame_system::Config>::RuntimeOrigin::root(),
			self.xcm_domain.fee_asset_location.clone(),
			self.xcm_domain.fee_per_second,
		)
	}

	/// Encodes the Connectors message to the required format and executes the
	/// call via the XCM transactor pallet.
	pub fn do_send(&self, sender: T::AccountId, msg: Vec<u8>) -> DispatchResult {
		let ethereum_xcm_call = get_encoded_ethereum_xcm_call::<T>(self.xcm_domain.clone(), msg)
			.map_err(|_| DispatchError::Other("encoded ethereum xcm call retrieval"))?;

		pallet_xcm_transactor::Pallet::<T>::transact_through_sovereign(
			<T as frame_system::Config>::RuntimeOrigin::root(),
			// The destination to which the message should be sent.
			self.xcm_domain.location.clone(),
			// The sender will pay for this transaction.
			sender,
			// The currency in which we want to pay fees.
			CurrencyPayment {
				currency: Currency::AsCurrencyId(self.xcm_domain.fee_currency.clone()),
				fee_amount: None,
			},
			// The call to be executed in the destination chain.
			ethereum_xcm_call,
			OriginKind::SovereignAccount,
			TransactWeights {
				// Convert the max gas_limit into a max transact weight following
				// Moonbeam's formula.
				transact_required_weight_at_most: Weight::from_all(
					self.xcm_domain.max_gas_limit * 25_000 + 100_000_000,
				),
				overall_weight: None,
			},
		)?;

		Ok(())
	}
}

pub(crate) fn get_encoded_ethereum_xcm_call<T>(
	xcm_domain: XcmDomain<T::CurrencyId>,
	msg: Vec<u8>,
) -> Result<Vec<u8>, ()>
where
	T: frame_system::Config + pallet_xcm_transactor::Config,
{
	let input =
		BoundedVec::<u8, ConstU32<{ xcm_primitives::MAX_ETHEREUM_XCM_INPUT_SIZE }>>::try_from(msg)
			.map_err(|_| ())?;

	let mut encoded: Vec<u8> = Vec::new();

	encoded.append(&mut xcm_domain.ethereum_xcm_transact_call_index.into_inner());

	encoded.append(
		&mut xcm_primitives::EthereumXcmTransaction::V1(xcm_primitives::EthereumXcmTransactionV1 {
			gas_limit: U256::from(xcm_domain.max_gas_limit),
			fee_payment: xcm_primitives::EthereumXcmFee::Auto,
			action: pallet_ethereum::TransactionAction::Call(xcm_domain.contract_address),
			value: U256::zero(),
			input,
			access_list: None,
		})
		.encode(),
	);

	Ok(encoded)
}

/// XcmDomain gathers all the required fields to build and send remote
/// calls to a specific XCM-based Domain.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct XcmDomain<CurrencyId> {
	/// The XCM multilocation of the domain.
	pub location: Box<VersionedMultiLocation>,

	/// The ethereum_xcm::Call::transact call index on a given domain.
	/// It should contain the pallet index + the `transact` call index, to which
	/// we will append the eth_tx param.
	///
	/// You can obtain this value by building an ethereum_xcm::transact call
	/// with Polkadot JS on the target chain.
	pub ethereum_xcm_transact_call_index:
		BoundedVec<u8, ConstU32<{ xcm_primitives::MAX_ETHEREUM_XCM_INPUT_SIZE }>>,

	/// The target contract address on a given domain.
	pub contract_address: H160,

	/// The max gas_limit we want to propose for a remote evm execution
	pub max_gas_limit: u64,

	/// The XCM transact info that will be stored in the
	/// `TransactInfoWithWeightLimit` storage of the XCM transactor pallet.
	pub transact_info: XcmTransactInfo,

	/// The currency in which execution fees will be paid on
	pub fee_currency: CurrencyId,

	/// The fee per second that will be stored in the
	/// `DestinationAssetFeePerSecond` storage of the XCM transactor pallet.
	pub fee_per_second: u128,

	/// The location of the asset used for paying XCM fees.
	pub fee_asset_location: Box<VersionedMultiLocation>,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
/// XcmTransactInfo hold all the weight related information required for the XCM
/// transactor pallet.
pub struct XcmTransactInfo {
	pub transact_extra_weight: Weight,
	pub max_weight: Weight,
	pub transact_extra_weight_signed: Option<Weight>,
}

/// NOTE: Remove this custom implementation once the following underlying data
/// implements MaxEncodedLen:
/// * Polkadot Repo: xcm::VersionedMultiLocation
/// * PureStake Repo: pallet_xcm_transactor::Config<Self = T>::CurrencyId
impl<CurrencyId> MaxEncodedLen for XcmDomain<CurrencyId>
where
	XcmDomain<CurrencyId>: Encode,
{
	fn max_encoded_len() -> usize {
		// The domain's `VersionedMultiLocation` (custom bound)
		MultiLocation::max_encoded_len()
			// From the enum wrapping of `VersionedMultiLocation` for the XCM domain location.
			.saturating_add(1)
			// From the enum wrapping of `VersionedMultiLocation` for the asset fee location.
			.saturating_add(1)
			// The ethereum xcm call index (default bound)
			.saturating_add(BoundedVec::<
				u8,
				ConstU32<{ xcm_primitives::MAX_ETHEREUM_XCM_INPUT_SIZE }>,
			>::max_encoded_len())
			// The contract address (default bound)
			.saturating_add(H160::max_encoded_len())
			// The fee currency (custom bound)
			.saturating_add(cfg_types::tokens::CurrencyId::max_encoded_len())
			// The XcmTransactInfo
			.saturating_add(XcmTransactInfo::max_encoded_len())
	}
}