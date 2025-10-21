//! Materialize commands

use unilang::registry::CommandRegistry;

/// Register materialize commands
pub fn register( _registry : &mut CommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  // TODO: Implement materialize command registration
  Ok( () )
}
