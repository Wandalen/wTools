#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/willbe/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Utility with set of tools for managing developer routines.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ allow( unused_imports ) ]
use ::willbe::*;

//

#[ cfg( not( feature = "no_std" ) ) ]
fn main() -> Result< (), wtools::error::for_app::Error >
{
  let args = env::args().skip( 1 ).collect::< Vec< String > >();

  let ca = wca::CommandsAggregator::former()
  // .exit_code_on_error( 1 )
  .grammar( commands::grammar_form() )
  .executor( commands::executor_form() )
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

#[ cfg( feature = "no_std" ) ]
fn main()
{
}