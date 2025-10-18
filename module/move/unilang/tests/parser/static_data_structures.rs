//!
//! Tests for static data structures extension.
//!
//! This module tests the extended static command data structures including
//! `StaticCommandDefinition`, `StaticArgumentDefinition`, and PHF map compatibility.
//!

use unilang::prelude::*;
use unilang::data::ValidationRule;

#[ test ]
fn test_static_command_definition_creation()
{
  // Test basic creation of StaticCommandDefinition
  let static_cmd = StaticCommandDefinition
  {
    name: "test_command",
    namespace: "test",
    description: "A test command",
    hint: "test hint",
    arguments: &[],
    routine_link: Some("test_routine"),
    status: "stable",
    version: "1.0.0",
    tags: &["test", "example"],
    aliases: &["tc", "test"],
    permissions: &["read"],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &["test_command --help"],
  };

  assert_eq!( static_cmd.name, "test_command" );
  assert_eq!( static_cmd.namespace, "test" );
  assert_eq!( static_cmd.description, "A test command" );
  assert_eq!( static_cmd.hint, "test hint" );
  assert_eq!( static_cmd.routine_link, Some( "test_routine" ) );
  assert_eq!( static_cmd.status, "stable" );
  assert_eq!( static_cmd.version, "1.0.0" );
  assert_eq!( static_cmd.tags, &[ "test", "example" ] );
  assert_eq!( static_cmd.aliases, &[ "tc", "test" ] );
  assert_eq!( static_cmd.permissions, &[ "read" ] );
  assert!( static_cmd.idempotent );
  assert_eq!( static_cmd.deprecation_message, "" );
  assert_eq!( static_cmd.http_method_hint, "GET" );
  assert_eq!( static_cmd.examples, &[ "test_command --help" ] );
}

#[ test ]
fn test_static_argument_definition_creation()
{
  // Test creation of StaticArgumentDefinition with various kinds
  let attributes = StaticArgumentAttributes
  {
    optional: false,
    multiple: false,
    default: None,
    sensitive: false,
    interactive: false,
  };

  let validation_rules = &[
    StaticValidationRule::MinLength( 1 ),
    StaticValidationRule::MaxLength( 100 ),
  ];

  let static_arg = StaticArgumentDefinition
  {
    name: "input",
    kind: StaticKind::String,
    attributes,
    hint: "Input value",
    description: "The input value to process",
    validation_rules,
    aliases: &["i", "in"],
    tags: &["required"],
  };

  assert_eq!( static_arg.name, "input" );
  assert!( matches!( static_arg.kind, StaticKind::String ) );
  assert!( !static_arg.attributes.optional );
  assert_eq!( static_arg.hint, "Input value" );
  assert_eq!( static_arg.description, "The input value to process" );
  assert_eq!( static_arg.validation_rules.len(), 2 );
  assert_eq!( static_arg.aliases, &[ "i", "in" ] );
  assert_eq!( static_arg.tags, &[ "required" ] );
}

#[ test ]
fn test_static_kind_variants()
{
  // Test all StaticKind variants
  let kinds = [
    StaticKind::String,
    StaticKind::Integer,
    StaticKind::Float,
    StaticKind::Boolean,
    StaticKind::Path,
    StaticKind::File,
    StaticKind::Directory,
    StaticKind::Enum( &[ "option1", "option2" ] ),
    StaticKind::Url,
    StaticKind::DateTime,
    StaticKind::Pattern,
    StaticKind::List( &StaticKind::String, Some( ',' ) ),
    StaticKind::Map( &StaticKind::String, &StaticKind::Integer, Some( ',' ), Some( ':' ) ),
    StaticKind::JsonString,
    StaticKind::Object,
  ];

  for kind in &kinds
  {
    // Each kind should be debuggable and cloneable
    let _ = format!( "{kind:?}" );
    let _ = *kind;
  }
}

#[ test ]
fn test_static_validation_rules()
{
  // Test all StaticValidationRule variants
  let rules = [
    StaticValidationRule::Min( 0.0 ),
    StaticValidationRule::Max( 100.0 ),
    StaticValidationRule::MinLength( 1 ),
    StaticValidationRule::MaxLength( 255 ),
    StaticValidationRule::Pattern( "^[a-z]+$" ),
    StaticValidationRule::MinItems( 1 ),
  ];

  for rule in &rules
  {
    // Each rule should be debuggable and cloneable
    let _ = format!( "{rule:?}" );
    let _ = *rule;
  }
}

#[ test ]
fn test_conversion_static_to_dynamic_command()
{
  // Test conversion from StaticCommandDefinition to CommandDefinition
  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition
  {
    name: "test_cmd",
    namespace: "test",
    description: "Test description",
    hint: "Test hint",
    arguments: &[],
    routine_link: Some( "test_routine" ),
    status: "stable",
    version: "1.0.0",
    tags: &[ "test" ],
    aliases: &[ "tc" ],
    permissions: &[ "read" ],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[ "test_cmd --help" ],
  };

  let dynamic_cmd: CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.name, "test_cmd" );
  assert_eq!( dynamic_cmd.namespace, "test" );
  assert_eq!( dynamic_cmd.description, "Test description" );
  assert_eq!( dynamic_cmd.hint, "Test hint" );
  assert_eq!( dynamic_cmd.routine_link, Some( "test_routine".to_string() ) );
  assert_eq!( dynamic_cmd.status, "stable" );
  assert_eq!( dynamic_cmd.version, "1.0.0" );
  assert_eq!( dynamic_cmd.tags, vec![ "test" ] );
  assert_eq!( dynamic_cmd.aliases, vec![ "tc" ] );
  assert_eq!( dynamic_cmd.permissions, vec![ "read" ] );
  assert!( dynamic_cmd.idempotent );
  assert_eq!( dynamic_cmd.deprecation_message, "" );
  assert_eq!( dynamic_cmd.http_method_hint, "GET" );
  assert_eq!( dynamic_cmd.examples, vec![ "test_cmd --help" ] );
  assert!( !dynamic_cmd.auto_help_enabled ); // Static commands don't auto-generate help by default
}

#[ test ]
fn test_conversion_static_to_dynamic_argument()
{
  // Test conversion from StaticArgumentDefinition to ArgumentDefinition
  let attributes = StaticArgumentAttributes
  {
    optional: true,
    multiple: false,
    default: Some( "default_value" ),
    sensitive: false,
    interactive: false,
  };

  let validation_rules = &[
    StaticValidationRule::MinLength( 3 ),
    StaticValidationRule::Pattern( "^[a-z]+$" ),
  ];

  let static_arg = StaticArgumentDefinition
  {
    name: "test_arg",
    kind: StaticKind::String,
    attributes,
    hint: "Test argument",
    description: "A test argument",
    validation_rules,
    aliases: &[ "ta" ],
    tags: &[ "optional" ],
  };

  let dynamic_arg: ArgumentDefinition = ( &static_arg ).into();

  assert_eq!( dynamic_arg.name, "test_arg" );
  assert!( matches!( dynamic_arg.kind, Kind::String ) );
  assert!( dynamic_arg.attributes.optional );
  assert!( !dynamic_arg.attributes.multiple );
  assert_eq!( dynamic_arg.attributes.default, Some( "default_value".to_string() ) );
  assert!( !dynamic_arg.attributes.sensitive );
  assert!( !dynamic_arg.attributes.interactive );
  assert_eq!( dynamic_arg.hint, "Test argument" );
  assert_eq!( dynamic_arg.description, "A test argument" );
  assert_eq!( dynamic_arg.validation_rules.len(), 2 );
  assert_eq!( dynamic_arg.aliases, vec![ "ta" ] );
  assert_eq!( dynamic_arg.tags, vec![ "optional" ] );
}

#[ test ]
fn test_conversion_static_to_dynamic_kind()
{
  // Test conversion of various StaticKind to Kind
  let test_cases = [
    ( StaticKind::String, Kind::String ),
    ( StaticKind::Integer, Kind::Integer ),
    ( StaticKind::Float, Kind::Float ),
    ( StaticKind::Boolean, Kind::Boolean ),
    ( StaticKind::Path, Kind::Path ),
    ( StaticKind::File, Kind::File ),
    ( StaticKind::Directory, Kind::Directory ),
    ( StaticKind::Url, Kind::Url ),
    ( StaticKind::DateTime, Kind::DateTime ),
    ( StaticKind::Pattern, Kind::Pattern ),
    ( StaticKind::JsonString, Kind::JsonString ),
    ( StaticKind::Object, Kind::Object ),
  ];

  for ( static_kind, expected_kind ) in test_cases
  {
    let converted_kind: Kind = ( &static_kind ).into();
    assert_eq!(
      core::mem::discriminant( &converted_kind ),
      core::mem::discriminant( &expected_kind ),
      "Failed conversion for {static_kind:?}"
    );
  }
}

#[ test ]
fn test_conversion_static_enum_kind()
{
  // Test conversion of StaticKind::Enum to Kind::Enum
  let choices = &[ "choice1", "choice2", "choice3" ];
  let static_enum = StaticKind::Enum( choices );
  let dynamic_enum: Kind = ( &static_enum ).into();

  if let Kind::Enum( dynamic_choices ) = dynamic_enum
  {
    assert_eq!( dynamic_choices, vec![ "choice1", "choice2", "choice3" ] );
  }
  else
  {
    panic!( "Expected Kind::Enum, got {dynamic_enum:?}" );
  }
}

#[ test ]
fn test_conversion_static_list_kind()
{
  // Test conversion of StaticKind::List to Kind::List
  let static_list = StaticKind::List( &StaticKind::String, Some( ',' ) );
  let dynamic_list: Kind = ( &static_list ).into();

  if let Kind::List( item_kind, delimiter ) = dynamic_list
  {
    assert!( matches!( item_kind.as_ref(), Kind::String ) );
    assert_eq!( delimiter, Some( ',' ) );
  }
  else
  {
    panic!( "Expected Kind::List, got {dynamic_list:?}" );
  }
}

#[ test ]
fn test_conversion_static_map_kind()
{
  // Test conversion of StaticKind::Map to Kind::Map
  let static_map = StaticKind::Map( &StaticKind::String, &StaticKind::Integer, Some( ',' ), Some( ':' ) );
  let dynamic_map: Kind = ( &static_map ).into();

  if let Kind::Map( key_kind, value_kind, entry_delimiter, kv_delimiter ) = dynamic_map
  {
    assert!( matches!( key_kind.as_ref(), Kind::String ) );
    assert!( matches!( value_kind.as_ref(), Kind::Integer ) );
    assert_eq!( entry_delimiter, Some( ',' ) );
    assert_eq!( kv_delimiter, Some( ':' ) );
  }
  else
  {
    panic!( "Expected Kind::Map, got {dynamic_map:?}" );
  }
}

#[ test ]
fn test_conversion_static_validation_rules()
{
  // Test conversion of all StaticValidationRule variants
  let static_rules = [
    StaticValidationRule::Min( 10.5 ),
    StaticValidationRule::Max( 99.9 ),
    StaticValidationRule::MinLength( 5 ),
    StaticValidationRule::MaxLength( 50 ),
    StaticValidationRule::Pattern( "^test" ),
    StaticValidationRule::MinItems( 2 ),
  ];

  for static_rule in &static_rules
  {
    let dynamic_rule: ValidationRule = static_rule.into();

    match ( static_rule, &dynamic_rule )
    {
      ( StaticValidationRule::Min( val ), ValidationRule::Min( converted_val ) ) |
      ( StaticValidationRule::Max( val ), ValidationRule::Max( converted_val ) ) =>
        assert!( ( val - converted_val ).abs() < f64::EPSILON ),
      ( StaticValidationRule::MinLength( val ), ValidationRule::MinLength( converted_val ) ) |
      ( StaticValidationRule::MaxLength( val ), ValidationRule::MaxLength( converted_val ) ) |
      ( StaticValidationRule::MinItems( val ), ValidationRule::MinItems( converted_val ) ) =>
        assert_eq!( val, converted_val ),
      ( StaticValidationRule::Pattern( pattern ), ValidationRule::Pattern( converted_pattern ) ) =>
        assert_eq!( pattern, converted_pattern ),
      _ => panic!( "Validation rule conversion mismatch" ),
    }
  }
}

#[ test ]
fn test_phf_map_compatibility()
{
  // Test PHF map type compatibility
  use phf::Map;

  // This test verifies that our static structures can be used with PHF maps
  static TEST_COMMANDS: Map< &'static str, &'static StaticCommandDefinition > = phf::phf_map!
  {
    "test" => &StaticCommandDefinition
    {
      name: "test",
      namespace: "test",
      description: "Test command",
      hint: "test",
      arguments: &[],
      routine_link: None,
      status: "stable",
      version: "1.0.0",
      tags: &[],
      aliases: &[],
      permissions: &[],
      idempotent: true,
      deprecation_message: "",
      http_method_hint: "GET",
      examples: &[],
    },
  };

  // Verify the PHF map works correctly
  let cmd = TEST_COMMANDS.get( "test" );
  assert!( cmd.is_some() );

  let cmd = cmd.unwrap();
  assert_eq!( cmd.name, "test" );
  assert_eq!( cmd.description, "Test command" );
}

#[ test ]
fn test_static_command_with_arguments()
{
  // Test StaticCommandDefinition with complex arguments
  static ARG_ATTRIBUTES: StaticArgumentAttributes = StaticArgumentAttributes
  {
    optional: false,
    multiple: true,
    default: None,
    sensitive: false,
    interactive: false,
  };

  static VALIDATION_RULES: &[StaticValidationRule] = &[
    StaticValidationRule::MinLength( 1 ),
    StaticValidationRule::Pattern( "^[a-zA-Z0-9_]+$" ),
  ];

  static STATIC_ARG: StaticArgumentDefinition = StaticArgumentDefinition
  {
    name: "files",
    kind: StaticKind::List( &StaticKind::File, Some( ',' ) ),
    attributes: ARG_ATTRIBUTES,
    hint: "Input files",
    description: "List of input files to process",
    validation_rules: VALIDATION_RULES,
    aliases: &[ "f", "input" ],
    tags: &[ "files", "input" ],
  };

  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition
  {
    name: "process",
    namespace: "file",
    description: "Process multiple files",
    hint: "file processor",
    arguments: &[ STATIC_ARG ],
    routine_link: Some( "process_files" ),
    status: "stable",
    version: "2.0.0",
    tags: &[ "file", "processing" ],
    aliases: &[ "proc", "p" ],
    permissions: &[ "read", "write" ],
    idempotent: false,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[ "process --files file1.txt,file2.txt" ],
  };

  // Test the command structure
  assert_eq!( STATIC_CMD.arguments.len(), 1 );
  assert_eq!( STATIC_CMD.arguments[ 0 ].name, "files" );
  assert!( matches!( STATIC_CMD.arguments[ 0 ].kind, StaticKind::List( _, _ ) ) );
  assert!( STATIC_CMD.arguments[ 0 ].attributes.multiple );
  assert!( !STATIC_CMD.arguments[ 0 ].attributes.optional );

  // Test conversion to dynamic
  let dynamic_cmd: CommandDefinition = ( &STATIC_CMD ).into();
  assert_eq!( dynamic_cmd.arguments.len(), 1 );
  assert_eq!( dynamic_cmd.arguments[ 0 ].name, "files" );
  assert!( dynamic_cmd.arguments[ 0 ].attributes.multiple );
}

#[ test ]
fn test_static_command_serialization_roundtrip()
{
  // Test that static structures can be serialized and used for code generation
  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition
  {
    name: "serialize_test",
    namespace: "test",
    description: "Test serialization",
    hint: "serialization test",
    arguments: &[],
    routine_link: Some( "serialize_test_routine" ),
    status: "experimental",
    version: "0.1.0",
    tags: &[ "test", "serialization" ],
    aliases: &[ "st" ],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[ "serialize_test" ],
  };

  // Convert to dynamic and verify data integrity
  let dynamic_cmd: CommandDefinition = ( &STATIC_CMD ).into();

  // Verify all fields match
  assert_eq!( dynamic_cmd.name, STATIC_CMD.name );
  assert_eq!( dynamic_cmd.namespace, STATIC_CMD.namespace );
  assert_eq!( dynamic_cmd.description, STATIC_CMD.description );
  assert_eq!( dynamic_cmd.hint, STATIC_CMD.hint );
  assert_eq!( dynamic_cmd.routine_link.as_deref(), STATIC_CMD.routine_link );
  assert_eq!( dynamic_cmd.status, STATIC_CMD.status );
  assert_eq!( dynamic_cmd.version, STATIC_CMD.version );
  assert_eq!( dynamic_cmd.tags, STATIC_CMD.tags.iter().map( | &s | s.to_string() ).collect::< Vec< _ > >() );
  assert_eq!( dynamic_cmd.aliases, STATIC_CMD.aliases.iter().map( | &s | s.to_string() ).collect::< Vec< _ > >() );
  assert_eq!( dynamic_cmd.idempotent, STATIC_CMD.idempotent );
  assert_eq!( dynamic_cmd.examples, STATIC_CMD.examples.iter().map( | &s | s.to_string() ).collect::< Vec< _ > >() );
}

#[ test ]
fn test_static_command_map_wrapper()
{
  // Test the StaticCommandMap wrapper functionality
  use phf::phf_map;

  const COMMAND_MAP_INTERNAL: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map!
  {
    "cmd1" => &StaticCommandDefinition
    {
      name: "cmd1",
      namespace: "test",
      description: "First command",
      hint: "cmd1",
      arguments: &[],
      routine_link: None,
      status: "stable",
      version: "1.0.0",
      tags: &[],
      aliases: &[],
      permissions: &[],
      idempotent: true,
      deprecation_message: "",
      http_method_hint: "GET",
      examples: &[],
    },
    "cmd2" => &StaticCommandDefinition
    {
      name: "cmd2",
      namespace: "test",
      description: "Second command",
      hint: "cmd2",
      arguments: &[],
      routine_link: None,
      status: "stable",
      version: "1.0.0",
      tags: &[],
      aliases: &[],
      permissions: &[],
      idempotent: true,
      deprecation_message: "",
      http_method_hint: "POST",
      examples: &[],
    },
  };

  static COMMAND_MAP: unilang::static_data::StaticCommandMap = unilang::static_data::StaticCommandMap::from_phf_internal(&COMMAND_MAP_INTERNAL);

  // Test map functionality through wrapper
  assert_eq!( COMMAND_MAP.len(), 2 );
  assert!( COMMAND_MAP.contains_key( "cmd1" ) );
  assert!( COMMAND_MAP.contains_key( "cmd2" ) );
  assert!( !COMMAND_MAP.contains_key( "cmd3" ) );

  let cmd1 = COMMAND_MAP.get( "cmd1" ).unwrap();
  assert_eq!( cmd1.name, "cmd1" );
  assert_eq!( cmd1.description, "First command" );

  let cmd2 = COMMAND_MAP.get( "cmd2" ).unwrap();
  assert_eq!( cmd2.name, "cmd2" );
  assert_eq!( cmd2.description, "Second command" );
}