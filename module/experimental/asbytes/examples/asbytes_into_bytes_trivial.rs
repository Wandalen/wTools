//! This example showcases the `IntoBytes` trait, demonstrating how it facilitates writing different data types to an I/O stream (simulated here by a Vec< u8 >). The generic `send_data` function accepts any type T that implements `IntoBytes`. Inside the function, `data.into_bytes()` consumes the input data and returns an owned Vec< u8 >. This owned vector is necessary when the receiving function or operation (like `writer.write_all`) requires ownership or when the data needs to live beyond the current scope (e.g., in asynchronous operations). The example sends a POD struct (with explicit padding for Pod safety), a String, a Vec< f32 >, and an array, showing how `IntoBytes` provides a uniform way to prepare diverse data for serialization or transmission. Note that types like String and Vec are moved and consumed, while Copy types are technically moved but the original variable remains usable due to the copy.

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  // Add dependencies to Cargo.toml:
  // asbytes = { version = "0.2", features = [ "derive" ] }
  use asbytes ::IntoBytes;
  use std ::io ::Write;

  // Define a POD struct with explicit padding to satisfy `Pod` requirements.
  #[ repr( C ) ]
  #[ derive( Clone, Copy, Debug, asbytes ::Pod, asbytes ::Zeroable ) ]
  struct DataPacketHeader
  {
    packet_id : u64,       // 8 bytes
    payload_len : u32,     // 4 bytes
    checksum : u16,        // 2 bytes
    _padding : [ u8; 2 ],  // 2 bytes explicit padding; total 16 bytes
  }

  /// Simulates writing any data that implements `IntoBytes` to a writer.
  /// Consumes the input data, converting it to an owned byte vector for I/O.
  fn send_data< T : IntoBytes, W : Write >( data : T, writer : &mut W ) -> std ::io ::Result< () >
  {
    let bytes : Vec< u8 > = data.into_bytes();
    writer.write_all( &bytes )?;
    Ok( () )
  }

  let mut output_buffer : Vec< u8 > = Vec ::new();

  let header = DataPacketHeader
  {
    packet_id : 0xABCD_EF01_2345_6789,
    payload_len : 128,
    checksum : 0x55AA,
    _padding : [ 0, 0 ],
  };
  let payload_message = String ::from( "This is the core message payload." );
  let sensor_readings : Vec< f32 > = vec![ 25.5, -10.0, 99.9, 0.1 ];
  let end_marker : [ u8; 4 ] = [ 0xDE, 0xAD, 0xBE, 0xEF ];

  println!( "Sending different data types to the buffer...\n" );

  // Send the header (struct wrapped in tuple — consumes the tuple, header is Copy so remains valid).
  println!( "Sending Header: {header:?}" );
  send_data( ( header, ), &mut output_buffer ).expect( "Failed to write header" );

  // Send the payload (String — consumed and moved).
  println!( "Sending Payload Message: \"{payload_message}\"" );
  send_data( payload_message, &mut output_buffer ).expect( "Failed to write payload message" );

  // Send sensor readings (Vec< f32 > — consumed and moved).
  println!( "Sending Sensor Readings: {sensor_readings:?}" );
  send_data( sensor_readings, &mut output_buffer ).expect( "Failed to write sensor readings" );

  // Send the end marker (array — Copy so remains valid after send).
  println!( "Sending End Marker: {end_marker:?}" );
  send_data( end_marker, &mut output_buffer ).expect( "Failed to write end marker" );

  println!( "\n--- Final Buffer Content ({} bytes) ---", output_buffer.len() );
  for ( i, chunk ) in output_buffer.chunks( 16 ).enumerate()
  {
    print!( "{:08x} : ", i * 16 );
    for byte in chunk
    {
      print!( "{byte:02x} " );
    }
    print!( " |" );
    for &byte in chunk
    {
      if ( 32..=126 ).contains( &byte )
      {
        print!( "{}", byte as char );
      }
      else
      {
        print!( "." );
      }
    }
    println!( "|" );
  }

  println!( "\nDemonstration complete. The send_data function handled multiple data types" );
  println!( "by converting them to owned byte vectors using IntoBytes, suitable for I/O operations." );
}
