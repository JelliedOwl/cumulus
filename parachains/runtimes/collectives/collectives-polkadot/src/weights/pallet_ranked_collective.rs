// Copyright Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_ranked_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_ranked_collective
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_ranked_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_ranked_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ranked_collective::WeightInfo for WeightInfo<T> {
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:1)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:0 w:1)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:0 w:1)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3507`
		// Minimum execution time: 17_437_000 picoseconds.
		Weight::from_parts(17_646_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:11 w:11)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:11 w:11)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:11 w:11)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `584 + r * (281 ±0)`
		//  Estimated: `3519 + r * (2529 ±0)`
		// Minimum execution time: 28_698_000 picoseconds.
		Weight::from_parts(31_441_420, 0)
			.saturating_add(Weight::from_parts(0, 3519))
			// Standard Error: 25_785
			.saturating_add(Weight::from_parts(12_450_241, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2529).saturating_mul(r.into()))
	}
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:1)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:0 w:1)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:0 w:1)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281 + r * (17 ±0)`
		//  Estimated: `3507`
		// Minimum execution time: 20_319_000 picoseconds.
		Weight::from_parts(21_050_040, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			// Standard Error: 3_562
			.saturating_add(Weight::from_parts(338_796, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:1)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:1 w:1)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:1 w:1)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `599 + r * (72 ±0)`
		//  Estimated: `3519`
		// Minimum execution time: 28_799_000 picoseconds.
		Weight::from_parts(31_074_181, 0)
			.saturating_add(Weight::from_parts(0, 3519))
			// Standard Error: 17_336
			.saturating_add(Weight::from_parts(614_951, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipCollective Members (r:1 w:0)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective Voting (r:1 w:1)
	/// Proof: FellowshipCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `317568`
		// Minimum execution time: 48_431_000 picoseconds.
		Weight::from_parts(48_867_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective VotingCleanup (r:1 w:0)
	/// Proof: FellowshipCollective VotingCleanup (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective Voting (r:100 w:100)
	/// Proof: FellowshipCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467 + n * (50 ±0)`
		//  Estimated: `4365 + n * (2540 ±0)`
		// Minimum execution time: 15_419_000 picoseconds.
		Weight::from_parts(19_562_203, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			// Standard Error: 1_399
			.saturating_add(Weight::from_parts(986_173, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2540).saturating_mul(n.into()))
	}
}
