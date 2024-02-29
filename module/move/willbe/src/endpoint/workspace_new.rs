mod private
{
  use crate::*;
  use std::collections::BTreeMap;
  use std::fs;
  use std::io::Write;
  use std::path::Path;
  use handlebars::{no_escape, RenderError, TemplateError};
  use error_tools::for_lib::Error;
  use error_tools::dependency::*;
  use wtools::iter::Itertools;
  
  #[ derive( Debug, Error ) ]
  pub enum WorkspaceNewError
  {
    #[ error ( "Directory should be empty" ) ]
    NonEmptyDirectory,
    #[ error( "I/O error: {0}" ) ]
    IO(#[ from ] std::io::Error ),
    #[ error( "Template error: {0}") ]
    Template( #[ from ] TemplateError ),
    #[ error( "Render error: {0}" ) ]
    Render( #[ from ] RenderError ),
  }
  

  // qqq : for Petro : should return report
  // qqq : for Petro : should have typed error
  // qqq : parametrized templates??
  /// Creates workspace template
  pub fn workspace_new( path : &Path, repository_url : String, branches: Vec< String > ) -> Result< (), WorkspaceNewError >
  {
    if fs::read_dir( path )?.count() != 0
    {
      return Err( WorkspaceNewError::NonEmptyDirectory )
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

  fn module1( path : &Path ) -> std::io::Result< () >
  {
    create_dir( path, "module" )?;
    create_dir( &path.join( "module" ), "module1" )?;
    create_file( &path.join( "module" ).join( "module1" ), "Cargo.toml", include_str!( "../../template/workspace/module/module1/Cargo.toml" ) )?;
    create_file( &path.join( "module" ).join( "module1" ), "Readme.md", include_str!( "../../template/workspace/module/module1/Readme.md" ) )?;
    create_dir( &path.join( "module" ).join( "module1" ), "examples" )?;
    create_dir( &path.join( "module" ).join( "module1" ), "src" )?;
    create_dir( &path.join( "module" ).join( "module1" ), "tests" )?;
    create_file( &path.join( "module" ).join( "module1" ).join( "examples" ), "module1_trivial_sample.rs", include_str!( "../../template/workspace/module/module1/examples/module1_example.rs" ) )?;
    create_file( &path.join( "module" ).join( "module1" ).join( "src" ), "lib.rs", include_str!( "../../template/workspace/module/module1/src/lib.rs" ) )?;
    create_file( &path.join( "module" ).join( "module1" ).join( "tests" ), "hello_test.rs", include_str!( "../../template/workspace/module/module1/tests/hello_test.rs" ) )?;

    Ok( () )
  }

  fn static_files( path : &Path ) -> std::io::Result< () >
  {
    create_file( path, "Readme.md", include_str!( "../../template/workspace/Readme.md" ) )?;
    create_file( path, ".gitattributes", include_str!( "../../template/workspace/.gitattributes" ) )?;
    create_file( path, ".gitignore", include_str!( "../../template/workspace/.gitignore1" ) )?;
    create_file( path, ".gitpod.yml", include_str!( "../../template/workspace/.gitpod.yml" ) )?;
    create_file( path, "Makefile", include_str!( "../../template/workspace/Makefile" ) )?;

    Ok( () )
  }

  fn static_dirs( path : &Path ) -> std::io::Result< () >
  {
    create_dir( path, "assets" )?;
    create_dir( path, "docs" )?;

    Ok( () )
  }

  fn dot_github( path : &Path ) -> std::io::Result< () >
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

  fn dot_cargo( path : &Path ) -> std::io::Result< () >
  {
    create_dir( path, ".cargo" )?;
    create_file( &path.join( ".cargo" ), "config.toml", include_str!( "../../template/workspace/.cargo/config.toml" ) )?;

    Ok( () )
  }

  fn create_dir( path : &Path, name : &str ) -> std::io::Result< () >
  {
    fs::create_dir( path.join( name ) )?;
    Ok( () )
  }

  fn create_file( path : &Path, name : &str, content : &str ) -> std::io::Result< () >
  {
    let mut file = fs::File::create( path.join( name ) )?;
    file.write_all( content.as_bytes() )?;
    Ok( () )
  }
}

crate::mod_interface!
{
  exposed use workspace_new;
}
