
#[ allow( unused_imports ) ]
use super::*;

// #[ cfg_attr( feature = "enabled", module_mod_interface ) ]

only_for_terminal_module!
{

  // #[ cfg( module_mod_interface ) ]
  #[ test_tools::rustversion::nightly ]
  tests_impls!
  {

    fn trybuild_tests()
    {
      use test_tools::dependency::trybuild;
      println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
      let t = trybuild::TestCases::new();

      // micro module

      t.pass( "tests/inc/front/micro_modules/trybuild.rs" );
      t.pass( "tests/inc/front/micro_modules_two/trybuild.rs" );
      t.pass( "tests/inc/front/micro_modules_two_joined/trybuild.rs" );
      t.compile_fail( "tests/inc/front/micro_modules_bad_vis/trybuild.rs" );
      t.compile_fail( "tests/inc/front/micro_modules_unknown_vis/trybuild.rs" );

      // layer

      t.pass( "tests/inc/front/layer/trybuild.rs" );
      t.pass( "tests/inc/front/layer_have_layer/trybuild.rs" );
      t.pass( "tests/inc/front/layer_have_layer_separate_use/trybuild.rs" );
      t.pass( "tests/inc/front/layer_have_layer_separate_use_two/trybuild.rs" );
      t.pass( "tests/inc/front/layer_have_layer_cfg/trybuild.rs" );
      t.pass( "tests/inc/front/layer_use_cfg/trybuild.rs" );
      t.pass( "tests/inc/front/layer_have_mod_cfg/trybuild.rs" );
      t.pass( "tests/inc/front/layer_use_macro/trybuild.rs" );
      t.compile_fail( "tests/inc/front/layer_bad_vis/trybuild.rs" );
      t.compile_fail( "tests/inc/front/layer_unknown_vis/trybuild.rs" );

      // etc

      t.pass( "tests/inc/front/attr_debug/trybuild.rs" );
      t.pass( "tests/inc/front/use_non_layer/trybuild.rs" );
      t.compile_fail( "tests/inc/front/use_bad_vis/trybuild.rs" );
      t.compile_fail( "tests/inc/front/use_unknown_vis/trybuild.rs" );

      //

    }

  }

  // #[ cfg( module_mod_interface ) ]
  #[ test_tools::rustversion::nightly ]
  tests_index!
  {
    trybuild_tests,
  }

}