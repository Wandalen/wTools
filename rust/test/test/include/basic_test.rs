
#[cfg( feature = "in_wtools" )]
use wtools::test::*;
#[cfg( not( feature = "in_wtools" ) )]
use wtest_basic::*;

#[ path = "../dynamic/basic.rs" ]
mod basic;

// mod basic
// {
//   include!( "../dynamic/basic.rs" );
// }

// #[ path = "../dynamic/composition.rs" ]
// mod composition;

// mod composition
// {
//   include!( "../dynamic/composition.rs" );
// }

//

test_routine!{ trybuild =>
{
  // let t = trybuild::TestCases::new();
  // t.pass( "../../../rust/test/test/dynamic/basic.rs" );
}}

//

test_suite!
{
  trybuild,
}
