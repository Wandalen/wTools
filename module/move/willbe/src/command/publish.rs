/// Internal namespace.
mod private
{
  use crate::*;

  use wca::{ Args, Props };
  use wtools::error::Result;


  ///
  /// Publish package.
  ///

  pub fn publish( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let patterns : Vec< _ > = args.get_owned( 0 ).unwrap_or_else( || vec![ "./".into() ] );

    let dry : bool = properties
    .get_owned( "dry" )
    .unwrap_or( true );
    
    let temp : bool = properties
    .get_owned( "temp" )
    .unwrap_or( true );

    match action::publish( patterns, dry, temp )
    {
      core::result::Result::Ok( report ) =>
      {
        println!( "{report}" );

        if dry && report.packages.iter().find( |( _, p )| p.publish_required ).is_some()
        {
          println!( "To perform actual publishing, call the command with `dry : 0` property." )
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
