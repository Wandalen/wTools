//! A trivial example for `error_tools`.

use error_tools::untyped::{Result};

fn get_message() -> Result<&'static str> {
  Ok("Hello, world!")
  // Err( format_err!( "An unexpected error!" ) )
}

fn main() {
  match get_message() {
    Ok(msg) => println!("Success: {msg}"),
    Err(e) => println!("Error: {e:?}"),
  }
}
