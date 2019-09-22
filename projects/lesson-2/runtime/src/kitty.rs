/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, ensure, dispatch::Result};
use sr_primitives::traits::{Hash};
use codec::{Encode, Decode};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// Kitty type
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash> {
	pub id: Hash
}

/// index of the list
type Index = u64;

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
		/// kitty of a given id
		Kitties get(kitty_id): map T::Hash => Kitty<T::Hash>;

		/// list of all kitties (include add)
		KittiesList get(kitty_index): map Index => T::Hash;
		KittiesCounter get(kitties_counter): Index;
		
		/// list of all kitties for a given owner (include add and remove)
		KittiesOfOwnerList get(kitty_of_owner_index): map (T::AccountId, Index) => T::Hash;
		KittiesOfOwnerCounter get(kitties_of_owner_counter): map T::AccountId => Index;

		Nonce: Index;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		fn deposit_event() = default;

		/// create kitty
		fn create(origin) -> Result {
			let owner = ensure_signed(origin)?;

			// generate kitty id
			let kitty_id = Self::generate_kitty_id(&owner);
			ensure!(!<Kitties<T>>::exists(kitty_id), "Kitty already exists");

			// create new kitty instance
			let new_kitty = Kitty { id: kitty_id };

			// add kitty to kitties map
			<Kitties<T>>::insert(kitty_id, new_kitty);

			// add kitty to kitties list
			let kitties_counter = Self::kitties_counter();
			let kitty_index = kitties_counter.checked_add(1)
				.ok_or("Overflow when adding a new kitty to total count")?;
			<KittiesList<T>>::insert(kitty_index, kitty_id);

			// add kitty to owner list
			let kitties_owner_counter = Self::kitties_of_owner_counter(&owner);
			let owner_kitty_index = kitties_owner_counter.checked_add(1)
				.ok_or("Overflow when adding a new kitty to owner list")?;
			<KittiesOfOwnerList<T>>::insert((owner.clone(), owner_kitty_index), kitty_id);

			// increased nonce value
			<Nonce>::mutate(|n| *n += 1);

			// send event
			// Self::deposit_event(RawEvent::Created(kitty_id, origin));

			Ok(())
		}
	}
}

impl <T: Trait> Module<T> {
	pub fn generate_kitty_id(owner: &T::AccountId) -> T::Hash {
		let nonce = <Nonce>::get();
		let random_hash = (<system::Module<T>>::random_seed(), owner, nonce)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		random_hash
	}
}

decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
	{
		/// Kitty created (kitty_id, owner).
		Created(Hash, AccountId),
	}
);
