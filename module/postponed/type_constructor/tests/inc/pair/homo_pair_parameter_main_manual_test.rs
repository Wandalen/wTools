#[ allow( unused_imports ) ]
use super::*;

///
/// Attribute which is inner.
///

#[ derive( Debug, Clone, PartialEq ) ]
struct Pair< T1 >( pub T1, pub T1 );
impl< T1 > core::ops::Deref for Pair< T1 >
{
  type Target = ( T1, T1 );

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
impl< T1 > core::ops::DerefMut for Pair< T1 >
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
impl< T1 > From< ( T1, T1 ) > for Pair< T1 >
{
  #[ inline ]
  fn from( src : ( T1, T1 ) ) -> Self { Self( src.0, src.1 ) }
}
impl< T1 > From< Pair< T1 >> for ( T1, T1 )
{
  #[ inline ]
  fn from( src : Pair< T1 > ) -> Self { ( src.0, src.1 ) }
}
impl< T1 > From< [ T1; 2 ]> for Pair< T1 >
where
  T1 : Clone,
{
  #[ inline ]
  fn from( src : [ T1; 2 ] ) -> Self { Self( src[ 0 ].clone(), src[ 1 ].clone() ) }
}
impl< T1 > From< Pair< T1 >> for [ T1; 2 ]
{
  #[ inline ]
  fn from( src : Pair< T1 > ) -> Self { [ src.0, src.1 ] }
}
impl< T1 > From< &[ T1 ]> for Pair< T1 >
where
  T1 : Clone,
{
  #[ inline ]
  fn from( src : &[ T1 ] ) -> Self
  {
    debug_assert_eq!( src.len(), 2 );
    Self( src[ 0 ].clone(), src[ 1 ].clone() )
  }
}
impl< T1 > From< T1 > for Pair< T1 >
where
  T1 : Clone,
{
  #[ inline ]
  fn from( src : T1 ) -> Self { Self( src.clone(), src.clone() ) }
}
impl< T1 > the_module::CloneAsTuple< ( T1, T1 ) > for Pair< T1 >
where
  T1 : Clone,
{
  #[ inline ]
  fn clone_as_tuple( &self ) -> ( T1, T1 ) { ( self.0.clone(), self.1.clone() ) }
}
impl< T1 > the_module::CloneAsArray< T1, 2 > for Pair< T1 >
where
  T1 : Clone,
{
  #[ inline ]
  fn clone_as_array( &self ) -> [ T1; 2 ] { [ self.0.clone(), self.1.clone() ] }
}
impl< T1 > the_module::AsTuple< ( T1, T1 ) > for Pair< T1 >
{
  #[ inline ]
  fn as_tuple( &self ) -> &( T1, T1 ) { unsafe { core::mem::transmute::< &_, &( T1, T1 ) >( self ) } }
}
impl< T1 > the_module::AsArray< T1, 2 > for Pair< T1 >
{
  #[ inline ]
  fn as_array( &self ) -> &[ T1; 2 ] { unsafe { core::mem::transmute::< &_, &[ T1; 2 ]>( self ) } }
}
impl< T1 > the_module::AsSlice< T1 > for Pair< T1 >
{
  #[ inline ]
  fn as_slice( &self ) -> &[ T1 ] { &the_module::AsArray::as_array( self )[ ..] }
}
impl< T1 > the_module::From_0 for Pair< T1 >
where
  T1 : Default,
{
  #[ inline ]
  fn from_0() -> Self { Self( Default::default(), Default::default() ) }
}
impl< T1 > the_module::From_1< T1 > for Pair< T1 >
where
  T1 : Clone,
{
  #[ inline ]
  fn from_1( _0 : T1 ) -> Self { Self( _0.clone(), _0.clone() ) }
}
impl< T1 > the_module::From_2< T1, T1 > for Pair< T1 >
{
  #[ inline ]
  fn from_2( _0 : T1, _1 : T1 ) -> Self { Self( _0, _1 ) }
}

include!( "./homo_pair_parameter_main_test_only.rs" );
