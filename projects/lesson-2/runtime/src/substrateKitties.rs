/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
/// 

/// requirements:
/// - 链上存储加密猫数据
/// - 遍历所有加密猫
/// - 每只猫都有⾃己的dna，为 128bit的数据, 伪代码算法
/// - 每个用户可以拥有零到多只猫
/// - 遍历用户拥有的所有猫

use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;
use sr_primitives::traits::{Hash};
use codec::{Encode, Decode};

pub struct Kitty<Hash, U64> {
    id: Hash,
    dna: Hash,
    price: U64,
    generation: U64,
}

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


decl_storage! {
	trait Store for Module<T: Trait> as KittiesModule {
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		// Something get(something): Option<u32>;

        Kitties get(kitty): map T::Hash => Kitty<T::Hash, u64>;
        KittyOwner get(owner): map T::Hash => Option<T::AccountId>;

        KittiesArrayByOwner get(kitty_by_owner_index): map (T::AccountId, u64) => T::Hash;
        KittiesCountByOwner get(all_kitty_count_by_onwer): map T::AccountId => u64;
        KittiesIndexByOwner: map T::Hash => u64;
     
        AllKittiesArray get(kitty_with_indexes): map u64 => T::Hash;
        AllKittiesCount get(all_kitty_count): u64;
        AllKittiesIndex: map T::Hash => u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn create_kitty(origin, ) -> Result {
			let who = ensure_signed(origin)?;

			// TODO: u128
			let ramdom_hash = (<system::Module<T>>::random_seed(), &who, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty exists already!!");

			let counted_kitty_by_owner = Self::all_kitty_count_by_onwer(&who);

		    let new_all_kitty_count_by_owner = counted_kitty_by_owner.checked_add(1)
			    .ok_or("Overflow adding a new kitty !!")?;

			let all_kitty_count = Self::all_kitty_count();

			let new_all_kitty_count = all_kitty_count.checked_add(1)
				.ok_or("Overflow adding a new kitty to total supply!!!")?;

			let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: 0,
                generation: 0,
            };	

			<Kitties<T>>::insert(ramdom_hash, new_kitty);
			<KittyOwner<T>>::insert(ramdom_hash, &who);

			<KittiesArrayByOwner<T>>::insert((who.clone(), counted_kitty_by_owner), ramdom_hash);
			<KittiesCountByOwner<T>>::insert(&who, new_all_kitty_count_by_owner);
			<KittiesIndexByOwner<T>>::insert(random_hash, counted_kitty_by_owner);

			<AllKittiesArray<T>>::insert(all_kitty_count, random_hash);
			<AllKittiesCount>::put(new_all_kitty_count);
			<AllKittiesIndex<T>>::insert(kitty_id, all_kitty_count);

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			// Something::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::KittyStored(who,random_hash));
			Ok(())
		}

        // Iterate all kitties
		fn get_all_kittys(origin) -> Result {
		let who = ensure_signed(origin)?;

		for n in 0..Self::all_kitty_count()  {
			let kitty_id = Self::kitty_by_owner_index(n);
			let kitty =Self::kitty(kitty_id);
			println!("kitty");
		}
		Ok(())
	}

	    // Interate all kitties by owner 
		fn get_kitties_accountId(origin) -> Result {
			let who = ensure_signed(origin)?;

			let owned_count = Self::all_kitty_count_by_onwer(&who);
			for n in 0..owned_count  {
				let kitty_id = Self::kitty_by_owner_index((who.clone(), n));
				let kitty =Self::kitty(kitty_id);
			}
			Ok(())
		}

	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// TODO: to sort out the event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		KittyStored(AccountId, Hash),
	}
);

decl_error!();

/// TODO: Add test 
/// tests for this module
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
