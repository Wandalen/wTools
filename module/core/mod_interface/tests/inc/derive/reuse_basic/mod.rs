
// use super::*;

/// Internal namespace.
mod private
{
}

mod child;

//

crate::the_module::mod_interface!
{
  reuse child;
}

//

#[ test ]
fn basic()
{

  let _ = child::Own;
  let _ = child::Orphan;
  let _ = child::Exposed;
  let _ = child::Prelude;

  let _ = Own;
  let _ = Orphan;
  let _ = Exposed;
  let _ = Prelude;

}
