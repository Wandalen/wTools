//!
//! Contains `const`-compatible data structures for static command definitions.
//!

/// Internal namespace.
mod private
{
  ///
  /// Static, const-compatible version of `CommandDefinition`.
  ///
  /// Uses &'static str and &'static [...] instead of String and Vec
  /// to enable compile-time storage in PHF maps.
  #[ derive( Debug, Clone ) ]
  pub struct StaticCommandDefinition
  {
    /// The name of the command, used to invoke it from the command line.
    pub name : &'static str,
    /// The namespace of the command.
    pub namespace : &'static str,
    /// A brief, one-line description of what the command does.
    pub description : &'static str,
    /// A short hint for the command.
    pub hint : &'static str,
    /// A list of arguments that the command accepts.
    pub arguments : &'static [ StaticArgumentDefinition ],
    /// An optional link to the routine that executes this command.
    pub routine_link : Option< &'static str >,
    /// The status of the command.
    pub status : &'static str,
    /// The version of the command.
    pub version : &'static str,
    /// Tags associated with the command.
    pub tags : &'static [ &'static str ],
    /// Aliases for the command.
    pub aliases : &'static [ &'static str ],
    /// Permissions required to execute the command.
    pub permissions : &'static [ &'static str ],
    /// Indicates if the command is idempotent.
    pub idempotent : bool,
    /// If `status` is `Deprecated`, explains the reason and suggests alternatives.
    pub deprecation_message : &'static str,
    /// A suggested HTTP method (`GET`, `POST`, etc.) for the Web API modality.
    pub http_method_hint : &'static str,
    /// Illustrative usage examples for help text.
    pub examples : &'static [ &'static str ],
  }

  ///
  /// Static, const-compatible version of `ArgumentDefinition`.
  ///
  #[ derive( Debug, Clone, Copy ) ]
  pub struct StaticArgumentDefinition
  {
    /// The name of the argument, used to reference it in commands and validation.
    pub name : &'static str,
    /// The data type and structure expected for this argument.
    pub kind : StaticKind,
    /// Attributes that control the behavior of this argument.
    pub attributes : StaticArgumentAttributes,
    /// A brief, one-line hint about the argument's purpose.
    pub hint : &'static str,
    /// A more detailed description of the argument.
    pub description : &'static str,
    /// Validation rules that apply to this argument.
    pub validation_rules : &'static [ StaticValidationRule ],
    /// Alternative names for this argument.
    pub aliases : &'static [ &'static str ],
    /// Tags associated with this argument.
    pub tags : &'static [ &'static str ],
  }

  ///
  /// Static, const-compatible version of `ArgumentAttributes`.
  ///
  #[allow(clippy::struct_excessive_bools)]
  #[ derive( Debug, Clone, Copy ) ]
  pub struct StaticArgumentAttributes
  {
    /// Indicates if the argument is optional.
    pub optional : bool,
    /// Indicates if the argument can accept multiple values.
    pub multiple : bool,
    /// The default value for the argument if not provided.
    pub default : Option< &'static str >,
    /// Indicates if the argument contains sensitive data.
    pub sensitive : bool,
    /// Indicates if the argument might require user interaction.
    pub interactive : bool,
  }

  ///
  /// Static, const-compatible version of Kind.
  ///
  #[ derive( Debug, Clone, Copy ) ]
  pub enum StaticKind
  {
    /// A simple text string.
    String,
    /// An integer number.
    Integer,
    /// A floating-point number.
    Float,
    /// A boolean value.
    Boolean,
    /// A file system path.
    Path,
    /// A file system path that must point to an existing file.
    File,
    /// A file system path that must point to an existing directory.
    Directory,
    /// An enumeration with a predefined set of allowed values.
    Enum( &'static [ &'static str ] ),
    /// A URL (web address).
    Url,
    /// A date and time value.
    DateTime,
    /// A regular expression pattern.
    Pattern,
    /// A list (array) of values of the same type.
    List( &'static StaticKind, Option< char > ),
    /// A map (dictionary) of key-value pairs.
    Map( &'static StaticKind, &'static StaticKind, Option< char >, Option< char > ),
    /// A JSON string.
    JsonString,
    /// A generic object.
    Object,
  }

  ///
  /// Static, const-compatible version of `ValidationRule`.
  ///
  #[ derive( Debug, Clone, Copy ) ]
  pub enum StaticValidationRule
  {
    /// Minimum value for numeric types.
    Min( f64 ),
    /// Maximum value for numeric types.
    Max( f64 ),
    /// Minimum length for string types.
    MinLength( usize ),
    /// Maximum length for string types.
    MaxLength( usize ),
    /// Pattern that string values must match.
    Pattern( &'static str ),
    /// Minimum number of items for collection types.
    MinItems( usize ),
  }

  // Conversion implementations to convert from static to dynamic versions
  impl From< &'static StaticCommandDefinition > for crate::data::CommandDefinition
  {
    fn from( static_cmd : &'static StaticCommandDefinition ) -> Self
    {
      crate::data::CommandDefinition
      {
        name : static_cmd.name.to_string(),
        namespace : static_cmd.namespace.to_string(),
        description : static_cmd.description.to_string(),
        hint : static_cmd.hint.to_string(),
        arguments : static_cmd.arguments.iter().map( core::convert::Into::into ).collect(),
        routine_link : static_cmd.routine_link.map( str::to_string ),
        status : static_cmd.status.to_string(),
        version : static_cmd.version.to_string(),
        tags : static_cmd.tags.iter().map( | &s | s.to_string() ).collect(),
        aliases : static_cmd.aliases.iter().map( | &s | s.to_string() ).collect(),
        permissions : static_cmd.permissions.iter().map( | &s | s.to_string() ).collect(),
        idempotent : static_cmd.idempotent,
        deprecation_message : static_cmd.deprecation_message.to_string(),
        http_method_hint : static_cmd.http_method_hint.to_string(),
        examples : static_cmd.examples.iter().map( | &s | s.to_string() ).collect(),
      }
    }
  }

  impl From< &StaticArgumentDefinition > for crate::data::ArgumentDefinition
  {
    fn from( static_arg : &StaticArgumentDefinition ) -> Self
    {
      crate::data::ArgumentDefinition
      {
        name : static_arg.name.to_string(),
        kind : ( &static_arg.kind ).into(),
        attributes : ( &static_arg.attributes ).into(),
        hint : static_arg.hint.to_string(),
        description : static_arg.description.to_string(),
        validation_rules : static_arg.validation_rules.iter().map( core::convert::Into::into ).collect(),
        aliases : static_arg.aliases.iter().map( | &s | s.to_string() ).collect(),
        tags : static_arg.tags.iter().map( | &s | s.to_string() ).collect(),
      }
    }
  }

  impl From< &StaticArgumentAttributes > for crate::data::ArgumentAttributes
  {
    fn from( static_attrs : &StaticArgumentAttributes ) -> Self
    {
      crate::data::ArgumentAttributes
      {
        optional : static_attrs.optional,
        multiple : static_attrs.multiple,
        default : static_attrs.default.map( str::to_string ),
        sensitive : static_attrs.sensitive,
        interactive : static_attrs.interactive,
      }
    }
  }

  impl From< &StaticKind > for crate::data::Kind
  {
    fn from( static_kind : &StaticKind ) -> Self
    {
      match static_kind
      {
        StaticKind::String => crate::data::Kind::String,
        StaticKind::Integer => crate::data::Kind::Integer,
        StaticKind::Float => crate::data::Kind::Float,
        StaticKind::Boolean => crate::data::Kind::Boolean,
        StaticKind::Path => crate::data::Kind::Path,
        StaticKind::File => crate::data::Kind::File,
        StaticKind::Directory => crate::data::Kind::Directory,
        StaticKind::Enum( choices ) => crate::data::Kind::Enum( choices.iter().map( | &s | s.to_string() ).collect() ),
        StaticKind::Url => crate::data::Kind::Url,
        StaticKind::DateTime => crate::data::Kind::DateTime,
        StaticKind::Pattern => crate::data::Kind::Pattern,
        StaticKind::List( item_kind, delimiter ) => crate::data::Kind::List( Box::new( ( *item_kind ).into() ), *delimiter ),
        StaticKind::Map( key_kind, value_kind, entry_delimiter, kv_delimiter ) => 
          crate::data::Kind::Map( Box::new( ( *key_kind ).into() ), Box::new( ( *value_kind ).into() ), *entry_delimiter, *kv_delimiter ),
        StaticKind::JsonString => crate::data::Kind::JsonString,
        StaticKind::Object => crate::data::Kind::Object,
      }
    }
  }

  impl From< &StaticValidationRule > for crate::data::ValidationRule
  {
    fn from( static_rule : &StaticValidationRule ) -> Self
    {
      match static_rule
      {
        StaticValidationRule::Min( value ) => crate::data::ValidationRule::Min( *value ),
        StaticValidationRule::Max( value ) => crate::data::ValidationRule::Max( *value ),
        StaticValidationRule::MinLength( value ) => crate::data::ValidationRule::MinLength( *value ),
        StaticValidationRule::MaxLength( value ) => crate::data::ValidationRule::MaxLength( *value ),
        StaticValidationRule::Pattern( pattern ) => crate::data::ValidationRule::Pattern( (*pattern).to_string() ),
        StaticValidationRule::MinItems( value ) => crate::data::ValidationRule::MinItems( *value ),
      }
    }
  }

  #[cfg(test)]
  mod tests
  {
    use super::*;

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

      let dynamic_cmd: crate::data::CommandDefinition = (&STATIC_CMD).into();

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
      let string_kind: crate::data::Kind = (&StaticKind::String).into();
      assert!(matches!(string_kind, crate::data::Kind::String));

      let integer_kind: crate::data::Kind = (&StaticKind::Integer).into();
      assert!(matches!(integer_kind, crate::data::Kind::Integer));

      let float_kind: crate::data::Kind = (&StaticKind::Float).into();
      assert!(matches!(float_kind, crate::data::Kind::Float));

      let boolean_kind: crate::data::Kind = (&StaticKind::Boolean).into();
      assert!(matches!(boolean_kind, crate::data::Kind::Boolean));

      let path_kind: crate::data::Kind = (&StaticKind::Path).into();
      assert!(matches!(path_kind, crate::data::Kind::Path));

      let file_kind: crate::data::Kind = (&StaticKind::File).into();
      assert!(matches!(file_kind, crate::data::Kind::File));

      let directory_kind: crate::data::Kind = (&StaticKind::Directory).into();
      assert!(matches!(directory_kind, crate::data::Kind::Directory));

      let url_kind: crate::data::Kind = (&StaticKind::Url).into();
      assert!(matches!(url_kind, crate::data::Kind::Url));

      let datetime_kind: crate::data::Kind = (&StaticKind::DateTime).into();
      assert!(matches!(datetime_kind, crate::data::Kind::DateTime));

      let pattern_kind: crate::data::Kind = (&StaticKind::Pattern).into();
      assert!(matches!(pattern_kind, crate::data::Kind::Pattern));

      let json_string_kind: crate::data::Kind = (&StaticKind::JsonString).into();
      assert!(matches!(json_string_kind, crate::data::Kind::JsonString));

      let object_kind: crate::data::Kind = (&StaticKind::Object).into();
      assert!(matches!(object_kind, crate::data::Kind::Object));
    }

    #[test]
    fn test_static_kind_conversion_enum()
    {
      let static_enum = StaticKind::Enum(&["red", "green", "blue"]);
      let dynamic_kind: crate::data::Kind = (&static_enum).into();
      
      if let crate::data::Kind::Enum(choices) = dynamic_kind {
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
      let dynamic_kind: crate::data::Kind = (&static_list).into();

      if let crate::data::Kind::List(inner_kind, delimiter) = dynamic_kind {
        assert!(matches!(*inner_kind, crate::data::Kind::String));
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
      let dynamic_kind: crate::data::Kind = (&static_map).into();

      if let crate::data::Kind::Map(k_kind, v_kind, entry_delim, kv_delim) = dynamic_kind {
        assert!(matches!(*k_kind, crate::data::Kind::String));
        assert!(matches!(*v_kind, crate::data::Kind::Integer));
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
      let dynamic_rule: crate::data::ValidationRule = (&min_rule).into();
      assert!(matches!(dynamic_rule, crate::data::ValidationRule::Min(10.0)));

      // Test Max rule
      let max_rule = StaticValidationRule::Max(100.0);
      let dynamic_rule: crate::data::ValidationRule = (&max_rule).into();
      assert!(matches!(dynamic_rule, crate::data::ValidationRule::Max(100.0)));

      // Test MinLength rule
      let min_length_rule = StaticValidationRule::MinLength(5);
      let dynamic_rule: crate::data::ValidationRule = (&min_length_rule).into();
      assert!(matches!(dynamic_rule, crate::data::ValidationRule::MinLength(5)));

      // Test MaxLength rule
      let max_length_rule = StaticValidationRule::MaxLength(50);
      let dynamic_rule: crate::data::ValidationRule = (&max_length_rule).into();
      assert!(matches!(dynamic_rule, crate::data::ValidationRule::MaxLength(50)));

      // Test Pattern rule
      let pattern_rule = StaticValidationRule::Pattern(r"\d+");
      let dynamic_rule: crate::data::ValidationRule = (&pattern_rule).into();
      if let crate::data::ValidationRule::Pattern(pattern) = dynamic_rule {
        assert_eq!(pattern, r"\d+");
      } else {
        panic!("Expected Pattern validation rule");
      }

      // Test MinItems rule
      let min_items_rule = StaticValidationRule::MinItems(3);
      let dynamic_rule: crate::data::ValidationRule = (&min_items_rule).into();
      assert!(matches!(dynamic_rule, crate::data::ValidationRule::MinItems(3)));
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

      let dynamic_attrs: crate::data::ArgumentAttributes = (&static_attrs).into();

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

      let dynamic_arg: crate::data::ArgumentDefinition = (&STATIC_ARG).into();

      assert_eq!(dynamic_arg.name, "complex_arg");
      assert!(matches!(dynamic_arg.kind, crate::data::Kind::Float));
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

      let dynamic_cmd: crate::data::CommandDefinition = (&STATIC_CMD).into();

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
  }
}

mod_interface::mod_interface!
{
  exposed use private::StaticCommandDefinition;
  exposed use private::StaticArgumentDefinition;
  exposed use private::StaticArgumentAttributes;
  exposed use private::StaticKind;
  exposed use private::StaticValidationRule;
}