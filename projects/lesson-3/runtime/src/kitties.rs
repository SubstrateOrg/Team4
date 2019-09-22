use support::{decl_module, decl_storage, StorageValue, StorageMap,ensure};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {
}

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
			KittiesCount::put(count + 1);
		}

		///breed a baby kitty
        pub fn breed(origin,cat_id_1:u32,cat_id_2:u32){
            let sender = ensure_signed(origin)?;
            //TODO:确保还能生小猫
            let count = Self::kitties_count();
            if count == u32::max_value() {
                return Err("Kitties count overflow");
            }
            //TODO:确保父母存在
            ensure!(<Kitties>::exists(cat_id_1),"cat_id_1 does not exist");
            ensure!(<Kitties>::exists(cat_id_2),"cat_id_2 does not exist");
            let kitty_1 = Self::kitty(cat_id_1);
            let kitty_2 = Self::kitty(cat_id_2);
            //TODO:生成小猫基因算法(确保dna随机来自父母)
            let payload = (<system::Module<T>>::random_seed(), &sender,<system::Module<T>>::extrinsic_index())
                .using_encoded(blake2_128);
            let mut baby_dna = kitty_1.0;
            for (i,(kitty_2_element,r)) in kitty_2.0.as_ref().iter().zip(payload.as_ref().iter()).enumerate() {
                if r % 2 == 0 {
                    baby_dna.as_mut()[i] = *kitty_2_element;
                }
            }
            //TODO:增加新的小猫
            let baby_cat = Kitty(baby_dna);
            Kitties::insert(count, baby_cat);
            KittiesCount::put(count + 1);
        }

	}
}
