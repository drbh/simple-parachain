#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Escrow<T: Config> {
	#[codec(compact)]
	pub amount: u64,
	pub recp: AccountIdOf<T>,
	pub timestamp: BlockNumberOf<T>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;
	use sp_runtime::ArithmeticError;

	pub type Balance = u64;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::type_value]
	pub fn CountDefault() -> u64 {
		0_u64
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	#[pallet::getter(fn escrow_count)]
	pub type EscrowsCount<T> =
		StorageValue<Value = u64, QueryKind = ValueQuery, OnEmpty = CountDefault>;

	#[pallet::storage]
	#[pallet::getter(fn escrow)]
	pub type Escrows<T> = StorageMap<_, Blake2_128Concat, u64, Escrow<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		EscrowStarted {
			amount: u64,
			initiator: AccountIdOf<T>,
			recipient: AccountIdOf<T>,
			timestamp: BlockNumberOf<T>,
		},
		EscrowWithdrawn {
			amount: u64,
			recipient: AccountIdOf<T>,
			escrow_timestamp: BlockNumberOf<T>,
			withdrawn_timestamp: BlockNumberOf<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Only legitimate recipient can withdraw.
		SenderIsNotRecipient,
		/// Can not withdraw escrow before time.
		CannotWithdrawBeforeTime,
		/// Cannot withdraw amount grater than escrow amount.
		CannotWithdrawAmountGraterThanEscrow,
		/// No escrow information found with given id.
		NoEscrowFound,
		/// Escrow amount can't be zero.
		EscrowAmountCannotBeZero,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// my custom function
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn start_escrow(
			origin: OriginFor<T>,
			amount: u64,
			recp: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			if amount == 0_u64 {
				return Err(Error::<T>::EscrowAmountCannotBeZero.into())
			}
			let current_block_number = <frame_system::Pallet<T>>::block_number();

			// create new escrow
			let escrow = Escrow { amount, recp: recp.clone(), timestamp: current_block_number };

			let escrow_id = EscrowsCount::<T>::get();

			// insert into memory
			Escrows::<T>::insert(escrow_id, escrow);
			let escrow_count = escrow_id.checked_add(1_u64).ok_or(ArithmeticError::Overflow)?;
			EscrowsCount::<T>::set(escrow_count);

			Self::deposit_event(Event::<T>::EscrowStarted {
				amount,
				initiator: who,
				recipient: recp,
				timestamp: current_block_number,
			});

			Ok(().into())
		}

		/// my custom function
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn withdraw_escrow(
			origin: OriginFor<T>,
			amount: u64,
			escrow_id: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let current_block_number = <frame_system::Pallet<T>>::block_number();

			let escrow = Escrows::<T>::take(escrow_id).ok_or(Error::<T>::NoEscrowFound)?;
			if amount > escrow.amount {
				return Err(Error::<T>::CannotWithdrawAmountGraterThanEscrow.into())
			}

			let allowed_block = escrow.timestamp + 2_u32.into();
			// only allow after atleast 2 blocks are passed
			if current_block_number < allowed_block {
				return Err(Error::<T>::CannotWithdrawBeforeTime.into())
			}

			// check if sender is the correct recp
			match escrow.recp == sender {
				true => {
					let remaining_amount = escrow.amount - amount;
					if remaining_amount != 0 {
						// insert updated escrow details
						let updated_escrow = Escrow { amount: remaining_amount, ..escrow };
						Escrows::<T>::insert(escrow_id, updated_escrow);
					}
					Self::deposit_event(Event::<T>::EscrowWithdrawn {
						amount,
						recipient: sender,
						escrow_timestamp: escrow.timestamp,
						withdrawn_timestamp: current_block_number,
					});
					Ok(().into())
				},
				false => Err(Error::<T>::SenderIsNotRecipient.into()),
			}
		}

		/// mint user some tokens
		#[pallet::weight(10_000)]
		pub fn mint(_origin: OriginFor<T>, _amount: u64) -> DispatchResultWithPostInfo {
			todo!()
		}

		/// get a users balance
		#[pallet::weight(10_000)]
		pub fn balance_of(_origin: OriginFor<T>, _amount: u64) -> DispatchResultWithPostInfo {
			todo!()
		}
	}
	impl<T: Config> Pallet<T> {
		pub fn test_rpc(amount: u128) -> Result<u128, DispatchError> {
			Ok(amount)
		}
	}
}
