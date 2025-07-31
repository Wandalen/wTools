// Simple test to isolate the E0106 lifetime issue

use former::Former;

#[derive(Debug, PartialEq, Former)]
pub struct SimpleTest<'a> {
    data: &'a str,
}

#[test]
fn simple_test() {
    let input = "test";
    let instance = SimpleTest::former().data(input).form();
    assert_eq!(instance.data, "test");
}