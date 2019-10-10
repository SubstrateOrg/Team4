use codec::{Decode, Encode};
use sr_primitives::traits::Member;
use support::{Parameter, StorageMap};

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct LinkedItem<Value> {
	pub prev: Option<Value>,
	pub next: Option<Value>,
}

pub struct LinkedList<Storage, Key, Value>(rstd::marker::PhantomData<(Storage, Key, Value)>);

impl<Storage, Key, Value> LinkedList<Storage, Key, Value>
where
	Value: Parameter + Member + Copy,
	Key: Parameter,
	Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query = Option<LinkedItem<Value>>>,
{
	fn read_head(key: &Key) -> LinkedItem<Value> {
		Self::read(key, None)
	}

	fn write_head(account: &Key, item: LinkedItem<Value>) {
		Self::write(account, None, item);
	}

	fn read(key: &Key, value: Option<Value>) -> LinkedItem<Value> {
		Storage::get(&(key.clone(), value)).unwrap_or_else(|| LinkedItem {
			prev: None,
			next: None,
		})
	}

	fn write(key: &Key, value: Option<Value>, item: LinkedItem<Value>) {
		Storage::insert(&(key.clone(), value), item);
	}

	pub fn append(key: &Key, value: Value) {
		let mut head = Self::read_head(key);
		let new_item = LinkedItem {
			prev: head.prev,
			next: Option::None,
		};
		Self::write(key, Option::Some(value), new_item);

		match head.prev {
			Some(v) => {
				let mut last = Self::read(key, head.prev);
				head.prev = Option::Some(value);
				last.next = Option::Some(value);
				Self::write(key, Some(v), last);
				Self::write_head(key, head);
			}
			None => {
				head.prev = Option::Some(value);
				head.next = Option::Some(value);
				Self::write_head(key, head);
			}
		}
	}

	pub fn remove(key: &Key, value: Value) {
		let item = Self::read(key, Option::Some(value));
		if item.prev == item.next {
			let mut head = Self::read(key, item.prev);
			head.next = Option::None;
			head.prev = Option::None;
			Self::write_head(key, head);
		} else {
			let mut prev = Self::read(key, item.prev);
			let mut next = Self::read(key, item.next);
			prev.next = item.next;
			next.prev = item.prev;
			Self::write(key, item.prev, prev);
			Self::write(key, item.next, next);
		}
		Storage::remove(&(key.clone(), Option::Some(value)));
	}
}
