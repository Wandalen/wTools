//! Tests for Row 8: Rust DSL → Static Registry with const fn constructors.
//!
//! This module tests the const fn constructor API for creating static command
//! definitions at compile-time without macros.
//!
//! **Test Matrix for Const Fn Constructors**

#![ allow( clippy::assertions_on_constants ) ]
//!
//! | Test | Category | Validates | Status |
//! |------|----------|-----------|--------|
//! | CC1.1 | Command Creation | Basic const command creation | ✅ Tested |
//! | CC1.2 | Command Creation | Command with all fields | ✅ Tested |
//! | CC1.3 | Command Creation | Fluent builder API | ✅ Tested |
//! | CC2.1 | Argument Creation | Basic const argument creation | ✅ Tested |
//! | CC2.2 | Argument Creation | Argument with all fields | ✅ Tested |
//! | CC2.3 | Argument Creation | Fluent builder API | ✅ Tested |
//! | CC3.1 | Attributes Creation | Basic const attributes creation | ✅ Tested |
//! | CC3.2 | Attributes Creation | Attributes with all fields | ✅ Tested |
//! | CC3.3 | Attributes Creation | Fluent builder API | ✅ Tested |
//! | CC4.1 | Conversion | Static to dynamic command conversion | ✅ Tested |
//! | CC4.2 | Conversion | Static to dynamic argument conversion | ✅ Tested |
//! | CC4.3 | Conversion | Static to dynamic attributes conversion | ✅ Tested |
//! | CC5.1 | Compile-Time | Const evaluation works | ✅ Tested |
//! | CC5.2 | Compile-Time | Can be used in const context | ✅ Tested |

use unilang::static_data::
{
  StaticCommandDefinition,
  StaticArgumentDefinition,
  StaticArgumentAttributes,
  StaticKind,
  StaticValidationRule,
};
use unilang::data::CommandDefinition;

// ============================================================================
// CC1: Command Creation Tests
// ============================================================================

#[ test ]
fn test_cc1_1_basic_const_command_creation()
{
  // Test Matrix Row: CC1.1
  // Create basic command using const fn constructor

  const CMD : StaticCommandDefinition = StaticCommandDefinition::new(
    ".greet",
    "",
    "Greets the user",
  );

  assert_eq!( CMD.name, ".greet" );
  assert_eq!( CMD.namespace, "" );
  assert_eq!( CMD.description, "Greets the user" );
  assert_eq!( CMD.status, "stable" ); // Default
  assert_eq!( CMD.version, "1.0.0" ); // Default
  assert!( CMD.idempotent ); // Default true
  assert_eq!( CMD.http_method_hint, "GET" ); // Default
}

#[ test ]
fn test_cc1_2_command_with_all_fields()
{
  // Test Matrix Row: CC1.2
  // Create command with all fields explicitly set

  const TAGS : &[ &str ] = &[ "greeting", "user" ];
  const ALIASES : &[ &str ] = &[ "hello", "hi" ];
  const PERMISSIONS : &[ &str ] = &[ "public" ];
  const EXAMPLES : &[ &str ] = &[ ".greet name::Alice" ];

  const CMD : StaticCommandDefinition = StaticCommandDefinition::new(
    ".greet",
    ".social",
    "Greets the user",
  )
  .with_hint( "Say hello to someone" )
  .with_status( "stable" )
  .with_version( "2.0.0" )
  .with_tags( TAGS )
  .with_aliases( ALIASES )
  .with_permissions( PERMISSIONS )
  .with_idempotent( true )
  .with_http_method_hint( "POST" )
  .with_examples( EXAMPLES );

  assert_eq!( CMD.name, ".greet" );
  assert_eq!( CMD.namespace, ".social" );
  assert_eq!( CMD.description, "Greets the user" );
  assert_eq!( CMD.hint, "Say hello to someone" );
  assert_eq!( CMD.status, "stable" );
  assert_eq!( CMD.version, "2.0.0" );
  assert_eq!( CMD.tags.len(), 2 );
  assert_eq!( CMD.aliases.len(), 2 );
  assert_eq!( CMD.permissions.len(), 1 );
  assert!( CMD.idempotent );
  assert_eq!( CMD.http_method_hint, "POST" );
  assert_eq!( CMD.examples.len(), 1 );
}

#[ test ]
fn test_cc1_3_fluent_builder_api()
{
  // Test Matrix Row: CC1.3
  // Verify fluent builder pattern works

  const CMD : StaticCommandDefinition = StaticCommandDefinition::new(
    ".test",
    "",
    "Test command",
  )
  .with_hint( "Hint 1" )
  .with_status( "experimental" )
  .with_version( "0.1.0" )
  .with_idempotent( false );

  assert_eq!( CMD.hint, "Hint 1" );
  assert_eq!( CMD.status, "experimental" );
  assert_eq!( CMD.version, "0.1.0" );
  assert!( !CMD.idempotent );
}

// ============================================================================
// CC2: Argument Creation Tests
// ============================================================================

#[ test ]
fn test_cc2_1_basic_const_argument_creation()
{
  // Test Matrix Row: CC2.1
  // Create basic argument using const fn constructor

  const ARG : StaticArgumentDefinition = StaticArgumentDefinition::new(
    "name",
    StaticKind::String,
    "User name",
  );

  assert_eq!( ARG.name, "name" );
  assert!( matches!( ARG.kind, StaticKind::String ) );
  assert_eq!( ARG.description, "User name" );
  assert_eq!( ARG.hint, "" ); // Default
  assert!( !ARG.attributes.optional ); // Default
}

#[ test ]
fn test_cc2_2_argument_with_all_fields()
{
  // Test Matrix Row: CC2.2
  // Create argument with all fields explicitly set

  const ATTRS : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_optional( true )
    .with_default( "Guest" );

  const RULES : &[ StaticValidationRule ] = &[
    StaticValidationRule::MinLength( 1 ),
    StaticValidationRule::MaxLength( 50 ),
  ];

  const ALIASES : &[ &str ] = &[ "n", "username" ];
  const TAGS : &[ &str ] = &[ "required", "user" ];

  const ARG : StaticArgumentDefinition = StaticArgumentDefinition::new(
    "name",
    StaticKind::String,
    "User name",
  )
  .with_attributes( ATTRS )
  .with_hint( "Enter your name" )
  .with_validation_rules( RULES )
  .with_aliases( ALIASES )
  .with_tags( TAGS );

  assert_eq!( ARG.name, "name" );
  assert_eq!( ARG.hint, "Enter your name" );
  assert!( ARG.attributes.optional );
  assert_eq!( ARG.attributes.default, Some( "Guest" ) );
  assert_eq!( ARG.validation_rules.len(), 2 );
  assert_eq!( ARG.aliases.len(), 2 );
  assert_eq!( ARG.tags.len(), 2 );
}

#[ test ]
fn test_cc2_3_argument_fluent_builder_api()
{
  // Test Matrix Row: CC2.3
  // Verify fluent builder pattern works for arguments

  const ATTRS : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_optional( true )
    .with_multiple( true );

  const ARG : StaticArgumentDefinition = StaticArgumentDefinition::new(
    "files",
    StaticKind::List( &StaticKind::File, Some( ',' ) ),
    "List of files",
  )
  .with_attributes( ATTRS )
  .with_hint( "Comma-separated file paths" );

  assert_eq!( ARG.name, "files" );
  assert!( ARG.attributes.optional );
  assert!( ARG.attributes.multiple );
  assert_eq!( ARG.hint, "Comma-separated file paths" );
}

// ============================================================================
// CC3: Attributes Creation Tests
// ============================================================================

#[ test ]
fn test_cc3_1_basic_const_attributes_creation()
{
  // Test Matrix Row: CC3.1
  // Create basic attributes using const fn constructor

  const ATTRS : StaticArgumentAttributes = StaticArgumentAttributes::new();

  assert!( !ATTRS.optional ); // Default: required
  assert!( !ATTRS.multiple ); // Default: single value
  assert_eq!( ATTRS.default, None ); // Default: no default
  assert!( !ATTRS.sensitive ); // Default: not sensitive
  assert!( !ATTRS.interactive ); // Default: not interactive
}

#[ test ]
fn test_cc3_2_attributes_with_all_fields()
{
  // Test Matrix Row: CC3.2
  // Create attributes with all fields explicitly set

  const ATTRS : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_optional( true )
    .with_multiple( true )
    .with_default( "default_value" )
    .with_sensitive( true )
    .with_interactive( true );

  assert!( ATTRS.optional );
  assert!( ATTRS.multiple );
  assert_eq!( ATTRS.default, Some( "default_value" ) );
  assert!( ATTRS.sensitive );
  assert!( ATTRS.interactive );
}

#[ test ]
fn test_cc3_3_attributes_fluent_builder_api()
{
  // Test Matrix Row: CC3.3
  // Verify fluent builder pattern works for attributes

  const ATTRS1 : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_optional( true );

  const ATTRS2 : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_multiple( true )
    .with_default( "[]" );

  const ATTRS3 : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_sensitive( true )
    .with_interactive( true );

  assert!( ATTRS1.optional );
  assert!( !ATTRS1.multiple );

  assert!( !ATTRS2.optional );
  assert!( ATTRS2.multiple );
  assert_eq!( ATTRS2.default, Some( "[]" ) );

  assert!( ATTRS3.sensitive );
  assert!( ATTRS3.interactive );
}

// ============================================================================
// CC4: Conversion Tests
// ============================================================================

#[ test ]
fn test_cc4_1_static_to_dynamic_command_conversion()
{
  // Test Matrix Row: CC4.1
  // Convert static command to dynamic command

  const STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition::new(
    ".convert_test",
    ".test",
    "Test conversion",
  )
  .with_hint( "Conversion test hint" )
  .with_version( "3.0.0" );

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( &dynamic_cmd.name().to_string(), ".convert_test" );
  assert_eq!( &dynamic_cmd.namespace().to_string(), ".test" );
  assert_eq!( &dynamic_cmd.description().to_string(), "Test conversion" );
  assert_eq!( &dynamic_cmd.hint().to_string(), "Conversion test hint" );
  assert_eq!( &dynamic_cmd.version().to_string(), "3.0.0" );
  // Status string "stable" is converted to CommandStatus::Active (displays as "active")
  assert_eq!( &dynamic_cmd.status().to_string(), "active" );
}

#[ test ]
fn test_cc4_2_static_to_dynamic_argument_conversion()
{
  // Test Matrix Row: CC4.2
  // Convert static argument to dynamic argument

  const STATIC_ARG : StaticArgumentDefinition = StaticArgumentDefinition::new(
    "arg_test",
    StaticKind::Integer,
    "Test argument",
  )
  .with_hint( "Test hint" );

  let dynamic_arg : unilang::data::ArgumentDefinition = ( &STATIC_ARG ).into();

  assert_eq!( dynamic_arg.name, "arg_test" );
  assert_eq!( dynamic_arg.description, "Test argument" );
  assert_eq!( dynamic_arg.hint, "Test hint" );
  assert!( matches!( dynamic_arg.kind, unilang::data::Kind::Integer ) );
}

#[ test ]
fn test_cc4_3_static_to_dynamic_attributes_conversion()
{
  // Test Matrix Row: CC4.3
  // Convert static attributes to dynamic attributes

  const STATIC_ATTRS : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_optional( true )
    .with_default( "test_default" )
    .with_sensitive( true );

  let dynamic_attrs : unilang::data::ArgumentAttributes = ( &STATIC_ATTRS ).into();

  assert!( dynamic_attrs.optional );
  assert_eq!( dynamic_attrs.default, Some( "test_default".to_string() ) );
  assert!( dynamic_attrs.sensitive );
  assert!( !dynamic_attrs.multiple );
  assert!( !dynamic_attrs.interactive );
}

// ============================================================================
// CC5: Compile-Time Tests
// ============================================================================

#[ test ]
fn test_cc5_1_const_evaluation_works()
{
  // Test Matrix Row: CC5.1
  // Verify that constructors work in const context

  // This is a compile-time test - if it compiles, it works
  const _CMD1 : StaticCommandDefinition = StaticCommandDefinition::new(
    ".test1",
    "",
    "Test 1",
  );

  const _CMD2 : StaticCommandDefinition = StaticCommandDefinition::new(
    ".test2",
    ".namespace",
    "Test 2",
  ).with_version( "1.2.3" );

  const _ARG : StaticArgumentDefinition = StaticArgumentDefinition::new(
    "arg",
    StaticKind::Boolean,
    "Test arg",
  );

  const _ATTRS : StaticArgumentAttributes = StaticArgumentAttributes::new()
    .with_optional( true );

  // If this test compiles, const evaluation works
}

#[ test ]
fn test_cc5_2_can_be_used_in_const_context()
{
  // Test Matrix Row: CC5.2
  // Verify can be used in arrays and other const contexts

  const ARGS : &[ StaticArgumentDefinition ] = &[
    StaticArgumentDefinition::new( "arg1", StaticKind::String, "First arg" ),
    StaticArgumentDefinition::new( "arg2", StaticKind::Integer, "Second arg" ),
    StaticArgumentDefinition::new( "arg3", StaticKind::Boolean, "Third arg" ),
  ];

  const CMD : StaticCommandDefinition = StaticCommandDefinition::new(
    ".multi_arg",
    "",
    "Command with multiple arguments",
  )
  .with_arguments( ARGS );

  assert_eq!( CMD.arguments.len(), 3 );
  assert_eq!( CMD.arguments[ 0 ].name, "arg1" );
  assert_eq!( CMD.arguments[ 1 ].name, "arg2" );
  assert_eq!( CMD.arguments[ 2 ].name, "arg3" );
}
