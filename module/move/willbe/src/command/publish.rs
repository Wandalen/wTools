/// Internal namespace.
mod private
{
  use crate::*;
  // use std::path::PathBuf;
  use tools::bool_like::BoolLike;
  use wca::{ Args, Props };
  use wtools::error;


  ///
  /// Publish package.
  ///

  pub fn publish( ( args, properties ) : ( Args, Props ) ) -> error::Result< () >
  {
    let patterns : Vec< _ > = args.get_owned( 0 ).unwrap_or_else( || vec![ "./".into() ] );

    let dry: bool = properties
    .get_owned( "dry" )
    .map( | dry : String | dry.parse().map_or_else( | _ | BoolLike::True, | e | e ) )
    .unwrap_or_else( || BoolLike::True )
    .into();

    match endpoint::publish( patterns, dry )
    {
      core::result::Result::Ok( report ) =>
      {
        println!( "{report}" );

        if dry && report.packages.iter().find( |( _, p )| p.publish_required ).is_some()
        {
          println!( "To perform actual publishing, call the command with `dry:0` property." )
        }

        Ok( () )
      }
      Err( ( report, e ) ) =>
      {
        eprintln!( "{report}" );
        Err( e.context( "publish command" ) )
      }
    }
  }
}

//

crate::mod_interface!
{
  /// List packages.
  orphan use publish;
}
