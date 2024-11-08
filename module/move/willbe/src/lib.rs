#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/willbe/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

pub use mod_interface::mod_interface;

/// Define a private namespace for all its items.
mod private
{
  use crate::*;

  /// Takes the command line arguments and perform associated function(s).
  /// If no arguments are provided, the function identifies this as an ambiguous state and prompts the user with a help message, suggesting possible commands they might want to execute.
  /// It then terminates the program with an exit code of 1 to indicate an error due to the lack of input.
  ///
  /// Do not support interactive mode.
  pub fn run( args : Vec< String > ) -> Result< (), error::untyped::Error >
  {
    #[ cfg( feature = "tracing" ) ]
    {
      tracing_subscriber::fmt().pretty().init();
    }

    let args : Vec< String > = args.into_iter().skip( 1 ).collect();

    let ca = command::ca()
    .help_variants( [ wca::HelpVariants::General, wca::HelpVariants::SubjectCommand ] )
    .perform();

    let program = args.join( " " );
    if program.is_empty()
    {
      eprintln!( "Ambiguity. Did you mean?" );
      ca.perform( ".help" )?;
      std::process::exit( 1 )
    }
    else
    {
      Ok( ca.perform( program.as_str() )? )
    }

  }

}

mod_interface!
{

  own use run;

  /// Entities of which spaces consists of.
  layer entity;

  /// Genera-purpose tools which might be moved out one day.
  layer tool;

  /// Describes CLI commands.
  layer command;

  /// Describes functions that can be called from an interface.
  layer action;

}
