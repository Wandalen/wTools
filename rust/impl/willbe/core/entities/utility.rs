/// Internal namespace.
pub( crate ) mod private
{
  /// Ordering strategies
  #[ derive( Debug, Clone, Copy ) ]
  pub enum OrderStrategy
  {
    /// Not ordered
    Random
  }
}

//

wtools::meta::mod_interface!
{
  prelude use OrderStrategy;
}
