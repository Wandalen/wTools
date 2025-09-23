#![allow(dead_code)]
#![allow(clippy ::doc_markdown)]
// use super :: *;

/// Define a private namespace for all its items.
mod private 
{
  pub struct Struct1;
  pub struct Struct2;
}

//

crate ::the_module ::mod_interface! {
  own use
  {
  *
 };
}

//

#[ test ]
fn basic() 
{
  let _ = Struct1;
  let _ = Struct2;
}
