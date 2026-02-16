use merge_it::*;

#[derive(Merge, Default)]
struct Test {
	vec: Vec<i32>,
	#[merge(with = overwrite_if_true)]
	boolean: bool,
	#[merge(with = |left, right| *left = right)]
	with_closure: String,
}

#[test]
fn basic_test() {
	let mut initial = Test::default();

	let other = Test {
		vec: vec![1, 2, 3],
		boolean: true,
		with_closure: "abc".into(),
	};

	initial.merge(other);

	assert!(initial.boolean);
	assert_eq!(initial.vec, &[1, 2, 3]);
	assert_eq!(initial.with_closure, "abc");
}

fn merge_double(left: &mut Vec<i32>, right: Vec<i32>) {
	left.extend(right.into_iter().map(|n| n * 2));
}

#[derive(Merge, Debug, Clone, PartialEq)]
enum TestEnum {
	#[merge(with = merge_double)]
	Path(Vec<i32>),

	#[merge(with = |left, right| *left = Some(right.unwrap_or_default() * 2))]
	Closure(Option<i32>),
}

#[test]
fn enum_test() {
	let mut test_enum = TestEnum::Path(vec![1]);

	test_enum.merge(test_enum.clone());
	assert_eq!(test_enum, TestEnum::Path(vec![1, 2]));

	test_enum = TestEnum::Closure(None);
	test_enum.merge(TestEnum::Closure(Some(1)));
	assert_eq!(test_enum, TestEnum::Closure(Some(2)));
}
