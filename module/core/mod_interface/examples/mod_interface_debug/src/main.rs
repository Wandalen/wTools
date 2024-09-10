//! qqq : write proper description
use mod_interface::mod_interface;

//

fn main()
{
  assert_eq!( prelude::inner_is(), child::prelude::inner_is() );
}

//

mod private {}

//

mod_interface!
{
  #![ debug ]
  /// Child.
  layer child;
}
