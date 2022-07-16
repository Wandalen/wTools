#[ allow( unused_imports ) ]
use super::*;

// trace_macros!( true );
// TheModule::types!
// {
//   #[ derive( Debug, Clone ) ]
//   #[ derive( PartialEq, Default ) ]
//   many Many : < T >;
// }
// trace_macros!( false );

#[ derive( Debug, Clone ) ]
#[ derive( PartialEq, Default ) ]
struct Many< T > ( pub TheModule::_Vec < T > );

impl< T > core::ops::Deref for Many< T >
{
  type Target = TheModule::_Vec < T >;
  #[inline]
  fn deref( &self) -> & Self::Target
  {
    &self.0
  }
}

impl< T > core::ops::DerefMut for Many< T >
{
  #[inline]
  fn deref_mut( &mut self) -> & mut Self::Target
  {
    &mut self.0
  }
}

impl< Collection, T, IntoT >
From< Collection >
for Many< T >
where
  Collection : IntoIterator< Item = IntoT >,
  IntoT : Into< T >,
{
  #[ inline ]
  fn from( src : Collection ) -> Self
  {
    Self( src.into_iter().map( | e | e.into() ).collect::< Vec< T > >() )
  }
}

// impl< T > From < T > for Many< T >
// {
//   #[inline]
//   fn from( src : T ) -> Self
//   {
//     Self( TheModule::_vec![ src ] )
//   }
// }
//
// impl < T > From < & T > for Many< T >
// where T : Clone,
// {
//   #[inline]
//   fn from( src : &T ) -> Self
//   {
//     Self( TheModule::_vec![ src.clone() ] )
//   }
// }
//
// impl< T > From < ( T, ) > for Many< T >
// {
//   #[inline]
//   fn from( src : ( T, ) ) -> Self
//   {
//     Self( TheModule::_vec![ src.0 ] )
//   }
// }
//
// impl < T, const N : usize > From < [T ; N] > for Many< T >
// {
//   #[inline]
//   fn from( src : [ T ; N ] ) -> Self
//   {
//     Self( TheModule::_Vec::from( src ) )
//   }
// }
//
// impl< T > From < &[ T ] > for Many< T > where T : Clone,
// {
//   #[inline]
//   fn from( src : &[ T ] ) -> Self
//   {
//     Self( TheModule::_Vec::from( src ) )
//   }
// }

impl< T > TheModule::AsSlice< T > for Many< T >
{
  #[inline] fn as_slice(& self) -> &[ T ]
  {
    &self[ .. ]
  }
}

TheModule::_if_make!
{

  impl< T > TheModule::Make0 for Many< T >
  {
    #[inline]
    fn make_0() -> Self
    {
      Self( TheModule::_Vec::new() )
    }
  }

  impl< T > TheModule::Make1 < T > for Many< T >
  {
    #[inline]
    fn make_1(_0 : T) -> Self
    {
      Self(TheModule::_vec! [_0])
    }
  }

  impl< T > TheModule::Make2 < T, T > for Many< T >
  {
    #[inline]
    fn make_2(_0 : T, _1 : T) -> Self
    {
      Self( TheModule::_vec![ _0, _1 ] )
    }
  }

  impl< T > TheModule::Make3 < T, T, T > for Many< T >
  {
    #[inline] fn make_3(_0 : T, _1 : T, _2 : T) -> Self
    {
      Self( TheModule::_vec![ _0, _1, _2 ] )
    }
  }

}

include!( "./many_parameter_main_test_only.rs" );
