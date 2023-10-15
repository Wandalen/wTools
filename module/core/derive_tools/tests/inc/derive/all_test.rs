use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, Default, FromInner, InnerFrom, Deref, DerefMut, AsRef, AsMut ) ]
pub struct IsTransparent( bool );

include!( "./only_test/all.rs" );
