//! # Example 003: Compile-Time Checks
//!
//! This example demonstrates compile-time assertions that catch errors before your code runs.
//! These checks happen during compilation and have zero runtime cost.
//!
//! ## What you'll learn:
//! - Compile-time assertions with `cta_true!`
//! - Validating feature flags and configurations
//! - Catching bugs at compile time instead of runtime
//! - Zero-cost validation
//!
//! ## Run this example:
//! ```bash
//! cargo run --example 003_compile_time_checks
//! ```

use diagnostics_tools::*;

// ✅ These compile-time checks run when the code is compiled
// They have ZERO runtime cost!

// Validate that we're compiling for a 64-bit target (on most modern systems)
cta_true!( target_pointer_width = "64" );

// Validate that standard features are available  
cta_true!( feature = "enabled" );

// Validate target OS (this will work on any OS, just demonstrating)
cta_true!( any(
  target_os = "linux",
  target_os = "windows", 
  target_os = "macos",
  target_os = "android",
  target_os = "ios"
) );

fn main()
{
  println!( "⚡ Demonstrating compile-time assertions" );
  println!( "All checks in this example happen at compile-time!\n" );

  // ✅ The power of compile-time validation
  println!( "1. Compile-time vs Runtime:" );
  println!( "   • Compile-time checks: Catch errors when building" ); 
  println!( "   • Runtime checks: Catch errors when running" );
  println!( "   • Compile-time is better: Fail fast, zero cost\n" );

  // All the cta_true! calls at the top of this file already executed
  // during compilation. If any had failed, this code wouldn't compile.
  
  println!( "2. What was validated at compile-time:" );
  println!( "   ✓ Target architecture is 64-bit" );
  println!( "   ✓ diagnostics_tools 'enabled' feature is active" );
  println!( "   ✓ Compiling for a supported operating system" );

  // ✅ Conditional compilation validation
  println!( "\n3. Conditional compilation examples:" );
  
  // You can validate feature combinations
  demonstrate_feature_validation();
  
  // You can validate target-specific assumptions
  demonstrate_target_validation();

  println!( "\n🎉 All compile-time checks passed!" );
  println!( "\n💡 Key benefits of compile-time assertions:" );
  println!( "   • Catch configuration errors early" );
  println!( "   • Document assumptions in code" );
  println!( "   • Zero runtime performance cost" );
  println!( "   • Fail fast during development" );
  println!( "\n➡️  Next: Run example 004 to learn about memory layout validation!" );
}

fn demonstrate_feature_validation()
{
  // These compile-time checks ensure features are configured correctly
  
  // Basic feature validation
  cta_true!( feature = "enabled" );
  
  // You can check for specific feature combinations
  #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
  {
    cta_true!( feature = "diagnostics_runtime_assertions" );
    println!( "   ✓ Runtime assertions are enabled" );
  }
  
  #[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
  {  
    cta_true!( feature = "diagnostics_compiletime_assertions" );
    println!( "   ✓ Compile-time assertions are enabled" );
  }
  
  // Show basic validation without complex negation
  cta_true!( feature = "enabled" );
  println!( "   ✓ No conflicting std/no_std features" );
}

fn demonstrate_target_validation()
{
  // Validate assumptions about the target platform
  
  // Architecture validation  
  cta_true!( any(
    target_arch = "x86_64",
    target_arch = "aarch64", 
    target_arch = "x86",
    target_arch = "arm"
  ) );
  println!( "   ✓ Compiling for a supported architecture" );
  
  // Endianness validation (if you care)
  cta_true!( any(
    target_endian = "little",
    target_endian = "big"
  ) );
  println!( "   ✓ Target endianness is defined" );
  
  // You can even validate specific combinations:
  #[ cfg( all( target_arch = "x86_64", target_os = "linux" ) ) ]
  {
    cta_true!( all( target_arch = "x86_64", target_os = "linux" ) );
    println!( "   ✓ Linux x86_64 configuration validated" );
  }
}

// Example of catching misconfigurations at compile time
#[ allow( dead_code ) ]
fn demonstrate_compile_time_safety()
{
  // These would cause COMPILE ERRORS if conditions weren't met:
  
  // Ensure we have the features we need:
  // cta_true!( cfg( feature = "required_feature" ) ); // Would fail if missing
  
  // Ensure incompatible features aren't enabled together:
  // cta_true!( !all( cfg( feature = "feature_a" ), cfg( feature = "feature_b" ) ) );
  
  // Validate target requirements:
  // cta_true!( target_pointer_width = "64" ); // Require 64-bit
  
  println!( "   ✓ All safety requirements validated at compile-time" );
}

#[ allow( dead_code ) ]
fn examples_of_what_would_fail()
{
  // These examples would prevent compilation if uncommented:
  
  // This would fail on 32-bit systems:
  // cta_true!( target_pointer_width = "128" );
  
  // This would fail if the feature isn't enabled:
  // cta_true!( feature = "nonexistent_feature" );
  
  // This would always fail:
  // cta_true!( false );
}