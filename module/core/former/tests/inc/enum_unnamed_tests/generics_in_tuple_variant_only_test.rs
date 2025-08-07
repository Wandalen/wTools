// Purpose: Tests generic tuple variant functionality
// This file is included by generics_in_tuple_variant derive/manual files

use super::*; // Should import EnumOuter and InnerGeneric from either the manual or derive file

#[ test ]
fn basic_construction()
{
  // Define a concrete type that satisfies the bounds (Debug + Copy + Default + PartialEq)
  #[derive(Debug, Copy, Clone, Default, PartialEq)]
  struct TypeForT {
    pub data: i32,
  }

  // This should work if the enum correctly handles generics
  let got = EnumOuter::<TypeForT>::variant()
    .inner_field(TypeForT { data: 42 })
    .form();

  let expected = EnumOuter::Variant(InnerGeneric { inner_field: TypeForT { data: 42 } });
  assert_eq!(got, expected);
}