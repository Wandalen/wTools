//!
//! Builder pattern for CommandRegistry construction.
//!

use crate::data::{ CommandDefinition, OutputData, ErrorData };
use crate::error::Error;
use crate::interpreter::ExecutionContext;
use super::CommandRegistry;

///
/// A builder for constructing `CommandRegistry` instances with a fluent API.
///
/// This provides a convenient way to construct a `CommandRegistry` by
/// chaining `command` calls.
#[ allow( missing_debug_implementations ) ]
pub struct CommandRegistryBuilder
{
  registry : CommandRegistry,
  /// Accumulated errors during registration (command_name, error)
  errors : Vec< ( String, Error ) >,
}

impl Default for CommandRegistryBuilder
{
  fn default() -> Self
  {
    Self
    {
      registry : CommandRegistry::default(),
      errors : Vec::new(),
    }
  }
}

impl CommandRegistryBuilder
{
  ///
  /// Creates a new `CommandRegistryBuilder`.
  ///
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Adds a command to the registry being built.
  ///
  /// # Errors
  ///
  /// Returns `Error::Registration` if command validation fails or if duplicate detected.
  pub fn command( mut self, command : CommandDefinition ) -> Result< Self, Error >
  {
    self.registry.register( command )?;
    Ok( self )
  }

  ///
  /// Loads command definitions from a YAML string and adds them to the registry.
  ///
  /// **Requires feature**: `yaml_parser` (enabled by YAML approaches)
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the YAML string is invalid or if routine links cannot be resolved.
  #[ cfg( feature = "yaml_parser" ) ]
  pub fn load_from_yaml_str( mut self, yaml_str : &str ) -> Result< Self, Error >
  {
    let command_defs = crate::loader::load_command_definitions_from_yaml_str( yaml_str )?;
    for command_def in command_defs
    {
      if let Some( link ) = command_def.routine_link()
      {
        let routine = crate::loader::resolve_routine_link( link )?;
        #[ allow( deprecated ) ]
        self.registry.command_add_runtime( &command_def, routine )?;
      }
      else
      {
        self.registry.register( command_def )?;
      }
    }
    Ok( self )
  }

  ///
  /// Loads command definitions from a JSON string and adds them to the registry.
  ///
  /// **Requires feature**: `json_parser` (enabled by JSON approaches)
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the JSON string is invalid or if routine links cannot be resolved.
  #[ cfg( feature = "json_parser" ) ]
  pub fn load_from_json_str( mut self, json_str : &str ) -> Result< Self, Error >
  {
    let command_defs = crate::loader::load_command_definitions_from_json_str( json_str )?;
    for command_def in command_defs
    {
      if let Some( link ) = command_def.routine_link()
      {
        let routine = crate::loader::resolve_routine_link( link )?;
        #[ allow( deprecated ) ]
        self.registry.command_add_runtime( &command_def, routine )?;
      }
      else
      {
        self.registry.register( command_def )?;
      }
    }
    Ok( self )
  }

  ///
  /// Adds a command with inline routine using a fluent builder.
  ///
  /// This provides Row 7 (Rust DSL → Dynamic HashMap) functionality,
  /// allowing commands and routines to be defined together inline.
  ///
  /// # Arguments
  /// * `name` - Command name (must start with '.')
  /// * `description` - Command description
  /// * `routine` - Inline closure for command execution
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::builder()
  ///   .command_with_routine(
  ///     ".greet",
  ///     "Greets user by name",
  ///     |_cmd, _ctx| {
  ///       Ok(unilang::data::OutputData {
  ///         content: "Hello!".to_string(),
  ///         format: "text".to_string(),
  ///         execution_time_ms: None,
  ///       })
  ///     }
  ///   )
  ///   .build();
  /// ```
  #[ must_use ]
  pub fn command_with_routine< F >(
    mut self,
    name : &str,
    description : &str,
    routine : F
  ) -> Self
  where
    F : Fn( crate::semantic::VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static
  {
    use crate::data::{ CommandName, CommandStatus, VersionType };

    let cmd = CommandDefinition::new(
      CommandName::new( name ).expect( "valid command name" ),
      description.to_string(),
    )
    .with_status( CommandStatus::Active )
    .with_version( VersionType::new( "1.0.0" ).expect( "valid version" ) )
    .with_auto_help( true )
    .with_idempotent( true )
    .with_http_method_hint( "GET" );

    // Register with routine - collect errors for later checking
    #[ allow( deprecated ) ]
    if let Err( e ) = self.registry.command_add_runtime( &cmd, Box::new( routine ) )
    {
      self.errors.push( ( name.to_string(), e ) );
    }

    self
  }

  ///
  /// Builds and returns the `CommandRegistry`, ignoring any registration errors.
  ///
  /// **Warning:** This method silently ignores registration errors. Use `build_checked()`
  /// if you need to ensure all commands were registered successfully.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::builder()
  ///   .command_with_routine(".test", "Test command", |_, _| {
  ///     Ok(unilang::data::OutputData {
  ///       content: "Success".to_string(),
  ///       format: "text".to_string(),
  ///       execution_time_ms: None,
  ///     })
  ///   })
  ///   .build();
  /// ```
  #[ must_use ]
  pub fn build( self ) -> CommandRegistry
  {
    self.registry
  }

  ///
  /// Builds and returns the `CommandRegistry`, returning an error if any registration failed.
  ///
  /// This method provides proper error propagation, ensuring that all registration errors
  /// are caught and reported. Use this method instead of `build()` when you need to
  /// guarantee that all commands were successfully registered.
  ///
  /// # Errors
  ///
  /// Returns an error if any command failed to register. The error will contain details
  /// about all failed registrations.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let result = CommandRegistry::builder()
  ///   .command_with_routine(".test", "Test command", |_, _| {
  ///     Ok(unilang::data::OutputData {
  ///       content: "Success".to_string(),
  ///       format: "text".to_string(),
  ///       execution_time_ms: None,
  ///     })
  ///   })
  ///   .build_checked();
  ///
  /// match result {
  ///   Ok(registry) => println!("All commands registered successfully"),
  ///   Err(e) => eprintln!("Registration failed: {}", e),
  /// }
  /// ```
  pub fn build_checked( self ) -> Result< CommandRegistry, Error >
  {
    if self.errors.is_empty()
    {
      Ok( self.registry )
    }
    else
    {
      // Construct detailed error message with all failures
      let mut error_message = String::from( "Command registration failed for the following commands:\n" );

      for ( cmd_name, err ) in &self.errors
      {
        error_message.push_str( &format!( "  - '{}': {}\n", cmd_name, err ) );
      }

      Err( Error::Registration( error_message ) )
    }
  }
}
