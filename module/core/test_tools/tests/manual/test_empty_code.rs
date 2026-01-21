//! Manual test: Empty code string handling
//!
//! This test verifies that SmokeModuleTest handles empty code strings correctly.
//! Expected: Should succeed (empty Rust project is valid)

use test_tools::*;

fn main()
{
  println!("Testing empty code string...");

  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");
  smoke_test.code("".to_string());

  match smoke_test.form()
  {
    Ok(_) => println!("✓ form() succeeded"),
    Err(e) => {
      println!("✗ form() failed: {}", e);
      return;
    }
  }

  match smoke_test.perform()
  {
    Ok(_) => println!("✓ perform() succeeded"),
    Err(e) => println!("✗ perform() failed (this may be expected): {}", e),
  }

  match smoke_test.clean(true)
  {
    Ok(_) => println!("✓ cleanup succeeded"),
    Err(e) => println!("✗ cleanup failed: {}", e),
  }

  println!("\nResult: Empty code handling test completed");
}
