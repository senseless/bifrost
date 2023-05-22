// This file is part of Bifrost.

// Copyright (C) 2019-2022 Liebi Technologies (UK) Ltd.
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
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bifrost_salp_lite
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `VM-16-3-ubuntu`, CPU: `Intel(R) Xeon(R) Platinum 8374C CPU @ 2.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_salp_lite
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./weight.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for bifrost_salp_lite.
pub trait WeightInfo {
	fn redeem() -> Weight;
	fn refund() -> Weight;
	fn set_multisig_confirm_account() -> Weight;
	fn issue() -> Weight;
	fn fund_success() -> Weight;
	fn fund_fail() -> Weight;
	fn continue_fund() -> Weight;
	fn fund_retire() -> Weight;
	fn create() -> Weight;
	fn edit() -> Weight;
	fn withdraw() -> Weight;
	fn dissolve_refunded() -> Weight;
	fn dissolve() -> Weight;
}

/// Weights for bifrost_salp_lite using the Bifrost node and recommended hardware.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BifrostWeight<T> {
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite RedeemPool (r:1 w:1)
	/// Proof Skipped: SalpLite RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	fn redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1979`
		//  Estimated: `21594`
		// Minimum execution time: 103_822_000 picoseconds.
		Weight::from_parts(106_370_000, 21594)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: SalpLite FailedFundsToRefund (r:1 w:0)
	/// Proof Skipped: SalpLite FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite RedeemPool (r:1 w:1)
	/// Proof Skipped: SalpLite RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1998`
		//  Estimated: `26124`
		// Minimum execution time: 112_549_000 picoseconds.
		Weight::from_parts(115_798_000, 26124)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: SalpLite MultisigConfirmAccount (r:0 w:1)
	/// Proof Skipped: SalpLite MultisigConfirmAccount (max_values: Some(1), max_size: None, mode: Measured)
	fn set_multisig_confirm_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_535_000 picoseconds.
		Weight::from_parts(4_670_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite MultisigConfirmAccount (r:1 w:0)
	/// Proof Skipped: SalpLite MultisigConfirmAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn issue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1704`
		//  Estimated: `24948`
		// Minimum execution time: 100_711_000 picoseconds.
		Weight::from_parts(103_325_000, 24948)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 21_801_000 picoseconds.
		Weight::from_parts(22_528_000, 2733)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 21_972_000 picoseconds.
		Weight::from_parts(22_642_000, 2733)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite FailedFundsToRefund (r:0 w:1)
	/// Proof Skipped: SalpLite FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	fn continue_fund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3057`
		// Minimum execution time: 27_479_000 picoseconds.
		Weight::from_parts(28_432_000, 3057)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_retire() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 21_578_000 picoseconds.
		Weight::from_parts(22_711_000, 2733)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite CurrentTrieIndex (r:1 w:1)
	/// Proof Skipped: SalpLite CurrentTrieIndex (max_values: Some(1), max_size: None, mode: Measured)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `3198`
		// Minimum execution time: 20_075_000 picoseconds.
		Weight::from_parts(20_658_000, 3198)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn edit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 19_699_000 picoseconds.
		Weight::from_parts(20_115_000, 2733)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite RedeemPool (r:1 w:1)
	/// Proof Skipped: SalpLite RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `3486`
		// Minimum execution time: 24_497_000 picoseconds.
		Weight::from_parts(25_339_000, 3486)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: SalpLite FailedFundsToRefund (r:1 w:1)
	/// Proof Skipped: SalpLite FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	fn dissolve_refunded() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `368`
		//  Estimated: `2843`
		// Minimum execution time: 28_628_000 picoseconds.
		Weight::from_parts(29_220_000, 2843)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: unknown `0x` (r:1 w:0)
	/// Proof Skipped: unknown `0x` (r:1 w:0)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn dissolve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `8760`
		// Minimum execution time: 37_507_000 picoseconds.
		Weight::from_parts(39_793_000, 8760)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite RedeemPool (r:1 w:1)
	/// Proof Skipped: SalpLite RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	fn redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1979`
		//  Estimated: `21594`
		// Minimum execution time: 103_822_000 picoseconds.
		Weight::from_parts(106_370_000, 21594)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: SalpLite FailedFundsToRefund (r:1 w:0)
	/// Proof Skipped: SalpLite FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite RedeemPool (r:1 w:1)
	/// Proof Skipped: SalpLite RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1998`
		//  Estimated: `26124`
		// Minimum execution time: 112_549_000 picoseconds.
		Weight::from_parts(115_798_000, 26124)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: SalpLite MultisigConfirmAccount (r:0 w:1)
	/// Proof Skipped: SalpLite MultisigConfirmAccount (max_values: Some(1), max_size: None, mode: Measured)
	fn set_multisig_confirm_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_535_000 picoseconds.
		Weight::from_parts(4_670_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite MultisigConfirmAccount (r:1 w:0)
	/// Proof Skipped: SalpLite MultisigConfirmAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn issue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1704`
		//  Estimated: `24948`
		// Minimum execution time: 100_711_000 picoseconds.
		Weight::from_parts(103_325_000, 24948)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 21_801_000 picoseconds.
		Weight::from_parts(22_528_000, 2733)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 21_972_000 picoseconds.
		Weight::from_parts(22_642_000, 2733)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite FailedFundsToRefund (r:0 w:1)
	/// Proof Skipped: SalpLite FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	fn continue_fund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3057`
		// Minimum execution time: 27_479_000 picoseconds.
		Weight::from_parts(28_432_000, 3057)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_retire() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 21_578_000 picoseconds.
		Weight::from_parts(22_711_000, 2733)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite CurrentTrieIndex (r:1 w:1)
	/// Proof Skipped: SalpLite CurrentTrieIndex (max_values: Some(1), max_size: None, mode: Measured)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `3198`
		// Minimum execution time: 20_075_000 picoseconds.
		Weight::from_parts(20_658_000, 3198)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	fn edit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `2733`
		// Minimum execution time: 19_699_000 picoseconds.
		Weight::from_parts(20_115_000, 2733)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SalpLite RedeemPool (r:1 w:1)
	/// Proof Skipped: SalpLite RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `3486`
		// Minimum execution time: 24_497_000 picoseconds.
		Weight::from_parts(25_339_000, 3486)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: SalpLite FailedFundsToRefund (r:1 w:1)
	/// Proof Skipped: SalpLite FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	fn dissolve_refunded() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `368`
		//  Estimated: `2843`
		// Minimum execution time: 28_628_000 picoseconds.
		Weight::from_parts(29_220_000, 2843)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SalpLite Funds (r:1 w:1)
	/// Proof Skipped: SalpLite Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: unknown `0x` (r:1 w:0)
	/// Proof Skipped: unknown `0x` (r:1 w:0)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn dissolve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `8760`
		// Minimum execution time: 37_507_000 picoseconds.
		Weight::from_parts(39_793_000, 8760)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
