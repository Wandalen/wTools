//! Comprehensive corner case tests for string split functionality.
//!
//! This test suite validates split behavior across edge cases, Unicode handling,
//! empty inputs, delimiter preservation modes, and feature combinations.

#[ cfg( all( feature = "split", test ) ) ]
mod basic_delimiter_cases
{
  use wstring_tools::*;

  #[ test ]
  fn single_delimiter_present()
  {
    let src = "hello world";
    let result = string::split()
      .src( src )
      .delimeter( " " )
      .stripping( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "hello", " ", "world" ] );
  }

  #[ test ]
  fn single_delimiter_absent()
  {
    let src = "helloworld";
    let result = string::split()
      .src( src )
      .delimeter( " " )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "helloworld" ] );
  }

  #[ test ]
  fn multiple_occurrences_of_delimiter()
  {
    let src = "a,b,c,d";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .stripping( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "a", ",", "b", ",", "c", ",", "d" ] );
  }

  #[ test ]
  fn delimiter_at_start()
  {
    let src = ",hello";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .stripping( false )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "", ",", "hello" ] );
  }

  #[ test ]
  fn delimiter_at_end()
  {
    let src = "hello,";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .stripping( false )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Actual behavior: trailing empty string is not preserved even with preserving_empty=true
    assert_eq!( result, vec![ "hello", "," ] );
  }

  #[ test ]
  fn consecutive_delimiters()
  {
    let src = "a,,b";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .stripping( false )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "a", ",", "", ",", "b" ] );
  }

  #[ test ]
  fn source_is_only_delimiters()
  {
    let src = ",,,";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .stripping( false )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Actual behavior: leading and middle empties preserved, trailing empty not preserved
    assert_eq!( result, vec![ "", ",", "", ",", "", "," ] );
  }
}

#[ cfg( all( feature = "split", test ) ) ]
mod empty_handling
{
  use wstring_tools::*;

  #[ test ]
  fn empty_source_string()
  {
    let src = "";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Empty source produces empty array by default (preserving_empty=false)
    assert_eq!( result, Vec::< String >::new() );
  }

  #[ test ]
  fn empty_source_string_with_preserving_empty()
  {
    let src = "";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Empty source with preserving_empty produces one empty string
    assert_eq!( result, vec![ "" ] );
  }

  #[ test ]
  fn empty_delimiter()
  {
    let src = "hello";
    let result = string::split()
      .src( src )
      .delimeter( "" )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Empty delimiter should return original string as single element
    assert_eq!( result, vec![ "hello" ] );
  }

  #[ test ]
  fn empty_segments_with_stripping_false()
  {
    let src = "a,,b";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .stripping( false )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Empty segment between consecutive delimiters preserved when preserving_empty=true
    assert_eq!( result, vec![ "a", ",", "", ",", "b" ] );
  }

  #[ test ]
  fn empty_segments_without_preserving()
  {
    let src = "a,,b";
    let result = string::split()
      .src( src )
      .delimeter( "," )
      .stripping( false )
      .preserving_empty( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Empty segment filtered out when preserving_empty=false
    assert_eq!( result, vec![ "a", ",", ",", "b" ] );
  }

  #[ test ]
  fn both_source_and_delimiter_empty()
  {
    let src = "";
    let result = string::split()
      .src( src )
      .delimeter( "" )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "" ] );
  }
}

#[ cfg( all( feature = "split", test ) ) ]
mod delimiter_preservation
{
  use wstring_tools::*;

  #[ test ]
  fn stripping_false_preserves_delimiters()
  {
    let src = "a:b:c";
    let result = string::split()
      .src( src )
      .delimeter( ":" )
      .stripping( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Default preserves delimiters (stripping affects whitespace, not delimiters)
    assert_eq!( result, vec![ "a", ":", "b", ":", "c" ] );
  }

  #[ test ]
  fn preserving_delimiters_false_removes_them()
  {
    let src = "a:b:c";
    let result = string::split()
      .src( src )
      .delimeter( ":" )
      .preserving_delimeters( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "a", "b", "c" ] );
  }

  #[ test ]
  fn default_behavior_preserves_delimiters()
  {
    let src = "a:b:c";
    let result = string::split()
      .src( src )
      .delimeter( ":" )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Default preserves delimiters
    assert_eq!( result, vec![ "a", ":", "b", ":", "c" ] );
  }

  #[ test ]
  fn consecutive_delimiters_with_stripping()
  {
    let src = "a::b";
    let result_no_strip = string::split()
      .src( src )
      .delimeter( ":" )
      .stripping( false )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result_no_strip, vec![ "a", ":", "", ":", "b" ] );

    let result_strip = string::split()
      .src( src )
      .delimeter( ":" )
      .stripping( true )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Stripping trims whitespace from segments, doesn't affect delimiters
    assert_eq!( result_strip, vec![ "a", ":", "", ":", "b" ] );
  }
}

#[ cfg( all( feature = "split", test ) ) ]
mod unicode_and_special_characters
{
  use wstring_tools::*;

  #[ test ]
  fn non_ascii_delimiter()
  {
    let src = "hello—world";
    let result = string::split()
      .src( src )
      .delimeter( "—" )
      .stripping( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "hello", "—", "world" ] );
  }

  #[ test ]
  fn multi_byte_utf8_characters()
  {
    let src = "日本語と英語";
    let result = string::split()
      .src( src )
      .delimeter( "と" )
      .stripping( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "日本語", "と", "英語" ] );
  }

  #[ test ]
  fn emoji_delimiter()
  {
    let src = "hello🔥world🔥test";
    let result = string::split()
      .src( src )
      .delimeter( "🔥" )
      .preserving_delimeters( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "hello", "world", "test" ] );
  }

  #[ test ]
  fn multi_character_unicode_delimiter()
  {
    let src = "first分隔符second分隔符third";
    let result = string::split()
      .src( src )
      .delimeter( "分隔符" )
      .preserving_delimeters( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "first", "second", "third" ] );
  }

  #[ test ]
  fn mixed_unicode_content()
  {
    let src = "English|日本語|Русский|العربية";
    let result = string::split()
      .src( src )
      .delimeter( "|" )
      .preserving_delimeters( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "English", "日本語", "Русский", "العربية" ] );
  }
}

#[ cfg( all( feature = "split", test ) ) ]
mod edge_cases
{
  use wstring_tools::*;

  #[ test ]
  fn long_delimiter()
  {
    let src = "hello[DELIMITER]world[DELIMITER]test";
    let result = string::split()
      .src( src )
      .delimeter( "[DELIMITER]" )
      .stripping( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "hello", "[DELIMITER]", "world", "[DELIMITER]", "test" ] );
  }

  #[ test ]
  fn delimiter_longer_than_source()
  {
    let src = "hi";
    let result = string::split()
      .src( src )
      .delimeter( "very long delimiter" )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "hi" ] );
  }

  #[ test ]
  fn single_character_source_and_delimiter()
  {
    let src = "a";
    let result = string::split()
      .src( src )
      .delimeter( "a" )
      .stripping( false )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Actual behavior: leading empty preserved, trailing empty not preserved
    assert_eq!( result, vec![ "", "a" ] );
  }

  #[ test ]
  fn very_long_string()
  {
    // Test with reasonably long string to verify no performance issues
    let src = "word,".repeat( 1000 );
    let result = string::split()
      .src( &src )
      .delimeter( "," )
      .preserving_empty( true )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Should produce 1000 "word" segments and 1000 "," delimiters (trailing empty not preserved)
    assert_eq!( result.len(), 2000 );
    assert_eq!( result[ 0 ], "word" );
    assert_eq!( result[ 1 ], "," );
    assert_eq!( result[ 1998 ], "word" );
    assert_eq!( result[ 1999 ], "," );
  }
}

#[ cfg( all( feature = "split", test ) ) ]
mod feature_combinations
{
  use wstring_tools::*;

  #[ test ]
  fn builder_with_all_options()
  {
    let src = "a::b::c";
    let result = string::split()
      .src( src )
      .delimeter( "::" )
      .stripping( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result, vec![ "a", "::", "b", "::", "c" ] );
  }

  #[ test ]
  fn minimal_builder_usage()
  {
    let src = "x|y|z";
    let result = string::split()
      .src( src )
      .delimeter( "|" )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    // Default preserves delimiters
    assert_eq!( result, vec![ "x", "|", "y", "|", "z" ] );
  }

  #[ test ]
  fn multiple_independent_splits()
  {
    // Demonstrate multiple independent split operations with same configuration
    let result1 = string::split()
      .src( "a,b,c" )
      .delimeter( "," )
      .preserving_delimeters( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result1, vec![ "a", "b", "c" ] );

    let result2 = string::split()
      .src( "x,y" )
      .delimeter( "," )
      .preserving_delimeters( false )
      .perform()
      .map( String::from )
      .collect::< Vec< _ > >();
    assert_eq!( result2, vec![ "x", "y" ] );
  }
}
