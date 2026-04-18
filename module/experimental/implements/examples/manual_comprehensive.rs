//! Comprehensive manual testing for implements! macro
//!
//! Tests all corner cases identified in specification including:
//! - Basic trait checking
//! - Multiple trait bounds
//! - Generic type checking
//! - Reference patterns
//! - Standard library traits
//! - Marker traits and ZSTs
//! - Fn trait patterns (with known limitations)
//! - instance_of! alias
//! - Edge cases

#![allow(clippy::uninlined_format_args)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::std_instead_of_core)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::needless_pass_by_value)]
#![allow(unused_imports)]
#![allow(dead_code)]

use implements::*;
use std::fmt::{Debug, Display};

// Custom trait for testing
trait Custom {
    fn custom_method(&self);
}

struct CustomType;
impl Custom for CustomType {
    fn custom_method(&self) {}
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main() {
    println!("=== COMPREHENSIVE MANUAL TESTING ===\n");

    // ===================================================================
    // Section 1: Basic Trait Checking
    // ===================================================================
    println!("--- Section 1: Basic Trait Checking ---");

    // Test 1.1: Copy trait on primitive
    let copy_check = implements!(13_i32 => Copy);
    println!("Test 1.1 - i32 implements Copy: {} (expected: true)", copy_check);
    assert_eq!(copy_check, true, "FAIL: i32 should implement Copy");

    // Test 1.2: Copy trait on Box (negative case)
    let no_copy_check = implements!(Box::new(13_i32) => Copy);
    println!("Test 1.2 - Box<i32> implements Copy: {} (expected: false)", no_copy_check);
    assert_eq!(no_copy_check, false, "FAIL: Box<i32> should NOT implement Copy");

    // Test 1.3: Clone trait
    let clone_check = implements!(vec![1, 2, 3] => Clone);
    println!("Test 1.3 - Vec<i32> implements Clone: {} (expected: true)", clone_check);
    assert_eq!(clone_check, true, "FAIL: Vec<i32> should implement Clone");

    // Test 1.4: Send trait
    let send_check = implements!(42_i32 => Send);
    println!("Test 1.4 - i32 implements Send: {} (expected: true)", send_check);
    assert_eq!(send_check, true, "FAIL: i32 should implement Send");

    // Test 1.5: Sync trait
    let sync_check = implements!(42_i32 => Sync);
    println!("Test 1.5 - i32 implements Sync: {} (expected: true)\n", sync_check);
    assert_eq!(sync_check, true, "FAIL: i32 should implement Sync");

    // ===================================================================
    // Section 2: Multiple Trait Bounds
    // ===================================================================
    println!("--- Section 2: Multiple Trait Bounds ---");

    // Test 2.1: Single trait
    let single_trait = implements!(42_i32 => Send);
    println!("Test 2.1 - i32 implements Send: {} (expected: true)", single_trait);
    assert_eq!(single_trait, true, "FAIL: Single trait check failed");

    // Test 2.2: Multiple traits (Send + Sync)
    let multiple_traits = implements!(42_i32 => Send + Sync);
    println!("Test 2.2 - i32 implements Send + Sync: {} (expected: true)", multiple_traits);
    assert_eq!(multiple_traits, true, "FAIL: Multiple trait check failed");

    // Test 2.3: Copy + Clone
    let copy_clone = implements!("hello" => Copy + Clone);
    println!("Test 2.3 - &str implements Copy + Clone: {} (expected: true)", copy_clone);
    assert_eq!(copy_clone, true, "FAIL: &str should implement Copy + Clone");

    // Test 2.4: Negative case - Rc is NOT Send
    use std::rc::Rc;
    let not_send = implements!(Rc::new(42) => Send);
    println!("Test 2.4 - Rc<i32> implements Send: {} (expected: false)\n", not_send);
    assert_eq!(not_send, false, "FAIL: Rc<i32> should NOT implement Send");

    // ===================================================================
    // Section 3: Generic Type Checking
    // ===================================================================
    println!("--- Section 3: Generic Type Checking ---");

    fn analyze<T>(value: T, _name: &str) -> bool {
        implements!(value => Clone)
    }

    // Test 3.1: i32 in generic context
    let is_clone1 = analyze(42, "i32");
    println!("Test 3.1 - i32 is cloneable: {} (expected: true)", is_clone1);
    assert_eq!(is_clone1, true, "FAIL: i32 should implement Clone");

    // Test 3.2: Box<i32> in generic context
    let is_clone2 = analyze(Box::new(42), "Box<i32>");
    println!("Test 3.2 - Box<i32> is cloneable: {} (expected: true)", is_clone2);
    assert_eq!(is_clone2, true, "FAIL: Box<i32> should implement Clone");

    // Test 3.3: String in generic context
    let is_clone3 = analyze(String::from("hello"), "String");
    println!("Test 3.3 - String is cloneable: {} (expected: true)", is_clone3);
    assert_eq!(is_clone3, true, "FAIL: String should implement Clone");
    println!();

    // ===================================================================
    // Section 4: Reference Patterns
    // ===================================================================
    println!("--- Section 4: Reference Patterns ---");

    let value = String::from("hello");

    // Test 4.1: Check value itself
    let value_check = implements!(value => Clone);
    println!("Test 4.1 - String implements Clone: {} (expected: true)", value_check);
    assert_eq!(value_check, true, "FAIL: String should implement Clone");

    // Test 4.2: Check through reference
    let reference = &value;
    let ref_check = implements!(*reference => Clone);
    println!("Test 4.2 - *&String implements Clone: {} (expected: true)", ref_check);
    assert_eq!(ref_check, true, "FAIL: Dereferenced &String should implement Clone");

    // Test 4.3: Reference types are Copy!
    let ref_copy = implements!(reference => Copy);
    println!("Test 4.3 - &String implements Copy: {} (expected: true)\n", ref_copy);
    assert_eq!(ref_copy, true, "FAIL: &String (reference) should implement Copy");

    // ===================================================================
    // Section 5: Standard Library Traits
    // ===================================================================
    println!("--- Section 5: Standard Library Traits ---");

    // Test 5.1: Debug trait
    let debug_check = implements!(42_i32 => Debug);
    println!("Test 5.1 - i32 implements Debug: {} (expected: true)", debug_check);
    assert_eq!(debug_check, true, "FAIL: i32 should implement Debug");

    // Test 5.2: Display trait
    let display_check = implements!(42_i32 => Display);
    println!("Test 5.2 - i32 implements Display: {} (expected: true)", display_check);
    assert_eq!(display_check, true, "FAIL: i32 should implement Display");

    // Test 5.3: Default trait
    let default_check = implements!(0_i32 => Default);
    println!("Test 5.3 - i32 implements Default: {} (expected: true)", default_check);
    assert_eq!(default_check, true, "FAIL: i32 should implement Default");

    // Test 5.4: Negative case - no Default for some types
    struct NoDefault;
    let no_default = implements!(NoDefault => Default);
    println!("Test 5.4 - NoDefault struct implements Default: {} (expected: false)\n", no_default);
    assert_eq!(no_default, false, "FAIL: NoDefault should NOT implement Default");

    // ===================================================================
    // Section 6: Marker Traits and ZSTs
    // ===================================================================
    println!("--- Section 6: Marker Traits and ZSTs ---");

    // Test 6.1: PhantomData
    let phantom_check = implements!(std::marker::PhantomData::<i32> => Copy);
    println!("Test 6.1 - PhantomData<i32> implements Copy: {} (expected: true)", phantom_check);
    assert_eq!(phantom_check, true, "FAIL: PhantomData should implement Copy");

    // Test 6.2: Unit type
    let unit_check = implements!(() => Copy);
    println!("Test 6.2 - () implements Copy: {} (expected: true)", unit_check);
    assert_eq!(unit_check, true, "FAIL: () should implement Copy");

    // Test 6.3: Empty struct
    struct EmptyStruct;
    let empty_check = implements!(EmptyStruct => Copy);
    println!("Test 6.3 - EmptyStruct implements Copy: {} (expected: false)\n", empty_check);
    assert_eq!(empty_check, false, "FAIL: EmptyStruct without #[derive(Copy)] should NOT implement Copy");

    // ===================================================================
    // Section 7: Fn Trait Patterns
    // ===================================================================
    println!("--- Section 7: Fn Trait Patterns ---");

    // Test 7.1: Closure with Fn trait (should work)
    let closure = || {};
    let closure_check = implements!(closure => Fn());
    println!("Test 7.1 - Closure implements Fn(): {} (expected: true)", closure_check);
    assert_eq!(closure_check, true, "FAIL: Closure should implement Fn()");

    // Test 7.2: Function pointer with Fn trait (should work)
    fn my_func() {}
    let fn_ptr: fn() = my_func;
    let fn_ptr_check = implements!(fn_ptr => Fn());
    println!("Test 7.2 - Function pointer implements Fn(): {} (expected: true)", fn_ptr_check);
    assert_eq!(fn_ptr_check, true, "FAIL: Function pointer should implement Fn()");

    // Test 7.3: Function item limitation (would cause compile error - documented in spec)
    println!("Test 7.3 - Function item Fn() check: SKIPPED (known compile error limitation)\n");

    // ===================================================================
    // Section 8: instance_of! Alias
    // ===================================================================
    println!("--- Section 8: instance_of! Alias ---");

    // Test 8.1: Basic usage
    let alias_basic = instance_of!(vec![1, 2, 3] => Clone);
    println!("Test 8.1 - instance_of!(Vec) => Clone: {} (expected: true)", alias_basic);
    assert_eq!(alias_basic, true, "FAIL: instance_of! should work like implements!");

    // Test 8.2: Complex traits
    let alias_complex = instance_of!(42_i32 => Send + Sync);
    println!("Test 8.2 - instance_of!(i32) => Send + Sync: {} (expected: true)", alias_complex);
    assert_eq!(alias_complex, true, "FAIL: instance_of! with multiple traits should work");

    // Test 8.3: Verify both macros return same result
    let impl_result = implements!(String::from("test") => Clone);
    let inst_result = instance_of!(String::from("test") => Clone);
    println!("Test 8.3 - implements! == instance_of!: {} (expected: true)\n", impl_result == inst_result);
    assert_eq!(impl_result, inst_result, "FAIL: implements! and instance_of! should return identical results");

    // ===================================================================
    // Section 9: Edge Cases
    // ===================================================================
    println!("--- Section 9: Edge Cases ---");

    // Test 9.1: Nested generics
    let nested = implements!(Vec::<Vec<i32>>::new() => Clone);
    println!("Test 9.1 - Vec<Vec<i32>> implements Clone: {} (expected: true)", nested);
    assert_eq!(nested, true, "FAIL: Nested generics should work");

    // Test 9.2: Option types
    let option_some = implements!(Some(42) => Clone);
    println!("Test 9.2 - Option<i32> implements Clone: {} (expected: true)", option_some);
    assert_eq!(option_some, true, "FAIL: Option should implement Clone");

    // Test 9.3: Result types
    let result_ok: Result<i32, String> = Ok(42);
    let result_check = implements!(result_ok => Clone);
    println!("Test 9.3 - Result<i32, String> implements Clone: {} (expected: true)", result_check);
    assert_eq!(result_check, true, "FAIL: Result should implement Clone");

    // Test 9.4: Custom trait on custom type
    let custom = implements!(CustomType => Custom);
    println!("Test 9.4 - CustomType implements Custom: {} (expected: true)", custom);
    assert_eq!(custom, true, "FAIL: CustomType should implement Custom trait");

    println!("\n=== ALL MANUAL TESTS PASSED ===");
    println!("Total tests executed: 30 (7.3 skipped - known limitation)");
    println!("All assertions passed successfully!");
}
