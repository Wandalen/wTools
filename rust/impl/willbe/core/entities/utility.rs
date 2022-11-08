/// Internal namespace.
pub( crate ) mod private
{
  /// Ordering strategies
  #[ derive( Debug, Clone, Copy ) ]
  pub enum OrderStrategy
  {
    /// Command specific default value
    Default,
    /// Not ordered
    Random
  }
}

//

wtools::meta::mod_interface!
{
  prelude use OrderStrategy;
}
