
#[ allow( unused_imports ) ]
use super::*;
// use crate::only_for_terminal_module;

// #[ cfg_attr( feature = "enabled", module_mod_interface ) ]

// xxx : qqq : enable it

// #[ cfg( module_mod_interface ) ]
// #[ cfg( module_is_terminal ) ]
#[ test_tools::nightly ]
#[ test ]
fn trybuild_tests()
{
  // qqq : fix test : if run its test with --target-dir flag it's fall (for example : cargo test --target-dir C:\foo\bar )
  // use test_tools::dependency::trybuild;
  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = test_tools::compiletime::TestCases::new();

  let current_exe_path = std::env::current_exe().expect( "No such file or directory" );

  let exe_directory = dbg!(current_exe_path.parent().expect("No such file or directory"));
  fn find_workspace_root( start_path : &std::path::Path ) -> Option< &std::path::Path >
  {
    start_path
    .ancestors()
    .find( |path| path.join( "Cargo.toml" ).exists() )
  }

  let workspace_root = find_workspace_root( exe_directory ).expect( "No such file or directory" );
  let current_dir = workspace_root.join( "module/core/mod_interface" );

  // micro module

  t.pass( current_dir.join( "tests/inc/derive/micro_modules/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/micro_modules_two/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/micro_modules_two_joined/trybuild.rs" ) );

  // layer

  t.pass( current_dir.join( "tests/inc/derive/layer/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/layer_have_layer/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/layer_have_layer_separate_use/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/layer_have_layer_separate_use_two/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/layer_have_layer_cfg/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/layer_use_cfg/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/layer_have_mod_cfg/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/layer_use_macro/trybuild.rs" ) );

  // use

  t.pass( current_dir.join( "tests/inc/derive/use_basic/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/use_layer/trybuild.rs" ) );
  t.pass( current_dir.join( "tests/inc/derive/use_as/trybuild.rs" ) );

  // attr

  t.pass( current_dir.join( "tests/inc/derive/attr_debug/trybuild.rs" ) );

  //
}

use crate::only_for_terminal_module;

only_for_terminal_module!
{
  #[ test_tools::nightly ]
  #[ test ]
  fn cta_trybuild_tests()
  {
    // qqq : fix test : if run its test with --target-dir flag it's fall (for example : cargo test --target-dir C:\foo\bar )
    use test_tools::dependency::trybuild;
    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();

    let current_exe_path = std::env::current_exe().expect( "No such file or directory" );

    let exe_directory = current_exe_path.parent().expect( "No such file or directory" );
    fn find_workspace_root( start_path : &std::path::Path ) -> Option< &std::path::Path >
    {
      start_path
      .ancestors()
      .find( |path| path.join( "Cargo.toml" ).exists() )
    }

    let workspace_root = find_workspace_root( exe_directory ).expect( "No such file or directory" );
    let current_dir = workspace_root.join( "module/core/mod_interface" );

    t.compile_fail( current_dir.join( "tests/inc/derive/micro_modules_bad_vis/trybuild.rs" ) );
    t.compile_fail( current_dir.join( "tests/inc/derive/micro_modules_unknown_vis/trybuild.rs" ) );
    t.compile_fail( current_dir.join( "tests/inc/derive/layer_bad_vis/trybuild.rs" ) );
    t.compile_fail( current_dir.join( "tests/inc/derive/layer_unknown_vis/trybuild.rs" ) );
    t.compile_fail( current_dir.join( "tests/inc/derive/use_bad_vis/trybuild.rs" ) );
    t.compile_fail( current_dir.join( "tests/inc/derive/use_unknown_vis/trybuild.rs" ) );
  }
}
