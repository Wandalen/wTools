mod private
{
  use std::fmt::Formatter;

  /// Rust optimization
  #[ derive( Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd ) ]
  pub enum Optimization
  {
    /// Debug
    #[ default ]
    Debug,
    /// Release
    Release,
  }

  impl std::fmt::Display for Optimization
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result 
    {
      match self
      {
        Optimization::Debug => write!( f, "debug" ),
        Optimization::Release => write!( f, "release" ),
      }
    }
  }
}

crate::mod_interface!
{
  protected use Optimization;
}