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

//! Autogenerated weights for module_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-46-101`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex::WeightInfo for WeightInfo<T> {
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn enable_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1072`
		//  Estimated: `3660`
		// Minimum execution time: 21_663 nanoseconds.
		Weight::from_parts(22_378_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn disable_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1109`
		//  Estimated: `3660`
		// Minimum execution time: 22_573 nanoseconds.
		Weight::from_parts(23_068_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:1 w:0)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	fn list_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1114`
		//  Estimated: `10823`
		// Minimum execution time: 29_699 nanoseconds.
		Weight::from_parts(30_679_000, 10823)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn update_provisioning_parameters() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `915`
		//  Estimated: `3660`
		// Minimum execution time: 14_085 nanoseconds.
		Weight::from_parts(14_492_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Dex InitialShareExchangeRates (r:0 w:1)
	// Proof: Dex InitialShareExchangeRates (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	fn end_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1706`
		//  Estimated: `17988`
		// Minimum execution time: 57_514 nanoseconds.
		Weight::from_parts(58_533_000, 17988)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	fn add_provision() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1683`
		//  Estimated: `17118`
		// Minimum execution time: 90_657 nanoseconds.
		Weight::from_parts(92_335_000, 17118)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:2 w:1)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	// Storage: Dex InitialShareExchangeRates (r:1 w:1)
	// Proof: Dex InitialShareExchangeRates (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_dex_share() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2060`
		//  Estimated: `23350`
		// Minimum execution time: 79_355 nanoseconds.
		Weight::from_parts(81_859_000, 23350)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2121`
		//  Estimated: `26757`
		// Minimum execution time: 111_424 nanoseconds.
		Weight::from_parts(114_259_000, 26757)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	fn add_liquidity_and_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2528`
		//  Estimated: `41365`
		// Minimum execution time: 158_892 nanoseconds.
		Weight::from_parts(163_324_000, 41365)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1950`
		//  Estimated: `19572`
		// Minimum execution time: 104_369 nanoseconds.
		Weight::from_parts(107_247_000, 19572)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn remove_liquidity_by_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2468`
		//  Estimated: `40188`
		// Minimum execution time: 174_245 nanoseconds.
		Weight::from_parts(177_943_000, 40188)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:3 w:3)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1626 + u * (112 ±0)`
		//  Estimated: `17078 + u * (1270 ±19)`
		// Minimum execution time: 86_437 nanoseconds.
		Weight::from_parts(65_172_443, 17078)
			// Standard Error: 72_350
			.saturating_add(Weight::from_parts(12_364_670, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 1270).saturating_mul(u.into()))
	}
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:3 w:3)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_target(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1626 + u * (112 ±0)`
		//  Estimated: `17078 + u * (1270 ±18)`
		// Minimum execution time: 85_960 nanoseconds.
		Weight::from_parts(65_844_974, 17078)
			// Standard Error: 78_321
			.saturating_add(Weight::from_parts(12_115_363, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 1270).saturating_mul(u.into()))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex InitialShareExchangeRates (r:1 w:0)
	// Proof: Dex InitialShareExchangeRates (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Proof: Dex ProvisioningPool (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn refund_provision() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2122`
		//  Estimated: `24234`
		// Minimum execution time: 93_450 nanoseconds.
		Weight::from_parts(96_209_000, 24234)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	fn abort_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1246`
		//  Estimated: `3660`
		// Minimum execution time: 27_463 nanoseconds.
		Weight::from_parts(28_248_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
