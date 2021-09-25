//	当feature不等于std的时候，设置no_std
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    // use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`

    #[pallet::config]  // <-- Step 2. code block will replace this.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    #[pallet::event]   // <-- Step 3. code block will replace this.
    #[pallet::metadata(T::AccountId = "AccountId")]
    // #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a proof has been claimed. [who, claim]
        ClaimCreated(T::AccountId, Vec<u8>),
    /// Event emitted when a claim is revoked by the owner. [who, claim]
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error]   // <-- Step 4. code block will replace this.
    pub enum Error<T> {
		/// The proof has already been claimed.
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,
	}
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // #[pallet::storage]T
    // pub(super) type Thing1<T: Config> = StorageValue<_, u32>;

    // #[pallet::storage]
    // #[pallet::getter(fn thing1)]
    // pub(super) type Thing1<T> = StorageValue<_, u32>;

    #[pallet::storage]
	#[pallet::getter(fn something1)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Thing1<T> = StorageValue<_, u32>;

    #[pallet::storage]
	#[pallet::getter(fn something2)]
	pub type Thing2<T> = StorageValue<_, u32>;
    
    #[pallet::storage] // <-- Step 5. code block will replace this.
    pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;
	
	#[pallet::storage] // <-- Step 5. code block will replace this.
    pub(super) type Wptest<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, ValueQuery>;  

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        fn set_thing_1(
            origin: OriginFor<T>,
            value: u32
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            <Thing1<T>>::put(value);
            //  Self::deposit_event(Event::SetValue(value));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        fn set_thing_2(
            origin: OriginFor<T>,
            value: u32
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            <Thing2<T>>::put(value);
            // Self::deposit_event(Event::SetValue(value));

            Ok(().into())
        }
	}

	impl<T: Config> Pallet<T> {
		pub fn get_sum() -> u32 {
            Thing1::<T>::get().unwrap_or_else(|| 12) + <Thing2<T>>::get().unwrap_or_else(|| 10)
			//  <Thing1<u32>>::get().unwrap_or_else(|| 10) + Thing2::get().unwrap_or_else(|| 10)
		}
	}

    // #[pallet::type_value]
    // pub(super) fn get_sum() -> u32 {
    //     45
    //     // Thing1::get() + Thing2::get()
    // }
}

// //	当feature不等于std的时候，设置no_std
// #![cfg_attr(not(feature = "std"), no_std)]

// // Re-export pallet items so that they can be accessed from the crate namespace.
// pub use pallet::*;

// #[frame_support::pallet]
// pub mod pallet {
//     use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
//     use frame_system::pallet_prelude::*;
//     use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`

//     #[pallet::config]  // <-- Step 2. code block will replace this.
//     pub trait Config: frame_system::Config {
//         /// Because this pallet emits events, it depends on the runtime's definition of an event.
//         type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
//     }
//     #[pallet::event]   // <-- Step 3. code block will replace this.
//     #[pallet::metadata(T::AccountId = "AccountId")]
//     #[pallet::generate_deposit(pub(super) fn deposit_event)]
//     pub enum Event<T: Config> {
//         /// Event emitted when a proof has been claimed. [who, claim]
//         ClaimCreated(T::AccountId, Vec<u8>),
//     /// Event emitted when a claim is revoked by the owner. [who, claim]
//         ClaimRevoked(T::AccountId, Vec<u8>),
//     }
//     #[pallet::error]   // <-- Step 4. code block will replace this.
//     pub enum Error<T> {
// 		/// The proof has already been claimed.
// 		ProofAlreadyClaimed,
// 		/// The proof does not exist, so it cannot be revoked.
// 		NoSuchProof,
// 		/// The proof is claimed by another account, so caller can't revoke it.
// 		NotProofOwner,
// 	}
//     #[pallet::pallet]
//     #[pallet::generate_store(pub(super) trait Store)]
//     pub struct Pallet<T>(_);
    
//     #[pallet::storage] // <-- Step 5. code block will replace this.
//     pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;   
//     #[pallet::hooks]
//     impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
//     // Dispatchable functions allows users to interact with the pallet and invoke state changes.
//     // These functions materialize as "extrinsics", which are often compared to transactions.
//     // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {
// 		#[pallet::weight(1_000)]
// 		pub(super) fn create_claim(
// 			origin: OriginFor<T>,
// 			proof: Vec<u8>,
// 		) -> DispatchResultWithPostInfo {

// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
// 			let sender = ensure_signed(origin)?;
		
// 			// Verify that the specified proof has not already been claimed.         
// 			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

// 			// Get the block number from the FRAME System module.
// 			let current_block = <frame_system::Module<T>>::block_number();

// 			// Store the proof with the sender and block number.
// 			Proofs::<T>::insert(&proof, (&sender, current_block));

// 			// Emit an event that the claim was created.
// 			Self::deposit_event(Event::ClaimCreated(sender, proof));

// 			Ok(().into())
// 		}

// 		#[pallet::weight(10_000)]
// 		fn revoke_claim(
// 			origin: OriginFor<T>,
// 			proof: Vec<u8>,
// 		) -> DispatchResultWithPostInfo {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
// 			let sender = ensure_signed(origin)?;

// 			// Verify that the specified proof has been claimed.
// 			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

// 			// Get owner of the claim.
// 			let (owner, _) = Proofs::<T>::get(&proof);

// 			// Verify that sender of the current call is the claim owner.
// 			ensure!(sender == owner, Error::<T>::NotProofOwner);

// 			// Remove claim from storage.
// 			Proofs::<T>::remove(&proof);

// 			// Emit an event that the claim was erased.
// 			Self::deposit_event(Event::ClaimRevoked(sender, proof));

// 			Ok(().into())
// 		}
// 	}
// }


// #![cfg_attr(not(feature = "std"), no_std)]

// //! A simple pallet with two storage values. The pallet itself does not teach any new concepts.
// //! Rather we use this pallet as demonstration case as we demonstrate custom runtime APIs.
// //! This pallet supports a runtime API which will allow querying the runtime for the sum of
// //! the two storage items.

// use frame_support::{decl_event, decl_module, decl_storage, dispatch};
// use frame_system::ensure_signed;

// // #[cfg(test)]
// // mod tests;

// /// The module's configuration trait.
// pub trait Config: frame_system::Config {
// 	/// The overarching event type.
// 	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
// }

// decl_storage! {
// 	trait Store for Module<T: Config> as SumStorage {
// 		Thing1 get(fn thing1): u32;
// 		Thing2 get(fn thing2): u32;
// 	}
// }

// // The module's dispatchable functions.
// decl_module! {
// 	/// The module declaration.
// 	pub struct Module<T: Config> for enum Call where origin: T::Origin {
// 		fn deposit_event() = default;

// 		/// Sets the first simple storage value
// 		#[weight = 10_000]
// 		pub fn set_thing_1(origin, val: u32) -> dispatch::DispatchResult {
// 			let _ = ensure_signed(origin)?;

// 			Thing1::put(val);

// 			Self::deposit_event(Event::ValueSet(1, val));
// 			Ok(())
// 		}

// 		/// Sets the second stored value
// 		#[weight = 10_000]
// 		pub fn set_thing_2(origin, val: u32) -> dispatch::DispatchResult {
// 			let _ = ensure_signed(origin)?;

// 			Thing2::put(val);

// 			Self::deposit_event(Event::ValueSet(2, val));
// 			Ok(())
// 		}
// 	}
// }

// impl<T: Config> pallet<T> {
// 	pub fn get_sum() -> u32 {
// 		Thing1::get() + Thing2::get()
// 	}
// }

// decl_event!(
// 	pub enum Event {
// 		ValueSet(u32, u32),
// 	}
// );
