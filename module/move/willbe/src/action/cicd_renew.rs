mod private
{
  use crate::*;

  use std::
  {
    fs::File,
    io::{ Write, Read },
  };

  use path::{ Path };
  use collection::BTreeMap;
  use convert_case::{ Casing, Case };
  use handlebars::{ RenderError, TemplateError };
  use toml_edit::Document;

  use entity::PathError;
  use error::typed::Error;

  use error::untyped::{ Result, Error as wError };
  use entity::WorkspaceInitError;
  use error::err;

  #[ derive( Debug, Error ) ]
  pub enum CiCdGenerateError
  {
    #[ error( "Common error: {0}" ) ]
    Common(#[ from ] wError ),
    #[ error( "I/O error: {0}" ) ]
    IO( #[ from ] std::io::Error ),
    #[ error( "Crate directory error: {0}" ) ]
    CrateDir( #[ from ] PathError ),
    #[ error( "Workspace error: {0}" ) ]
    Workspace( #[ from ] WorkspaceInitError ),
    #[ error( "Template error: {0}" ) ]
    Template( #[ from ] TemplateError ),
    #[ error( "Render error: {0}" ) ]
    Render( #[ from ] RenderError ),
  }

  // qqq : for Petro : should return Report and typed error in Result
  /// Generate workflows for modules in .github/workflows directory.
  pub fn cicd_renew( base_path : &Path ) -> Result< (), CiCdGenerateError >
  {
    let workspace_cache = Workspace::try_from( CrateDir::try_from( base_path )? )?;
    let packages = workspace_cache.packages();
    let username_and_repository = &username_and_repository
    (
      &workspace_cache.workspace_root().join( "Cargo.toml" ).to_path_buf().try_into()?, // qqq
      packages.clone(),
      // packages.as_slice(),
    )?;
    let workspace_root : &Path = &workspace_cache.workspace_root();
    // find directory for workflows
    let workflow_root = workspace_root.join( ".github" ).join( "workflows" );
    // map packages name's to naming standard
    // let names = packages.map( | p | p.name() ).collect::< Vec< _ > >();
    let names = packages.clone().map( | p | p.name().to_string() );

    // dbg!( &workflow_root );

    // map packages path to relative paths fom workspace root,
    // for example D:/work/wTools/module/core/iter_tools => module/core/iter_tools
    let relative_paths = packages
    .map( | p | p.manifest_file().ok() ) // aaa : rid of unwrap : removed
    .filter_map( | p |
    {
      // dbg!( &workspace_root );
      Some( path::normalize( workspace_root ) ).and_then( | root_str |
      {

        dbg!( &root_str );
        dbg!( &p );

        Some( p?.strip_prefix( root_str ).ok()?.to_path_buf() )
        //.map( | s | s.display().to_string() ).ok()
      })
    })
    .map( | p |
    {
      path::normalize( p.parent().unwrap() )
      // dbg!( &p );
      // let mut path = PathBuf::from( p );
      // path.set_file_name( "" );
      // path
    });

    // preparing templates
    let mut handlebars = handlebars::Handlebars::new();

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
      "auto_merge_to",
      include_str!( "../../template/workflow/auto_merge_to.hbs" )
    )?;
    handlebars.register_template_string
    (
      "standard_rust_pull_request",
      include_str!( "../../template/workflow/standard_rust_pull_request.hbs" )
    )?;
    handlebars.register_template_string
    (
      "module_push",
      include_str!( "../../template/workflow/module_push.hbs" )
    )?;

    // qqq : for Petro : instead of iterating each file manually, iterate each file in loop

    // use similar::DiffableStr;

    // creating workflow for each module
    for ( name, relative_path ) in names.zip( relative_paths )
    {
      // generate file names
      let workflow_file_name = workflow_root
      .join( format!( "module_{}_push.yml", name.to_case( Case::Snake ) ) );
      let manifest_file = relative_path.join( "Cargo.toml" );
      let mut data : BTreeMap< &str, &str > = BTreeMap::new();
      data.insert( "name", name.as_str() );
      data.insert( "username_and_repository", username_and_repository.0.as_str() );
      data.insert( "branch", "alpha" );
      let manifest_file = manifest_file.to_string_lossy().replace( "\\", "/" );
      let manifest_file = manifest_file.trim_start_matches( '/' );
      data.insert( "manifest_path", manifest_file );
      let content = handlebars.render( "module_push", &data )?;
      file_write( &workflow_file_name, &content )?;

      println!( "file_write : {:?}", &workflow_file_name )
    }

    dbg!( &workflow_root );

    file_write
    (
      &workflow_root
      .join("appropriate_branch.yml" ),
      include_str!( "../../template/workflow/appropriate_branch.yml" )
    )?;

    let data = map_prepare_for_appropriative_branch
    (
      "- beta",
      username_and_repository.0.as_str(),
      "alpha",
      "alpha",
      "beta"
    );
    file_write
    (
      &workflow_root.join( "appropriate_branch_beta.yml" ),
      &handlebars.render( "appropraite_branch_for", &data )?
    )?;

    let data = map_prepare_for_appropriative_branch
    (
      "- main\n      - master",
      username_and_repository.0.as_str(),
      "alpha",
      "beta",
      "master"
    );

    file_write
    (
      &workflow_root.join( "appropriate_branch_master.yml" ),
      &handlebars.render( "appropraite_branch_for", &data )?
    )?;

    let mut data = BTreeMap::new();
    data.insert( "name", "beta" );
    data.insert( "group_branch", "beta" );
    data.insert( "branch", "alpha" );

    file_write
    (
      &workflow_root.join( "auto_merge_to_beta.yml" ),
      &handlebars.render( "auto_merge_to", &data )?
    )?;
    file_write
    (
      &workflow_root.join( "auto_pr.yml" ),
      include_str!( "../../template/workflow/auto_pr.yml" )
    )?;

    let mut data = BTreeMap::new();
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
      - '!beta'
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

    let mut data = BTreeMap::new();
    data.insert( "name", "beta" );
    data.insert( "branches",  "- alpha" );
    data.insert( "username_and_repository", username_and_repository.0.as_str() );
    data.insert( "uses_branch", "alpha" );
    data.insert( "src_branch", "alpha" );
    data.insert( "dest_branch", "beta" );

    file_write
    (
      &workflow_root.join( "auto_pr_to_beta.yml" ),
      &handlebars.render( "auto_pr_to", &data )?
    )?;

    let mut data = BTreeMap::new();
    data.insert( "name", "master" );
    data.insert( "branches",  "- beta" );
    data.insert( "username_and_repository", username_and_repository.0.as_str() );
    data.insert( "uses_branch", "alpha" );
    data.insert( "src_branch", "beta" );
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

    let mut data = BTreeMap::new();
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
      &workflow_root.join( "standard_rust_scheduled.yml" ),
      include_str!( "../../template/workflow/standard_rust_scheduled.yml" )
    )?;

    file_write
    (
      &workflow_root.join( "standard_rust_status.yml" ),
      include_str!( "../../template/workflow/standard_rust_status.yml" )
    )?;

    file_write
    (
      &workflow_root.join( "status_checks_rules_update.yml" ),
      include_str!( "../../template/workflow/status_checks_rules_update.yml" )
    )?;

    file_write
    (
      &workflow_root.join( "Readme.md" ),
      include_str!( "../../template/workflow/Readme.md" )
    )?;

    Ok( () )
  }

  /// Prepare params for render appropriative_branch_for template.
  fn map_prepare_for_appropriative_branch< 'a >
  (
    branches : &'a str,
    username_and_repository : &'a str,
    uses_branch : &'a str,
    src_branch : &'a str,
    name : &'a str
  )
  -> BTreeMap< &'a str, &'a str >
  {
    let mut data = BTreeMap::new();
    data.insert( "branches", branches );
    data.insert( "username_and_repository", username_and_repository );
    data.insert( "uses_branch", uses_branch );
    data.insert( "src_branch", src_branch );
    data.insert( "name", name );
    data
  }

  /// Create and write or rewrite content in file.
  pub fn file_write( filename : &Path, content : &str ) -> Result< () >
  {
    if let Some( folder ) = filename.parent()
    {
      match std::fs::create_dir_all( folder )
      {
        Ok( _ ) => {},
        Err( e ) if e.kind() == std::io::ErrorKind::AlreadyExists => {},
        Err( e ) => return Err( e.into() ),
      }
    }

    let mut file = File::create( filename )?;
    file.write_all( content.as_bytes() )?;
    Ok( () )
  }

  #[derive( Debug ) ]
  struct UsernameAndRepository( String );

  /// Searches and extracts the username and repository name from the repository URL.
  /// The repository URL is first sought in the Cargo.toml file of the workspace;
  /// if not found there, it is then searched in the Cargo.toml file of the module.
  /// If it is still not found, the search continues in the GitHub remotes.
  /// Result looks like this: `Wandalen/wTools`
  fn username_and_repository< 'a >
  (
    cargo_toml_path : &AbsolutePath,
    packages : impl Iterator< Item = WorkspacePackageRef< 'a > >,
  )
  -> Result< UsernameAndRepository >
  {
      let mut contents = String::new();
      File::open( cargo_toml_path )?.read_to_string( &mut contents )?;
      let doc = contents.parse::< Document >()?;
      let url =
      doc
      .get( "workspace" )
      .and_then( | workspace  | workspace.get( "metadata" ) )
      .and_then( | metadata | metadata.get( "repo_url" ) )
      .and_then( | url | url.as_str() )
      .map( String::from );
      if let Some( url ) = url
      {
        return url::repo_url_extract( &url )
        .and_then( | url | url::git_info_extract( &url ).ok() )
        .map( UsernameAndRepository )
        .ok_or_else( || err!( "Fail to parse repository url from workspace Cargo.toml"))
      }
      else
      {
        let mut url = None;
        for package in packages
        {
          // if let Ok( wu ) = manifest::private::repo_url( package.manifest_file().parent().unwrap().as_std_path() )
          if let Ok( wu ) = manifest::repo_url( &package.crate_dir()? )
          {
            url = Some( wu );
            break;
          }
        }
        return url
        .and_then( | url | url::repo_url_extract( &url ) )
        .and_then( | url | url::git_info_extract( &url ).ok() )
        .map( UsernameAndRepository )
        .ok_or_else( || err!( "Fail to extract repository url") )
      }
    }

}

crate::mod_interface!
{
  exposed use cicd_renew;
}
