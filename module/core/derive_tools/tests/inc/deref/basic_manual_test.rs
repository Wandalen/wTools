use super::*;
// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparentSimple(bool);

impl core::ops::Deref for IsTransparentSimple {
  type Target = bool;
  #[ inline( always ) ]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[ derive( Debug, Clone, Copy, PartialEq ) ]
#[ allow( dead_code ) ]
pub struct IsTransparentComplex<'a, 'b: 'a, T, U: ToString + ?Sized, const N: usize>(&'a T, core::marker::PhantomData<&'b U>)
where
  'a: 'b,
  T: AsRef<U>;

impl<'a, 'b: 'a, T, U: ToString + ?Sized, const N: usize> core::ops::Deref for IsTransparentComplex<'a, 'b, T, U, N>
where
  'a: 'b,
  T: AsRef<U>,
{
  type Target = &'a T;
  #[ inline( always ) ]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// Content from only_test/deref.rs
use test_tools::a_id;

/// Tests the `Deref` derive macro and manual implementation for various struct types.
#[ test ]
fn deref_test() {
  // Test for IsTransparentSimple
  let got = IsTransparentSimple(true);
  let exp = true;
  a_id!(*got, exp);

  // Test for IsTransparentComplex
  let got_tmp = "hello".to_string();
  let got = IsTransparentComplex::<'_, '_, String, str, 0>(&got_tmp, core::marker::PhantomData);
  let exp = &got_tmp;
  a_id!(*got, exp);
}
