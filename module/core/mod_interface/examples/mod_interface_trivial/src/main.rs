//! qqq : write proper descriptionuse mod_interface::mod_interface;

//

use mod_interface::mod_interface;

fn main()
{
  assert_eq!( prelude::inner_is(), prelude::inner_is() );
}

//

mod private {}

//

mod_interface!
{
  /// Child.
  layer child;
}

// qqq : rewrite sample
