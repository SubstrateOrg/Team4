use codec::{Decode, Encode};
use runtime_io::blake2_128;
use support::{decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {}

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        /// Stores all the kitties, key is the kitty id / index
        pub Kitties get(kitty): map u32 => Kitty;
        /// Stores the total number of kitties. i.e. the next kitty index
        pub KittiesCount get(kitties_count): u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Create a new kitty
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;
            let count = Self::kitties_count();
            if count == u32::max_value() {
                return Err("Kitties count overflow");
            }
            let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
            let dna = payload.using_encoded(blake2_128);
            let kitty = Kitty(dna);
            Kitties::insert(count, kitty);

            //checked_add
            let new_count = count.checked_add(1).ok_or("Overflow creating a new one to all")?;
            KittiesCount::put(new_count);
        }

        /// Breed a baby kitty
        pub fn breed_kitty(origin, kitty_id_1:u32, kitty_id_2:u32) -> Result {
            let sender = ensure_signed(origin)?;

            let count = Self::kitties_count();
            if count == u32::max_value() {
                return Err("Kitties count overflow");
            }

            ensure!(<Kitties>::exists(kitty_id_1), "Kitty_1 not exist");
            ensure!(<Kitties>::exists(kitty_id_2), "Kitty_2 not exist");
            ensure!(kitty_id_1!=kitty_id_2, "Kitty cound not be breeded from same parents");

            let Kitty(kitty_1_dna) = <Kitties>::get(kitty_id_1);
            let Kitty(kitty_2_dna) = <Kitties>::get(kitty_id_2);

            //add random hash
            let random_hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), sender.clone()).using_encoded(blake2_128);
            let mut final_dna = kitty_1_dna;
            for (i, (dna_2_element, r)) in kitty_2_dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate() {
                if r % 2 == 0 {
                    final_dna.as_mut()[i] = *dna_2_element;
                }
            }

            let kitty = Kitty(final_dna);
            Kitties::insert(count, kitty);

            //checked_add
            let new_count = count.checked_add(1).ok_or("Overflow breedig a new one to all")?;
            KittiesCount::put(new_count);

            Ok(())
        }
    }
}
