#![allow(unused_imports)]
use super :: *;
use crate ::the_module :: { AsMut, AsRef, Deref, DerefMut, From, Index, IndexMut, InnerFrom, Not, New };

#[ derive( Debug, Clone, Copy, PartialEq, Default, From, Deref, DerefMut, AsRef, AsMut ) ]
pub struct IsTransparent(bool);

include!("./only_test/all.rs");
