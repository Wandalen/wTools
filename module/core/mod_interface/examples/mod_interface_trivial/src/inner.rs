pub( crate ) mod private
{
  /// Routine of inner module.
  pub fn inner_is() -> bool
  {
    true
  }
}

//

mod_interface::mod_interface!
{
  prelude use inner_is;
}
