// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Shareable Substrate traits.

#[cfg(feature = "std")]
use crate::{crypto::KeyTypeId, ed25519, sr25519};

/// Something that generates, stores and provides access to keys.
#[cfg(feature = "std")]
pub trait BareCryptoStore: Send + Sync {
	/// Returns all sr25519 public keys for the given key type.
	fn sr25519_public_keys(&self, id: KeyTypeId) -> Vec<sr25519::Public>;
	/// Generate a new sr25519 key pair for the given key type and an optional seed.
	///
	/// If the given seed is `Some(_)`, the key pair will only be stored in memory.
	///
	/// Returns the public key of the generated key pair.
	fn sr25519_generate_new(
		&mut self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> Result<sr25519::Public, String>;
	/// Returns the sr25519 key pair for the given key type and public key combination.
	fn sr25519_key_pair(&self, id: KeyTypeId, pub_key: &sr25519::Public) -> Option<sr25519::Pair>;

	/// Returns all ed25519 public keys for the given key type.
	fn ed25519_public_keys(&self, id: KeyTypeId) -> Vec<ed25519::Public>;
	/// Generate a new ed25519 key pair for the given key type and an optional seed.
	///
	/// If the given seed is `Some(_)`, the key pair will only be stored in memory.
	///
	/// Returns the public key of the generated key pair.
	fn ed25519_generate_new(
		&mut self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> Result<ed25519::Public, String>;

	/// Returns the ed25519 key pair for the given key type and public key combination.
	fn ed25519_key_pair(&self, id: KeyTypeId, pub_key: &ed25519::Public) -> Option<ed25519::Pair>;

	/// Insert a new key. This doesn't require any known of the crypto; but a public key must be
	/// manually provided.
	///
	/// Places it into the file system store.
	///
	/// `Err` if there's some sort of weird filesystem error, but should generally be `Ok`.
	fn insert_unknown(&mut self, _key_type: KeyTypeId, _suri: &str, _public: &[u8]) -> Result<(), ()>;

	/// Get the password for this store.
	fn password(&self) -> Option<&str>;
}

/// A pointer to the key store.
#[cfg(feature = "std")]
pub type BareCryptoStorePtr = std::sync::Arc<parking_lot::RwLock<dyn BareCryptoStore>>;
