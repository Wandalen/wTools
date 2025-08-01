//! Test to isolate proc macro issue with lifetime-only structs

// Custom attribute macro that does nothing - just to test the issue
// use former::Former; // Unused - commented out

/// Test struct without derive to ensure compilation works.
#[allow(dead_code)]
#[derive(Debug)]
pub struct WorksWithoutDerive<'a> {
    /// Test data field.
    data: &'a str,
}

/// Test struct with standard derives.
#[derive(Debug, Clone)]
pub struct WorksWithStandardDerives<'a> {
    /// Test data field.
    data: &'a str,
}

// This fails - our custom Former derive
// #[derive(Former)]
// pub struct FailsWithFormerDerive<'a> {
//     data: &'a str,
// }

#[test]
fn test_standard_derives_work() {
    let data = "test";
    let instance = WorksWithStandardDerives { data };
    let _cloned = instance.clone();
    // Standard derives work fine with lifetime-only structs
    assert_eq!(_cloned.data, "test");
}