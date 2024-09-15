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

//! Autogenerated weights for module_session_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 35.0.1
//! DATE: 2024-04-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-41-141`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_session_manager.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_session_manager::WeightInfo for WeightInfo<T> {
	// Storage: `Session::CurrentIndex` (r:1 w:0)
	// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `SessionManager::SessionDuration` (r:1 w:0)
	// Proof: `SessionManager::SessionDuration` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `SessionManager::DurationOffset` (r:1 w:0)
	// Proof: `SessionManager::DurationOffset` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `SessionManager::SessionDurationChanges` (r:0 w:1)
	// Proof: `SessionManager::SessionDurationChanges` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn schedule_session_duration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1556`
		//  Estimated: `3041`
		// Minimum execution time: 19_464 nanoseconds.
		Weight::from_parts(20_302_000, 3041)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `SessionManager::SessionDurationChanges` (r:1 w:1)
	// Proof: `SessionManager::SessionDurationChanges` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn on_initialize_skip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `809`
		//  Estimated: `3485`
		// Minimum execution time: 5_555 nanoseconds.
		Weight::from_parts(5_978_000, 3485)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `SessionManager::SessionDurationChanges` (r:1 w:1)
	// Proof: `SessionManager::SessionDurationChanges` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	// Storage: `SessionManager::DurationOffset` (r:0 w:1)
	// Proof: `SessionManager::DurationOffset` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `SessionManager::SessionDuration` (r:0 w:1)
	// Proof: `SessionManager::SessionDuration` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `809`
		//  Estimated: `3485`
		// Minimum execution time: 6_151 nanoseconds.
		Weight::from_parts(6_415_000, 3485)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `SessionManager::DurationOffset` (r:1 w:0)
	// Proof: `SessionManager::DurationOffset` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `SessionManager::SessionDuration` (r:1 w:0)
	// Proof: `SessionManager::SessionDuration` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn estimate_current_session_progress() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769`
		//  Estimated: `1489`
		// Minimum execution time: 4_135 nanoseconds.
		Weight::from_parts(4_346_000, 1489)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: `SessionManager::DurationOffset` (r:1 w:0)
	// Proof: `SessionManager::DurationOffset` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `SessionManager::SessionDuration` (r:1 w:0)
	// Proof: `SessionManager::SessionDuration` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn estimate_next_session_rotation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769`
		//  Estimated: `1489`
		// Minimum execution time: 4_172 nanoseconds.
		Weight::from_parts(4_407_000, 1489)
			.saturating_add(T::DbWeight::get().reads(2))
	}
}
