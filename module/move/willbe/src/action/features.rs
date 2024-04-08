mod private
{
  use _path::AbsolutePath;
  use workspace::Workspace;
  use features::FeaturesReport;
  use crate::*;
  use error_tools::{for_app::Context, Result};

  /// List features
  pub fn features( o: AbsolutePath ) -> Result< FeaturesReport >
  {
    let workspace = Workspace::with_crate_dir( CrateDir::try_from( o.clone() )? ).context( "Failed to find workspace" )?;
    let packages = workspace.packages()?.into_iter().filter
    ( | package |
      package.manifest_path().as_str().starts_with( o.as_ref().as_os_str().to_str().unwrap() )
    ).collect::< Vec< _ > >();
    let mut report = FeaturesReport::default();
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
}
