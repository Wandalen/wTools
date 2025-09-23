// Define a private namespace where all items are initially defined.
mod private 
{
  /// This item should only be accessible within the `child` module itself.
  /// It will be placed in the `own` exposure level.
  #[ must_use ] pub fn my_thing() -> bool {
  true
 }
  /// This item should be accessible in the `child` module and its immediate parent.
  /// It will be placed in the `orphan` exposure level.
  #[ must_use ] pub fn orphan_thing() -> bool {
  true
 }
  /// This item should be accessible throughout the module hierarchy (ancestors).
  /// It will be placed in the `exposed` exposure level.
  #[ must_use ] pub fn exposed_thing() -> bool {
  true
 }
  /// This item should be accessible everywhere and intended for glob imports.
  /// It will be placed in the `prelude` exposure level.
  #[ must_use ] pub fn prelude_thing() -> bool {
  true
 }
}

// Use `mod_interface!` to re-export items from `private`
// into the appropriate public exposure levels.
crate ::mod_interface! {
  // `my_thing` goes into the `own` level (not propagated).
  own use my_thing;
  // `orphan_thing` goes into the `orphan` level (propagates to immediate parent).
  orphan use orphan_thing;
  // `exposed_thing` goes into the `exposed` level (propagates to all ancestors).
  exposed use exposed_thing;
  // `prelude_thing` goes into the `prelude` level (propagates like exposed, intended for glob).
  prelude use prelude_thing;
}
