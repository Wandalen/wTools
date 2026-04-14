//! Render configuration tests
//!
//! ## Domain
//!
//! Tests for rendering configuration:
//! - Default render config values
//! - Custom config creation
//!
//! ## Organization
//!
//! Tests migrated from `src/render.rs` module tests.

use multiline_input::render::RenderConfig;

#[ test ]
fn test_render_config_default()
{
  let config = RenderConfig::default();
  assert!( !config.show_line_numbers );
  assert!( !config.show_status );
  assert!( config.color );
}

#[ test ]
fn test_render_config_creation()
{
  let config = RenderConfig
  {
    show_line_numbers: true,
    show_status: true,
    show_char_count: true,
    color: false,
    prompt: "Test:".to_string(),
    placeholder: Some( "Enter text...".to_string() ),
  };

  assert!( config.show_line_numbers );
  assert!( config.show_status );
  assert!( !config.color );
  assert_eq!( config.prompt, "Test:" );
}
