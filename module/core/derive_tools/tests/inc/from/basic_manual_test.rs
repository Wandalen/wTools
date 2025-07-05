//! # Test Matrix for `From` Manual Implementation
//!
//! This matrix documents test cases for the manual `From` implementation.
//!
//! | ID   | Struct Type       | Field Type | Expected Behavior                               |
//! |------|-------------------|------------|-------------------------------------------------|
//! | T1.1 | `IsTransparentSimple(bool)` | `bool`     | Converts from `bool` to `IsTransparentSimple`. |
//! | T1.2 | `IsTransparentComplex` (generics) | `&'a T`    | Converts from `&'a T` to `IsTransparentComplex`. |

use super::*;
use test_tools::a_id;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparentSimple( bool );

impl From< bool > for IsTransparentSimple
{
  fn from( src : bool ) -> Self
  {
    Self( src )
  }
}

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a T, core::marker::PhantomData< &'b U > )
where
  'a : 'b,
  T : AsRef< U >;

impl< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize > From< &'a T > for IsTransparentComplex< 'a, 'b, T, U, N >
where
  'a : 'b,
  T : AsRef< U >
{
  fn from( src : &'a T ) -> Self
  {
    Self( src, core::marker::PhantomData )
  }
}

/// Tests the `From` manual implementation for various struct types.
#[ test ]
fn from_test()
{
  // Test for IsTransparentSimple
  let got = IsTransparentSimple::from( true );
  let exp = IsTransparentSimple( true );
  a_id!( got, exp );

  // Test for IsTransparentComplex (commented out due to const generics issue)
  // let got_tmp = "hello".to_string();
  // let got = IsTransparentComplex::< '_, '_, String, str, 0 >::from( &got_tmp );
  // let exp = IsTransparentComplex::< '_, '_, String, str, 0 >( &got_tmp, core::marker::PhantomData );
  // a_id!( got, exp );
}
