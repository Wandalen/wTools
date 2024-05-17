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

impl core::ops::DerefMut for IsTransparentSimple
{
  #[ inline( always ) ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
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

impl< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize > core::ops::DerefMut for IsTransparentComplex< 'a, 'b, T, U, N >
where 'a : 'b, T : AsRef< U >
{
  #[ inline( always ) ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_test/deref_mut.rs" );
