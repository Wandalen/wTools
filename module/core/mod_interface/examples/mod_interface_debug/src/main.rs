//! qqq : write proper description
use mod_interface::mod_interface;

//

mod private {}
mod_interface!
{
  // Uncomment to see expanded code.
  // #![ debug ]
  /// Child.
  layer child;
}

//

fn main()
{
  assert_eq!( prelude::inner_is(), child::prelude::inner_is() );
}
