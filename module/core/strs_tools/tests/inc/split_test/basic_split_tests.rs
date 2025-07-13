//! Tests for default behavior, simple delimiters, and no complex options.
use strs_tools::string::split::*;

// Test Matrix ID: Basic_Default_NoDelim_SimpleSrc
// Tests the default behavior of split when no delimiters are specified.
#[test]
fn test_scenario_default_char_split()
{
  let src = "abc";
  let iter = split()
  .src( src )
  // No delimiter specified, preserving_delimeters default (true) has no effect.
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "abc" ] );
}

// Test Matrix ID: Basic_Default_FormMethods_SimpleSrc
// Tests the default behavior using .form() and .split_fast() methods.
#[test]
fn test_scenario_default_char_split_form_methods()
{
  let src = "abc";
  let opts = split()
  .src( src )
  .form();
  let iter = opts.split();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "abc" ] );

  let src = "abc";
  let opts = split()
  .src( src )
  .form();
  let iter = opts.split_fast();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "abc" ] );
}

// Test Matrix ID: Basic_MultiDelim_InclEmpty_Defaults
// Effective delimiters ["a", "b"]. New default preserving_delimeters=true.
// PE=F (default).
// "abc" -> SFI: ""(D), "a"(L), ""(D), "b"(L), "c"(D)
// SI yields: "a", "b", "c"
#[test]
fn test_scenario_multi_delimiters_incl_empty_char_split()
{
  let src = "abc";
  let iter = split()
  .src( src )
  .delimeter( vec![ "a", "b", "" ] )
  // preserving_delimeters defaults to true
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

// Test Matrix ID: Basic_MultiDelim_SomeMatch_Defaults
// Tests splitting with multiple delimiters where some match and some don't.
// Delimiters ["b", "d"]. New default preserving_delimeters=true.
// PE=F (default).
// "abc" -> SFI: "a"(D), "b"(L), "c"(D)
// SI yields: "a", "b", "c"
#[test]
fn test_basic_multi_delimiters_some_match()
{
  let src = "abc";
  let iter = split()
  .src( src )
  .delimeter( vec![ "b", "d" ] )
  // preserving_delimeters defaults to true
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

// Test Matrix ID: N/A
// Tests that escaped characters within a quoted string are correctly unescaped.
#[test]
fn unescaping_in_quoted_string()
{
  // Test case 1: Escaped quote
  let src = r#""hello \" world""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"hello " world"# ] );

  // Test case 2: Escaped backslash
  let src = r#""path\\to\\file""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"path\to\file"# ] );
}

#[test]
fn unescaping_only_escaped_quote()
{
  let src = r#""\"""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"""# ] );
}

#[test]
fn unescaping_only_escaped_backslash()
{
  let src = r#""\\""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"\"# ] );
}

#[test]
fn unescaping_consecutive_escaped_backslashes()
{
  let src = r#""\\\\""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"\\"# ] );
}

#[test]
fn unescaping_mixed_escaped_and_normal()
{
  let src = r#""a\\b\"c""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"a\b"c"# ] );
}

#[test]
fn unescaping_at_start_and_end()
{
  let src = r#""\\a\"""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"\a""# ] );
}

#[test]
fn unescaping_with_delimiters_outside()
{
  let src = r#"a "b\"c" d"#;
  let iter = split()
  .src( src )
  .quoting( true )
  .delimeter( " " )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ "a", " ", r#"b"c"#, " ", "d" ] );
}

#[test]
fn unescaping_with_delimiters_inside_and_outside()
{
  let src = r#"a "b c\"d" e"#;
  let iter = split()
  .src( src )
  .quoting( true )
  .delimeter( " " )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ "a", " ", r#"b c"d"#, " ", "e" ] );
}

#[test]
fn unescaping_empty_string()
{
  let src = r#""""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ "" ] );
}

#[test]
fn unescaping_unterminated_quote()
{
  let src = r#""abc\""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"abc""# ] );
}

#[test]
fn unescaping_unterminated_quote_with_escape()
{
  let src = r#""abc\\""#;
  let iter = split()
  .src( src )
  .quoting( true )
  .preserving_empty( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | String::from( e.string ) ).collect();
  assert_eq!( splits, vec![ r#"abc\"# ] );
}