#![allow(unused_imports)]
use super::*;
use crate::the_module::{AsMut, AsRef, Deref, DerefMut, From, Index, IndexMut, InnerFrom, Not, New};

#[ derive( Debug, Clone, Copy, PartialEq, From, Deref, DerefMut, AsRef, AsMut ) ]
pub struct IsTransparent(bool);

impl Default for IsTransparent {
  #[ inline( always ) ]
  fn default() -> Self {
    Self(true)
  }
}

include!("./only_test/all.rs");
