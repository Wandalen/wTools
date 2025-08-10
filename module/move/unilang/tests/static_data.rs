//!
//! Tests for the `static_data` module
//!

use unilang::static_data::*;

#[test]
fn test_static_command_definition_conversion()
{
  static STATIC_ARG: StaticArgumentDefinition = StaticArgumentDefinition {
    name: "test_arg",
    kind: StaticKind::String,
    attributes: StaticArgumentAttributes {
      optional: true,
      multiple: false,
      default: Some("default_value"),
      sensitive: false,
      interactive: false,
    },
    hint: "test hint",
    description: "test description",
    validation_rules: &[],
    aliases: &["alias1", "alias2"],
    tags: &["tag1", "tag2"],
  };

  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition {
    name: "test_command",
    namespace: ".test",
    description: "A test command",
    hint: "Test hint",
    arguments: &[STATIC_ARG],
    routine_link: Some("test.routine"),
    status: "stable",
    version: "1.0.0",
    tags: &["test", "example"],
    aliases: &["tc", "test"],
    permissions: &["user", "admin"],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &["test_command arg::value"],
  };

  let dynamic_cmd: unilang::data::CommandDefinition = (&STATIC_CMD).into();

  assert_eq!(dynamic_cmd.name, "test_command");
  assert_eq!(dynamic_cmd.namespace, ".test");
  assert_eq!(dynamic_cmd.description, "A test command");
  assert_eq!(dynamic_cmd.hint, "Test hint");
  assert_eq!(dynamic_cmd.status, "stable");
  assert_eq!(dynamic_cmd.version, "1.0.0");
  assert_eq!(dynamic_cmd.tags, vec!["test", "example"]);
  assert_eq!(dynamic_cmd.aliases, vec!["tc", "test"]);
  assert_eq!(dynamic_cmd.permissions, vec!["user", "admin"]);
  assert!(dynamic_cmd.idempotent);
  assert_eq!(dynamic_cmd.deprecation_message, "");
  assert_eq!(dynamic_cmd.http_method_hint, "GET");
  assert_eq!(dynamic_cmd.examples, vec!["test_command arg::value"]);
  assert_eq!(dynamic_cmd.routine_link, Some("test.routine".to_string()));

  assert_eq!(dynamic_cmd.arguments.len(), 1);
  let arg = &dynamic_cmd.arguments[0];
  assert_eq!(arg.name, "test_arg");
  assert_eq!(arg.hint, "test hint");
  assert_eq!(arg.description, "test description");
  assert_eq!(arg.aliases, vec!["alias1", "alias2"]);
  assert_eq!(arg.tags, vec!["tag1", "tag2"]);
  assert!(arg.attributes.optional);
  assert!(!arg.attributes.multiple);
  assert_eq!(arg.attributes.default, Some("default_value".to_string()));
  assert!(!arg.attributes.sensitive);
  assert!(!arg.attributes.interactive);
}

#[test]
fn test_static_kind_conversion_primitives()
{
  // Test primitive types
  let string_kind: unilang::data::Kind = (&StaticKind::String).into();
  assert!(matches!(string_kind, unilang::data::Kind::String));

  let integer_kind: unilang::data::Kind = (&StaticKind::Integer).into();
  assert!(matches!(integer_kind, unilang::data::Kind::Integer));

  let float_kind: unilang::data::Kind = (&StaticKind::Float).into();
  assert!(matches!(float_kind, unilang::data::Kind::Float));

  let boolean_kind: unilang::data::Kind = (&StaticKind::Boolean).into();
  assert!(matches!(boolean_kind, unilang::data::Kind::Boolean));

  let path_kind: unilang::data::Kind = (&StaticKind::Path).into();
  assert!(matches!(path_kind, unilang::data::Kind::Path));

  let file_kind: unilang::data::Kind = (&StaticKind::File).into();
  assert!(matches!(file_kind, unilang::data::Kind::File));

  let directory_kind: unilang::data::Kind = (&StaticKind::Directory).into();
  assert!(matches!(directory_kind, unilang::data::Kind::Directory));

  let url_kind: unilang::data::Kind = (&StaticKind::Url).into();
  assert!(matches!(url_kind, unilang::data::Kind::Url));

  let datetime_kind: unilang::data::Kind = (&StaticKind::DateTime).into();
  assert!(matches!(datetime_kind, unilang::data::Kind::DateTime));

  let pattern_kind: unilang::data::Kind = (&StaticKind::Pattern).into();
  assert!(matches!(pattern_kind, unilang::data::Kind::Pattern));

  let json_string_kind: unilang::data::Kind = (&StaticKind::JsonString).into();
  assert!(matches!(json_string_kind, unilang::data::Kind::JsonString));

  let object_kind: unilang::data::Kind = (&StaticKind::Object).into();
  assert!(matches!(object_kind, unilang::data::Kind::Object));
}

#[test]
fn test_static_kind_conversion_enum()
{
  let static_enum = StaticKind::Enum(&["red", "green", "blue"]);
  let dynamic_kind: unilang::data::Kind = (&static_enum).into();
  
  if let unilang::data::Kind::Enum(choices) = dynamic_kind {
    assert_eq!(choices, vec!["red", "green", "blue"]);
  } else {
    panic!("Expected Enum kind");
  }
}

#[test]
fn test_static_kind_conversion_list()
{
  static ITEM_KIND: StaticKind = StaticKind::String;
  let static_list = StaticKind::List(&ITEM_KIND, Some(','));
  let dynamic_kind: unilang::data::Kind = (&static_list).into();

  if let unilang::data::Kind::List(inner_kind, delimiter) = dynamic_kind {
    assert!(matches!(*inner_kind, unilang::data::Kind::String));
    assert_eq!(delimiter, Some(','));
  } else {
    panic!("Expected List kind");
  }
}

#[test]
fn test_static_kind_conversion_map()
{
  static KEY_KIND: StaticKind = StaticKind::String;
  static VALUE_KIND: StaticKind = StaticKind::Integer;
  let static_map = StaticKind::Map(&KEY_KIND, &VALUE_KIND, Some(','), Some('='));
  let dynamic_kind: unilang::data::Kind = (&static_map).into();

  if let unilang::data::Kind::Map(k_kind, v_kind, entry_delim, kv_delim) = dynamic_kind {
    assert!(matches!(*k_kind, unilang::data::Kind::String));
    assert!(matches!(*v_kind, unilang::data::Kind::Integer));
    assert_eq!(entry_delim, Some(','));
    assert_eq!(kv_delim, Some('='));
  } else {
    panic!("Expected Map kind");
  }
}

#[test]
fn test_static_validation_rule_conversion()
{
  // Test Min rule
  let min_rule = StaticValidationRule::Min(10.0);
  let dynamic_rule: unilang::data::ValidationRule = (&min_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::Min(10.0)));

  // Test Max rule
  let max_rule = StaticValidationRule::Max(100.0);
  let dynamic_rule: unilang::data::ValidationRule = (&max_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::Max(100.0)));

  // Test MinLength rule
  let min_length_rule = StaticValidationRule::MinLength(5);
  let dynamic_rule: unilang::data::ValidationRule = (&min_length_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::MinLength(5)));

  // Test MaxLength rule
  let max_length_rule = StaticValidationRule::MaxLength(50);
  let dynamic_rule: unilang::data::ValidationRule = (&max_length_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::MaxLength(50)));

  // Test Pattern rule
  let pattern_rule = StaticValidationRule::Pattern(r"\d+");
  let dynamic_rule: unilang::data::ValidationRule = (&pattern_rule).into();
  if let unilang::data::ValidationRule::Pattern(pattern) = dynamic_rule {
    assert_eq!(pattern, r"\d+");
  } else {
    panic!("Expected Pattern validation rule");
  }

  // Test MinItems rule
  let min_items_rule = StaticValidationRule::MinItems(3);
  let dynamic_rule: unilang::data::ValidationRule = (&min_items_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::MinItems(3)));
}

#[test]
fn test_static_argument_attributes_conversion()
{
  let static_attrs = StaticArgumentAttributes {
    optional: true,
    multiple: false,
    default: Some("test_default"),
    sensitive: true,
    interactive: false,
  };

  let dynamic_attrs: unilang::data::ArgumentAttributes = (&static_attrs).into();

  assert!(dynamic_attrs.optional);
  assert!(!dynamic_attrs.multiple);
  assert_eq!(dynamic_attrs.default, Some("test_default".to_string()));
  assert!(dynamic_attrs.sensitive);
  assert!(!dynamic_attrs.interactive);
}

#[test]
fn test_static_argument_definition_conversion()
{
  static VALIDATION_RULES: [StaticValidationRule; 2] = [
    StaticValidationRule::Min(0.0),
    StaticValidationRule::MaxLength(100),
  ];

  static STATIC_ARG: StaticArgumentDefinition = StaticArgumentDefinition {
    name: "complex_arg",
    kind: StaticKind::Float,
    attributes: StaticArgumentAttributes {
      optional: false,
      multiple: true,
      default: None,
      sensitive: false,
      interactive: true,
    },
    hint: "Complex argument hint",
    description: "A complex argument for testing",
    validation_rules: &VALIDATION_RULES,
    aliases: &["ca", "complex"],
    tags: &["complex", "test"],
  };

  let dynamic_arg: unilang::data::ArgumentDefinition = (&STATIC_ARG).into();

  assert_eq!(dynamic_arg.name, "complex_arg");
  assert!(matches!(dynamic_arg.kind, unilang::data::Kind::Float));
  assert!(!dynamic_arg.attributes.optional);
  assert!(dynamic_arg.attributes.multiple);
  assert_eq!(dynamic_arg.attributes.default, None);
  assert!(!dynamic_arg.attributes.sensitive);
  assert!(dynamic_arg.attributes.interactive);
  assert_eq!(dynamic_arg.hint, "Complex argument hint");
  assert_eq!(dynamic_arg.description, "A complex argument for testing");
  assert_eq!(dynamic_arg.aliases, vec!["ca", "complex"]);
  assert_eq!(dynamic_arg.tags, vec!["complex", "test"]);
  assert_eq!(dynamic_arg.validation_rules.len(), 2);
}

#[test]
fn test_static_command_definition_with_empty_arrays()
{
  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition {
    name: "minimal_command",
    namespace: ".minimal",
    description: "Minimal command",
    hint: "Minimal hint",
    arguments: &[],
    routine_link: None,
    status: "experimental",
    version: "0.1.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: false,
    deprecation_message: "Deprecated for testing",
    http_method_hint: "POST",
    examples: &[],
  };

  let dynamic_cmd: unilang::data::CommandDefinition = (&STATIC_CMD).into();

  assert_eq!(dynamic_cmd.name, "minimal_command");
  assert_eq!(dynamic_cmd.namespace, ".minimal");
  assert!(dynamic_cmd.arguments.is_empty());
  assert_eq!(dynamic_cmd.routine_link, None);
  assert_eq!(dynamic_cmd.status, "experimental");
  assert_eq!(dynamic_cmd.version, "0.1.0");
  assert!(dynamic_cmd.tags.is_empty());
  assert!(dynamic_cmd.aliases.is_empty());
  assert!(dynamic_cmd.permissions.is_empty());
  assert!(!dynamic_cmd.idempotent);
  assert_eq!(dynamic_cmd.deprecation_message, "Deprecated for testing");
  assert_eq!(dynamic_cmd.http_method_hint, "POST");
  assert!(dynamic_cmd.examples.is_empty());
}