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

//! Autogenerated weights for orml_oracle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 35.0.1
//! DATE: 2024-04-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-40-129`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for orml_oracle.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_oracle::WeightInfo for WeightInfo<T> {
	// Storage: `AcalaOracle::HasDispatched` (r:1 w:1)
	// Proof: `AcalaOracle::HasDispatched` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	// Storage: `Timestamp::Now` (r:1 w:0)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	// Storage: `OperatorMembershipAcala::Members` (r:1 w:0)
	// Proof: `OperatorMembershipAcala::Members` (`max_values`: Some(1), `max_size`: Some(1601), added: 2096, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::Values` (r:4 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::RawValues` (r:0 w:4)
	// Proof: `AcalaOracle::RawValues` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 4]`.
	fn feed_values(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1204 + c * (16 ±0)`
		//  Estimated: `3086 + c * (2550 ±0)`
		// Minimum execution time: 15_849 nanoseconds.
		Weight::from_parts(17_688_161, 3086)
			// Standard Error: 41_289
			.saturating_add(Weight::from_parts(5_801_886, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2550).saturating_mul(c.into()))
	}
	// Storage: `AcalaOracle::HasDispatched` (r:0 w:1)
	// Proof: `AcalaOracle::HasDispatched` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `840`
		//  Estimated: `0`
		// Minimum execution time: 6_200 nanoseconds.
		Weight::from_parts(6_437_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
