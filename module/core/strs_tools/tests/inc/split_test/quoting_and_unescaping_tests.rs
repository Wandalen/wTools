//!
//! These tests cover the combined functionality of quoting and unescaping in the `strs_tools::split` iterator.
//!

use super::*;
use std::borrow::Cow;

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
    Cow::Borrowed("instruction"),
    Cow::Borrowed("arg1"),
    Cow::Borrowed("arg2 \" "),
    Cow::Borrowed("arg3 \\"),
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
  let expected = vec![ Cow::Borrowed("a"), Cow::Borrowed("b"), Cow::Borrowed("c") ];
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
  let expected = vec![ Cow::Borrowed("a"), Cow::Borrowed(""), Cow::Borrowed("b") ];
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
  let expected = vec![ Cow::Borrowed("a\n\t\"\\"), Cow::Borrowed("b") ];
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
  let expected = vec![ Cow::Borrowed("start"), Cow::Borrowed("middle"), Cow::Borrowed("end") ];
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
  let expected = vec![ Cow::Borrowed("a"), Cow::Borrowed("b c") ];
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
  let expected = vec![ Cow::Borrowed("a\"b") ];
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
  let expected = vec![ Cow::Borrowed("a\\b") ];
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
  let expected = vec![ Cow::Borrowed(r#"a\"b"#) ];
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
  let expected = vec![ Cow::Borrowed("a\\\\b") ];
  assert_eq!( splits, expected );
}


#[test]
fn test_mre_arg2_isolated()
{
  // Part of the original MRE: "arg2 \" "
  let src = r#""arg2 \" ""#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ Cow::Borrowed(r#"arg2 " "#) ];
  assert_eq!( splits, expected );
}

#[test]
fn test_mre_arg3_isolated()
{
  // Part of the original MRE: "arg3 \\"
  let src = r#""arg3 \\""#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ Cow::Borrowed(r#"arg3 \"#) ];
  assert_eq!( splits, expected );
}

#[test]
fn test_consecutive_escaped_backslashes_and_quote()
{
  // Tests `\\\\\"` -> `\\"`
  let src = r#""a\\\\\"b""#;
  let splits : Vec<_> = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .perform()
  .map( | e | e.string ).collect();
  let expected = vec![ Cow::Borrowed(r#"a\\"b"#) ];
  assert_eq!( splits, expected );
}

#[test]
fn mre_from_task_test()
{
  let input = r#"cmd key::"value with \"quotes\" and \\slash\\""#;
  let splits_iter = strs_tools::string::split()
      .src( input )
      .delimeter( vec![ " ", "::" ] )
      .preserving_delimeters( true )
      .quoting( true )
      .form()
      .split();

  let splits: Vec<strs_tools::string::split::Split<'_>> = splits_iter.collect();

  use strs_tools::string::split::Split;
  use strs_tools::string::split::SplitType::{ Delimiter, Delimeted };

  let expected = vec!
  [
    Split { string: Cow::Borrowed("cmd"), typ: Delimeted, start: 0, end: 3 },
    Split { string: Cow::Borrowed(" "), typ: Delimiter, start: 3, end: 4 },
    Split { string: Cow::Borrowed("key"), typ: Delimeted, start: 4, end: 7 },
    Split { string: Cow::Borrowed("::"), typ: Delimiter, start: 7, end: 9 },
    // This is the crucial part. The current implementation will likely fail here.
    // Expected unescaped string: "value with \"quotes\" and \\slash\\"
    // But the current implementation will probably return the raw string with escapes.
    Split { string: Cow::Borrowed("value with \"quotes\" and \\slash\\"), typ: Delimeted, start: 9, end: 45 },
  ];

  assert_eq!( splits, expected );
}
