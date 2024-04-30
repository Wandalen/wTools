#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

// xxx : make it working

/// Parameter description.
#[ allow( explicit_outlives_requirements ) ]
// #[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ]
#[ derive( Debug, PartialEq ) ]
pub struct Child< 'child, T >
where
  T : 'child,
{
  name : String,
  arg : &'child T,
}

// == begin of generated

// == end of generated

// xxx : uncomment
// #[ test ]
// fn basic()
// {
//   let got = Child::< 'static, str >::former().name( "abc" ).arg( "arg1" ).end();
//   let exp = Child::< 'static, str >{ name : "abc".into(), arg : "arg1" };
//   a_id!( got, exp );
// }
