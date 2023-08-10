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
use cfg_traits::connectors::Codec;
use codec::{Decode, Encode, MaxEncodedLen};
use ethabi::{Contract, Function, Param, ParamType, Token};
use frame_support::dispatch::{DispatchError, DispatchResult};
use scale_info::{
	prelude::string::{String, ToString},
	TypeInfo,
};
use sp_core::H160;
use sp_std::{collections::btree_map::BTreeMap, marker::PhantomData, vec, vec::Vec};

use crate::{
	AccountIdOf, EVMRouter, MessageOf, AXELAR_DESTINATION_CHAIN_PARAM,
	AXELAR_DESTINATION_CONTRACT_ADDRESS_PARAM, AXELAR_FUNCTION_NAME, AXELAR_PAYLOAD_PARAM,
	CONNECTORS_FUNCTION_NAME, CONNECTORS_MESSAGE_PARAM,
};

/// EVMChain holds all supported EVM chains.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum EVMChain {
	Ethereum,
	Goerli,
}

/// Required due to the naming convention defined by Axelar here:
/// <https://docs.axelar.dev/dev/reference/mainnet-chain-names>
impl ToString for EVMChain {
	fn to_string(&self) -> String {
		match self {
			EVMChain::Ethereum => "Ethereum".to_string(),
			EVMChain::Goerli => "ethereum-2".to_string(),
		}
	}
}

/// The router used for executing the Connectors contract via Axelar.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct AxelarEVMRouter<T>
where
	T: frame_system::Config
		+ pallet_connectors_gateway::Config
		+ pallet_ethereum_transaction::Config
		+ pallet_evm::Config,
{
	pub router: EVMRouter<T>,
	pub evm_chain: EVMChain,
	pub connectors_contract_address: H160,
	pub _marker: PhantomData<T>,
}

impl<T> AxelarEVMRouter<T>
where
	T: frame_system::Config
		+ pallet_connectors_gateway::Config
		+ pallet_ethereum_transaction::Config
		+ pallet_evm::Config,
	T::AccountId: AsRef<[u8; 32]>,
{
	/// Calls the init function on the EVM router.
	pub fn do_init(&self) -> DispatchResult {
		self.router.do_init()
	}

	/// Encodes the Connectors message to the required format,
	/// then executes the EVM call using the generic EVM router.
	pub fn do_send(&self, sender: AccountIdOf<T>, msg: MessageOf<T>) -> DispatchResult {
		let eth_msg = get_axelar_encoded_msg(
			msg.serialize(),
			self.evm_chain.clone(),
			self.connectors_contract_address,
		)
		.map_err(DispatchError::Other)?;

		self.router.do_send(sender, eth_msg)
	}
}

/// Encodes the provided message into the format required for submitting it
/// to the Axelar contract which in turn submits it to the Connectors
/// contract.
///
/// Axelar contract call:
/// <https://github.com/axelarnetwork/axelar-cgp-solidity/blob/v4.3.2/contracts/AxelarGateway.sol#L78>
///
/// Connectors contract call:
/// <https://github.com/centrifuge/connectors/blob/383d279f809a01ab979faf45f31bf9dc3ce6a74a/src/routers/Gateway.sol#L276>
pub(crate) fn get_axelar_encoded_msg(
	serialized_msg: Vec<u8>,
	target_chain: EVMChain,
	target_contract: H160,
) -> Result<Vec<u8>, &'static str> {
	#[allow(deprecated)]
	let encoded_connectors_contract = Contract {
		constructor: None,
		functions: BTreeMap::<String, Vec<Function>>::from([(
			CONNECTORS_FUNCTION_NAME.to_string(),
			vec![Function {
				name: CONNECTORS_FUNCTION_NAME.into(),
				inputs: vec![Param {
					name: CONNECTORS_MESSAGE_PARAM.into(),
					kind: ParamType::Bytes,
					internal_type: None,
				}],
				outputs: vec![],
				constant: false,
				state_mutability: Default::default(),
			}],
		)]),
		events: Default::default(),
		errors: Default::default(),
		receive: false,
		fallback: false,
	}
	.function(CONNECTORS_FUNCTION_NAME)
	.map_err(|_| "cannot retrieve Connectors contract function")?
	.encode_input(&[Token::Bytes(serialized_msg)])
	.map_err(|_| "cannot encode input for Connectors contract function")?;

	#[allow(deprecated)]
	let encoded_axelar_contract = Contract {
		constructor: None,
		functions: BTreeMap::<String, Vec<Function>>::from([(
			AXELAR_FUNCTION_NAME.into(),
			vec![Function {
				name: AXELAR_FUNCTION_NAME.into(),
				inputs: vec![
					Param {
						name: AXELAR_DESTINATION_CHAIN_PARAM.into(),
						kind: ParamType::String,
						internal_type: None,
					},
					Param {
						name: AXELAR_DESTINATION_CONTRACT_ADDRESS_PARAM.into(),
						kind: ParamType::String,
						internal_type: None,
					},
					Param {
						name: AXELAR_PAYLOAD_PARAM.into(),
						kind: ParamType::Bytes,
						internal_type: None,
					},
				],
				outputs: vec![],
				constant: false,
				state_mutability: Default::default(),
			}],
		)]),
		events: Default::default(),
		errors: Default::default(),
		receive: false,
		fallback: false,
	}
	.function(AXELAR_FUNCTION_NAME)
	.map_err(|_| "cannot retrieve Axelar contract function")?
	.encode_input(&[
		Token::String(target_chain.to_string()),
		Token::String(target_contract.to_string()),
		Token::Bytes(encoded_connectors_contract),
	])
	.map_err(|_| "cannot encode input for Axelar contract function")?;

	Ok(encoded_axelar_contract)
}