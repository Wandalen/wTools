//! Minimal registry file for testing

// Include the generated static commands PHF map
include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));

/// Internal namespace.
mod private
{
  pub struct CommandRegistry;

  impl CommandRegistry
  {
    pub fn new() -> Self
    {
      Self
    }
  }
}

mod_interface::mod_interface!
{
  exposed use private::CommandRegistry;
}