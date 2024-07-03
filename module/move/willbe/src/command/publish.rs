/// Internal namespace.
mod private
{
  use crate::*;
  use colored::Colorize;

  use wca::VerifiedCommand;
  use error::{ untyped::Context }; // xxx
  use former::Former;
  use std::fmt::Write;
  use channel::Channel;

  #[ derive( Former ) ]
  struct PublishProperties
  {
    #[ former( default = Channel::Stable ) ]
    channel : Channel,
    #[ former( default = true ) ]
    dry : bool,
    #[ former( default = true ) ]
    temp : bool,
  }

  ///
  /// Publish package.
  ///

  pub fn publish( o : VerifiedCommand ) -> error::untyped::Result< () > // qqq : use typed error
  {
    let args_line = format!
    (
      "{}",
      o
      .args
      .get_owned( 0 )
      .unwrap_or( std::path::PathBuf::from( "" ) ).display()
    );
    let prop_line = format!
    (
      "{}",
      o
      .props
      .iter()
      .map( | p | format!( "{}:{}", p.0, p.1.to_string() ) )
      .collect::< Vec< _ > >().join(" ") );

    let patterns : Vec< _ > = o
    .args
    .get_owned( 0 )
    .unwrap_or_else( || vec![ "./".into() ] );

    let PublishProperties
    {
      channel,
      dry,
      temp
    } = o.props.try_into()?;
    let plan = action::publish_plan( patterns, channel, dry, temp )
    .context( "Failed to plan the publication process" )?;

    let mut formatted_plan = String::new();
    writeln!( &mut formatted_plan, "Tree :" )?;
    plan.write_as_tree( &mut formatted_plan )?;

    if !plan.plans.is_empty()
    {
      writeln!( &mut formatted_plan, "The following packages are pending for publication :" )?;
      plan.write_as_list( &mut formatted_plan )?;
    }
    println!( "{formatted_plan}" );

    match action::publish( plan )
    {
      Ok( report ) =>
      {
        println!( "{report}" );

        if dry && !report.packages.is_empty()
        {
          let args = if args_line.is_empty() { String::new() } else { format!(" {}", args_line) };
          let prop = if prop_line.is_empty() { String::new() } else { format!(" {}", prop_line) };
          let line = format!("will .publish{}{} dry:0", args, prop );
          println!("To apply plan, call the command `{}`", line.blue() );
          // aaa : for Petro : for Bohdan : bad. should be exact command with exact parameters
          // aaa : it`s already works
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
    type Error = error::untyped::Error;
    fn try_from( value : wca::Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value
      .get_owned( "channel" )
      {
        this.channel::< Channel >( { let v : String = v; Channel::try_from( v )? } )
      }
      else
      { this };

      this = if let Some( v ) = value
      .get_owned( "dry" ) { this.dry::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "temp" ) { this.temp::< bool >( v ) } else { this };

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
