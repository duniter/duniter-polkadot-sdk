// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Test that `construct_runtime!` also works when `frame-support` or `frame-system` are renamed in
//! the `Cargo.toml`.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, Everything},
};
use sp_core::{sr25519, H256};
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentityLookup, Verify},
};
use sp_version::RuntimeVersion;

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("frame-support-test-compile-pass"),
	impl_name: alloc::borrow::Cow::Borrowed("substrate-frame-support-test-compile-pass-runtime"),
	authoring_version: 0,
	spec_version: 0,
	impl_version: 0,
	apis: sp_version::create_apis_vec!([]),
	transaction_version: 0,
	system_version: 0,
};

pub type Signature = sr25519::Signature;
pub type AccountId = <Signature as Verify>::Signer;
pub type BlockNumber = u64;

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Nonce = u128;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Block = Block;
	type Lookup = IdentityLookup<Self::AccountId>;
	type BlockHashCount = ConstU64<2400>;
	type Version = Version;
	type AccountData = ();
	type RuntimeOrigin = RuntimeOrigin;
	type AccountId = AccountId;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type DbWeight = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<0>;
}

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, RuntimeCall, Signature, ()>;

construct_runtime!(
	pub enum Runtime {
		System: frame_system,
	}
);
