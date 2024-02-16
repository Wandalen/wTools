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

  /// Info command declaration
  pub fn info_command() -> wca::Command
  {
    wca::Command::former()
    .hint( "Prints information about package" )
    .long_hint( "Prints information about package at current directory" )
    .phrase( "crate.info" )
    .form()
  }

  ///
  /// Prints information about package
  ///

  pub fn info( _ : ( Args, Props ), ctx : Context ) -> Result< () >
  {
    println!( "[LOG] Called info command" );

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
    let info = info.all().to_owned();

    println!
    (
      r#"
Name: "{}"
Version: "{}"
Description: "{}"
Documentation: "{}"
License: "{}"
Readme: "{}"
Dependencies: {:?}
Location: "{}"
      "#,
      info.name,
      info.version,
      info.description.unwrap_or_else( || "Not found".to_string() ),
      info.documentation.unwrap_or_else( || "Not found".to_string() ),
      info.license.unwrap_or_else( || "Not found".to_string() ),
      info.readme.map( String::from ).unwrap_or_else( || "Not found".to_string() ),
      info.dependencies.iter().map( | d | &d.name ).collect::< Vec< _ > >(),
      info.manifest_path.parent().unwrap()
    );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use info_command;
  prelude use info;
}
