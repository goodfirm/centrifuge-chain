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

use cfg_primitives::OrderId;
use frame_support::pallet_prelude::RuntimeDebug;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{EnsureAddAssign, Zero},
	DispatchResult,
};
use sp_std::cmp::PartialEq;

use crate::orders::Order;

/// A representation of a investment identifier that can be converted to an
/// account address
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct InvestmentAccount<InvestmentId> {
	pub investment_id: InvestmentId,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct InvestmentInfo<AccountId, Currency, InvestmentId> {
	pub owner: AccountId,
	pub id: InvestmentId,
	pub payment_currency: Currency,
}

/// The outstanding collections for an account
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct InvestCollection<Balance> {
	/// This is the payout in the denomination currency
	/// of an investment
	/// * If investment: In payment currency
	/// * If payout: In denomination currency
	pub payout_investment_invest: Balance,

	/// This is the remaining investment in the payment currency
	/// of an investment
	/// * If investment: In payment currency
	/// * If payout: In denomination currency
	pub remaining_investment_invest: Balance,
}

impl<Balance: Zero> Default for InvestCollection<Balance> {
	fn default() -> Self {
		InvestCollection {
			payout_investment_invest: Zero::zero(),
			remaining_investment_invest: Zero::zero(),
		}
	}
}

impl<Balance: Zero + Copy> InvestCollection<Balance> {
	/// Create a `InvestCollection` directly from an active invest order of
	/// a user.
	/// The field `remaining_investment_invest` is set to the
	/// amount of the active invest order of the user and will
	/// be subtracted from upon given fulfillment's
	pub fn from_order(order: &Order<Balance, OrderId>) -> Self {
		InvestCollection {
			payout_investment_invest: Zero::zero(),
			remaining_investment_invest: order.amount(),
		}
	}
}

/// The outstanding collections for an account
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct RedeemCollection<Balance> {
	/// This is the payout in the payment currency
	/// of an investment
	/// * If redemption: In denomination currency
	/// * If payout: In payment currency
	pub payout_investment_redeem: Balance,

	/// This is the remaining redemption in the denomination currency
	/// of an investment
	/// * If redemption: In denomination currency
	/// * If payout: In payment currency
	pub remaining_investment_redeem: Balance,
}

impl<Balance: Zero> Default for RedeemCollection<Balance> {
	fn default() -> Self {
		RedeemCollection {
			payout_investment_redeem: Zero::zero(),
			remaining_investment_redeem: Zero::zero(),
		}
	}
}

impl<Balance: Zero + Copy> RedeemCollection<Balance> {
	/// Create a `RedeemCollection` directly from an active redeem order of
	/// a user.
	/// The field `remaining_investment_redeem` is set to the
	/// amount of the active redeem order of the user and will
	/// be subtracted from upon given fulfillment's
	pub fn from_order(order: &Order<Balance, OrderId>) -> Self {
		RedeemCollection {
			payout_investment_redeem: Zero::zero(),
			remaining_investment_redeem: order.amount(),
		}
	}
}

/// The collected investment/redemption amount for an account
#[derive(Encode, Default, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectedAmount<Collected, Payment> {
	/// The amount which was collected
	/// * If investment: Tranche tokens
	/// * If redemption: Payment currency
	pub amount_collected: Collected,

	/// The amount which was converted during processing based on the
	/// fulfillment price(s)
	/// * If investment: Payment currency
	/// * If redemption: Tranche tokens
	pub amount_payment: Payment,
}

impl<Collected: EnsureAddAssign + Copy, Payment: EnsureAddAssign + Copy>
	CollectedAmount<Collected, Payment>
{
	pub fn increase(&mut self, other: &Self) -> DispatchResult {
		self.amount_collected
			.ensure_add_assign(other.amount_collected)?;
		self.amount_payment
			.ensure_add_assign(other.amount_payment)?;

		Ok(())
	}
}

/// A representation of an investment portfolio consisting of free, pending and
/// claimable pool currency as well as tranche tokens.
#[derive(Encode, Decode, Default, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct InvestmentPortfolio<Balance, CurrencyId> {
	/// The identifier of the pool currency
	pub pool_currency_id: CurrencyId,
	/// The unprocessed invest order amount in pool currency
	pub pending_invest_currency: Balance,
	/// The amount of tranche tokens which can be collected for an invest order
	pub claimable_tranche_tokens: Balance,
	/// The amount of tranche tokens which can be transferred
	pub free_tranche_tokens: Balance,
	/// The amount of tranche tokens which can not be used at all and could get
	/// slashed
	pub reserved_tranche_tokens: Balance,
	/// The unprocessed redeem order amount in tranche tokens
	pub pending_redeem_tranche_tokens: Balance,
	/// The amount of pool currency which can be collected for a redeem order
	pub claimable_currency: Balance,
}

impl<Balance: Default, CurrencyId> InvestmentPortfolio<Balance, CurrencyId> {
	pub fn new(pool_currency_id: CurrencyId) -> Self {
		Self {
			pool_currency_id,
			pending_invest_currency: Balance::default(),
			claimable_tranche_tokens: Balance::default(),
			free_tranche_tokens: Balance::default(),
			reserved_tranche_tokens: Balance::default(),
			pending_redeem_tranche_tokens: Balance::default(),
			claimable_currency: Balance::default(),
		}
	}

	pub fn with_pending_invest_currency(mut self, amount: Balance) -> Self {
		self.pending_invest_currency = amount;
		self
	}

	pub fn with_free_tranche_tokens(mut self, amount: Balance) -> Self {
		self.free_tranche_tokens = amount;
		self
	}

	pub fn with_reserved_tranche_tokens(mut self, amount: Balance) -> Self {
		self.reserved_tranche_tokens = amount;
		self
	}

	pub fn with_claimable_tranche_tokens(mut self, amount: Balance) -> Self {
		self.claimable_tranche_tokens = amount;
		self
	}

	pub fn with_pending_redeem_tranche_tokens(mut self, amount: Balance) -> Self {
		self.pending_redeem_tranche_tokens = amount;
		self
	}

	pub fn with_claimable_currency(mut self, amount: Balance) -> Self {
		self.claimable_currency = amount;
		self
	}
}
