#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::From ) ]
// #[ debug ]
pub enum GetData< 'a, T : ToString >
{
  Nothing,
  FromT( &'a T ),
}

// == begin of generated

// == end of generated

#[ test ]
fn variant_from()
{

  let got : GetData< '_, str > = From::from( &b"abc"[ .. ] );
  let exp = GetData::< '_, str >::FromT( "abc" );
  a_id!( got, exp );

}
