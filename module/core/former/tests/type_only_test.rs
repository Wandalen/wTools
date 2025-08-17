//! Test for type-only struct with Former derive.

use former::Former;

/// Test struct for type-only Former functionality.
#[ derive( Debug, PartialEq, Former ) ]
pub struct TypeOnlyTest<T> {
    /// Generic data field.
    data: T,
}

#[ test ]
fn test_type_only_struct() {
    let instance: TypeOnlyTest<i32> = TypeOnlyTest::former().data(42i32).form();
    assert_eq!(instance.data, 42);
}