//! Trivial example of `proper_tools` usage
//!
//! This example demonstrates the minimal usage of the `proper_tools` crate.
//! Currently the crate is a placeholder with only a single function `f1()`.
//!
//! When the crate is fully implemented with actual utility functions,
//! this example will be updated to showcase core functionality.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example proper_tools_trivial
//! ```

fn main()
{
  println!( "proper_tools trivial example" );

  // Demonstrate placeholder functionality
  #[ cfg( feature = "enabled" ) ]
  {
    proper_tools::f1();
    println!( "✓ proper_tools::f1() executed successfully" );
  }

  #[ cfg( not( feature = "enabled" ) ) ]
  {
    println!( "⚠ 'enabled' feature not active - no functionality available" );
  }

  println!( "Example completed successfully" );
}
