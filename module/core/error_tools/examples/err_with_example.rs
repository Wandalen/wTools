//! A runnable example demonstrating the `ErrWith` trait.

use error_tools::error::{ErrWith};
use std::io;

fn might_fail_io(fail: bool) -> io::Result<u32> {
  if fail {
    Err(io::Error::other("simulated I/O error"))
  } else {
    core::result::Result::Ok(42)
  }
}

fn process_data(input: &str) -> core::result::Result<String, (String, Box<dyn core::error::Error>)> {
  let num = input.parse::<u32>().err_with(|| "Failed to parse input".to_string())?;

  let result = might_fail_io(num % 2 != 0).err_with_report(&format!("Processing number {num}"))?;

  core::result::Result::Ok(format!("Processed result: {result}"))
}

fn main() {
  println!("--- Successful case ---");
  match process_data("100") {
    core::result::Result::Ok(msg) => println!("Success: {msg}"),
    core::result::Result::Err((report, err)) => println!("Error: {report} - {err:?}"),
  }

  println!("\n--- Parsing error case ---");
  match process_data("abc") {
    core::result::Result::Ok(msg) => println!("Success: {msg}"),
    core::result::Result::Err((report, err)) => println!("Error: {report} - {err:?}"),
  }

  println!("\n--- I/O error case ---");
  match process_data("1") {
    core::result::Result::Ok(msg) => println!("Success: {msg}"),
    core::result::Result::Err((report, err)) => println!("Error: {report} - {err:?}"),
  }
}
