/// Internal namespace.
mod private
{
  use crate::package::functions::{ self as package, * };

  use crate::tools::
  {
    files,
    manifest,
    path,
    process,
    output,
  };
  use crate::
  {
    wtools,
    digest,
    http,
    version::bump,
  };
  use wtools::error::Result;
  use anyhow::*;
  use std::
  {
    fs,
    env,
    path::PathBuf,
  };
  use cargo_metadata::
  {
    MetadataCommand,
  };

  ///
  /// Publish packages.
  ///

  pub fn publish( patterns : Vec< String >, dry : bool ) -> Result< () >
  {
    let current_path = env::current_dir()?;

    let paths = files::find( &current_path, &patterns );
    let mut paths = paths.iter().filter_map( | s | if s.ends_with( "Cargo.toml" ) { Some( s.into() ) } else { None } ).collect::< Vec< PathBuf > >();
    if !patterns.is_empty() && paths.is_empty() && path::valid_is( &patterns[ 0 ] )
    {
      paths.push( PathBuf::from( &patterns[ 0 ] ) );
    }

    for path in paths
    {
      dbg!(publish_single( &current_path, &path, dry ).context( "Publish list of packages" )?);
    }

    Ok( () )
  }

  ///
  /// Publish single packages.
  ///
  pub fn publish_single( current_path : &PathBuf, path : &PathBuf, dry : bool ) -> Result< output::Output >
  {
    let mut command_output = output::Output
    {
      context: format!( "Publish package with path: `{}`", path.display() ),
      actions: vec![]
    };

    let mut manifest = manifest::get( path )?;
    if !manifest.package_is() || manifest.local_is()
    {
      return Ok( command_output );
    }
    let data = manifest.manifest_data.as_deref_mut().ok_or( anyhow!( "Failed to get manifest data" ) )?;

    let mut package_dir = manifest.manifest_path.clone();
    package_dir.pop();

    let output = process::start_sync( "cargo package", &package_dir ).context( "Take information about package" )?;
    let action = output::Action::with_output( format!( "`cargo package` for `{}`", package_dir.display() ), output )?;
    command_output.actions.push( action );

    let name = &data[ "package" ][ "name" ].clone();
    let name = name.as_str().ok_or( anyhow!( "Package has no name" ) )?;
    let version = &data[ "package" ][ "version" ].clone();
    let version = version.as_str().ok_or( anyhow!( "Package has no version" ) )?;
    let local_package_path = local_path_get( name, version, &manifest.manifest_path );

    let local_package = fs::read( dbg!(local_package_path) ).context( "Read local package" )?;
    let remote_package = http::retrieve_bytes( name, version ).unwrap_or_default();

    let digest_of_local = digest::hash( &local_package );
    let digest_of_remote = digest::hash( &remote_package );

    if digest_of_local != digest_of_remote
    {
      data[ "package" ][ "version" ] = bump( version )?;
      let version = &data[ "package" ][ "version" ].clone();
      let version = version.as_str().ok_or( anyhow!( "Failed to take package version after bump" ) )?;
      manifest.store()?;

      if dry
      {
        let buf = format!( "git commit --dry-run -am \"{} v{}\"", name, version );
        let output = process::start_sync( &buf, current_path ).context( "Dry commit while publishing" )?;
        let action = output::Action::with_output( format!( "dry `{buf}` for `{}`", current_path.display() ), output )?;
        command_output.actions.push( action );

        let output = process::start_sync( "git push --dry-run", current_path ).context( "Dry push while publishing" )?;
        let action = output::Action::with_output( format!( "dry `git push --dry-run` for `{}`", current_path.display() ), output )?;
        command_output.actions.push( action );

        let output = process::start_sync( "cargo publish --dry-run --allow-dirty", &package_dir ).context( "Dry publish" )?;
        let action = output::Action::with_output( format!( "dry `cargo publish --dry-run --allow-dirty` for `{}`", package_dir.display() ), output )?;
        command_output.actions.push( action );

        let buf = format!( "git checkout {:?}", &package_dir );
        let output = process::start_sync( &buf, current_path )?;
        let action = output::Action::with_output( format!( "dry `{buf}` for `{}`", current_path.display() ), output )?;
        command_output.actions.push( action );
      }
      else
      {
        let buf = format!( "git commit -am \"{} v{}\"", name, version );
        let output = process::start_sync( &buf, current_path ).context( "Commit changes while publishing" )?;
        let action = output::Action::with_output( format!( "`{buf}` for `{}`", current_path.display() ), output )?;
        command_output.actions.push( action );

        let output = process::start_sync( "git push", current_path ).context( "Push while publishing" )?;
        let action = output::Action::with_output( format!( "`git push` for `{}`", current_path.display() ), output )?;
        command_output.actions.push( action );

        let output = process::start_sync( "cargo publish", &package_dir ).context( "Publish" )?;
        let action = output::Action::with_output( format!( "`cargo publish` for `{}`", package_dir.display() ), output )?;
        command_output.actions.push( action );
      }
    }
    else
    {
      let action = output::Action
      {
        context: "Complete publish".into(),
        out: format!( "Package {} is up to date", name ),
        err: String::new(),
      };
      command_output.actions.push( action );
    }

    Ok( command_output )
  }

  ///
  /// Publish packages from workspace.
  ///

  pub fn workspace_publish( path_to_workspace : PathBuf, dry : bool ) -> Result< () >
  {
    let current_path = env::current_dir()?;

    let mut manifest = manifest::Manifest::new();
    let manifest_path = manifest.manifest_path_from_str( &path_to_workspace )?;
    let package_metadata = MetadataCommand::new()
    .manifest_path( &manifest_path )
    .no_deps()
    .exec()?;

    let packages_map = package::filter( &package_metadata );
    let sorted = package::toposort( &packages_map );

    for name in sorted.iter()
    {
      publish_single( &current_path, &packages_map[ name ].manifest_path.clone().into(), dry ).context( "Publish workspace" )?;
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  /// Publish package.
  prelude use publish;
  /// Publish packages from workspace.
  prelude use workspace_publish;
}
