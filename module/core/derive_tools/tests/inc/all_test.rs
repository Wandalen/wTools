use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, /* TheModule::Default,*/ TheModule::FromInner, TheModule::InnerFrom, TheModule::Deref, TheModule::DerefMut, TheModule::AsRef, TheModule::AsMut ) ]
// #[ default( value = false ) ]
pub struct IsTransparent( bool );

// qqq2 : make Default derive working

impl Default for IsTransparent
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( true )
  }
}

include!( "./only_test/all.rs" );
