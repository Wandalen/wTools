
use super::TheModule;

#[ path = "../dynamic/basic.rs" ]
mod basic;

//

fn trybuild_test()
{

  // let t = trybuild::TestCases::new();
  // t.pass( "../../../rust/test/test/dynamic/basic.rs" );

}

//

TheModule::test_suite!
{
  trybuild,
}
