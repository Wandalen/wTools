//! This example demonstrates the `AsBytes` trait. It shows how to get a `&[u8]` view of various data types (a `Vec`, a slice, an array, a single struct wrapped in a tuple, and a scalar wrapped in a tuple) without consuming the original data. This is useful for operations like inspecting byte patterns, hashing data without modification, or passing byte slices to functions that only need read access. The `.byte_size()` and `.len()` methods provide convenient ways to get the size in bytes and the number of elements, respectively.

// Make sure asbytes is available for derives
// asbytes = { version = "0.2", features = [ "derive" ] }
use asbytes::AsBytes; // Import the trait

// Define a POD struct
#[ repr( C ) ]
#[ derive( Debug, Clone, Copy, asbytes::Pod, asbytes::Zeroable ) ]
struct Point {
  x: f32,
  y: f32,
}

fn main() {
  // --- Collections ---
  let points_vec: Vec<Point> = vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }];
  let points_slice: &[Point] = &points_vec[..];
  let points_array: [Point; 1] = [Point { x: 5.0, y: 6.0 }];

  // Use AsBytes to get byte slices (&[u8]) without consuming the original data
  let vec_bytes: &[u8] = points_vec.as_bytes();
  let slice_bytes: &[u8] = points_slice.as_bytes();
  let array_bytes: &[u8] = points_array.as_bytes();

  println!("Vec Bytes: length={}, data={:?}", points_vec.byte_size(), vec_bytes);
  println!("Slice Bytes: length={}, data={:?}", slice_bytes.byte_size(), slice_bytes);
  println!("Array Bytes: length={}, data={:?}", points_array.byte_size(), array_bytes);
  println!("Vec Element Count: {}", points_vec.len()); // Output: 2
  println!("Array Element Count: {}", points_array.len()); // Output: 1

  // --- Single POD Item (using tuple trick) ---
  let single_point = Point { x: -1.0, y: -2.0 };
  let single_point_tuple = (single_point,); // Wrap in a single-element tuple

  let point_bytes: &[u8] = single_point_tuple.as_bytes();
  println!(
    "Single Point Bytes: length={}, data={:?}",
    single_point_tuple.byte_size(),
    point_bytes
  );
  println!("Single Point Element Count: {}", single_point_tuple.len()); // Output: 1

  let scalar_tuple = (12345u32,);
  let scalar_bytes: &[u8] = scalar_tuple.as_bytes();
  println!("Scalar Bytes: length={}, data={:?}", scalar_tuple.byte_size(), scalar_bytes);

  // Original data is still available after calling .as_bytes()
  println!("Original Vec still usable: {points_vec:?}");
}
