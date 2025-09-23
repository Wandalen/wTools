//! Enhanced Secret Parsing Tests
//!
//! These tests verify the enhanced secret file parsing functionality that supports
//! multiple formats including export statements, dotenv format, and mixed formats.

#![ cfg( feature = "testing" ) ]

use workspace_tools ::testing ::create_test_workspace_with_structure;
use std ::fs;

/// Test parsing export statements in secret files
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_export_statement_parsing()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_content = r#"
# Example secret file with export statements
export API_KEY="sk-1234567890abcdef"
export DATABASE_URL="postgresql: //user: pass@localhost/db"
export DEBUG=true
export TOKEN='bearer-token-here'
"#;
  
  let secret_file = workspace.secret_file( "-test-exports.sh" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "-test-exports.sh" ).unwrap();
  
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "sk-1234567890abcdef" );
  assert_eq!( secrets.get( "DATABASE_URL" ).unwrap(), "postgresql: //user: pass@localhost/db" );
  assert_eq!( secrets.get( "DEBUG" ).unwrap(), "true" );
  assert_eq!( secrets.get( "TOKEN" ).unwrap(), "bearer-token-here" );
}

/// Test parsing mixed format secret files (export + standard)
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_mixed_format_parsing()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_content = r#"
# Mixed format secret file
API_KEY=standard-format-key
export DATABASE_URL="postgresql: //localhost/db"
REDIS_URL=redis: //localhost: 6379
export SMTP_HOST="smtp.example.com"
SMTP_PORT=587
"#;
  
  let secret_file = workspace.secret_file( "-mixed-format.sh" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "-mixed-format.sh" ).unwrap();
  
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "standard-format-key" );
  assert_eq!( secrets.get( "DATABASE_URL" ).unwrap(), "postgresql: //localhost/db" );
  assert_eq!( secrets.get( "REDIS_URL" ).unwrap(), "redis: //localhost: 6379" );
  assert_eq!( secrets.get( "SMTP_HOST" ).unwrap(), "smtp.example.com" );
  assert_eq!( secrets.get( "SMTP_PORT" ).unwrap(), "587" );
}

/// Test that commented export statements are ignored
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_commented_exports_ignored()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_content = r#"
# Active secrets
export API_KEY="active-key"
API_SECRET=active-secret

# Commented out secrets should be ignored
# export OLD_API_KEY="old-key"
# DATABASE_URL=old-db-url
#export DISABLED_KEY="disabled"

# More active secrets
export REDIS_URL="redis: //localhost"
"#;
  
  let secret_file = workspace.secret_file( "-commented-test.sh" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "-commented-test.sh" ).unwrap();
  
  // Should have only the active secrets
  assert_eq!( secrets.len(), 3 );
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "active-key" );
  assert_eq!( secrets.get( "API_SECRET" ).unwrap(), "active-secret" );
  assert_eq!( secrets.get( "REDIS_URL" ).unwrap(), "redis: //localhost" );
  
  // Should not have commented secrets
  assert!( !secrets.contains_key( "OLD_API_KEY" ) );
  assert!( !secrets.contains_key( "DATABASE_URL" ) );
  assert!( !secrets.contains_key( "DISABLED_KEY" ) );
}

/// Test quote handling in export statements
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_export_quote_handling()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_content = r#"
export DOUBLE_QUOTED="value with spaces"
export SINGLE_QUOTED='another value with spaces'
export NO_QUOTES=simple_value
export EMPTY_DOUBLE=""
export EMPTY_SINGLE=''
export QUOTES_IN_VALUE="He said 'Hello World!'"
"#;
  
  let secret_file = workspace.secret_file( "-quotes-test.sh" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "-quotes-test.sh" ).unwrap();
  
  assert_eq!( secrets.get( "DOUBLE_QUOTED" ).unwrap(), "value with spaces" );
  assert_eq!( secrets.get( "SINGLE_QUOTED" ).unwrap(), "another value with spaces" );
  assert_eq!( secrets.get( "NO_QUOTES" ).unwrap(), "simple_value" );
  assert_eq!( secrets.get( "EMPTY_DOUBLE" ).unwrap(), "" );
  assert_eq!( secrets.get( "EMPTY_SINGLE" ).unwrap(), "" );
  assert_eq!( secrets.get( "QUOTES_IN_VALUE" ).unwrap(), "He said 'Hello World!'" );
}

/// Test backward compatibility with existing KEY=VALUE format
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_backward_compatibility()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // This is the original format that should continue to work
  let secret_content = r#"
API_KEY="sk-1234567890abcdef"
DATABASE_URL="postgresql: //user: pass@localhost/db"
DEBUG=true
TOKEN='bearer-token-here'
"#;
  
  let secret_file = workspace.secret_file( "-backward-compat.sh" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "-backward-compat.sh" ).unwrap();
  
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "sk-1234567890abcdef" );
  assert_eq!( secrets.get( "DATABASE_URL" ).unwrap(), "postgresql: //user: pass@localhost/db" );
  assert_eq!( secrets.get( "DEBUG" ).unwrap(), "true" );
  assert_eq!( secrets.get( "TOKEN" ).unwrap(), "bearer-token-here" );
}

/// Test edge cases and malformed lines are handled gracefully
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_malformed_lines_handling()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_content = r"
# Valid secrets
API_KEY=valid-key

# Malformed lines (should be ignored gracefully)
export  
export =
= 
just-text-no-equals
export KEY_WITH_NO_VALUE=
export SPACED_KEY = spaced-value

# More valid secrets
DATABASE_URL=valid-url
";
  
  let secret_file = workspace.secret_file( "-malformed-test.sh" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "-malformed-test.sh" ).unwrap();
  
  // Should parse valid entries
  assert_eq!( secrets.get( "API_KEY" ).unwrap(), "valid-key" );
  assert_eq!( secrets.get( "DATABASE_URL" ).unwrap(), "valid-url" );
  assert_eq!( secrets.get( "KEY_WITH_NO_VALUE" ).unwrap(), "" );
  assert_eq!( secrets.get( "SPACED_KEY" ).unwrap(), "spaced-value" );
  
  // Should handle malformed lines gracefully without crashing
  assert!( secrets.len() >= 4 );
}

/// Test integration with existing `load_secret_key` function
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_load_secret_key_with_exports()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let secret_content = r#"
export API_KEY="export-format-key"
DATABASE_URL=standard-format-url
"#;
  
  let secret_file = workspace.secret_file( "-integration-test.sh" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  // Test loading individual keys works with both formats
  let api_key = workspace.load_secret_key( "API_KEY", "-integration-test.sh" ).unwrap();
  let db_url = workspace.load_secret_key( "DATABASE_URL", "-integration-test.sh" ).unwrap();
  
  assert_eq!( api_key, "export-format-key" );
  assert_eq!( db_url, "standard-format-url" );
  
  // Test fallback to environment still works
  std ::env ::set_var( "TEST_ENV_VAR", "from-environment" );
  let env_var = workspace.load_secret_key( "TEST_ENV_VAR", "-integration-test.sh" ).unwrap();
  assert_eq!( env_var, "from-environment" );
  std ::env ::remove_var( "TEST_ENV_VAR" );
}