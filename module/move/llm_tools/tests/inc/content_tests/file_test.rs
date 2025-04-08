
use super::*;
use core::fmt;
use std::convert::TryFrom;
// Helper to create a default Source for tests using Vec<u8>
fn default_test_source() -> the_module::content::Source< Vec< u8 > >
{
  the_module::content::Source::< Vec< u8 > >
  {
    media_type : "application/octet-stream".to_string(),
    encoding : "binary".to_string(),
    data : vec![ 1, 2, 3 ], // Use Vec<u8> directly
  }
}

/// Tests constructing a File using the `File::new()` method with Vec<u8> data.
#[ test ]
fn construct_with_new()
{
  use the_module::content::File; // Local use

  let source = default_test_source();
  let file = File::< Vec< u8 > >::new( source.clone() );
  assert_eq!( file.source, source );
}

/// Tests constructing a File using `Default::default()` with Vec<u8> data.
#[ test ]
fn construct_with_default()
{
  use the_module::content::{ File, Source }; // Local use

  let file_default = File::< Vec< u8 > >::default();
  let expected_source = Source::< Vec< u8 > >::default(); // Default source
  assert_eq!( file_default.source, expected_source );
  assert_eq!( file_default.source.media_type, "" );
  assert_eq!( file_default.source.encoding, "" );
  assert_eq!( file_default.source.data, Vec::< u8 >::default() ); // Compare with default Vec<u8>
}

/// Tests constructing a File using the `content::file()` shortcut function with Vec<u8> data.
#[ test ]
fn construct_with_function_constructor()
{
  use the_module::content; // Local use

  // Use the constructor function from the builder module
  let source = default_test_source();
  let file = content::file( source.clone() );
  assert_eq!( file.source, source );
}

/// Tests constructing a File using the `.file()` method on a Source builder chain with Vec<u8> data.
#[ test ]
fn construct_with_source_builder_method()
{
  use the_module::content; // Local use
  use the_module::content::File; // Local use

  let data = vec![ 10, 20, 30 ]; // Use Vec<u8> directly
  let source = content::source()
  .media_type( "text/plain".to_string() )
  .encoding( "utf-8".to_string() )
  .data( data.clone() );

  // Use the .file() method on the Source builder
  let file : File< Vec< u8 > > = source.clone().file();

  assert_eq!( file.source.media_type, "text/plain" );
  assert_eq!( file.source.encoding, "utf-8" );
  assert_eq!( file.source.data, data );
  assert_eq!( file.source, source ); // Ensure the whole source was transferred
}

/// Tests converting a File<Vec<u8>> into ContentAny using `into()`.
#[ test ]
fn convert_into_content_any()
{
  use the_module::content::{ File, ContentAny }; // Local use

  let file = File::< Vec< u8 > > { source : default_test_source() };
  let file_clone = file.clone(); // Clone for comparison

  // Use .into()
  let content : ContentAny< Vec< u8 > > = file.into();

  match content
  {
    ContentAny::File( inner_file ) =>
    {
      assert_eq!( inner_file, file_clone );
      assert_eq!( inner_file.source, file_clone.source );
    }
    _ =>
      panic!( "Conversion into ContentAny resulted in wrong variant" ),
  }
}

/// Tests converting a File<Vec<u8>> into ContentAny using `ContentAny::from()`.
#[ test ]
fn convert_from_file_for_content_any()
{
  use the_module::content::{ File, ContentAny }; // Local use

  let file = File::< Vec< u8 > > { source : default_test_source() };
  let file_clone = file.clone(); // Clone for comparison

  // Use ContentAny::from() - relies on derive(From)
  let content = ContentAny::from( file );

  match content
  {
    ContentAny::File( inner_file ) =>
    {
      assert_eq!( inner_file, file_clone );
      assert_eq!( inner_file.source, file_clone.source );
    }
    _ =>
      panic!( "Conversion using From<File> for ContentAny resulted in wrong variant" ),
  }
}

/// Tests extracting a File from a ContentAny::File variant using pattern matching with Vec<u8> data.
#[ test ]
fn try_from_content_any_match()
{
  use the_module::content::{ File, ContentAny, ContentLike }; // Local use
  use the_module::content; // Local use for builder functions

  // We can't directly TryFrom<ContentAny> into File,
  // but we can match on ContentAny to extract it.
  let file = File::< Vec< u8 > > { source : default_test_source() };
  let content : ContentAny< Vec< u8 > > = file.clone().into();

  match content
  {
    ContentAny::File( extracted_file ) =>
    {
      assert_eq!( extracted_file, file );
    }
    other =>
    {
      // Provide the type annotation for content_type here as well
      panic!( "Expected ContentAny::File, got {:?}", <ContentAny<Vec<u8>> as ContentLike<Vec<u8>>>::content_type(&other) );
    }
  }

  // Test that other variants don't match
  let content_string : ContentAny< Vec< u8 > > = content::string( "hello".to_string() );
  match content_string
  {
    ContentAny::File( _ ) =>
    {
      panic!( "Incorrectly matched ContentAny::String as File" );
    }
    _ =>
    {
      // Correctly does not match
    }
  }
}

/// Tests that `TryFrom<serde_json::Value>` fails for JSON objects representing media (using Vec<u8>).
#[ test ]
fn try_from_json_value_for_content_any()
{
  use the_module::content::ContentAny; // Local use
  use serde_json; // Local use

  // Test that TryFrom<serde_json::Value> for ContentAny *cannot* produce a File variant,
  // as data reconstruction isn't supported this way.
  let json_file_repr = serde_json::json!
  ({
    "type" : "file",
    "media_type" : "application/pdf",
    "encoding" : "base64"
    // No data field - this is how media is represented in the current From<ContentAny> impl
  });

  let result = ContentAny::< Vec< u8 > >::try_from( json_file_repr );

  assert!( result.is_err(), "Conversion from JSON object should fail" );
  match result
  {
    Err( the_module::content::Error::UnsupportedType( msg ) ) =>
    {
      assert!( msg.contains( "Object" ) );
    }
    _ =>
      panic!( "Expected UnsupportedType error" ),
  }

  // Test other JSON types
  let json_string = serde_json::json!( "just a string" );
  let content_result = ContentAny::< Vec< u8 > >::try_from( json_string );
  assert!( content_result.is_ok() );
  match content_result.unwrap()
  {
    ContentAny::String( s ) =>
      assert_eq!( s, "just a string" ),
    _ =>
      panic!( "Expected ContentAny::String" ),
  }
}

/// Tests adding a File<Vec<u8>> as an element to a ContentAny::Array using `push_to`.
#[ test ]
fn add_file_as_leaf_to_array()
{
  use the_module::content::{ File, ContentAny, ContentLike }; // Local use
  use the_module::content; // Local use for builder functions

  let file = File::< Vec< u8 > > { source : default_test_source() };
  let file_consumed_marker = file.clone(); // Clone to compare after consumption
  let mut content_array : ContentAny< Vec< u8 > > = content::array();

  // Use the push_to method from ContentLike trait
  // Specify the type S for ContentLike as Vec<u8>
  content_array = <the_module::content::File<Vec<u8>> as ContentLike<Vec<u8>>>::push_to( file, content_array );

  match content_array
  {
    ContentAny::Array( arr ) =>
    {
      assert_eq!( arr.len(), 1 );
      match &arr[ 0 ]
      {
        ContentAny::File( inner_file ) =>
        {
          // Compare with the clone made before consumption
          assert_eq!( inner_file, &file_consumed_marker );
        }
        _ =>
          panic!( "Element in array is not a File variant" ),
      }
    }
    _ =>
      panic!( "Content is not an Array variant after push" ),
  }
}

/// Tests that attempting to push content into a File<Vec<u8>> (which is not an array) panics.
#[ test ]
#[ should_panic ]
fn add_leaf_to_file_panics()
{
  use the_module::content::{ File, ContentAny }; // Local use
  use the_module::content; // Local use for builder functions

  // File cannot contain other content like Array does.
  // Attempting to push to it should panic.
  let file_content : ContentAny< Vec< u8 > > = File::< Vec< u8 > > { source : default_test_source() }.into();
  let string_content : ContentAny< Vec< u8 > > = content::string( "another leaf".to_string() );

  // This calls ContentAny::push which panics if not an Array
  let _ = ContentAny::push( file_content, string_content );
}

/// Tests the methods provided by the `ContentLike` trait for the File<Vec<u8>> type.
#[ test ]
fn content_like_trait_implementation()
{
  use the_module::content::{ File, ContentAny, ContentLike, ContentType }; // Local use
  use serde_json; // Local use

  let file = File::< Vec< u8 > > { source : default_test_source() };
  let file_clone = file.clone(); // Clone for methods that consume

  // content_type() - Provide explicit type annotation for S
  assert_eq!( <the_module::content::File<Vec<u8>> as ContentLike<Vec<u8>>>::content_type(&file), ContentType::File );

  // content_to_bytes() - Provide explicit type annotation for S
  let bytes = <the_module::content::File<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_bytes(file_clone);
  assert_eq!( bytes, vec![ 1, 2, 3 ] ); // Matches the data in default_test_source

  // content_to_json() - Provide explicit type annotation for S
  let json_val = <the_module::content::File<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_json(file.clone()); // Clone again as content_to_json consumes
  let expected_json = serde_json::json!
  ({
    "type" : "file",
    "media_type" : "application/octet-stream",
    "encoding" : "binary",
    // Data is omitted in this conversion
  });
  assert_eq!( json_val, expected_json );

  // into_any() - Converting File<Vec<u8>> to ContentAny<Vec<u8>>
  // Provide explicit type annotation for S
  let any_content : ContentAny< Vec< u8 > > = <the_module::content::File<Vec<u8>> as ContentLike<Vec<u8>>>::into_any(file); // Consumes file
  match any_content
  {
    ContentAny::File( inner_file ) =>
    {
      assert_eq!( inner_file.source.media_type, "application/octet-stream" );
      assert_eq!( inner_file.source.encoding, "binary" );
      // Compare Vec<u8> directly
      assert_eq!( inner_file.source.data, vec![ 1, 2, 3 ] );
    }
    _ =>
      panic!( "into_any() did not produce ContentAny::File" ),
  }
}

/// Tests the equality and inequality comparisons for File<Vec<u8>> instances.
#[ test ]
fn equality()
{
  use the_module::content::{ Source, File }; // Local use
  use the_module::content; // Local use for builder functions

  let source1 = Source::< Vec< u8 > >
  {
    media_type : "a".into(),
    encoding : "b".into(),
    data : vec![ 1 ] // Use Vec<u8> directly
  };
  let source2 = Source::< Vec< u8 > >
  {
    media_type : "a".into(),
    encoding : "b".into(),
    data : vec![ 1 ] // Use Vec<u8> directly
  };
  let source3 = Source::< Vec< u8 > >
  {
    media_type : "x".into(),
    encoding : "y".into(),
    data : vec![ 9 ] // Use Vec<u8> directly
  };

  let file1 = content::file( source1.clone() );
  let file2 = content::file( source2.clone() );
  let file3 = content::file( source3.clone() );

  assert_eq!( file1, file2 ); // Same source, should be equal
  assert_ne!( file1, file3 ); // Different source, should not be equal
  assert_ne!( file2, file3 ); // Different source, should not be equal
}