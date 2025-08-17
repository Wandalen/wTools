use super::*;

#[ test ]
fn test_needs_drop() {
  struct NeedsDrop;

  impl Drop for NeedsDrop {
    fn drop(&mut self) {}
  }

  assert!(core::mem::needs_drop::<NeedsDrop>());

  // Test each of the types with a handwritten TrivialDrop impl above.
  assert!(!core::mem::needs_drop::<core::iter::Empty<NeedsDrop>>());
  assert!(!core::mem::needs_drop::<core::slice::Iter<'_, NeedsDrop>>());
  assert!(!core::mem::needs_drop::<core::slice::IterMut<'_, NeedsDrop>>());
  assert!(!core::mem::needs_drop::<core::option::IntoIter<&NeedsDrop>>());
  assert!(!core::mem::needs_drop::<core::option::IntoIter<&mut NeedsDrop>>());
}
