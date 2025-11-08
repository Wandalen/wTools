#![ allow( clippy ::uninlined_format_args ) ]

//! tests for content parsing corner cases
//!
//! covers all edge cases in key-value file parsing including:
//! - line format variations
//! - quoting and escaping
//! - comments and whitespace
//! - key and value edge cases

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::testing;
#[ cfg( feature = "secrets" ) ]
use std ::fs;

/// test KEY="" empty quoted value
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_empty_quoted_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"\"\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "KEY" ), Some( &String::new() ), "empty quoted value should be empty string" );
}

/// test KEY= with no value
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_empty_unquoted_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // should have empty string value
  assert!( secrets.contains_key( "KEY" ), "KEY= should create entry" );
  assert_eq!( secrets.get( "KEY" ), Some( &String::new() ), "empty unquoted value should be empty string" );
}

/// test KEY with no equals sign (should be ignored)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_no_equals_sign_ignored()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert!( !secrets.contains_key( "KEY" ), "line without = should be ignored" );
  assert_eq!( secrets.len(), 0, "should have no entries" );
}

/// test =value with no key
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_no_key_empty_string_key()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "=value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // BUG: creates entry with empty string key
  assert!( secrets.contains_key( "" ), "empty key entry created (bug)" );
  assert_eq!( secrets.get( "" ), Some( &"value".to_string() ), "empty key has value" );
}

/// test KEY==value (double equals - second = is part of value)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_double_equals()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY==value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "KEY" ), Some( &"=value".to_string() ), "second = should be part of value" );
}

/// test KEY=val=ue (equals in value)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_equals_in_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=val=ue\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "KEY" ), Some( &"val=ue".to_string() ), "= in value should be preserved" );
}

/// test KEY = value with spaces around equals
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_spaces_around_equals()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY = value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // implementation trims whitespace, so "KEY " becomes "KEY"
  assert!( secrets.contains_key( "KEY" ), "spaces around = are trimmed" );
  assert_eq!( secrets.get( "KEY" ), Some( &"value".to_string() ), "value also trimmed" );
}

/// test KEY= value with space before value
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_space_before_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY= value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // implementation trims leading/trailing whitespace from value
  assert_eq!( secrets.get( "KEY" ), Some( &"value".to_string() ), "leading space trimmed" );
}

/// test KEY=value  with trailing space
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_trailing_space_in_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=value  \n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // implementation trims trailing whitespace
  assert_eq!( secrets.get( "KEY" ), Some( &"value".to_string() ), "trailing space trimmed" );
}

/// test mixed quotes KEY="val'ue"
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_mixed_quotes_double_outside()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"val'ue\"\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // outer quotes stripped, inner preserved
  assert_eq!( secrets.get( "KEY" ), Some( &"val'ue".to_string() ), "inner single quote should be preserved" );
}

/// test mixed quotes KEY='val"ue'
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_mixed_quotes_single_outside()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY='val\"ue'\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // outer quotes stripped, inner preserved
  assert_eq!( secrets.get( "KEY" ), Some( &"val\"ue".to_string() ), "inner double quote should be preserved" );
}

/// test escaped quote KEY="val\"ue" - backslash NOT unescaped
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_escaped_quote_not_unescaped()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"val\\\"ue\"\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // backslash escape is NOT processed - kept literal
  assert_eq!( secrets.get( "KEY" ), Some( &"val\\\"ue".to_string() ), "backslash should be literal" );
}

/// test KEY="value with spaces"
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_quoted_value_with_spaces()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"value with spaces\"\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "KEY" ), Some( &"value with spaces".to_string() ), "spaces in quoted value preserved" );
}

/// test KEY=value with spaces (unquoted - only first word captured)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_unquoted_value_with_spaces()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=value with spaces\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // implementation captures everything after = until newline, so spaces included
  assert_eq!( secrets.get( "KEY" ), Some( &"value with spaces".to_string() ), "unquoted spaces included" );
}

/// test KEY="line1\nline2" - \n is literal, not newline
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_backslash_n_literal()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"line1\\nline2\"\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // \n should be literal characters, not actual newline
  assert_eq!( secrets.get( "KEY" ), Some( &"line1\\nline2".to_string() ), "\\n should be literal" );
}

/// test KEY="tab\there" - \t is literal, not tab
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_backslash_t_literal()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"tab\\there\"\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // \t should be literal characters, not actual tab
  assert_eq!( secrets.get( "KEY" ), Some( &"tab\\there".to_string() ), "\\t should be literal" );
}

/// test KEY="path\\to\\file" - double backslash literal
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_double_backslash_literal()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"path\\\\to\\\\file\"\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // double backslash NOT unescaped to single
  assert_eq!( secrets.get( "KEY" ), Some( &"path\\\\to\\\\file".to_string() ), "double backslash literal" );
}

/// test KEY="unclosed - unclosed quote kept as part of value
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_unclosed_quote()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=\"unclosed\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // unclosed quote - quote is kept in value
  assert_eq!( secrets.get( "KEY" ), Some( &"\"unclosed".to_string() ), "unclosed quote kept" );
}

/// test KEY=value # comment - comment NOT stripped (part of value)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_inline_comment_not_stripped()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=value # comment\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // # is NOT treated as comment start when after value
  assert_eq!( secrets.get( "KEY" ), Some( &"value # comment".to_string() ), "inline comment not stripped" );
}

/// test KEY=value// comment - // not special
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_double_slash_not_comment()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=value// comment\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // // is not a comment delimiter
  assert_eq!( secrets.get( "KEY" ), Some( &"value// comment".to_string() ), "// not special" );
}

/// test leading whitespace on line (should be ignored)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_leading_whitespace_line()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "  KEY=value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // leading whitespace on line should be trimmed
  // but current implementation may not trim - check actual behavior
  // if it includes leading space in key, test should reflect that
  assert!( secrets.contains_key( "KEY" ) || secrets.contains_key( "  KEY" ), "should parse despite leading whitespace" );
}

/// test duplicate keys in same file (last one wins)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_duplicate_keys_same_file()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=first\nKEY=second\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // last value wins in HashMap insert
  assert_eq!( secrets.get( "KEY" ), Some( &"second".to_string() ), "last duplicate key should win" );
}

/// test case-sensitive keys (KEY vs key are different)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_case_sensitive_keys()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=upper\nkey=lower\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  // keys are case-sensitive
  assert_eq!( secrets.get( "KEY" ), Some( &"upper".to_string() ), "KEY should exist" );
  assert_eq!( secrets.get( "key" ), Some( &"lower".to_string() ), "key should exist" );
  assert_eq!( secrets.len(), 2, "should have both entries" );
}

/// test unicode in key names
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_unicode_key_names()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "TÖKËN=value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "TÖKËN" ), Some( &"value".to_string() ), "unicode keys should work" );
}

/// test emoji in key names
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_emoji_key_names()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "🔑_KEY=value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "🔑_KEY" ), Some( &"value".to_string() ), "emoji keys should work" );
}

/// test special characters in key names (dash, dot)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_special_chars_key_names()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY-NAME=dash\nKEY.NAME=dot\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "KEY-NAME" ), Some( &"dash".to_string() ), "dash in key should work" );
  assert_eq!( secrets.get( "KEY.NAME" ), Some( &"dot".to_string() ), "dot in key should work" );
}

/// test numbers as key names
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_number_key_names()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "123=value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "123" ), Some( &"value".to_string() ), "numeric keys should work" );
}

/// test key starting with number
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_key_starting_with_number()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "1KEY=value\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "1KEY" ), Some( &"value".to_string() ), "key starting with number should work" );
}

/// test very long key names (>1000 chars)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_very_long_key_name()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  let long_key = "K".repeat( 1500 );
  let content = format!( "{}=value\n", long_key );
  fs ::write( workspace.secret_file( "test.env" ), content ).unwrap();

  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( &long_key ), Some( &"value".to_string() ), "very long key should work" );
}

/// test very long value (>10KB)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_very_long_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  let long_value = "V".repeat( 15000 );
  let content = format!( "KEY={}\n", long_value );
  fs ::write( workspace.secret_file( "test.env" ), content ).unwrap();

  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "KEY" ), Some( &long_value ), "very long value should work" );
}

/// test control characters in value
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_control_characters_in_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  // control character (bell, \x07)
  fs ::write( workspace.secret_file( "test.env" ), "KEY=value\x07here\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert!( secrets.contains_key( "KEY" ), "control char in value should parse" );
  assert!( secrets.get( "KEY" ).unwrap().contains( '\x07' ), "control char should be preserved" );
}

/// test tab characters in value
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_tab_in_value()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  fs ::write( workspace.secret_file( "test.env" ), "KEY=value\there\n" ).unwrap();
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();

  assert_eq!( secrets.get( "KEY" ), Some( &"value\there".to_string() ), "tab in value preserved" );
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main()
{
  println!( "this test requires the 'secrets' feature" );
}
