mod private
{
  use crate::*;
  use std::collections::BTreeMap;
  use std::fs;
  use std::io::Write;
  use std::path::Path;
  use handlebars::no_escape;
  use error_tools::for_app::bail;
  use error_tools::Result;
  use wtools::iter::Itertools;
  use crate::template::{Template, TemplateFileDescriptor, TemplateFiles, TemplateFilesBuilder, TemplateParameters, TemplateValues};

  /// Template for creating deploy files.
  ///
  /// Includes terraform deploy options to GCP, and Hetzner,
  /// a Makefile for useful commands, and a key directory.
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
  }

  impl Default for WorkspaceTemplate
  {
    fn default() -> Self
    {
      Self
      {
        files : Default::default(),
        parameters : TemplateParameters::new
          (
            &
              [
                "project_name",
                "url",
                "branches",
              ]
          ),
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
        .file().data( include_str!( "../../template/workspace/Makefile" ) ).path( "./Makefile" ).is_template( true ).end()
        .file().data( include_str!( "../../template/workspace/Readme.md" ) ).path( "./Makefile" ).is_template( true ).end()

        .file().data( include_str!( "../../template/workspace/.cargo/config.toml" ) ).path( "./.cargo/config.toml" ).end()
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
  // qqq : parametrized templates??
  /// Creates workspace template
  pub fn workspace_renew( path : &Path, repository_url : String, branches : Vec< String > ) -> Result< () >
  {
    if fs::read_dir( path )?.count() != 0
    {
      bail!( "Directory should be empty" )
    }
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_escape_fn( no_escape );
    let branches = branches.into_iter().map( | b | format!( r#""{}""#, b ) ).join( ", " );
    let data = BTreeMap::from_iter
    (
      [
        ( "project_name", path.file_name().unwrap().to_string_lossy() ),
        ( "url", repository_url.into() ),
        ( "branches", branches.into() ),
      ]
    );
    handlebars.register_template_string( "cargo_toml", include_str!( "../../template/workspace/Cargo.hbs" ) )?;
    let cargo_toml = &handlebars.render( "cargo_toml", &data )?;

    create_file( path, "Cargo.toml", cargo_toml )?;

    dot_cargo( &path )?;
    // dot_circleci( &path )?;
    dot_github( &path )?;
    static_dirs( &path )?;
    static_files( &path )?;
    module1( &path )?;
    Ok( () )
  }

  fn module1( path : &Path ) -> Result< () >
  {
    create_dir( path, "module" )?;
    create_dir( &path.join( "module" ), "module1" )?;
    create_file( &path.join( "module" ).join( "module1" ), "Cargo.toml", include_str!( "../../template/workspace/module/module1/Cargo.toml.x" ) )?;
    create_file( &path.join( "module" ).join( "module1" ), "Readme.md", include_str!( "../../template/workspace/module/module1/Readme.md" ) )?;
    create_dir( &path.join( "module" ).join( "module1" ), "examples" )?;
    create_dir( &path.join( "module" ).join( "module1" ), "src" )?;
    create_dir( &path.join( "module" ).join( "module1" ), "tests" )?;
    create_file( &path.join( "module" ).join( "module1" ).join( "examples" ), "module1_trivial_sample.rs", include_str!( "../../template/workspace/module/module1/examples/module1_example.rs" ) )?;
    create_file( &path.join( "module" ).join( "module1" ).join( "src" ), "lib.rs", include_str!( "../../template/workspace/module/module1/src/lib.rs" ) )?;
    create_file( &path.join( "module" ).join( "module1" ).join( "tests" ), "hello_test.rs", include_str!( "../../template/workspace/module/module1/tests/hello_test.rs" ) )?;

    Ok( () )
  }

  fn static_files( path : &Path ) -> Result< () >
  {
    create_file( path, "Readme.md", include_str!( "../../template/workspace/Readme.md" ) )?;
    create_file( path, ".gitattributes", include_str!( "../../template/workspace/.gitattributes" ) )?;
    create_file( path, ".gitignore", include_str!( "../../template/workspace/.gitignore1" ) )?;
    create_file( path, ".gitpod.yml", include_str!( "../../template/workspace/.gitpod.yml" ) )?;
    create_file( path, "Makefile", include_str!( "../../template/workspace/Makefile" ) )?;

    Ok( () )
  }

  fn static_dirs( path : &Path ) -> Result< () >
  {
    create_dir( path, "assets" )?;
    create_dir( path, "docs" )?;

    Ok( () )
  }

  fn dot_github( path : &Path ) -> Result< () >
  {
    create_dir( path, ".github" )?;
    create_dir( &path.join( ".github" ), "workflows" )?;

    Ok( () )
  }

//   fn dot_circleci( path : &Path ) -> Result< () >
//   {
//     create_dir( path, ".circleci" )?;
//     create_file( &path.join( ".circleci" ), "config.yml", include_str!( "../../template/workspace/.circleci1/config.yml" ) )?;
//
//     Ok( () )
//   }

  fn dot_cargo( path : &Path ) -> Result< () >
  {
    create_dir( path, ".cargo" )?;
    create_file( &path.join( ".cargo" ), "config.toml", include_str!( "../../template/workspace/.cargo/config.toml" ) )?;

    Ok( () )
  }

  fn create_dir( path : &Path, name : &str ) -> Result< () >
  {
    fs::create_dir( path.join( name ) )?;
    Ok( () )
  }

  fn create_file( path : &Path, name : &str, content : &str ) -> Result< () >
  {
    let mut file = fs::File::create( path.join( name ) )?;
    file.write_all( content.as_bytes() )?;
    Ok( () )
  }
}

crate::mod_interface!
{
  exposed use workspace_renew;
}
