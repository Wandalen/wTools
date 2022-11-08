/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };

  ///
  /// Verify and publish a package
  /// 

  pub fn publish( _instruction : &crate::instruction::Instruction ) -> Result< (), BasicError >
  {
    let current_path = env::current_dir().unwrap();

    let package = Package::try_from( current_path )
    .map_err( | _ | err!( "Package not found at current directory" ) )?;

    let info = PackageMetadata::try_from( package )
    .map_err( | _ | err!( "Can not parse package metadata" ) )?;

    println!
    (
      "=== Verification ===\nLicense: {}\nReadme: {}\nDocumentation: {}\nTests: {}",
      if info.has_license() { "Yes" } else { "No" },
      if info.has_readme() { "Yes" } else { "No" },
      if info.has_documentation() { "Yes" } else { "No" },
      if info.is_tests_passed() { "Passed" } else { "Failed" }
    );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use publish;
}
