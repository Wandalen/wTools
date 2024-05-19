#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;
use derive_tools::From;

#[ derive( Debug, PartialEq, From ) ]
#[ debug ]
pub enum GetData< 'a, T : ToString + ?Sized >
{
  Nothing,
  FromT( &'a T ),
}

// == begin of generated
// == end of generated

include!( "./only_test/from_inner_variants_generics.rs" );

// xxx2 : get complete
// xxx2 : test name conflicts