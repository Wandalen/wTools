#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Test if derive macros work with lifetime-only structs

#![allow(unused_imports)]

use former as the_module;

/// Test struct for minimal derive functionality.
#[ derive( Debug, PartialEq, Clone ) ]
pub struct MinimalTest<'a> {
    /// Test data field.
    data: &'a str,
}

#[ test ]
fn minimal_test() {
    let input = "test";
    let instance = MinimalTest { data: input };
    let cloned = instance.clone();
    assert_eq!(instance.data, cloned.data);
}