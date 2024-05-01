//! qqq : write proper descriptionuse mod_interface::mod_interface;

//

use mod_interface::mod_interface;

fn main()
{
  assert_eq!( prelude::inner_is(), prelude::inner_is() );
}

//

mod_interface!
{
  /// Inner.
  layer inner;
}

// qqq : rewrite sample
/* aaa : Dmytro : sample with layer */
