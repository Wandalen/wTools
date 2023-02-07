/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };
  use wca::
  {
    Args,
    Properties,
    Context,
    string::parse_request::OpType,
  };

  #[derive(Debug, Default)]
  pub struct DepProperties
  {
    dry: bool,
  }

  // TODO: WCA: Properties will be extended. Change here too
  impl Properties for DepProperties
  {
    fn parse( properties : &HashMap< String, OpType< String > > ) -> Result< Self, BasicError >
    {
      if let Some( dry ) = properties.get( "dry" )
      {
        let dry = dry.clone().primitive().unwrap();
        return Ok( Self { dry : &dry == "1" })
      }

      Ok( Self::default() )
    }
  }

  ///
  /// Works with packages dependencies
  /// 

  pub fn dep( args : Args< String, DepProperties >, ctx : Context ) -> Result< (), BasicError >
  {
    // Get package from context or try to read package at current directory
    let mut package = match ctx.get_ref::< Option< Package > >()
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

    match args.subject.split_once( ' ' )
    {
      Some(( "add", packages )) =>
      {
        // TODO: Think about packages features
        let dependencies = packages.split( ' ' ).collect::< Vec< _ > >();
        for dep in dependencies
        {
          let mut dep = package.dependency( dep );
          if args.properties.dry
          {
            dep = dep.dry();
          }

          package = dep.add()?;
        }
      }
      Some(( "remove", packages )) =>
      {
        let dependencies = packages.split( ' ' ).collect::< Vec< _ > >();
        for dep in dependencies
        {
          package = package.dependency( dep ).remove()?;
        }
      }
      _ => {}
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use dep;
}
