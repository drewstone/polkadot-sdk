// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_nis`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-anb7yjbi-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_nis
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/nis/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_nis`.
pub trait WeightInfo {
	fn place_bid(l: u32, ) -> Weight;
	fn place_bid_max() -> Weight;
	fn retract_bid(l: u32, ) -> Weight;
	fn fund_deficit() -> Weight;
	fn communify() -> Weight;
	fn privatize() -> Weight;
	fn thaw_private() -> Weight;
	fn thaw_communal() -> Weight;
	fn process_queues() -> Weight;
	fn process_queue() -> Weight;
	fn process_bid() -> Weight;
}

/// Weights for `pallet_nis` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6210 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 47_065_000 picoseconds.
		Weight::from_parts(52_894_557, 51487)
			// Standard Error: 275
			.saturating_add(Weight::from_parts(48_441, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn place_bid_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54212`
		//  Estimated: `51487`
		// Minimum execution time: 111_930_000 picoseconds.
		Weight::from_parts(114_966_000, 51487)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6210 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 47_726_000 picoseconds.
		Weight::from_parts(48_162_043, 51487)
			// Standard Error: 187
			.saturating_add(Weight::from_parts(38_372, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:0)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn fund_deficit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `3593`
		// Minimum execution time: 31_194_000 picoseconds.
		Weight::from_parts(32_922_000, 3593)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn communify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `702`
		//  Estimated: `3675`
		// Minimum execution time: 73_288_000 picoseconds.
		Weight::from_parts(76_192_000, 3675)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn privatize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `863`
		//  Estimated: `3675`
		// Minimum execution time: 94_307_000 picoseconds.
		Weight::from_parts(96_561_000, 3675)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn thaw_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3658`
		// Minimum execution time: 49_873_000 picoseconds.
		Weight::from_parts(51_361_000, 3658)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn thaw_communal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `807`
		//  Estimated: `3675`
		// Minimum execution time: 96_884_000 picoseconds.
		Weight::from_parts(98_867_000, 3675)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn process_queues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6658`
		//  Estimated: `7487`
		// Minimum execution time: 21_019_000 picoseconds.
		Weight::from_parts(22_057_000, 7487)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	fn process_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `51487`
		// Minimum execution time: 4_746_000 picoseconds.
		Weight::from_parts(4_953_000, 51487)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:0 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn process_bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_836_000 picoseconds.
		Weight::from_parts(5_093_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6210 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 47_065_000 picoseconds.
		Weight::from_parts(52_894_557, 51487)
			// Standard Error: 275
			.saturating_add(Weight::from_parts(48_441, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn place_bid_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54212`
		//  Estimated: `51487`
		// Minimum execution time: 111_930_000 picoseconds.
		Weight::from_parts(114_966_000, 51487)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6210 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 47_726_000 picoseconds.
		Weight::from_parts(48_162_043, 51487)
			// Standard Error: 187
			.saturating_add(Weight::from_parts(38_372, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:0)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn fund_deficit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `3593`
		// Minimum execution time: 31_194_000 picoseconds.
		Weight::from_parts(32_922_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn communify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `702`
		//  Estimated: `3675`
		// Minimum execution time: 73_288_000 picoseconds.
		Weight::from_parts(76_192_000, 3675)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn privatize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `863`
		//  Estimated: `3675`
		// Minimum execution time: 94_307_000 picoseconds.
		Weight::from_parts(96_561_000, 3675)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn thaw_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3658`
		// Minimum execution time: 49_873_000 picoseconds.
		Weight::from_parts(51_361_000, 3658)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn thaw_communal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `807`
		//  Estimated: `3675`
		// Minimum execution time: 96_884_000 picoseconds.
		Weight::from_parts(98_867_000, 3675)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn process_queues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6658`
		//  Estimated: `7487`
		// Minimum execution time: 21_019_000 picoseconds.
		Weight::from_parts(22_057_000, 7487)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	fn process_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `51487`
		// Minimum execution time: 4_746_000 picoseconds.
		Weight::from_parts(4_953_000, 51487)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:0 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn process_bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_836_000 picoseconds.
		Weight::from_parts(5_093_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
