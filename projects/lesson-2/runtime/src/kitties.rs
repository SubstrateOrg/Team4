use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    dna: Hash,
    price: Balance,
}

pub trait Trait: balances::Trait{}

decl_storage! {
    trait Store for Module<T: Trait> as kittyStorage {
        Kitties get(kitty):map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        KittyArray get(kitty_index):map u64 => T::Hash;
        KittyCount get(kitty_count): u64;
        KittyIndex: map T::Hash => u64;

        KittyOwnersArray get(kitty_owner_index): map(T::AccountId, u64) => T::Hash;
        KittyOwnedCount get(owned_kitty_count): map T::AccountId => u64;
        KittyOwnedIndex: map T::Hash => u64;

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_value(origin) -> Result {
            let _sender = ensure_signed(origin)?;

            let owned_kitty_count = Self::owned_kitty_count(&_sender);
            let new_owned_kitty_count = owned_kitty_count.checked_add(1).ok_or("failed to add all count for own");

            let kitty_count = Self::kitty_count();
            let new_kitty_count = kitty_count.checked_add(1).ok_or("failed to add all count");

            let random_hash = (<system::Module<T>>::random_seed(), &_sender, timestamp, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

            let new_kitty = Kitty {
                dna: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
            };

            <Kitties<T>>::insert(random_hash, new_kitty);
            <KittyOwner<T>>::insert(random_hash, &_sender);

            <KittyArray<T>>::insert(kitty_count, random_hash);
            <KittyCount<T>>::put(new_kitty_count);
            <KittyIndex<T>>::insert(random_hash, kitty_count);

            <KittyOwnersArray<T>>::insert((_sender.clone(), owned_kitty_count), random_hash);
            <KittyOwnedCount<T>>::insert(&_sender, new_owned_kitty_count);
            <KittyOwnedIndex<T>>::insert(random_hash, owned_kitty_count);

            Ok(())
	    }
    }
}