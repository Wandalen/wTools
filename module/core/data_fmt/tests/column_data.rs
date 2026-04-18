//! Tests for ColumnData structure

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use data_fmt::ColumnData;

// =============================================================================
// Basic Construction Tests
// =============================================================================

#[ test ]
fn test_column_data_new_empty()
{
  let data = ColumnData::new( vec![] );

  assert_eq!( data.columns.len(), 0 );
  assert!( data.is_empty() );
  assert_eq!( data.len(), 0 );
}

#[ test ]
fn test_column_data_new_single_column()
{
  let data = ColumnData::new( vec![ "value".to_string() ] );

  assert_eq!( data.columns.len(), 1 );
  assert!( !data.is_empty() );
  assert_eq!( data.len(), 1 );
  assert_eq!( data.columns[ 0 ].render(), "value" );
}

#[ test ]
fn test_column_data_new_two_columns()
{
  let data = ColumnData::new( vec![
    "name".to_string(),
    "version".to_string()
  ]);

  assert_eq!( data.len(), 2 );
  assert_eq!( data.columns[ 0 ].render(), "name" );
  assert_eq!( data.columns[ 1 ].render(), "version" );
}

#[ test ]
fn test_column_data_new_three_columns()
{
  let data = ColumnData::new( vec![
    "api_ollama".to_string(),
    "v0.1.0".to_string(),
    "(api/ollama)".to_string()
  ]);

  assert_eq!( data.len(), 3 );
  assert_eq!( data.columns[ 0 ].render(), "api_ollama" );
  assert_eq!( data.columns[ 1 ].render(), "v0.1.0" );
  assert_eq!( data.columns[ 2 ].render(), "(api/ollama)" );
}

#[ test ]
fn test_column_data_new_many_columns()
{
  let columns : Vec< String > = ( 0..10 )
    .map( | i | format!( "column_{}", i ) )
    .collect();

  let data = ColumnData::new( columns );

  assert_eq!( data.len(), 10 );
  assert_eq!( data.columns[ 0 ].render(), "column_0" );
  assert_eq!( data.columns[ 9 ].render(), "column_9" );
}

// =============================================================================
// from_pairs Tests
// =============================================================================

#[ test ]
fn test_column_data_from_pairs_empty()
{
  let data = ColumnData::from_pairs( vec![] );

  assert_eq!( data.len(), 0 );
  assert!( data.is_empty() );
}

#[ test ]
fn test_column_data_from_pairs_single()
{
  let data = ColumnData::from_pairs( vec![
    ( "name", "api_ollama" )
  ]);

  assert_eq!( data.len(), 1 );
  assert_eq!( data.columns[ 0 ].render(), "api_ollama" );
}

#[ test ]
fn test_column_data_from_pairs_multiple()
{
  let data = ColumnData::from_pairs( vec![
    ( "name", "api_ollama" ),
    ( "version", "v0.1.0" ),
    ( "path", "(api/ollama)" )
  ]);

  assert_eq!( data.len(), 3 );
  assert_eq!( data.columns[ 0 ].render(), "api_ollama" );
  assert_eq!( data.columns[ 1 ].render(), "v0.1.0" );
  assert_eq!( data.columns[ 2 ].render(), "(api/ollama)" );
}

#[ test ]
fn test_column_data_from_pairs_keys_discarded()
{
  let data = ColumnData::from_pairs( vec![
    ( "key1", "value1" ),
    ( "different_key", "value2" )
  ]);

  // Keys are discarded, only values retained
  assert_eq!( data.len(), 2 );
  assert_eq!( data.columns[ 0 ].render(), "value1" );
  assert_eq!( data.columns[ 1 ].render(), "value2" );
}

// =============================================================================
// Display Tests
// =============================================================================

#[ test ]
fn test_column_data_display_empty()
{
  let data = ColumnData::new( vec![] );
  let output = format!( "{}", data );

  assert_eq!( output, "" );
}

#[ test ]
fn test_column_data_display_single()
{
  let data = ColumnData::new( vec![ "value".to_string() ] );
  let output = format!( "{}", data );

  assert_eq!( output, "value" );
}

#[ test ]
fn test_column_data_display_multiple()
{
  let data = ColumnData::new( vec![
    "name".to_string(),
    "version".to_string(),
    "path".to_string()
  ]);
  let output = format!( "{}", data );

  assert_eq!( output, "name | version | path" );
}

// =============================================================================
// Clone Tests
// =============================================================================

#[ test ]
fn test_column_data_clone()
{
  let data1 = ColumnData::new( vec![
    "value1".to_string(),
    "value2".to_string()
  ]);

  let data2 = data1.clone();

  assert_eq!( data1.len(), data2.len() );
  assert_eq!( data1.columns, data2.columns );
}

#[ test ]
fn test_column_data_clone_independence()
{
  let mut data1 = ColumnData::new( vec![ "original".to_string() ] );
  let data2 = data1.clone();

  // Modify original
  data1.columns[ 0 ] = "modified".into();

  // Clone should be unchanged
  assert_eq!( data2.columns[ 0 ].render(), "original" );
  assert_eq!( data1.columns[ 0 ].render(), "modified" );
}

// =============================================================================
// Edge Cases
// =============================================================================

#[ test ]
fn test_column_data_with_empty_strings()
{
  let data = ColumnData::new( vec![
    "".to_string(),
    "value".to_string(),
    "".to_string()
  ]);

  assert_eq!( data.len(), 3 );
  assert_eq!( data.columns[ 0 ].render(), "" );
  assert_eq!( data.columns[ 1 ].render(), "value" );
  assert_eq!( data.columns[ 2 ].render(), "" );
}

#[ test ]
fn test_column_data_with_unicode()
{
  let data = ColumnData::new( vec![
    "日本語".to_string(),
    "emoji_😀".to_string(),
    "Русский".to_string()
  ]);

  assert_eq!( data.len(), 3 );
  assert_eq!( data.columns[ 0 ].render(), "日本語" );
  assert_eq!( data.columns[ 1 ].render(), "emoji_😀" );
  assert_eq!( data.columns[ 2 ].render(), "Русский" );
}

#[ test ]
fn test_column_data_with_long_strings()
{
  let long_string = "a".repeat( 1000 );
  let data = ColumnData::new( vec![ long_string.clone() ] );

  assert_eq!( data.len(), 1 );
  assert_eq!( data.columns[ 0 ].text.len(), 1000 );
  assert_eq!( data.columns[ 0 ].render(), long_string );
}

#[ test ]
fn test_column_data_with_special_chars()
{
  let data = ColumnData::new( vec![
    "tab\there".to_string(),
    "new\nline".to_string(),
    "quote\"test".to_string()
  ]);

  assert_eq!( data.len(), 3 );
  assert!( data.columns[ 0 ].text.contains( '\t' ) );
  assert!( data.columns[ 1 ].text.contains( '\n' ) );
  assert!( data.columns[ 2 ].text.contains( '"' ) );
}
