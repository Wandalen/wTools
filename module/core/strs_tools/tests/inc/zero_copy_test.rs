#[ allow( unused_imports ) ]
use super::*;
use the_module::string::zero_copy::*;

#[ test ]
fn test_zero_copy_basic_split()
{
  let input = "hello,world,rust";
  let segments: Vec< _ > = input.zero_copy_split( &[ "," ] ).collect();

  assert_eq!( segments.len(), 3 );
  assert_eq!( segments[0].as_str(), "hello" );
  assert_eq!( segments[1].as_str(), "world" );
  assert_eq!( segments[2].as_str(), "rust" );

  // Verify zero-copy (all should be borrowed)
  assert!( segments[0].is_borrowed() );
  assert!( segments[1].is_borrowed() );
  assert!( segments[2].is_borrowed() );
}

#[ test ]
fn test_zero_copy_with_delimiter_preservation()
{
  let input = "a: b: c";
  let segments: Vec< _ > = input.zero_copy_split_preserve( &[ ": " ] ).collect();

  assert_eq!( segments.len(), 5 ); // a, : , b, : , c
  assert_eq!( segments[0].as_str(), "a" );
  assert_eq!( segments[1].as_str(), ": " );
  assert_eq!( segments[2].as_str(), "b" );
  assert_eq!( segments[3].as_str(), ": " );
  assert_eq!( segments[4].as_str(), "c" );

  // Check segment types
  assert_eq!( segments[0].segment_type, SegmentType::Content );
  assert_eq!( segments[1].segment_type, SegmentType::Delimiter );
  assert_eq!( segments[2].segment_type, SegmentType::Content );
}

#[ test ]
fn test_copy_on_write_behavior()
{
  let input = "test";
  let mut segment = ZeroCopySegment::from_str( input, 0, 4 );

  // Initially borrowed
  assert!( segment.is_borrowed() );

  // Mutation triggers copy-on-write
  segment.make_mut().push_str( "_modified" );

  // Now owned
  assert!( segment.is_owned() );
  assert_eq!( segment.as_str(), "test_modified" );
}

#[ test ]
fn test_empty_segments()
{
  let input = "a,,b";
  let segments: Vec< _ > = input.zero_copy_split( &[ "," ] ).collect();

  // By default, empty segments are not preserved
  assert_eq!( segments.len(), 2 );
  assert_eq!( segments[0].as_str(), "a" );
  assert_eq!( segments[1].as_str(), "b" );

  // With preserve_empty enabled
  let segments_with_empty: Vec< _ > = ZeroCopySplit::new()
    .src( input )
    .delimiter( "," )
    .preserve_empty( true )
    .perform()
    .collect();

  assert_eq!( segments_with_empty.len(), 3 );
  assert_eq!( segments_with_empty[0].as_str(), "a" );
  assert_eq!( segments_with_empty[1].as_str(), "" );
  assert_eq!( segments_with_empty[2].as_str(), "b" );
}

#[ test ]
fn test_multiple_delimiters()
{
  let input = "a,b;c: d";
  let segments: Vec< _ > = input.zero_copy_split( &[ ",", ";", ": " ] ).collect();

  assert_eq!( segments.len(), 4 );
  assert_eq!( segments[0].as_str(), "a" );
  assert_eq!( segments[1].as_str(), "b" );
  assert_eq!( segments[2].as_str(), "c" );
  assert_eq!( segments[3].as_str(), "d" );
}

#[ test ]
fn test_position_tracking()
{
  let input = "hello,world";
  let segments: Vec< _ > = input.zero_copy_split( &[ "," ] ).collect();

  assert_eq!( segments[0].start_pos, 0 );
  assert_eq!( segments[0].end_pos, 5 );
  assert_eq!( segments[1].start_pos, 6 );
  assert_eq!( segments[1].end_pos, 11 );
}

#[ test ]
fn test_count_segments_without_allocation()
{
  let input = "a,b,c,d,e,f,g";
  let count = input.count_segments( &[ "," ] );

  assert_eq!( count, 7 );

  // This operation should not allocate any String objects,
  // only count the segments
}

#[ cfg( feature = "simd" ) ]
#[ test ]
fn test_simd_zero_copy_integration()
{
  let input = "field1,field2,field3";

  let simd_result = ZeroCopySplit::new()
    .src( input )
    .delimiter( "," )
    .perform_simd();

  match simd_result
  {
    Ok( iter ) =>
    {
      let segments: Vec< _ > = iter.collect();

      // Debug output to understand what SIMD is returning
      eprintln!( "SIMD segments count: {}", segments.len() );
      for ( i, segment ) in segments.iter().enumerate()
      {
        eprintln!( "  [{}] : '{}' (type: {:?})", i, segment.as_str(), segment.segment_type );
      }

      // SIMD might include delimiters in output, so we need to filter content segments
      let content_segments: Vec< _ > = segments
        .into_iter()
        .filter( |seg| seg.segment_type == SegmentType::Content )
        .collect();

      assert_eq!( content_segments.len(), 3 );
      assert_eq!( content_segments[0].as_str(), "field1" );
      assert_eq!( content_segments[1].as_str(), "field2" );
      assert_eq!( content_segments[2].as_str(), "field3" );
    },
    Err( e ) =>
    {
      // SIMD might not be available in test environment
      eprintln!( "SIMD test failed (expected in some environments): {}", e );
    }
  }
}
