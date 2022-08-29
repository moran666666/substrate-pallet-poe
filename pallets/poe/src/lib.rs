#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MaxBytesInHash: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		ClaimTransfered(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
	}

	#[pallet::storage]
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxBytesInHash>, (T::AccountId, T::BlockNumber), OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxBytesInHash>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Pallet<T>>::block_number();

			Proofs::<T>::insert(&proof, (&sender, current_block));

			Self::deposit_event(Event::ClaimCreated(sender, proof));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			receiver: T::AccountId,
			proof: BoundedVec<u8, T::MaxBytesInHash>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			let current_block = <frame_system::Pallet<T>>::block_number();
			// Proofs::<T>::insert(&proof, (&receiver, current_block)); // 直接覆盖原值的形式修改
			Proofs::<T>::mutate(&proof, |value| { // 修改方法修改
				value.as_mut().unwrap().0 = receiver.clone();
				value.as_mut().unwrap().1 = current_block;
			});

			Self::deposit_event(Event::ClaimTransfered(sender, receiver, proof));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxBytesInHash>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");

			ensure!(sender == owner, Error::<T>::NotProofOwner);

			Proofs::<T>::remove(&proof);

			Self::deposit_event(Event::ClaimRevoked(sender, proof));
			Ok(())
		}
	}
}