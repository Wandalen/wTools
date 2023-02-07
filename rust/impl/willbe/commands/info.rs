/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };
  use wca::
  {
    Args,
    NoSubject,
    NoProperties,
    Context,
  };

  ///
  /// Prints information about package
  ///

  pub fn info( _ : Args< NoSubject, NoProperties >, ctx : Context ) -> Result< (), BasicError >
  {
    // Get package from context or try to read package at current directory
    let package = match ctx.get_ref::< Option< Package > >()
    {
      Some( Some( package ) ) => package.to_owned(),
      None =>
      {
        let path = env::current_dir().unwrap();
        Package::try_from( path )
        .map_err( | _ | err!( "Package not found at current directory" ) )?
      }
      _ => return Ok( () )
    };

    let info =  package.clone().metadata();
    if info.is_err()
    {
      println!( "`{}` can not parse package metadata", package.path().display() );
      return Ok( () )
    }
    let info = info.unwrap().all().to_owned();

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
  prelude use info;
}
