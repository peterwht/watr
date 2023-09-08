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

//! Autogenerated weights for `pallet_xc_asset_config`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `david-tmp-bench`, CPU: `Intel(R) Xeon(R) CPU @ 2.80GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("mainnet-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/watr-node
// benchmark
// pallet
// --chain=mainnet-dev
// --wasm-execution=compiled
// --pallet=pallet_xc_asset_config
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./runtime/mainnet/src/weights/pallet_xc_asset_config.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xc_asset_config`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xc_asset_config::WeightInfo for WeightInfo<T> {
	/// Storage: `XcAssetConfig::AssetIdToLocation` (r:1 w:1)
	/// Proof: `XcAssetConfig::AssetIdToLocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountCodes` (r:0 w:1)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationToId` (r:0 w:1)
	/// Proof: `XcAssetConfig::AssetLocationToId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_asset_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 26_850_000 picoseconds.
		Weight::from_parts(27_466_000, 0)
			.saturating_add(Weight::from_parts(0, 3471))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `XcAssetConfig::AssetLocationToId` (r:1 w:0)
	/// Proof: `XcAssetConfig::AssetLocationToId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationUnitsPerSecond` (r:0 w:1)
	/// Proof: `XcAssetConfig::AssetLocationUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_asset_units_per_second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `93`
		//  Estimated: `3558`
		// Minimum execution time: 23_301_000 picoseconds.
		Weight::from_parts(23_683_000, 0)
			.saturating_add(Weight::from_parts(0, 3558))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcAssetConfig::AssetIdToLocation` (r:1 w:1)
	/// Proof: `XcAssetConfig::AssetIdToLocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationUnitsPerSecond` (r:1 w:2)
	/// Proof: `XcAssetConfig::AssetLocationUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationToId` (r:0 w:2)
	/// Proof: `XcAssetConfig::AssetLocationToId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn change_existing_asset_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141`
		//  Estimated: `3606`
		// Minimum execution time: 35_187_000 picoseconds.
		Weight::from_parts(35_837_000, 0)
			.saturating_add(Weight::from_parts(0, 3606))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `XcAssetConfig::AssetLocationUnitsPerSecond` (r:0 w:1)
	/// Proof: `XcAssetConfig::AssetLocationUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_payment_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_467_000 picoseconds.
		Weight::from_parts(16_781_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcAssetConfig::AssetIdToLocation` (r:1 w:1)
	/// Proof: `XcAssetConfig::AssetIdToLocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountCodes` (r:0 w:1)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationUnitsPerSecond` (r:0 w:1)
	/// Proof: `XcAssetConfig::AssetLocationUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationToId` (r:0 w:1)
	/// Proof: `XcAssetConfig::AssetLocationToId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128`
		//  Estimated: `3593`
		// Minimum execution time: 29_460_000 picoseconds.
		Weight::from_parts(29_706_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
