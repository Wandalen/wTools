use former::Former;

#[derive(Debug, PartialEq, Former)]
pub struct TypeOnlyTest<T> {
    data: T,
}

#[test]
fn test_type_only_struct() {
    let instance: TypeOnlyTest<i32> = TypeOnlyTest::former().data(42i32).form();
    assert_eq!(instance.data, 42);
}