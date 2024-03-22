#[ allow( unused_imports ) ]
use super::*;

macro_rules! mk
{
  (
    $( $Rest : tt )*
  )
  =>
  {
    mod1::Floats::from( $( $Rest )* )
  };
}

mod mod1
{

  #[ derive( Debug, Clone, PartialEq ) ]
  pub struct Floats< T1 : PartialEq + Copy, T2 : Default >
  (
    pub T1,
    pub T2,
  );

  impl< T1 : PartialEq + Copy, T2 : Default > core::ops::Deref
  for Floats< T1, T2 >
  {
    type Target = T1;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< T1 : PartialEq + Copy, T2 : Default > From< T1 >
  for Floats< T1, T2 >
  {
    fn from( src : T1 ) -> Self
    {
      Floats::< T1, T2 >( src, T2::default() )
    }
  }

}

// trace_macros!( true );
// the_module::types!
// {
//   #[ derive( Debug, Clone ) ]
//   #[ derive( PartialEq ) ]
//   single Single : mod1::Floats< T1 : PartialEq + std::marker::Copy, T2 : Default >;
// }
// trace_macros!( false );

#[ derive( Debug, Clone ) ]
#[ derive( PartialEq ) ]
struct Single< T1 : PartialEq + std::marker::Copy, T2 : Default >
( pub mod1::Floats< T1, T2 > );

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
core::ops::Deref
for Single< T1, T2 >
{
  type Target = mod1::Floats< T1, T2 >;
  #[ inline ]
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
core::ops::DerefMut
for Single< T1, T2 >
{
  #[ inline ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
From< mod1::Floats< T1, T2 > >
for Single< T1, T2 >
{
  #[ inline ]
  fn from( src : mod1::Floats< T1, T2 > ) -> Self
  {
    Self( src )
  }
}

impl< __FromRef, T1 : PartialEq + std::marker::Copy, T2 : Default >
From< &__FromRef >
for Single< T1, T2 >
where
  __FromRef : Clone,
  Self : From< __FromRef >,
{
  #[ inline ]
  fn from( src : &__FromRef ) -> Self
  {
    From::from( (*src).clone() )
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
From< Single< T1, T2 > >
for mod1::Floats< T1, T2 >
{
  #[ inline ]
  fn from( src : Single< T1, T2 > ) -> Self
  {
    src.0
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
From< ( mod1::Floats< T1, T2 >, ) >
for Single< T1, T2 >
{
  #[ inline ]
  fn from( src : ( mod1::Floats< T1, T2 >, ) ) -> Self
  {
    Self( src.0 )
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
From< [ mod1::Floats< T1, T2 > ; 1 ] >
for Single< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn from( src : [ mod1::Floats< T1, T2 > ; 1 ] ) -> Self
  {
    Self( src[ 0 ].clone() )
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
From< &[ mod1::Floats< T1, T2 > ] >
for Single< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn from( src : &[ mod1::Floats< T1, T2 > ] ) -> Self
  {
    debug_assert_eq!( src.len(), 1 );
    Self( src[ 0 ].clone() )
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
the_module::CloneAsTuple < ( mod1::Floats< T1, T2 >, ) >
for Single< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn clone_as_tuple( &self ) -> ( mod1::Floats< T1, T2 >, )
  {
    ( self.0.clone(), )
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
the_module::CloneAsArray < mod1::Floats< T1, T2 >, 1 >
for Single< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn clone_as_array( &self ) -> [ mod1::Floats< T1, T2 > ; 1 ]
  {
    [ self.0.clone() ]
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
the_module::AsTuple< ( mod1::Floats< T1, T2 >, ) >
for Single< T1, T2 >
{
  #[ inline ]
  fn as_tuple( &self ) -> &( mod1::Floats< T1, T2 >, )
  {
    unsafe
    {
      core::mem::transmute::< _, _ >( self )
    }
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
the_module::AsArray< mod1::Floats< T1, T2 >, 1 >
for Single< T1, T2 >
{
  #[ inline ]
  fn as_array( &self ) -> &[ mod1::Floats< T1, T2 > ; 1 ]
  {
    unsafe
    {
      core::mem::transmute::< _, _ >( self )
    }
  }
}

impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
the_module::AsSlice
< mod1::Floats< T1, T2 > >
for Single< T1, T2 >
{
  #[ inline ]
  fn as_slice( &self ) -> &[ mod1::Floats< T1, T2 > ]
  {
    &the_module::AsArray::as_array( self )[ .. ]
  }
}

the_module::_if_from!
{
  impl< T1 : PartialEq + std::marker::Copy, T2 : Default > the_module::From_1< mod1::Floats< T1, T2 > >
  for Single< T1, T2 >
  {
    #[ inline ]
    fn from_1( _0 : mod1::Floats< T1, T2 > ) -> Self
    {
      Self( _0 )
    }
  }
}

include!( "./single_parametrized_main_test_only.rs" );
