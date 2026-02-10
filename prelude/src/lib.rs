#![no_std]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

pub trait Merge<Other = Self> {
	fn merge(&mut self, other: Other);
}

#[doc(inline)]
pub use proc_macro_impls::Merge;

#[inline]
pub fn overwrite_if_false(left: &mut bool, right: bool) {
	*left &= right;
}

#[inline]
pub fn overwrite_if_true(left: &mut bool, right: bool) {
	*left |= right;
}

#[inline]
pub fn overwrite_always<T>(left: &mut T, right: T) {
	*left = right;
}

#[inline]
pub fn overwrite_if_not_default<T: Default + PartialEq>(left: &mut T, right: T) {
	if right != T::default() {
		*left = right;
	}
}

#[inline]
pub fn overwrite_if_none<T>(left: &mut Option<T>, right: Option<T>) {
	if right.is_none() {
		*left = None;
	}
}

impl<T> Merge for Option<T> {
	#[inline]
	fn merge(&mut self, other: Self) {
		if self.is_none()
			&& let Some(other) = other
		{
			*self = Some(other);
		}
	}
}

#[cfg(feature = "alloc")]
mod alloc_impls {
	use super::*;

	use alloc::{
		collections::{BTreeMap, BTreeSet},
		vec::Vec,
	};

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
}

#[cfg(feature = "indexmap")]
mod indexmap_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use indexmap::{IndexMap, IndexSet};

	impl<I, T, S> Merge<I> for IndexSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher + Default,
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
		S: BuildHasher + Default,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}
}

#[cfg(feature = "ordermap")]
mod ordermap_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use ordermap::{OrderMap, OrderSet};

	impl<I, T, S> Merge<I> for OrderSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher + Default,
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
		S: BuildHasher + Default,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}
}

#[cfg(feature = "std")]
mod std_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use std::collections::{HashMap, HashSet};

	impl<I, T, S> Merge<I> for HashSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher + Default,
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
		S: BuildHasher + Default,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
		}
	}
}

#[cfg(feature = "hashbrown")]
mod hashbrown_impls {
	use super::*;

	use core::hash::{BuildHasher, Hash};
	use hashbrown::{HashMap, HashSet};

	impl<I, T, S> Merge<I> for HashSet<T, S>
	where
		I: IntoIterator<Item = T>,
		T: Eq + Hash,
		S: BuildHasher + Default,
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
		S: BuildHasher + Default,
	{
		#[inline]
		fn merge(&mut self, other: I) {
			self.extend(other);
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
