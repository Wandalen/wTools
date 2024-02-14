/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wca::
  {
    Args, Props,
    Context,
  };
  use error_tools::{ Result, for_app::format_err };

  /// Publish command declaration
  pub fn publish_command() -> wca::Command
  {
    wca::Command::former()
    .hint( "Publish a package" )
    .long_hint( "Validate, runs tests and publish a package" )
    .phrase( "crate.publish" )
    .form()
  }

  ///
  /// Verify and publish a package
  ///

  pub fn publish( _ : ( Args, Props ), ctx : Context ) -> Result< () >
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
        .map_err( | _ | format_err!( "Package not found at current directory" ) )?
      }
      _ => return Ok( () )
    };

    let info = PackageMetadata::try_from( package )
    .map_err( | _ | format_err!( "Can not parse package metadata" ) )?;

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
  prelude use publish_command;
  prelude use publish;
}
