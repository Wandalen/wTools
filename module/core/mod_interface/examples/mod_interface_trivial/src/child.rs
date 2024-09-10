
// Define a private namespace for all its items.
mod private
{
  /// Only my thing.
  pub fn my_thing() -> bool { true }
  /// Parent module should also has this thing.
  pub fn orphan_thing() -> bool { true }
  /// This thing should be exposed.
  pub fn exposed_thing() -> bool { true }
  /// This thing should be in prelude.
  pub fn prelude_thing() -> bool { true }
}

//

crate::mod_interface!
{
  own use my_thing;
  orphan use orphan_thing;
  exposed use exposed_thing;
  prelude use prelude_thing;
}
