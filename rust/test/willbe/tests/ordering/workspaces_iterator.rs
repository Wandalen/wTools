use super::*;

#[ test ]
fn alphabetical()
{
  let mut source = 
  [
    "a", "c", // First workspace
    "b", "d"  // Second workspace
  ];
  let expected = [ "a", "b", "c", "d" ];

  source.sort(); //* Will be removed

  assert_eq!
  (
    expected,
    source // Call "workspaces_packages_iterate( source.iter(), OrderStrategy::Alphabetical )"
  )
}
