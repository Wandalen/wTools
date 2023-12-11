#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/willbe/" ) ]

//!
//! Utility with set of tools for managing developer routines.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use mod_interface::mod_interface;
/// Micro wtools
pub mod wtools;

// qqq : for Bohdan : poor description, make it useful

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// qqq : for Bohdan : write description
  pub fn run() -> Result< (), wtools::error::for_app::Error >
  {
    let args = std::env::args().skip( 1 ).collect::< Vec< String > >();

    let ca = wca::CommandsAggregator::former()
    // .exit_code_on_error( 1 )
    .grammar( command::grammar_form() )
    .executor( command::executor_form() )
    .build();

    let program = args.join( " " );
    if program.is_empty()
    {
      eprintln!( "Ambiguity. Did you mean?" );
      ca.perform( ".help" )?;
      std::process::exit( 1 )
    }
    else
    {
      ca.perform( program.as_str() )
    }

  }
}

wtools::meta::mod_interface!
{

  protected use run;

  /// The tools for operating over packages.
  layer tools;

  /// Commands library.
  layer command;

  /// Endpoints library.
  layer endpoint;

  /// Package library.
  layer package;

  /// query
  layer query;

  /// methods for url
  layer url;
  /// Version library.
  layer version;

  /// Git library.
  layer git;

  /// Cargo library.
  layer cargo;

  /// Metadata cache.
  layer workspace;

  /// To manipulate manifest data.
  layer manifest;

}
