//! Baseline test - same struct without derive macro to ensure it compiles

/// Baseline test struct for comparison.
#[ derive( Debug, PartialEq ) ]
pub struct BaselineTest<'a> {
    /// Test data field.
    data: &'a str,
}

#[ test ]
fn baseline_test() {
    let input = "test";
    let instance = BaselineTest { data: input };
    assert_eq!(instance.data, "test");
}