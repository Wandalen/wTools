//! Extended manual test for `is_slice` corner cases not covered by automated tests

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  use is_slice :: is_slice;
  println!("=== Extended Manual Testing for is_slice ===\n");

  // Test Category 1: Mutable Slices
  println!("--- Category 1: Mutable Slices ---");
  let mut arr = [1, 2, 3, 4, 5];
  let mut_slice: &mut [i32] = &mut arr;
  println!("Mutable slice (&mut [i32]): {}", is_slice!(mut_slice));

  let mut_slice_from_range: &mut [i32] = &mut arr[1..3];
  println!("Mutable slice from range (&mut arr[1..3]): {}", is_slice!(mut_slice_from_range));

  // Test Category 2: Sub-slicing Operations
  println!("\n--- Category 2: Sub-slicing Operations ---");
  let arr = [1, 2, 3, 4, 5];

  let partial_range = &arr[1..3];
  println!("Partial range (&arr[1..3]): {}", is_slice!(partial_range));

  let range_from = &arr[1..];
  println!("Range from (&arr[1..]): {}", is_slice!(range_from));

  let range_to = &arr[..3];
  println!("Range to (&arr[..3]): {}", is_slice!(range_to));

  let inclusive_range = &arr[1..=3];
  println!("Inclusive range (&arr[1..=3]): {}", is_slice!(inclusive_range));

  // Test Category 3: Zero-Sized Types
  println!("\n--- Category 3: Zero-Sized Types ---");
  let unit_slice: &[()] = &[(), (), ()];
  println!("Unit type slice (&[()]): {}", is_slice!(unit_slice));

  #[derive(Debug)]
  struct EmptyStruct;
  let empty_struct_slice: &[EmptyStruct] = &[EmptyStruct, EmptyStruct];
  println!("Empty struct slice: {}", is_slice!(empty_struct_slice));

  // Test Category 4: Static/Const Slices
  println!("\n--- Category 4: Static Slices ---");
  static STATIC_SLICE: &[i32] = &[1, 2, 3];
  println!("Static slice: {}", is_slice!(STATIC_SLICE));

  const CONST_ARRAY: [i32; 3] = [1, 2, 3];
  let const_slice: &[i32] = &CONST_ARRAY;
  println!("Slice from const array: {}", is_slice!(const_slice));

  // Test Category 5: Large Arrays
  println!("\n--- Category 5: Large Arrays ---");
  let large_array = [0; 100];
  let large_array_ref = &large_array;
  println!("Large array reference (&[i32; 100]): {}", is_slice!(large_array_ref));

  let large_slice: &[i32] = &large_array[..];
  println!("Large slice (&[i32; 100][..]): {}", is_slice!(large_slice));

  // Test Category 6: String/Byte Conversions
  println!("\n--- Category 6: String/Byte Conversions ---");
  let string = String::from("hello");

  let bytes_from_string = string.as_bytes();
  println!("String.as_bytes(): {}", is_slice!(bytes_from_string));

  let str_ref: &str = "world";
  let bytes_from_str = str_ref.as_bytes();
  println!("&str.as_bytes(): {}", is_slice!(bytes_from_str));

  let as_ref_slice: &[u8] = string.as_ref();
  println!("String.as_ref() -> &[u8]: {}", is_slice!(as_ref_slice));

  // Test Category 7: Vec Sub-slicing
  println!("\n--- Category 7: Vec Sub-slicing ---");
  let vec = [1, 2, 3, 4, 5];

  let vec_partial = &vec[1..3];
  println!("Vec partial range (&vec[1..3]): {}", is_slice!(vec_partial));

  let vec_range_from = &vec[2..];
  println!("Vec range from (&vec[2..]): {}", is_slice!(vec_range_from));

  // Test Category 8: Nested Slices
  println!("\n--- Category 8: Nested Slices ---");
  let nested: &[&[i32]] = &[&[1, 2], &[3, 4]];
  println!("Slice of slice refs (&[&[i32]]): {}", is_slice!(nested));

  // Test Category 9: Generic Type Handling
  println!("\n--- Category 9: Generic Functions ---");
  fn generic_slice_test<T>(slice: &[T]) -> bool
  {
    is_slice!(slice)
  }

  let int_slice: &[i32] = &[1, 2, 3];
  println!("Generic function with &[i32]: {}", generic_slice_test(int_slice));

  let str_slice: &[&str] = &["a", "b", "c"];
  println!("Generic function with &[&str]: {}", generic_slice_test(str_slice));

  // Test Category 10: Tuple Slices
  println!("\n--- Category 10: Tuple Slices ---");
  let tuple_slice: &[(i32, i32)] = &[(1, 2), (3, 4)];
  println!("Tuple slice (&[(i32, i32)]): {}", is_slice!(tuple_slice));

  println!("\n=== Manual Testing Complete ===");
}
