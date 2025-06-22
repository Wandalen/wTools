//!
//! The command registry for the Unilang framework.
//!

use crate::data::CommandDefinition;
use std::collections::HashMap;

///
/// A registry for commands.
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
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Registers a command.
  ///
  pub fn register( &mut self, command : CommandDefinition )
  {
    self.commands.insert( command.name.clone(), command );
  }

  ///
  /// Returns a builder for the `CommandRegistry`.
  ///
  pub fn builder() -> CommandRegistryBuilder
  {
    CommandRegistryBuilder::new()
  }
}

///
/// A builder for the `CommandRegistry`.
///
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
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Adds a command to the registry.
  ///
  pub fn command( mut self, command : CommandDefinition ) -> Self
  {
    self.registry.register( command );
    self
  }

  ///
  /// Builds the `CommandRegistry`.
  ///
  pub fn build( self ) -> CommandRegistry
  {
    self.registry
  }
}