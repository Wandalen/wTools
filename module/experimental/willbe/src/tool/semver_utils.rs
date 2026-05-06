#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  use semver :: { Version, VersionReq };

  /// Check if a version satisfies a version requirement.
  ///
  /// This is a thin wrapper around `VersionReq::matches` for consistency.
  #[ must_use ]
  pub fn version_satisfies( requirement: &VersionReq, version: &Version ) -> bool
  {
    requirement.matches( version )
  }

  /// Check if a version does NOT satisfy a version requirement.
  ///
  /// Returns true if the version is incompatible with the requirement.
  #[ must_use ]
  pub fn version_incompatible( requirement: &VersionReq, version: &Version ) -> bool
  {
    !requirement.matches( version )
  }
}

//

crate ::mod_interface!
{
  own use version_satisfies;
  own use version_incompatible;
}
