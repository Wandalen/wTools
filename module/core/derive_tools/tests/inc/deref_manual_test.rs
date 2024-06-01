use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, ) ]
pub struct IsTransparentSimple( bool );

impl core::ops::Deref for IsTransparentSimple
{
  type Target = bool;
  #[ inline ( always) ]
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a T, core::marker::PhantomData< &'b U > )
where 'a : 'b, T : AsRef< U >;

impl< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize > core::ops::Deref for IsTransparentComplex< 'a, 'b, T, U, N >
where 'a : 'b, T : AsRef< U >
{
  type Target = &'a T;
  #[ inline( always ) ]
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_test/deref.rs" );
