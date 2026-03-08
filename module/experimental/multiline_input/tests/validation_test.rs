//! Input validation tests
//!
//! ## Domain
//!
//! Tests for editor input validation logic:
//! - Empty input validation
//! - Minimum length validation
//! - Maximum length validation
//! - Initial text handling
//!
//! ## Organization
//!
//! Tests migrated from `src/editor.rs` module tests.

use multiline_input::Builder;

#[ test ]
fn test_editor_creation()
{
  let editor = Builder::new()
    .prompt( "Test:" )
    .build();

  // Verify validation works
  assert!( editor.validate( "hello" ).is_ok() );
}

#[ test ]
fn test_validation_empty()
{
  let editor = Builder::new()
    .allow_empty( false )
    .build();

  assert!( editor.validate( "" ).is_err() );
  assert!( editor.validate( "text" ).is_ok() );
}

#[ test ]
fn test_validation_min_length()
{
  let editor = Builder::new()
    .min_length( 5 )
    .build();

  assert!( editor.validate( "hi" ).is_err() );
  assert!( editor.validate( "hello" ).is_ok() );
}

#[ test ]
fn test_validation_max_length()
{
  let editor = Builder::new()
    .max_length( 5 )
    .build();

  assert!( editor.validate( "hi" ).is_ok() );
  assert!( editor.validate( "hello" ).is_ok() );
  assert!( editor.validate( "hello world" ).is_err() );
}

#[ test ]
fn test_initial_text()
{
  let editor = Builder::new()
    .initial_text( "Initial content" )
    .build();

  // Just verify it doesnt panic
  assert!( editor.initial_text.is_some() );
}
