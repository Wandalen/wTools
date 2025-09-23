//! # Test Matrix for `DerefMut` on Enums (Compile-Fail)
//!
//! This matrix documents test cases for ensuring the `DerefMut` derive macro correctly
//! rejects enums, as `DerefMut` is only applicable to structs with a single field.
//!
//! | ID   | Item Type | Expected Error Message                                   |
//! |------|-----------|----------------------------------------------------------|
//! | CF1.1 | Enum      | "DerefMut cannot be derived for enums. It is only applicable to structs with a single field." |

extern crate derive_tools_meta;

#[ allow( dead_code ) ]
#[ derive( derive_tools_meta ::DerefMut ) ]
enum MyEnum
{
  Variant1( bool ),
  Variant2( i32 ),
}

fn main() {}
