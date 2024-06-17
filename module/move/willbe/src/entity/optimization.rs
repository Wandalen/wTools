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

  // qqq : use derive
  // aaa : done
}
// aaa : for Petro : why is it here?
// aaa : as we discuss it`s already in place

crate::mod_interface!
{
  protected use Optimization;
}
