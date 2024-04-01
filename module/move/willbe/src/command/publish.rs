/// Internal namespace.
mod private
{
  use crate::*;
  use colored::Colorize;

  use wca::VerifiedCommand;
  use wtools::error::Result;
  use former::Former;

  #[ derive( Former ) ]
  struct PublishProperties
  {
    #[ default( true ) ]
    dry : bool,
    #[ default( true ) ]
    temp : bool,
  }

  ///
  /// Publish package.
  ///

  pub fn publish( o : VerifiedCommand ) -> Result< () >
  {
    let args_line = format!( "{}", o.args.get_owned( 0 ).unwrap_or( std::path::PathBuf::from( "" ) ).display() );
    let prop_line = format!( "{}", o.props.iter().map( | p | format!( "{}:{}", p.0, p.1.to_string() ) ).collect::< Vec< _ > >().join(" ") );

    let patterns : Vec< _ > = o.args.get_owned( 0 ).unwrap_or_else( || vec![ "./".into() ] );

    let PublishProperties { dry, temp } = o.props.try_into()?;

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

  impl TryFrom< wca::Props > for PublishProperties
  {
    type Error = wtools::error::for_app::Error;
    fn try_from( value : wca::Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value.get_owned( "dry" ) { this.dry::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "temp" ) { this.temp::< bool >( v ) } else { this };

      Ok( this.form() )
    }
  }
}

//

crate::mod_interface!
{
  /// List packages.
  orphan use publish;
}
