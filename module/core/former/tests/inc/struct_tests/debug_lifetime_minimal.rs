#![allow(dead_code)]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ] // Commented out - debug attribute only for temporary debugging
pub struct MinimalLifetime<'a> {
  data: &'a str,
}