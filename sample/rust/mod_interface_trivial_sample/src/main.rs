use mod_interface::mod_interface;

//

mod_interface!
{
  exposed mod inner;
}

//

fn main()
{
  assert_eq!( exposed::inner::inner_is(), true );
}
