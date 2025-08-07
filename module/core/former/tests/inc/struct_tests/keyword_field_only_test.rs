// File: module/core/former/tests/inc/former_tests/keyword_field_only_test.rs
use super::*;

#[ test ]
fn basic_construction()
{
  // Test using the generated former methods which should handle raw identifiers
  let got = KeywordFieldsStruct::former()
  .r#if( true )           // Setter for r#if field
  .r#type( "example".to_string() ) // Setter for r#type field
  .r#struct( 101 )        // Setter for r#struct field
  .form();

  let expected = KeywordFieldsStruct
  {
    r#if : true,
    r#type : "example".to_string(),
    r#struct : 101,
  };

  assert_eq!( got, expected );
}

#[ test ]
fn default_values()
{
  // Test that default values work even if fields are keywords
  // This relies on the struct deriving Default as well.
  let got = KeywordFieldsStruct::former().form();
  let expected = KeywordFieldsStruct::default(); // Assuming Default derive
  assert_eq!( got, expected );
}