#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;

pub trait CloneAny{}
pub trait Context{}
pub trait End{}

#[ derive( Clone, the_module::Former ) ]
pub struct OnEnd
{
  inner : std::sync::Arc< core::cell::RefCell< dyn CloneAny > >
}
