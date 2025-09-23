//! # Test Matrix for `From` Derive
//!
//! This matrix documents test cases for the `From` derive macro.
//!
//! | ID   | Struct Type       | Field Type | Expected Behavior                               |
//! |------|-------------------|------------|-------------------------------------------------|
//! | T1.1 | `IsTransparentSimple(bool)` | `bool`     | Converts from `bool` to `IsTransparentSimple`. |
//! | T1.2 | `IsTransparentComplex` (generics) | `&'a T`    | Converts from `&'a T` to `IsTransparentComplex`. |

use macro_tools ::diag;
use super :: *;
use derive_tools_meta ::From;
use test_tools ::a_id;

#[ derive( Debug, Clone, Copy, PartialEq, From ) ]
#[ allow( dead_code ) ]
pub struct IsTransparentSimple(bool);

#[ derive( Debug, Clone, Copy, PartialEq, From ) ]
#[ allow( dead_code ) ]
pub struct IsTransparentComplex< 'a, 'b: 'a, T, U: ToString + ?Sized >(#[ from ] &'a T, core ::marker ::PhantomData< &'b U >)
where
  'a: 'b,
  T: AsRef< U >;

/// Tests the `From` derive macro for various struct types.
#[ test ]
fn from_test() 
{
  // Test for IsTransparentSimple
  a_id!(IsTransparentSimple ::from(true), IsTransparentSimple(true));

  // Test for IsTransparentComplex
  let _got_tmp = "hello".to_string();
  a_id!(IsTransparentComplex :: < '_, '_, String, str > ::from(&_got_tmp), IsTransparentComplex :: < '_, '_, String, str >(&_got_tmp, core ::marker ::PhantomData));
}
