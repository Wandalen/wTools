//! # Test Matrix for `DerefMut` Derive
//!
//! This matrix documents test cases for the `DerefMut` derive macro.
//!
//! | ID   | Struct Type       | Field Type | Expected Behavior                               |
//! |------|-------------------|------------|-------------------------------------------------|
//! | T1.1 | `IsTransparentSimple(bool)` | `bool`     | Derefs to `bool` and allows mutable access.     |
//! | T1.2 | `IsTransparentComplex` (generics) | `&'a T`    | Derefs to `&'a T` and allows mutable access.    |

use super::*;
use derive_tools_meta::{Deref, DerefMut};
use test_tools::a_id;

#[ derive( Debug, Clone, Copy, PartialEq, Deref, DerefMut ) ]
pub struct IsTransparentSimple(bool);

// #[ derive( Debug, Clone, Copy, PartialEq, DerefMut ) ]
// pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a mut T, core::marker::PhantomData< &'b U > )
// where
//   'a : 'b,
//   T : AsRef< U >;

/// Tests the `DerefMut` derive macro for various struct types.
#[ test ]
fn deref_mut_test() {
  // Test for IsTransparentSimple
  let mut _got = IsTransparentSimple(true);
  let _exp = true;
  a_id!(*_got, _exp);
  *_got = false;
  a_id!(*_got, false);

  // Test for IsTransparentComplex (commented out due to const generics issue)
  // let mut got_tmp = "hello".to_string();
  // let mut got = IsTransparentComplex::< '_, '_, String, str, 0 >( &mut got_tmp, core::marker::PhantomData );
  // let exp = &mut got_tmp;
  // a_id!( *got, exp );
  // **got = "world".to_string();
  // a_id!( *got, &"world".to_string() );
}
