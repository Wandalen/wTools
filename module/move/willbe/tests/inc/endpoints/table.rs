const ASSETS_PATH : &str = "tests/assets";

use assert_fs::prelude::*;
use crate::TheModule::endpoint::{ self };

mod table_create_test
{
  use std::io::Read;

  use super::*;

  fn arrange( source: &str ) -> assert_fs::TempDir
  {
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

    temp
  }

  #[ test ]
  // should panic, because the url to the repository is not in Cargo.toml of the workspace or in Cargo.toml of the module. 
  fn without_any_toml_configurations_test()
  {
    // Arrange
    let temp = arrange( "without_any_toml_configurations" );
    // Act
    let result  = endpoint::table_create( &temp );
    // Assert
    assert!( result.is_err() );
  }

  #[ test ]
  // url to repository and list of branches should be taken from workspace Cargo.toml, stability - experimental by default 
  fn without_module_toml_configurations_test()
  {
    // Arrange
    let expected = 
    "<!--{ generate.healthtable( '.' ) } -->\r| Module | Stability | test_branch1 | test_branch2 | Docs | Sample |\n|--------|-----------|--------|--------|:----:|:------:|\n| [c](./c) |[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status](https://img.shields.io/github/actions/workflow/status/Username/test/ModuleCPush.yml?label=&branch=test_branch1)](https://https://github.com/Username/test/actions/workflows/ModuleCPush.yml) | [![rust-status](https://img.shields.io/github/actions/workflow/status/Username/test/ModuleCPush.yml?label=&branch=test_branch2)](https://https://github.com/Username/test/actions/workflows/ModuleCPush.yml) | [![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/c) | [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fc_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20c_trivial_sample/https://github.com/Username/test) | \n<!--{ generate.healthtable.end } -->\r\n\r\n";
    let temp = arrange( "without_module_toml_configurations" );

    // Act
    _  = endpoint::table_create( &temp ).unwrap();

    // Assert
    let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
    let mut actual = String::new();
    _ = file.read_to_string( &mut actual ).unwrap();
    assert_eq!( expected, actual );
  }

  #[ test ]
  // url to repository and stability should be taken from module Cargo.toml, branches should not be awarded because they are not listed in the workspace Cargo.toml 
  fn without_workspace_toml_configurations_test()
  {
    // Arrange
    let expected = 
    "<!--{ generate.healthtable( '.' ) } -->\r| Module | Stability | Docs | Sample |\n|--------|-----------|:----:|:------:|\n| [c](./c) |[![stability-stable](https://img.shields.io/badge/stability-stable-green.svg)](https://github.com/emersion/stability-badges#stable) | [![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/c) | [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fc_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20c_trivial_sample/https://github.com/Testusername/TestProject) | \n<!--{ generate.healthtable.end } -->\r\n\r\n";
    let temp = arrange( "without_workspace_toml_configurations" );

    // Act
    _  = endpoint::table_create( &temp ).unwrap();

    // Assert
    let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
    let mut actual = String::new();
    _ = file.read_to_string( &mut actual ).unwrap();
    assert_eq!( expected, actual );
  }

  #[ test ]
  fn variadic_tag_configuration_test() 
  {
    // Arrange
    let explicit_all_true_flag = 
    "<!--{ generate.healthtable( path: '.', with_stability : 1, with_branches : 1, with_docs : 1, with_gitpod : 1 ) } -->\r| Module | Stability | test_branch1 | test_branch2 | Docs | Sample |\n|--------|-----------|--------|--------|:----:|:------:|\n| [c](./c) |[![stability-deprecated](https://img.shields.io/badge/stability-deprecated-red.svg)](https://github.com/emersion/stability-badges#deprecated) | [![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/ModuleCPush.yml?label=&branch=test_branch1)](https://https://github.com/SomeName/SomeCrate/C/actions/workflows/ModuleCPush.yml) | [![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/ModuleCPush.yml?label=&branch=test_branch2)](https://https://github.com/SomeName/SomeCrate/C/actions/workflows/ModuleCPush.yml) | [![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/c) | [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fc_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20c_trivial_sample/https://github.com/SomeName/SomeCrate/C) | \n<!--{ generate.healthtable.end } -->";
    let all_true_flag = 
    "<!--{ generate.healthtable( '.' ) } -->\r| Module | Stability | test_branch1 | test_branch2 | Docs | Sample |\n|--------|-----------|--------|--------|:----:|:------:|\n| [c](./c) |[![stability-deprecated](https://img.shields.io/badge/stability-deprecated-red.svg)](https://github.com/emersion/stability-badges#deprecated) | [![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/ModuleCPush.yml?label=&branch=test_branch1)](https://https://github.com/SomeName/SomeCrate/C/actions/workflows/ModuleCPush.yml) | [![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/ModuleCPush.yml?label=&branch=test_branch2)](https://https://github.com/SomeName/SomeCrate/C/actions/workflows/ModuleCPush.yml) | [![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/c) | [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fc_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20c_trivial_sample/https://github.com/SomeName/SomeCrate/C) | \n<!--{ generate.healthtable.end } -->";
    let with_stability_only = 
    "<!--{ generate.healthtable( path: '.', with_stability : 1, with_branches : 0, with_docs : 0, with_gitpod : 0 ) } -->\r| Module | Stability |\n|--------|-----------|\n| [c](./c) |[![stability-deprecated](https://img.shields.io/badge/stability-deprecated-red.svg)](https://github.com/emersion/stability-badges#deprecated) | \n<!--{ generate.healthtable.end } -->";
    let with_branches_only =
    "<!--{ generate.healthtable( path: '.', with_stability : 0, with_branches : 1, with_docs : 0, with_gitpod : 0  ) } -->\r| Module | test_branch1 | test_branch2 |\n|--------|--------|--------|\n| [c](./c) |[![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/ModuleCPush.yml?label=&branch=test_branch1)](https://https://github.com/SomeName/SomeCrate/C/actions/workflows/ModuleCPush.yml) | [![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/ModuleCPush.yml?label=&branch=test_branch2)](https://https://github.com/SomeName/SomeCrate/C/actions/workflows/ModuleCPush.yml) | \n<!--{ generate.healthtable.end } -->";
    let with_docs_only = 
    "<!--{ generate.healthtable( path: '.', with_stability : 0, with_branches : 0, with_docs : 1, with_gitpod : 0 ) } -->\r| Module | Docs |\n|--------|:----:|\n| [c](./c) |[![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/c) | \n<!--{ generate.healthtable.end } -->";
    let with_gitpod_only = 
    "<!--{ generate.healthtable( path: '.', with_stability : 0, with_branches : 0, with_docs : 0, with_gitpod : 1 ) } -->\r| Module | Sample |\n|--------|:------:|\n| [c](./c) |[![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fc_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20c_trivial_sample/https://github.com/SomeName/SomeCrate/C) | \n<!--{ generate.healthtable.end } -->";

    let expected = vec![ explicit_all_true_flag, all_true_flag, with_stability_only, with_branches_only, with_docs_only, with_gitpod_only ];
    let temp = arrange( "variadic_tag_configurations" );

    // Act
    _  = endpoint::table_create( &temp ).unwrap();

    // Assert
    let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
    let mut content = String::new();
    _ = file.read_to_string( &mut content ).unwrap();
    for ( index, actual ) in content.split( "###" ).into_iter().enumerate() 
    {
      assert_eq!( expected[ index ], actual.trim() ); 
    }
  }

}
