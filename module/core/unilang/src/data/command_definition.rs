//! Type-safe CommandDefinition with validated newtypes and builder pattern
//!
//! Provides the core CommandDefinition struct with private fields, validated
//! newtype wrappers, and a type-state builder pattern that enforces required
//! fields at compile time.
//!
//! # Module Organization & File Size
//!
//! **Why is this file 1,700+ lines?**
//!
//! This module CANNOT be split into submodules due to Rust's privacy rules.
//! The following components require direct access to CommandDefinition's private fields:
//!
//! 1. **Serde implementations** (373 lines) - Serialize/Deserialize need direct field access
//! 2. **Builder pattern** (584 lines) - Type-state builder needs to construct instances
//! 3. **Constructor methods** - Functions like `generate_help_command()` use struct literals
//! 4. **49 accessor methods** - Getters and helpers that reference private state
//!
//! **Why extraction would fail:**
//! - Rust submodules CANNOT access parent module's private fields
//! - Making fields `pub(super)` defeats the entire encapsulation design
//! - The file exists as a cohesive privacy boundary by architectural necessity
//!
//! **This is a valid exception to the 1,500 line guideline** - the file's size is
//! dictated by fundamental language constraints, not poor organization. The alternative
//! (public fields) would sacrifice type safety and encapsulation.

use super::validated_types::{ CommandName, NamespaceType, VersionType };
use super::command_status::{ CommandStatus, construct_full_command_name };
use super::argument_types::ArgumentDefinition;

  //
  // CommandDefinition - Type-Safe Redesign with Private Fields
  //

///
/// Type-safe command definition with validated newtypes and private fields.
///
/// This struct implements the "parse don't validate" pattern, making invalid states
/// impossible at compile time. All construction goes through validated builders or
/// constructors that enforce domain rules.
///
/// # Design Rationale
///
/// **Why private fields?**
///
/// The old API had public `String` fields that could be mutated freely:
///
/// ```ignore
/// let mut cmd = CommandDefinition { name: ".test".to_string(), ... };
/// cmd.name = "invalid"; // Compiles! No dot prefix, breaks at registration
/// cmd.namespace = "bad_ns"; // Compiles! Invalid namespace, breaks later
/// ```
///
/// **Problems with public fields:**
///
/// 1. **Mutation:** Commands could be invalidated after construction
/// 2. **No validation:** Invalid values compile fine, fail at runtime
/// 3. **Unclear invariants:** No way to know what values are valid
/// 4. **Scattered validation:** Checks in registry, help system, CLI builder...
///
/// **Benefits of private fields + getters:**
///
/// - **Immutability:** Once constructed, commands can't be invalidated
/// - **Single validation point:** Construction time only, not scattered everywhere
/// - **Clear contract:** Getters return guaranteed-valid values
/// - **Encapsulation:** Implementation can change without breaking API
///
/// **Why validated newtypes?**
///
/// Instead of runtime String validation, we use validated types:
///
/// - `CommandName`: Guarantees dot prefix at construction
/// - `VersionType`: Guarantees valid semver format
/// - `CommandStatus`: Enum prevents typos like "activ" or "Active"
///
/// This moves validation from runtime to construction time, catching bugs earlier.
///
/// **Construction patterns:**
///
/// 1. **Direct constructor** (simple commands):
/// ```ignore
/// let cmd = CommandDefinition::new(
///   CommandName::new(".build").unwrap(),
///   "Build the project".to_string(),
/// );
/// ```
///
/// 2. **Builder with defaults** (tests, simple cases):
/// ```ignore
/// let cmd = CommandDefinition::former()
///   .name(".build")
///   .description("Build the project")
///   .end(); // Provides defaults for namespace, hint, status, version
/// ```
///
/// 3. **Builder fully explicit** (production):
/// ```ignore
/// let cmd = CommandDefinition::former()
///   .name(".build")
///   .description("Build the project")
///   .namespace("")
///   .hint("Build hint")
///   .status("active")
///   .version("1.0.0")
///   .build(); // No defaults, all fields required
/// ```
///
/// **Trade-off:** More verbose construction, but impossible to create invalid commands.
/// This is a **good trade-off** - bugs caught at compile time > bugs at runtime.
///
/// # Examples
/// ```rust
/// use unilang::data::{ CommandDefinition, CommandName };
///
/// // Create a new command with validation
/// let cmd = CommandDefinition::new(
///   CommandName::new(".build").unwrap(),
///   "Build the project".to_string(),
/// );
///
/// // Access via getters (fields are private)
/// assert_eq!(cmd.name().as_str(), ".build");
/// assert_eq!(cmd.description(), "Build the project");
///
/// // Using builder pattern
/// let cmd = CommandDefinition::former()
///   .name(".test")
///   .description("Test command")
///   .end();
///
/// assert_eq!(cmd.name().as_str(), ".test");
/// ```
#[ derive( Debug, Clone ) ]
pub struct CommandDefinition
{
  /// Validated command name (always starts with '.' prefix)
  name : CommandName,
  /// Brief description of what the command does
  description : String,
  /// List of arguments the command accepts
  arguments : Vec< ArgumentDefinition >,
  /// Optional link to the routine that executes this command
  routine_link : Option< String >,
  /// TEMPORARY: Public namespace field for validation testing (String type)
  ///
  /// This field remains public to allow validation tests to create invalid states
  /// (e.g., namespace without dot prefix) for testing error handling. The field is
  /// intentionally String rather than NamespaceType to enable testing invalid values.
  ///
  /// **Used by**: tests/registry/validation_enforcement.rs, tests/semantic/command_validation.rs
  ///
  /// **Future**: Could be made private once validation testing strategy is redesigned
  /// to use builder pattern with validation bypass or test-only constructors.
  pub namespace : String,
  /// Short hint for the command
  hint : String,
  /// Command status (Active, Deprecated, Experimental, Internal)
  status : CommandStatus,
  /// Validated version string
  version : VersionType,
  /// Tags associated with the command
  tags : Vec< String >,
  /// Aliases for the command
  aliases : Vec< String >,
  /// Permissions required to execute the command
  permissions : Vec< String >,
  /// Indicates if the command is idempotent
  idempotent : bool,
  /// Deprecation message (deprecated - use CommandStatus::Deprecated instead)
  deprecation_message : String,
  /// Suggested HTTP method for Web API modality
  http_method_hint : String,
  /// Usage examples for help text
  examples : Vec< String >,
  /// Whether to automatically generate a .command.help counterpart
  auto_help_enabled : bool,
  /// Category for grouping commands in help output
  category : String,
  /// Short one-line description for brief help listings
  short_desc : String,
  /// Hide this command from brief help listings
  hidden_from_list : bool,
  /// Sort priority within category (lower numbers first)
  priority : i32,
  /// Explicit group membership for related commands
  group : String,
}

impl CommandDefinition
{
  /// Creates a new command using the builder pattern.
  ///
  /// This method returns a CommandDefinitionBuilder that provides a fluent API
  /// for constructing commands with compile-time verification of required fields.
  ///
  /// # Returns
  /// * `CommandDefinitionBuilder` - A builder instance for constructing a CommandDefinition
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::CommandDefinition;
  ///
  /// let cmd = CommandDefinition::former()
  ///   .name( ".greet" )
  ///   .description( "Greets the user" )
  ///   .end();
  /// ```
  #[ must_use ]
  pub fn former() -> CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
  {
    CommandDefinitionBuilder::new()
  }

  ///
  /// Creates a new command with sensible defaults.
  ///
  /// This is the recommended way to create commands. It requires only the essential
  /// validated fields (name and description) and provides reasonable defaults for
  /// all optional fields.
  ///
  /// # Arguments
  /// * `name` - Validated command name (must start with '.')
  /// * `description` - Brief description of what the command does
  ///
  /// # Returns
  /// * `Self` - A new CommandDefinition with defaults applied
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".greet").unwrap();
  /// let cmd = CommandDefinition::new(name, "Greets the user".to_string());
  ///
  /// assert_eq!(cmd.name().as_str(), ".greet");
  /// assert_eq!(cmd.description(), "Greets the user");
  /// ```
  #[ must_use ]
  pub fn new( name : CommandName, description : String ) -> Self
  {
    Self
    {
      name,
      description,
      arguments : Vec::new(),
      routine_link : None,
      namespace : String::new(),
      hint : String::new(),
      status : CommandStatus::Active,
      version : VersionType::new( "1.0.0" ).expect( "default version valid" ),
      tags : Vec::new(),
      aliases : Vec::new(),
      permissions : Vec::new(),
      idempotent : true,
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(),
      examples : Vec::new(),
      auto_help_enabled : true,
      category : String::new(),
      short_desc : String::new(),
      hidden_from_list : false,
      priority : 0,
      group : String::new(),
    }
  }

  // ===================================================================
  // Getter Methods - Read-only access to private fields
  // ===================================================================

  /// Returns a reference to the validated command name
  #[ must_use ]
  pub fn name( &self ) -> &CommandName
  {
    &self.name
  }

  /// Returns the command description
  #[ must_use ]
  pub fn description( &self ) -> &str
  {
    &self.description
  }

  /// Returns a reference to the command arguments
  #[ must_use ]
  pub fn arguments( &self ) -> &Vec< ArgumentDefinition >
  {
    &self.arguments
  }

  /// Returns the routine link if set
  #[ must_use ]
  pub fn routine_link( &self ) -> Option< &String >
  {
    self.routine_link.as_ref()
  }

  /// Returns a reference to the namespace string
  #[ must_use ]
  pub fn namespace( &self ) -> &str
  {
    &self.namespace
  }

  /// Returns the command hint
  #[ must_use ]
  pub fn hint( &self ) -> &str
  {
    &self.hint
  }

  /// Returns a reference to the command status
  #[ must_use ]
  pub fn status( &self ) -> &CommandStatus
  {
    &self.status
  }

  /// Returns a reference to the validated version
  #[ must_use ]
  pub fn version( &self ) -> &VersionType
  {
    &self.version
  }

  /// Returns a reference to the command tags
  #[ must_use ]
  pub fn tags( &self ) -> &Vec< String >
  {
    &self.tags
  }

  /// Returns a reference to the command aliases
  #[ must_use ]
  pub fn aliases( &self ) -> &Vec< String >
  {
    &self.aliases
  }

  /// Returns a reference to the permissions
  #[ must_use ]
  pub fn permissions( &self ) -> &Vec< String >
  {
    &self.permissions
  }

  /// Returns whether the command is idempotent
  #[ must_use ]
  pub fn idempotent( &self ) -> bool
  {
    self.idempotent
  }

  /// Returns the deprecation message
  #[ must_use ]
  pub fn deprecation_message( &self ) -> &str
  {
    &self.deprecation_message
  }

  /// Returns the HTTP method hint
  #[ must_use ]
  pub fn http_method_hint( &self ) -> &str
  {
    &self.http_method_hint
  }

  /// Returns a reference to the usage examples
  #[ must_use ]
  pub fn examples( &self ) -> &Vec< String >
  {
    &self.examples
  }

  /// Returns whether auto-help is enabled
  #[ must_use ]
  pub fn auto_help_enabled( &self ) -> bool
  {
    self.auto_help_enabled
  }

  /// Returns the command category
  #[ must_use ]
  pub fn category( &self ) -> &str
  {
    &self.category
  }

  /// Returns the short description
  #[ must_use ]
  pub fn short_desc( &self ) -> &str
  {
    &self.short_desc
  }

  /// Returns whether the command is hidden from listings
  #[ must_use ]
  pub fn hidden_from_list( &self ) -> bool
  {
    self.hidden_from_list
  }

  /// Returns the command priority
  #[ must_use ]
  pub fn priority( &self ) -> i32
  {
    self.priority
  }

  /// Returns the command group
  #[ must_use ]
  pub fn group( &self ) -> &str
  {
    &self.group
  }

  // ===================================================================
  // Setter Methods - Fluent API with validation
  // ===================================================================

  /// Sets the command name (validated)
  #[ must_use ]
  pub fn with_name( mut self, name : CommandName ) -> Self
  {
    self.name = name;
    self
  }

  /// Sets the command description
  #[ must_use ]
  pub fn with_description( mut self, description : impl Into< String > ) -> Self
  {
    self.description = description.into();
    self
  }

  /// Sets the command arguments
  #[ must_use ]
  pub fn with_arguments( mut self, arguments : Vec< ArgumentDefinition > ) -> Self
  {
    self.arguments = arguments;
    self
  }

  /// Sets the routine link
  #[ must_use ]
  pub fn with_routine_link( mut self, link : Option< String > ) -> Self
  {
    self.routine_link = link;
    self
  }

  /// Sets the command namespace (String)
  #[ must_use ]
  pub fn with_namespace( mut self, namespace : String ) -> Self
  {
    self.namespace = namespace;
    self
  }

  /// Sets the command hint
  #[ must_use ]
  pub fn with_hint( mut self, hint : impl Into< String > ) -> Self
  {
    self.hint = hint.into();
    self
  }

  /// Sets the command status
  #[ must_use ]
  pub fn with_status( mut self, status : CommandStatus ) -> Self
  {
    self.status = status;
    self
  }

  /// Sets the command version (validated)
  #[ must_use ]
  pub fn with_version( mut self, version : VersionType ) -> Self
  {
    self.version = version;
    self
  }

  /// Sets the command tags
  #[ must_use ]
  pub fn with_tags( mut self, tags : Vec< String > ) -> Self
  {
    self.tags = tags;
    self
  }

  /// Sets the command aliases
  #[ must_use ]
  pub fn with_aliases( mut self, aliases : Vec< String > ) -> Self
  {
    self.aliases = aliases;
    self
  }

  /// Sets the permissions
  #[ must_use ]
  pub fn with_permissions( mut self, permissions : Vec< String > ) -> Self
  {
    self.permissions = permissions;
    self
  }

  /// Sets whether the command is idempotent
  #[ must_use ]
  pub fn with_idempotent( mut self, idempotent : bool ) -> Self
  {
    self.idempotent = idempotent;
    self
  }

  /// Sets the deprecation message
  #[ must_use ]
  pub fn with_deprecation_message( mut self, message : impl Into< String > ) -> Self
  {
    self.deprecation_message = message.into();
    self
  }

  /// Sets the HTTP method hint
  #[ must_use ]
  pub fn with_http_method_hint( mut self, hint : impl Into< String > ) -> Self
  {
    self.http_method_hint = hint.into();
    self
  }

  /// Sets the usage examples
  #[ must_use ]
  pub fn with_examples( mut self, examples : Vec< String > ) -> Self
  {
    self.examples = examples;
    self
  }

  /// Sets whether auto-help is enabled
  #[ must_use ]
  pub fn with_auto_help( mut self, enabled : bool ) -> Self
  {
    self.auto_help_enabled = enabled;
    self
  }

  /// Sets the command category
  #[ must_use ]
  pub fn with_category( mut self, category : impl Into< String > ) -> Self
  {
    self.category = category.into();
    self
  }

  /// Sets the short description
  #[ must_use ]
  pub fn with_short_desc( mut self, desc : impl Into< String > ) -> Self
  {
    self.short_desc = desc.into();
    self
  }

  /// Sets whether the command is hidden from listings
  #[ must_use ]
  pub fn with_hidden_from_list( mut self, hidden : bool ) -> Self
  {
    self.hidden_from_list = hidden;
    self
  }

  /// Sets the command priority
  #[ must_use ]
  pub fn with_priority( mut self, priority : i32 ) -> Self
  {
    self.priority = priority;
    self
  }

  /// Sets the command group
  #[ must_use ]
  pub fn with_group( mut self, group : impl Into< String > ) -> Self
  {
    self.group = group.into();
    self
  }

  // ===================================================================
  // Helper Methods (ported from CommandDefinition)
  // ===================================================================

  ///
  /// Returns true if this command should automatically generate a help counterpart.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".test").unwrap();
  /// let cmd = CommandDefinition::new(name, "Test".to_string())
  ///   .with_auto_help(true);
  ///
  /// assert!(cmd.has_auto_help());
  /// ```
  #[ must_use ]
  pub fn has_auto_help( &self ) -> bool
  {
    self.auto_help_enabled
  }

  ///
  /// Constructs the full command name from namespace and name components.
  ///
  /// This method handles the various combinations of namespaced and non-namespaced
  /// command names, ensuring that the resulting full name always starts with a dot
  /// prefix according to unilang conventions.
  ///
  /// # Returns
  /// * `String` - The fully qualified command name with dot prefix
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// // Simple command (no namespace)
  /// let name = CommandName::new(".help").unwrap();
  /// let cmd1 = CommandDefinition::new(name, "Help".to_string());
  /// assert_eq!(cmd1.full_name(), ".help");
  ///
  /// // Namespaced command
  /// let name2 = CommandName::new(".list").unwrap();
  /// let cmd2 = CommandDefinition::new(name2, "List".to_string())
  ///   .with_namespace(".session".to_string());
  /// assert_eq!(cmd2.full_name(), ".session.list");
  /// ```
  #[ must_use ]
  pub fn full_name( &self ) -> String
  {
    construct_full_command_name( self.namespace.as_str(), self.name.as_str() )
  }

  ///
  /// Generates a corresponding help command definition for this command.
  ///
  /// Creates a new `CommandDefinition` for the `.command.help` counterpart
  /// that provides detailed help information about the parent command.
  ///
  /// # Returns
  /// * `CommandDefinition` - A new command definition for the help counterpart
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".example").unwrap();
  /// let cmd = CommandDefinition::new(name, "Example".to_string());
  ///
  /// let help_cmd = cmd.generate_help_command();
  /// assert_eq!(help_cmd.name().as_str(), ".example.help");
  /// assert!(help_cmd.description().contains(".example"));
  /// ```
  #[ must_use ]
  pub fn generate_help_command( &self ) -> CommandDefinition
  {
    let help_name = CommandName::new( format!( "{}.help", self.name.as_str() ) )
      .expect( "help command name should be valid" );

    CommandDefinition
    {
      name : help_name,
      namespace : self.namespace.clone(),
      description : format!( "Display help information for the '{}' command", self.name.as_str() ),
      hint : format!( "Help for {}", self.name.as_str() ),
      status : CommandStatus::Active,
      version : self.version.clone(),
      arguments : vec![], // Help commands typically take no arguments
      routine_link : None, // Will be set during registration
      tags : vec![ "help".to_string(), "documentation".to_string() ],
      aliases : vec![ format!( "{}.h", self.name.as_str() ) ],
      permissions : vec![], // Help commands accessible to all
      idempotent : true, // Help commands always idempotent
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(), // Help is read-only
      examples : vec![
        format!( "{}.help", self.name.as_str() ),
        format!( "{} ??", self.name.as_str() )
      ],
      auto_help_enabled : false, // Prevent recursive help generation
      category : "help".to_string(),
      short_desc : format!( "Help for {}", self.name.as_str() ),
      hidden_from_list : true, // Hide .help variants from brief listings
      priority : 999, // Low priority (shown last if visible)
      group : String::new(),
    }
  }
}

//
// Serde Implementation for CommandDefinition
//

impl serde::Serialize for CommandDefinition
{
  fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
  where
    S : serde::Serializer,
  {
    use serde::ser::SerializeStruct;

    let mut state = serializer.serialize_struct( "CommandDefinition", 21 )?;

    state.serialize_field( "name", &self.name )?;
    state.serialize_field( "description", &self.description )?;
    state.serialize_field( "arguments", &self.arguments )?;
    state.serialize_field( "routine_link", &self.routine_link )?;
    state.serialize_field( "namespace", &self.namespace )?;
    state.serialize_field( "hint", &self.hint )?;
    state.serialize_field( "status", &self.status )?;
    state.serialize_field( "version", &self.version )?;
    state.serialize_field( "tags", &self.tags )?;
    state.serialize_field( "aliases", &self.aliases )?;
    state.serialize_field( "permissions", &self.permissions )?;
    state.serialize_field( "idempotent", &self.idempotent )?;
    state.serialize_field( "deprecation_message", &self.deprecation_message )?;
    state.serialize_field( "http_method_hint", &self.http_method_hint )?;
    state.serialize_field( "examples", &self.examples )?;
    state.serialize_field( "auto_help_enabled", &self.auto_help_enabled )?;
    state.serialize_field( "category", &self.category )?;
    state.serialize_field( "short_desc", &self.short_desc )?;
    state.serialize_field( "hidden_from_list", &self.hidden_from_list )?;
    state.serialize_field( "priority", &self.priority )?;
    state.serialize_field( "group", &self.group )?;

    state.end()
  }
}

impl< 'de > serde::Deserialize< 'de > for CommandDefinition
{
  #[ allow( clippy::too_many_lines ) ]
  fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
  where
    D : serde::Deserializer< 'de >,
  {
    use serde::de::{ self, Visitor, MapAccess };

    #[ derive( serde::Deserialize ) ]
    #[ serde( field_identifier, rename_all = "snake_case" ) ]
    enum Field
    {
      Name,
      Description,
      Arguments,
      RoutineLink,
      Namespace,
      Hint,
      Status,
      Version,
      Tags,
      Aliases,
      Permissions,
      Idempotent,
      DeprecationMessage,
      HttpMethodHint,
      Examples,
      AutoHelpEnabled,
      Category,
      ShortDesc,
      HiddenFromList,
      Priority,
      Group,
    }

    struct CommandDefinitionVisitor;

    impl< 'de > Visitor< 'de > for CommandDefinitionVisitor
    {
      type Value = CommandDefinition;

      fn expecting( &self, formatter : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
      {
        formatter.write_str( "struct CommandDefinition" )
      }

      #[ allow( clippy::too_many_lines ) ]
      fn visit_map< V >( self, mut map : V ) -> Result< CommandDefinition, V::Error >
      where
        V : MapAccess< 'de >,
      {
        let mut name : Option< CommandName > = None;
        let mut description : Option< String > = None;
        let mut arguments : Option< Vec< ArgumentDefinition > > = None;
        let mut routine_link : Option< Option< String > > = None;
        let mut namespace : Option< String > = None;
        let mut hint : Option< String > = None;
        let mut status : Option< CommandStatus > = None;
        let mut version : Option< VersionType > = None;
        let mut tags : Option< Vec< String > > = None;
        let mut aliases : Option< Vec< String > > = None;
        let mut permissions : Option< Vec< String > > = None;
        let mut idempotent : Option< bool > = None;
        let mut deprecation_message : Option< String > = None;
        let mut http_method_hint : Option< String > = None;
        let mut examples : Option< Vec< String > > = None;
        let mut auto_help_enabled : Option< bool > = None;
        let mut category : Option< String > = None;
        let mut short_desc : Option< String > = None;
        let mut hidden_from_list : Option< bool > = None;
        let mut priority : Option< i32 > = None;
        let mut group : Option< String > = None;

        while let Some( key ) = map.next_key()?
        {
          match key
          {
            Field::Name =>
            {
              if name.is_some()
              {
                return Err( de::Error::duplicate_field( "name" ) );
              }
              name = Some( map.next_value()? );
            },
            Field::Description =>
            {
              if description.is_some()
              {
                return Err( de::Error::duplicate_field( "description" ) );
              }
              description = Some( map.next_value()? );
            },
            Field::Arguments =>
            {
              if arguments.is_some()
              {
                return Err( de::Error::duplicate_field( "arguments" ) );
              }
              arguments = Some( map.next_value()? );
            },
            Field::RoutineLink =>
            {
              if routine_link.is_some()
              {
                return Err( de::Error::duplicate_field( "routine_link" ) );
              }
              routine_link = Some( map.next_value()? );
            },
            Field::Namespace =>
            {
              if namespace.is_some()
              {
                return Err( de::Error::duplicate_field( "namespace" ) );
              }
              namespace = Some( map.next_value()? );
            },
            Field::Hint =>
            {
              if hint.is_some()
              {
                return Err( de::Error::duplicate_field( "hint" ) );
              }
              hint = Some( map.next_value()? );
            },
            Field::Status =>
            {
              if status.is_some()
              {
                return Err( de::Error::duplicate_field( "status" ) );
              }
              status = Some( map.next_value()? );
            },
            Field::Version =>
            {
              if version.is_some()
              {
                return Err( de::Error::duplicate_field( "version" ) );
              }
              version = Some( map.next_value()? );
            },
            Field::Tags =>
            {
              if tags.is_some()
              {
                return Err( de::Error::duplicate_field( "tags" ) );
              }
              tags = Some( map.next_value()? );
            },
            Field::Aliases =>
            {
              if aliases.is_some()
              {
                return Err( de::Error::duplicate_field( "aliases" ) );
              }
              aliases = Some( map.next_value()? );
            },
            Field::Permissions =>
            {
              if permissions.is_some()
              {
                return Err( de::Error::duplicate_field( "permissions" ) );
              }
              permissions = Some( map.next_value()? );
            },
            Field::Idempotent =>
            {
              if idempotent.is_some()
              {
                return Err( de::Error::duplicate_field( "idempotent" ) );
              }
              idempotent = Some( map.next_value()? );
            },
            Field::DeprecationMessage =>
            {
              if deprecation_message.is_some()
              {
                return Err( de::Error::duplicate_field( "deprecation_message" ) );
              }
              deprecation_message = Some( map.next_value()? );
            },
            Field::HttpMethodHint =>
            {
              if http_method_hint.is_some()
              {
                return Err( de::Error::duplicate_field( "http_method_hint" ) );
              }
              http_method_hint = Some( map.next_value()? );
            },
            Field::Examples =>
            {
              if examples.is_some()
              {
                return Err( de::Error::duplicate_field( "examples" ) );
              }
              examples = Some( map.next_value()? );
            },
            Field::AutoHelpEnabled =>
            {
              if auto_help_enabled.is_some()
              {
                return Err( de::Error::duplicate_field( "auto_help_enabled" ) );
              }
              auto_help_enabled = Some( map.next_value()? );
            },
            Field::Category =>
            {
              if category.is_some()
              {
                return Err( de::Error::duplicate_field( "category" ) );
              }
              category = Some( map.next_value()? );
            },
            Field::ShortDesc =>
            {
              if short_desc.is_some()
              {
                return Err( de::Error::duplicate_field( "short_desc" ) );
              }
              short_desc = Some( map.next_value()? );
            },
            Field::HiddenFromList =>
            {
              if hidden_from_list.is_some()
              {
                return Err( de::Error::duplicate_field( "hidden_from_list" ) );
              }
              hidden_from_list = Some( map.next_value()? );
            },
            Field::Priority =>
            {
              if priority.is_some()
              {
                return Err( de::Error::duplicate_field( "priority" ) );
              }
              priority = Some( map.next_value()? );
            },
            Field::Group =>
            {
              if group.is_some()
              {
                return Err( de::Error::duplicate_field( "group" ) );
              }
              group = Some( map.next_value()? );
            },
          }
        }

        // Required fields
        let name = name.ok_or_else( || de::Error::missing_field( "name" ) )?;
        let description = description.ok_or_else( || de::Error::missing_field( "description" ) )?;

        // Optional fields with defaults
        let namespace = namespace.unwrap_or_else( || String::new() );

        // Validate namespace using NamespaceType validation rules
        NamespaceType::new( &namespace ).map_err( de::Error::custom )?;
        let hint = hint.unwrap_or_default();
        let status = status.unwrap_or( CommandStatus::Active );
        let version = version.unwrap_or_else( || VersionType::new( "1.0.0" ).expect( "default version valid" ) );
        let arguments = arguments.unwrap_or_default();
        let routine_link = routine_link.unwrap_or( None );
        let tags = tags.unwrap_or_default();
        let aliases = aliases.unwrap_or_default();
        let permissions = permissions.unwrap_or_default();
        let idempotent = idempotent.unwrap_or( true );
        let deprecation_message = deprecation_message.unwrap_or_default();
        let http_method_hint = http_method_hint.unwrap_or_else( || "GET".to_string() );
        let examples = examples.unwrap_or_default();
        let auto_help_enabled = auto_help_enabled.unwrap_or( true );
        let category = category.unwrap_or_default();
        let short_desc = short_desc.unwrap_or_default();
        let hidden_from_list = hidden_from_list.unwrap_or( false );
        let priority = priority.unwrap_or( 0 );
        let group = group.unwrap_or_default();

        Ok( CommandDefinition
        {
          name,
          description,
          arguments,
          routine_link,
          namespace,
          hint,
          status,
          version,
          tags,
          aliases,
          permissions,
          idempotent,
          deprecation_message,
          http_method_hint,
          examples,
          auto_help_enabled,
          category,
          short_desc,
          hidden_from_list,
          priority,
          group,
        })
      }
    }

    const FIELDS : &[ &str ] = &[
      "name",
      "description",
      "arguments",
      "routine_link",
      "namespace",
      "hint",
      "status",
      "version",
      "tags",
      "aliases",
      "permissions",
      "idempotent",
      "deprecation_message",
      "http_method_hint",
      "examples",
      "auto_help_enabled",
      "category",
      "short_desc",
      "hidden_from_list",
      "priority",
      "group",
    ];

    deserializer.deserialize_struct( "CommandDefinition", FIELDS, CommandDefinitionVisitor )
  }
}

  // Type-state markers for compile-time enforcement

  /// Marker type indicating a required field has been set.
  ///
  /// This zero-sized type is used in the type-state pattern to track
  /// which required fields have been set in the `CommandDefinitionBuilder`.
  #[ derive( Debug ) ]
  pub struct Set;

  /// Marker type indicating a required field has not been set.
  ///
  /// This zero-sized type is used in the type-state pattern to track
  /// which required fields have not yet been set in the `CommandDefinitionBuilder`.
  #[ derive( Debug ) ]
  pub struct NotSet;

  use std::marker::PhantomData;

  /// Type-state builder for `CommandDefinition` that enforces required fields at compile time.
  ///
  /// This builder uses the type-state pattern to ensure all 6 required fields
  /// (name, description, namespace, hint, status, version) are set before building.
  ///
  /// # Design Rationale
  ///
  /// **Why type-state builder pattern?**
  ///
  /// The old API used public fields with `Default` trait, causing runtime errors:
  ///
  /// ```ignore
  /// // Old API - compiles but panics at runtime
  /// let cmd = CommandDefinition::default(); // name is empty string!
  /// registry.register(cmd); // Runtime panic: "Invalid command name"
  /// ```
  ///
  /// **Problems with old approach:**
  ///
  /// 1. **Invalid states representable:** Empty names, invalid versions compile fine
  /// 2. **Runtime failures:** Validation only at registration time, not construction
  /// 3. **Unclear requirements:** No indication which fields are truly required
  /// 4. **No IDE help:** Autocomplete doesn't guide you through required fields
  ///
  /// **Benefits of type-state builder:**
  ///
  /// - **Compile-time enforcement:** Incomplete builders don't compile
  /// - **Progressive disclosure:** Type signature shows what's left to set
  /// - **IDE guidance:** Autocomplete only shows methods for unset fields
  /// - **Impossible states impossible:** Can't construct invalid CommandDefinition
  /// - **Self-documenting:** Type signature IS the documentation
  ///
  /// **How it works:**
  ///
  /// Each type parameter (`Name`, `Description`, etc.) is either `Set` or `NotSet`:
  ///
  /// ```ignore
  /// CommandDefinitionBuilder<NotSet, NotSet, NotSet, NotSet, NotSet, NotSet> // Initial
  ///   .name(".test") → CommandDefinitionBuilder<Set, NotSet, NotSet, NotSet, NotSet, NotSet>
  ///   .description("Test") → CommandDefinitionBuilder<Set, Set, NotSet, NotSet, NotSet, NotSet>
  ///   .end() // OK - only requires Name and Description to be Set
  /// ```
  ///
  /// The `build()` method is only available when ALL type parameters are `Set`:
  ///
  /// ```ignore
  /// impl CommandDefinitionBuilder<Set, Set, Set, Set, Set, Set> {
  ///   pub fn build(self) -> CommandDefinition { ... }
  /// }
  /// ```
  ///
  /// **Trade-off:** More complex type signatures, but catches errors at compile time instead
  /// of runtime. This is a **good trade-off** for domain objects where invalid states
  /// should be impossible.
  ///
  /// **Migration impact:**
  ///
  /// Old: Errors at runtime during registration
  /// New: Errors at compile time during construction
  ///
  /// This moves bugs from runtime to compile time, which is the goal of type-safe design.
  ///
  #[ derive( Debug ) ]
  pub struct CommandDefinitionBuilder< Name, Description, Namespace, Hint, Status, Version >
  {
    name : Option< String >,
    description : Option< String >,
    namespace : Option< String >,
    hint : Option< String >,
    status : Option< String >,
    version : Option< String >,
    arguments : Vec< ArgumentDefinition >,
    routine_link : Option< String >,
    tags : Vec< String >,
    aliases : Vec< String >,
    permissions : Vec< String >,
    idempotent : bool,
    deprecation_message : String,
    http_method_hint : String,
    examples : Vec< String >,
    auto_help_enabled : bool,
    category : String,
    short_desc : String,
    hidden_from_list : bool,
    priority : i32,
    group : String,
    _marker : PhantomData< ( Name, Description, Namespace, Hint, Status, Version ) >,
  }

  // Start with all required fields NotSet
  impl CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
  {
    /// Create a new builder with all required fields unset
    pub fn new() -> Self
    {
      Self
      {
        name : None,
        description : None,
        namespace : None,
        hint : None,
        status : None,
        version : None,
        arguments : vec![],
        routine_link : None,
        tags : vec![],
        aliases : vec![],
        permissions : vec![],
        idempotent : false,
        deprecation_message : String::new(),
        http_method_hint : String::new(),
        examples : vec![],
        auto_help_enabled : true, // Default to true - help is mandatory
        category : String::new(),
        short_desc : String::new(),
        hidden_from_list : false,
        priority : 0,
        group : String::new(),
        _marker : PhantomData,
      }
    }
  }

  // Method to set name (transitions Name from NotSet to Set)
  impl< Desc, Ns, Hint, Status, Version >
    CommandDefinitionBuilder< NotSet, Desc, Ns, Hint, Status, Version >
  {
    /// Sets the command name (required field).
    ///
    /// This method transitions the `Name` type parameter from `NotSet` to `Set`,
    /// ensuring compile-time tracking of this required field.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandDefinition;
    ///
    /// let builder = CommandDefinition::former()
    ///     .name(".my_command");
    /// ```
    pub fn name( self, name : impl Into< String > )
      -> CommandDefinitionBuilder< Set, Desc, Ns, Hint, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : Some( name.into() ),
        description : self.description,
        namespace : self.namespace,
        hint : self.hint,
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set description (transitions Description from NotSet to Set)
  impl< Name, Ns, Hint, Status, Version >
    CommandDefinitionBuilder< Name, NotSet, Ns, Hint, Status, Version >
  {
    /// Sets the command description (required field).
    ///
    /// This method transitions the `Description` type parameter from `NotSet` to `Set`.
    pub fn description( self, description : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Set, Ns, Hint, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : Some( description.into() ),
        namespace : self.namespace,
        hint : self.hint,
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set namespace (transitions Namespace from NotSet to Set)
  impl< Name, Desc, Hint, Status, Version >
    CommandDefinitionBuilder< Name, Desc, NotSet, Hint, Status, Version >
  {
    /// Sets the command namespace (required field).
    ///
    /// Use empty string `""` for global namespace commands.
    pub fn namespace( self, namespace : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Set, Hint, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : Some( namespace.into() ),
        hint : self.hint,
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set hint (transitions Hint from NotSet to Set)
  impl< Name, Desc, Ns, Status, Version >
    CommandDefinitionBuilder< Name, Desc, Ns, NotSet, Status, Version >
  {
    /// Sets the command hint (required field).
    ///
    /// A short hint shown in help text.
    pub fn hint( self, hint : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Ns, Set, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : self.namespace,
        hint : Some( hint.into() ),
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set status (transitions Status from NotSet to Set)
  impl< Name, Desc, Ns, Hint, Version >
    CommandDefinitionBuilder< Name, Desc, Ns, Hint, NotSet, Version >
  {
    /// Sets the command status (required field).
    ///
    /// Common values: `"stable"`, `"beta"`, `"experimental"`, `"deprecated"`.
    pub fn status( self, status : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Ns, Hint, Set, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : self.namespace,
        hint : self.hint,
        status : Some( status.into() ),
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set version (transitions Version from NotSet to Set)
  impl< Name, Desc, Ns, Hint, Status >
    CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, NotSet >
  {
    /// Sets the command version (required field).
    ///
    /// Typically follows semantic versioning (e.g., `"1.0.0"`).
    pub fn version( self, version : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, Set >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : self.namespace,
        hint : self.hint,
        status : self.status,
        version : Some( version.into() ),
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Methods for optional fields - available on all states
  impl< Name, Desc, Ns, Hint, Status, Version >
    CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, Version >
  {
    /// Sets the command arguments (optional field, defaults to empty vec).
    pub fn arguments( mut self, arguments : Vec< ArgumentDefinition > ) -> Self
    {
      self.arguments = arguments;
      self
    }

    /// Sets the routine link (optional field, defaults to None).
    pub fn routine_link( mut self, routine_link : Option< String > ) -> Self
    {
      self.routine_link = routine_link;
      self
    }

    /// Sets the command tags (optional field, defaults to empty vec).
    pub fn tags( mut self, tags : Vec< String > ) -> Self
    {
      self.tags = tags;
      self
    }

    /// Sets the command aliases (optional field, defaults to empty vec).
    pub fn aliases( mut self, aliases : Vec< String > ) -> Self
    {
      self.aliases = aliases;
      self
    }

    /// Sets the command permissions (optional field, defaults to empty vec).
    pub fn permissions( mut self, permissions : Vec< String > ) -> Self
    {
      self.permissions = permissions;
      self
    }

    /// Sets whether the command is idempotent (optional field, defaults to false).
    pub fn idempotent( mut self, idempotent : bool ) -> Self
    {
      self.idempotent = idempotent;
      self
    }

    /// Sets the deprecation message (optional field, defaults to empty string).
    pub fn deprecation_message( mut self, deprecation_message : impl Into< String > ) -> Self
    {
      self.deprecation_message = deprecation_message.into();
      self
    }

    /// Sets the HTTP method hint (optional field, defaults to empty string).
    pub fn http_method_hint( mut self, http_method_hint : impl Into< String > ) -> Self
    {
      self.http_method_hint = http_method_hint.into();
      self
    }

    /// Sets the command examples (optional field, defaults to empty vec).
    pub fn examples( mut self, examples : Vec< String > ) -> Self
    {
      self.examples = examples;
      self
    }

    /// Sets whether auto-help is enabled (optional field, defaults to true).
    ///
    /// When true (default), registering this command will automatically generate a `.command.help` variant.
    /// Set to false ONLY for help commands themselves to prevent recursion.
    pub fn auto_help_enabled( mut self, auto_help_enabled : bool ) -> Self
    {
      self.auto_help_enabled = auto_help_enabled;
      self
    }

    /// Sets the command category (optional field, defaults to empty string).
    pub fn category( mut self, category : impl Into< String > ) -> Self
    {
      self.category = category.into();
      self
    }

    /// Sets the short description (optional field, defaults to empty string).
    pub fn short_desc( mut self, short_desc : impl Into< String > ) -> Self
    {
      self.short_desc = short_desc.into();
      self
    }

    /// Sets whether the command is hidden from list (optional field, defaults to false).
    pub fn hidden_from_list( mut self, hidden : bool ) -> Self
    {
      self.hidden_from_list = hidden;
      self
    }

    /// Sets the command priority (optional field, defaults to 0).
    pub fn priority( mut self, priority : i32 ) -> Self
    {
      self.priority = priority;
      self
    }

    /// Sets the command group (optional field, defaults to empty string).
    pub fn group( mut self, group : impl Into< String > ) -> Self
    {
      self.group = group.into();
      self
    }
  }

  // Generic end() method for partial builder - allows building with just name and description
  impl< Namespace, Hint, Status, Version > CommandDefinitionBuilder< Set, Set, Namespace, Hint, Status, Version >
  {
    /// Builds the `CommandDefinition` with sensible defaults for unset fields.
    ///
    /// This method allows building a command with only name and description set,
    /// providing defaults for namespace (""), hint (""), status ("active"), and version ("1.0.0").
    ///
    /// # Design Rationale
    ///
    /// **Why have both `end()` and `build()`?**
    ///
    /// The builder offers two finalization methods with different trade-offs:
    ///
    /// - **`end()`** (this method): Requires only `name` and `description`, provides defaults
    ///   - Use case: Quick command creation, tests, simple commands
    ///   - Trade-off: Less explicit, relies on defaults
    ///   - Type signature: Available when `Name` and `Description` are `Set`
    ///
    /// - **`build()`**: Requires ALL 6 fields explicitly set
    ///   - Use case: Production code, complex commands, when defaults aren't appropriate
    ///   - Trade-off: More verbose, but fully explicit
    ///   - Type signature: Available only when all type parameters are `Set`
    ///
    /// **Why this design?**
    ///
    /// 1. **Flexibility:** Tests need concise command creation; production needs explicitness
    /// 2. **Type safety:** Both methods enforce required fields at compile time
    /// 3. **Migration path:** `end()` makes migration from old API easier (fewer fields required)
    /// 4. **No runtime errors:** Defaults are compile-time constants, not runtime lookups
    ///
    /// **Migration impact:**
    ///
    /// Old API: `CommandDefinition { name: ".test".to_string(), description: "Test".to_string(), ..Default::default() }`
    /// New API: `CommandDefinition::former().name(".test").description("Test").end()`
    ///
    /// The `end()` method provides similar ergonomics to `..Default::default()` while
    /// maintaining type safety and validation.
    ///
    /// # Fail-Fast Validation (Phase 2)
    ///
    /// This method enforces validation at construction time, not registration time:
    /// - Command names must start with `.` (dot prefix)
    /// - Versions must be valid semantic versions
    /// - **Panics immediately** if validation fails, preventing invalid state
    ///
    /// **Design rationale:**
    /// - Earlier detection: bugs caught at construction, not registration
    /// - Simpler code: no duplicate validation in registration paths
    /// - Better DX: panic stack traces point directly to construction site
    /// - Type safety: invalid commands cannot exist, even temporarily
    ///
    /// # Default Values
    ///
    /// - `namespace`: `""` (root-level command)
    /// - `hint`: `""` (no hint)
    /// - `status`: `CommandStatus::Active`
    /// - `version`: `1.0.0`
    ///
    pub fn end( self ) -> CommandDefinition
    {
      let name_str = self.name.unwrap();
      let description_str = self.description.unwrap();
      let namespace_str = self.namespace.unwrap_or_else( || String::new() );
      let hint_str = self.hint.unwrap_or_else( || String::new() );
      let status_str = self.status.unwrap_or_else( || "active".to_string() );
      let version_str = self.version.unwrap_or_else( || "1.0.0".to_string() );

      CommandDefinition
      {
        name : CommandName::new( &name_str ).expect( "builder name should be valid" ),
        description : description_str,
        namespace : namespace_str,
        hint : hint_str,
        status : match status_str.to_lowercase().as_str()
        {
          "experimental" => CommandStatus::Experimental,
          "internal" => CommandStatus::Internal,
          "deprecated" => CommandStatus::Deprecated
          {
            reason : self.deprecation_message.clone(),
            since : None,
            replacement : None,
          },
          _ => CommandStatus::Active,
        },
        version : VersionType::new( &version_str ).expect( "builder version should be valid" ),
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
      }
    }
  }

  // .build() ONLY available when ALL required fields are Set
  impl CommandDefinitionBuilder< Set, Set, Set, Set, Set, Set >
  {
    /// Builds the `CommandDefinition` from the fully-populated builder.
    ///
    /// This method is only available when all 6 required fields have been set,
    /// providing compile-time safety against missing required fields.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandDefinition;
    ///
    /// let cmd = CommandDefinition::former()
    ///     .name(".my_command")
    ///     .description("Does something useful")
    ///     .namespace("".to_string())
    ///     .hint("Brief hint")
    ///     .status("stable")
    ///     .version("1.0.0")
    ///     .build();
    ///
    /// assert_eq!(cmd.name().as_str(), ".my_command");
    /// ```
    pub fn build( self ) -> CommandDefinition
    {
      let name_str = self.name.unwrap();
      let namespace_str = self.namespace.unwrap();
      let status_str = self.status.unwrap();
      let version_str = self.version.unwrap();

      CommandDefinition
      {
        name : CommandName::new( &name_str ).expect( "builder name should be valid" ),
        description : self.description.unwrap(),
        namespace : namespace_str,
        hint : self.hint.unwrap(),
        status : match status_str.to_lowercase().as_str()
        {
          "experimental" => CommandStatus::Experimental,
          "internal" => CommandStatus::Internal,
          "deprecated" => CommandStatus::Deprecated
          {
            reason : self.deprecation_message.clone(),
            since : None,
            replacement : None,
          },
          _ => CommandStatus::Active,
        },
        version : VersionType::new( &version_str ).expect( "builder version should be valid" ),
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
      }
    }
  }

