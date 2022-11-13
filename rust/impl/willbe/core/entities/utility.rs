/// Internal namespace.
pub( crate ) mod private
{
  /// Represent with which ordering strategy to iterate over packages
  #[ derive( Debug, Clone, Copy ) ]
  pub enum OrderStrategy
  {
    /// Alphabetical by package name
    Alphabetical,
    /// Based on their dependencies
    Topological,
    /// Shuffle packages
    Random,
  }
}

//

wtools::meta::mod_interface!
{
  prelude use OrderStrategy;
}
