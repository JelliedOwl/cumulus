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

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westmint-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=westmint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_utility
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/westmint/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_005 nanoseconds.
		Weight::from_ref_time(20_853_430)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 2_915
			.saturating_add(Weight::from_ref_time(3_962_321).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_630 nanoseconds.
		Weight::from_ref_time(4_793_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_221 nanoseconds.
		Weight::from_ref_time(18_105_158)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 2_971
			.saturating_add(Weight::from_ref_time(4_208_542).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_983 nanoseconds.
		Weight::from_ref_time(8_212_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_030 nanoseconds.
		Weight::from_ref_time(12_833_718)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 2_633
			.saturating_add(Weight::from_ref_time(3_956_586).saturating_mul(c.into()))
	}
}
