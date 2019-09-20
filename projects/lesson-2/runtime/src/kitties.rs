/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue, StorageMap, ensure};
use system::ensure_signed;
use sr_primitives::traits::{Hash};
use codec::{Decode, Encode};
use runtime_io::blake2_128;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Encode, Decode, Default, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, U64> {
    id: Hash,
    dna: u128,
    gen: U64,
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
    // TODO: Add other types and constants required configure this module.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as KittiesStorage {
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, u64>;
        KittyOwner get(owner): map T::Hash => Option<T::AccountId>;

        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(all_kitty_count): u64;
        AllKittiesIndex: map T::Hash => u64;

        KittiesByOwner get(kitty_by_owner): map (T::AccountId, u64) => T::Hash;
        KittiesCountByOwner get(kitty_count_by_onwer): map T::AccountId => u64;
        KittiesIndexByOwner: map T::Hash => u64;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        pub fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let kitty_hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), sender.clone())
                        .using_encoded(<T as system::Trait>::Hashing::hash);
            
            //convert to u128
            let dna_buff = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), sender.clone())
                        .using_encoded(blake2_128);
            let dna = LittleEndian::read_u128(&dna_buff);

            ensure!(!<KittyOwner<T>>::exists(kitty_hash), "Kitty with same dna exists already!!");

            let all_kitty_count = Self::all_kitty_count();
            let new_all_kitty_count = all_kitty_count.checked_add(1).ok_or("Overflow adding a new one to all")?;

            let kitty_count_by_owner = Self::kitty_count_by_onwer(&sender);
            let new_kitty_count_by_owner = kitty_count_by_owner.checked_add(1).ok_or("Overflow adding a new one")?;

            let new_kitty = Kitty {
                id: kitty_hash,
                dna: dna,
                gen: 0,
            };	

            <Kitties<T>>::insert(kitty_hash, new_kitty);
            <KittyOwner<T>>::insert(kitty_hash, &sender);

            <AllKittiesArray<T>>::insert(all_kitty_count, kitty_hash);
			<AllKittiesCount>::put(new_all_kitty_count);
			<AllKittiesIndex<T>>::insert(kitty_hash, all_kitty_count);

            <KittiesByOwner<T>>::insert((sender.clone(), kitty_count_by_owner), kitty_hash);
			<KittiesCountByOwner<T>>::insert(&sender, new_kitty_count_by_owner);
			<KittiesIndexByOwner<T>>::insert(kitty_hash, kitty_count_by_owner);

            // Raise event for the Kitty Stored
            Self::deposit_event(RawEvent::KittyStored(sender, dna));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        KittyStored(AccountId, u128),
    }
);

// TODO /// tests for this module
// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	use runtime_io::with_externalities;
// 	use primitives::{H256, Blake2Hasher};
// 	use support::{impl_outer_origin, assert_ok, parameter_types};
// 	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
// 	use sr_primitives::weights::Weight;
// 	use sr_primitives::Perbill;

// 	impl_outer_origin! {
// 		pub enum Origin for Test {}
// 	}

// 	// For testing the module, we construct most of a mock runtime. This means
// 	// first constructing a configuration type (`Test`) which `impl`s each of the
// 	// configuration traits of modules we want to use.
// 	#[derive(Clone, Eq, PartialEq)]
// 	pub struct Test;
// 	parameter_types! {
// 		pub const BlockHashCount: u64 = 250;
// 		pub const MaximumBlockWeight: Weight = 1024;
// 		pub const MaximumBlockLength: u32 = 2 * 1024;
// 		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
// 	}
// 	impl system::Trait for Test {
// 		type Origin = Origin;
// 		type Call = ();
// 		type Index = u64;
// 		type BlockNumber = u64;
// 		type Hash = H256;
// 		type Hashing = BlakeTwo256;
// 		type AccountId = u64;
// 		type Lookup = IdentityLookup<Self::AccountId>;
// 		type Header = Header;
// 		type WeightMultiplierUpdate = ();
// 		type Event = ();
// 		type BlockHashCount = BlockHashCount;
// 		type MaximumBlockWeight = MaximumBlockWeight;
// 		type MaximumBlockLength = MaximumBlockLength;
// 		type AvailableBlockRatio = AvailableBlockRatio;
// 		type Version = ();
// 	}
// 	impl Trait for Test {
// 		type Event = ();
// 	}
// 	type TemplateModule = Module<Test>;

// 	// This function basically just builds a genesis storage key/value store according to
// 	// our desired mockup.
// 	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
// 		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// 	}

// 	#[test]
// 	fn it_works_for_default_value() {
// 		with_externalities(&mut new_test_ext(), || {
// 			// Just a dummy test for the dummy funtion `do_something`
// 			// calling the `do_something` function with a value 42
// 			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 			// asserting that the stored value is equal to what we stored
// 			assert_eq!(TemplateModule::something(), Some(42));
// 		});
// 	}
// }
