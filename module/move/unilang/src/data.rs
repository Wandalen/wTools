//!
//! Core data structures for the Unilang framework.
//!

/// Internal namespace.
mod private
{
  use crate::error::Error;
  // Removed strs_tools dependencies - using standard Rust string operations

  // use former::Former;

  ///
  /// Helper function to construct a full command name from namespace and name components.
  ///
  /// This function implements the canonical algorithm for combining namespace and name
  /// into a fully qualified command name that always starts with a dot prefix.
  ///
  /// # Arguments
  /// * `namespace` - The command's namespace (may be empty or dot-prefixed)
  /// * `name` - The command's name (may already include dot prefix)
  ///
  /// # Returns
  /// * `String` - The fully qualified command name with dot prefix
  ///
  /// # Algorithm
  /// 1. If name already starts with '.':
  ///    - If namespace is empty OR name contains '.', return name as-is (already full format)
  ///    - Otherwise, strip '.' from name and concatenate with namespace
  /// 2. If name doesn't start with '.':
  ///    - If namespace is empty, prepend '.' to name
  ///    - If namespace exists, concatenate with proper dot handling
  pub fn construct_full_command_name( namespace : &str, name : &str ) -> String
  {
    if name.starts_with( '.' )
    {
      // Name already has dot prefix
      if namespace.is_empty() || name.contains( ".." ) || name[ 1.. ].contains( '.' )
      {
        // Name is already in full format (e.g., ".integration.test")
        // OR has multiple dots (e.g., ".a.b") indicating it's already complete
        name.to_string()
      }
      else
      {
        // Name has dot but is just the command part (e.g., ".test")
        // Need to prepend namespace
        let name_without_dot = &name[ 1.. ];
        if namespace.starts_with( '.' )
        {
          format!( "{}.{}", namespace, name_without_dot )
        }
        else
        {
          format!( ".{}.{}", namespace, name_without_dot )
        }
      }
    }
    else if namespace.is_empty()
    {
      // No namespace, no dot: add dot prefix
      format!( ".{}", name )
    }
    else
    {
      // Has namespace, name has no dot: concatenate
      if namespace.starts_with( '.' )
      {
        format!( "{}.{}", namespace, name )
      }
      else
      {
        format!( ".{}.{}", namespace, name )
      }
    }
  }

  ///
  /// Defines a command, including its name, arguments, and other metadata.
  ///
  /// This struct is the central piece of a command's definition, providing all
  /// the necessary information for parsing, validation, and execution.
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize, former::Former, Default ) ]
  pub struct CommandDefinition
  {
    /// The name of the command, used to invoke it from the command line.
    pub name : String,
    /// A brief, one-line description of what the command does.
    pub description : String,
    /// A list of arguments that the command accepts.
    // #[ former( default ) ]
    pub arguments : Vec< ArgumentDefinition >,
    /// An optional link to the routine that executes this command.
    pub routine_link : Option< String >,
    /// The namespace of the command.
    pub namespace : String, // Changed from Option<String> to String
    /// A short hint for the command.
    pub hint : String,
    /// The status of the command.
    pub status : String,
    /// The version of the command.
    pub version : String,
    /// Tags associated with the command.
    pub tags : Vec< String >,
    /// Aliases for the command.
    pub aliases : Vec< String >,
    /// Permissions required to execute the command.
    pub permissions : Vec< String >,
    /// Indicates if the command is idempotent.
    pub idempotent : bool,
    /// If `status` is `Deprecated`, explains the reason and suggests alternatives.
    pub deprecation_message : String, // Added
    /// A suggested HTTP method (`GET`, `POST`, etc.) for the Web API modality.
    pub http_method_hint : String, // Added
    /// Illustrative usage examples for help text.
    pub examples : Vec< String >, // Added
    /// Whether this command should automatically generate a `.command.help` counterpart.
    pub auto_help_enabled : bool, // Help Convention Support
  }

  ///
  /// Holds attributes and configuration for a specific argument within a command.
  ///
  /// This struct enables fine-grained control over how arguments behave,
  /// such as whether they are required, accept multiple values, or have
  /// default values.
  #[allow(clippy::struct_excessive_bools)]
  #[ derive( Debug, Clone, Default, serde::Serialize, serde::Deserialize ) ]
  pub struct ArgumentAttributes
  {
    /// Indicates if the argument is optional.
    /// If true, the argument can be omitted without causing validation errors.
    pub optional : bool,
    /// Indicates if the argument can accept multiple values.
    /// If true, the argument can be provided multiple times in a single command invocation.
    pub multiple : bool,
    /// The default value for the argument if not provided.
    /// Only applicable when the argument is optional.
    pub default : Option< String >,
    /// Indicates if the argument contains sensitive data (e.g., passwords).
    /// If true, the argument might be masked or logged differently.
    pub sensitive : bool,
    /// Indicates if the argument might require user interaction (e.g., prompts).
    /// If true, the system may need to handle interactive input.
    ///
    /// # REPL Implementation Notes
    /// 
    /// **Critical Behavior**: When `interactive: true` and the argument is required but not provided:
    /// - Semantic analysis returns `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error
    /// - REPL loops should catch this error and prompt for secure input
    /// - **Important**: Optional interactive arguments with defaults do NOT trigger the error
    /// 
    /// **Security Best Practices**:
    /// - Always combine with `sensitive: true` for passwords/API keys
    /// - Never log or store interactive argument values
    /// - Use secure input methods (masked input) in REPL implementations
    /// 
    /// **Common Pitfalls**:
    /// - ❌ Don't handle interactive prompts during command execution
    /// - ❌ Don't store interactive values in command history  
    /// - ✅ Handle interactive prompts at the REPL level before re-execution
    /// - ✅ Clear sensitive values from memory after use
    pub interactive : bool,
  }

  ///
  /// Defines an argument within a command, including its name, type, and constraints.
  ///
  /// This struct provides all the necessary information to parse, validate,
  /// and process a single argument within a command.
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize, former::Former ) ]
  pub struct ArgumentDefinition
  {
    /// The name of the argument, used to reference it in commands and validation.
    pub name : String,
    /// The data type and structure expected for this argument.
    pub kind : Kind,
    /// Attributes that control the behavior of this argument.
    pub attributes : ArgumentAttributes,
    /// A brief, one-line hint about the argument's purpose.
    pub hint : String,
    /// A more detailed description of the argument.
    pub description : String,
    /// Validation rules that apply to this argument.
    pub validation_rules : Vec< ValidationRule >,
    /// Alternative names for this argument.
    pub aliases : Vec< String >,
    /// Tags associated with this argument.
    pub tags : Vec< String >,
  }

  ///
  /// Represents the data type and structure of an argument or value.
  ///
  /// The `Kind` enum defines all supported data types and their validation rules,
  /// enabling robust type checking and conversion throughout the system.
  #[ derive( Debug, Clone, PartialEq, Eq, serde::Serialize ) ]
  #[ serde( untagged ) ]
  pub enum Kind
  {
    /// A simple text string.
    String,
    /// An integer number (positive, negative, or zero).
    Integer,
    /// A floating-point number.
    Float,
    /// A boolean value (true or false).
    Boolean,
    /// A file system path (file or directory).
    Path,
    /// A file system path that must point to an existing file.
    File,
    /// A file system path that must point to an existing directory.
    Directory,
    /// An enumeration with a predefined set of allowed values.
    Enum( Vec< String > ),
    /// A URL (web address).
    Url,
    /// A date and time value.
    DateTime,
    /// A regular expression pattern.
    Pattern,
    /// A list (array) of values of the same type.
    /// The optional character specifies the delimiter used to separate list items.
    List( Box< Kind >, Option< char > ),
    /// A map (dictionary) of key-value pairs.
    /// The optional characters specify the entry delimiter and key-value delimiter.
    Map( Box< Kind >, Box< Kind >, Option< char >, Option< char > ),
    /// A JSON string that can be parsed into complex data structures.
    JsonString,
    /// A generic object that can hold any structured data.
    Object,
  }

  /// Validation rule for argument values.
  #[ derive( Debug, Clone, PartialEq, serde::Serialize ) ]
  pub enum ValidationRule
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
    Pattern( String ),
    /// Minimum number of items for collection types.
    MinItems( usize ),
  }

  impl core::str::FromStr for Kind
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      match s.trim()
      {
        "String" => Ok( Kind::String ),
        "Integer" => Ok( Kind::Integer ),
        "Float" => Ok( Kind::Float ),
        "Boolean" => Ok( Kind::Boolean ),
        "Path" => Ok( Kind::Path ),
        "File" => Ok( Kind::File ),
        "Directory" => Ok( Kind::Directory ),
        "Url" => Ok( Kind::Url ),
        "DateTime" => Ok( Kind::DateTime ),
        "Pattern" => Ok( Kind::Pattern ),
        "JsonString" => Ok( Kind::JsonString ),
        "Object" => Ok( Kind::Object ),
        s if s.starts_with( "Enum(" ) && s.ends_with( ')' ) =>
        {
          let inner = s.strip_prefix( "Enum(" ).unwrap().strip_suffix( ')' ).unwrap();
          if inner.is_empty()
          {
            return Err( Error::Registration( "Empty enum choices".to_string() ) );
          }
          // Use standard Rust string splitting for enum choices
          let choices : Vec< String > = inner
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
          Ok( Kind::Enum( choices ) )
        },
        s if s.starts_with( "List(" ) && s.ends_with( ')' ) =>
        {
          let inner = s.strip_prefix( "List(" ).unwrap().strip_suffix( ')' ).unwrap();
          // Use standard Rust string splitting for list parsing
          let parts : Vec< String > = inner
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
          if parts.is_empty()
          {
            return Err( Error::Registration( "List requires item type".to_string() ) );
          }
          let item_kind = parts[ 0 ].trim().parse::<Kind>()?;
          let delimiter = if parts.len() > 1 && !parts[ 1 ].trim().is_empty()
          {
            Some( parts[ 1 ].trim().chars().next().unwrap() )
          }
          else
          {
            None
          };
          Ok( Kind::List( Box::new( item_kind ), delimiter ) )
        },
        s if s.starts_with( "Map(" ) && s.ends_with( ')' ) =>
        {
          let inner = s.strip_prefix( "Map(" ).unwrap().strip_suffix( ')' ).unwrap();
          // Use standard Rust string splitting for map parsing
          let parts : Vec< String > = inner
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
          if parts.len() < 2
          {
            return Err( Error::Registration( "Map requires key and value types".to_string() ) );
          }
          let key_kind = parts[ 0 ].trim().parse::<Kind>()?;
          let value_kind = parts[ 1 ].trim().parse::<Kind>()?;
          let entry_delimiter = if parts.len() > 2 && !parts[ 2 ].trim().is_empty()
          {
            Some( parts[ 2 ].trim().chars().next().unwrap() )
          }
          else
          {
            None
          };
          let kv_delimiter = if parts.len() > 3 && !parts[ 3 ].trim().is_empty()
          {
            Some( parts[ 3 ].trim().chars().next().unwrap() )
          }
          else
          {
            None
          };
          Ok( Kind::Map( Box::new( key_kind ), Box::new( value_kind ), entry_delimiter, kv_delimiter ) )
        },
        _ => Err( Error::Registration( format!( "Unknown kind: {s}" ) ) ),
      }
    }
  }

  /// 
  /// Represents a namespace within the command system.
  ///
  /// Namespaces provide hierarchical organization for commands, allowing
  /// related commands to be grouped together (e.g., `math.add`, `math.subtract`).
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize ) ]
  pub struct Namespace
  {
    /// The name of the namespace.
    pub name : String,
    /// Commands that belong to this namespace.
    pub commands : Vec< CommandDefinition >,
  }

  ///
  /// Represents the output of a successfully executed command.
  ///
  /// This struct provides a standardized way to return data from command execution,
  /// including both the actual content and metadata about its format.
  #[ derive( Debug, Clone /*, Former*/ ) ]
  pub struct OutputData
  {
    /// The actual content produced by the command.
    pub content : String,
    /// The format of the content (e.g., "`text`", "`json`", "`xml`").
    pub format : String,
  }

  ///
  /// Represents an error that occurred during command execution.
  ///
  /// This struct provides a standardized way to report errors, including a
  /// unique, machine-readable code and a human-readable message.
  #[ derive( Debug, Clone /*, Former*/ ) ]
  pub struct ErrorData
  {
    /// A unique, machine-readable code for the error (e.g., "`COMMAND_NOT_FOUND`").
    pub code : String,
    /// A human-readable message explaining the error.
    pub message : String,
    /// Optional source error for error chaining.
    pub source : Option< Box< ErrorData > >,
  }

  impl core::fmt::Display for ErrorData
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      writeln!( f, "{}", self.message )?;
      
      // Display error chain if present
      if let Some( source ) = &self.source
      {
        Self::fmt_error_chain( f, source, 1 )?;
      }
      
      Ok(())
    }
  }

  impl ErrorData
  {
    ///
    /// Creates a new `ErrorData` with no source error.
    ///
    #[ must_use ]
    pub fn new( code : String, message : String ) -> Self
    {
      Self { code, message, source : None }
    }

    ///
    /// Creates a new `ErrorData` with a source error for chaining.
    ///
    #[ must_use ]
    pub fn with_source( code : String, message : String, source : ErrorData ) -> Self
    {
      Self { code, message, source : Some( Box::new( source ) ) }
    }

    ///
    /// Formats the error chain recursively with proper indentation.
    ///
    fn fmt_error_chain( f : &mut core::fmt::Formatter< '_ >, error : &ErrorData, depth : usize ) -> core::fmt::Result
    {
      // Create indentation
      let indent = "  ".repeat( depth );
      writeln!( f, "{}↳ {}", indent, error.message )?;
      
      // Recursively display deeper sources
      if let Some( source ) = &error.source
      {
        Self::fmt_error_chain( f, source, depth + 1 )?;
      }
      
      Ok(())
    }
  }

  impl core::fmt::Display for Kind
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      let s : String = self.clone().into();
      write!( f, "{s}" )
    }
  }

  impl From< Kind > for String
  {
    fn from( kind : Kind ) -> Self
    {
      match kind
      {
        Kind::String => "String".to_string(),
        Kind::Integer => "Integer".to_string(),
        Kind::Float => "Float".to_string(),
        Kind::Boolean => "Boolean".to_string(),
        Kind::Path => "Path".to_string(),
        Kind::File => "File".to_string(),
        Kind::Directory => "Directory".to_string(),
        Kind::Enum( choices ) => format!( "Enum({})", choices.join( "," ) ),
        Kind::Url => "Url".to_string(),
        Kind::DateTime => "DateTime".to_string(),
        Kind::Pattern => "Pattern".to_string(),
        Kind::List( item_kind, delimiter ) =>
        {
          let item_kind_str : String = ( *item_kind ).into();
          if let Some( d ) = delimiter
          {
            format!( "List({item_kind_str},{d})" )
          }
          else
          {
            format!( "List({item_kind_str})" )
          }
        },
        Kind::Map( key_kind, value_kind, entry_delimiter, kv_delimiter ) =>
        {
          let key_kind_str : String = ( *key_kind ).into();
          let value_kind_str : String = ( *value_kind ).into();
          let mut s = format!( "Map({key_kind_str},{value_kind_str})" );
          if let Some( ed ) = entry_delimiter
          {
            s.push( ',' );
            s.push( ed );
          }
          if let Some( kvd ) = kv_delimiter
          {
            s.push( ',' );
            s.push( kvd );
          }
          s
        },
        Kind::JsonString => "JsonString".to_string(),
        Kind::Object => "Object".to_string(),
      }
    }
  }

  impl core::convert::TryFrom< String > for Kind
  {
    type Error = crate::error::Error;

    fn try_from( s : String ) -> Result< Self, Self::Error >
    {
      s.parse()
    }
  }

  impl< 'de > serde::Deserialize< 'de > for Kind
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      s.parse().map_err( serde::de::Error::custom )
    }
  }

  impl core::str::FromStr for ValidationRule
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let s = s.trim();
      if s.starts_with( "min:" )
      {
        let value_str = s.strip_prefix( "min:" ).unwrap();
        let value : f64 = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid min value: {e}" ) ) )?;
        Ok( ValidationRule::Min( value ) )
      }
      else if s.starts_with( "max:" )
      {
        let value_str = s.strip_prefix( "max:" ).unwrap();
        let value : f64 = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid max value: {e}" ) ) )?;
        Ok( ValidationRule::Max( value ) )
      }
      else if s.starts_with( "minlength:" )
      {
        let value_str = s.strip_prefix( "minlength:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid minlength value: {e}" ) ) )?;
        Ok( ValidationRule::MinLength( value ) )
      }
      else if s.starts_with( "maxlength:" )
      {
        let value_str = s.strip_prefix( "maxlength:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid maxlength value: {e}" ) ) )?;
        Ok( ValidationRule::MaxLength( value ) )
      }
      else if s.starts_with( "pattern:" )
      {
        let pattern = s.strip_prefix( "pattern:" ).unwrap();
        Ok( ValidationRule::Pattern( pattern.to_string() ) )
      }
      else if s.starts_with( "minitems:" )
      {
        let value_str = s.strip_prefix( "minitems:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid minitems value: {e}" ) ) )?;
        Ok( ValidationRule::MinItems( value ) )
      }
      else
      {
        Err( Error::Registration( format!( "Unknown validation rule: {s}" ) ) )
      }
    }
  }

  impl< 'de > serde::Deserialize< 'de > for ValidationRule
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      s.parse().map_err( serde::de::Error::custom )
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
        auto_help_enabled : false,
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
    /// let builder = CommandDefinition::builder()
    ///     .name("my_command");
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

    /// Sets whether auto-help is enabled (optional field, defaults to false).
    pub fn auto_help_enabled( mut self, auto_help_enabled : bool ) -> Self
    {
      self.auto_help_enabled = auto_help_enabled;
      self
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
    /// let cmd = CommandDefinition::builder()
    ///     .name("my_command")
    ///     .description("Does something useful")
    ///     .namespace("")
    ///     .hint("Brief hint")
    ///     .status("stable")
    ///     .version("1.0.0")
    ///     .build();
    ///
    /// assert_eq!(cmd.name, "my_command");
    /// ```
    pub fn build( self ) -> CommandDefinition
    {
      CommandDefinition
      {
        name : self.name.unwrap(),
        description : self.description.unwrap(),
        namespace : self.namespace.unwrap(),
        hint : self.hint.unwrap(),
        status : self.status.unwrap(),
        version : self.version.unwrap(),
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
      }
    }
  }

  impl CommandDefinition
  {
    /// Create a new type-state builder for constructing a `CommandDefinition`.
    ///
    /// This builder enforces that all 6 required fields (name, description, namespace,
    /// hint, status, version) are set before building, providing compile-time safety.
    pub fn builder() -> CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
    {
      CommandDefinitionBuilder::new()
    }

    ///
    /// Builder method to enable/disable automatic help command generation for this specific command.
    ///
    /// This method follows the fluent builder pattern to configure help conventions.
    /// When enabled, registering this command will automatically create a `.command.help`
    /// counterpart that provides detailed help information.
    ///
    /// # Arguments
    /// * `enabled` - Whether to automatically generate help commands
    ///
    /// # Returns
    /// * `Self` - The modified `CommandDefinition` for method chaining
    ///
    /// # Examples
    /// ```rust,ignore
    /// use unilang::data::CommandDefinition;
    ///
    /// let cmd = CommandDefinition::builder()
    ///     .name("example".to_string())
    ///     .description("An example command".to_string())
    ///     .with_auto_help(true)  // Enable automatic help generation
    ///     .build();
    /// ```
    #[ must_use ]
    pub fn with_auto_help( mut self, enabled : bool ) -> Self
    {
      self.auto_help_enabled = enabled;
      self
    }

    ///
    /// Returns true if this command should automatically generate a help counterpart.
    ///
    /// This method checks whether the command is configured to automatically
    /// generate `.command.help` commands during registration.
    ///
    /// # Returns
    /// * `bool` - Whether auto-help generation is enabled for this command
    ///
    /// # Examples
    /// ```rust,ignore
    /// use unilang::data::CommandDefinition;
    ///
    /// let cmd = CommandDefinition::builder()
    ///     .with_auto_help(true)
    ///     .build();
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
    /// # Note on Dot Prefix Requirement
    /// While this method normalizes names without dots (for YAML manifest compatibility),
    /// the API validation (`validate_command_name`) REQUIRES all commands to start with `.`
    /// and will reject registrations that don't. Only YAML/JSON manifests use names without
    /// dots, as the build script adds them automatically.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use unilang::data::CommandDefinition;
    ///
    /// // Correctly formatted command (dot-prefixed name)
    /// let cmd1 = CommandDefinition { name: ".help".to_string(), namespace: "".to_string(), ..Default::default() };
    /// assert_eq!(cmd1.full_name(), ".help");
    ///
    /// // YAML manifest format (normalization for build script compatibility only)
    /// // NOTE: Direct API registration requires dot prefix and will reject "help" without validation bypass
    /// let cmd2 = CommandDefinition { name: "help".to_string(), namespace: "".to_string(), ..Default::default() };
    /// assert_eq!(cmd2.full_name(), ".help"); // Internal normalization
    ///
    /// // Namespaced command (both should have dot prefix for API usage)
    /// let cmd3 = CommandDefinition { name: ".list".to_string(), namespace: ".session".to_string(), ..Default::default() };
    /// assert_eq!(cmd3.full_name(), ".session.list");
    ///
    /// // YAML manifest format (no dots - build script adds them)
    /// let cmd4 = CommandDefinition { name: "list".to_string(), namespace: "session".to_string(), ..Default::default() };
    /// assert_eq!(cmd4.full_name(), ".session.list"); // Internal normalization
    /// ```
    #[ must_use ]
    pub fn full_name( &self ) -> String
    {
      construct_full_command_name( &self.namespace, &self.name )
    }

    ///
    /// Generates a corresponding help command definition for this command.
    ///
    /// Creates a new `CommandDefinition` for the `.command.help` counterpart
    /// that provides detailed help information about the parent command.
    /// The help command includes comprehensive information about arguments,
    /// usage examples, and command metadata.
    ///
    /// # Returns
    /// * `CommandDefinition` - A new command definition for the help counterpart
    ///
    /// # Examples
    /// ```rust,ignore
    /// use unilang::data::CommandDefinition;
    ///
    /// let cmd = CommandDefinition::builder()
    ///     .name("example".to_string())
    ///     .description("An example command".to_string())
    ///     .build();
    ///
    /// let help_cmd = cmd.generate_help_command();
    /// assert_eq!(help_cmd.name, "example.help");
    /// ```
    #[ must_use ]
    pub fn generate_help_command( &self ) -> CommandDefinition
    {
      CommandDefinition
      {
        name : format!( "{}.help", self.name ),
        namespace : self.namespace.clone(),
        description : format!( "Display help information for the '{}' command", self.name ),
        hint : format!( "Help for {}", self.name ),
        status : "stable".to_string(),
        version : self.version.clone(),
        arguments : vec![], // Help commands typically take no arguments
        routine_link : None, // Will be set during registration
        tags : vec![ "help".to_string(), "documentation".to_string() ],
        aliases : vec![ format!( "{}.h", self.name ) ], // Add short alias
        permissions : vec![], // Help commands should be accessible to all
        idempotent : true, // Help commands are always idempotent
        deprecation_message : String::new(),
        http_method_hint : "GET".to_string(), // Help is read-only
        examples : vec![
          format!( "{}.help", self.name ),
          format!( "{} ??", self.name )
        ],
        auto_help_enabled : false, // Prevent recursive help generation
      }
    }
  }
}

mod_interface::mod_interface!
{
  exposed use private::CommandDefinition;
  exposed use private::ArgumentDefinition;
  exposed use private::ArgumentAttributes;
  exposed use private::Kind;
  exposed use private::ValidationRule;
  exposed use private::Namespace;
  exposed use private::OutputData;
  exposed use private::ErrorData;
  exposed use private::CommandDefinitionBuilder;
  exposed use private::Set;
  exposed use private::NotSet;

  prelude use private::CommandDefinition;
  prelude use private::ArgumentDefinition;
  prelude use private::ArgumentAttributes;
  prelude use private::Kind;
  prelude use private::OutputData;
  prelude use private::ErrorData;
}