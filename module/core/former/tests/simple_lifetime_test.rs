#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Simple test to isolate the E0106 lifetime issue

use former::Former;

/// Simple test struct with lifetime parameter.
#[ derive( Debug, PartialEq, Former ) ]
pub struct SimpleTest<'a> {
    /// Test data field.
    data: &'a str,
}

#[ test ]
fn simple_test() {
    let input = "test";
    let instance = SimpleTest::former().data(input).form();
    assert_eq!(instance.data, "test");
}