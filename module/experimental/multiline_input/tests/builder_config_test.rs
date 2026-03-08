//! Builder configuration tests
//!
//! ## Domain
//!
//! Tests for Builder pattern and editor configuration:
//! - Default configuration values
//! - Builder method chaining
//! - Configuration propagation to Editor
//!
//! ## Organization
//!
//! Tests migrated from `src/builder.rs` module tests.

use multiline_input::Builder;

#[ test ]
fn test_builder_default()
{
  let builder = Builder::new();
  assert_eq!( builder.prompt, "" );
  assert!( builder.allow_empty );
  assert!( builder.color );
}

#[ test ]
fn test_builder_configuration()
{
  let builder = Builder::new()
    .prompt( "Enter text:" )
    .min_length( 5 )
    .max_length( 100 )
    .show_line_numbers( true )
    .show_status( true )
    .color( false );

  assert_eq!( builder.prompt, "Enter text:" );
  assert_eq!( builder.min_length, Some( 5 ) );
  assert_eq!( builder.max_length, Some( 100 ) );
  assert!( builder.show_line_numbers );
  assert!( builder.show_status );
  assert!( !builder.color );
}

#[ test ]
fn test_builder_builds_editor()
{
  let builder = Builder::new().prompt( "Test:" );
  let _editor = builder.build();
  // Just verify it doesnt panic
}
