#[allow(unused_imports)]
use super::*;

//

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
struct Pair< T1 : PartialEq + std::marker::Copy, T2 : Default, T : Copy >( pub mod1::Floats< T1, T2 >, pub std::sync::Arc< T >);
impl< T1 : PartialEq + std::marker::Copy, T2 : Default, T : Copy > From< ( mod1::Floats< T1, T2 >, std::sync::Arc< T > ) >
  for Pair< T1, T2, T >
{
  #[ inline ]
  fn from( src : ( mod1::Floats< T1, T2 >, std::sync::Arc< T >) ) -> Self { Self( src.0, src.1 ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default, T : Copy > From< Pair< T1, T2, T > >
  for ( mod1::Floats< T1, T2 >, std::sync::Arc< T > )
{
  #[ inline ]
  fn from( src : Pair< T1, T2, T > ) -> Self { ( src.0, src.1 ) }
}
impl< T1 : PartialEq + std::marker::Copy, T2 : Default, T : Copy >
  the_module::From_2< mod1::Floats< T1, T2 >, std::sync::Arc< T > > for Pair< T1, T2, T >
{
  #[ inline ]
  fn from_2( _0 : mod1::Floats< T1, T2 >, _1 : std::sync::Arc< T > ) -> Self { Self( _0, _1 ) }
}

include!("./pair_parametrized_main_test_only.rs");
