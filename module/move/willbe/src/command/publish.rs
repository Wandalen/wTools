/// Internal namespace.
mod private
{
  use crate::*;
  // use std::path::PathBuf;
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
    .unwrap_or( true );

    // println!( "`publish` command patterns: {patterns:?}, dry: {dry}" );
    match endpoint::publish( patterns, dry )
    {
      core::result::Result::Ok( report ) =>
      {
        println!( "{report}" );
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
