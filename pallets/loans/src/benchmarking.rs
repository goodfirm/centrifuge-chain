// Copyright 2023 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_primitives::CFG;
use cfg_traits::{
	benchmarking::FundedPoolBenchmarkHelper,
	changes::ChangeGuard,
	interest::{CompoundingSchedule, InterestAccrual, InterestRate},
	Permissions, PoolWriteOffPolicyMutate, TimeAsSecs, ValueProvider,
};
use cfg_types::{
	adjustments::Adjustment,
	permissions::{PermissionScope, PoolRole, Role},
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::tokens::nonfungibles::{Create, Mutate};
use frame_system::RawOrigin;
use sp_arithmetic::FixedPointNumber;
use sp_runtime::traits::{Bounded, Get, One, Zero};

use crate::{
	entities::{
		changes::{Change, LoanMutation},
		input::{PrincipalInput, RepaidInput},
		loans::LoanInfo,
		pricing::{
			internal::{InternalPricing, MaxBorrowAmount},
			Pricing,
		},
	},
	pallet::*,
	types::{
		cashflow::{InterestPayments, Maturity, PayDownSchedule, RepaymentSchedule},
		valuation::{DiscountedCashFlow, ValuationMethod},
		BorrowRestrictions, LoanRestrictions, RepayRestrictions,
	},
};

const COLLECION_ID: u16 = 42;
const COLLATERAL_VALUE: u128 = 1_000_000;
const FUNDS: u128 = 1_000_000_000;

type MaxRateCountOf<T> = <<T as Config>::InterestAccrual as InterestAccrual<
	<T as Config>::Rate,
	<T as Config>::Balance,
	Adjustment<<T as Config>::Balance>,
>>::MaxRateCount;

#[cfg(test)]
fn config_mocks() {
	use cfg_mocks::pallet_mock_data::util::MockDataCollection;

	use crate::tests::mock::{MockChangeGuard, MockPermissions, MockPools, MockPrices, MockTimer};

	MockPermissions::mock_add(|_, _, _| Ok(()));
	MockPermissions::mock_has(|_, _, _| true);
	MockPools::mock_pool_exists(|_| true);
	MockPools::mock_account_for(|_| 0);
	MockPools::mock_withdraw(|_, _, _| Ok(()));
	MockPools::mock_deposit(|_, _, _| Ok(()));
	MockPrices::mock_register_id(|_, _| Ok(()));
	MockPrices::mock_collection(|_| Ok(MockDataCollection::new(|_| Ok(Default::default()))));
	MockChangeGuard::mock_note(|_, change| {
		MockChangeGuard::mock_released(move |_, _| Ok(change.clone()));
		Ok(sp_core::H256::default())
	});
	MockTimer::mock_now(|| 0);
}

struct Helper<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> Helper<T>
where
	T::Balance: From<u128>,
	T::NonFungible: Create<T::AccountId> + Mutate<T::AccountId>,
	T::CollectionId: From<u16>,
	T::ItemId: From<u16>,
	T::PriceId: From<u32>,
	T::Pool: FundedPoolBenchmarkHelper<
		PoolId = T::PoolId,
		AccountId = T::AccountId,
		Balance = T::Balance,
	>,
	T::Moment: Default,
	T::PriceRegistry: ValueProvider<(u32, T::PoolId), T::PriceId, Value = PriceOf<T>>,
{
	fn prepare_benchmark() -> T::PoolId {
		#[cfg(test)]
		config_mocks();

		let pool_id = Default::default();

		let pool_admin = account("pool_admin", 0, 0);
		T::Pool::bench_create_funded_pool(pool_id, &pool_admin);

		let loan_admin = account("loan_admin", 0, 0);
		T::Permissions::add(
			PermissionScope::Pool(pool_id),
			loan_admin,
			Role::PoolRole(PoolRole::LoanAdmin),
		)
		.unwrap();

		let borrower = account::<T::AccountId>("borrower", 0, 0);
		T::Pool::bench_investor_setup(pool_id, borrower.clone(), (FUNDS * CFG).into());
		T::NonFungible::create_collection(&COLLECION_ID.into(), &borrower, &borrower).unwrap();
		T::Permissions::add(
			PermissionScope::Pool(pool_id),
			borrower,
			Role::PoolRole(PoolRole::Borrower),
		)
		.unwrap();

		pool_id
	}

	fn base_loan(item_id: T::ItemId) -> LoanInfo<T> {
		let maturity_offset = 40 * 365 * 24 * 3600; // 40 years

		LoanInfo {
			schedule: RepaymentSchedule {
				maturity: Maturity::fixed(T::Time::now() + maturity_offset),
				interest_payments: InterestPayments::OnceAtMaturity,
				pay_down_schedule: PayDownSchedule::None,
			},
			collateral: (COLLECION_ID.into(), item_id),
			interest_rate: InterestRate::Fixed {
				rate_per_year: T::Rate::saturating_from_rational(1, 5000),
				compounding: CompoundingSchedule::Secondly,
			},
			pricing: Pricing::Internal(InternalPricing {
				collateral_value: COLLATERAL_VALUE.into(),
				max_borrow_amount: MaxBorrowAmount::UpToOutstandingDebt {
					advance_rate: T::Rate::one(),
				},
				valuation_method: ValuationMethod::DiscountedCashFlow(DiscountedCashFlow {
					probability_of_default: T::Rate::zero(),
					loss_given_default: T::Rate::zero(),
					discount_rate: InterestRate::Fixed {
						rate_per_year: T::Rate::saturating_from_rational(1, 5000),
						compounding: CompoundingSchedule::Secondly,
					},
				}),
			}),
			restrictions: LoanRestrictions {
				borrows: BorrowRestrictions::NotWrittenOff,
				repayments: RepayRestrictions::None,
			},
		}
	}

	fn create_loan(pool_id: T::PoolId, item_id: T::ItemId) -> T::LoanId {
		let borrower = account("borrower", 0, 0);

		T::NonFungible::mint_into(&COLLECION_ID.into(), &item_id, &borrower).unwrap();

		Pallet::<T>::create(
			RawOrigin::Signed(borrower).into(),
			pool_id,
			Self::base_loan(item_id),
		)
		.unwrap();

		LastLoanId::<T>::get(pool_id)
	}

	fn borrow_loan(pool_id: T::PoolId, loan_id: T::LoanId) {
		let borrower = account("borrower", 0, 0);
		Pallet::<T>::borrow(
			RawOrigin::Signed(borrower).into(),
			pool_id,
			loan_id,
			PrincipalInput::Internal(10.into()),
		)
		.unwrap();
	}

	fn fully_repay_loan(pool_id: T::PoolId, loan_id: T::LoanId) {
		let borrower = account("borrower", 0, 0);
		Pallet::<T>::repay(
			RawOrigin::Signed(borrower).into(),
			pool_id,
			loan_id,
			RepaidInput {
				principal: PrincipalInput::Internal(10.into()),
				interest: T::Balance::max_value(),
				unscheduled: 0.into(),
			},
		)
		.unwrap();
	}

	fn create_mutation() -> LoanMutation<T::Rate> {
		LoanMutation::InterestPayments(InterestPayments::OnceAtMaturity)
	}

	fn propose_mutation(pool_id: T::PoolId, loan_id: T::LoanId) -> T::Hash {
		let pool_admin = account::<T::AccountId>("loan_admin", 0, 0);

		Pallet::<T>::propose_loan_mutation(
			RawOrigin::Signed(pool_admin).into(),
			pool_id,
			loan_id,
			Self::create_mutation(),
		)
		.unwrap();

		// We need to call noted again
		// (that is idempotent for the same change and instant)
		// to obtain the ChangeId used previously.
		T::ChangeGuard::note(
			pool_id,
			Change::<T>::Loan(loan_id, Self::create_mutation()).into(),
		)
		.unwrap()
	}

	fn propose_policy(pool_id: T::PoolId) -> T::Hash {
		let pool_admin = account("pool_admin", 0, 0);
		let policy = Pallet::<T>::worst_case_policy();
		Pallet::<T>::propose_write_off_policy(
			RawOrigin::Signed(pool_admin).into(),
			pool_id,
			policy.clone(),
		)
		.unwrap();

		// We need to call noted again
		// (that is idempotent for the same change and instant)
		// to obtain the ChangeId used previously.
		T::ChangeGuard::note(pool_id, Change::<T>::Policy(policy).into()).unwrap()
	}

	fn propose_transfer_debt(pool_id: T::PoolId) -> T::Hash {
		let borrower = account("borrower", 0, 0);
		let loan_1 = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_1);
		let loan_2 = Helper::<T>::create_loan(pool_id, (u16::MAX - 1).into());

		let repaid_amount = RepaidInput {
			principal: PrincipalInput::Internal(10.into()),
			interest: 0.into(),
			unscheduled: 0.into(),
		};
		let borrow_amount = PrincipalInput::Internal(10.into());

		Pallet::<T>::propose_transfer_debt(
			RawOrigin::Signed(borrower).into(),
			pool_id,
			loan_1,
			loan_2,
			repaid_amount.clone(),
			borrow_amount.clone(),
		)
		.unwrap();

		// We need to call noted again
		// (that is idempotent for the same change and instant)
		// to obtain the ChangeId used previously.
		T::ChangeGuard::note(
			pool_id,
			Change::<T>::TransferDebt(loan_1, loan_2, repaid_amount, borrow_amount).into(),
		)
		.unwrap()
	}

	fn set_policy(pool_id: T::PoolId) {
		let change_id = Self::propose_policy(pool_id);

		let any = account("any", 0, 0);
		Pallet::<T>::apply_write_off_policy(RawOrigin::Signed(any).into(), pool_id, change_id)
			.unwrap();
	}

	fn expire_loan(pool_id: T::PoolId, loan_id: T::LoanId) {
		Pallet::<T>::expire_action(pool_id, loan_id).unwrap();
	}

	fn initialize_active_state(n: u32) -> T::PoolId {
		let pool_id = Self::prepare_benchmark();

		for i in 1..MaxRateCountOf::<T>::get() {
			// First `i` (i=0) used by the loan's interest rate.
			T::InterestAccrual::reference_rate(&InterestRate::Fixed {
				rate_per_year: T::Rate::saturating_from_rational(i + 1, 5000),
				compounding: CompoundingSchedule::Secondly,
			})
			.unwrap();
		}

		// Populate the price registry with prices
		for i in 0..n {
			T::PriceRegistry::set(&(0, pool_id), &T::PriceId::from(i), Default::default());
		}

		for i in 0..n {
			let item_id = (i as u16).into();
			let loan_id = Self::create_loan(pool_id, item_id);
			Self::borrow_loan(pool_id, loan_id);
		}

		pool_id
	}

	fn max_active_loans() -> u32 {
		T::MaxActiveLoansPerPool::get().min(10)
	}
}

benchmarks! {
	where_clause {
	where
		T::Balance: From<u128>,
		T::NonFungible: Create<T::AccountId> + Mutate<T::AccountId>,
		T::CollectionId: From<u16>,
		T::ItemId: From<u16>,
		T::PriceId: From<u32>,
		T::Pool: FundedPoolBenchmarkHelper<PoolId = T::PoolId, AccountId = T::AccountId, Balance = T::Balance>,
		T::Moment: Default,
		T::PriceRegistry: ValueProvider<(u32, T::PoolId), T::PriceId, Value = PriceOf<T>>,
	}

	create {
		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::prepare_benchmark();

		let (collection_id, item_id) = (COLLECION_ID.into(), 1.into());
		T::NonFungible::mint_into(&collection_id, &item_id, &borrower).unwrap();
		let loan_info = Helper::<T>::base_loan(item_id);

	}: _(RawOrigin::Signed(borrower), pool_id, loan_info)

	borrow {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id, PrincipalInput::Internal(10.into()))

	repay {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);

		let repaid = RepaidInput {
			principal: PrincipalInput::Internal(10.into()),
			interest: 0.into(),
			unscheduled: 0.into()
		};

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id, repaid)

	write_off {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);
		Helper::<T>::set_policy(pool_id);
		Helper::<T>::expire_loan(pool_id, loan_id);

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id)

	admin_write_off {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let loan_admin = account("loan_admin", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);
		Helper::<T>::set_policy(pool_id);

	}: _(RawOrigin::Signed(loan_admin), pool_id, loan_id, T::Rate::zero(), T::Rate::zero())

	propose_loan_mutation {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let loan_admin = account("loan_admin", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);

		let mutation = Helper::<T>::create_mutation();

	}: _(RawOrigin::Signed(loan_admin), pool_id, loan_id, mutation)

	apply_loan_mutation {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let any = account("any", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);

		let change_id = Helper::<T>::propose_mutation(pool_id, loan_id);

	}: _(RawOrigin::Signed(any), pool_id, change_id)

	close {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);
		Helper::<T>::fully_repay_loan(pool_id, loan_id);

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id)

	propose_write_off_policy {
		let pool_admin = account("pool_admin", 0, 0);
		let pool_id = Helper::<T>::prepare_benchmark();
		let policy = Pallet::<T>::worst_case_policy();

	}: _(RawOrigin::Signed(pool_admin), pool_id, policy)

	apply_write_off_policy {
		let any = account("any", 0, 0);
		let pool_id = Helper::<T>::prepare_benchmark();
		let change_id = Helper::<T>::propose_policy(pool_id);

	}: _(RawOrigin::Signed(any), pool_id, change_id)

	update_portfolio_valuation {
		let n in 1..Helper::<T>::max_active_loans();

		let any = account("any", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);

	}: _(RawOrigin::Signed(any), pool_id)
	verify {
		assert!(Pallet::<T>::portfolio_valuation(pool_id).value() > Zero::zero());
	}

	propose_transfer_debt {
		let n in 2..Helper::<T>::max_active_loans() - 2;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_1 = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_1);
		let loan_2 = Helper::<T>::create_loan(pool_id, (u16::MAX - 1).into());

		let repaid_amount = RepaidInput {
			principal: PrincipalInput::Internal(10.into()),
			interest: 0.into(),
			unscheduled: 0.into()
		};
		let borrow_amount = PrincipalInput::Internal(10.into());

	}: _(RawOrigin::Signed(borrower), pool_id, loan_1, loan_2, repaid_amount, borrow_amount)

	apply_transfer_debt {
		let n in 2..Helper::<T>::max_active_loans() - 2;

		let any = account("any", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let change_id = Helper::<T>::propose_transfer_debt(pool_id);

	}: _(RawOrigin::Signed(any), pool_id, change_id)

	increase_debt {
		let n in 1..Helper::<T>::max_active_loans() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id, PrincipalInput::Internal(10.into()))
}

impl_benchmark_test_suite!(
	Pallet,
	crate::tests::mock::new_test_ext(),
	crate::tests::mock::Runtime
);
