//! Simple test to verify compile-time optimization macros work.

#[ allow( unused_imports ) ]
use strs_tools::*;

fn main() {
  println!( "Testing compile-time pattern optimization..." );
  
  #[ cfg( all( feature = "compile_time_optimizations", feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
    use strs_tools::string::zero_copy::ZeroCopyStringExt;
    
    // Test basic functionality without macros first
    let input = "a,b,c";
    let result: Vec<_> = input.zero_copy_split( &[","] ).collect();
    
    println!( "Zero-copy split result: {:?}", 
             result.iter().map( |s| s.as_str() ).collect::< Vec<_> >() );
    
    // Note: Macro testing disabled - optimize_split! macro not yet fully implemented
    println!( "ℹ️  Compile-time optimization macros are prototype features" );
    println!( "   The optimize_split! macro is not yet fully implemented" );
  }
  
  #[ cfg( not( all( feature = "compile_time_optimizations", feature = "string_split", not( feature = "no_std" ) ) ) ) ]
  {
    println!( "Compile-time optimizations or string_split feature not enabled" );
    println!( "Enable with: --features compile_time_optimizations,string_split" );
  }
}