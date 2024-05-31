#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;
use derive_tools::From;

#[ derive( Debug, PartialEq, From ) ]
// #[ debug ]
pub enum GetData< 'a, T : ToString + ?Sized = str >
{
  Nothing,
  FromT( &'a T ),
}

// == begin of generated
// == end of generated

include!( "./only_test/variants_generics.rs" );
