//! # Test Matrix for `DerefMut` Manual Implementation
//!
//! This matrix documents test cases for the manual `DerefMut` implementation.
//!
//! | ID   | Struct Type       | Field Type | Expected Behavior                               |
//! |------|-------------------|------------|-------------------------------------------------|
//! | T1.1 | `IsTransparentSimple(bool)` | `bool`     | Derefs to `bool` and allows mutable access.     |
//! | T1.2 | `IsTransparentComplex` (generics) | `&'a T`    | Derefs to `&'a T` and allows mutable access.    |

use super::*;
use test_tools::a_id;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsTransparentSimple(bool);

impl core::ops::Deref for IsTransparentSimple {
  type Target = bool;
  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl core::ops::DerefMut for IsTransparentSimple {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

// #[ derive( Debug, Clone, Copy, PartialEq ) ]
// pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a mut T, core::marker::PhantomData< &'b U > )
// where
//   'a : 'b,
//   T : AsRef< U >;

// impl< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize > core::ops::Deref for IsTransparentComplex< 'a, 'b, T, U, N >
// where
//   'a : 'b,
//   T : AsRef< U >
// {
//   type Target = &'a mut T;
//   #[ inline( always ) ]
//   fn deref( &self ) -> &Self::Target
//   {
//     &self.0
//   }
// }

// impl< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize > core::ops::DerefMut for IsTransparentComplex< 'a, 'b, T, U, N >
// where
//   'a : 'b,
//   T : AsRef< U >
// {
//   #[ inline( always ) ]
//   fn deref_mut( &mut self ) -> &mut Self::Target
//   {
//     &mut self.0
//   }
// }

/// Tests the `DerefMut` manual implementation for various struct types.
#[test]
fn deref_mut_test() {
  // Test for IsTransparentSimple
  let mut got = IsTransparentSimple(true);
  let exp = true;
  a_id!(*got, exp);
  *got = false;
  a_id!(*got, false);

  // Test for IsTransparentComplex (commented out due to const generics issue)
  // let mut got_tmp = "hello".to_string();
  // let mut got = IsTransparentComplex::< '_, '_, String, str, 0 >( &mut got_tmp, core::marker::PhantomData );
  // let exp = &mut got_tmp;
  // a_id!( *got, exp );
  // **got = "world".to_string();
  // a_id!( *got, &"world".to_string() );
}
