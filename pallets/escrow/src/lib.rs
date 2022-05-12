#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
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
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::DispatchResultWithPostInfo;

	pub type Balance = u64;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn escrow)]
	pub type Escrows<T> = StorageMap<_, Blake2_128Concat, u64, Escrow<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(().into())
				},
			}
		}

		/// my custom function
		#[pallet::weight(10_000)]
		pub fn start_escrow(
			origin: OriginFor<T>,
			amount: u64,
			recp: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;

			// create new escrow
			let escrow = Escrow { amount, recp };

			// insert into memory
			Escrows::<T>::insert(0, escrow.clone());

			Ok(().into())
		}

		/// my custom function
		#[pallet::weight(10_000)]
		pub fn withdraw_escrow(
			origin: OriginFor<T>,
			amount: u64,
			escrow_id: u64,
		) -> DispatchResultWithPostInfo {
			let _sender = ensure_signed(origin)?;
			let _current_block_number = <frame_system::Pallet<T>>::block_number();

			// get the escrow as a mutable ref
			let mut escrow = Escrows::<T>::get(escrow_id).ok_or(Error::<T>::NoneValue)?;

			// check if sender is the correct recp
			match escrow.recp == escrow.recp {
				true => {
					escrow.amount = escrow.amount - amount;
					Ok(().into())
				},
				false => Err(Error::<T>::NoneValue.into()),
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
			return Ok(amount)
		}
	}
}
