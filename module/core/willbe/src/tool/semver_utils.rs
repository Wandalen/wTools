#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  use semver :: { Version, VersionReq };

  /// Check if a version satisfies a version requirement.
  ///
  /// This is a thin wrapper around `VersionReq::matches` for consistency.
  ///
  /// # Examples
  ///
  /// ```
  /// use semver::{Version, VersionReq};
  /// use willbe::tool::semver_utils::version_satisfies;
  ///
  /// let req = VersionReq::parse("^2.36.0").unwrap();
  /// let v1 = Version::parse("2.36.1").unwrap();
  /// let v2 = Version::parse("2.37.0").unwrap();
  ///
  /// assert!(version_satisfies(&req, &v1));  // 2.36.1 satisfies ^2.36.0
  /// assert!(version_satisfies(&req, &v2));  // 2.37.0 satisfies ^2.36.0
  /// ```
  #[must_use] 
  pub fn version_satisfies( requirement: &VersionReq, version: &Version ) -> bool
  {
    requirement.matches( version )
  }

  /// Check if a version does NOT satisfy a version requirement.
  ///
  /// Returns true if the version is incompatible with the requirement.
  #[must_use] 
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
