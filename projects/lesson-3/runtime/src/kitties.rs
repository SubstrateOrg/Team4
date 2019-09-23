use support::{decl_module, decl_storage, StorageValue, StorageMap, ensure};
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
		
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();

			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}

			let payload = (
				<system::Module<T>>::random_seed(),
				sender, 
				<system::Module<T>>::extrinsic_index(),
				<system::Module<T>>::block_number()
			);
			/// do a Blake2 128-bit hash encoded and return result `[u8; 16]`.
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);

			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}

		pub fn create_from(origin, f_kitty_id: u32, m_kitty_id: u32) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();

			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}

			ensure!(<Kitties>::exists(f_kitty_id), "f_kitty id doesn't exist");
			ensure!(<Kitties>::exists(m_kitty_id), "m_kitty id doesn't exist");

			let payload = (
				<system::Module<T>>::random_seed(),
				sender,
				Self::kitty(f_kitty_id),
				Self::kitty(m_kitty_id),
				<system::Module<T>>::extrinsic_index(),
				<system::Module<T>>::block_number()
			);

			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);

			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}
	}
}