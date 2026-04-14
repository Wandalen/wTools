#![ doc( html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https: //docs.rs/willbe2/latest/willbe2/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Build tool binary" ) ]

#[ allow( unused_imports ) ]
use ::willbe2::*;

fn main() -> Result< (), willbe::error::untyped::Error >
{
  let args : Vec< String > = std::env::args().collect();
  willbe::run( args )
}
