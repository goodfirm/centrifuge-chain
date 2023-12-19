
//! Autogenerated weights for `pallet_order_book`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_order_book
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_order_book.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_order_book`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_order_book::WeightInfo for WeightInfo<T> {
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderIdNonceStore (r:1 w:1)
	/// Proof: OrderBook OrderIdNonceStore (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:1 w:1)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrderBook TradingPair (r:1 w:0)
	/// Proof: OrderBook TradingPair (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: OrderBook AssetPairOrders (r:1 w:1)
	/// Proof: OrderBook AssetPairOrders (max_values: None, max_size: Some(8000070), added: 8002545, mode: MaxEncodedLen)
	/// Storage: OrderBook Orders (r:0 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:0 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `763`
		//  Estimated: `8003535`
		// Minimum execution time: 74_690_000 picoseconds.
		Weight::from_parts(76_263_000, 0)
			.saturating_add(Weight::from_parts(0, 8003535))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: OrderBook Orders (r:1 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrmlTokens Accounts (r:1 w:1)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrderBook TradingPair (r:1 w:0)
	/// Proof: OrderBook TradingPair (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:1 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn user_update_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1205`
		//  Estimated: `7145`
		// Minimum execution time: 74_459_000 picoseconds.
		Weight::from_parts(75_872_000, 0)
			.saturating_add(Weight::from_parts(0, 7145))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: OrderBook Orders (r:1 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:1 w:1)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook AssetPairOrders (r:1 w:1)
	/// Proof: OrderBook AssetPairOrders (max_values: None, max_size: Some(8000070), added: 8002545, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:0 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn user_cancel_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `996`
		//  Estimated: `8003535`
		// Minimum execution time: 61_856_000 picoseconds.
		Weight::from_parts(63_399_000, 0)
			.saturating_add(Weight::from_parts(0, 8003535))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: OrderBook Orders (r:1 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:4 w:4)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook AssetPairOrders (r:1 w:1)
	/// Proof: OrderBook AssetPairOrders (max_values: None, max_size: Some(8000070), added: 8002545, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments ForeignInvestmentInfo (r:1 w:0)
	/// Proof: ForeignInvestments ForeignInvestmentInfo (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:0 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn fill_order_full() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1350`
		//  Estimated: `8003535`
		// Minimum execution time: 159_919_000 picoseconds.
		Weight::from_parts(162_424_000, 0)
			.saturating_add(Weight::from_parts(0, 8003535))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	/// Storage: OrderBook Orders (r:1 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:4 w:4)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook TradingPair (r:1 w:0)
	/// Proof: OrderBook TradingPair (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:1 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments ForeignInvestmentInfo (r:1 w:0)
	/// Proof: ForeignInvestments ForeignInvestmentInfo (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn fill_order_partial() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1535`
		//  Estimated: `11406`
		// Minimum execution time: 169_597_000 picoseconds.
		Weight::from_parts(171_672_000, 0)
			.saturating_add(Weight::from_parts(0, 11406))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: OrderBook TradingPair (r:0 w:1)
	/// Proof: OrderBook TradingPair (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn add_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_605_000 picoseconds.
		Weight::from_parts(14_127_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: OrderBook TradingPair (r:0 w:1)
	/// Proof: OrderBook TradingPair (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn rm_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_937_000 picoseconds.
		Weight::from_parts(15_349_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: OrderBook TradingPair (r:1 w:1)
	/// Proof: OrderBook TradingPair (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn update_min_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `209`
		//  Estimated: `3547`
		// Minimum execution time: 21_991_000 picoseconds.
		Weight::from_parts(22_632_000, 0)
			.saturating_add(Weight::from_parts(0, 3547))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}