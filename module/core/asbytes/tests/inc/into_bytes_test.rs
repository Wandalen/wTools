#![cfg(all(feature = "enabled", feature = "into_bytes"))]

use asbytes::IntoBytes; // Import the specific trait
use std::mem;

// Define a simple POD struct for testing (can be copied from basic_test.rs)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
struct Point {
  x: i32,
  y: i32,
}

#[test]
fn test_tuple_scalar_into_bytes() {
  let scalar_tuple = (123u32,);
  let expected_bytes = 123u32.to_le_bytes().to_vec();
  let bytes = scalar_tuple.into_bytes();

  assert_eq!(bytes.len(), mem::size_of::<u32>());
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_tuple_struct_into_bytes() {
  let point = Point { x: 10, y: -20 };
  let struct_tuple = (point,);
  let expected_bytes = bytemuck::bytes_of(&point).to_vec();
  let bytes = struct_tuple.into_bytes();

  assert_eq!(bytes.len(), mem::size_of::<Point>());
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_string_into_bytes() {
  let s = String::from("hello");
  let expected_bytes = vec![b'h', b'e', b'l', b'l', b'o'];
  // Clone s before moving it into into_bytes for assertion
  let bytes = s.clone().into_bytes();

  assert_eq!(bytes.len(), s.len());
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_str_into_bytes() {
  let s = "hello";
  let expected_bytes = vec![b'h', b'e', b'l', b'l', b'o'];
  // Clone s before moving it into into_bytes for assertion
  let bytes = s.into_bytes();

  assert_eq!(bytes.len(), s.len());
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_array_into_bytes() {
  let arr: [u16; 3] = [100, 200, 300];
  let expected_bytes = bytemuck::cast_slice(&arr).to_vec();
  let bytes = arr.into_bytes(); // arr is Copy

  assert_eq!(bytes.len(), arr.len() * mem::size_of::<u16>());
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_vec_into_bytes() {
  let v = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
  let expected_bytes = bytemuck::cast_slice(v.as_slice()).to_vec();
  let expected_len = v.len() * mem::size_of::<Point>();
  // Clone v before moving it into into_bytes for assertion
  let bytes = v.clone().into_bytes();

  assert_eq!(bytes.len(), expected_len);
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_box_t_into_bytes() {
  let b = Box::new(Point { x: 5, y: 5 });
  let expected_bytes = bytemuck::bytes_of(&*b).to_vec();
  let expected_len = mem::size_of::<Point>();
  let bytes = b.into_bytes();

  assert_eq!(bytes.len(), expected_len);
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_slice_into_bytes() {
  let slice: &[u32] = &[10, 20, 30][..];
  let expected_bytes = bytemuck::cast_slice(&*slice).to_vec();
  let expected_len = slice.len() * mem::size_of::<u32>();
  let bytes = slice.into_bytes();

  assert_eq!(bytes.len(), expected_len);
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_box_slice_into_bytes() {
  let slice: Box<[u32]> = vec![10, 20, 30].into_boxed_slice();
  let expected_bytes = bytemuck::cast_slice(&*slice).to_vec();
  let expected_len = slice.len() * mem::size_of::<u32>();
  let bytes = slice.into_bytes();

  assert_eq!(bytes.len(), expected_len);
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_vecdeque_into_bytes() {
  use std::collections::VecDeque; // Keep local use for VecDeque
  let mut deque: VecDeque<u16> = VecDeque::new();
  deque.push_back(10);
  deque.push_back(20);
  deque.push_front(5); // deque is now [5, 10, 20]

  // Expected bytes for [5, 10, 20] (little-endian)
  let expected_bytes = vec![
    5u16.to_le_bytes()[0],
    5u16.to_le_bytes()[1],
    10u16.to_le_bytes()[0],
    10u16.to_le_bytes()[1],
    20u16.to_le_bytes()[0],
    20u16.to_le_bytes()[1],
  ];
  let expected_len = deque.len() * mem::size_of::<u16>();
  let bytes = deque.into_bytes();

  assert_eq!(bytes.len(), expected_len);
  assert_eq!(bytes, expected_bytes);
}

#[test]
fn test_cstring_into_bytes() {
  use std::ffi::CString; // Keep local use for CString
  let cs = CString::new("world").unwrap();
  let expected_bytes = vec![b'w', b'o', b'r', b'l', b'd']; // No NUL byte
  let expected_len = expected_bytes.len();
  let bytes = cs.into_bytes();

  assert_eq!(bytes.len(), expected_len);
  assert_eq!(bytes, expected_bytes);
}
