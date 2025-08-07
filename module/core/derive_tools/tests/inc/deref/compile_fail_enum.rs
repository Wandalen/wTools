extern crate derive_tools_meta;
// # Test Matrix for `Deref` on Enums (Compile-Fail)
//
// This matrix documents test cases for ensuring the `Deref` derive macro correctly
// rejects enums, as `Deref` is only applicable to structs with a single field.
//
// | ID   | Item Type | Expected Error Message                                   |
// |------|-----------|----------------------------------------------------------|
// | CF1.1 | Enum      | "Deref cannot be derived for enums. It is only applicable to structs with a single field." |

#[ allow( dead_code ) ]
#[ derive( derive_tools_meta::Deref ) ]
enum MyEnum
{
  Variant1( bool ),
  Variant2( i32 ),
}

fn main() {}