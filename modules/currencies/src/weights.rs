// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
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

//! Autogenerated weights for module_currencies
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-17, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-22-123`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_currencies
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/currencies/src/weights.rs
// --template=./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_currencies.
pub trait WeightInfo {
	fn transfer_non_native_currency() -> Weight;
	fn transfer_native_currency() -> Weight;
	fn update_balance_non_native_currency() -> Weight;
	fn update_balance_native_currency_creating() -> Weight;
	fn update_balance_native_currency_killing() -> Weight;
	fn sweep_dust(c: u32, ) -> Weight;
	fn force_set_lock() -> Weight;
	fn force_remove_lock() -> Weight;
}

/// Weights for module_currencies using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_non_native_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2493`
		//  Estimated: `13352`
		// Minimum execution time: 86_216 nanoseconds.
		Weight::from_parts(88_106_000, 13352)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn transfer_native_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1977`
		//  Estimated: `7118`
		// Minimum execution time: 68_140 nanoseconds.
		Weight::from_parts(69_315_000, 7118)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_non_native_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2008`
		//  Estimated: `10737`
		// Minimum execution time: 54_990 nanoseconds.
		Weight::from_parts(55_756_000, 10737)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_native_currency_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1707`
		//  Estimated: `3593`
		// Minimum execution time: 50_095 nanoseconds.
		Weight::from_parts(51_020_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn update_balance_native_currency_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1846`
		//  Estimated: `7118`
		// Minimum execution time: 49_296 nanoseconds.
		Weight::from_parts(50_228_000, 7118)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 3]`.
	fn sweep_dust(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1880 + c * (339 ±0)`
		//  Estimated: `4602 + c * (5225 ±0)`
		// Minimum execution time: 63_930 nanoseconds.
		Weight::from_parts(28_195_038, 4602)
			// Standard Error: 55_030
			.saturating_add(Weight::from_parts(37_716_994, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 5225).saturating_mul(c.into()))
	}
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_set_lock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2209`
		//  Estimated: `11970`
		// Minimum execution time: 56_749 nanoseconds.
		Weight::from_parts(57_522_000, 11970)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_remove_lock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2314`
		//  Estimated: `11970`
		// Minimum execution time: 57_795 nanoseconds.
		Weight::from_parts(58_743_000, 11970)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_non_native_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2493`
		//  Estimated: `13352`
		// Minimum execution time: 86_216 nanoseconds.
		Weight::from_parts(88_106_000, 13352)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn transfer_native_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1977`
		//  Estimated: `7118`
		// Minimum execution time: 68_140 nanoseconds.
		Weight::from_parts(69_315_000, 7118)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_non_native_currency() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2008`
		//  Estimated: `10737`
		// Minimum execution time: 54_990 nanoseconds.
		Weight::from_parts(55_756_000, 10737)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_native_currency_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1707`
		//  Estimated: `3593`
		// Minimum execution time: 50_095 nanoseconds.
		Weight::from_parts(51_020_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn update_balance_native_currency_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1846`
		//  Estimated: `7118`
		// Minimum execution time: 49_296 nanoseconds.
		Weight::from_parts(50_228_000, 7118)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 3]`.
	fn sweep_dust(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1880 + c * (339 ±0)`
		//  Estimated: `4602 + c * (5225 ±0)`
		// Minimum execution time: 63_930 nanoseconds.
		Weight::from_parts(28_195_038, 4602)
			// Standard Error: 55_030
			.saturating_add(Weight::from_parts(37_716_994, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 5225).saturating_mul(c.into()))
	}
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_set_lock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2209`
		//  Estimated: `11970`
		// Minimum execution time: 56_749 nanoseconds.
		Weight::from_parts(57_522_000, 11970)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_remove_lock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2314`
		//  Estimated: `11970`
		// Minimum execution time: 57_795 nanoseconds.
		Weight::from_parts(58_743_000, 11970)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}
