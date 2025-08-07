mod private
{
  /// Routine of child module.
  #[must_use] pub fn inner_is() -> bool
  {
    true
  }
}

//

mod_interface::mod_interface!
{
  prelude use inner_is;
}
