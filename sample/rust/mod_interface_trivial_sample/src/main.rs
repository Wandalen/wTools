use mod_interface::mod_interface;

//

mod_interface!
{

  exposed mod inner;
  // exposed( crate ) use inner;
  // pub mod inner;

}

//

fn main()
{
  assert_eq!( exposed::inner::inner_is(), true );
}

// qqq : rewrite sample
