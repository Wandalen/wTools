#![cfg(all(feature = "enabled", feature = "as_bytes"))]

// Define a simple POD struct for testing
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
struct Point {
  x: i32,
  y: i32,
}

#[test]
fn test_tuple_scalar_as_bytes() {
  {
    use asbytes::AsBytes;
    use std::mem;

    let scalar_tuple = (123u32,);
    let bytes = scalar_tuple.as_bytes();
    let expected_length = mem::size_of::<u32>();

    assert_eq!(bytes.len(), expected_length);
    assert_eq!(scalar_tuple.byte_size(), expected_length);
    assert_eq!(scalar_tuple.len(), 1); // Length of tuple is 1 element

    // Verify content (assuming little-endian)
    assert_eq!(bytes, &123u32.to_le_bytes());
  }
}

#[test]
fn test_tuple_struct_as_bytes() {
  {
    use asbytes::AsBytes;
    use std::mem;

    let point = Point { x: 10, y: -20 };
    let struct_tuple = (point,);
    let bytes = struct_tuple.as_bytes();
    let expected_length = mem::size_of::<Point>();

    assert_eq!(bytes.len(), expected_length);
    assert_eq!(struct_tuple.byte_size(), expected_length);
    assert_eq!(struct_tuple.len(), 1); // Length of tuple is 1 element

    // Verify content using bytemuck::bytes_of for comparison
    assert_eq!(bytes, bytemuck::bytes_of(&point));
  }
}

#[test]
fn test_vec_as_bytes() {
  {
    use asbytes::AsBytes;
    use std::mem;
    let v = vec![1u32, 2, 3, 4];
    let bytes = v.as_bytes();
    let expected_length = v.len() * mem::size_of::<u32>();
    assert_eq!(bytes.len(), expected_length);
    assert_eq!(v.byte_size(), expected_length);
    assert_eq!(v.len(), 4); // Length of Vec is number of elements
  }
}

#[test]
fn test_slice_as_bytes() {
  {
    use asbytes::exposed::AsBytes; // Using exposed path
    use std::mem;
    let slice: &[u32] = &[10, 20, 30];
    let bytes = slice.as_bytes();
    let expected_length = slice.len() * mem::size_of::<u32>();
    assert_eq!(bytes.len(), expected_length);
    assert_eq!(slice.byte_size(), expected_length);
    assert_eq!(slice.len(), 3); // Length of slice is number of elements
  }
}

#[test]
fn test_array_as_bytes() {
  {
    use asbytes::own::AsBytes; // Using own path
    use std::mem;
    let arr: [u32; 3] = [100, 200, 300];
    let bytes = arr.as_bytes();
    let expected_length = arr.len() * mem::size_of::<u32>();
    assert_eq!(bytes.len(), expected_length);
    assert_eq!(arr.byte_size(), expected_length);
    assert_eq!(arr.len(), 3); // Length of array is compile-time size N
  }
}

#[test]
fn test_vec_struct_as_bytes() {
  {
    use asbytes::AsBytes;
    use std::mem;
    let points = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    let bytes = points.as_bytes();
    let expected_length = points.len() * mem::size_of::<Point>();
    assert_eq!(bytes.len(), expected_length);
    assert_eq!(points.byte_size(), expected_length);
    assert_eq!(points.len(), 2);

    // Verify content using bytemuck::cast_slice for comparison
    assert_eq!(bytes, bytemuck::cast_slice(&points[..]));
  }
}
