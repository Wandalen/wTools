mod private
{
  use crate::*;

  use std::
  {
    collections::{ BTreeMap, HashMap },
    fmt
  };

  use _path::AbsolutePath;
  use former::Former;
  use error_tools::{ for_app::Context, Result };
  use workspace::Workspace;

  /// Options available for the .features command
  #[ derive( Debug, Former ) ]
  pub struct FeaturesOptions
  {
    manifest_dir : AbsolutePath,
    with_features_deps : bool,
  }

  /// Represents a report about features available in the package
  #[ derive( Debug, Default ) ]
  pub struct FeaturesReport
  {
    /// Flag to turn off/on displaying feature dependencies - "feature: [deps...]"
    pub with_features_deps : bool,

    /// A key-value pair structure representing available features.
    ///
    /// Key: name of the package (useful for workspaces, where multiple packages can be found).
    ///
    /// Value: Another key-value pair representing a feature and its dependencies
    pub inner : HashMap< String, BTreeMap< String, Vec< String > > >,
  }

  impl fmt::Display for FeaturesReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ >) -> Result< (), fmt::Error >
    {
      self.inner.iter().try_for_each
      ( | ( package, features ) |
      {
        writeln!(f, "Package {}:", package)?;
        features.iter().try_for_each
        ( | ( feature, dependencies ) |
        {
          let feature = match self.with_features_deps
          {
            false => format!( "\t{feature}" ),
            true
            =>
            {
              let deps = dependencies.join( ", " );
              format!( "\t{feature}: [{deps}]" )
            }
          };
          writeln!( f, "{feature}" )
        }
        )
      }
      )
    } 
  }

  /// List features
  pub fn features( FeaturesOptions { manifest_dir, with_features_deps } : FeaturesOptions ) -> Result< FeaturesReport >
  {
    let workspace = Workspace::with_crate_dir( CrateDir::try_from( manifest_dir.clone() )? ).context( "Failed to find workspace" )?;
    let packages = workspace.packages()?.into_iter().filter
    ( | package |
      package.manifest_path().as_str().starts_with( manifest_dir.as_ref().as_os_str().to_str().unwrap() )
    ).collect::< Vec< _ > >();
    let mut report = FeaturesReport
    {
      with_features_deps,
      ..Default::default()
    };
    packages.iter().for_each
    ( | package |
    {
      let features = package.features();
      report.inner.insert(package.name().to_owned(), features.to_owned());
    }
    );
    Ok( report )
  }
}

crate::mod_interface!
{
  orphan use features;
  orphan use FeaturesOptions;
  orphan use FeaturesReport;
}
