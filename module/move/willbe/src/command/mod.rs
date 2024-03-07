/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use std::collections::HashMap;
  use wca::{ Type, Routine };

  ///
  /// Form CA commands grammar.
  ///

  pub fn grammar_form() -> Vec< wca::Command >
  {
     let publish_command = wca::Command::former()
    .hint( "publish the specified package to `crates.io`" )
    .long_hint("used to publish the specified local package, which is located in the provided directory path, to the `crates.io` crate registry.")
    .phrase( "publish" )
    .subject( "Provide path(s) to the package(s) that you want to publish.\n\t  Each path should point to a directory that contains a `Cargo.toml` file.\n\t  Paths should be separated by a comma.", Type::List( Type::String.into(), ',' ), true )
    .property( "dry", "Enables 'dry run'. Does not publish, only simulates. Default is `true`.", Type::Bool, true )
    // .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    // .property_alias( "verbosity", "v" )
    .form();

    let list_command = wca::Command::former()
    .hint( "list packages from a directory" )
    .long_hint( "generates a list of packages based on the provided directory path. The directory must contain a `Cargo.toml` file." )
    .phrase( "list" )
    .subject( "The command will generate a list of packages based on a path that must containing a `Cargo.toml` file. If no path is provided, the current directory is used.", Type::Path, true )
    .property( "format", "Adjusts the output format - 'topsort' for a topologically sorted list or 'tree' for a structure of independent crates trees. The default is `tree`.", Type::String, true )
    .property( "with_version", "`true` to include the versions of the packages in the output. Defaults to `false`.", Type::Bool, true )
    .property( "with_path", "`true` to include the paths of the packages in the output. Defaults to `false`.", Type::Bool, true )
    .property( "with_primary", "`true` to include primary packages in the output, `false` otherwise. Defaults to `true`.", Type::Bool, true )
    .property( "with_dev", "`true` to include development packages in the output, `false` otherwise. Defaults to `false`.", Type::Bool, true )
    .property( "with_build", "`true` to include build packages in the output, `false` otherwise. Defaults to `false`.", Type::Bool, true )
    .property( "with_local", "`true` to include local packages in the output, `false` otherwise. Defaults to `true`.", Type::Bool, true )
    .property( "with_remote", "`true` to include remote packages in the output, `false` otherwise. Defaults to `false`.", Type::Bool, true )
    .form();

    let create_table_command = wca::Command::former()
    .hint( "Generate a table for the root `Readme.md`" )
    .long_hint( "Generates a data summary table for the `Readme.md` file located in the root of the workspace." )
    .phrase( "readme.health.table.generate" )
    .form();

    let run_tests_command = wca::Command::former()
    .hint( "execute tests in specific packages" )
    .long_hint( "this command runs tests in designated packages based on the provided path. It allows for inclusion and exclusion of features, testing on different Rust version channels, parallel execution, and feature combination settings." )
    .phrase( "test" )
    .subject( "A path to directories with packages. If no path is provided, the current directory is used.", Type::Path, true )
    .property( "dry", "Enables 'dry run'. Does not run tests, only simulates. Default is `true`.", Type::Bool, true )
    .property( "include", "A list of features to include in testing. Separate multiple features by comma.", Type::List( Type::String.into(), ',' ), true )
    .property( "exclude", "A list of features to exclude from testing. Separate multiple features by comma.", Type::List( Type::String.into(), ',' ), true )
    .property( "with_stable", "Specifies whether or not to run tests on stable Rust version. Default is `true`", Type::Bool, true )
    .property( "with_nightly", "Specifies whether or not to run tests on nightly Rust version. Default is `false`.", Type::Bool, true )
    .property( "concurrent", "Indicates how match test will be run at the same time. Default is `0` - which means the same number of cores.", Type::Number, true )
    .property( "power", "Defines the depth of feature combination testing. Default is `1`.", Type::Number, true )
    .form();

    let generate_workflow = wca::Command::former()
    .hint( "generate a workflow for the workspace" )
    .long_hint( "this command generates a development workflow for the entire workspace inferred from the current directory. The workflow outlines the build steps, dependencies, test processes, and more for all modules within the workspace.")
    .phrase( "workflow.generate")
    .form();


    let w_new = wca::Command::former()
    .hint( "Create workspace template" )
    .long_hint( "Creates static files and directories.\nIn workspace`s Cargo.toml and module Cargo.toml you need to specify some fields, fill them before use this template." )
    .phrase( "workspace.renew" )
    .property( "branches", "List of branches in your project, this parameter affects the branches that will be specified in Cargo.toml of workspace, which in turn will affect the operation of other commands.", Type::List( Box::new( Type::String ), ',' ), false )
    .property( "repository_url", "Link to project repository, this parameter affects the repo_url will be specified in Cargo.toml of workspace, which in turn will affect the operation of other commands..", Type::String , false )
    .form();

    let d_new = wca::Command::former()
    .hint( "Create deploy template" )
    .long_hint( "" )
    .phrase( "deploy.renew" )
    .property( "gcp_project_id", "", Type::String , false )
    .property( "gcp_region", "", Type::String , false )
    .property( "gcp_artifact_repo_name", "", Type::String , false )
    .property( "docker_image_name", "", Type::String , false )
    .form();

    let readme_header_renew = wca::Command::former()
    .hint( "Generate header in workspace`s Readme.md file")
    .long_hint( "For use this command you need to specify:\n\n[workspace.metadata]\nmaster_branch = \"alpha\"\nworkspace_name = \"wtools\"\nrepo_url = \"https://github.com/Wandalen/wTools\"\ndiscord_url = \"https://discord.gg/123123\"\n\nin workspace's Cargo.toml.")
    .phrase( "readme.header.generate" )
    .form();

    let readme_modules_headers_renew = wca::Command::former()
    .hint( "Generates header for each workspace member." )
    .long_hint( "For use this command you need to specify:\n\n[package]\nname = \"test_module\"\nrepository = \"https://github.com/Username/ProjectName/tree/master/module/test_module\"\n...\n[package.metadata]\nstability = \"stable\" (Optional)\ndiscord_url = \"https://discord.gg/1234567890\" (Optional)\n\nin module's Cargo.toml." )
    .phrase( "readme.modules.headers.generate" )
    .form();

    vec!
    [
      publish_command,
      list_command,
      create_table_command,
      run_tests_command,
      generate_workflow,
      w_new,
      d_new,
      readme_header_renew,
      readme_modules_headers_renew,
    ]
  }

  ///
  /// Form CA commands executor.
  ///

  pub fn executor_form() -> HashMap< String, Routine >
  {
    use command::*;
    HashMap::from
    ([
      ( "publish".to_owned(), Routine::new( publish ) ),
      ( "list".to_owned(), Routine::new( list ) ),
      ( "readme.health.table.generate".to_owned(), Routine::new( readme_health_table_renew ) ),
      ( "test".to_owned(), Routine::new( test ) ),
      ( "workflow.renew".to_owned(), Routine::new( workflow_renew ) ),
      ( "workspace.renew".to_owned(), Routine::new( workspace_renew ) ),
      ( "deploy.renew".to_owned(), Routine::new( deploy_renew ) ),
      ( "readme.header.generate".to_owned(), Routine::new( readme_header_renew ) ),
      ( "readme.modules.headers.generate".to_owned(), Routine::new( readme_modules_headers_renew ) ),
    ])
  }
}

crate::mod_interface!
{

  protected use grammar_form;
  protected use executor_form;

  /// List packages.
  layer list;
  /// Publish packages.
  layer publish;
  /// Generate tables
  // qqq : for Petro : what a table??
  layer readme_health_table_renew;
  /// Run all tests
  layer test;
  /// Generate workflow
  layer workflow_renew;
  /// Workspace new
  layer workspace_renew;
  /// Deploy new
  layer deploy_renew;
  /// Generate header in main readme.md
  layer main_header;
  /// Generate headers
  layer readme_modules_headers_renew;

}
