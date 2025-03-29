mod private
{
  /// Rust optimization
  #[ derive( Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, derive_tools::Display ) ]
  #[ display( style = "snake_case" ) ]
  pub enum Optimization
  {
    /// Debug
    #[ default ]
    Debug,
    /// Release
    Release,
  }
}

crate::mod_interface!
{
  own use Optimization;
}
