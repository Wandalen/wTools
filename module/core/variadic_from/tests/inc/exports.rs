#[ allow( unused_imports ) ]
use super::*;

// make sure all entities are exported

mod m1
{
  use super::*;
  use the_module::variadic::{ From1, Into1, From2, From3, from };
}

mod m2
{
  use super::*;
  use the_module::prelude::{ From1, Into1, From2, From3, from };
}

mod m3
{
  use super::*;
  use the_module::exposed::{ From1, Into1, From2, From3, from };
}
