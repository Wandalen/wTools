//! Demonstrates basic time retrieval functionality.
//!
//! This example shows how to get current time in different units:
//! - Milliseconds (default and explicit)
//! - Seconds
//! - Nanoseconds
//!
//! All times are measured from UNIX epoch (1970-01-01 00:00:00 UTC).

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  {
  use time_tools as the_module;

  println!( "Time Tools - Current Time Demonstration\n" );

  // Get current time in milliseconds (default)
  let now = the_module ::now();
  println!( "Milliseconds (default): {now}" );

  // Get current time in milliseconds (explicit)
  let now_ms = the_module ::ms ::now();
  println!( "Milliseconds (explicit): {now_ms}" );

  // Get current time in seconds
  let now_s = the_module ::s ::now();
  println!( "Seconds:                {now_s}" );

  // Get current time in nanoseconds
  let now_ns = the_module ::ns ::now();
  println!( "Nanoseconds:            {now_ns}" );

  // Demonstrate unit conversions
  println!( "\nUnit Conversion Verification:" );
  let ms = the_module ::now();
  let ns = the_module ::ns ::now();
  let s = the_module ::s ::now();

  let ms_to_s = ms / 1000;
  let diff_s = ( ms_to_s - s ).abs();
  println!( "  ms / 1000 == s:        {eq} (diff: {diff_s})",
    eq = ms_to_s == s
  );

  let ns_to_ms = ns / 1_000_000;
  let diff_ms = ( ns_to_ms - ms ).abs();
  println!( "  ns / 1_000_000 == ms:  {eq} (diff: {diff_ms})",
    eq = ns_to_ms == ms
  );
  }

  #[ cfg( feature = "no_std" ) ]
  {
    println!( "Time functions require standard library (no_std mode active)" );
  }
}
