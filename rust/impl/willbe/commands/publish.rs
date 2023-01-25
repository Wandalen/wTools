/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };
  use wca::
  {
    Args,
    NoSubject, NoProperties,
    Context,
  };

  ///
  /// Verify and publish a package
  ///

  pub fn publish( _ : Args< NoSubject, NoProperties >, ctx : Context ) -> Result< (), BasicError >
  {
    println!( "[LOG] Called publish command" );

    // Get package from context or try to read package at current directory
    let package = match ctx.get_ref::< Option< Package > >()
    {
      Some( Some( package ) ) => package.to_owned(),
      None =>
      {
        let path = env::current_dir().unwrap().to_owned();
        Package::try_from( path )
        .map_err( | _ | err!( "Package not found at current directory" ) )?
      }
      _ => return Ok( () )
    };

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
