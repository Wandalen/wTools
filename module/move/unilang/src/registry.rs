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
/// while dynamic commands are stored in a HashMap for runtime flexibility.
///
#[ derive( Default ) ] // Removed Clone since CommandRoutine can't be cloned
#[ allow( missing_debug_implementations ) ]
pub struct CommandRegistry
{
  /// A map of dynamically registered command names to their definitions.
  /// Static commands are stored in the STATIC_COMMANDS PHF map.
  dynamic_commands : HashMap< String, CommandDefinition >,
  /// A map of command names to their executable routines.
  routines : HashMap< String, CommandRoutine >,
}

impl CommandRegistry
{
  ///
  /// Creates a new, empty `CommandRegistry`.
  ///
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Retrieves a command definition by name using hybrid lookup.
  /// 
  /// First checks the static PHF map for compile-time commands, then
  /// falls back to the dynamic HashMap for runtime-registered commands.
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
    let full_name = if command_def.name.starts_with( '.' )
    {
      // Command name is already in full format
      command_def.name.clone()
    }
    else if command_def.namespace.is_empty()
    {
      format!( ".{}", command_def.name )
    }
    else
    {
      let ns = &command_def.namespace;
      if ns.starts_with( '.' )
      {
        format!( "{}.{}", ns, command_def.name )
      }
      else
      {
        format!( ".{}.{}", ns, command_def.name )
      }
    };
    // Check if command exists in either static or dynamic registries
    if super::STATIC_COMMANDS.contains_key( &full_name ) || self.dynamic_commands.contains_key( &full_name )
    {
      return Err( Error::Execution( ErrorData
      {
        code : "COMMAND_ALREADY_EXISTS".to_string(),
        message : format!( "Command '{full_name}' already exists." ),
      }));
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
      all_commands.insert( name.to_string(), (*static_cmd).into() );
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
