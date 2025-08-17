//! # Example 001: Basic Runtime Assertions
//!
//! This example introduces the fundamental runtime assertion macros.
//! Start here to learn the basics of `diagnostics_tools`.
//!
//! ## What you'll learn:
//! - Basic runtime assertion macros (`a_true`, `a_false`) 
//! - How they compare to standard Rust assertions
//! - When to use each type
//!
//! ## Run this example:
//! ```bash
//! cargo run --example 001_basic_runtime_assertions
//! ```

use diagnostics_tools::*;

fn main()
{
  println!( "🚀 Welcome to diagnostics_tools!" );
  println!( "This example demonstrates basic runtime assertions.\n" );

  // ✅ Basic boolean assertions
  println!( "1. Testing basic boolean conditions:" );
  
  let number = 42;
  let is_even = number % 2 == 0;
  
  // Instead of assert!(condition), use a_true!(condition)
  a_true!( is_even, "Expected number to be even" );
  println!( "   ✓ {number} is even" );
  
  // Instead of assert!(!condition), use a_false!(condition)  
  a_false!( number < 0, "Expected number to be positive" );
  println!( "   ✓ {number} is positive" );

  // ✅ Assertions without custom messages work too
  println!( "\n2. Testing without custom messages:" );
  
  let name = "Alice";
  a_true!( !name.is_empty() );
  a_false!( name.is_empty() );
  println!( "   ✓ Name '{name}' is valid" );

  // ✅ Comparing with standard assertions
  println!( "\n3. Comparison with standard Rust assertions:" );
  
  // These do the same thing, but diagnostics_tools provides better error context:
  
  // Standard way:
  assert!( number > 0 );
  
  // Enhanced way (better error messages):
  a_true!( number > 0 );
  
  println!( "   ✓ Both assertion styles work" );

  // ✅ Common patterns
  println!( "\n4. Common assertion patterns:" );
  
  let items = ["apple", "banana", "cherry"];
  
  // Check collection properties
  a_true!( !items.is_empty(), "Items list should not be empty" );
  a_true!( items.len() == 3, "Expected exactly 3 items" );
  
  // Check string properties
  let text = "Hello, World!";
  a_true!( text.contains( "World" ), "Text should contain 'World'" );
  a_false!( text.starts_with( "Goodbye" ), "Text should not start with 'Goodbye'" );
  
  println!( "   ✓ All collection and string checks passed" );

  println!( "\n🎉 All basic assertions passed!" );
  println!( "\n💡 Key takeaways:" );
  println!( "   • Use a_true!() instead of assert!() for better error messages" );  
  println!( "   • Use a_false!() instead of assert!(!condition) for clarity" );
  println!( "   • Custom error messages are optional but helpful" );
  println!( "   • Same performance as standard assertions" );
  println!( "\n➡️  Next: Run example 002 to see better error message formatting!" );
}

// This function demonstrates how assertions help catch bugs
#[ allow( dead_code ) ]
fn demonstrate_assertion_failure()
{
  // Uncomment this line to see how assertion failures look:
  // a_true!( false, "This will fail and show a clear error message" );
  
  // The error will be much clearer than standard assertion failures!
}