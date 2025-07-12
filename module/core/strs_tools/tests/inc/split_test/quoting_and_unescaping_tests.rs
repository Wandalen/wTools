//! Tests focusing on `quoting` and unescaping behavior.
use strs_tools::string::split::*;

// Test case from the original issue description
#[test]
fn test_mre_unescaping()
{
  let src = r#"sub .asset.get path:"a b""#;
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_quoting( false )
  .perform();
  let splits : Vec<_> = iter.map( | e | e.string ).collect();
  assert_eq!( splits, vec![ "sub", ".asset.get", "path:a b" ] );
}

#[test]
fn test_no_quotes()
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .perform();
  let splits : Vec<_> = iter.map( | e | e.string ).collect();
  assert_eq!( splits, vec![ "a", "b", "c" ] );
}

#[test]
fn test_empty_quoted_sections()
{
  let src = r#"a "" b"#;
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_quoting( false )
  .perform();
  let splits : Vec<_> = iter.map( | e | e.string ).collect();
  assert_eq!( splits, vec![ "a", "", "b" ] );
}

#[test]
fn test_multiple_escape_sequences()
{
  let src = r#""\n\t\\\"""#;
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_quoting( false )
  .perform();
  let splits : Vec<_> = iter.map( | e | e.string ).collect();
  assert_eq!( splits, vec![ "\n\t\\\"" ] );
}

#[test]
fn test_quoted_at_start()
{
  let src = r#""a b" c"#;
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_quoting( false )
  .perform();
  let splits : Vec<_> = iter.map( | e | e.string ).collect();
  assert_eq!( splits, vec![ "a b", "c" ] );
}

#[test]
fn test_quoted_at_end()
{
  let src = r#"a "b c""#;
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_quoting( false )
  .perform();
  let splits : Vec<_> = iter.map( | e | e.string ).collect();
  assert_eq!( splits, vec![ "a", "b c" ] );
}

#[test]
fn test_unterminated_quote()
{
  let src = r#"a "b c"#;
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_quoting( false )
  .perform();
  let splits : Vec<_> = iter.map( | e | e.string ).collect();
  assert_eq!( splits, vec![ "a", "b c" ] );
}