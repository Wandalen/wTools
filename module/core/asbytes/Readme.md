<!-- {{# generate.module_header{} #}} -->

# Module :: asbytes
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleasbytesPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleasbytesPush.yml) [![docs.rs](https://img.shields.io/docsrs/asbytes?color=e3e8f0&logo=docs.rs)](https://docs.rs/asbytes) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

The `asbytes` crate provides a convenient trait, `AsBytes`, for viewing common data structures as raw byte slices (`&[u8]`). It focuses on types that are safe to represent as bytes (Plain Old Data, or POD), leveraging the safety guarantees of the underlying `bytemuck` crate.

## Why `asbytes`?

While `bytemuck` provides the core functionality for safe byte-level casting (like `bytemuck::cast_slice` for collections and `bytemuck::bytes_of` for single items), `asbytes` offers a unified trait-based approach for common use cases:

1.  **Consistency:** The `AsBytes` trait provides a single method, `.as_bytes()`, that works consistently across supported types like `Vec<T>`, slices (`&[T]`), arrays (`[T; N]`), and single POD items wrapped in a tuple `(T,)`.
2.  **Readability:** Calling `.as_bytes()` clearly signals the intent to get the raw byte representation, which is useful for tasks like serialization, hashing, or interfacing with low-level APIs (graphics, networking, etc.).
3.  **Simpler Generics:** Functions can accept `T: AsBytes` to work generically with the byte representation of different compatible data structures.
4.  **Convenience:** The trait also provides `.byte_size()` and `.len()` methods for easily getting the size in bytes and the number of elements, respectively.

Essentially, `asbytes` acts as a focused convenience layer on top of `bytemuck` for the specific task of viewing data as bytes via a consistent trait method.

## How asbytes Differs from bytemuck

While bytemuck offers safe transmutation via its `Pod` trait and functions like `cast_slice`, it does not expose a dedicated trait for converting data structures into byte slices. `asbytes` introduces the `AsBytes` trait, abstracting these conversions and providing additional conveniences—such as direct byte size computation—on top of bytemuck's proven foundation.

### Example

```rust
// Make sure bytemuck is available for derives
extern crate bytemuck;
use asbytes::AsBytes; // Import the trait

// Define a POD struct
#[ repr( C ) ]
#[ derive( Clone, Copy, bytemuck::Pod, bytemuck::Zeroable ) ]
struct Point
{
  x : f32,
  y : f32,
}

fn main()
{
  // --- Collections ---
  let points_vec : Vec< Point > = vec![ Point { x : 1.0, y : 2.0 }, Point { x : 3.0, y : 4.0 } ];
  let points_slice : &[ Point ] = &points_vec[ .. ];
  let points_array : [ Point; 1 ] = [ Point { x : 5.0, y : 6.0 } ];

  let vec_bytes = points_vec.as_bytes();
  let slice_bytes = points_slice.as_bytes();
  let array_bytes = points_array.as_bytes();

  println!( "Vec Bytes: length={}, data={:?}", points_vec.byte_size(), vec_bytes );
  println!( "Slice Bytes: length={}, data={:?}", slice_bytes.byte_size(), slice_bytes );
  println!( "Array Bytes: length={}, data={:?}", points_array.byte_size(), array_bytes );
  println!( "Vec Element Count: {}", points_vec.len() ); // Output: 2
  println!( "Array Element Count: {}", points_array.len() ); // Output: 1

  // --- Single POD Item (using tuple trick) ---
  let single_point = Point { x : -1.0, y : -2.0 };
  let single_point_tuple = ( single_point, ); // Wrap in a single-element tuple

  let point_bytes = single_point_tuple.as_bytes();
  println!( "Single Point Bytes: length={}, data={:?}", single_point_tuple.byte_size(), point_bytes );
  println!( "Single Point Element Count: {}", single_point_tuple.len() ); // Output: 1

  let scalar_tuple = ( 12345u32, );
  let scalar_bytes = scalar_tuple.as_bytes();
  println!( "Scalar Bytes: length={}, data={:?}", scalar_tuple.byte_size(), scalar_bytes );
}
```

### To add to your project

```sh
cargo add asbytes
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/asbytes
cargo run
```
