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

}

mod_interface::mod_interface!
{
  exposed use private::StaticCommandDefinition;
  exposed use private::StaticArgumentDefinition;
  exposed use private::StaticArgumentAttributes;
  exposed use private::StaticKind;
  exposed use private::StaticValidationRule;
}