//! Minimal Reproducible Example for ISSUE-001: Escaped Quotes in Quoted Strings
//!
//! This test reproduces the exact scenario described in:
//! `task/issue_001_escaped_quotes_in_quoted_strings.md`

use strs_tools::string::split;

#[ test ]
fn mre_exact_from_issue_001()
{
  // Test case: String with escaped quotes inside quoted section
  let input = r#"key::"value with \"inner\" quotes""#;
  //              ^1   ^2           ^3               ^4
  // Expected tokenization:
  //   Token 1: "key"
  //   Token 2: "::"
  //   Token 3: "value with \"inner\" quotes" (unescaped to: value with "inner" quotes)

  let result : Vec< _ > = split::split()
    .src( input )
    .delimeters( &[ "::", " ", "\t" ] )
    .quoting( true )
    .preserving_quoting( false ) // Remove quotes, unescape content
    .perform()
    .collect();

  println!( "Input: {input:?}" );
  println!( "Result tokens ({}): {result:#?}", result.len() );

  // EXPECTED: Should parse successfully with 3 tokens
  assert_eq!( result.len(), 3, "Expected 3 tokens" );
  assert_eq!( result[ 0 ].string, "key" );
  assert_eq!( result[ 1 ].string, "::" );
  assert_eq!( result[ 2 ].string, r#"value with "inner" quotes"# );
}

#[ test ]
fn mre_simpler_case()
{
  // Simplified case: just escaped quote inside quoted string
  let input = r#""value with \"inner\" quotes""#;

  let result : Vec< _ > = split::split()
    .src( input )
    .delimeter( " " )
    .quoting( true )
    .preserving_quoting( false ) // Remove quotes, unescape content
    .perform()
    .collect();

  println!( "Simpler case input: {input:?}" );
  println!( "Result tokens ({}): {result:#?}", result.len() );

  // Should get single token with unescaped quotes
  assert_eq!( result.len(), 1 );
  assert_eq!( result[ 0 ].string, r#"value with "inner" quotes"# );
}

#[ test ]
fn mre_without_unescaping()
{
  // Same test but preserve the escape sequences
  let input = r#"key::"value with \"inner\" quotes""#;

  let result : Vec< _ > = split::split()
    .src( input )
    .delimeters( &[ "::", " ", "\t" ] )
    .quoting( true )
    .preserving_quoting( true ) // Keep quotes in output
    .perform()
    .collect();

  println!( "Without unescaping input: {input:?}" );
  println!( "Result tokens ({}): {result:#?}", result.len() );

  // Should still correctly identify token boundaries even if not unescaping
  assert_eq!( result.len(), 3, "Expected 3 tokens even without unescaping" );
}

#[ test ]
fn mre_unilang_exact_input()
{
  // EXACT input from unilang_parser failing test
  // This is what unilang_parser.parse_single_instruction() receives
  let input = r#"cmd::"value with \"inner\" quotes""#;

  println!( "=== UNILANG EXACT INPUT TEST ===" );
  println!( "Input: {input:?}" );

  let result : Vec< _ > = split::split()
    .src( input )
    .delimeters( &[ "::", " ", "\t" ] )
    .quoting( true )
    .preserving_quoting( false ) // Unescape content
    .perform()
    .collect();

  println!( "Result tokens ({}): {result:#?}", result.len() );

  // Verify correct parsing
  assert_eq!( result.len(), 3, "Expected 3 tokens" );
  assert_eq!( result[ 0 ].string, "cmd" );
  assert_eq!( result[ 1 ].string, "::" );
  assert_eq!( result[ 2 ].string, r#"value with "inner" quotes"#, "Should unescape to actual quotes" );

  println!( "✅ PASS: strs_tools correctly handles the input that fails in unilang_parser" );
}

#[ test ]
fn mre_exact_unilang_parser_call()
{
  // EXACT replication of how unilang_parser calls strs_tools
  // From parser_engine.rs:48-62
  let input = r#"cmd::"value with \"inner\" quotes""#;

  println!( "=== EXACT UNILANG PARSER CALL REPLICATION ===" );
  println!( "Input: {input:?}" );

  // Replicate exact call pattern
  let all_delimiters = vec![ " ", "\n", "\t", "\r", "#", "::" ];

  let splits_iter = split::split()
    .delimeters( all_delimiters.as_slice() )
    .quoting( true )
    .preserving_empty( false )
    .src( input )
    .perform();

  let result: Vec<_> = splits_iter.collect();

  println!( "Result tokens ({}): {result:#?}", result.len() );

  // Analyze results
  for (i, token) in result.iter().enumerate() {
    println!( "  Token {i}: '{:?}' (type: {:?}, was_quoted: {})",
             token.string, token.typ, token.was_quoted );
  }

  // Verify correct parsing
  assert!( result.len() >= 3, "Expected at least 3 tokens, got {}", result.len() );

  // Find the cmd token
  let cmd_token = result.iter().find( | t | t.string == "cmd" );
  assert!( cmd_token.is_some(), "Should find 'cmd' token" );

  // Find the :: token
  let op_token = result.iter().find( | t | t.string == "::" );
  assert!( op_token.is_some(), "Should find '::' delimiter token" );

  // Find the quoted value token
  let value_token = result.iter().find( | t | t.was_quoted );
  assert!( value_token.is_some(), "Should find quoted token" );

  if let Some( val ) = value_token {
    let val_string = &val.string;
    println!( "✅ Quoted value token: {val_string:?}" );
    assert_eq!( val.string, r#"value with "inner" quotes"#,
               "Quoted value should be unescaped" );
  }

  println!( "✅ PASS: Exact unilang_parser call pattern works correctly" );
}
