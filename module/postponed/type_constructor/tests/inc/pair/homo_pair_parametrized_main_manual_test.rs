#[ allow( unused_imports ) ]
use super::*;

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

#[ derive( Debug, Clone, PartialEq ) ]
struct Pair< T1 : PartialEq + std::marker::Copy, T2 : Default >( pub mod1::Floats< T1, T2 >, pub mod1::Floats< T1, T2 > );
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > core::ops::Deref for Pair< T1, T2 >
{
  type Target = ( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > );

  #[ inline ]
  fn deref( &self ) -> &Self::Target
  {
    #[ cfg( debug_assertions ) ]
    {
      let layout1 = core::alloc::Layout::new::< Self >();
      let layout2 = core::alloc::Layout::new::< Self::Target >();
      debug_assert_eq!( layout1, layout2 );
    }
    unsafe { core::mem::transmute::< _, _ >( self ) }
  }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > core::ops::DerefMut for Pair< T1, T2 >
{
  #[ inline ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    #[ cfg( debug_assertions ) ]
    {
      let layout1 = core::alloc::Layout::new::< Self >();
      let layout2 = core::alloc::Layout::new::< Self::Target >();
      debug_assert_eq!( layout1, layout2 );
    }
    unsafe { core::mem::transmute::< _, _ >( self ) }
  }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > From< ( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > ) > for Pair< T1, T2 >
{
  #[ inline ]
  fn from( src : ( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > ) ) -> Self { Self( src.0, src.1 ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > From< Pair< T1, T2 >> for ( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > )
{
  #[ inline ]
  fn from( src : Pair< T1, T2 > ) -> Self { ( src.0, src.1 ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > From< [ mod1::Floats< T1, T2 >; 2 ]> for Pair< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn from( src : [ mod1::Floats< T1, T2 >; 2 ] ) -> Self { Self( src[ 0 ].clone(), src[ 1 ].clone() ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > From< Pair< T1, T2 >> for [ mod1::Floats< T1, T2 >; 2 ]
{
  #[ inline ]
  fn from( src : Pair< T1, T2 > ) -> Self { [ src.0, src.1 ] }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > From< &[ mod1::Floats< T1, T2 > ]> for Pair< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn from( src : &[ mod1::Floats< T1, T2 > ] ) -> Self { Self( src[ 0 ].clone(), src[ 1 ].clone() ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default >
  the_module::CloneAsTuple< ( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > ) > for Pair< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn clone_as_tuple( &self ) -> ( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > ) { ( self.0.clone(), self.1.clone() ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > the_module::CloneAsArray< mod1::Floats< T1, T2 >, 2 > for Pair< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn clone_as_array( &self ) -> [ mod1::Floats< T1, T2 >; 2 ] { [ self.0.clone(), self.1.clone() ] }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > the_module::AsTuple< ( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > ) >
  for Pair< T1, T2 >
{
  #[ inline ]
  fn as_tuple( &self ) -> &( mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 > ) { unsafe { core::mem::transmute::< _, _ >( self ) } }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > the_module::AsArray< mod1::Floats< T1, T2 >, 2 > for Pair< T1, T2 >
{
  #[ inline ]
  fn as_array( &self ) -> &[ mod1::Floats< T1, T2 >; 2 ] { unsafe { core::mem::transmute::< _, _ >( self ) } }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > the_module::AsSlice< mod1::Floats< T1, T2 >> for Pair< T1, T2 >
{
  #[ inline ]
  fn as_slice( &self ) -> &[ mod1::Floats< T1, T2 > ] { &the_module::AsArray::as_array( self )[ .. ] }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > the_module::From_1< mod1::Floats< T1, T2 >> for Pair< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn from_1( _0 : mod1::Floats< T1, T2 > ) -> Self { Self( _0.clone(), _0.clone() ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default > the_module::From_2< mod1::Floats< T1, T2 >, mod1::Floats< T1, T2 >>
  for Pair< T1, T2 >
where
  mod1::Floats< T1, T2 > : Clone,
{
  #[ inline ]
  fn from_2( _0 : mod1::Floats< T1, T2 >, _1 : mod1::Floats< T1, T2 > ) -> Self { Self( _0.clone(), _1.clone() ) }
}


include!( "./homo_pair_parametrized_main_test_only.rs" );
