#![no_std]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//!
//! ## Features
#![cfg_attr(
		feature = "document-features",
		doc = ::document_features::document_features!()
)]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

/// A trait for merging values.
///
/// Implemented by default for most collections and for [`Option`].
pub trait Merge<Other = Self> {
	fn merge(&mut self, other: Other);
}

#[doc(inline)]
pub use merge_it_derive::Merge;

/// Merges two [`Option`]s of values that implement [`Merge`] themselves.
#[inline]
pub fn merge_options<T: Merge>(left: &mut Option<T>, right: Option<T>) {
	if let Some(right) = right {
		if let Some(left) = left {
			left.merge(right);
		} else {
			*left = Some(right);
		}
	}
}

/// When merging, it overwrites the previous value only if the new value is `false`
#[inline]
pub fn overwrite_if_false(left: &mut bool, right: bool) {
	*left &= right;
}

/// When merging, it overwrites the previous value only if the new value is `true`
#[inline]
pub fn overwrite_if_true(left: &mut bool, right: bool) {
	*left |= right;
}

/// When merging, it always overwrites the previous value
#[inline]
pub fn overwrite_always<T>(left: &mut T, right: T) {
	*left = right;
}

/// When merging, overwrite the previous value only if the new value is NOT the default
#[inline]
pub fn overwrite_if_not_default<T: Default + PartialEq>(left: &mut T, right: T) {
	if right != T::default() {
		*left = right;
	}
}

/// When merging [`Option`]s, overwrite the previous value only if the new value is None
///
/// This overrides the default merging behaviour for [`Option`], which is to overwrite only if the new value is [`Some`].
#[inline]
pub fn overwrite_if_none<T>(left: &mut Option<T>, right: Option<T>) {
	if right.is_none() {
		*left = None;
	}
}

impl<T> Merge for Option<T> {
	#[inline]
	fn merge(&mut self, other: Self) {
		if let Some(other) = other {
			*self = Some(other);
		}
	}
}

#[cfg(feature = "alloc")]
pub use alloc_impls::*;

#[cfg(feature = "alloc")]
mod alloc_impls {
	use super::*;

	use alloc::{
		boxed::Box,
		collections::{BTreeMap, BTreeSet, btree_map::Entry},
		vec::Vec,
	};

	impl<T: Merge> Merge<Box<T>> for Box<T> {
		#[inline]
		fn merge(&mut self, other: Self) {
			T::merge(self, *other);
		}
	}

	impl<T: Merge> Merge<T> for Box<T> {
		#[inline]
		fn merge(&mut self, other: T) {
			T::merge(self, other);
		}
	}

	impl<I, T> Merge<I> for Vec<T>
	where
		I: IntoIterator<Item = T>,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	impl<I, T> Merge<I> for BTreeSet<T>
	where
		I: IntoIterator<Item = T>,
		T: Ord,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	impl<I, K, V> Merge<I> for BTreeMap<K, V>
	where
		I: IntoIterator<Item = (K, V)>,
		K: Ord,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	/// Deeply merges maps with values that implement [`Merge`].
	///
	/// If a value is already present in the map, it is not replaced but merged with the new value.
	pub fn merge_btree_maps<K, V>(left: &mut BTreeMap<K, V>, right: BTreeMap<K, V>)
	where
		K: Ord,
		V: Merge,
	{
		for (key, val) in right {
			match left.entry(key) {
				Entry::Vacant(vacant) => {
					vacant.insert(val);
				}
				Entry::Occupied(mut occupied) => {
					occupied.get_mut().merge(val);
				}
			};
		}
	}
}

#[cfg(feature = "indexmap")]
pub use indexmap_impls::*;

#[cfg(feature = "indexmap")]
mod indexmap_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use indexmap::{IndexMap, IndexSet, map::Entry};

	impl<I, T, S> Merge<I> for IndexSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	impl<I, K, V, S> Merge<I> for IndexMap<K, V, S>
	where
		I: IntoIterator<Item = (K, V)>,
		K: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	/// Deeply merges maps with values that implement [`Merge`].
	///
	/// If a value is already present in the map, it is not replaced but merged with the new value.
	pub fn merge_index_maps<K, V, S>(left: &mut IndexMap<K, V, S>, right: IndexMap<K, V, S>)
	where
		K: Eq + Hash,
		V: Merge,
		S: BuildHasher,
	{
		for (key, val) in right {
			match left.entry(key) {
				Entry::Vacant(vacant) => {
					vacant.insert(val);
				}
				Entry::Occupied(mut occupied) => {
					occupied.get_mut().merge(val);
				}
			};
		}
	}
}

#[cfg(feature = "ordermap")]
pub use ordermap_impls::*;

#[cfg(feature = "ordermap")]
mod ordermap_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use ordermap::{OrderMap, OrderSet, map::Entry};

	impl<I, T, S> Merge<I> for OrderSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	impl<I, K, V, S> Merge<I> for OrderMap<K, V, S>
	where
		I: IntoIterator<Item = (K, V)>,
		K: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	/// Deeply merges maps with values that implement [`Merge`].
	///
	/// If a value is already present in the map, it is not replaced but merged with the new value.
	pub fn merge_order_maps<K, V, S>(left: &mut OrderMap<K, V, S>, right: OrderMap<K, V, S>)
	where
		K: Eq + Hash,
		V: Merge,
		S: BuildHasher,
	{
		for (key, val) in right {
			match left.entry(key) {
				Entry::Vacant(vacant) => {
					vacant.insert(val);
				}
				Entry::Occupied(mut occupied) => {
					occupied.get_mut().merge(val);
				}
			};
		}
	}
}

#[cfg(feature = "std")]
pub use std_impls::*;

#[cfg(feature = "std")]
mod std_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use std::collections::{HashMap, HashSet, hash_map::Entry};

	impl<I, T, S> Merge<I> for HashSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	impl<I, K, V, S> Merge<I> for HashMap<K, V, S>
	where
		I: IntoIterator<Item = (K, V)>,
		K: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	/// Deeply merges maps with values that implement [`Merge`].
	///
	/// If a value is already present in the map, it is not replaced but merged with the new value.
	pub fn merge_hash_maps<K, V, S>(left: &mut HashMap<K, V, S>, right: HashMap<K, V, S>)
	where
		K: Eq + Hash,
		V: Merge,
		S: BuildHasher,
	{
		for (key, val) in right {
			match left.entry(key) {
				Entry::Vacant(vacant) => {
					vacant.insert(val);
				}
				Entry::Occupied(mut occupied) => {
					occupied.get_mut().merge(val);
				}
			};
		}
	}
}

#[cfg(feature = "hashbrown")]
pub use hashbrown_impls::*;

#[cfg(feature = "hashbrown")]
mod hashbrown_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use hashbrown::{HashMap, HashSet, hash_map::Entry};

	impl<I, T, S> Merge<I> for HashSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	impl<I, K, V, S> Merge<I> for HashMap<K, V, S>
	where
		I: IntoIterator<Item = (K, V)>,
		K: Eq + Hash,
		S: BuildHasher,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}

	/// Deeply merges maps with values that implement [`Merge`].
	///
	/// If a value is already present in the map, it is not replaced but merged with the new value.
	pub fn merge_hashbrown_maps<K, V, S>(left: &mut HashMap<K, V, S>, right: HashMap<K, V, S>)
	where
		K: Eq + Hash,
		V: Merge,
		S: BuildHasher,
	{
		for (key, val) in right {
			match left.entry(key) {
				Entry::Vacant(vacant) => {
					vacant.insert(val);
				}
				Entry::Occupied(mut occupied) => {
					occupied.get_mut().merge(val);
				}
			};
		}
	}
}

#[doc(hidden)]
#[inline]
pub fn __apply<I, Other, O, F>(input: &mut I, other: Other, f: F) -> O
where
	F: FnOnce(&mut I, Other) -> O,
{
	f(input, other)
}
