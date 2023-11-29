
#[ allow( unused_imports ) ]
use super::*;
use crate::only_for_terminal_module;

// #[ cfg_attr( feature = "enabled", module_mod_interface ) ]

only_for_terminal_module!
{

  // #[ cfg( module_mod_interface ) ]
  // #[ cfg( module_is_terminal ) ]
  #[ test_tools::nightly ]
  // #[ cfg( RUSTC_IS_NIGHTLY ) ]
  tests_impls!
  {

    fn trybuild_tests()
    {
      // use test_tools::dependency::trybuild;
      println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
      // let t = trybuild::TestCases::new();
      let t = test_tools::compiletime::TestCases::new();

      // micro module

      t.pass( "tests/inc/derive/micro_modules/trybuild.rs" );
      t.pass( "tests/inc/derive/micro_modules_two/trybuild.rs" );
      t.pass( "tests/inc/derive/micro_modules_two_joined/trybuild.rs" );
      t.compile_fail( "tests/inc/derive/micro_modules_bad_vis/trybuild.rs" );
      t.compile_fail( "tests/inc/derive/micro_modules_unknown_vis/trybuild.rs" );

      // layer

      t.pass( "tests/inc/derive/layer/trybuild.rs" );
      t.pass( "tests/inc/derive/layer_have_layer/trybuild.rs" );
      t.pass( "tests/inc/derive/layer_have_layer_separate_use/trybuild.rs" );
      t.pass( "tests/inc/derive/layer_have_layer_separate_use_two/trybuild.rs" );
      t.pass( "tests/inc/derive/layer_have_layer_cfg/trybuild.rs" );
      t.pass( "tests/inc/derive/layer_use_cfg/trybuild.rs" );
      t.pass( "tests/inc/derive/layer_have_mod_cfg/trybuild.rs" );
      t.pass( "tests/inc/derive/layer_use_macro/trybuild.rs" );
      t.compile_fail( "tests/inc/derive/layer_bad_vis/trybuild.rs" );
      t.compile_fail( "tests/inc/derive/layer_unknown_vis/trybuild.rs" );

      // use

      t.pass( "tests/inc/derive/use_basic/trybuild.rs" );
      t.pass( "tests/inc/derive/use_layer/trybuild.rs" );
      t.pass( "tests/inc/derive/use_as/trybuild.rs" );
      t.compile_fail( "tests/inc/derive/use_bad_vis/trybuild.rs" );
      t.compile_fail( "tests/inc/derive/use_unknown_vis/trybuild.rs" );

      // attr

      t.pass( "tests/inc/derive/attr_debug/trybuild.rs" );

      //

    }

  }

  // #[ cfg( module_mod_interface ) ]
  // #[ cfg( module_is_terminal ) ]
  // #[ cfg( RUSTC_IS_NIGHTLY ) ]
  #[ test_tools::nightly ]
  tests_index!
  {
    trybuild_tests,
  }

}