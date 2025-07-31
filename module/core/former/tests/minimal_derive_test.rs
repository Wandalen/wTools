// Test if derive macros work with lifetime-only structs

#[derive(Debug, PartialEq, Clone)]
pub struct MinimalTest<'a> {
    data: &'a str,
}

#[test]
fn minimal_test() {
    let input = "test";
    let instance = MinimalTest { data: input };
    let cloned = instance.clone();
    assert_eq!(instance.data, cloned.data);
}