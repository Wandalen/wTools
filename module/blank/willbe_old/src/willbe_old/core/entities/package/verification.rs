/// Internal namespace.
pub( crate ) mod private
{
  use std::process::Command;

  use crate::PackageMetadata;

  /// All package verification methods
  pub trait Verification
  {
    /// Do all checks
    fn check_all( &self ) -> bool
    {
      self.has_readme()
      && self.has_documentation()
      && self.has_license()
      && self.is_tests_passed()
    }
    /// Check if readme exists
    fn has_readme( &self ) -> bool;
    /// Check if documentation exists
    fn has_documentation( &self ) -> bool;
    /// Check if the package has a license
    fn has_license( &self ) -> bool;
    /// Check if all tests is passed
    fn is_tests_passed( &self ) -> bool;
  }

  impl Verification for PackageMetadata
  {
    fn has_readme( &self ) -> bool
    {
      self.readme().is_some()
    }

    fn has_documentation( &self ) -> bool
    {
      self.documentation().is_some()
    }

    fn has_license( &self ) -> bool
    {
      self.license().is_some()
      ||
      self.license_file().is_some()
    }

    fn is_tests_passed( &self ) -> bool
    {
      let tests_output = Command::new( "cargo" )
      .current_dir( self.as_package().path() )
      .args([ "test" ])
      .output().unwrap();

      tests_output.status.success()
    }
  }
}

//

crate::mod_interface!
{
  prelude use Verification;
}
