use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap,dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

//TODO:定义猫结构体
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct KittyCat {
    cat_id: u32,
    dna: u128,
}


decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		//TODO: 当前catId
		CatId get(cat_id_getter): u32;
		//TODO: map (catId,cat) array[catIds]
		CatMap get(cat_map_getter): map u32 => KittyCat;
		//TODO: map(catId,accountId)
		CatOwner get(cat_owner_getter): map u32 => T::AccountId;
		//TODO: map(accountId,array[catIds])
		OwnerCats get(owner_cats_getter): map T::AccountId => array:[u32];
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		//TODO:遍历加密猫
		fn getAllCats(origin) -> Result {

		}

		//TODO:createCat(dna随机数算法)
		fn createCat(origin, hash: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            let curCatId = Self::cat_id_getter();

            //TODO:dna生成算法
			let nonce = <Nonce<T>>::get();
			let random_seed = <system::Module<T>>::random_seed();
			//TODO：hash类型转换为u128？
			let random_hash = (random_seed, sender, hash).using_encoded(<T as system::Trait>::Hashing::hash);

            let newCat = KittyCat {
    			cat_id: curCatId,
    			dna: random_hash,
            };

            <CatMap<T>>::insert(curCatId, newCat);
            <CatId<T>>::put(curCatId + 1);
            Ok(())
        }

		//TODO:获取当前用户拥有的猫ids
		fn getOwnerCats(origin) -> Result {
			
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
	}
);
