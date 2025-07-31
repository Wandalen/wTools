// Test to isolate proc macro issue with lifetime-only structs

// Custom attribute macro that does nothing - just to test the issue
use former::Former;

// This works fine - no derive
#[allow(dead_code)]
pub struct WorksWithoutDerive<'a> {
    data: &'a str,
}

// This should work - standard derives
#[derive(Debug, Clone)]
pub struct WorksWithStandardDerives<'a> {
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