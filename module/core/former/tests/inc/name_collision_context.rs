#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;

pub trait CloneAny{}
pub trait End{}
pub trait OnEnd{}

#[ derive( Clone, former::Former ) ]
pub struct Context
{
  inner : std::sync::Arc< core::cell::RefCell< dyn CloneAny > >
}
