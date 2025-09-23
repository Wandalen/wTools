use super :: *;

mod mod1
{
  use super :: *;

  // #[ derive( Debug, PartialEq, Eq, the_module ::New ) ]
  
  pub struct Struct1
  {
  pub a: i32,
  pub b: bool,
 }

}

// include!( "./only_test/multiple_named.rs" );
