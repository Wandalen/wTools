mod private
{
  use crate::*;
  use std::fs;
  use std::path::Path;
  use error_tools::for_app::bail;
  use error_tools::Result;
  use wtools::iter::Itertools;
  use crate::template::{Template, TemplateFileDescriptor, TemplateFiles, TemplateFilesBuilder, TemplateParameters, TemplateValues};

  /// Template for creating workspace files.
  #[ derive( Debug ) ]
  pub struct WorkspaceTemplate
  {
    files : WorkspaceTemplateFiles,
    parameters : TemplateParameters,
    values : TemplateValues,
  }

  impl Template<WorkspaceTemplateFiles> for WorkspaceTemplate
  {
    fn create_all( self, path : &Path ) -> Result< () >
    {
      self.files.create_all( path, &self.values )
    }

    fn parameters( &self ) -> &TemplateParameters
    {
      &self.parameters
    }

    fn set_values( &mut self, values : TemplateValues )
    {
      self.values = values
    }

    fn parameter_storage( &self ) -> &Path
    {
      "./.workspace_template.toml".as_ref()
    }

    fn template_name( &self ) -> &'static str
    {
      "workspace"
    }

    fn get_values( &self ) -> &TemplateValues
    {
      &self.values
    }

    fn get_values_mut( &mut self ) -> &mut TemplateValues
    {
      &mut self.values
    }


  }

  impl Default for WorkspaceTemplate
  {
    fn default() -> Self
    {
      let parameters = TemplateParameters::former()
      .parameter( "project_name" ).is_mandatory( true ).end()
      .parameter( "url" ).is_mandatory( true ).end()
      .parameter( "branches" ).is_mandatory( true ).end()
      .form();
      Self
      {
        files : Default::default(),
        parameters,
        values : Default::default(),
      }
    }
  }

  /// Files for the deploy template.
  ///
  /// Default implementation contains all required files.
  #[ derive( Debug ) ]
  pub struct WorkspaceTemplateFiles(Vec< TemplateFileDescriptor > );

  impl Default for WorkspaceTemplateFiles
  {
    fn default() -> Self
    {
      let formed = TemplateFilesBuilder::former()
      .file().data( include_str!( "../../template/workspace/.gitattributes" ) ).path( "./.gitattributes" ).end()
      .file().data( include_str!( "../../template/workspace/.gitignore1" ) ).path( "./.gitignore" ).end()
      .file().data( include_str!( "../../template/workspace/.gitpod.yml" ) ).path( "./.gitpod.yml" ).end()
      .file().data( include_str!( "../../template/workspace/Cargo.hbs" ) ).path( "./Cargo.toml" ).is_template( true ).end()
      .file().data( include_str!( "../../template/workspace/Makefile" ) ).path( "./Makefile" ).end()
      .file().data( include_str!( "../../template/workspace/Readme.md" ) ).path( "./Readme.md" ).end()
      .file().data( include_str!( "../../template/workspace/.cargo/config.toml" ) ).path( "./.cargo/config.toml" ).end()
      .file().data( include_str!( "../../template/workspace/module/module1/Cargo.toml.x" ) ).path( "./module/Cargo.toml" ).end()
      .file().data( include_str!( "../../template/workspace/module/module1/Readme.md" ) ).path( "./module/module1/Readme.md" ).end()
      .file().data( include_str!( "../../template/workspace/module/module1/examples/module1_example.rs" ) ).path( "./module/module1/examples/module1_example.rs" ).end()
      .file().data( include_str!( "../../template/workspace/module/module1/src/lib.rs" ) ).path( "./module/module1/src/lib.rs" ).end()
      .file().data( include_str!( "../../template/workspace/module/module1/tests/hello_test.rs" ) ).path( "./module/module1/tests/hello_test.rs" ).end()
      .form();

      Self( formed.files )
    }
  }

  impl TemplateFiles for WorkspaceTemplateFiles {}
  impl IntoIterator for WorkspaceTemplateFiles
  {
    type Item = TemplateFileDescriptor;

    type IntoIter = std::vec::IntoIter< Self::Item >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.0.into_iter()
    }
  }

  // qqq : for Petro : should return report
  // qqq : for Petro : should have typed error
  // aaa : parametrized templates??
  // aaa : use Viktor lib
  /// Creates workspace template
  pub fn workspace_renew( path : &Path, mut template : WorkspaceTemplate, repository_url : String, branches : Vec< String > ) -> Result< () >
  {
    if fs::read_dir(path)?.count() != 0
    {
      bail!( "Directory should be empty" )
    }
    template.values.insert_if_empty( "project_name", wca::Value::String( path.file_name().unwrap().to_string_lossy().into() ) );
    template.values.insert_if_empty( "url", wca::Value::String( repository_url ) );
    template.values.insert_if_empty( "branches", wca::Value::String( branches.into_iter().map( | b | format!( r#""{}""#, b ) ).join( ", " ) ) );
    template.create_all( path )?;
    Ok( () )
  }
}

crate::mod_interface!
{
  exposed use workspace_renew;
  orphan use WorkspaceTemplate;
}
