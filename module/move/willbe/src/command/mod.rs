/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use wca::{ Type, CommandsAggregator, CommandsAggregatorFormer };

  ///
  /// Form CA commands grammar.
  ///

  pub fn ca() -> CommandsAggregatorFormer
  {
    CommandsAggregator::former()

    .command( "publish" )
      .hint( "publish the specified package to `crates.io`" )
      .long_hint( "used to publish the specified local package, which is located in the provided directory path, to the `crates.io` crate registry." )
      .subject()
        .hint( "Provide path(s) to the package(s) that you want to publish.\n\t  Each path should point to a directory that contains a `Cargo.toml` file.\n\t  Paths should be separated by a comma." )
        .kind( Type::List( Type::String.into(), ',' ) )
        .optional( true )
        .end()
      .property( "dry" )
        .hint( "Enables 'dry run'. Does not publish, only simulates. Default is `true`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "temp" )
        .hint( "If flag is `true` all test will be running in temporary directories. Default `true`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      // .property( "verbosity" ).hint( "Setup level of verbosity." ).kind( Type::String ).optional( true ).alias( "v" ).end()
      .routine( command::publish )
      .end()

    .command( "list" )
      .hint( "list packages from a directory" )
      .long_hint( "generates a list of packages based on the provided directory path. The directory must contain a `Cargo.toml` file." )
      .subject()
        .hint( "The command will generate a list of packages based on a path that must containing a `Cargo.toml` file. If no path is provided, the current directory is used." )
        .kind( Type::Path )
        .optional( true )
        .end()
      .property( "format" )
        .hint( "Adjusts the output format - 'topsort' for a topologically sorted list or 'tree' for a structure of independent crates trees. The default is `tree`." )
        .kind( Type::String )
        .optional( true )
        .end()
      .property( "with_version" )
        .hint( "`true` to include the versions of the packages in the output. Defaults to `false`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_path" )
        .hint( "`true` to include the paths of the packages in the output. Defaults to `false`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_primary" )
        .hint( "`true` to include primary packages in the output, `false` otherwise. Defaults to `true`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_dev" )
        .hint( "`true` to include development packages in the output, `false` otherwise. Defaults to `false`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_build" )
        .hint( "`true` to include build packages in the output, `false` otherwise. Defaults to `false`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_local" )
        .hint( "`true` to include local packages in the output, `false` otherwise. Defaults to `true`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_remote" )
        .hint( "`true` to include remote packages in the output, `false` otherwise. Defaults to `false`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .routine( command::list )
      .end()

    .command( "readme.health.table.generate" )
      .hint( "Generate a table for the root `Readme.md`" )
      .long_hint( "Generates a data summary table for the `Readme.md` file located in the root of the workspace." )
      .routine( command::readme_health_table_renew )
      .end()

    .command( "test" )
      .hint( "execute tests in specific packages" )
      .long_hint( "this command runs tests in designated packages based on the provided path. It allows for inclusion and exclusion of features, testing on different Rust version channels, parallel execution, and feature combination settings." )
      .subject().hint( "A path to directories with packages. If no path is provided, the current directory is used." ).kind( Type::Path ).optional( true ).end()
      .property( "dry" ).hint( "Enables 'dry run'. Does not run tests, only simulates. Default is `true`." ).kind( Type::Bool ).optional( true ).end()
      .property( "temp" ).hint( "If flag is `true` all test will be running in temporary directories. Default `true`." ).kind( Type::Bool ).optional( true ).end()
      .property( "include" )
        .hint( "A list of features to include in testing. Separate multiple features by comma." )
        .kind( Type::List( Type::String.into(), ',' ) )
        .optional( true )
        .end()
      .property( "exclude" )
        .hint( "A list of features to exclude from testing. Separate multiple features by comma." )
        .kind( Type::List( Type::String.into(), ',' ) )
        .optional( true )
        .end()
      .property( "with_stable" )
        .hint( "Specifies whether or not to run tests on stable Rust version. Default is `true`" )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_nightly" )
        .hint( "Specifies whether or not to run tests on nightly Rust version. Default is `false`." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "concurrent" )
        .hint( "Indicates how match test will be run at the same time. Default is `0` - which means the same number of cores." )
        .kind( Type::Number )
        .optional( true )
        .end()
      .property( "power" )
        .hint( "Defines the depth of feature combination testing. Default is `2`." )
        .kind( Type::Number )
        .optional( true )
        .end()
      .property( "enabled_features")
        .hint( "This features will be always present in feature's combinations.")
        .kind( Type::List( Type::String.into(), ',' ) )
        .optional( true )
        .end()
      .property( "with_all_features" )
        .hint( "Will be only one combination of features ( with all possible features )." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_none_features" )
        .hint( "Will be only one combination of features ( without features )." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_release" )
        .hint( "Indicates whether or not tests will be run on the release optimization." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .property( "with_debug" )
        .hint( "Indicates whether or not tests will be run on the debug optimization." )
        .kind( Type::Bool )
        .optional( true )
        .end()
      .routine( command::test )
      .end()

    // qqq : is it right?
    .command( "workflow.renew" )
      .hint( "generate a workflow for the workspace" )
      .long_hint( "this command generates a development workflow for the entire workspace inferred from the current directory. The workflow outlines the build steps, dependencies, test processes, and more for all modules within the workspace." )
      .routine( command::workflow_renew )
      .end()

    .command( "workspace.renew" )
      .hint( "Create workspace template" )
      .long_hint( "Creates static files and directories.\nIn workspace`s Cargo.toml and module Cargo.toml you need to specify some fields, fill them before use this template." )
      .property( "branches" )
        .hint( "List of branches in your project, this parameter affects the branches that will be specified in Cargo.toml of workspace, which in turn will affect the operation of other commands." )
        .kind( Type::List( Type::String.into(), ',' ) )
        .optional( false )
        .end()
      .property( "repository_url" )
        .hint( "Link to project repository, this parameter affects the repo_url will be specified in Cargo.toml of workspace, which in turn will affect the operation of other commands.." )
        .kind( Type::String )
        .optional( false )
        .end()
      .routine( command::workspace_renew )
      .end()

    .command( "deploy.renew" )
      .hint( "Create deploy template" )
      .long_hint( "Creates static files and directories.\nDeployment to different hosts is done via Makefile." )
      .property( "gcp_project_id" )
        .hint( "Google Cloud Platform Project id for image deployment, terraform state bucket, and, if specified, GCE instance deployment." )
        .kind( Type::String )
        .optional( false )
        .end()
      .property( "gcp_region" )
        .hint( "Google Cloud Platform region location. Default: `europe-central2` (Warsaw)" )
        .kind( Type::String )
        .optional( true )
        .end()
      .property( "gcp_artifact_repo_name" )
        .hint( "Google Cloud Platform Artifact Repository to store docker image in. Will be generated from current directory name if unspecified." )
        .kind( Type::String )
        .optional( false )
        .end()
      .property( "docker_image_name" )
        .hint( "Docker image name to build and deploy. Will be generated from current directory name if unspecified." )
        .kind( Type::String )
        .optional( false )
        .end()
      .routine( command::deploy_renew )
      .end()

    .command( "readme.header.generate" )
      .hint( "Generate header in workspace`s Readme.md file")
      .long_hint( "For use this command you need to specify:\n\n[workspace.metadata]\nmaster_branch = \"alpha\"\nworkspace_name = \"wtools\"\nrepo_url = \"https://github.com/Wandalen/wTools\"\ndiscord_url = \"https://discord.gg/123123\"\n\nin workspace's Cargo.toml.")
      .routine( command::readme_header_renew )
      .end()

    .command( "readme.modules.headers.generate" )
      .hint( "Generates header for each workspace member." )
      .long_hint( "For use this command you need to specify:\n\n[package]\nname = \"test_module\"\nrepository = \"https://github.com/Username/ProjectName/tree/master/module/test_module\"\n...\n[package.metadata]\nstability = \"stable\" (Optional)\ndiscord_url = \"https://discord.gg/1234567890\" (Optional)\n\nin module's Cargo.toml." )
      .routine( command::readme_modules_headers_renew )
      .end()
  }
}

crate::mod_interface!
{

  protected use ca;

  /// List packages.
  layer list;
  /// Publish packages.
  layer publish;
  /// Generates health table in main Readme.md file of workspace.
  // aaa : for Petro : what a table??
  // aaa : add more details to documentation
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
