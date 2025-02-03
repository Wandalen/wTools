use super::*;

#[ test ]
fn exposed_main_namespace()
{

  let _v : Vec< u32 > = the_module::collection::Vec::new();
  let _v : Vec< u32 > = the_module::exposed::collection::Vec::new();
  use the_module::exposed::*;
  let _v : Vec< u32 > = collection::Vec::new();

}