#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_runtime::{traits::MaybeDisplay, DispatchError};

sp_api::decl_runtime_apis! {
	pub trait TemplateApi<Balance> where
		Balance: Codec,
	{
		fn test_rpc(amount: Balance) -> Result<Balance, DispatchError>;
	}
}
