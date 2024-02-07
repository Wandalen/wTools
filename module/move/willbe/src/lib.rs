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

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Takes the command line arguments and perform associated function(s).
  /// If no arguments are provided, the function identifies this as an ambiguous state and prompts the user with a help message, suggesting possible commands they might want to execute.
  /// It then terminates the program with an exit code of 1 to indicate an error due to the lack of input.
  ///
  /// Do not support interactive mode.
  pub fn run() -> Result< (), wtools::error::for_app::Error >
  {
    let args = std::env::args().skip( 1 ).collect::< Vec< String > >();

    let ca = wca::CommandsAggregator::former()
    // .exit_code_on_error( 1 )
    .grammar( command::grammar_form() )
    .executor( command::executor_form() )
    .help_variants( [ wca::HelpVariants::General, wca::HelpVariants::SubjectCommand ] )
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
      Ok( ca.perform( program.as_str() )? )
    }

  }
}

wtools::meta::mod_interface!
{

  protected use run;

  /// The tools for operating over packages.
  layer tools;

  /// Describes CLI commands.
  layer command;

  /// Describes functions that can be called from an interface.
  layer endpoint;

  /// Offers capabilities for package management, facilitating the handling and organization of packages.
  layer package;

  /// Provides a set of functionalities for handling and manipulating packages.
  layer packages;

  /// The parse function parses an input string into a HashMap where the keys are String and the values are of type Value.
  layer query;

  /// Tools for parsing and extracting information from url.
  layer url;

  /// Provides an opportunity to work with versions.
  layer version;

  /// Git interaction module that enables seamless integration and management of version control workflows.
  layer git;

  /// Interaction module with the `cargo` utilities.
  layer cargo;

  /// It features the ability to interact with workspaces, manage their participants, and other functionalities.
  layer workspace;

  /// To manipulate manifest data.
  layer manifest;

  /// Handles operations related to packed Rust crates
  layer packed_crate;

}
