
//! Autogenerated weights for `pallet_interest_accrual`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_interest_accrual
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_interest_accrual.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_interest_accrual`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_interest_accrual::WeightInfo for WeightInfo<T> {
	/// The range of component `n` is `[1, 25]`.
	fn calculate_accumulated_rate(n: u32, ) -> Weight {
		// Minimum execution time: 800 nanoseconds.
		Weight::from_ref_time(801_000 as u64)
			// Standard Error: 1_919
			.saturating_add(Weight::from_ref_time(924_319 as u64).saturating_mul(n as u64))
	}
}