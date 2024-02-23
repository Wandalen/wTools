mod private
{
  use crate::*;

  use std::
  {
    path::Path,
    fs::File,
    io::{ Write, Read },
    collections::BTreeMap
  };

  use convert_case::{ Casing, Case };
  use toml_edit::Document;

  use wtools::error::for_app::{ Result, anyhow };
  use path::AbsolutePath;

  /// Generate workflows for modules in .github/workflows directory.
  pub fn workflow_generate( base_path: &Path ) -> Result< () >
  {
    let mut workspace_cache = Workspace::with_crate_dir( AbsolutePath::try_from( base_path )?.try_into()? )?;
    let username_and_repository = &username_and_repository( &mut workspace_cache )?;
    let workspace_root = workspace_cache.workspace_root()?;
    // find directory for workflows
    let workflow_root = workspace_root.join( ".github" ).join( "workflows" );
    // map packages name's to naming standard
    let names = workspace_cache.packages_get().and_then( | packages | Ok(packages.iter().map( | p | &p.name).collect::< Vec< _ > >()) )?;
    // map packages path to relative paths fom workspace root, for example D:/work/wTools/module/core/iter_tools => module/core/iter_tools
    let relative_paths = workspace_cache
    .packages_get()
    .map_err( | err | anyhow!( err ) )?
    .iter()
    .map( | p | &p.manifest_path )
    .filter_map( | p | p.strip_prefix( workspace_root ).ok() )
    .map( | p | p.with_file_name( "" ) )
    .collect::< Vec< _ > >();
    
    // preparing templates
    let mut handlebars = handlebars::Handlebars::new();

    handlebars.register_template_string( "auto_pr_to", include_str!( "../../files/workflow/auto_pr_to.hbs" ) )?;
    handlebars.register_template_string( "appropraite_branch_for", include_str!( "../../files/workflow/appropraite_branch_for.hbs" ) )?;
    handlebars.register_template_string( "auto_merge_to", include_str!( "../../files/workflow/auto_merge_to.hbs" ) )?;
    handlebars.register_template_string( "standard_rust_pull_request", include_str!( "../../files/workflow/standard_rust_pull_request.hbs" ) )?;
    handlebars.register_template_string( "module_push", include_str!( "../../files/workflow/module_push.hbs" ) )?;



    // creating workflow for each module
    for ( name, relative_path ) in names.iter().zip( relative_paths.iter() )
    {
      // generate file names
      let workflow_file_name = workflow_root.join( format!( "Module{}Push.yml", name.to_case( Case::Pascal ) ) );
      let path = relative_path.join( "Cargo.toml" );
      let mut data = BTreeMap::new();
      data.insert( "name", name.as_str() );
      data.insert( "username_and_repository", username_and_repository.as_str() );
      data.insert( "branch", "alpha" );
      let path = path.as_str().replace( "\\", "/" );
      data.insert( "manifest_path", path.as_str() );
      let content = handlebars.render( "module_push", &data )?;
      file_write( &workflow_file_name, &content )?;
    }
  
    file_write( &workflow_root.join( "AppropriateBranch.yml" ), include_str!( "../../files/workflow/appropriate_branch.yml" ) )?;

    let data = map_prepare_for_appropriative_branch( "- beta", username_and_repository, "alpha", "alpha", "beta" );
    file_write( &workflow_root.join( "AppropriateBranchBeta.yml" ), &handlebars.render( "appropraite_branch_for", &data )? )?;

    let data = map_prepare_for_appropriative_branch( "- main\n      - master", username_and_repository, "alpha", "beta", "master" );
    file_write( &workflow_root.join( "AppropriateBranchMaster.yml" ), &handlebars.render( "appropraite_branch_for", &data )? )?;

    let mut data = BTreeMap::new();
    data.insert( "name", "beta" );
    data.insert( "group_branch", "beta" );
    data.insert( "branch", "alpha" );

    file_write( &workflow_root.join( "AutoMergeToBeta.yml" ), &handlebars.render( "auto_merge_to", &data )? )?;

    file_write( &workflow_root.join( "AutoPr.yml" ), include_str!( "../../files/workflow/auto_pr.yml" ) )?;
    
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
    data.insert( "username_and_repository", username_and_repository.as_str() );
    data.insert( "uses_branch", "alpha" );
    data.insert( "src_branch", "${{ github.ref_name }}" );
    data.insert( "dest_branch", "alpha" );

    file_write( &workflow_root.join( "AutoPrToAlpha.yml" ), &handlebars.render( "auto_pr_to", &data )? )?;

    let mut data = BTreeMap::new();
    data.insert( "name", "beta" );
    data.insert( "branches",  "- alpha" ); 
    data.insert( "username_and_repository", username_and_repository.as_str() );
    data.insert( "uses_branch", "alpha" );
    data.insert( "src_branch", "alpha" );
    data.insert( "dest_branch", "beta" );

    file_write( &workflow_root.join( "AutoPrToBeta.yml" ), &handlebars.render( "auto_pr_to", &data )? )?;

    let mut data = BTreeMap::new();
    data.insert( "name", "master" );
    data.insert( "branches",  "- beta" ); 
    data.insert( "username_and_repository", username_and_repository.as_str() );
    data.insert( "uses_branch", "alpha" );
    data.insert( "src_branch", "beta" );
    data.insert( "dest_branch", "master" );

    file_write( &workflow_root.join( "AutoPrToMaster.yml" ), &handlebars.render( "auto_pr_to", &data )? )?;

    file_write( &workflow_root.join( "RunsClean.yml" ),  include_str!( "../../files/workflow/rust_clean.yml" ) )?;

    let mut data = BTreeMap::new();
    data.insert( "username_and_repository", username_and_repository.as_str() );

    file_write( &workflow_root.join( "StandardRustPullRequest.yml" ), &handlebars.render( "standard_rust_pull_request", &data )? )?;

    file_write( &workflow_root.join( "StandardRustPush.yml" ), include_str!( "../../files/workflow/standard_rust_push.yml" ) )?;

    file_write( &workflow_root.join( "StandardRustScheduled.yml" ), include_str!("../../files/workflow/standard_rust_scheduled.yml") )?;

    file_write( &workflow_root.join( "StandardRustStatus.yml" ), include_str!("../../files/workflow/standard_rust_status.yml") )?;

    file_write( &workflow_root.join( "StatusChecksRulesUpdate.yml" ), include_str!("../../files/workflow/status_checks_rules_update.yml") )?;
    Ok( () )
  }

  /// Prepare params for render appropriative_branch_for template.
  fn map_prepare_for_appropriative_branch< 'a >( branches: &'a str, username_and_repository: &'a str, uses_branch: &'a str, src_branch: &'a str, name: &'a str ) -> BTreeMap< &'a str, &'a str >
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
  pub fn file_write( filename: &Path, content: &str ) -> Result< () > 
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

  /// Searches and extracts the username and repository name from the repository URL. 
  /// The repository URL is first sought in the Cargo.toml file of the workspace; 
  /// if not found there, it is then searched in the Cargo.toml file of the module. 
  /// If it is still not found, the search continues in the GitHub remotes.
  fn username_and_repository( workspace: &mut Workspace ) -> Result< String > 
  {
    let cargo_toml_path = workspace.workspace_root()?.join( "Cargo.toml" );
    if cargo_toml_path.exists() 
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
        return url::extract_repo_url( &url )
        .and_then( | url | url::git_info_extract( &url ).ok() )
        .ok_or_else( || anyhow!( "Fail to parse repository url from workspace Cargo.toml"))
      } 
      else 
      {
        let mut url = None;
        for package in workspace.packages_get()?
        {
          if let Ok( wu ) = manifest::private::repo_url( package.manifest_path.parent().unwrap().as_std_path() )
          {
            url = Some( wu );
            break;
          }
        }
        return url
        .and_then( | url | url::extract_repo_url( &url ) )
        .and_then( | url | url::git_info_extract( &url ).ok() )
        .ok_or_else( || anyhow!( "Fail to extract repository url") )
      }
    } 
    else 
    {
      return Err( anyhow!( "Fail to find workspace Cargo.toml" ) );
    }
  }

}

crate::mod_interface!
{
  prelude use workflow_generate;
}