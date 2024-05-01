/// Internal namespace.
mod private
{
  use crate::*;

  use std::path::PathBuf;
  use std::collections::HashMap;
  use std::fmt::Formatter;
  use colored::Colorize;
  use crates_tools::CrateArchive;

  use action::list::{ ListReport, ListNodeReport };
  use _path::AbsolutePath;
  use wtools::error::for_app::Result;
  use diff::{ DiffReport, crate_diff };
  use error_tools::for_app::format_err;

  /// Options for `publish_diff` command
  #[ derive( Debug, former::Former ) ]
  pub struct PublishDiffOptions
  {
    path : PathBuf,
    keep_archive : Option< PathBuf >,
  }
  
  #[ derive( Debug ) ]
  pub struct PublishDiffReport
  {
    pub diffs : HashMap< AbsolutePath, DiffReport >,
    pub root_path : AbsolutePath,
    pub tree : ListNodeReport,
  }
  
  impl std::fmt::Display for PublishDiffReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      let mut tree = self.tree.clone();
      let root_path = tree.path.as_ref().unwrap().clone();
      let root_name = tree.name.clone();
      let root_version = tree.version.as_ref().unwrap().clone();
      
      fn modify( diffs : &HashMap< AbsolutePath, DiffReport >, tree : &mut ListNodeReport )
      {
        let path = tree.path.take().unwrap();
        let path = path.as_path().to_string_lossy();
        let path = path.strip_suffix( "Cargo.toml" ).unwrap_or( &path );
        let root = AbsolutePath::try_from( path ).unwrap();

        let diff = diffs.get( &root ).unwrap();

        let has_changes = diff.has_changes();
        tree.name = if has_changes { format!( "{}", tree.name.yellow() ) } else { tree.name.clone() };
        tree.version.as_mut().map( | v | *v = format!( "{} {}", if has_changes { v.yellow() } else { v.as_str().into() }, if has_changes { "MODIFIED" } else { "" } ) );
        
        for dep in &mut tree.normal_dependencies
        {
          modify( diffs, dep )
        }
      }
      modify( &self.diffs, &mut tree );

      let path = root_path.as_path().to_string_lossy();
      let path = path.strip_suffix( "Cargo.toml" ).unwrap_or( &path );
      let root = AbsolutePath::try_from( path ).unwrap();
      let diff = self.diffs.get( &root ).unwrap();
      
      write!( f, "Tree:\n{}\nChanges in `{root_name} {root_version}`:\n{}", tree, diff )?;
      
      Ok( () )
    }
  }

  /// Return the differences between a local and remote package versions.
  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn publish_diff( o : PublishDiffOptions ) -> Result< PublishDiffReport >
  {
    let path = AbsolutePath::try_from( o.path )?;
    let dir = CrateDir::try_from( path.clone() )?;
    
    let list = action::list
    (
      action::list::ListOptions::former()
      .path_to_manifest( dir )
      .format( action::list::ListFormat::Tree )
      .info([ action::list::PackageAdditionalInfo::Version, action::list::PackageAdditionalInfo::Path ])
      .dependency_sources([ action::list::DependencySource::Local ])
      .dependency_categories([ action::list::DependencyCategory::Primary ])
      .form()
    )
    .unwrap();
    let ListReport::Tree( mut tree ) = list else { return Err( format_err!( "Logical error. Unexpected list format" ) ) };
    let mut tasks = vec![ tree[ 0 ].clone() ];
    let mut diffs = HashMap::new();
    let mut current_idx = 0;
    while current_idx < tasks.len()
    {
      let path = tasks[ current_idx ].path.as_ref().unwrap().to_string_lossy();
      let path = path.strip_suffix( "Cargo.toml" ).unwrap_or( &path );
      let path = AbsolutePath::try_from( path )?;
      let dir = CrateDir::try_from( path.clone() )?;

      let package = package::Package::try_from( dir.clone() )?;
      let name = &package.name()?;
      let version = &package.version()?;

    _ = cargo::pack( cargo::PackOptions::former().path( dir.as_ref() ).allow_dirty( true ).no_verify( true ).dry( false ).form() )?;
    let l = CrateArchive::read( packed_crate::local_path( name, version, dir )? )?;
    let r = CrateArchive::download_crates_io( name, version ).unwrap();


      if let Some( out_path ) = &o.keep_archive
      {
        _ = std::fs::create_dir_all( &out_path );
        for path in r.list()
        {
          let local_path = out_path.join( path );
          let folder = local_path.parent().unwrap();
          _ = std::fs::create_dir_all( folder );

          let content = r.content_bytes( path ).unwrap();

          std::fs::write( local_path, content )?;
        }
      }
      diffs.insert( path, crate_diff( &l, &r ).exclude( diff::PUBLISH_IGNORE_LIST ) );
      tasks.extend( tasks[ current_idx ].normal_dependencies.clone() );
      
      current_idx += 1;
    }
    let report = PublishDiffReport
    {
      root_path : path.clone(),
      diffs,
      tree : tree.remove( 0 ),
    };

    Ok( report )
  }
}

//

crate::mod_interface!
{
  orphan use PublishDiffOptions;
  /// Publishes the difference between the local and published versions of a package.
  orphan use publish_diff;
}
