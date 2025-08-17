//! Simple test to verify compile-time optimization macros work.

#[ allow( unused_imports ) ]
use strs_tools::*;

fn main() {
  println!( "Testing compile-time pattern optimization..." );
  
  #[ cfg( all( feature = "compile_time_optimizations", feature = "string_split" ) ) ]
  {
    use strs_tools::string::zero_copy::ZeroCopyStringExt;
    
    // Test basic functionality without macros first
    let input = "a,b,c";
    let result: Vec<_> = input.zero_copy_split( &[","] ).collect();
    
    println!( "Zero-copy split result: {:?}", 
             result.iter().map( |s| s.as_str() ).collect::< Vec<_> >() );
    
    // Test the macro
    #[ cfg( feature = "compile_time_optimizations" ) ]
    {
      use strs_tools::optimize_split;
      
      // This should work if the macro generates correct code
      let optimized: Vec<_> = optimize_split!( input, "," ).collect();
      println!( "Compile-time optimized result: {:?}", 
               optimized.iter().map( |s| s.as_str() ).collect::< Vec<_> >() );
      
      println!( "✓ Compile-time optimization working!" );
    }
  }
  
  #[ cfg( not( all( feature = "compile_time_optimizations", feature = "string_split" ) ) ) ]
  {
    println!( "Compile-time optimizations or string_split feature not enabled" );
    println!( "Enable with: --features compile_time_optimizations,string_split" );
  }
}