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

//! State RPC errors.

use jsonrpsee::types::error::{ErrorObject, ErrorObjectOwned};

/// State RPC Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// State RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Client error.
	#[error("Client error: {}", .0)]
	Client(#[from] Box<dyn std::error::Error + Send + Sync>),
	/// Provided block range couldn't be resolved to a list of blocks.
	#[error("Cannot resolve a block range ['{:?}' ... '{:?}]. {}", .from, .to, .details)]
	InvalidBlockRange {
		/// Beginning of the block range.
		from: String,
		/// End of the block range.
		to: String,
		/// Details of the error message.
		details: String,
	},
	/// Provided count exceeds maximum value.
	#[error("count exceeds maximum value. value: {}, max: {}", .value, .max)]
	InvalidCount {
		/// Provided value
		value: u32,
		/// Maximum allowed value
		max: u32,
	},
	/// Call to an unsafe RPC was denied.
	#[error(transparent)]
	UnsafeRpcCalled(#[from] crate::policy::UnsafeRpcError),
	/// A legacy unpaged storage-key query returned too many results.
	#[error("storage key result exceeds maximum value. max: {max}; use state_getKeysPaged")]
	TooManyStorageKeys {
		/// Maximum number of results returned by an unpaged query.
		max: usize,
	},
	/// Runtime API call data exceeds the public RPC limit.
	#[error("runtime API call data exceeds maximum size. value: {value}, max: {max}")]
	RuntimeCallDataTooLarge {
		/// Provided call data size.
		value: usize,
		/// Maximum allowed call data size.
		max: usize,
	},
	/// All runtime API execution slots are occupied.
	#[error("too many concurrent runtime API calls")]
	TooManyRuntimeCalls,
}

/// Base code for all state errors.
const BASE_ERROR: i32 = crate::error::base::STATE;

impl From<Error> for ErrorObjectOwned {
	fn from(e: Error) -> ErrorObjectOwned {
		match e {
			Error::InvalidBlockRange { .. } => {
				ErrorObject::owned(BASE_ERROR + 1, e.to_string(), None::<()>)
			},
			Error::InvalidCount { .. } => {
				ErrorObject::owned(BASE_ERROR + 2, e.to_string(), None::<()>)
			},
			e => ErrorObject::owned(BASE_ERROR + 3, e.to_string(), None::<()>),
		}
	}
}
