//! Tests for the unescaping functionality.

use strs_tools::string::split::*;
use std::borrow::Cow;

#[test]
fn no_escapes()
{
  let input = "hello world";
  let result = unescape_str( input );
  assert!( matches!( result, Cow::Borrowed( _ ) ) );
  assert_eq!( result, "hello world" );
}

#[test]
fn valid_escapes()
{
  let input = r#"hello \"world\\, \n\t\r end"#;
  let expected = "hello \"world\\, \n\t\r end";
  let result = unescape_str( input );
  assert!( matches!( result, Cow::Owned( _ ) ) );
  assert_eq!( result, expected );
}

#[test]
fn mixed_escapes()
{
  let input = r#"a\"b\\c\nd"#;
  let expected = "a\"b\\c\nd";
  let result = unescape_str( input );
  assert!( matches!( result, Cow::Owned( _ ) ) );
  assert_eq!( result, expected );
}

#[test]
fn unrecognized_escape()
{
  let input = r"hello \z world";
  let result = unescape_str( input );
  assert!( matches!( result, Cow::Owned( _ ) ) );
  assert_eq!( result, r"hello \z world" );
}

#[test]
fn empty_string()
{
  let input = "";
  let result = unescape_str( input );
  assert!( matches!( result, Cow::Borrowed( _ ) ) );
  assert_eq!( result, "" );
}

#[test]
fn trailing_backslash()
{
  let input = r"hello\";
  let result = unescape_str( input );
  assert!( matches!( result, Cow::Owned( _ ) ) );
  assert_eq!( result, r"hello\" );
}

#[test]
fn unescape_trailing_escaped_quote()
{
  let input = r#"abc\""#;
  let expected = r#"abc""#;
  let result = unescape_str( input );
  assert!( matches!( result, Cow::Owned( _ ) ) );
  assert_eq!( result, expected );
}