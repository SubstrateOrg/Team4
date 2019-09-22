use support::{decl_storage, decl_module, StorageValue, StorageMap,
			  dispatch::Result, ensure, decl_event};
use system::ensure_signed;
//use runtime_primitives::traits::{As, Hash};
use sr_primitives::traits::{Hash, SimpleArithmetic,UniqueSaturatedFrom,UniqueSaturatedInto};
use codec::{Decode, Encode};
use runtime_io::{blake2_128};

/*
需求

1 链上存储加密猫数据
将加密猫的数据定义在一个module，放入runtime中即可

2 遍历所有加密猫

3 每只猫都有自己的dna，为128bit的数据，设计如何生成dna

4 每个用户可以拥有零到多只猫，每只猫只有一个主人

5遍历用户拥有的所有加密猫

*/

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
	id: Hash,
//	dna: Hash,
	dna: u128,
	price: Balance,
	gen: u64,
}

pub trait Trait: balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}



decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
        // ACTION: Add the `Balance` trait here
    {
        Created(AccountId, Hash),
        // ACTION: Create a `PriceSet` event here
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Substratekitties {
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(all_kitties_count): u64;
        AllKittiesIndex: map T::Hash => u64;

        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;

        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

	  	fn deposit_event() = default;

		//遍历所有加密猫
	  	fn show_all_kittys(origin) -> Result {
			let sender = ensure_signed(origin)?;

			for n in 0..Self::all_kitties_count()  {
				let kitty_hash = Self::kitty_by_index(n);
				let kitty_entity=Self::kitty(kitty_hash);
			}
			Ok(())
		}

		//遍历用户拥有的所有加密猫
		fn show_kittys_accountId(origin) -> Result {
			let sender = ensure_signed(origin)?;

			let owned_count = Self::owned_kitty_count(&sender);
			for n in 0..owned_count  {
				let kitty_hash = Self::kitty_of_owner_by_index((sender.clone(), n));
				let kitty_entity=Self::kitty(kitty_hash);
			}
			Ok(())
		}

        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

			let dna_random = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(blake2_128);//返回值[u8; 16]

//            println!("Now {:?} will print!", );
//			let dna_random = <T::Balance as UniqueSaturatedInto<u128>>::unique_saturated_into(dna_hash);

            let new_kitty = Kitty {
                id: random_hash,
                dna: dna_random[2] as u128,
//                price: <T::Balance as As<u64>>::sa(0),
                price: <T::Balance as UniqueSaturatedFrom<u64>>::unique_saturated_from(0),
                gen: 0,
            };

            Self::mint(sender, random_hash, new_kitty)?;

            <Nonce>::mutate(|n| *n += 1);

            Ok(())
        }

        // NOTE: We added this `set_price` template for you
        fn set_price(origin, kitty_id: T::Hash, new_price: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Check that the kitty with `kitty_id` exists

            // ACTION: Check if owner exists for `kitty_id`
            //         - If it does, check that `sender` is the `owner`
            //         - If it doesn't, return an `Err()` that no `owner` exists

            let mut kitty = Self::kitty(kitty_id);

            // ACTION: Set the new price for the kitty

            // ACTION: Update the kitty in storage

            // ACTION: Deposit a `PriceSet` event with relevant data
            //         - owner
            //         - kitty id
            //         - the new price

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
	fn mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
		ensure!(!<KittyOwner<T>>::exists(kitty_id), "Kitty already exists");

		let owned_kitty_count = Self::owned_kitty_count(&to);

		let new_owned_kitty_count = owned_kitty_count.checked_add(1)
			.ok_or("Overflow adding a new kitty to account balance")?;

		let all_kitties_count = Self::all_kitties_count();

		let new_all_kitties_count = all_kitties_count.checked_add(1)
			.ok_or("Overflow adding a new kitty to total supply")?;

		<Kitties<T>>::insert(kitty_id, new_kitty);
		<KittyOwner<T>>::insert(kitty_id, &to);

		<AllKittiesArray<T>>::insert(all_kitties_count, kitty_id);
		<AllKittiesCount>::put(new_all_kitties_count);
		<AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);

		<OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count), kitty_id);
		<OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
		<OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);

		Self::deposit_event(RawEvent::Created(to, kitty_id));

		Ok(())
	}
}