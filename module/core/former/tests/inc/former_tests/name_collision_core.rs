#![ allow( dead_code ) ]
#![ allow( non_camel_case_types ) ]

#[ allow( unused_imports ) ]
use super::*;

// pub mod core {}
pub mod std {}
pub mod marker {}
pub trait CloneAny{}
pub trait Context{}
pub trait Formed{}
pub trait OnEnd{}

#[ derive( Clone, the_module::Former ) ]
pub struct core
{
  inner : ::std::sync::Arc< ::core::cell::RefCell< dyn CloneAny > >
}
