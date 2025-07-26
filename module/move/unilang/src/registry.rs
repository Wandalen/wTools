//!
//! The command registry for the Unilang framework.
//!

use crate::data::{CommandDefinition, ErrorData, OutputData};
use crate::semantic::VerifiedCommand;
use crate::interpreter::ExecutionContext;
use std::collections::HashMap;
use crate::error::Error; // Import Error for Result type

/// Type alias for a command routine.
/// A routine takes a `VerifiedCommand` and an `ExecutionContext`, and returns a `Result` of `OutputData` or `ErrorData`.
pub type CommandRoutine = Box<dyn Fn(VerifiedCommand, ExecutionContext) -> Result<OutputData, ErrorData> + Send + Sync + 'static>;

///
/// A registry for commands, responsible for storing and managing all
/// available command definitions.
///
#[derive(Default)] // Removed Debug
#[allow(missing_debug_implementations)]
pub struct CommandRegistry {
  /// A map of command names to their definitions.
  pub commands: HashMap<String, CommandDefinition>,
  /// A map of command names to their executable routines.
  routines: HashMap<String, CommandRoutine>,
}

impl CommandRegistry {
  ///
  /// Creates a new, empty `CommandRegistry`.
  ///
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  ///
  /// Registers a command, adding it to the registry.
  ///
  /// If a command with the same name already exists, it will be overwritten.
  pub fn register(&mut self, command: CommandDefinition) {
    let full_name = if let Some(ns) = &command.namespace {
      if ns.starts_with('.') {
        format!("{}.{}", ns, command.name)
      } else {
        format!(".{}.{}", ns, command.name)
      }
    } else {
      format!(".{}", command.name)
    };

    self.commands.insert(full_name, command);
  }

  ///
  /// Registers a command with its executable routine at runtime.
  ///
  /// # Errors
  ///
  /// Returns an `Error::Registration` if a command with the same name
  /// is already registered and cannot be overwritten (e.g., if it was
  /// a compile-time registered command).
  pub fn command_add_runtime(&mut self, command_def: &CommandDefinition, routine: CommandRoutine) -> Result<(), Error> {
    let full_name = if let Some(ns) = &command_def.namespace {
      if ns.starts_with('.') {
        format!("{}.{}", ns, command_def.name)
      } else {
        format!(".{}.{}", ns, command_def.name)
      }
    } else {
      format!(".{}", command_def.name)
    };
    if self.commands.contains_key(&full_name) {
      return Err(Error::Execution(ErrorData {
        code: "COMMAND_ALREADY_EXISTS".to_string(),
        message: format!("Command '{full_name}' already exists."),
      }));
    }

    self.commands.insert(full_name.clone(), command_def.clone()); // Cloned command_def
    self.routines.insert(full_name.clone(), routine);
    Ok(())
  }

  ///
  /// Retrieves the routine for a given command name.
  ///
  #[must_use]
  pub fn get_routine(&self, command_name: &str) -> Option<&CommandRoutine> {
    self.routines.get(command_name)
  }

  ///
  /// Returns a builder for creating a `CommandRegistry` with a fluent API.
  ///
  #[must_use]
  pub fn builder() -> CommandRegistryBuilder {
    CommandRegistryBuilder::new()
  }
}

///
/// A builder for the `CommandRegistry`.
///
/// This provides a convenient way to construct a `CommandRegistry` by
/// chaining `command` calls.
#[allow(missing_debug_implementations)]
#[derive(Default)] // Removed Debug
pub struct CommandRegistryBuilder {
  registry: CommandRegistry,
}

impl CommandRegistryBuilder {
  ///
  /// Creates a new `CommandRegistryBuilder`.
  ///
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  ///
  /// Adds a command to the registry being built.
  ///
  #[must_use]
  pub fn command(mut self, command: CommandDefinition) -> Self {
    self.registry.register(command);
    self
  }

  ///
  /// Loads command definitions from a YAML string and adds them to the registry.
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the YAML string is invalid or if routine links cannot be resolved.
  pub fn load_from_yaml_str(mut self, yaml_str: &str) -> Result<Self, Error> {
    let command_defs = crate::loader::load_command_definitions_from_yaml_str(yaml_str)?;
    for command_def in command_defs {
      if let Some(link) = &command_def.routine_link {
        let routine = crate::loader::resolve_routine_link(link)?;
        self.registry.command_add_runtime(&command_def, routine)?;
      } else {
        self.registry.register(command_def);
      }
    }
    Ok(self)
  }

  ///
  /// Loads command definitions from a JSON string and adds them to the registry.
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the JSON string is invalid or if routine links cannot be resolved.
  pub fn load_from_json_str(mut self, json_str: &str) -> Result<Self, Error> {
    let command_defs = crate::loader::load_command_definitions_from_json_str(json_str)?;
    for command_def in command_defs {
      if let Some(link) = &command_def.routine_link {
        let routine = crate::loader::resolve_routine_link(link)?;
        self.registry.command_add_runtime(&command_def, routine)?;
      } else {
        self.registry.register(command_def);
      }
    }
    Ok(self)
  }

  ///
  /// Builds and returns the `CommandRegistry`.
  ///
  #[must_use]
  pub fn build(self) -> CommandRegistry {
    self.registry
  }
}
