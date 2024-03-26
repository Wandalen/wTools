/// Internal namespace.
mod private
{
  use crate::*;
  use colored::Colorize;

  use wca::{ Args, Props };
  use wtools::error::Result;


  ///
  /// Publish package.
  ///

  pub fn publish( args : Args, properties : Props ) -> Result< () >
  {
    let args_line = format!( "{}", args.get_owned( 0 ).unwrap_or( std::path::PathBuf::from( "" ) ).display() );
    let prop_line = format!( "{}", properties.iter().map( | p | format!( "{}:{}", p.0, p.1.to_string() ) ).collect::< Vec< _ > >().join(" ") );
    let patterns : Vec< _ > = args.get_owned( 0 ).unwrap_or_else( || vec![ "./".into() ] );

    let dry : bool = properties
    .get_owned( "dry" )
    .unwrap_or( true );

    let temp : bool = properties
    .get_owned( "temp" )
    .unwrap_or( true );

    match action::publish( patterns, dry, temp )
    {
      Ok( report ) =>
      {
        println!( "{report}" );

        if dry && report.packages.iter().find( |( _, p )| p.publish_required ).is_some()
        {
          let args = if args_line.is_empty() { String::new() } else { format!(" {}", args_line) };
          let prop = if prop_line.is_empty() { String::new() } else { format!(" {}", prop_line) };
          let line = format!("will .publish{}{} dry:0", args, prop);
          println!("To apply plan, call the command `{}`", line.blue());
          // qqq : for Petro : for Bohdan : bad. should be exact command with exact parameters
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
