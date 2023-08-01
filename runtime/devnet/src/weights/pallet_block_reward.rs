// Copyright 2023 Watr Foundation
// This file is part of Watr.

// Watr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Watr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Watr.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_block_reward`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-18, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `PAR03610`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("devnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/watr-node
// benchmark
// pallet
// --chain=devnet-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_block_reward
// --extrinsic=*
// --json
// --header=./file_header.txt
// --output=runtime/devnet/src/weights/pallet_block_reward.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_block_reward`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_block_reward::WeightInfo for WeightInfo<T> {
	// Storage: BlockReward RewardDistributionConfigStorage (r:0 w:1)
	fn set_configuration() -> Weight {
		Weight::from_ref_time(17_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
