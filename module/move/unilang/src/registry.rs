//!
//! The command registry for the Unilang framework.
//!

use crate::data::CommandDefinition;
use std::collections::HashMap;

///
/// A registry for commands, responsible for storing and managing all
/// available command definitions.
///
#[ derive( Debug, Default ) ]
pub struct CommandRegistry
{
  /// A map of command names to their definitions.
  pub commands : HashMap< String, CommandDefinition >,
}

impl CommandRegistry
{
  ///
  /// Creates a new, empty `CommandRegistry`.
  ///
  #[must_use]
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Registers a command, adding it to the registry.
  ///
  /// If a command with the same name already exists, it will be overwritten.
  pub fn register( &mut self, command : CommandDefinition )
  {
    self.commands.insert( command.name.clone(), command );
  }

  ///
  /// Returns a builder for creating a `CommandRegistry` with a fluent API.
  ///
  #[must_use]
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
#[ derive( Debug, Default ) ]
pub struct CommandRegistryBuilder
{
  registry : CommandRegistry,
}

impl CommandRegistryBuilder
{
  ///
  /// Creates a new `CommandRegistryBuilder`.
  ///
  #[must_use]
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Adds a command to the registry being built.
  ///
  #[must_use]
  pub fn command( mut self, command : CommandDefinition ) -> Self
  {
    self.registry.register( command );
    self
  }

  ///
  /// Builds and returns the `CommandRegistry`.
  ///
  #[must_use]
  pub fn build( self ) -> CommandRegistry
  {
    self.registry
  }
}