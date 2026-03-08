//! Comprehensive corner case testing for `mem_tools` crate.
//!
//! Tests all edge cases systematically including:
//! - Zero-sized types (ZST)
//! - Fat pointers and slices
//! - String literal deduplication
//! - Cross-type comparisons
//! - Empty collections
//! - Unicode strings
//! - Nested structures
//! - All documentation examples

use mem_tools as mem;
use core::marker::PhantomData;

fn main()
{
  println!("=== Manual Corner Case Testing ===\n");

  // Test same_ptr()
  test_same_ptr();

  // Test same_size()
  test_same_size();

  // Test same_region()
  test_same_region();

  // Test same_data()
  test_same_data();

  // Test documentation examples
  test_documentation_examples();

  println!("\n=== All Manual Tests Passed ===");
}

fn test_same_ptr()
{
  println!("Testing same_ptr():");

  // Basic: Same reference (self-reference)
  let x = 42;
  assert!(mem::same_ptr(&x, &x));
  println!("  ✓ Self-reference");

  // Basic: Different references to same value type
  let x = 42;
  let y = 42;
  assert!(!mem::same_ptr(&x, &y));
  println!("  ✓ Different allocations");

  // Cross-type comparisons
  let num: u32 = 42;
  let arr: [u8; 4] = [42, 0, 0, 0];
  assert!(!mem::same_ptr(&num, &arr));
  println!("  ✓ Cross-type comparison");

  // Zero-sized types
  let zst1 = ();
  let zst2 = ();
  // Note: ZST behavior is implementation-defined
  println!("  ✓ ZST comparison: same_ptr={}", mem::same_ptr(&zst1, &zst2));

  // PhantomData (another ZST)
  let phantom1: PhantomData<i32> = PhantomData;
  let phantom2: PhantomData<i32> = PhantomData;
  println!("  ✓ PhantomData comparison: same_ptr={}", mem::same_ptr(&phantom1, &phantom2));

  // String literal deduplication
  let s1 = "test_string_literal";
  let s2 = "test_string_literal";
  println!("  ✓ String literal deduplication: same_ptr={}", mem::same_ptr(s1, s2));

  // Fat pointers (slices)
  let arr = [1, 2, 3, 4];
  let slice1: &[i32] = &arr[..];
  let slice2: &[i32] = &arr[..];
  assert!(mem::same_ptr(slice1, slice2)); // Same slice of same array
  println!("  ✓ Same slice from array");

  let slice3: &[i32] = &arr[1..3];
  assert!(!mem::same_ptr(slice1, slice3)); // Different slices (different starting pointer)
  println!("  ✓ Different slices (different data pointers)");

  // Empty slices
  let empty1: &[i32] = &[];
  let empty2: &[i32] = &[];
  println!("  ✓ Empty slices: same_ptr={}", mem::same_ptr(empty1, empty2));

  // Stack vs heap data
  let stack_val = 42;
  let heap_val = Box::new(42);
  assert!(!mem::same_ptr(&stack_val, &*heap_val));
  println!("  ✓ Stack vs heap pointers");

  // Nested references
  let x = 42;
  let r1 = &x;
  let r2 = &x;
  assert!(mem::same_ptr(r1, r2)); // Both point to x
  println!("  ✓ Multiple references to same value");

  println!();
}

fn test_same_size()
{
  println!("Testing same_size():");

  // Basic: Same type
  let x: i32 = 10;
  let y: i32 = 20;
  assert!(mem::same_size(&x, &y));
  println!("  ✓ Same type (i32)");

  // Different types, same size
  let num: u32 = 42;
  let arr: [u8; 4] = [1, 2, 3, 4];
  assert!(mem::same_size(&num, &arr));
  println!("  ✓ Different types, same size (u32 vs [u8; 4])");

  // Different sizes
  let x: u16 = 10;
  let y: u32 = 20;
  assert!(!mem::same_size(&x, &y));
  println!("  ✓ Different sizes (u16 vs u32)");

  // Zero-sized types
  let zst1 = ();
  let zst2: PhantomData<i32> = PhantomData;
  assert!(mem::same_size(&zst1, &zst2)); // Both size 0
  println!("  ✓ ZST same size");

  // Empty string vs empty slice
  let empty_str: &str = "";
  let empty_slice: &[u8] = &[];
  assert!(mem::same_size(empty_str, empty_slice)); // Both size 0
  println!("  ✓ Empty string vs empty slice");

  // String vs &str of same length
  let string = String::from("abc");
  let str_ref: &str = "xyz";
  assert!(mem::same_size(string.as_str(), str_ref));
  println!("  ✓ Same-length strings");

  // Slice vs array of same length
  let arr: [i32; 3] = [1, 2, 3];
  let slice: &[i32] = &[4, 5, 6];
  assert!(mem::same_size(&arr[..], slice));
  println!("  ✓ Slice vs array (same length)");

  // Unicode strings (multi-byte characters)
  let ascii = "abc"; // 3 bytes
  let unicode = "π"; // π is 2 bytes in UTF-8
  assert!(!mem::same_size(ascii, unicode));
  println!("  ✓ ASCII vs Unicode strings (different byte sizes)");

  // Tuple vs struct with same fields
  let tuple: (i32, i32) = (10, 20);

  #[repr(C)]
  struct Point
  {
    x: i32,
    y: i32,
  }
  let point = Point { x: 10, y: 20 };
  assert!(mem::same_size(&tuple, &point));
  println!("  ✓ Tuple vs struct (same layout)");

  // Minimum sized primitive
  let bool_val = true;
  let u8_val: u8 = 42;
  assert!(mem::same_size(&bool_val, &u8_val)); // Both 1 byte
  println!("  ✓ bool vs u8 (both 1 byte)");

  // Maximum sized primitive
  let u128_val: u128 = 42;
  let i128_val: i128 = 42;
  assert!(mem::same_size(&u128_val, &i128_val));
  println!("  ✓ u128 vs i128 (both 16 bytes)");

  println!();
}

fn test_same_region()
{
  println!("Testing same_region():");

  // Same reference (identical region)
  let x = 42;
  assert!(mem::same_region(&x, &x));
  println!("  ✓ Self-reference (same region)");

  // Different references to same data
  let x = 42;
  let y = 42;
  assert!(!mem::same_region(&x, &y));
  println!("  ✓ Different allocations (different regions)");

  // String literal deduplication (may be same region)
  let s1 = "deduplicated_literal";
  let s2 = "deduplicated_literal";
  println!("  ✓ String literal deduplication: same_region={}", mem::same_region(s1, s2));

  // String::from vs literal (different region)
  let s1 = "abc";
  let s2 = String::from("abc");
  assert!(!mem::same_region(s1, s2.as_str()));
  println!("  ✓ Literal vs String::from (different regions)");

  // Same pointer, same size
  let arr = [1, 2, 3];
  let slice1: &[i32] = &arr[..];
  let slice2: &[i32] = &arr[..];
  assert!(mem::same_region(slice1, slice2));
  println!("  ✓ Same slice (same region)");

  // Subslice vs full slice (different pointer)
  let arr = [1, 2, 3, 4];
  let full_slice: &[i32] = &arr[..];
  let sub_slice: &[i32] = &arr[1..3];
  assert!(!mem::same_region(full_slice, sub_slice));
  println!("  ✓ Full slice vs subslice (different regions)");

  // Empty slices at different locations
  let arr1 = [1, 2, 3];
  let arr2 = [4, 5, 6];
  let empty1: &[i32] = &arr1[0..0];
  let empty2: &[i32] = &arr2[0..0];
  println!("  ✓ Empty slices at different locations: same_region={}", mem::same_region(empty1, empty2));

  // ZST references
  let zst1 = ();
  let zst2 = ();
  println!("  ✓ ZST references: same_region={}", mem::same_region(&zst1, &zst2));

  println!();
}

fn test_same_data()
{
  println!("Testing same_data():");

  // Identical primitive values
  let x: i32 = 42;
  let y: i32 = 42;
  assert!(mem::same_data(&x, &y));
  println!("  ✓ Identical primitives");

  // Different primitive values
  let x: i32 = 42;
  let y: i32 = 100;
  assert!(!mem::same_data(&x, &y));
  println!("  ✓ Different primitives");

  // Identical struct values
  #[derive(Clone)]
  #[allow(dead_code)]
  struct Point
  {
    x: i32,
    y: i32,
  }
  let p1 = Point { x: 10, y: 20 };
  let p2 = Point { x: 10, y: 20 };
  assert!(mem::same_data(&p1, &p2));
  println!("  ✓ Identical structs");

  // Different struct values
  let p3 = Point { x: 30, y: 40 };
  assert!(!mem::same_data(&p1, &p3));
  println!("  ✓ Different structs");

  // Identical arrays
  let arr1 = [1, 2, 3, 4];
  let arr2 = [1, 2, 3, 4];
  assert!(mem::same_data(&arr1, &arr2));
  println!("  ✓ Identical arrays");

  // Different arrays
  let arr3 = [5, 6, 7, 8];
  assert!(!mem::same_data(&arr1, &arr3));
  println!("  ✓ Different arrays");

  // Identical slices
  let slice1: &[i32] = &[1, 2, 3];
  let slice2: &[i32] = &[1, 2, 3];
  assert!(mem::same_data(slice1, slice2));
  println!("  ✓ Identical slices");

  // ZST comparison
  let zst1 = ();
  let zst2 = ();
  assert!(mem::same_data(&zst1, &zst2)); // Always true for ZST
  println!("  ✓ ZST data comparison (always equal)");

  // Empty slices
  let empty1: &[i32] = &[];
  let empty2: &[i32] = &[];
  assert!(mem::same_data(empty1, empty2)); // Both empty
  println!("  ✓ Empty slices (always equal)");

  // Single byte comparison
  let byte1: u8 = 0xFF;
  let byte2: u8 = 0xFF;
  assert!(mem::same_data(&byte1, &byte2));
  println!("  ✓ Single byte comparison");

  // Type-agnostic comparison (same data, different types)
  let tuple: (u8, u8, u8, u8) = (1, 2, 3, 4);
  let array: [u8; 4] = [1, 2, 3, 4];
  assert!(mem::same_data(&tuple, &array));
  println!("  ✓ Type-agnostic: tuple vs array (same data)");

  // Nested structures
  #[allow(dead_code)]
  struct Outer
  {
    inner: Point,
    value: i32,
  }
  let outer1 = Outer
  {
    inner: Point { x: 1, y: 2 },
    value: 42,
  };
  let outer2 = Outer
  {
    inner: Point { x: 1, y: 2 },
    value: 42,
  };
  assert!(mem::same_data(&outer1, &outer2));
  println!("  ✓ Nested structures");

  // Unicode strings
  let str1 = "hello";
  let str2 = "hello";
  assert!(mem::same_data(str1, str2));
  println!("  ✓ Unicode strings (identical)");

  let str3 = "héllo"; // Different bytes
  assert!(!mem::same_data(str1, str3));
  println!("  ✓ Unicode strings (different)");

  println!();
}

fn test_documentation_examples()
{
  println!("Testing Documentation Examples:");

  // Example from trivial.rs and readme.md
  {
    let src1 = (1,);
    let src2 = (1,);
    assert!(!mem::same_ptr(&src1, &src2));

    let src1 = "abc";
    let src2 = "cba";
    assert!(mem::same_size(src1, src2));

    let src1 = "abc";
    let src2 = "abc";
    assert!(mem::same_region(src1, src2));

    println!("  ✓ Basic use-case example");
  }

  // Pattern 1: Pointer Equality Checking
  {
    let src1 = (1,);
    let src2 = (1,);
    assert!(!mem::same_ptr(&src1, &src2));
    assert!(mem::same_ptr(&src1, &src1));

    let num: i32 = 42;
    let bytes: [u8; 4] = num.to_ne_bytes();
    assert!(!mem::same_ptr(&num, &bytes));

    println!("  ✓ Pattern 1: Pointer Equality");
  }

  // Pattern 2: Size Comparison
  {
    let src1 = "abc";
    let src2 = "xyz";
    assert!(mem::same_size(src1, src2));

    let src3 = "abcd";
    assert!(!mem::same_size(src1, src3));

    let num: u32 = 42;
    let arr: [u8; 4] = [1, 2, 3, 4];
    assert!(mem::same_size(&num, &arr));

    println!("  ✓ Pattern 2: Size Comparison");
  }

  // Pattern 3: Region Validation
  {
    let src1 = "abc";
    let src2 = "abc";
    assert!(mem::same_region(src1, src2));

    let src3 = String::from("abc");
    assert!(!mem::same_region(src1, src3.as_str()));

    println!("  ✓ Pattern 3: Region Validation");
  }

  // Pattern 4: Data Content Comparison
  {
    let src1 = "abc";
    let src2 = "abc";
    assert!(mem::same_data(src1, src2));

    let src3 = "xyz";
    assert!(!mem::same_data(src1, src3));

    #[derive(Clone)]
    #[allow(dead_code)]
    struct Point
    {
      x: i32,
      y: i32,
    }
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 10, y: 20 };
    assert!(mem::same_data(&p1, &p2));

    println!("  ✓ Pattern 4: Data Content Comparison");
  }

  // Pattern 5: Buffer Comparison
  {
    fn validate_buffer( expected: &[u8], actual: &[u8] ) -> bool
    {
      mem::same_data(expected, actual)
    }

    let buf1 = [1, 2, 3, 4];
    let buf2 = [1, 2, 3, 4];
    assert!(validate_buffer(&buf1, &buf2));

    println!("  ✓ Pattern 5: Buffer Comparison");
  }

  // Pattern 6: Type-Agnostic Comparison
  {
    let tuple: (u8, u8, u8, u8) = (1, 2, 3, 4);
    let array: [u8; 4] = [1, 2, 3, 4];

    assert!(mem::same_size(&tuple, &array));
    assert!(mem::same_data(&tuple, &array));

    println!("  ✓ Pattern 6: Type-Agnostic Comparison");
  }

  // Pattern 7: String Literal Deduplication Check
  {
    let s1 = "hello";
    let s2 = "hello";

    if mem::same_ptr(s1, s2)
    {
      println!("  ✓ Pattern 7: String literals deduplicated");
    }
    else
    {
      println!("  ✓ Pattern 7: String literals not deduplicated");
    }
  }

  // Pattern 8: DST Handling
  {
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3];
    let slice1: &[i32] = &arr1;
    let slice2: &[i32] = &arr2;

    assert!(mem::same_size(slice1, slice2));
    assert!(mem::same_data(slice1, slice2));
    assert!(!mem::same_ptr(slice1, slice2)); // Different arrays, different pointers

    println!("  ✓ Pattern 8: DST Handling");
  }

  println!();
}
