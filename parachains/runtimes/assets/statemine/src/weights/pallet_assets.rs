// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemine-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=statemine-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_assets
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemine/src/weights/pallet_assets.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141`
		//  Estimated: `5288`
		// Minimum execution time: 21_480 nanoseconds.
		Weight::from_ref_time(22_479_000)
			.saturating_add(Weight::from_proof_size(5288))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `2685`
		// Minimum execution time: 10_581 nanoseconds.
		Weight::from_ref_time(10_806_000)
			.saturating_add(Weight::from_proof_size(2685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `2685`
		// Minimum execution time: 13_004 nanoseconds.
		Weight::from_ref_time(13_495_000)
			.saturating_add(Weight::from_proof_size(2685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1001 w:1000)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1000 w:1000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (240 ±0)`
		//  Estimated: `5262 + c * (5180 ±0)`
		// Minimum execution time: 15_576 nanoseconds.
		Weight::from_ref_time(15_884_000)
			.saturating_add(Weight::from_proof_size(5262))
			// Standard Error: 11_854
			.saturating_add(Weight::from_ref_time(13_888_242).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_proof_size(5180).saturating_mul(c.into()))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1001 w:1000)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `446 + a * (86 ±0)`
		//  Estimated: `5308 + a * (2623 ±0)`
		// Minimum execution time: 16_017 nanoseconds.
		Weight::from_ref_time(16_338_000)
			.saturating_add(Weight::from_proof_size(5308))
			// Standard Error: 8_169
			.saturating_add(Weight::from_ref_time(13_483_682).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_proof_size(2623).saturating_mul(a.into()))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `5300`
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_ref_time(13_303_000)
			.saturating_add(Weight::from_proof_size(5300))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `5262`
		// Minimum execution time: 23_279 nanoseconds.
		Weight::from_ref_time(23_636_000)
			.saturating_add(Weight::from_proof_size(5262))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `5262`
		// Minimum execution time: 29_202 nanoseconds.
		Weight::from_ref_time(29_626_000)
			.saturating_add(Weight::from_proof_size(5262))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `10442`
		// Minimum execution time: 39_208 nanoseconds.
		Weight::from_ref_time(39_809_000)
			.saturating_add(Weight::from_proof_size(10442))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `10442`
		// Minimum execution time: 34_563 nanoseconds.
		Weight::from_ref_time(35_110_000)
			.saturating_add(Weight::from_proof_size(10442))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `10442`
		// Minimum execution time: 39_294 nanoseconds.
		Weight::from_ref_time(39_786_000)
			.saturating_add(Weight::from_proof_size(10442))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `5262`
		// Minimum execution time: 16_510 nanoseconds.
		Weight::from_ref_time(16_785_000)
			.saturating_add(Weight::from_proof_size(5262))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `5262`
		// Minimum execution time: 16_277 nanoseconds.
		Weight::from_ref_time(16_620_000)
			.saturating_add(Weight::from_proof_size(5262))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `2685`
		// Minimum execution time: 12_837 nanoseconds.
		Weight::from_ref_time(13_097_000)
			.saturating_add(Weight::from_proof_size(2685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `2685`
		// Minimum execution time: 12_379 nanoseconds.
		Weight::from_ref_time(12_751_000)
			.saturating_add(Weight::from_proof_size(2685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:0)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `5300`
		// Minimum execution time: 13_801 nanoseconds.
		Weight::from_ref_time(14_147_000)
			.saturating_add(Weight::from_proof_size(5300))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `2685`
		// Minimum execution time: 12_785 nanoseconds.
		Weight::from_ref_time(13_045_000)
			.saturating_add(Weight::from_proof_size(2685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `5300`
		// Minimum execution time: 22_261 nanoseconds.
		Weight::from_ref_time(25_126_392)
			.saturating_add(Weight::from_proof_size(5300))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `471`
		//  Estimated: `5300`
		// Minimum execution time: 22_941 nanoseconds.
		Weight::from_ref_time(23_301_000)
			.saturating_add(Weight::from_proof_size(5300))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `5300`
		// Minimum execution time: 12_008 nanoseconds.
		Weight::from_ref_time(14_212_294)
			.saturating_add(Weight::from_proof_size(5300))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(140), added: 2615, mode: MaxEncodedLen)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `471`
		//  Estimated: `5300`
		// Minimum execution time: 23_111 nanoseconds.
		Weight::from_ref_time(23_495_000)
			.saturating_add(Weight::from_proof_size(5300))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `2685`
		// Minimum execution time: 12_171 nanoseconds.
		Weight::from_ref_time(12_430_000)
			.saturating_add(Weight::from_proof_size(2685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `5308`
		// Minimum execution time: 25_733 nanoseconds.
		Weight::from_ref_time(26_003_000)
			.saturating_add(Weight::from_proof_size(5308))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `553`
		//  Estimated: `13065`
		// Minimum execution time: 52_240 nanoseconds.
		Weight::from_ref_time(52_888_000)
			.saturating_add(Weight::from_proof_size(13065))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `479`
		//  Estimated: `5308`
		// Minimum execution time: 43_187 nanoseconds.
		Weight::from_ref_time(44_259_000)
			.saturating_add(Weight::from_proof_size(5308))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Approvals (r:1 w:1)
	/// Proof: Assets Approvals (max_values: None, max_size: Some(148), added: 2623, mode: MaxEncodedLen)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `479`
		//  Estimated: `5308`
		// Minimum execution time: 27_781 nanoseconds.
		Weight::from_ref_time(28_228_000)
			.saturating_add(Weight::from_proof_size(5308))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
