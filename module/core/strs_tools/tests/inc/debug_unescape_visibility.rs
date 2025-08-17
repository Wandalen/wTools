//! Test for visibility of `test_unescape_str`.



include!( "./test_helpers.rs" );

#[ test ]
fn test_unescape_str_visibility()
{
  let input = r#"abc\""#;
  let expected = r#"abc""#;
  let result = test_unescape_str( input );
  assert_eq!( result, expected );
}