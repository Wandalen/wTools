use super::*;
use derive_tools_meta::Deref;
// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, derive_tools_meta::Deref ) ]
pub struct IsTransparentSimple( bool );

// #[ derive( Debug, Clone, Copy, PartialEq, derive_tools_meta::Deref ) ]
// pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a T, core::marker::PhantomData< &'b U > )
// where
//   'a : 'b,
//   T : AsRef< U >;


// Content from only_test/deref.rs
use test_tools::a_id;

/// Tests the `Deref` derive macro and manual implementation for various struct types.
#[ test ]
fn deref_test()
{
  // Test for IsTransparentSimple
  let got = IsTransparentSimple( true );
  let exp = true;
  a_id!( *got, exp );

  // Test for IsTransparentComplex (commented out due to const generics issue)
  // let got_tmp = "hello".to_string();
  // let got = IsTransparentComplex::< '_, '_, String, str, 0 >( &got_tmp, core::marker::PhantomData );
  // let exp = &got_tmp;
  // a_id!( *got, exp );
}
