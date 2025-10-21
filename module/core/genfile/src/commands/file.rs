//! File commands

use unilang::registry::CommandRegistry;

/// Register file commands
pub fn register( _registry : &mut CommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  // TODO: Implement file command registration
  Ok( () )
}
