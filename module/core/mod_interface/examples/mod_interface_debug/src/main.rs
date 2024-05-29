//! qqq : write proper description
use mod_interface::mod_interface;

//

fn main()
{
  assert_eq!( prelude::inner_is(), inner::prelude::inner_is() );
}

//

mod_interface!
{
  #![ debug ]
  /// Inner.
  layer inner;
}
