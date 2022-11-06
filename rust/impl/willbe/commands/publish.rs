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
    .or( Err( err!( "Package not found at current directory" ) ) )?;

    let info = package.info();

    println!
    (
      "=== Verification ===\nLicense: {}\nReadme: {}\nDocumentation: {}\nTests: {}",
      if info.license.is_some() { "Yes" } else { "No" },
      if info.readme.is_some() { "Yes" } else { "No" },
      if info.documentation.is_some() { "Yes" } else { "No" },
      if package.is_tests_passed() { "Passed" } else { "Failed" }
    );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use publish;
}
