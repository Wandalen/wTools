//! This example showcases the `IntoBytes` trait, demonstrating how it facilitates writing different data types to an I/O stream (simulated here by a Vec<u8>). The generic `send_data` function accepts any type T that implements `IntoBytes`. Inside the function, `data.into_bytes()` consumes the input data and returns an owned Vec<u8>. This owned vector is necessary when the receiving function or operation (like `writer.write_all`) requires ownership or when the data needs to live beyond the current scope (e.g., in asynchronous operations). The example sends a POD struct (with explicit padding for Pod safety), a String, a Vec<f32>, and an array, showing how `IntoBytes` provides a uniform way to prepare diverse data for serialization or transmission. Note that types like String and Vec are moved and consumed, while Copy types are technically moved but the original variable remains usable due to the copy.

// Add dependencies to Cargo.toml:
// asbytes = { version = "0.2", features = [ "derive" ] }
use asbytes::IntoBytes;
use std::io::Write; // Using std::io::Write as a simulated target

// Define a POD struct
// Added explicit padding to ensure no implicit padding bytes, satisfying `Pod` requirements.
#[ repr( C ) ]
#[ derive( Clone, Copy, Debug, asbytes::Pod, asbytes::Zeroable ) ]
struct DataPacketHeader {
  packet_id: u64,    // 8 bytes
  payload_len: u32,  // 4 bytes
  checksum: u16,     // 2 bytes
  _padding: [u8; 2], // 2 bytes explicit padding to align to 8 bytes (u64 alignment)
} // Total size = 16 bytes (128 bits)

/// Simulates writing any data that implements `IntoBytes` to a writer (e.g., file, network stream).
/// This function consumes the input data.
/// It takes a mutable reference to a writer `W` which could be Vec<u8>, a File, `TcpStream`, etc.
fn send_data<T: IntoBytes, W: Write>(data: T, writer: &mut W) -> std::io::Result<()> {
  // 1. Consume the data into an owned byte vector using IntoBytes.
  // This is useful because the writer might perform operations asynchronously,
  // or the data might need manipulation before sending, requiring ownership.
  let bytes: Vec<u8> = data.into_bytes();

  // 2. Write the owned bytes to the provided writer.
  // The `write_all` method requires a byte slice (`&[u8]`).
  writer.write_all(&bytes)?;

  // Optional: Add a separator or framing bytes if needed for the protocol
  // writer.write_all( b"\n---\n" )?;

  Ok(())
}

fn main() {
  // --- Simulate an output buffer (could be a file, network socket, etc.) ---
  let mut output_buffer: Vec<u8> = Vec::new();

  // --- Different types of data to serialize and send ---
  let header = DataPacketHeader {
    packet_id: 0xABCDEF0123456789,
    payload_len: 128,
    checksum: 0x55AA,
    _padding: [0, 0], // Initialize padding
  };
  let payload_message = String::from("This is the core message payload.");
  let sensor_readings: Vec<f32> = vec![25.5, -10.0, 99.9, 0.1];
  // Ensure sensor readings are POD if necessary (f32 is Pod)
  let end_marker: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

  println!("Sending different data types to the buffer...\n");

  // --- Send data using the generic function ---

  // Send the header (struct wrapped in tuple). Consumes the tuple.
  println!("Sending Header: {header:?}");
  send_data((header,), &mut output_buffer).expect("Failed to write header");
  // The original `header` is still available because it's `Copy`.

  // Send the payload (String). Consumes the `payload_message` string.
  println!("Sending Payload Message: \"{payload_message}\"");
  send_data(payload_message, &mut output_buffer).expect("Failed to write payload message");
  // `payload_message` is no longer valid here.

  // Send sensor readings (Vec<f32>). Consumes the `sensor_readings` vector.
  // Check if f32 requires Pod trait - yes, bytemuck implements Pod for f32.
  // Vec<T> where T: Pod is handled by IntoBytes.
  println!("Sending Sensor Readings: {sensor_readings:?}");
  send_data(sensor_readings, &mut output_buffer).expect("Failed to write sensor readings");
  // `sensor_readings` is no longer valid here.

  // Send the end marker (array). Consumes the array (effectively Copy).
  println!("Sending End Marker: {end_marker:?}");
  send_data(end_marker, &mut output_buffer).expect("Failed to write end marker");
  // The original `end_marker` is still available because it's `Copy`.

  println!("\n--- Final Buffer Content ({} bytes) ---", output_buffer.len());
  // Print bytes in a more readable hex format
  for (i, chunk) in output_buffer.chunks(16).enumerate() {
    print!("{:08x}: ", i * 16);
    for byte in chunk {
      print!("{byte:02x} ");
    }
    // Print ASCII representation
    print!(" |");
    for &byte in chunk {
      if (32..=126).contains(&byte) {
        print!("{}", byte as char);
      } else {
        print!(".");
      }
    }
    println!("|");
  }

  println!("\nDemonstration complete. The send_data function handled multiple data types");
  println!("by converting them to owned byte vectors using IntoBytes, suitable for I/O operations.");
}
