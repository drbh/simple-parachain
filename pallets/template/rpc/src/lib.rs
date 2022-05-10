// Copyright 2021 Parallel Finance Developer.
// This file is part of Parallel Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

pub use pallet_template_rpc_runtime_api::TemplateApi as TemplateRuntimeApi;

use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{generic::BlockId, traits::Block as BlockT, FixedU128};
use sp_std::vec::Vec;

use codec::Decode;

#[rpc]
pub trait TemplateApi<BlockHash, Balance>
where
	Balance: Codec + Copy + TryFrom<NumberOrHex>,
{
	#[rpc(name = "template_testRpc")]
	fn test_rpc(&self, amount: NumberOrHex, at: Option<BlockHash>) -> Result<NumberOrHex>;
}

/// A struct that implements the [`RouteApi`].
pub struct Api<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> Api<C, B> {
	/// Create new `Route` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

pub enum Error {
	RuntimeError,
	TemplateApiError,
	InvalidParams,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::TemplateApiError => 2,
			Error::InvalidParams => 3,
		}
	}
}

impl<C, Block, Balance> TemplateApi<<Block as BlockT>::Hash, Balance> for Api<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: TemplateRuntimeApi<Block, Balance>,
	Balance: Codec + Copy + TryFrom<NumberOrHex> + Into<NumberOrHex> + std::fmt::Display,
{
	fn test_rpc(
		&self,
		amount: NumberOrHex,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<NumberOrHex> {
		let api = self.client.runtime_api();
		let at: BlockId<Block> = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

		let try_into_rpc_balance = |value: Balance| {
			value.try_into().map_err(|_| RpcError {
				code: ErrorCode::InvalidParams,
				message: format!("{} doesn't fit in NumberOrHex representation", value),
				data: None,
			})
		};

		let res = api
			.test_rpc(&at, decode_hex(amount, "balance")?)
			.map_err(runtime_error_into_rpc_error)?
			.map_err(test_rpc_error)?;

		try_into_rpc_balance(res)
	}
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_error(err: impl std::fmt::Debug) -> RpcError {
	RpcError {
		code: ErrorCode::ServerError(Error::RuntimeError.into()),
		message: "Runtime trapped".into(),
		data: Some(format!("{:?}", err).into()),
	}
}

fn test_rpc_error(err: impl std::fmt::Debug) -> RpcError {
	RpcError {
		code: ErrorCode::ServerError(Error::TemplateApiError.into()),
		message: "test rpc error".into(),
		data: Some(format!("{:?}", err).into()),
	}
}

fn decode_hex<H: std::fmt::Debug + Copy, T: TryFrom<H>>(from: H, name: &str) -> Result<T> {
	from.try_into().map_err(|_| RpcError {
		code: ErrorCode::InvalidParams,
		message: format!("{:?} does not fit into the {} type", from, name),
		data: None,
	})
}

fn try_into_rpc_balance(value: u128) -> Result<NumberOrHex> {
	value.try_into().map_err(|_| RpcError {
		code: ErrorCode::InvalidParams,
		message: format!("{} doesn't fit in NumberOrHex representation", value),
		data: None,
	})
}
