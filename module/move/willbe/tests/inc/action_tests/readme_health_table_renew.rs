use super::*;
use assert_fs::prelude::*;
use the_module::action;
use std::io::Read;

fn arrange( source : &str ) -> assert_fs::TempDir
{
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = std::path::Path::new( ASSET_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

  temp
}

#[ test ]
#[ should_panic ]
// should panic, because the url to the repository is not in Cargo.toml of the workspace or in Cargo.toml of the module.
fn without_any_toml_configurations_test()
{
  // Arrange
  let temp = arrange( "without_any_toml_configurations" );
  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();
}

#[ test ]
fn tags_should_stay()
{
  // Arrange
  let temp = arrange( "without_module_toml_configurations" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  assert!( actual.contains( "<!--{ generate.healthtable( '.' ) } -->" ) );
  assert!( actual.contains( "<!--{ generate.healthtable.end } -->" ) );
}

#[ test ]
// url to repository and list of branches should be taken from workspace Cargo.toml, stability - experimental by default
fn stability_experimental_by_default()
{
  // Arrange
  let temp = arrange( "without_module_toml_configurations" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  assert!( actual.contains( "[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) |" ) );
}

#[ test ]
// url to repository and stability should be taken from module Cargo.toml, branches should not be awarded because they are not listed in the workspace Cargo.toml
fn stability_and_repository_from_module_toml()
{
  // Arrange
  let temp = arrange( "without_workspace_toml_configurations" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  assert!( actual.contains( "[![stability-stable](https://img.shields.io/badge/stability-stable-green.svg)](https://github.com/emersion/stability-badges#stable)" ) );
}

#[ test ]
fn variadic_tag_configuration_test()
{
  // Arrange
  let explicit_all_true_flag =
    "-->\r| Module | Stability | test_branch1 | test_branch2 | Docs | Sample |\n|--------|-----------|--------|--------|:----:|:------:|\n";
  let all_true_flag =
    "-->\r| Module | Stability | test_branch1 | test_branch2 | Docs | Sample |\n|--------|-----------|--------|--------|:----:|:------:|\n";
  let with_stability_only =
    "-->\r| Module | Stability |\n|--------|-----------|\n";
  let with_branches_only =
    "-->\r| Module | test_branch1 | test_branch2 |\n|--------|--------|--------|\n";
  let with_docs_only =
    "-->\r| Module | Docs |\n|--------|:----:|\n";
  let with_gitpod_only =
    "-->\r| Module | Sample |\n|--------|:------:|\n";

  let expected = vec![ explicit_all_true_flag, all_true_flag, with_stability_only, with_branches_only, with_docs_only, with_gitpod_only ];
  let temp = arrange( "variadic_tag_configurations" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut content = String::new();
  _ = file.read_to_string( &mut content ).unwrap();
  for ( index, actual ) in content.split( "###" ).into_iter().enumerate()
  {
    assert!( actual.trim().contains( expected[ index ] ) );
  }
}

//      "   | Sample |\n|--------|-----------|--------|--------|:----:|:------:|\n|    |  | \n<!--{ generate.healthtable.end } -->";
#[ test ]
fn module_cell()
{
  // Arrange
  let temp = arrange( "full_config" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  // qqq : do not do like that. If it will fail how will I know what went wrong? What is the name of the package here?
  assert!( actual.contains( "[_willbe_variadic_tag_configurations_full_config_c](./_willbe_variadic_tag_configurations_full_config_c)" ) );
}

#[ test ]
fn stability_cell()
{
  // Arrange
  let temp = arrange( "full_config" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  dbg!( &actual );
  assert!( actual.contains( "[![stability-deprecated](https://img.shields.io/badge/stability-deprecated-red.svg)](https://github.com/emersion/stability-badges#deprecated)" ) );
}

#[ test ]
fn branches_cell()
{
  // Arrange
  let temp = arrange( "full_config" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  // qqq : do not do like that. If it will fail how will I know what went wrong? What is the name of the package here?
  assert!( actual.contains( "[![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/module_willbe_variadic_tag_configurations_full_config_c_push.yml?label=&branch=test_branch1)](https://github.com/SomeName/SomeCrate/C/actions/workflows/module_willbe_variadic_tag_configurations_full_config_c_push.yml?query=branch%3Atest_branch1) | [![rust-status](https://img.shields.io/github/actions/workflow/status/SomeCrate/C/module_willbe_variadic_tag_configurations_full_config_c_push.yml?label=&branch=test_branch2)](https://github.com/SomeName/SomeCrate/C/actions/workflows/module_willbe_variadic_tag_configurations_full_config_c_push.yml?query=branch%3Atest_branch2)" ) );
}

#[ test ]
fn docs_cell()
{
  // Arrange
  let temp = arrange( "full_config" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  // qqq : do not do like that. If it will fail how will I know what went wrong? What is the name of the package here?
  assert!( actual.contains( "[![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/_willbe_variadic_tag_configurations_full_config_c)" ) );
}

#[ test ]
fn sample_cell()
{
  // Arrange
  let temp = arrange( "full_config" );

  // Act
  _  = action::readme_health_table_renew( &temp ).unwrap();

  // Assert
  let mut file = std::fs::File::open( temp.path().join( "readme.md" ) ).unwrap();
  let mut actual = String::new();
  _ = file.read_to_string( &mut actual ).unwrap();

  // qqq : do not do like that. If it will fail how will I know what went wrong? What is the name of the package here?
  assert!( actual.contains( " [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=.%2F_willbe_variadic_tag_configurations_full_config_c%2Fexamples%2F_willbe_variadic_tag_configurations_c_trivial.rs,RUN_POSTFIX=--example%20_willbe_variadic_tag_configurations_c_trivial/https://github.com/SomeName/SomeCrate/C)" ) );
}
