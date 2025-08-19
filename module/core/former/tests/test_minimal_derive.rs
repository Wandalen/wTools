#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Test if the issue is with derive mechanism itself

// Try with a proc macro that generates nothing
// extern crate former_meta; // Unused - commented out

/// Test struct for working derive functionality.
#[ derive( Debug, PartialEq ) ]
pub struct WorkingTest<'a> {
    /// Test data field.
    data: &'a str,
}

// Now try with a custom proc macro - but we need to create it in a separate crate
// For now, let's test if the issue persists even with an empty generated result

#[ test ]
fn working_test() {
    let input = "test";
    let instance = WorkingTest { data: input };
    assert_eq!(instance.data, "test");
}