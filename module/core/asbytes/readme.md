<!-- {{# generate.module_header{} #}} -->

# Module :: asbytes
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleasbytesPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleasbytesPush.yml) [![docs.rs](https://img.shields.io/docsrs/asbytes?color=e3e8f0&logo=docs.rs)](https://docs.rs/asbytes) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)


The `asbytes` crate provides two convenient traits:
1.  `AsBytes`: For viewing common data structures as raw byte slices (`&[u8]`).
2.  `IntoBytes`: For consuming data structures into owned byte vectors (`Vec<u8>`).

Both traits focus on types that are safe to represent as bytes (Plain Old Data, or POD), leveraging the safety guarantees of the underlying `bytemuck` crate.

## Why `asbytes`?

While `bytemuck` provides the core functionality for safe byte-level casting (like `bytemuck::cast_slice` for collections and `bytemuck::bytes_of` for single items), `asbytes` offers a unified trait-based approach for common use cases:

1.  **Consistency:** The `AsBytes` trait provides `.as_bytes()` for borrowing as `&[u8]`, while `IntoBytes` provides `.into_bytes()` for consuming into `Vec<u8>`. This works consistently across supported types.
2.  **Readability:** Calling `.as_bytes()` or `.into_bytes()` clearly signals the intent to get a raw byte representation, useful for serialization, hashing, or low-level APIs (graphics, networking, etc.).
3.  **Simpler Generics:** Functions can accept `T: AsBytes` or `T: IntoBytes` to work generically with the byte representation of different compatible data structures.
4.  **Convenience:** `AsBytes` also provides `.byte_size()` and `.len()` methods for easily getting the size in bytes and the number of elements.

Essentially, `asbytes` acts as a focused convenience layer on top of `bytemuck` for the specific tasks of viewing or consuming data as bytes via consistent trait methods.

## How asbytes Differs from bytemuck

While `bytemuck` offers safe transmutation via its `Pod` trait and functions like `cast_slice`, it does not expose dedicated traits for converting data structures into byte slices or vectors. `asbytes` introduces `AsBytes` (for borrowing as `&[u8]`) and `IntoBytes` (for consuming into `Vec<u8>`), abstracting these conversions and providing additional conveniences—such as direct byte size computation with `AsBytes`—on top of `bytemuck`'s proven foundation.

## Examples

### `AsBytes` Example: Viewing Data as Byte Slices

This example demonstrates the `AsBytes` trait. It shows how to get a `&[u8]` view of various data types (a `Vec`, a slice, an array, a single struct wrapped in a tuple, and a scalar wrapped in a tuple) without consuming the original data. This is useful for operations like inspecting byte patterns, hashing data without modification, or passing byte slices to functions that only need read access. The `.byte_size()` and `.len()` methods provide convenient ways to get the size in bytes and the number of elements, respectively.

```rust

// Make sure asbytes is available for derives
// asbytes = { version = "0.2", features = [ "derive" ] }
use asbytes::AsBytes; // Import the trait

// Define a POD struct
#[ repr( C ) ]
#[ derive( Debug, Clone, Copy, asbytes::Pod, asbytes::Zeroable ) ]
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

  // Use AsBytes to get byte slices (&[u8]) without consuming the original data
  let vec_bytes : &[ u8 ] = points_vec.as_bytes();
  let slice_bytes : &[ u8 ] = points_slice.as_bytes();
  let array_bytes : &[ u8 ] = points_array.as_bytes();

  println!( "Vec Bytes: length={}, data={:?}", points_vec.byte_size(), vec_bytes );
  println!( "Slice Bytes: length={}, data={:?}", slice_bytes.byte_size(), slice_bytes );
  println!( "Array Bytes: length={}, data={:?}", points_array.byte_size(), array_bytes );
  println!( "Vec Element Count: {}", points_vec.len() ); // Output: 2
  println!( "Array Element Count: {}", points_array.len() ); // Output: 1

  // --- Single POD Item (using tuple trick) ---
  let single_point = Point { x : -1.0, y : -2.0 };
  let single_point_tuple = ( single_point, ); // Wrap in a single-element tuple

  let point_bytes : &[ u8 ] = single_point_tuple.as_bytes();
  println!( "Single Point Bytes: length={}, data={:?}", single_point_tuple.byte_size(), point_bytes );
  println!( "Single Point Element Count: {}", single_point_tuple.len() ); // Output: 1

  let scalar_tuple = ( 12345u32, );
  let scalar_bytes : &[ u8 ] = scalar_tuple.as_bytes();
  println!( "Scalar Bytes: length={}, data={:?}", scalar_tuple.byte_size(), scalar_bytes );

  // Original data is still available after calling .as_bytes()
  println!( "Original Vec still usable: {:?}", points_vec );
}
```

### `IntoBytes` Example: Consuming Data into Owned Byte Vectors for Hashing

This example showcases the IntoBytes trait, demonstrating how it facilitates writing different data types to an I/O stream (simulated here by a Vec<u8>). The generic send_data function accepts any type T that implements IntoBytes. Inside the function, data.into_bytes() consumes the input data and returns an owned Vec<u8>. This owned vector is necessary when the receiving function or operation (like writer.write_all) requires ownership or when the data needs to live beyond the current scope (e.g., in asynchronous operations). The example sends a POD struct (with explicit padding for Pod safety), a String, a Vec<f32>, and an array, showing how IntoBytes provides a uniform way to prepare diverse data for serialization or transmission. Note that types like String and Vec are moved and consumed, while Copy types are technically moved but the original variable remains usable due to the copy.

``````rust
// Add dependencies to Cargo.toml:
// asbytes = { version = "0.2", features = [ "derive" ] }
use asbytes::IntoBytes;
use std::io::Write; // Using std::io::Write as a simulated target

// Define a POD struct
// Added explicit padding to ensure no implicit padding bytes, satisfying `Pod` requirements.
#[ repr( C ) ]
#[ derive( Clone, Copy, Debug, asbytes::Pod, asbytes::Zeroable ) ]
struct DataPacketHeader
{
  packet_id : u64,      // 8 bytes
  payload_len : u32,    // 4 bytes
  checksum : u16,       // 2 bytes
  _padding : [ u8; 2 ], // 2 bytes explicit padding to align to 8 bytes (u64 alignment)
} // Total size = 16 bytes (128 bits)

/// Simulates writing any data that implements IntoBytes to a writer (e.g., file, network stream).
/// This function consumes the input data.
/// It takes a mutable reference to a writer `W` which could be Vec<u8>, a File, TcpStream, etc.
fn send_data< T : IntoBytes, W : Write >( data : T, writer : &mut W ) -> std::io::Result<()>
{
  // 1. Consume the data into an owned byte vector using IntoBytes.
  // This is useful because the writer might perform operations asynchronously,
  // or the data might need manipulation before sending, requiring ownership.
  let bytes : Vec< u8 > = data.into_bytes();

  // 2. Write the owned bytes to the provided writer.
  // The `write_all` method requires a byte slice (`&[u8]`).
  writer.write_all( &bytes )?;

  // Optional: Add a separator or framing bytes if needed for the protocol
  // writer.write_all( b"\n---\n" )?;

  Ok(())
}

fn main()
{
  // --- Simulate an output buffer (could be a file, network socket, etc.) ---
  let mut output_buffer : Vec< u8 > = Vec::new();

  // --- Different types of data to serialize and send ---
  let header = DataPacketHeader
  {
    packet_id : 0xABCDEF0123456789,
    payload_len : 128,
    checksum : 0x55AA,
    _padding : [ 0, 0 ], // Initialize padding
  };
  let payload_message = String::from( "This is the core message payload." );
  let sensor_readings : Vec< f32 > = vec![ 25.5, -10.0, 99.9, 0.1 ];
  // Ensure sensor readings are POD if necessary (f32 is Pod)
  let end_marker : [ u8; 4 ] = [ 0xDE, 0xAD, 0xBE, 0xEF ];

  println!( "Sending different data types to the buffer...\n" );

  // --- Send data using the generic function ---

  // Send the header (struct wrapped in tuple). Consumes the tuple.
  println!( "Sending Header: {:?}", header );
  send_data( ( header, ), &mut output_buffer ).expect( "Failed to write header" );
  // The original `header` is still available because it's `Copy`.

  // Send the payload (String). Consumes the `payload_message` string.
  println!( "Sending Payload Message: \"{}\"", payload_message );
  send_data( payload_message, &mut output_buffer ).expect( "Failed to write payload message" );
  // `payload_message` is no longer valid here.

  // Send sensor readings (Vec<f32>). Consumes the `sensor_readings` vector.
  // Check if f32 requires Pod trait - yes, bytemuck implements Pod for f32.
  // Vec<T> where T: Pod is handled by IntoBytes.
  println!( "Sending Sensor Readings: {:?}", sensor_readings );
  send_data( sensor_readings, &mut output_buffer ).expect( "Failed to write sensor readings" );
  // `sensor_readings` is no longer valid here.

  // Send the end marker (array). Consumes the array (effectively Copy).
  println!( "Sending End Marker: {:?}", end_marker );
  send_data( end_marker, &mut output_buffer ).expect( "Failed to write end marker" );
  // The original `end_marker` is still available because it's `Copy`.


  println!( "\n--- Final Buffer Content ({} bytes) ---", output_buffer.len() );
  // Print bytes in a more readable hex format
  for ( i, chunk ) in output_buffer.chunks( 16 ).enumerate()
  {
    print!( "{:08x}: ", i * 16 );
    for byte in chunk
    {
      print!( "{:02x} ", byte );
    }
    // Print ASCII representation
    print!( " |" );
    for &byte in chunk
    {
      if byte >= 32 && byte <= 126 {
        print!( "{}", byte as char );
      } else {
        print!( "." );
      }
    }
    println!( "|" );
  }

  println!( "\nDemonstration complete. The send_data function handled multiple data types" );
  println!( "by converting them to owned byte vectors using IntoBytes, suitable for I/O operations." );
}
``````

### To add to your project

```sh
cargo add asbytes
# Make sure bytemuck is also added if you need POD derives or its features
# cargo add bytemuck --features derive
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
# Run the AsBytes example (replace with actual example path if different)
# cargo run --example asbytes_as_bytes_trivial
# Or run the IntoBytes example (requires adding sha2 to the example's deps)
# cargo run --example asbytes_into_bytes_trivial
```
