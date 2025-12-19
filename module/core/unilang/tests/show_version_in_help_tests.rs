//!
//! # Show Version In Help Tests
//!
//! ## What This Tests
//!
//! This test suite validates the `show_version_in_help` field functionality:
//! - Per-command control of version display in help output
//! - Conversion preservation from `StaticCommandDefinition` to `CommandDefinition`
//! - Default behavior (version shown when true)
//! - Hide behavior (version hidden when false)
//!
//! ## Why This Matters
//!
//! Users may want to hide default version "1.0.0" from help output when they haven't
//! explicitly set a version. The `show_version_in_help` field provides opt-out capability.
//!
//! ## Related
//!
//! - `HelpDisplayOptions` for global version hiding
//! - `src/help.rs` for help formatting implementation

use unilang::static_data::StaticCommandDefinition;
use unilang::data::CommandDefinition;

//
// Test: show_version_in_help defaults to true
//

/// Verifies that `show_version_in_help` defaults to true in `CommandDefinition`.
#[ test ]
fn command_definition_show_version_defaults_to_true()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command".to_string() )
    .end();

  assert!( cmd.show_version_in_help(), "show_version_in_help should default to true" );
}

//
// Test: show_version_in_help can be set to false
//

/// Verifies that `show_version_in_help` can be set to false via builder.
#[ test ]
fn command_definition_show_version_can_be_false()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command".to_string() )
    .end()
    .with_show_version_in_help( false );

  assert!( !cmd.show_version_in_help(), "show_version_in_help should be false when set" );
}

//
// Test: static to dynamic conversion preserves show_version_in_help true
//

/// Verifies that conversion preserves `show_version_in_help` = true.
#[ test ]
fn from_static_preserves_show_version_true()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "",
    show_version_in_help : true,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert!( dynamic_cmd.show_version_in_help(), "show_version_in_help=true should be preserved" );
}

//
// Test: static to dynamic conversion preserves show_version_in_help false
//

/// Verifies that conversion preserves `show_version_in_help` = false.
#[ test ]
fn from_static_preserves_show_version_false()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "",
    show_version_in_help : false,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert!( !dynamic_cmd.show_version_in_help(), "show_version_in_help=false should be preserved" );
}

//
// Test: StaticCommandDefinition::new defaults to true
//

/// Verifies that `StaticCommandDefinition::new()` defaults `show_version_in_help` to true.
#[ test ]
fn static_command_new_defaults_show_version_true()
{
  let static_cmd = StaticCommandDefinition::new( ".test", "", "Test command" );

  assert!( static_cmd.show_version_in_help, "new() should default show_version_in_help to true" );
}

//
// Test: StaticCommandDefinition builder method works
//

/// Verifies that `with_show_version_in_help` builder method works.
#[ test ]
fn static_command_with_show_version_in_help()
{
  let static_cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_show_version_in_help( false );

  assert!( !static_cmd.show_version_in_help, "with_show_version_in_help(false) should set to false" );
}

//
// Test: help output respects show_version_in_help = true
//

/// Verifies that help output includes version when `show_version_in_help` is true.
#[ test ]
fn help_output_includes_version_when_true()
{
  use unilang::registry::CommandRegistry;

  let cmd = CommandDefinition::former()
    .name( ".test_version_shown" )
    .description( "Test command".to_string() )
    .version( "2.5.0".to_string() )
    .end()
    .with_show_version_in_help( true );

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let help = registry.get_help_for_command( ".test_version_shown" );
  assert!( help.is_some(), "Help should be generated" );

  let help_text = help.unwrap();
  assert!( help_text.contains( "2.5.0" ), "Help should include version when show_version_in_help=true" );
}

//
// Test: help output respects show_version_in_help = false
//

/// Verifies that help output excludes version when `show_version_in_help` is false.
#[ test ]
fn help_output_excludes_version_when_false()
{
  use unilang::registry::CommandRegistry;

  let cmd = CommandDefinition::former()
    .name( ".test_version_hidden" )
    .description( "Test command".to_string() )
    .version( "3.0.0".to_string() )
    .end()
    .with_show_version_in_help( false );

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let help = registry.get_help_for_command( ".test_version_hidden" );
  assert!( help.is_some(), "Help should be generated" );

  let help_text = help.unwrap();
  assert!( !help_text.contains( "3.0.0" ), "Help should NOT include version when show_version_in_help=false" );
}
