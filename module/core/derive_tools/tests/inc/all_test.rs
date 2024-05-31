use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, /* the_module::Default,*/ the_module::From, the_module::InnerFrom, the_module::Deref, the_module::DerefMut, the_module::AsRef, the_module::AsMut ) ]
// #[ default( value = false ) ]
pub struct IsTransparent( bool );

// qqq : xxx : make Default derive working

impl Default for IsTransparent
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( true )
  }
}

include!( "./only_test/all.rs" );
