// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_cdp_engine
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-26, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_cdp_engine
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/cdp-engine/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_cdp_engine.
pub trait WeightInfo {
	fn on_initialize(c: u32) -> Weight;
	fn set_collateral_params() -> Weight;
	fn liquidate_by_auction(b: u32) -> Weight;
	fn liquidate_by_dex() -> Weight;
	fn settle() -> Weight;
	fn register_liquidation_contract() -> Weight;
	fn deregister_liquidation_contract() -> Weight;
}

/// Weights for module_cdp_engine using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn on_initialize(c: u32) -> Weight {
		Weight::from_parts(33_360_000, 0)
			.saturating_add(Weight::from_parts(23_139_000, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	fn set_collateral_params() -> Weight {
		Weight::from_parts(37_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn liquidate_by_auction(_b: u32) -> Weight {
		Weight::from_parts(203_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(28 as u64))
			.saturating_add(T::DbWeight::get().writes(17 as u64))
	}
	fn liquidate_by_dex() -> Weight {
		Weight::from_parts(252_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(29 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
	fn settle() -> Weight {
		Weight::from_parts(97_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	fn register_liquidation_contract() -> Weight {
		Weight::from_parts(97_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	fn deregister_liquidation_contract() -> Weight {
		Weight::from_parts(97_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize(c: u32) -> Weight {
		Weight::from_parts(33_360_000, 0)
			.saturating_add(Weight::from_parts(23_139_000, 0).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	fn set_collateral_params() -> Weight {
		Weight::from_parts(37_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn liquidate_by_auction(_b: u32) -> Weight {
		Weight::from_parts(203_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(28 as u64))
			.saturating_add(RocksDbWeight::get().writes(17 as u64))
	}
	fn liquidate_by_dex() -> Weight {
		Weight::from_parts(252_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(29 as u64))
			.saturating_add(RocksDbWeight::get().writes(15 as u64))
	}
	fn settle() -> Weight {
		Weight::from_parts(97_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	fn register_liquidation_contract() -> Weight {
		Weight::from_parts(97_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}

	fn deregister_liquidation_contract() -> Weight {
		Weight::from_parts(97_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
}
