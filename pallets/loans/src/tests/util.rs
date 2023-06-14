use super::*;

pub fn total_borrowed_rate(value: f64) -> MaxBorrowAmount<Rate> {
	MaxBorrowAmount::UpToTotalBorrowed {
		advance_rate: Rate::from_float(value),
	}
}

pub fn outstanding_debt_rate(value: f64) -> MaxBorrowAmount<Rate> {
	MaxBorrowAmount::UpToOutstandingDebt {
		advance_rate: Rate::from_float(value),
	}
}

pub fn get_loan(loan_id: LoanId) -> ActiveLoan<Runtime> {
	ActiveLoans::<Runtime>::get(POOL_A)
		.into_iter()
		.find(|(id, _)| *id == loan_id)
		.unwrap()
		.1
}

pub fn current_loan_debt(loan_id: LoanId) -> Balance {
	match get_loan(loan_id).pricing() {
		ActivePricing::Internal(pricing) => pricing.calculate_debt().unwrap(),
		ActivePricing::External(pricing) => pricing.calculate_debt().unwrap(),
	}
}

pub fn current_loan_pv(loan_id: LoanId) -> Balance {
	get_loan(loan_id).present_value().unwrap()
}

pub fn interest_for(rate: f64, elapsed: Duration) -> f64 {
	(1.0 + rate / YEAR.as_secs() as f64).powi(elapsed.as_secs() as i32)
}

pub fn current_debt_for(interest: f64, balance: Balance) -> Balance {
	(interest * balance as f64) as Balance
}

pub fn set_up_policy(percentage: f64, penalty: f64) {
	MockPermissions::mock_has(|_, _, _| true);
	MockPools::mock_pool_exists(|_| true);

	Loans::update_write_off_policy(
		RuntimeOrigin::signed(0),
		POOL_A,
		vec![WriteOffRule::new(
			[WriteOffTrigger::PrincipalOverdueDays(1)],
			Rate::from_float(percentage),
			Rate::from_float(penalty),
		)]
		.try_into()
		.unwrap(),
	)
	.expect("successful policy");

	MockPermissions::mock_has(|_, _, _| panic!("no has() mock"));
	MockPools::mock_pool_exists(|_| panic!("no pool_exists() mock"));
}

pub fn base_internal_pricing() -> InternalPricing<Runtime> {
	InternalPricing {
		collateral_value: COLLATERAL_VALUE,
		interest_rate: Rate::from_float(DEFAULT_INTEREST_RATE),
		max_borrow_amount: util::total_borrowed_rate(1.0),
		valuation_method: ValuationMethod::OutstandingDebt,
	}
}

pub fn base_internal_loan() -> LoanInfo<Runtime> {
	LoanInfo {
		schedule: RepaymentSchedule {
			maturity: Maturity::Fixed((now() + YEAR).as_secs()),
			interest_payments: InterestPayments::None,
			pay_down_schedule: PayDownSchedule::None,
		},
		collateral: ASSET_AA,
		pricing: Pricing::Internal(base_internal_pricing()),
		restrictions: LoanRestrictions {
			borrows: BorrowRestrictions::NotWrittenOff,
			repayments: RepayRestrictions::None,
		},
	}
}

pub fn base_external_loan() -> LoanInfo<Runtime> {
	LoanInfo {
		schedule: RepaymentSchedule {
			maturity: Maturity::Fixed((now() + YEAR).as_secs()),
			interest_payments: InterestPayments::None,
			pay_down_schedule: PayDownSchedule::None,
		},
		collateral: ASSET_AA,
		pricing: Pricing::External(ExternalPricing {
			price_id: REGISTER_PRICE_ID,
			max_borrow_quantity: QUANTITY,
		}),
		restrictions: LoanRestrictions {
			borrows: BorrowRestrictions::NotWrittenOff,
			repayments: RepayRestrictions::None,
		},
	}
}

pub fn create_loan(loan: LoanInfo<Runtime>) -> LoanId {
	MockPermissions::mock_has(|_, _, _| true);
	MockPools::mock_pool_exists(|_| true);
	MockPools::mock_account_for(|_| POOL_A_ACCOUNT);
	MockPrices::mock_get(|_| Ok((PRICE_VALUE, BLOCK_TIME.as_secs())));

	Loans::create(RuntimeOrigin::signed(BORROWER), POOL_A, loan).expect("successful creation");

	MockPermissions::mock_has(|_, _, _| panic!("no has() mock"));
	MockPools::mock_pool_exists(|_| panic!("no pool_exists() mock"));
	MockPools::mock_account_for(|_| panic!("no account_for() mock"));
	MockPrices::mock_get(|_| panic!("no get() mock"));

	LastLoanId::<Runtime>::get(POOL_A)
}

pub fn borrow_loan(loan_id: LoanId, borrow_amount: Balance) {
	MockPools::mock_withdraw(|_, _, _| Ok(()));
	MockPrices::mock_get(|_| Ok((PRICE_VALUE, BLOCK_TIME.as_secs())));
	MockPrices::mock_register_id(|_, _| Ok(()));

	Loans::borrow(
		RuntimeOrigin::signed(BORROWER),
		POOL_A,
		loan_id,
		borrow_amount,
	)
	.expect("successful borrowing");

	MockPools::mock_withdraw(|_, _, _| panic!("no withdraw() mock"));
	MockPrices::mock_get(|_| panic!("no get() mock"));
	MockPrices::mock_register_id(|_, _| panic!("no register_id() mock"));
}

pub fn repay_loan(loan_id: LoanId, repay_amount: Balance) {
	MockPools::mock_deposit(|_, _, _| Ok(()));
	MockPrices::mock_get(|_| Ok((PRICE_VALUE, BLOCK_TIME.as_secs())));

	Loans::repay(
		RuntimeOrigin::signed(BORROWER),
		POOL_A,
		loan_id,
		repay_amount,
		0,
	)
	.expect("successful repaying");

	MockPools::mock_deposit(|_, _, _| panic!("no deposit() mock"));
	MockPrices::mock_get(|_| panic!("no get() mock"));
}

pub fn write_off_loan(loan_id: LoanId) {
	set_up_policy(POLICY_PERCENTAGE, POLICY_PENALTY);
	MockPrices::mock_get(|_| Ok((PRICE_VALUE, BLOCK_TIME.as_secs())));

	Loans::write_off(RuntimeOrigin::signed(ANY), POOL_A, loan_id).expect("successful write off");

	MockPrices::mock_get(|_| panic!("no get() mock"));
}

pub fn close_loan(loan_id: LoanId) {
	MockPrices::mock_unregister_id(|_, _| Ok(()));

	Loans::close(RuntimeOrigin::signed(BORROWER), POOL_A, loan_id).expect("successful clossing");

	MockPrices::mock_get(|_| panic!("no unregister_id() mock"));
}