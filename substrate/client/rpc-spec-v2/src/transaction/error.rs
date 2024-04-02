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

//! Transaction RPC errors.
//!
//! Errors are interpreted as transaction events for subscriptions.

use crate::transaction::event::{TransactionError, TransactionEvent};
use jsonrpsee::types::error::ErrorObject;
use sc_transaction_pool_api::error::Error as PoolError;
use sp_runtime::transaction_validity::InvalidTransaction;

/// Transaction RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Transaction pool error.
	#[error("Transaction pool error: {}", .0)]
	Pool(#[from] PoolError),
	/// Verification error.
	#[error("Extrinsic verification error: {}", .0)]
	Verification(Box<dyn std::error::Error + Send + Sync>),
}

impl<Hash> From<Error> for TransactionEvent<Hash> {
	fn from(e: Error) -> Self {
		match e {
			Error::Verification(e) => TransactionEvent::Invalid(TransactionError {
				error: format!("Verification error: {}", e),
			}),
			Error::Pool(PoolError::InvalidTransaction(InvalidTransaction::Custom(e))) =>
				TransactionEvent::Invalid(TransactionError {
					error: format!("Invalid transaction with custom error: {}", e),
				}),
			Error::Pool(PoolError::InvalidTransaction(e)) => {
				let msg: &str = e.into();
				TransactionEvent::Invalid(TransactionError {
					error: format!("Invalid transaction: {}", msg),
				})
			},
			Error::Pool(PoolError::UnknownTransaction(e)) => {
				let msg: &str = e.into();
				TransactionEvent::Invalid(TransactionError {
					error: format!("Unknown transaction validity: {}", msg),
				})
			},
			Error::Pool(PoolError::TemporarilyBanned) =>
				TransactionEvent::Invalid(TransactionError {
					error: "Transaction is temporarily banned".into(),
				}),
			Error::Pool(PoolError::AlreadyImported(_)) =>
				TransactionEvent::Invalid(TransactionError {
					error: "Transaction is already imported".into(),
				}),
			Error::Pool(PoolError::TooLowPriority { old, new }) =>
				TransactionEvent::Invalid(TransactionError {
					error: format!(
						"The priority of the transaction is too low (pool {} > current {})",
						old, new
					),
				}),
			Error::Pool(PoolError::CycleDetected) => TransactionEvent::Invalid(TransactionError {
				error: "The transaction contains a cyclic dependency".into(),
			}),
			Error::Pool(PoolError::ImmediatelyDropped) =>
				TransactionEvent::Invalid(TransactionError {
					error: "The transaction could not enter the pool because of the limit".into(),
				}),
			Error::Pool(PoolError::Unactionable) => TransactionEvent::Invalid(TransactionError {
				error: "Transaction cannot be propagated and the local node does not author blocks"
					.into(),
			}),
			Error::Pool(PoolError::NoTagsProvided) => TransactionEvent::Invalid(TransactionError {
				error: "Transaction does not provide any tags, so the pool cannot identify it"
					.into(),
			}),
			Error::Pool(PoolError::InvalidBlockId(_)) =>
				TransactionEvent::Invalid(TransactionError {
					error: "The provided block ID is not valid".into(),
				}),
			Error::Pool(PoolError::RejectedFutureTransaction) =>
				TransactionEvent::Invalid(TransactionError {
					error: "The pool is not accepting future transactions".into(),
				}),
		}
	}
}

/// TransactionBroadcast error.
#[derive(Debug, thiserror::Error)]
pub enum ErrorBroadcast {
	/// The provided operation ID is invalid.
	#[error("Invalid operation id")]
	InvalidOperationID,
}

/// General purpose errors, as defined in
/// <https://www.jsonrpc.org/specification#error_object>.
pub mod json_rpc_spec {
	/// Invalid parameter error.
	pub const INVALID_PARAM_ERROR: i32 = -32602;
}

impl From<ErrorBroadcast> for ErrorObject<'static> {
	fn from(e: ErrorBroadcast) -> Self {
		let msg = e.to_string();

		match e {
			ErrorBroadcast::InvalidOperationID =>
				ErrorObject::owned(json_rpc_spec::INVALID_PARAM_ERROR, msg, None::<()>),
		}
	}
}

/// TransactionWatch error.
#[derive(Debug, thiserror::Error)]
pub enum ErrorWatch {
	/// Maximum number of transactionWatch has been reached.
	#[error("Maximum number of transactionWatch has been reached")]
	ReachedLimits,
}

/// Errors for `transactionWatch` RPC module, as defined in
/// <https://github.com/paritytech/json-rpc-interface-spec>.
pub mod rpc_spec_v2 {
	/// Maximum number of chainHead_follow has been reached.
	pub const REACHED_LIMITS: i32 = -32800;
}

impl From<ErrorWatch> for ErrorObject<'static> {
	fn from(e: ErrorWatch) -> Self {
		let msg = e.to_string();

		match e {
			ErrorWatch::ReachedLimits =>
				ErrorObject::owned(rpc_spec_v2::REACHED_LIMITS, msg, None::<()>),
		}
	}
}
