#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn basic()
{

  use the_module::own::*;
  let _v : Vec< u32 > = collection::Vec::new();
  let _v : Vec< u32 > = the_module::collection::Vec::new();
  let _v : Vec< u32 > = the_module::own::collection::Vec::new();

}