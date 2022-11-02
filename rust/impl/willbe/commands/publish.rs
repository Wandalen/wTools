/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };

  ///
  /// 
  /// 

  pub fn publish( instruction : &crate::instruction::Instruction ) -> Result< (), BasicError >
  {
    dbg!( &instruction );

    let current_path = env::current_dir().unwrap();

    let package = Package::try_from( current_path )
    .or( Err( err!( "Package not found at current directory" ) ) )?;

    println!
    (
      "=== Verification ===\nLicense: {}\nReadme: {}\nDocumentation: {}\nTests: {}",
      if package.has_license() { "Yes" } else { "No" },
      if package.has_readme() { "Yes" } else { "No" },
      if package.has_documentation() { "Yes" } else { "No" },
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
