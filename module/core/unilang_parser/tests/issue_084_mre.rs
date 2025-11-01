//! Minimal Reproducible Example for Issue 084: Tokenizer doesn't handle escaped quotes
//!
//! This test demonstrates the bug where values containing both whitespace AND inner quotes
//! fail to parse because the tokenizer doesn't understand backslash-escaped quotes.

use unilang_parser::{ Parser, UnilangParserOptions };

/// MRE 1: Basic case - command with quoted path argument
///
/// Real-world command: w3 .crates.for.each 'cmd::cld -p "/start"'
/// Shell passes: ["w3", ".crates.for.each", "cmd::cld -p \"/start\""]
///
/// EXPECTED: Parses successfully, value = `cld -p "/start"`
/// ACTUAL: Parse error "Unexpected token '/start\"\"'"
#[test]
fn mre_double_quotes_with_path()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Shell removes outer single quotes, passes literal double quotes
  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    r#"cmd::cld -p "/start""#.to_string(),
  ]);

  // This SHOULD pass but currently FAILS
  match &result
  {
    Ok( instruction ) =>
    {
      let cmd_values = instruction.named_arguments.get( "cmd" );
      assert!( cmd_values.is_some(), "cmd parameter should exist" );
      assert_eq!(
        cmd_values.unwrap()[ 0 ].value,
        r#"cld -p "/start""#,
        "Inner quotes should be preserved"
      );
    }
    Err( e ) =>
    {
      panic!( "Parse failed with error: {:?}", e );
    }
  }
}

/// MRE 2: Command with quoted multi-word argument
///
/// Real-world command: w3 .crates.for.each 'cmd::cld -p "/start explore"'
/// Shell passes: ["w3", ".crates.for.each", "cmd::cld -p \"/start explore\""]
///
/// EXPECTED: Parses successfully, value = `cld -p "/start explore"`
/// ACTUAL: Parse error "Unexpected token '/start explore\"\"'"
#[test]
fn mre_double_quotes_with_whitespace()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    r#"cmd::cld -p "/start explore""#.to_string(),
  ]);

  match &result
  {
    Ok( instruction ) =>
    {
      let cmd_values = instruction.named_arguments.get( "cmd" );
      assert!( cmd_values.is_some(), "cmd parameter should exist" );
      assert_eq!(
        cmd_values.unwrap()[ 0 ].value,
        r#"cld -p "/start explore""#,
        "Inner quotes with whitespace should be preserved"
      );
    }
    Err( e ) =>
    {
      panic!( "Parse failed with error: {:?}", e );
    }
  }
}

/// MRE 3: Simple echo command with quoted string
///
/// Real-world command: w3 .crates.for.each 'cmd::echo "hello world"'
/// Shell passes: ["w3", ".crates.for.each", "cmd::echo \"hello world\""]
///
/// EXPECTED: Parses successfully, value = `echo "hello world"`
/// ACTUAL: Parse error "Unexpected token 'hello world\"\"'"
#[test]
fn mre_double_quotes_simple_echo()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    r#"cmd::echo "hello world""#.to_string(),
  ]);

  match &result
  {
    Ok( instruction ) =>
    {
      let cmd_values = instruction.named_arguments.get( "cmd" );
      assert!( cmd_values.is_some(), "cmd parameter should exist" );
      assert_eq!(
        cmd_values.unwrap()[ 0 ].value,
        r#"echo "hello world""#,
        "Quoted string should be preserved"
      );
    }
    Err( e ) =>
    {
      panic!( "Parse failed with error: {:?}", e );
    }
  }
}

/// MRE 4: Direct parsing (bypassing argv) to show reconstruction issue
///
/// This demonstrates that even with direct string parsing, the escaped quotes
/// cause issues when the reconstructor adds them.
#[test]
fn mre_direct_parse_with_escaped_quotes()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulating what argv reconstructor creates: cmd::"value with \"inner\" quotes"
  let result = parser.parse_single_instruction( r#"cmd::"value with \"inner\" quotes""# );

  match &result
  {
    Ok( instruction ) =>
    {
      let cmd_values = instruction.named_arguments.get( "cmd" );
      assert!( cmd_values.is_some(), "cmd parameter should exist" );
      assert_eq!(
        cmd_values.unwrap()[ 0 ].value,
        r#"value with "inner" quotes"#,
        "Escaped quotes should be unescaped in final value"
      );
    }
    Err( e ) =>
    {
      panic!( "Parse failed with error: {:?}", e );
    }
  }
}

/// Control test: Verify that single quotes work (they do, but may have bash issues)
///
/// This shows a workaround that currently works for parsing.
#[test]
fn control_single_quotes_work()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Using single quotes inside the value
  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    "cmd::cld -p '/start explore'".to_string(),
  ]);

  assert!( result.is_ok(), "Single quotes should parse successfully" );

  let instruction = result.unwrap();
  let cmd_values = instruction.named_arguments.get( "cmd" );
  assert!( cmd_values.is_some(), "cmd parameter should exist" );
  assert_eq!(
    cmd_values.unwrap()[ 0 ].value,
    "cld -p '/start explore'",
    "Single quotes are preserved as-is"
  );
}

/// Control test: Verify that no quotes with whitespace works
///
/// This shows another workaround.
#[test]
fn control_no_quotes_works()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    "cmd::cld -p /start explore".to_string(),
  ]);

  assert!( result.is_ok(), "Value without quotes should parse successfully" );

  let instruction = result.unwrap();
  let cmd_values = instruction.named_arguments.get( "cmd" );
  assert!( cmd_values.is_some(), "cmd parameter should exist" );
  assert_eq!(
    cmd_values.unwrap()[ 0 ].value,
    "cld -p /start explore",
    "Whitespace is preserved without quotes"
  );
}

/// Control test: Verify escaped space workaround
#[test]
fn control_escaped_space_works()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    r"cmd::cld -p /start\ explore".to_string(),
  ]);

  assert!( result.is_ok(), "Escaped space should parse successfully" );

  let instruction = result.unwrap();
  let cmd_values = instruction.named_arguments.get( "cmd" );
  assert!( cmd_values.is_some(), "cmd parameter should exist" );
  assert_eq!(
    cmd_values.unwrap()[ 0 ].value,
    r"cld -p /start\ explore",
    "Backslash escape is preserved"
  );
}
