#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;

pub trait CloneAny{}
pub trait Context{}
pub trait OnEnd{}

#[ derive( Clone, the_module::Former ) ]
pub struct End
{
  inner : std::sync::Arc< core::cell::RefCell< dyn CloneAny > >
}
