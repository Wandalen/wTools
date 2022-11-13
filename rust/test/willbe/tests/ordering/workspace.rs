use super::*;

#[ test ]
fn alphabetical()
{
  let mut source =
  [
    "b",  // first member
    "a",  // second member
  ];
  let expected = [ "a", "b" ];

  source.sort(); //* Will be removed

  assert_eq!
  (
    expected,
    source // Call "source.iterate( OrderStrategy::Alphabetical )"
  );
}
