
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development-local"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/development/src/weights/pallet_elections_phragmen.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 5]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 42_189_000 picoseconds.
		Weight::from_parts(43_316_483, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 9_129
			.saturating_add(Weight::from_parts(107_488, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 5]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 60_193_000 picoseconds.
		Weight::from_parts(61_551_446, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 17_156
			.saturating_add(Weight::from_parts(64_571, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 5]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 59_581_000 picoseconds.
		Weight::from_parts(60_619_408, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 16_127
			.saturating_add(Weight::from_parts(247_487, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `4764`
		// Minimum execution time: 62_898_000 picoseconds.
		Weight::from_parts(63_709_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `996 + c * (48 ±0)`
		//  Estimated: `2481 + c * (48 ±0)`
		// Minimum execution time: 44_774_000 picoseconds.
		Weight::from_parts(45_886_370, 0)
			.saturating_add(Weight::from_parts(0, 2481))
			// Standard Error: 2_440
			.saturating_add(Weight::from_parts(101_134, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351 + c * (48 ±0)`
		//  Estimated: `1836 + c * (48 ±0)`
		// Minimum execution time: 41_207_000 picoseconds.
		Weight::from_parts(42_757_501, 0)
			.saturating_add(Weight::from_parts(0, 1836))
			// Standard Error: 3_808
			.saturating_add(Weight::from_parts(26_961, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1210`
		//  Estimated: `2695`
		// Minimum execution time: 58_661_000 picoseconds.
		Weight::from_parts(59_281_000, 0)
			.saturating_add(Weight::from_parts(0, 2695))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `672`
		//  Estimated: `2157`
		// Minimum execution time: 41_217_000 picoseconds.
		Weight::from_parts(42_790_000, 0)
			.saturating_add(Weight::from_parts(0, 2157))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 500_000_000_000 picoseconds.
		Weight::from_parts(500_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1210`
		//  Estimated: `3593`
		// Minimum execution time: 66_274_000 picoseconds.
		Weight::from_parts(67_747_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Elections Voting (r:101 w:100)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:100 w:100)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:100 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: System Account (r:100 w:100)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[50, 100]`.
	/// The range of component `d` is `[0, 50]`.
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1193 + v * (457 ±0)`
		//  Estimated: `4528 + d * (1 ±0) + v * (3774 ±0)`
		// Minimum execution time: 3_484_190_000 picoseconds.
		Weight::from_parts(3_500_240_000, 0)
			.saturating_add(Weight::from_parts(0, 4528))
			// Standard Error: 273_567
			.saturating_add(Weight::from_parts(40_835_738, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:101 w:0)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:10 w:10)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections ElectionRounds (r:1 w:1)
	/// Proof Skipped: Elections ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	/// The range of component `v` is `[1, 100]`.
	/// The range of component `e` is `[100, 500]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (199 ±0) + e * (24 ±0) + v * (248 ±0)`
		//  Estimated: `7770 + c * (1665 ±11) + e * (19 ±0) + v * (2536 ±2)`
		// Minimum execution time: 447_939_000 picoseconds.
		Weight::from_parts(449_933_000, 0)
			.saturating_add(Weight::from_parts(0, 7770))
			// Standard Error: 554_684
			.saturating_add(Weight::from_parts(3_757_931, 0).saturating_mul(c.into()))
			// Standard Error: 110_341
			.saturating_add(Weight::from_parts(6_307_320, 0).saturating_mul(v.into()))
			// Standard Error: 23_962
			.saturating_add(Weight::from_parts(70_927, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 1665).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 19).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2536).saturating_mul(v.into()))
	}
}