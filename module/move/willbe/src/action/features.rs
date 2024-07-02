mod private
{
  use crate::*;

  use std::
  {
    fmt
  };
  use collection::{ BTreeMap, HashMap };

  // // use path::AbsolutePath;
  use former::Former;
  use error::{ untyped::Context };
  // use workspace::Workspace;

  /// Options available for the .features command
  #[ derive( Debug, Former ) ]
  pub struct FeaturesOptions
  {
    // crate_dir : AbsolutePath,
    crate_dir : CrateDir,
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
  pub fn features( FeaturesOptions { crate_dir, with_features_deps } : FeaturesOptions )
  -> error::untyped::Result< FeaturesReport >
  // qqq : typed error
  {
    let workspace = Workspace::try_from( crate_dir.clone() ).context( "Failed to find workspace" )?;
    let packages = workspace.packages().filter
    (
      | package |
      {
        if let Ok( manifest_file ) = package.manifest_file()
        {
          manifest_file.inner().starts_with(crate_dir.clone().absolute_path())
        }
        else
        {
          false
        }
      } // aaa : remove unwrap
      // aaa : done
    );
    // ).collect::< Vec< _ > >(); qqq : rid of. put type at var
    let mut report = FeaturesReport
    {
      with_features_deps,
      ..Default::default()
    };
    packages
    // .iter()
    .for_each
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
