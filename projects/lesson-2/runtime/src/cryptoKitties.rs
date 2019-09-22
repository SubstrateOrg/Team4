use support::{decl_module, decl_storage,StorageValue, StorageMap,dispatch::Vec};
use codec::{Encode,Decode};
use system::ensure_signed;
use runtime_io::blake2_128;

pub trait Trait: system::Trait {
}

///定义猫结构体
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct KittyCat {
    pub cat_id: u32,
    pub dna: [u8; 16],
}


decl_storage! {
	trait Store for Module<T: Trait> as CryptoKitties {
		///当前catId
		pub CatId get(cat_id_getter): u32;
		///Cat Map
		pub CatMap get(cat_map_getter): map u32 => KittyCat;
		///Map(catId,accountId)
		pub CatOwner get(cat_owner_getter): map u32 => T::AccountId;
		///Map(accountId,Vec[catIds])
		pub OwnerCats get(owner_cats_getter): map T::AccountId => Vec<u32>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// fn deposit_event() = default;

		///create_cat
		fn create_cat(origin){
            let sender = ensure_signed(origin)?;
            let curCatId = Self::cat_id_getter();

            //dna生成算法
			let random_seed = <system::Module<T>>::random_seed();
			let extrinsic_index = <system::Module<T>>::extrinsic_index();
			let dna = (random_seed, sender, extrinsic_index).using_encoded(blake2_128);

            let newCat = KittyCat {
    			cat_id: curCatId,
    			dna: dna,
            };

            <CatMap>::insert(curCatId, newCat);
            <CatId>::put(curCatId + 1);
        }
	}
}
