/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };

  ///
  /// Prints information about package
  /// 

  pub fn info( _instruction : &crate::instruction::Instruction ) -> Result< (), BasicError >
  {
    let current_path = env::current_dir().unwrap();

    let package = Package::try_from( current_path )
    .map_err( | _ | err!( "Package not found at current directory" ) )?;

    let info = PackageMetadata::try_from( package )
    .map_err( | _ | err!( "Can not parse package metadata" ) )?;
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
  prelude use info;
}
