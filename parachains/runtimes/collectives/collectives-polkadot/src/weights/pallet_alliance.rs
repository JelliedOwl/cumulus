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

//! Autogenerated weights for `pallet_alliance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_alliance
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_alliance.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_alliance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_alliance::WeightInfo for WeightInfo<T> {
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalCount (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:0 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `502 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `10983 + m * (128 ±0) + p * (144 ±0)`
		// Minimum execution time: 27_095 nanoseconds.
		Weight::from_ref_time(29_092_349)
			.saturating_add(Weight::from_proof_size(10983))
			// Standard Error: 66
			.saturating_add(Weight::from_ref_time(227).saturating_mul(b.into()))
			// Standard Error: 693
			.saturating_add(Weight::from_ref_time(16_473).saturating_mul(m.into()))
			// Standard Error: 684
			.saturating_add(Weight::from_ref_time(85_357).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(128).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `932 + m * (64 ±0)`
		//  Estimated: `9092 + m * (64 ±0)`
		// Minimum execution time: 23_326 nanoseconds.
		Weight::from_ref_time(23_898_089)
			.saturating_add(Weight::from_proof_size(9092))
			// Standard Error: 768
			.saturating_add(Weight::from_ref_time(42_173).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(64).saturating_mul(m.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438 + m * (96 ±0) + p * (36 ±0)`
		//  Estimated: `11067 + m * (388 ±0) + p * (144 ±0)`
		// Minimum execution time: 32_798 nanoseconds.
		Weight::from_ref_time(30_793_132)
			.saturating_add(Weight::from_proof_size(11067))
			// Standard Error: 627
			.saturating_add(Weight::from_ref_time(42_262).saturating_mul(m.into()))
			// Standard Error: 612
			.saturating_add(Weight::from_ref_time(80_469).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(388).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(_b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `923 + m * (96 ±0) + p * (41 ±0)`
		//  Estimated: `15422 + m * (388 ±0) + p * (160 ±0)`
		// Minimum execution time: 43_082 nanoseconds.
		Weight::from_ref_time(43_271_807)
			.saturating_add(Weight::from_proof_size(15422))
			// Standard Error: 2_181
			.saturating_add(Weight::from_ref_time(54_556).saturating_mul(m.into()))
			// Standard Error: 2_126
			.saturating_add(Weight::from_ref_time(92_432).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(388).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(160).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance Rule (r:0 w:1)
	/// Proof: Alliance Rule (max_values: Some(1), max_size: Some(87), added: 582, mode: MaxEncodedLen)
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `677 + m * (96 ±0) + p * (41 ±0)`
		//  Estimated: `14232 + m * (507 ±0) + p * (202 ±0)`
		// Minimum execution time: 39_628 nanoseconds.
		Weight::from_ref_time(42_678_513)
			.saturating_add(Weight::from_proof_size(14232))
			// Standard Error: 3_758
			.saturating_add(Weight::from_ref_time(96_851).saturating_mul(m.into()))
			// Standard Error: 3_713
			.saturating_add(Weight::from_ref_time(106_948).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(507).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(202).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `545 + m * (96 ±0) + p * (36 ±0)`
		//  Estimated: `12436 + m * (480 ±0) + p * (180 ±0)`
		// Minimum execution time: 34_315 nanoseconds.
		Weight::from_ref_time(32_211_295)
			.saturating_add(Weight::from_proof_size(12436))
			// Standard Error: 634
			.saturating_add(Weight::from_ref_time(44_215).saturating_mul(m.into()))
			// Standard Error: 611
			.saturating_add(Weight::from_ref_time(81_416).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(480).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:2 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Members (r:1 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(m: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12`
		//  Estimated: `11879`
		// Minimum execution time: 27_135 nanoseconds.
		Weight::from_ref_time(17_311_729)
			.saturating_add(Weight::from_proof_size(11879))
			// Standard Error: 540
			.saturating_add(Weight::from_ref_time(116_892).saturating_mul(m.into()))
			// Standard Error: 534
			.saturating_add(Weight::from_ref_time(100_375).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Alliance Members (r:2 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance DepositOf (r:200 w:50)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: System Account (r:50 w:50)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 100]`.
	/// The range of component `y` is `[0, 100]`.
	/// The range of component `z` is `[0, 50]`.
	fn disband(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + x * (52 ±0) + y * (53 ±0) + z * (282 ±0)`
		//  Estimated: `31580 + x * (2590 ±0) + y * (2590 ±0) + z * (3200 ±1)`
		// Minimum execution time: 225_966 nanoseconds.
		Weight::from_ref_time(226_907_000)
			.saturating_add(Weight::from_proof_size(31580))
			// Standard Error: 18_893
			.saturating_add(Weight::from_ref_time(445_537).saturating_mul(x.into()))
			// Standard Error: 18_801
			.saturating_add(Weight::from_ref_time(424_753).saturating_mul(y.into()))
			// Standard Error: 37_569
			.saturating_add(Weight::from_ref_time(9_783_365).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(z.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(z.into())))
			.saturating_add(Weight::from_proof_size(2590).saturating_mul(x.into()))
			.saturating_add(Weight::from_proof_size(2590).saturating_mul(y.into()))
			.saturating_add(Weight::from_proof_size(3200).saturating_mul(z.into()))
	}
	/// Storage: Alliance Rule (r:0 w:1)
	/// Proof: Alliance Rule (max_values: Some(1), max_size: Some(87), added: 582, mode: MaxEncodedLen)
	fn set_rule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_535 nanoseconds.
		Weight::from_ref_time(8_952_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Announcements (r:1 w:1)
	/// Proof: Alliance Announcements (max_values: Some(1), max_size: Some(8702), added: 9197, mode: MaxEncodedLen)
	fn announce() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `9197`
		// Minimum execution time: 11_462 nanoseconds.
		Weight::from_ref_time(11_723_000)
			.saturating_add(Weight::from_proof_size(9197))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Announcements (r:1 w:1)
	/// Proof: Alliance Announcements (max_values: Some(1), max_size: Some(8702), added: 9197, mode: MaxEncodedLen)
	fn remove_announcement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `9197`
		// Minimum execution time: 12_514 nanoseconds.
		Weight::from_ref_time(12_788_000)
			.saturating_add(Weight::from_proof_size(9197))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Members (r:3 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Alliance DepositOf (r:0 w:1)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	fn join_alliance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `23358`
		// Minimum execution time: 39_404 nanoseconds.
		Weight::from_ref_time(39_750_000)
			.saturating_add(Weight::from_proof_size(23358))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Alliance Members (r:3 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	fn nominate_ally() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `20755`
		// Minimum execution time: 27_853 nanoseconds.
		Weight::from_ref_time(28_288_000)
			.saturating_add(Weight::from_proof_size(20755))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Members (r:2 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn elevate_ally() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `12761`
		// Minimum execution time: 23_448 nanoseconds.
		Weight::from_ref_time(23_912_000)
			.saturating_add(Weight::from_proof_size(12761))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Alliance Members (r:4 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance RetiringMembers (r:0 w:1)
	/// Proof: Alliance RetiringMembers (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn give_retirement_notice() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `24133`
		// Minimum execution time: 31_605 nanoseconds.
		Weight::from_ref_time(32_289_000)
			.saturating_add(Weight::from_proof_size(24133))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Alliance RetiringMembers (r:1 w:1)
	/// Proof: Alliance RetiringMembers (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Alliance Members (r:1 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: Alliance DepositOf (r:1 w:1)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn retire() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `13355`
		// Minimum execution time: 33_060 nanoseconds.
		Weight::from_ref_time(33_601_000)
			.saturating_add(Weight::from_proof_size(13355))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Alliance Members (r:3 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance DepositOf (r:1 w:1)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn kick_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `716`
		//  Estimated: `35980`
		// Minimum execution time: 121_049 nanoseconds.
		Weight::from_ref_time(121_889_000)
			.saturating_add(Weight::from_proof_size(35980))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// Proof: Alliance UnscrupulousWebsites (max_values: Some(1), max_size: Some(25702), added: 26197, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `29894`
		// Minimum execution time: 6_924 nanoseconds.
		Weight::from_ref_time(7_075_000)
			.saturating_add(Weight::from_proof_size(29894))
			// Standard Error: 3_431
			.saturating_add(Weight::from_ref_time(1_179_954).saturating_mul(n.into()))
			// Standard Error: 1_343
			.saturating_add(Weight::from_ref_time(68_017).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// Proof: Alliance UnscrupulousWebsites (max_values: Some(1), max_size: Some(25702), added: 26197, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + n * (289 ±0) + l * (100 ±0)`
		//  Estimated: `29894`
		// Minimum execution time: 6_937 nanoseconds.
		Weight::from_ref_time(7_061_000)
			.saturating_add(Weight::from_proof_size(29894))
			// Standard Error: 185_664
			.saturating_add(Weight::from_ref_time(13_615_694).saturating_mul(n.into()))
			// Standard Error: 72_714
			.saturating_add(Weight::from_ref_time(573_200).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Alliance Members (r:3 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn abdicate_fellow_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `18447`
		// Minimum execution time: 30_087 nanoseconds.
		Weight::from_ref_time(30_925_000)
			.saturating_add(Weight::from_proof_size(18447))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
