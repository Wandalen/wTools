//!
//! These tests cover the combined functionality of quoting and unescaping in the `strs_tools::split` iterator.
//!

use super::*;

#[test]
fn mre_test()
{
  let src = r#"instruction "arg1" "arg2 \" "arg3 \\" "#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .stripping( false )
  .preserving_delimeters( false )
  .preserving_empty( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec!
  [
    "instruction",
    "arg1",
    "arg2 \" ",
    "arg3 \\",
  ];
  assert_eq!( splits, expected );
}

#[test]
fn no_quotes_test()
{
  let src = "a b c";
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "a", "b", "c" ];
  assert_eq!( splits, expected );
}

#[test]
fn empty_quoted_section_test()
{
  let src = r#"a "" b"#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_empty( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "a", "", "b" ];
  assert_eq!( splits, expected );
}

#[test]
fn multiple_escape_sequences_test()
{
  let src = r#" "a\n\t\"\\" b "#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "a\n\t\"\\", "b" ];
  assert_eq!( splits, expected );
}

#[test]
fn quoted_at_start_middle_end_test()
{
  let src = r#""start" middle "end""#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "start", "middle", "end" ];
  assert_eq!( splits, expected );
}

#[test]
fn unterminated_quote_test()
{
  let src = r#"a "b c"#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "a", "b c" ];
  assert_eq!( splits, expected );
}
#[test]
fn escaped_quote_only_test()
{
  let src = r#" "a\"b" "#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "a\"b" ];
  assert_eq!( splits, expected );
}

#[test]
fn escaped_backslash_only_test()
{
  let src = r#" "a\\b" "#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "a\\b" ];
  assert_eq!( splits, expected );
}

#[test]
fn escaped_backslash_then_quote_test()
{
  // This tests that the sequence `\\\"` correctly unescapes to `\"`.
  let src = r#" "a\\\"b" "#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ r#"a\"b"# ];
  assert_eq!( splits, expected );
}

#[test]
fn consecutive_escaped_backslashes_test()
{
  let src = r#" "a\\\\b" "#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ "a\\\\b" ];
  assert_eq!( splits, expected );
}
