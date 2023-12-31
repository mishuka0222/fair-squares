// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_housing_fund
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-07, STEPS: `130`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: ``, CPU: ``
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/fs-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_housing_fund
// --extrinsic
// *
// --steps
// 130
// --repeat
// 40
// --output
// pallets/housing_fund/src/weights.rs
// --template
// assets/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_housing_fund.
pub trait WeightInfo {
	fn contribute_to_fund() -> Weight;
	fn withdraw_fund() -> Weight;
	fn house_bidding() -> Weight;
}

/// Weights for pallet_housing_fund using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: RoleModule InvestorLog (r:1 w:0)
	// Storage: HousingFundModule FundBalance (r:1 w:1)
	// Storage: HousingFundModule Contributions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn contribute_to_fund() -> Weight {
		Weight::from_ref_time(71_245_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: RoleModule InvestorLog (r:1 w:0)
	// Storage: HousingFundModule Contributions (r:1 w:1)
	// Storage: HousingFundModule FundBalance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn withdraw_fund() -> Weight {
		Weight::from_ref_time(86_203_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: HousingFundModule FundBalance (r:1 w:1)
	// Storage: HousingFundModule Contributions (r:10 w:10)
	// Storage: System Account (r:1 w:1)
	// Storage: HousingFundModule Reservations (r:0 w:1)
	fn house_bidding() -> Weight {
		Weight::from_ref_time(168_208_000_u64)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(13_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: RoleModule InvestorLog (r:1 w:0)
	// Storage: HousingFundModule FundBalance (r:1 w:1)
	// Storage: HousingFundModule Contributions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn contribute_to_fund() -> Weight {
		Weight::from_ref_time(71_245_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: RoleModule InvestorLog (r:1 w:0)
	// Storage: HousingFundModule Contributions (r:1 w:1)
	// Storage: HousingFundModule FundBalance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn withdraw_fund() -> Weight {
		Weight::from_ref_time(86_203_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: HousingFundModule FundBalance (r:1 w:1)
	// Storage: HousingFundModule Contributions (r:10 w:10)
	// Storage: System Account (r:1 w:1)
	// Storage: HousingFundModule Reservations (r:0 w:1)
	fn house_bidding() -> Weight {
		Weight::from_ref_time(168_208_000_u64)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(13_u64))
	}
}