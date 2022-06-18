
use super::*;

#[ path = "../dynamic/basic.rs" ]
mod basic;

//

TheModule::tests_impls!
{

  fn trybuild_test()
  {

    // let t = trybuild::TestCases::new();
    // t.pass( "../../../rust/test/test/dynamic/basic.rs" );

  }

}

//

TheModule::tests_index!
{
  trybuild_test,
}
