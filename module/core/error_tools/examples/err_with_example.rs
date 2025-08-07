//! A runnable example demonstrating the `ErrWith` trait.

use error_tools::error::{ErrWith};
use std::io;

fn might_fail_io(fail: bool) -> io::Result<u32> {
  if fail {
    Err(io::Error::new(io::ErrorKind::Other, "simulated I/O error"))
  } else {
    std::result::Result::Ok(42)
  }
}

fn process_data(input: &str) -> std::result::Result<String, (String, Box<dyn std::error::Error>)> {
  let num = input.parse::<u32>().err_with(|| "Failed to parse input".to_string())?;

  let result = might_fail_io(num % 2 != 0).err_with_report(&format!("Processing number {}", num))?;

  std::result::Result::Ok(format!("Processed result: {}", result))
}

fn main() {
  println!("--- Successful case ---");
  match process_data("100") {
    std::result::Result::Ok(msg) => println!("Success: {}", msg),
    std::result::Result::Err((report, err)) => println!("Error: {} - {:?}", report, err),
  }

  println!("\n--- Parsing error case ---");
  match process_data("abc") {
    std::result::Result::Ok(msg) => println!("Success: {}", msg),
    std::result::Result::Err((report, err)) => println!("Error: {} - {:?}", report, err),
  }

  println!("\n--- I/O error case ---");
  match process_data("1") {
    std::result::Result::Ok(msg) => println!("Success: {}", msg),
    std::result::Result::Err((report, err)) => println!("Error: {} - {:?}", report, err),
  }
}
