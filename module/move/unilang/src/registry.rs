//!
//! The command registry for the Unilang framework.
//!

// Include the generated static commands PHF map
include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));

/// Internal namespace.
mod private
{
  use crate::data::{ CommandDefinition, ErrorData, OutputData };
  use crate::error::Error; // Import Error for Result type
  use crate::interpreter::ExecutionContext;
  use std::collections::HashMap;

/// Type alias for a command routine.
/// A routine takes a `VerifiedCommand` and an `ExecutionContext`, and returns a `Result` of `OutputData` or `ErrorData`.
pub type CommandRoutine = Box< dyn Fn( crate::semantic::VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static >;

///
/// A registry for commands, responsible for storing and managing all
/// available command definitions.
/// 
/// Uses a hybrid model: static commands are stored in a PHF map for zero overhead,
/// while dynamic commands are stored in a `HashMap` for runtime flexibility.
///
#[ allow( missing_debug_implementations ) ]
pub struct CommandRegistry
{
  /// A map of dynamically registered command names to their definitions.
  /// Static commands are stored in the `STATIC_COMMANDS` PHF map.
  dynamic_commands : HashMap< String, CommandDefinition >,
  /// A map of command names to their executable routines.
  routines : HashMap< String, CommandRoutine >,
  /// Whether automatic help command generation is enabled for new registrations.
  help_conventions_enabled : bool,
}

impl CommandRegistry
{
  ///
  /// Creates a new, empty `CommandRegistry`.
  ///
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      dynamic_commands : HashMap::new(),
      routines : HashMap::new(),
      help_conventions_enabled : true, // Enable by default for better UX
    }
  }

  ///
  /// Retrieves a command definition by name using hybrid lookup.
  /// 
  /// First checks the static PHF map for compile-time commands, then
  /// falls back to the dynamic `HashMap` for runtime-registered commands.
  ///
  #[ must_use ]
  pub fn command( &self, name : &str ) -> Option< CommandDefinition >
  {
    // First check static commands (PHF map)
    if let Some( static_cmd ) = super::STATIC_COMMANDS.get( name )
    {
      return Some( (*static_cmd).into() );
    }

    // Fall back to dynamic commands
    self.dynamic_commands.get( name ).cloned()
  }

  ///
  /// Registers a command, adding it to the dynamic registry.
  ///
  /// If a command with the same name already exists, it will be overwritten.
  /// Note: Static commands cannot be overwritten and will take precedence in lookups.
  pub fn register( &mut self, command : CommandDefinition )
  {
    let full_name = if command.name.starts_with( '.' )
    {
      // Command name is already in full format
      command.name.clone()
    }
    else if command.namespace.is_empty()
    {
      format!( ".{}", command.name )
    }
    else
    {
      let ns = &command.namespace;
      if ns.starts_with( '.' )
      {
        format!( "{}.{}", ns, command.name )
      }
      else
      {
        format!( ".{}.{}", ns, command.name )
      }
    };

    self.dynamic_commands.insert( full_name, command );
  }

  ///
  /// Registers a command with its executable routine at runtime.
  ///
  /// # Errors
  ///
  /// Returns an `Error::Registration` if a command with the same name
  /// is already registered and cannot be overwritten (e.g., if it was
  /// a compile-time registered command).
  pub fn command_add_runtime( &mut self, command_def : &CommandDefinition, routine : CommandRoutine ) -> Result< (), Error >
  {
    // EXPLICIT COMMAND NAMING ENFORCEMENT (FR-REG-6)
    // Following the governing principle: minimum implicit magic!
    
    // Validate that command names start with dot prefix
    if !command_def.name.starts_with( '.' )
    {
      return Err( Error::Registration( format!(
        "Invalid command name '{}'. All commands must start with dot prefix (e.g., '.chat'). \
        This enforces explicit naming with minimal implicit transformations.",
        command_def.name
      )));
    }
    
    // Validate namespace format if provided
    if !command_def.namespace.is_empty() && !command_def.namespace.starts_with( '.' )
    {
      return Err( Error::Registration( format!(
        "Invalid namespace '{}'. Non-empty namespaces must start with dot prefix (e.g., '.session'). \
        Use empty namespace for root-level commands.",
        command_def.namespace
      )));
    }
    
    // Build full command name explicitly - no magic transformations
    let full_name = if command_def.namespace.is_empty()
    {
      // Root-level command: use name as-is (already validated to have dot prefix)
      command_def.name.clone()
    }
    else
    {
      // Namespaced command: explicit concatenation
      format!( "{}.{}", command_def.namespace, command_def.name.strip_prefix('.').unwrap_or(&command_def.name) )
    };
    // Check if command exists in either static or dynamic registries
    if super::STATIC_COMMANDS.contains_key( &full_name ) || self.dynamic_commands.contains_key( &full_name )
    {
      return Err( Error::Execution( ErrorData::new(
        "UNILANG_COMMAND_ALREADY_EXISTS".to_string(),
        format!( "Registration Error: Command '{full_name}' already exists. Use a different name or remove the existing command first." ),
      )));
    }

    self.dynamic_commands.insert( full_name.clone(), command_def.clone() ); // Cloned command_def
    self.routines.insert( full_name.clone(), routine );
    Ok(())
  }

  ///
  /// Retrieves the routine for a given command name.
  ///
  #[ must_use ]
  pub fn get_routine( &self, command_name : &str ) -> Option< &CommandRoutine >
  {
    self.routines.get( command_name )
  }

  ///
  /// Returns a collection of all command definitions (both static and dynamic).
  /// 
  /// This is provided for backward compatibility and introspection.
  /// Static commands are converted from the PHF map.
  ///
  #[ must_use ]
  pub fn commands( &self ) -> HashMap< String, CommandDefinition >
  {
    let mut all_commands = HashMap::new();

    // Add static commands
    for ( name, static_cmd ) in super::STATIC_COMMANDS.entries()
    {
      all_commands.insert( (*name).to_string(), (*static_cmd).into() );
    }

    // Add dynamic commands (they can override static ones in this view)
    for ( name, cmd ) in &self.dynamic_commands
    {
      all_commands.insert( name.clone(), cmd.clone() );
    }

    all_commands
  }

  ///
  /// Returns a builder for creating a `CommandRegistry` with a fluent API.
  ///
  #[ must_use ]
  pub fn builder() -> CommandRegistryBuilder
  {
    CommandRegistryBuilder::new()
  }

  ///
  /// Enables/disables automatic `.command.help` generation for all subsequently registered commands.
  ///
  /// When enabled, all commands registered with `command_add_runtime` or `register_with_auto_help`
  /// will automatically generate corresponding `.command.help` commands that provide detailed
  /// help information about the parent command.
  ///
  /// # Arguments
  /// * `enabled` - Whether to enable automatic help command generation
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang::registry::CommandRegistry;
  ///
  /// let mut registry = CommandRegistry::new();
  /// registry.enable_help_conventions(true);
  /// // All subsequently registered commands will auto-generate help commands
  /// ```
  pub fn enable_help_conventions( &mut self, enabled : bool )
  {
    self.help_conventions_enabled = enabled;
  }

  ///
  /// Registers a command with automatic help command generation.
  ///
  /// This method provides explicit control over help generation, registering the main command
  /// and optionally generating a `.command.help` counterpart based on the command's configuration
  /// and the registry's global help conventions setting.
  ///
  /// # Arguments
  /// * `command` - The command definition to register
  /// * `routine` - The executable routine for the command
  ///
  /// # Returns
  /// * `Result<(), Error>` - Success or registration error
  ///
  /// # Errors
  /// Returns an error if command registration fails due to invalid naming or other validation issues.
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang::{registry::CommandRegistry, data::CommandDefinition};
  ///
  /// let mut registry = CommandRegistry::new();
  /// let cmd = CommandDefinition::former()
  ///     .name(".example".to_string())
  ///     .description("Example command".to_string())
  ///     .with_auto_help(true)
  ///     .end();
  ///
  /// let routine = Box::new(|_cmd, _ctx| Ok(OutputData::default()));
  /// registry.register_with_auto_help(cmd, routine)?;
  /// // Both ".example" and ".example.help" are now registered
  /// ```
  pub fn register_with_auto_help( &mut self, command : CommandDefinition, routine : CommandRoutine ) -> Result< (), Error >
  {
    // First register the main command
    self.command_add_runtime( &command, routine )?;

    // Generate help command if enabled (either globally or specifically for this command)
    if self.help_conventions_enabled || command.has_auto_help()
    {
      let help_command = command.generate_help_command();
      let help_routine = self.create_help_routine( &command );
      self.command_add_runtime( &help_command, help_routine )?;
    }

    Ok( () )
  }

  ///
  /// Retrieves formatted help text for any registered command.
  ///
  /// This method generates comprehensive help information for a given command,
  /// including its description, arguments, usage examples, and metadata.
  /// It works with both static and dynamic commands.
  ///
  /// # Arguments
  /// * `command_name` - The full name of the command (e.g., ".example" or ".fs.list")
  ///
  /// # Returns
  /// * `Option<String>` - Formatted help text, or None if command not found
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::new();
  /// if let Some(help_text) = registry.get_help_for_command(".example") {
  ///     println!("{}", help_text);
  /// }
  /// ```
  #[ must_use ]
  pub fn get_help_for_command( &self, command_name : &str ) -> Option< String >
  {
    if let Some( cmd_def ) = self.command( command_name )
    {
      Some( self.format_help_text( &cmd_def ) )
    }
    else
    {
      None
    }
  }

  ///
  /// Creates a help routine for a given command.
  ///
  /// This internal method generates the executable routine that will be used
  /// for `.command.help` commands. The routine returns formatted help information
  /// about the parent command.
  ///
  /// # Arguments
  /// * `parent_command` - The command for which to create a help routine
  ///
  /// # Returns
  /// * `CommandRoutine` - An executable routine that returns help information
  fn create_help_routine( &self, parent_command : &CommandDefinition ) -> CommandRoutine
  {
    let help_text = self.format_help_text( parent_command );

    Box::new( move | _cmd, _ctx |
    {
      Ok( OutputData
      {
        content : help_text.clone(),
        format : "text".to_string(),
      })
    })
  }

  ///
  /// Formats comprehensive help text for a command definition.
  ///
  /// This internal method generates detailed, human-readable help information
  /// including command description, arguments with types and defaults,
  /// usage examples, and metadata.
  ///
  /// # Arguments
  /// * `cmd_def` - The command definition to format help for
  ///
  /// # Returns
  /// * `String` - Formatted help text
  fn format_help_text( &self, cmd_def : &CommandDefinition ) -> String
  {
    let mut help = String::new();

    // Command header
    help.push_str( &format!( "Command: {}\n", cmd_def.name ) );
    help.push_str( &format!( "Description: {}\n", cmd_def.description ) );

    if !cmd_def.hint.is_empty()
    {
      help.push_str( &format!( "Hint: {}\n", cmd_def.hint ) );
    }

    help.push_str( &format!( "Version: {}\n", cmd_def.version ) );
    help.push_str( &format!( "Status: {}\n", cmd_def.status ) );

    // Arguments section
    if !cmd_def.arguments.is_empty()
    {
      help.push_str( "\nArguments:\n" );
      for arg in &cmd_def.arguments
      {
        let required = if arg.attributes.optional { "optional" } else { "required" };
        help.push_str( &format!( "  {} ({}, {})", arg.name, arg.kind, required ) );

        if let Some( default ) = &arg.attributes.default
        {
          help.push_str( &format!( " [default: {}]", default ) );
        }

        help.push_str( &format!( "\n    {}\n", arg.description ) );

        if !arg.aliases.is_empty()
        {
          help.push_str( &format!( "    Aliases: {}\n", arg.aliases.join( ", " ) ) );
        }
      }
    }

    // Examples section
    if !cmd_def.examples.is_empty()
    {
      help.push_str( "\nExamples:\n" );
      for example in &cmd_def.examples
      {
        help.push_str( &format!( "  {}\n", example ) );
      }
    }

    // Aliases section
    if !cmd_def.aliases.is_empty()
    {
      help.push_str( &format!( "\nAliases: {}\n", cmd_def.aliases.join( ", " ) ) );
    }

    // Usage patterns
    help.push_str( "\nUsage:\n" );
    help.push_str( &format!( "  {}  # Execute command\n", cmd_def.name ) );
    help.push_str( &format!( "  {}.help  # Show this help\n", cmd_def.name ) );
    help.push_str( &format!( "  {} ??  # Alternative help access\n", cmd_def.name ) );

    help
  }
}

impl Default for CommandRegistry
{
  fn default() -> Self
  {
    Self::new()
  }
}

///
/// A builder for the `CommandRegistry`.
///
/// This provides a convenient way to construct a `CommandRegistry` by
/// chaining `command` calls.
#[ allow( missing_debug_implementations ) ]
#[ derive( Default ) ] // Removed Debug
pub struct CommandRegistryBuilder
{
  registry : CommandRegistry,
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
  #[ must_use ]
  pub fn command( mut self, command : CommandDefinition ) -> Self
  {
    self.registry.register( command );
    self
  }

  ///
  /// Loads command definitions from a YAML string and adds them to the registry.
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the YAML string is invalid or if routine links cannot be resolved.
  pub fn load_from_yaml_str( mut self, yaml_str : &str ) -> Result< Self, Error >
  {
    let command_defs = crate::loader::load_command_definitions_from_yaml_str( yaml_str )?;
    for command_def in command_defs
    {
      if let Some( link ) = &command_def.routine_link
      {
        let routine = crate::loader::resolve_routine_link( link )?;
        self.registry.command_add_runtime( &command_def, routine )?;
      }
      else
      {
        self.registry.register( command_def );
      }
    }
    Ok( self )
  }

  ///
  /// Loads command definitions from a JSON string and adds them to the registry.
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the JSON string is invalid or if routine links cannot be resolved.
  pub fn load_from_json_str( mut self, json_str : &str ) -> Result< Self, Error >
  {
    let command_defs = crate::loader::load_command_definitions_from_json_str( json_str )?;
    for command_def in command_defs
    {
      if let Some( link ) = &command_def.routine_link
      {
        let routine = crate::loader::resolve_routine_link( link )?;
        self.registry.command_add_runtime( &command_def, routine )?;
      }
      else
      {
        self.registry.register( command_def );
      }
    }
    Ok( self )
  }

  ///
  /// Builds and returns the `CommandRegistry`.
  ///
  #[ must_use ]
  pub fn build( self ) -> CommandRegistry
  {
    self.registry
  }
}

}

mod_interface::mod_interface!
{
  exposed use private::CommandRoutine;
  exposed use private::CommandRegistry;
  exposed use private::CommandRegistryBuilder;
  
  prelude use private::CommandRoutine;
  prelude use private::CommandRegistry;
  prelude use private::CommandRegistryBuilder;
}
