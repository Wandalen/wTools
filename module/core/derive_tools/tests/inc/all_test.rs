use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, Default, TheModule::FromInner, TheModule::InnerFrom, TheModule::Deref, TheModule::DerefMut, TheModule::AsRef, TheModule::AsMut ) ]
pub struct IsTransparent( bool );

include!( "./only_test/all.rs" );
