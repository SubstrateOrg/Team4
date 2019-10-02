use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, dispatch::Result, Parameter};
use sr_primitives::traits::{SimpleArithmetic, Bounded};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use rstd::result;

pub trait Trait: system::Trait {
	type KittyIndex: Parameter + SimpleArithmetic + Bounded + Default + Copy;
}

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map T::KittyIndex => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): T::KittyIndex;

		/// Get kitty ID by account ID and user kitty index
		pub OwnedKitties get(owned_kitties): map (T::AccountId, T::KittyIndex) => T::KittyIndex;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(owned_kitties_count): map T::AccountId => T::KittyIndex;
		/// Get index of kitties
		pub OwnedKittiesIndex get(kitty_index): map T::KittyIndex => T::KittyIndex;

		/// Get AccountId By kittyIndex
		pub AccountIdKitty get(kitty_owner): map T::KittyIndex => T::AccountId;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;
			let dna = Self::random_value(&sender);
			Self::insert_kitty(sender,kitty_id,Kitty(dna));
		}

		/// Breed kitties
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

		/// Transfer kitties
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) -> Result {
            let sender = ensure_signed(origin)?;
            let owner = Self::kitty_owner(kitty_id);
            ensure!(owner == sender, "You do not own this kitty");

            Self::transfer_from(sender, to, kitty_id)?;

            Ok(())
        }
		
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	((selector & dna1) | (!selector & dna2))
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
	}

	fn next_kitty_id() -> result::Result<T::KittyIndex, &'static str> {
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err("Kitties count overflow");
		}
		Ok(kitty_id)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
		// Create and store kitty
		<Kitties<T>>::insert(kitty_id, kitty);
		<KittiesCount<T>>::put(kitty_id + 1.into());

		// Store the kitty owner information
		<AccountIdKitty<T>>::insert(kitty_id,owner.clone());

		// Store the ownership information
		let user_kitties_id = Self::owned_kitties_count(owner.clone());
		<OwnedKitties<T>>::insert((owner.clone(), user_kitties_id), kitty_id);
		<OwnedKittiesCount<T>>::insert(owner, user_kitties_id + 1.into());
		<OwnedKittiesIndex<T>>::insert(kitty_id,user_kitties_id);
	}

	fn do_breed(sender: T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> Result {
		let kitty1 = Self::kitty(kitty_id_1);
		let kitty2 = Self::kitty(kitty_id_2);

		ensure!(kitty1.is_some(), "Invalid kitty_id_1");
		ensure!(kitty2.is_some(), "Invalid kitty_id_2");
		ensure!(kitty_id_1 != kitty_id_2, "Needs different parent");

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.unwrap().0;
		let kitty2_dna = kitty2.unwrap().0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(())
	}

	fn transfer_from(from: T::AccountId, to: T::AccountId, kitty_id: T::KittyIndex) -> Result {

        let owned_kitties_count_from = Self::owned_kitties_count(&from);
        let owned_kitties_index_from = Self::kitty_index(kitty_id);
        let owned_kitties_count_to = Self::owned_kitties_count(&to);

        let new_owned_kitties_count_to = owned_kitties_count_to + 1.into();

        let new_owned_kitties_count_from = owned_kitties_count_from - 1.into();

        <AccountIdKitty<T>>::insert(&kitty_id, &to);

        <OwnedKitties<T>>::remove((from.clone(), owned_kitties_index_from));
        <OwnedKitties<T>>::insert((to.clone(), owned_kitties_count_to),kitty_id);

        <OwnedKittiesIndex<T>>::insert(&kitty_id, owned_kitties_count_to);

        <OwnedKittiesCount<T>>::insert(&from, new_owned_kitties_count_from);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitties_count_to);

        Ok(())
    }
}
