A small utility crate for merging values, mostly useful when dealing with things like configuration files.

Inspired by the [merge](https://crates.io/crates/merge) crate, with a few improvements:

1. If no merging strategy is specified, [`Merge::merge`](crate::Merge::merge) is used.
2. [`Merge::merge`](crate::Merge::merge) is automatically implemented for vectors, BTreeMap/BTreeSet, HashMap/HashSet (both the std and the hashbrown variants for `no_std` support), IndexMap/IndexSet, OrderMap/OrderSet
2. [`Merge::merge`](crate::Merge::merge) is automatically implemented for `Option`, such that an option value is overwritten only if the new value is `Some`.
3. Exports a variety of merging functions, like merging values in maps (rather than overwriting them) or overwriting a value only if the new value is not the default for that type.
4. Allows usage of closures, other than paths, for defining merging strategies at the field level.

```rust
use merge_it::Merge;

fn merge_double(left: &mut Vec<i32>, right: Vec<i32>) {
	left.extend(right.into_iter().map(|num| num * 2));
}

#[derive(Merge, Clone)]
struct Example {
	// Uses `Merge::merge` by default
	simple: Vec<i32>,
	#[merge(with = merge_double)]
	with_fn: Vec<i32>,
	#[merge(with = |left, right| left.push(right[0] * 5))]
	with_closure: Vec<i32>,
	#[merge(skip)]
	skipped: Vec<i32>,
}

#[derive(Merge, Clone)]
// Default logic for all fields
#[merge(with = merge_double)]
struct WithDefault {
	uses_default: Vec<i32>,
	// Can be overridden
	#[merge(with = |left, right| left.push(right[0] * 5))]
	with_override: Vec<i32>,
}

fn main() {
	let mut example = Example {
		simple: vec![1],
		with_fn: vec![1],
		with_closure: vec![1],
		skipped: vec![1],
	};

	example.merge(example.clone());

	assert_eq!(example.simple, [1, 1]);
	assert_eq!(example.with_fn, [1, 2]);
	assert_eq!(example.with_closure, [1, 5]);
	assert_eq!(example.skipped, [1]);

	let mut with_default_example = WithDefault {
		uses_default: vec![1],
		with_override: vec![1],
	};

	with_default_example.merge(with_default_example.clone());

	assert_eq!(with_default_example.uses_default, [1, 2]);
	assert_eq!(with_default_example.with_override, [1, 5]);
}
```
