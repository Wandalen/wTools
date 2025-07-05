use core::ops::Deref;
use derive_tools::Deref;
use core::marker::PhantomData;

#[ allow( dead_code ) ]
#[ derive( Debug, Clone, Copy, PartialEq, Deref ) ]
pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a T, PhantomData< &'b U > )
where
  'a : 'b,
  T : AsRef< U >;

include!( "./only_test/compile_fail_complex_struct.rs" );