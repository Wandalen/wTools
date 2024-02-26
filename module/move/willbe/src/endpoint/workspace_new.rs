mod private
{
  use std::collections::BTreeMap;
  use std::fs;
  use std::path::Path;
  use error_tools::for_app::bail;
  use error_tools::Result;
  use crate::files::{Content, write_to_file};
  use crate::git;

  struct WorkspaceStructure
  {
    //complicated files
    dot_cargo : Content,
    dot_github : Content,
    module : Content,
    //static files
    readmemd : String,
    gitattributes : String,
    gitignore : String,
    gitpod : String,
    makefile : String,
    //static dirs
    assets : Content,
    docs : Content,
    //files
    cargo_toml: String,
  }
  
  impl WorkspaceStructure
  {
    fn into_iter( self ) -> impl IntoIterator< Item = Content >
    {
      let Self
      {
        dot_cargo,
        dot_github,
        module,
        readmemd,
        gitattributes,
        gitignore,
        gitpod,
        makefile,
        assets,
        docs,
        cargo_toml,
      } = self;
      
      [
        dot_cargo,
        dot_github,
        module,
        Content::File { name : "Readme.md".to_string(), content : readmemd },
        Content::File { name : ".gitattributes".to_string(), content : gitattributes },
        Content::File { name : ".gitignore".to_string(), content : gitignore },
        Content::File { name : ".gitpod.yml".to_string(), content : gitpod },
        Content::File { name : "Makefile".to_string(), content : makefile },
        assets,
        docs,
        Content::File { name : "Cargo.toml".to_string(), content : cargo_toml },
      ]
    }
  }
  

  // qqq : for Petro : should return report
  // qqq : for Petro : should have typed error
  // qqq : parametrized templates??
  /// Creates workspace template
  pub fn workspace_new( path : &Path ) -> Result< () >
  {
    if fs::read_dir( path )?.count() != 0
    {
      bail!( "Directory should be empty" )
    }
    
    let mut handlebars = handlebars::Handlebars::new();
    let data = BTreeMap::from_iter( [ ( "project_name", path.file_name().unwrap().to_string_lossy() ) ] );
    handlebars.register_template_string( "cargo_toml", include_str!( "../../template/workspace/Cargo.hbs" ) )?;
    
    let cargo_toml = handlebars.render( "cargo_toml", &data )?;
    
    let structure = WorkspaceStructure
    {
      dot_cargo: dot_cargo(),
      dot_github: dot_github(),
      module: Content::Directory { name: "module".to_string(), content: vec![ module1() ] },
      readmemd: include_str!( "../../template/workspace/Readme.md" ).to_string(),
      gitattributes: include_str!( "../../template/workspace/.gitattributes" ).to_string(),
      gitignore: include_str!( "../../template/workspace/.gitignore1" ).to_string(),
      gitpod: include_str!( "../../template/workspace/.gitpod.yml" ).to_string(),
      makefile: include_str!( "../../template/workspace/Makefile" ).to_string(),
      assets: Content::Directory { name: "assets".to_string(), content: vec![] },
      docs: Content::Directory { name: "docs".to_string(), content: vec![] },
      cargo_toml,
    };
    
    write_to_file( path, structure.into_iter() )?;
    
    git::init( path )?;
    
    Ok( () )
  }

  fn module1() -> Content
  {
   Content::Directory
    { 
      name: "module1".to_string(), 
      content: vec!
      [
        Content::File{ name: "Cargo.toml".to_string(), content: include_str!( "../../template/workspace/module/module1/Cargo.toml" ).to_string() },
        Content::File{ name: "Readme.md".to_string(), content: include_str!( "../../template/workspace/module/module1/Readme.md" ).to_string() },
        Content::Directory 
        { 
          name: "examples".to_string(), 
          content: vec!
          [ 
            Content::File 
            { 
              name: "module1_trivial_sample.rs".to_string(), 
              content: include_str!( "../../template/workspace/module/module1/examples/module1_example.rs" ).to_string() 
            }
          ] 
        },
        Content::Directory
        {
          name: "src".to_string(),
          content: vec!
          [
            Content::File
            {
              name: "lib.rs".to_string(),
              content: include_str!( "../../template/workspace/module/module1/src/lib.rs" ).to_string()
            }
          ]
        },
        Content::Directory
        {
          name: "tests".to_string(),
          content: vec!
          [
            Content::File
            {
              name: "hello_test.rs".to_string(),
              content: include_str!( "../../template/workspace/module/module1/tests/hello_test.rs" ).to_string()
            }
          ]
        },
      ] 
    }
  }
  
  fn dot_github() -> Content
  {
    Content::Directory
    { 
      name: ".github".to_string(), 
      content: vec!
      [
        Content::Directory 
        { 
          name: "workflows".to_string(), content: vec![] 
        }
      ] 
    }
  }

//   fn dot_circleci( path : &Path ) -> Result< () >
//   {
//     create_dir( path, ".circleci" )?;
//     create_file( &path.join( ".circleci" ), "config.yml", include_str!( "../../template/workspace/.circleci1/config.yml" ) )?;
//
//     Ok( () )
//   }

  fn dot_cargo() -> Content
  {
    Content::Directory
    { 
      name: ".cargo".to_string(), 
      content: vec!
      [ 
        Content::File 
        { 
          name: "config.toml".to_string(), 
          content: include_str!( "../../template/workspace/.cargo/config.toml" ).to_string()
        }
      ] 
    }
  }
  
}

crate::mod_interface!
{
  exposed use workspace_new;
}
