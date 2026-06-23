//! SIMD-accelerated splitting tests.
//!
//! Verifies that the SIMD code path produces identical results to the scalar path.

#![ cfg( all( feature = "simd", feature = "std" ) ) ]

use strs_tools::string::split::own::
{
  BasicSplitBuilder,
  simd_split_cached,
};

/// Verifies SIMD and scalar paths yield identical segments for a single delimiter.
#[ test ]
fn simd_scalar_equivalence_single_delimiter()
{
  let input = "hello,world,foo,bar";

  let scalar: Vec< _ > = BasicSplitBuilder::new()
    .src( input )
    .delimiter( "," )
    .preserving_delimiters( true )
    .perform()
    .collect();

  let simd: Vec< _ > = simd_split_cached( input, &[ "," ] )
    .expect( "SIMD split should succeed" )
    .collect();

  assert_eq!( scalar.len(), simd.len(), "segment count mismatch: scalar={} simd={}", scalar.len(), simd.len() );
  for ( i, ( s, si ) ) in scalar.iter().zip( simd.iter() ).enumerate()
  {
    assert_eq!( s.string.as_ref(), si.string.as_ref(), "segment {i} content mismatch" );
    assert_eq!( s.typ, si.typ, "segment {i} type mismatch" );
  }
}

/// Verifies SIMD and scalar paths yield identical segments for multiple delimiters.
#[ test ]
fn simd_scalar_equivalence_multi_delimiter()
{
  let input = "a,b::c;d";
  let delimiters = &[ ",", "::", ";" ];

  let scalar: Vec< _ > = BasicSplitBuilder::new()
    .src( input )
    .delimiters( delimiters )
    .preserving_delimiters( true )
    .perform()
    .collect();

  let simd: Vec< _ > = simd_split_cached( input, delimiters )
    .expect( "SIMD split should succeed" )
    .collect();

  assert_eq!( scalar.len(), simd.len(), "segment count mismatch: scalar={} simd={}", scalar.len(), simd.len() );
  for ( i, ( s, si ) ) in scalar.iter().zip( simd.iter() ).enumerate()
  {
    assert_eq!( s.string.as_ref(), si.string.as_ref(), "segment {i} content mismatch" );
  }
}

/// Verifies pattern caching produces correct results on repeated calls.
#[ test ]
fn simd_pattern_caching_reuse()
{
  let delimiters = &[ ",", ";" ];

  let first: Vec< _ > = simd_split_cached( "a,b;c", delimiters )
    .expect( "first call should succeed" )
    .collect();

  let second: Vec< _ > = simd_split_cached( "x,y;z", delimiters )
    .expect( "cached delimiters should succeed" )
    .collect();

  assert_eq!( first.len(), second.len(), "cached and fresh should yield same segment count" );
  assert_eq!( first[ 0 ].string.as_ref(), "a", "first call first segment" );
  assert_eq!( second[ 0 ].string.as_ref(), "x", "second call first segment" );
}

/// Verifies SIMD split handles empty input without panic.
#[ test ]
fn simd_empty_input()
{
  let result: Vec< _ > = simd_split_cached( "", &[ "," ] )
    .expect( "empty input should not fail" )
    .collect();

  assert!( result.is_empty(), "empty input should produce no segments, got {}", result.len() );
}
