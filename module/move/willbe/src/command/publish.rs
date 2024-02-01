/// Internal namespace.
mod private
{
  use crate::*;

  use wca::{ Args, Props };
  use tools::bool_like::BoolLike;
  use wtools::error::Result;


  ///
  /// Publish package.
  ///

  pub fn publish( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let patterns : Vec< _ > = args.get_owned( 0 ).unwrap_or_else( || vec![ "./".into() ] );

    let dry: bool = properties
    .get_owned( "dry" )
    .map( | dry : String | dry.parse().map_or_else( | _ | BoolLike::True, | e | e ) )
    .unwrap_or_else( || BoolLike::True )
    .into()
    ;

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
