//! Investigation of `same_ptr` behavior with slices and fat pointers.
//!
//! Demonstrates that `same_ptr()` correctly compares the data pointer portion
//! of fat pointers, not the entire fat pointer structure.

#![allow(clippy::uninlined_format_args)]

use mem_tools as mem;

fn main()
{
  println!("=== Investigating same_ptr with slices ===\n");

  let arr = [1, 2, 3, 4];

  let slice1: &[i32] = &arr[..];    // Full slice [1,2,3,4]
  let slice2: &[i32] = &arr[..];    // Full slice [1,2,3,4]
  let slice3: &[i32] = &arr[1..3];  // Subslice [2,3]

  println!("Array: {:?}", arr);
  println!("slice1 (full): {:?} at {:p}", slice1, slice1.as_ptr());
  println!("slice2 (full): {:?} at {:p}", slice2, slice2.as_ptr());
  println!("slice3 (sub):  {:?} at {:p}", slice3, slice3.as_ptr());

  println!("\nComparisons:");
  println!("same_ptr(slice1, slice2) = {} (both full slices)", mem::same_ptr(slice1, slice2));
  println!("same_ptr(slice1, slice3) = {} (full vs sub)", mem::same_ptr(slice1, slice3));

  println!("\nAnalysis:");
  println!("  slice1 pointer: {:p}", slice1.as_ptr());
  println!("  slice3 pointer: {:p}", slice3.as_ptr());
  println!("  Are data pointers equal? {}", slice1.as_ptr() == slice3.as_ptr());

  // The question: Does `same_ptr` compare the fat pointer or just the data pointer?
  println!("\nConclusion:");
  println!("  `same_ptr()` compares the data pointer portion of fat pointers.");
  println!("  slice1 starts at &arr[0], slice3 starts at &arr[1].");
  println!("  Therefore `same_ptr(slice1, slice3)` should be FALSE.");

  if mem::same_ptr(slice1, slice3)
  {
    println!("  ✗ BUG: same_ptr returned true for different data pointers!");
  }
  else
  {
    println!("  ✓ CORRECT: Different data pointers detected");
  }
}
