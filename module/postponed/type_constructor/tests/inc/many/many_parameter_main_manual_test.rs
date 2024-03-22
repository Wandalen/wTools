#[ allow( unused_imports ) ]
use super::*;

// trace_macros!( true );
// the_module::types!
// {
//   #[ derive( Debug, Clone ) ]
//   #[ derive( PartialEq, Default ) ]
//   many Many : < T >;
// }
// trace_macros!( false );

#[ derive( Debug, Clone ) ]
#[ derive( PartialEq, Default ) ]
struct Many< T > ( pub the_module::_Vec < T > );

impl< T > core::ops::Deref for Many< T >
{
  type Target = the_module::_Vec < T >;
  #[ inline ]
  fn deref( &self) -> & Self::Target
  {
    &self.0
  }
}

impl< T > core::ops::DerefMut for Many< T >
{
  #[ inline ]
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
//   #[ inline ]
//   fn from( src : T ) -> Self
//   {
//     Self( the_module::_vec![ src ] )
//   }
// }
//
// impl < T > From < & T > for Many< T >
// where T : Clone,
// {
//   #[ inline ]
//   fn from( src : &T ) -> Self
//   {
//     Self( the_module::_vec![ src.clone() ] )
//   }
// }
//
// impl< T > From < ( T, ) > for Many< T >
// {
//   #[ inline ]
//   fn from( src : ( T, ) ) -> Self
//   {
//     Self( the_module::_vec![ src.0 ] )
//   }
// }
//
// impl < T, const N : usize > From < [T ; N] > for Many< T >
// {
//   #[ inline ]
//   fn from( src : [ T ; N ] ) -> Self
//   {
//     Self( the_module::_Vec::from( src ) )
//   }
// }
//
// impl< T > From < &[ T ] > for Many< T > where T : Clone,
// {
//   #[ inline ]
//   fn from( src : &[ T ] ) -> Self
//   {
//     Self( the_module::_Vec::from( src ) )
//   }
// }

impl< T > the_module::AsSlice< T > for Many< T >
{
  #[ inline ] fn as_slice(& self) -> &[ T ]
  {
    &self[ .. ]
  }
}

the_module::_if_from!
{

  // impl< T > the_module::From_0 for Many< T >
  // {
  //   #[ inline ]
  //   fn from_0() -> Self
  //   {
  //     Self( the_module::_Vec::new() )
  //   }
  // }

  impl< T > the_module::From_1 < T > for Many< T >
  {
    #[ inline ]
    fn from_1(_0 : T) -> Self
    {
      Self(the_module::_vec! [_0])
    }
  }

  impl< T > the_module::From_2 < T, T > for Many< T >
  {
    #[ inline ]
    fn from_2(_0 : T, _1 : T) -> Self
    {
      Self( the_module::_vec![ _0, _1 ] )
    }
  }

  impl< T > the_module::From_3 < T, T, T > for Many< T >
  {
    #[ inline ] fn from_3(_0 : T, _1 : T, _2 : T) -> Self
    {
      Self( the_module::_vec![ _0, _1, _2 ] )
    }
  }

}

include!( "./many_parameter_main_test_only.rs" );
