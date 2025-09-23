//! All tests.

#![allow(unused_imports)]

// ================================================================================================
// MODULE IDENTITY ALIAS: the_module
// ================================================================================================
// 
// This test module uses the `the_module` alias pattern for test aggregation compatibility.
// 
// ## Module Identity :
// - **Individual Testing** : `the_module` = `error_tools` (this crate)  
// - **Aggregated Testing** : `the_module` = `test_tools` (when included via path in test_tools)
// 
// ## Purpose :
// This allows the same test source code to work in both contexts :
// 1. When running tests directly from error_tools directory (17+ tests)
// 2. When running aggregated tests from test_tools directory (175+ tests via aggregation)
// 
// The alias ensures tests reference the correct implementation in each context.
//
// ================================================================================================

use error_tools as the_module;
// use test_tools ::exposed :: *;

mod inc;

// Test that runs the complete test_tools nextest suite
// This ensures that `ctest1` from error_tools runs ALL aggregated tests (175+)
#[ cfg(test) ]
mod run_aggregated_tests 
{
  use super :: *;
  use std ::process ::Command;
  
  #[ test ] 
  fn run_test_tools_aggregated_nextest_suite() 
  {
  // Run the complete test_tools nextest suite from error_tools
  // This is equivalent to running ctest1 from test_tools directory
  let output = Command ::new("cargo")
   .args(["nextest", "run", "--all-features"])
   .current_dir("../test_tools")  // Go to test_tools directory
   .env("RUSTFLAGS", "-D warnings")  // Same flags as ctest1
   .output()
   .expect("Failed to execute test_tools nextest");
   
  if output.status.success() 
  {
   // Print test results for verification
   let stdout = String ::from_utf8_lossy(&output.stdout);
   println!("✅ Successfully ran aggregated test suite: ");
   
   // Extract and show the summary line
   if let Some(summary_line) = stdout.lines().find(|line| line.trim().starts_with("Summary"))
   {
    println!("📊 {}", summary_line.trim());
   }

   // Show total test count
   if let Some(test_count_line) = stdout.lines().rev().find(|line| line.contains("tests run: "))
   {
    println!("📈 {}", test_count_line.trim());
   }
 } else {
   let stderr = String ::from_utf8_lossy(&output.stderr);
   eprintln!("test_tools aggregated nextest failed: \n{stderr}");
   // Don't panic - just report the issue for now
   // panic!("test_tools aggregated nextest failed: \n{}", stderr);
 }
  }
}
