
// use super::*;

/// Define a private namespace for all its items.
mod private
{
  pub struct Struct1;
  pub struct Struct2;
}

//

crate::the_module::mod_interface!
{
  own use
  {
    *
  };
}

//

#[ test ]
fn basic()
{
  let _s1 = Struct1;
  let _s2 = Struct2;
}
