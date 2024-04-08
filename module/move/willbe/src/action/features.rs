mod private
{
  use _path::AbsolutePath;
  use workspace::Workspace;
  use features::FeaturesReport;
  use former::Former;
  use crate::*;
  use error_tools::{for_app::Context, Result};

  #[ derive( Debug, Former ) ]
  pub struct FeaturesOptions
  {
    manifest_dir: AbsolutePath,
    with_features_deps: bool,
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
}
