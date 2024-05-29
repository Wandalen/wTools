#[ allow( unused_imports ) ]
use super::*;

// trace_macros!( true );
// the_module::types!
// {
// #[ derive( Debug, Clone ) ]
// #[ derive( PartialEq, Default ) ]
// single Single : < T >;
// }
// trace_macros!( false );

#[ derive( Debug, Clone ) ]
#[ derive( PartialEq, Default ) ]

struct Single< T >
( pub T );

impl< T > core::ops::Deref
for Single< T >
{
  type Target = T ;
  #[ inline ]
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< T > core::ops::DerefMut
for Single< T >
{
  #[ inline ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

impl< T > From < T >
for Single< T >
{
  #[ inline ]
  fn from( src : T ) -> Self
  {
    Self( src )
  }
}

// impl< T > Into< T >
// for Single< T >
// {
//   fn into( self ) -> T
//   {
//     self.0
//   }
// }

// impl< T > From < Single< T > >
// for T
// {
//   #[ inline ]
//   fn from( src : Single< T > ) -> Self
//   {
//     src.0
//   }
// }

impl< T > From < &T >
for Single< T >
where T : Clone,
{
  #[ inline ]
  fn from( src : &T ) -> Self
  {
    Self( src.clone() )
  }
}

impl< T > From< ( T, ) >
for Single< T >
{
  #[ inline ]
  fn from( src : ( T, ) ) -> Self
  {
    Self( src.0 )
  }
}

impl< T > From < Single< T > >
for( T, )
{
  #[ inline ]
  fn from( src : Single< T > ) -> Self
  {
    ( src.0, )
  }
}

impl< T > From< [ T ; 1 ] >
for Single< T >
where T : Clone,
{
  #[ inline ]
  fn from( src : [T ; 1] ) -> Self
  {
    Self( src[ 0 ].clone() )
  }
}

impl< T > From< Single< T > >
for [T ; 1]
{
  #[ inline ]
  fn from( src : Single< T > ) -> Self
  {
    [ src.0 ]
  }
}

impl< T > From< &[ T ] >
for Single< T >
where T : Clone,
{
  #[ inline ]
  fn from( src : &[ T ] ) -> Self
  {
    debug_assert_eq!( src.len(), 1 );
    Self( src[ 0 ].clone() )
  }
}

impl< T > the_module::CloneAsTuple < (T,) >
for Single< T >
where T : Clone,
{
  #[ inline ]
  fn clone_as_tuple( &self ) -> ( T, )
  {
    ( self.0.clone(), )
  }
}

impl< T > the_module::CloneAsArray< T, 1 >
for Single< T >
where T : Clone,
{
  #[ inline ]
  fn clone_as_array( &self ) -> [ T ; 1 ]
  {
    [ self.0.clone() ; 1 ]
  }
}

impl< T > the_module::AsTuple< ( T, ) >
for Single< T >
{
  #[ inline ]
  fn as_tuple( &self ) -> &( T, )
  {
    unsafe
    {
      core::mem::transmute::< _, _ >( self )
    }
  }
}

impl< T > the_module::AsArray< T, 1 >
for Single< T >
{
  #[ inline ]
  fn as_array( &self ) -> &[ T ; 1 ]
  {
    unsafe
    {
      core::mem::transmute::< _, _ >( self )
    }
  }
}

impl< T > the_module::AsSlice < T >
for Single< T >
{
  #[ inline ]
  fn as_slice( &self ) -> &[ T ]
  {
    &the_module::AsArray::as_array( self )[..]
  }
}

the_module::_if_from!
{

//   impl< T > the_module::From_0
//   for Single< T >
//   where T : Default
//   {
//     #[ inline ]
//     fn from_0() -> Self
//     {
//       Self( Default::default() )
//     }
//   }
//
//   impl< T > the_module::From_1< T >
//   for Single< T >
//   {
//     #[ inline ]
//     fn from_1( _0 : T ) -> Self
//     {
//       Self( _0 )
//     }
//   }

}

include!( "./single_parameter_main_test_only.rs" );
