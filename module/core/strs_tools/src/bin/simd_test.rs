//! Quick SIMD functionality test
//! 
//! Tests that SIMD string operations are working correctly and shows
//! basic performance characteristics.

use std::time::Instant;
use strs_tools::string::split;

#[ cfg( feature = "simd" ) ]
use strs_tools::simd::SIMDStringExt;

fn main() 
{
  println!( "🚀 SIMD String Operations Test" );
  println!( "===============================" );
  
  // Test data
  let test_input = "namespace:command:arg1,value1;arg2,value2.option1!flag1#config1";
  let delimiters = [ ":", ",", ";", ".", "!", "#" ];
  
  println!( "📝 Test input: {}", test_input );
  println!( "🔍 Delimiters: {:?}", delimiters );
  println!();
  
  // Test scalar implementation
  println!( "⚡ Scalar Implementation:" );
  let start = Instant::now();
  let scalar_result: Vec< _ > = split()
    .src( test_input )
    .delimeter( delimiters.to_vec() )
    .perform()
    .collect();
  let scalar_time = start.elapsed();
  
  println!( "   Time: {:?}", scalar_time );
  println!( "   Results: {} segments", scalar_result.len() );
  for ( i, segment ) in scalar_result.iter().enumerate() 
  {
    println!( "     [{}]: '{}' ({:?})", i, segment.string, segment.typ );
  }
  println!();
  
  // Test SIMD implementation if available
  #[ cfg( feature = "simd" ) ]
  {
    println!( "🏎️  SIMD Implementation:" );
    let start = Instant::now();
    match test_input.simd_split( &delimiters ) 
    {
      Ok( iter ) => 
      {
        let simd_result: Vec< _ > = iter.collect();
        let simd_time = start.elapsed();
        
        println!( "   Time: {:?}", simd_time );
        println!( "   Results: {} segments", simd_result.len() );
        for ( i, segment ) in simd_result.iter().enumerate() 
        {
          println!( "     [{}]: '{}' ({:?})", i, segment.string, segment.typ );
        }
        
        // Compare performance
        if scalar_time > simd_time 
        {
          let speedup = scalar_time.as_nanos() as f64 / simd_time.as_nanos() as f64;
          println!( "   🎯 SIMD is {:.2}x faster!", speedup );
        } 
        else 
        {
          let slowdown = simd_time.as_nanos() as f64 / scalar_time.as_nanos() as f64;
          println!( "   ⚠️ SIMD is {:.2}x slower (small input overhead)", slowdown );
        }
        
        // Verify results match
        if scalar_result.len() == simd_result.len() 
        {
          let mut all_match = true;
          for ( scalar, simd ) in scalar_result.iter().zip( simd_result.iter() ) 
          {
            if scalar.string != simd.string || scalar.typ != simd.typ 
            {
              all_match = false;
              break;
            }
          }
          
          if all_match 
          {
            println!( "   ✅ Results match perfectly!" );
          } 
          else 
          {
            println!( "   ❌ Results differ between implementations" );
          }
        } 
        else 
        {
          println!( "   ❌ Different number of segments: scalar={}, simd={}", 
            scalar_result.len(), simd_result.len() );
        }
      },
      Err( e ) => 
      {
        println!( "   ❌ SIMD failed: {}", e );
      }
    }
  }
  
  #[ cfg( not( feature = "simd" ) ) ]
  {
    println!( "⚠️ SIMD feature not enabled - compile with --features simd" );
  }
  
  println!();
  
  // Test other SIMD operations
  #[ cfg( feature = "simd" ) ]
  {
    println!( "🔎 SIMD Search Operations:" );
    
    // Test substring search
    let search_result = test_input.simd_find( "command" );
    println!( "   Find 'command': {:?}", search_result );
    
    // Test character counting
    let colon_count = test_input.simd_count( ':' );
    println!( "   Count ':': {}", colon_count );
    
    // Test multi-pattern search
    let patterns = [ "error", "command", "value" ];
    let multi_result = test_input.simd_find_any( &patterns );
    println!( "   Find any of {:?}: {:?}", patterns, multi_result );
  }
  
  println!();
  println!( "✨ Test completed!" );
}