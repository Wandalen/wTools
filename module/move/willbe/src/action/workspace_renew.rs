#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{

  use crate :: *;
  use std ::
  {
  fs,
  path :: { Path, PathBuf },
 };
  use error ::untyped ::bail;
  // qqq: group dependencies
  use iter_tools ::iter ::Itertools;
  use genfile_core ::
  {
  TemplateArchive,
  WriteMode,
  Value,
  ParameterDescriptor,
  Parameters,
 };

  /// Template for creating workspace files.
  #[ derive( Debug ) ]
  pub struct WorkspaceTemplate
  {
  archive: TemplateArchive,
 }

  impl WorkspaceTemplate
  {
  /// Returns list of parameter names
  #[ must_use ]
  pub fn get_parameters( &self ) -> Vec< String >
  {
   self.archive.parameters().descriptors().iter().map( | d | d.name().to_string() ).collect()
 }
 }

  impl Default for WorkspaceTemplate
  {
  fn default() -> Self
  {
   let mut archive = TemplateArchive ::new( "workspace" );

   // Define parameters
   archive.add_parameter
   (
  ParameterDescriptor ::new( "project_name" )
  .with_mandatory( true )
 );
   archive.add_parameter
   (
  ParameterDescriptor ::new( "url" )
  .with_mandatory( true )
 );
   archive.add_parameter
   (
  ParameterDescriptor ::new( "branches" )
  .with_mandatory( true )
 );

   // Add template files
   archive.add_text_file
   (
  PathBuf ::from( "./.gitattributes" ),
  include_str!( "../../template/workspace/.gitattributes" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./.gitignore" ),
  include_str!( "../../template/workspace/.gitignore1" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./.gitpod.yml" ),
  include_str!( "../../template/workspace/.gitpod.yml" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./Cargo.toml" ),
  include_str!( "../../template/workspace/Cargo.hbs" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./Makefile" ),
  include_str!( "../../template/workspace/Makefile" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./readme.md" ),
  include_str!( "../../template/workspace/readme.md" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./.cargo/config.toml" ),
  include_str!( "../../template/workspace/.cargo/config.toml" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./module/Cargo.toml" ),
  include_str!( "../../template/workspace/module/module1/Cargo.toml.x" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./module/module1/readme.md" ),
  include_str!( "../../template/workspace/module/module1/readme.md" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./module/module1/examples/module1_example.rs" ),
  include_str!( "../../template/workspace/module/module1/examples/module1_example.rs" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./module/module1/src/lib.rs" ),
  include_str!( "../../template/workspace/module/module1/src/lib.rs" ),
  WriteMode ::Rewrite
 );
   archive.add_text_file
   (
  PathBuf ::from( "./module/module1/tests/hello_test.rs" ),
  include_str!( "../../template/workspace/module/module1/tests/hello_test.rs" ),
  WriteMode ::Rewrite
 );

   Self { archive }
 }
 }

  // zzz
  // qqq: for Petro: should return report
  // qqq: for Petro: should have typed error
  /// Creates workspace template
  /// # Errors
  /// qqq: doc
  /// # Panics
  /// qqq: doc
  pub fn action
  (
  path: &Path,
  mut template: WorkspaceTemplate,
  repository_url: String,
  branches: Vec< String >
 )
  -> error ::untyped ::Result< () > // qqq: use typed error
  {
  if fs ::read_dir( path )?.count() != 0
  {
   bail!( "Directory should be empty" )
 }

  // Set parameter values
  let project_name = path.file_name().unwrap().to_string_lossy().to_string();
  template.archive.set_value( "project_name", Value ::String( project_name ) );
  template.archive.set_value( "url", Value ::String( repository_url ) );

  let branches_str = branches.into_iter().map( | b | format!( r#""{b}""# ) ).join( ", " );
  template.archive.set_value( "branches", Value ::String( branches_str ) );

  // Materialize the template
  template.archive.materialize( path ).map_err( | e | error ::untyped ::format_err!( "{}", e ) )?;

  Ok( () )
 }
}

crate ::mod_interface!
{
  own use action;
  orphan use WorkspaceTemplate;
}
