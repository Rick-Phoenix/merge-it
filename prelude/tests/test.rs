use prelude::*;

#[derive(Merge, Default)]
struct Test {
	vec: Vec<i32>,
	#[merge(with = overwrite_if_true)]
	boolean: bool,
}

#[test]
fn basic_test() {
	let mut initial = Test::default();

	let other = Test {
		vec: vec![1, 2, 3],
		boolean: true,
	};

	initial.merge(other);

	assert!(initial.boolean);
	assert_eq!(initial.vec, &[1, 2, 3]);
}
