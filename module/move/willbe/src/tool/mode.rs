mod private
{
  use std::fmt::Formatter;

  /// Rust mode
  #[ derive( Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd ) ]
  pub enum Mode
  {
    /// Debug
    #[ default ]
    Debug,
    /// Release
    Release,
  }

  impl std::fmt::Display for Mode
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result 
    {
      match self
      {
        Mode::Debug => write!( f, "debug" ),
        Mode::Release => write!( f, "release" ),
      }
    }
  }
}

crate::mod_interface!
{
  protected use Mode;
}