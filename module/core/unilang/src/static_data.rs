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
  /// to enable compile-time storage in optimized static registries.
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

  impl StaticCommandDefinition
  {
    /// Creates a new `StaticCommandDefinition` with required fields and sensible defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unilang::static_data::StaticCommandDefinition;
    ///
    /// const CMD: StaticCommandDefinition = StaticCommandDefinition::new(
    ///   ".greet",
    ///   "",
    ///   "Greets the user",
    /// );
    /// assert_eq!( CMD.name, ".greet" );
    /// ```
    #[ must_use ]
    pub const fn new(
      name : &'static str,
      namespace : &'static str,
      description : &'static str,
    ) -> Self
    {
      Self
      {
        name,
        namespace,
        description,
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
      }
    }

    /// Sets the hint for the command.
    #[ must_use ]
    pub const fn with_hint( mut self, hint : &'static str ) -> Self
    {
      self.hint = hint;
      self
    }

    /// Sets the arguments for the command.
    #[ must_use ]
    pub const fn with_arguments( mut self, arguments : &'static [ StaticArgumentDefinition ] ) -> Self
    {
      self.arguments = arguments;
      self
    }

    /// Sets the routine link for the command.
    #[ must_use ]
    pub const fn with_routine_link( mut self, routine_link : &'static str ) -> Self
    {
      self.routine_link = Some( routine_link );
      self
    }

    /// Sets the status for the command.
    #[ must_use ]
    pub const fn with_status( mut self, status : &'static str ) -> Self
    {
      self.status = status;
      self
    }

    /// Sets the version for the command.
    #[ must_use ]
    pub const fn with_version( mut self, version : &'static str ) -> Self
    {
      self.version = version;
      self
    }

    /// Sets the tags for the command.
    #[ must_use ]
    pub const fn with_tags( mut self, tags : &'static [ &'static str ] ) -> Self
    {
      self.tags = tags;
      self
    }

    /// Sets the aliases for the command.
    #[ must_use ]
    pub const fn with_aliases( mut self, aliases : &'static [ &'static str ] ) -> Self
    {
      self.aliases = aliases;
      self
    }

    /// Sets the permissions for the command.
    #[ must_use ]
    pub const fn with_permissions( mut self, permissions : &'static [ &'static str ] ) -> Self
    {
      self.permissions = permissions;
      self
    }

    /// Sets whether the command is idempotent.
    #[ must_use ]
    pub const fn with_idempotent( mut self, idempotent : bool ) -> Self
    {
      self.idempotent = idempotent;
      self
    }

    /// Sets the deprecation message for the command.
    #[ must_use ]
    pub const fn with_deprecation_message( mut self, deprecation_message : &'static str ) -> Self
    {
      self.deprecation_message = deprecation_message;
      self
    }

    /// Sets the HTTP method hint for the command.
    #[ must_use ]
    pub const fn with_http_method_hint( mut self, http_method_hint : &'static str ) -> Self
    {
      self.http_method_hint = http_method_hint;
      self
    }

    /// Sets the examples for the command.
    #[ must_use ]
    pub const fn with_examples( mut self, examples : &'static [ &'static str ] ) -> Self
    {
      self.examples = examples;
      self
    }
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

  impl StaticArgumentDefinition
  {
    /// Creates a new `StaticArgumentDefinition` with required fields and sensible defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unilang::static_data::{ StaticArgumentDefinition, StaticKind };
    ///
    /// const ARG: StaticArgumentDefinition = StaticArgumentDefinition::new(
    ///   "name",
    ///   StaticKind::String,
    ///   "User name",
    /// );
    /// assert_eq!( ARG.name, "name" );
    /// ```
    #[ must_use ]
    pub const fn new(
      name : &'static str,
      kind : StaticKind,
      description : &'static str,
    ) -> Self
    {
      Self
      {
        name,
        kind,
        attributes : StaticArgumentAttributes::new(),
        hint : "",
        description,
        validation_rules : &[],
        aliases : &[],
        tags : &[],
      }
    }

    /// Sets the attributes for the argument.
    #[ must_use ]
    pub const fn with_attributes( mut self, attributes : StaticArgumentAttributes ) -> Self
    {
      self.attributes = attributes;
      self
    }

    /// Sets the hint for the argument.
    #[ must_use ]
    pub const fn with_hint( mut self, hint : &'static str ) -> Self
    {
      self.hint = hint;
      self
    }

    /// Sets the validation rules for the argument.
    #[ must_use ]
    pub const fn with_validation_rules( mut self, validation_rules : &'static [ StaticValidationRule ] ) -> Self
    {
      self.validation_rules = validation_rules;
      self
    }

    /// Sets the aliases for the argument.
    #[ must_use ]
    pub const fn with_aliases( mut self, aliases : &'static [ &'static str ] ) -> Self
    {
      self.aliases = aliases;
      self
    }

    /// Sets the tags for the argument.
    #[ must_use ]
    pub const fn with_tags( mut self, tags : &'static [ &'static str ] ) -> Self
    {
      self.tags = tags;
      self
    }
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

  impl StaticArgumentAttributes
  {
    /// Creates a new `StaticArgumentAttributes` with sensible defaults.
    ///
    /// Defaults: required (not optional), single value, no default, not sensitive, not interactive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unilang::static_data::StaticArgumentAttributes;
    ///
    /// const ATTRS: StaticArgumentAttributes = StaticArgumentAttributes::new();
    /// const OPTIONAL_ATTRS: StaticArgumentAttributes = StaticArgumentAttributes::new().with_optional( true );
    /// assert!( !ATTRS.optional );
    /// assert!( OPTIONAL_ATTRS.optional );
    /// ```
    #[ must_use ]
    pub const fn new() -> Self
    {
      Self
      {
        optional : false,
        multiple : false,
        default : None,
        sensitive : false,
        interactive : false,
      }
    }

    /// Sets whether the argument is optional.
    #[ must_use ]
    pub const fn with_optional( mut self, optional : bool ) -> Self
    {
      self.optional = optional;
      self
    }

    /// Sets whether the argument can accept multiple values.
    #[ must_use ]
    pub const fn with_multiple( mut self, multiple : bool ) -> Self
    {
      self.multiple = multiple;
      self
    }

    /// Sets the default value for the argument.
    #[ must_use ]
    pub const fn with_default( mut self, default : &'static str ) -> Self
    {
      self.default = Some( default );
      self
    }

    /// Sets whether the argument contains sensitive data.
    #[ must_use ]
    pub const fn with_sensitive( mut self, sensitive : bool ) -> Self
    {
      self.sensitive = sensitive;
      self
    }

    /// Sets whether the argument might require user interaction.
    #[ must_use ]
    pub const fn with_interactive( mut self, interactive : bool ) -> Self
    {
      self.interactive = interactive;
      self
    }
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

  ///
  /// Wrapper for static command maps with zero-overhead compile-time lookup.
  ///
  /// This struct provides optimized O(1) command lookup using compile-time
  /// generated lookup tables. Downstream crates have zero additional dependencies.
  ///
  /// ## Performance
  ///
  /// - Lookup: O(1) with ~80ns overhead
  /// - Memory: Zero runtime allocation (compile-time data)
  /// - Binary size: Minimal (<1KB per 100 commands)
  ///
  /// ## Usage
  ///
  /// ```rust,ignore
  /// // Generated by build.rs
  /// pub static STATIC_COMMANDS: StaticCommandMap = /* ... */;
  ///
  /// // Access commands
  /// if let Some(cmd) = STATIC_COMMANDS.get(".help") {
  ///   println!("Command: {}", cmd.name);
  /// }
  /// ```
  #[ derive( Debug ) ]
  pub struct StaticCommandMap
  {
    /// Internal optimized map - implementation detail, never exposed in public API
    inner: &'static phf::Map< &'static str, &'static StaticCommandDefinition >,
  }

  impl StaticCommandMap
  {
    /// Create from internal optimized map (internal use only by generated code).
    ///
    /// This method is hidden from documentation and should only be called
    /// by code generated in build.rs.
    #[ doc( hidden ) ]
    #[ inline ]
    pub const fn from_phf_internal( map: &'static phf::Map< &'static str, &'static StaticCommandDefinition > ) -> Self
    {
      Self { inner: map }
    }

    /// Get a command definition by name.
    ///
    /// Returns `Some(&StaticCommandDefinition)` if found, `None` otherwise.
    ///
    /// ## Performance
    ///
    /// O(1) operation using perfect hash lookup (~80ns).
    #[ inline ]
    #[ must_use ]
    pub fn get( &self, name: &str ) -> Option< &'static StaticCommandDefinition >
    {
      self.inner.get( name ).copied()
    }

    /// Check if a command exists in the map.
    #[ inline ]
    #[ must_use ]
    pub fn contains_key( &self, name: &str ) -> bool
    {
      self.inner.contains_key( name )
    }

    /// Get an iterator over all command names.
    #[ must_use ]
    pub fn keys( &self ) -> impl Iterator< Item = &&'static str >
    {
      self.inner.keys()
    }

    /// Get an iterator over all (name, definition) pairs.
    #[ must_use ]
    pub fn entries( &self ) -> impl Iterator< Item = (&&'static str, &&'static StaticCommandDefinition) >
    {
      self.inner.entries()
    }

    /// Get an iterator over all command definitions.
    #[ must_use ]
    pub fn values( &self ) -> impl Iterator< Item = &&'static StaticCommandDefinition >
    {
      self.inner.values()
    }

    /// Get the number of commands in the map.
    #[ inline ]
    #[ must_use ]
    pub fn len( &self ) -> usize
    {
      self.inner.len()
    }

    /// Check if the map is empty.
    #[ inline ]
    #[ must_use ]
    pub fn is_empty( &self ) -> bool
    {
      self.inner.is_empty()
    }
  }

  impl std::ops::Index< &str > for StaticCommandMap
  {
    type Output = &'static StaticCommandDefinition;

    /// Index access panics if key not found - use `get()` for safe access.
    fn index( &self, name: &str ) -> &Self::Output
    {
      self.inner.get( name )
        .unwrap_or_else( || panic!( "Command '{}' not found in static map", name ) )
    }
  }

  // Conversion implementations to convert from static to dynamic versions
  impl From< &'static StaticCommandDefinition > for crate::data::CommandDefinition
  {
    fn from( static_cmd : &'static StaticCommandDefinition ) -> Self
    {
      use crate::data::{ CommandName, CommandStatus, VersionType };

      let status = if static_cmd.deprecation_message.is_empty()
      {
        // Parse status string
        match static_cmd.status.to_lowercase().as_str()
        {
          "experimental" => CommandStatus::Experimental,
          "internal" => CommandStatus::Internal,
          _ => CommandStatus::Active,
        }
      }
      else
      {
        CommandStatus::Deprecated
        {
          reason : static_cmd.deprecation_message.to_string(),
          since : None,
          replacement : None,
        }
      };

      crate::data::CommandDefinition::new(
        CommandName::new( static_cmd.name ).expect( "valid static command name" ),
        static_cmd.description.to_string(),
      )
      .with_namespace( static_cmd.namespace.to_string() )
      .with_hint( static_cmd.hint )
      .with_arguments( static_cmd.arguments.iter().map( core::convert::Into::into ).collect() )
      .with_routine_link( static_cmd.routine_link.map( str::to_string ) )
      .with_status( status )
      .with_version( VersionType::new( static_cmd.version ).expect( "valid static version" ) )
      .with_tags( static_cmd.tags.iter().map( | &s | s.to_string() ).collect() )
      .with_aliases( static_cmd.aliases.iter().map( | &s | s.to_string() ).collect() )
      .with_permissions( static_cmd.permissions.iter().map( | &s | s.to_string() ).collect() )
      .with_idempotent( static_cmd.idempotent )
      .with_deprecation_message( static_cmd.deprecation_message )
      .with_http_method_hint( static_cmd.http_method_hint )
      .with_examples( static_cmd.examples.iter().map( | &s | s.to_string() ).collect() )
      .with_auto_help( false ) // Static commands don't auto-generate help by default
      .with_category( "" )
      .with_short_desc( "" )
      .with_hidden_from_list( false )
      .with_priority( 0 )
      .with_group( "" )
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
  exposed use private::StaticCommandMap;

  prelude use private::StaticCommandDefinition;
  prelude use private::StaticArgumentDefinition;
  prelude use private::StaticArgumentAttributes;
  prelude use private::StaticKind;
  prelude use private::StaticValidationRule;
  prelude use private::StaticCommandMap;
}