mod private
{

  use crate :: *;

  use std ::
  {
  fs ::File,
  io :: { Write, Read },
 };

  use std ::path ::Path;
  use collection_tools ::collection ::BTreeMap;
  use handlebars :: { RenderError, TemplateError };
  use toml_edit ::Document;

  use entity :: { PathError, WorkspaceInitError };
  // Explicit import for Result and its variants for pattern matching
  use core ::result ::Result :: { Ok, Err };

  use error ::
  {
  typed ::Error,
  // err,
 };

  #[ derive( Debug, Error ) ]
  pub enum CiCdGenerateError
  {
  #[ error( "Common error: {0}" ) ]
  Common( #[ from ] error ::untyped ::Error ),
  #[ error( "I/O error: {0}" ) ]
  IO( #[ from ] std ::io ::Error ),
  #[ error( "Crate directory error: {0}" ) ]
  CrateDir( #[ from ] PathError ),
  #[ error( "Workspace error: {0}" ) ]
  Workspace( #[ from ] WorkspaceInitError ),
  #[ error( "Template error: {0}" ) ]
  Template( #[ from ] TemplateError ),
  #[ error( "Render error: {0}" ) ]
  Render( #[ from ] RenderError ),
 }

  /// Generate workflows for modules in .github/workflows directory.
  /// # Errors
  ///
  /// # Panics
  #[ allow( clippy ::too_many_lines, clippy ::result_large_err ) ]
  pub fn action( base_path: &Path ) -> Result< (), CiCdGenerateError >
  {
  let workspace_cache = Workspace ::try_from( CrateDir ::try_from( base_path )? )?;
  let username_and_repository = &username_and_repository
  (
   &workspace_cache.workspace_root().join( "Cargo.toml" )?.to_path_buf().try_into()?, // qqq
   workspace_cache.packages(),
 )?;
  let workspace_root: &Path = &workspace_cache.workspace_root();
  // find directory for workflows
  let workflow_root = workspace_root.join( ".github" ).join( "workflows" );

  // preparing templates
  let mut handlebars = handlebars ::Handlebars ::new();

  handlebars.register_template_string
  (
   "auto_pr_to",
   include_str!( "../../template/workflow/auto_pr_to.hbs" )
 )?;
  handlebars.register_template_string
  (
   "appropraite_branch_for",
   include_str!( "../../template/workflow/appropraite_branch_for.hbs" )
 )?;
  handlebars.register_template_string
  (
   "standard_rust_pull_request",
   include_str!( "../../template/workflow/standard_rust_pull_request.hbs" )
 )?;

  dbg!( &workflow_root );

  // workspace_push.yml - dynamic matrix for all crates (replaces per-crate module_*_push.yml)
  file_write
  (
   &workflow_root.join( "workspace_push.yml" ),
   include_str!( "../../template/workflow/workspace_push.yml" )
 )?;

  file_write
  (
   &workflow_root.join( "appropriate_branch.yml" ),
   include_str!( "../../template/workflow/appropriate_branch.yml" )
 )?;

  // appropriate_branch_master.yml - validates PRs to master originate from alpha
  let data = map_prepare_for_appropriative_branch
  (
   "- main\n      - master",
   username_and_repository.0.as_str(),
   "alpha",
   "alpha",
   "master"
 );

  file_write
  (
   &workflow_root.join( "appropriate_branch_master.yml" ),
   &handlebars.render( "appropraite_branch_for", &data )?
 )?;

  file_write
  (
   &workflow_root.join( "auto_pr.yml" ),
   include_str!( "../../template/workflow/auto_pr.yml" )
 )?;

  let mut data = BTreeMap ::new();
  data.insert( "name", "alpha" );
  data.insert
  (
   "branches",
   " - '*'
   - '*/*'
   - '**'
   - '!master'
   - '!main'
   - '!alpha'
   - '!*test*'
   - '!*test*/*'
   - '!*/*test*'
   - '!*experiment*'
   - '!*experiment*/*'
   - '!*/*experiment*'"
 );
  data.insert( "username_and_repository", username_and_repository.0.as_str() );
  data.insert( "uses_branch", "alpha" );
  data.insert( "src_branch", "${{ github.ref_name }}" );
  data.insert( "dest_branch", "alpha" );

  file_write
  (
   &workflow_root.join( "auto_pr_to_alpha.yml" ),
   &handlebars.render( "auto_pr_to", &data )?
 )?;

  let mut data = BTreeMap ::new();
  data.insert( "name", "master" );
  data.insert( "branches",  "- alpha" );
  data.insert( "username_and_repository", username_and_repository.0.as_str() );
  data.insert( "uses_branch", "alpha" );
  data.insert( "src_branch", "alpha" );
  data.insert( "dest_branch", "master" );

  file_write
  (
   &workflow_root.join( "auto_pr_to_master.yml" ),
   &handlebars.render( "auto_pr_to", &data )?
 )?;

  file_write
  (
   &workflow_root.join( "runs_clean.yml" ),
   include_str!( "../../template/workflow/rust_clean.yml" )
 )?;

  let mut data = BTreeMap ::new();
  data.insert( "username_and_repository", username_and_repository.0.as_str() );

  file_write
  (
   &workflow_root.join( "standard_rust_pull_request.yml" ),
   &handlebars.render( "standard_rust_pull_request", &data )?
 )?;

  file_write
  (
   &workflow_root.join( "standard_rust_push.yml" ),
   include_str!( "../../template/workflow/standard_rust_push.yml" )
 )?;

  file_write
  (
   &workflow_root.join( "for_pr_rust_push.yml" ),
   include_str!( "../../template/workflow/for_pr_rust_push.yml" )
 )?;

  file_write
  (
   &workflow_root.join( "readme.md" ),
   include_str!( "../../template/workflow/readme.md" )
 )?;

  Ok :: < _, CiCdGenerateError >( () )
 }

  /// Prepare params for render `appropriative_branch_for` template.
  fn map_prepare_for_appropriative_branch< 'a >
  (
  branches: &'a str,
  username_and_repository: &'a str,
  uses_branch: &'a str,
  src_branch: &'a str,
  name: &'a str
 )
  -> BTreeMap< &'a str, &'a str >
  {
  let mut data = BTreeMap ::new();
  data.insert( "branches", branches );
  data.insert( "username_and_repository", username_and_repository );
  data.insert( "uses_branch", uses_branch );
  data.insert( "src_branch", src_branch );
  data.insert( "name", name );
  data
 }

  /// Create and write or rewrite content in file.
  pub fn file_write( filename: &Path, content: &str ) -> error ::untyped ::Result< () >
  {
  if let Some( folder ) = filename.parent()
  {
   match std ::fs ::create_dir_all( folder )
   {
  Ok( () ) => {},
  Err( e ) if e.kind() == std ::io ::ErrorKind ::AlreadyExists => {},
  Err( e ) => return Err( e.into() ),
 }
 }

  let mut file = File ::create( filename )?;
  file.write_all( content.as_bytes() )?;
  Ok( () )
 }

  #[ derive( Debug ) ]
  struct UsernameAndRepository( String );

  /// Searches and extracts the username and repository name from the repository URL.
  /// The repository URL is first sought in the Cargo.toml file of the workspace;
  /// if not found there, it is then searched in the Cargo.toml file of the module.
  /// If it is still not found, the search continues in the GitHub remotes.
  /// Result looks like this: `Wandalen/wTools`
  fn username_and_repository< 'a >
  (
  cargo_toml_path: &AbsolutePath,
  packages: impl Iterator< Item = WorkspacePackageRef< 'a > >,
 )
  -> error ::untyped ::Result< UsernameAndRepository >
  {
   let mut contents = String ::new();
   File ::open( cargo_toml_path )?.read_to_string( &mut contents )?;
   let doc = contents.parse :: < Document >()?;
   let url =
   doc
   .get( "workspace" )
   .and_then( | workspace  | workspace.get( "metadata" ) )
   .and_then( | metadata | metadata.get( "repo_url" ) )
   .and_then( | url | url.as_str() )
   .map( String ::from );
   if let Some( url ) = url
   {
  url ::repo_url_extract( &url )
  .and_then( | url | url ::git_info_extract( &url ).ok() )
  .map( UsernameAndRepository )
  .ok_or_else( || error ::untyped ::format_err!( "Fail to parse repository url from workspace Cargo.toml") )
 }
   else
   {
  let mut url = None;
  for package in packages
  {
   // if let Ok( wu ) = manifest ::private ::repo_url( package.manifest_file().parent().unwrap().as_std_path() )
   if let Ok( wu ) = manifest ::repo_url( &package.crate_dir()? )
   {
  url = Some( wu );
  break;
 }
 }
  url
  .as_ref()
  .and_then( | url | url ::repo_url_extract( url ) )
  .and_then( | url | url ::git_info_extract( &url ).ok() )
  .map( UsernameAndRepository )
  .ok_or_else( || error ::untyped ::format_err!( "Fail to extract repository url") )
 }
 }

}

crate ::mod_interface!
{
  own use action;
}
