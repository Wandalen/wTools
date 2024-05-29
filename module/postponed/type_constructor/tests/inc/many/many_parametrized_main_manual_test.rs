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
  pub struct Floats< T1 : PartialEq, T2 : Default >
  (
    pub T1,
    pub T2,
  );

  impl< T1 : PartialEq, T2 : Default > core::ops::Deref
  for Floats< T1, T2 >
  {
    type Target = T1;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< T1 : PartialEq, T2 : Default > From< T1 >
  for Floats< T1, T2 >
  {
    fn from( src : T1 ) -> Self
    {
      Floats::< T1, T2 >( src, T2::default() )
    }
  }

}

//

// trace_macros!( true );
// the_module::types!
// {
//   #[  derive( Debug, Clone )  ]
//   #[  derive( PartialEq )  ]
//   many Many : mod1::Floats< T1 : PartialEq, T2 : Default >;
// }
// trace_macros!( false );

//

#[ derive( Debug, Clone ) ]
#[ derive( PartialEq ) ]
struct Many< T1 : PartialEq, T2 : Default >
( pub the_module::_Vec< mod1::Floats < T1, T2 > > );

impl< T1 : PartialEq, T2 : Default > core::ops::Deref
for Many < T1, T2 >
{
  type Target = the_module::_Vec < mod1::Floats < T1, T2 > >;
  #[ inline ]
  fn deref( & self ) -> & Self::Target
  {
    &self.0
  }
}

impl < T1 : PartialEq, T2 : Default > core::ops::DerefMut
for Many < T1, T2 >
{
  #[ inline ]
  fn deref_mut( & mut self ) -> & mut Self::Target
  {
    &mut self.0
  }
}

impl< Collection, Item, T1 : PartialEq, T2 : Default >
From< Collection >
for Many< T1, T2 >
where
  Collection : IntoIterator< Item = Item >,
  Item : Into< mod1::Floats< T1, T2 > >,
{
  #[ inline ]
  fn from( src : Collection ) -> Self
  {
    let src2 = src
    .into_iter()
    .map( | e | e.into() )
    .collect::< the_module::_Vec< mod1::Floats< T1, T2 > > >();
    Self( src2 )
  }
}

// impl
// < 'a, Collection, T1 : PartialEq + 'a, T2 : Default + 'a >
// From< Collection >
// for Many
// < T1, T2 >
// where
//   Collection : IntoIterator< Item = &'a mod1::Floats< T1, T2 > >,
// {
//   #[ inline ]
//   fn from( src : Collection ) -> Self
//   {
//     let src2 = src
//     .into_iter()
//     .map( | e | *e )
//     .collect::< the_module::_Vec< mod1::Floats< T1, T2 > > >();
//     Self( src2 )
//   }
// }

impl < T1 : PartialEq, T2 : Default >
From < mod1::Floats < T1, T2 > >
for Many < T1, T2 >
{
  #[ inline ]
  fn from( src : mod1::Floats < T1, T2 > ) -> Self
  {
    Self( the_module::_vec! [ src ] )
  }
}

// yyy
// impl < __FromRef, T1 : PartialEq, T2 : Default >
// From < & __FromRef >
// for Many < T1, T2 >
// where
//   __FromRef : Clone, Self : From < __FromRef >,
// {
//   #[ inline ]
//   fn from( src : & __FromRef ) -> Self
//   {
//     From::from( ( * src ).clone() )
//   }
// }

// impl < T1 : PartialEq, T2 : Default >
// From < ( mod1::Floats < T1, T2 >, ) >
// for Many < T1, T2 >
// {
//   #[ inline ]
//   fn from( src : ( mod1::Floats < T1, T2 >, ) ) -> Self
//   {
//     Self( the_module::_vec![  src.0  ] )
//   }
// }

// impl < T1 : PartialEq, T2 : Default, const N : usize >
// From < [ mod1::Floats < T1, T2 > ; N ] >
// for Many < T1, T2 >
// {
//   #[ inline ] fn from( src : [ mod1::Floats < T1, T2 > ; N ] ) -> Self
//   {
//     Self( the_module::_Vec::from( src ) )
//   }
// }

// impl < T1 : PartialEq, T2 : Default >
// From < &[ mod1::Floats < T1, T2 > ] >
// for Many < T1, T2 >
// where
//   mod1::Floats < T1, T2 > : Clone,
// {
//   #[ inline ]
//   fn from( src : & [ mod1::Floats < T1, T2 > ] ) -> Self
//   {
//     Self( the_module::_Vec::from( src ) )
//   }
// }
// yyy

impl < T1 : PartialEq, T2 : Default >
the_module::AsSlice
< mod1::Floats < T1, T2 > >
for Many < T1, T2 >
{
  #[ inline ]
  fn as_slice( & self ) -> &[ mod1::Floats < T1, T2 > ]
  {
    &self [ .. ]
  }
}

the_module::_if_from!
{

  impl < T1 : PartialEq, T2 : Default > the_module::From_0
  for Many < T1, T2 >
  {
    #[ inline ] fn from_0() -> Self
    {
      Self( the_module::_Vec::< mod1::Floats < T1, T2 > >::new() )
    }
  }

  impl < T1 : PartialEq, T2 : Default >
  the_module::From_1 < mod1::Floats < T1, T2 > >
  for Many < T1, T2 >
  {
    #[ inline ]
    fn from_1( _0 : mod1::Floats < T1, T2 >, ) -> Self
    {
      Self( the_module::_vec! [ _0 ] )
    }
  }

  impl < T1 : PartialEq, T2 : Default >
  the_module::From_2 < mod1::Floats < T1, T2 >, mod1::Floats < T1, T2 >, >
  for Many < T1, T2 >
  {
    #[ inline ]
    fn from_2( _0 : mod1::Floats < T1, T2 >, _1 : mod1::Floats < T1, T2 >, ) -> Self
    {
      Self( the_module::_vec! [ _0, _1 ] )
    }
  }

  impl < T1 : PartialEq, T2 : Default >
  the_module::From_3 < mod1::Floats < T1, T2 >, mod1::Floats < T1, T2 >, mod1::Floats < T1, T2 >, >
  for Many < T1, T2 >
  {
    #[ inline ]
    fn from_3( _0 : mod1::Floats < T1, T2 >, _1 : mod1::Floats < T1, T2 >, _2 : mod1::Floats < T1, T2 >, ) -> Self
    {
      Self( the_module::_vec! [ _0, _1, _2 ] )
    }
  }

}

//

include!( "./many_parametrized_main_test_only.rs" );
