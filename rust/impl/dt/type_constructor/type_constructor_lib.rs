#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square_icon_small.ico")]
#![ doc( html_root_url = "https://docs.rs/type_constructor/latest/type_constructor/")]

#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Fundamental data types and type constructors, like Single, Pair, Many.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// #![ no_std ]

#[ cfg( not( feature = "use_std" ) ) ]
extern crate core as std;
#[ cfg( all( not( feature = "use_std" ), feature = "use_alloc" ) ) ]
extern crate alloc;

#[ path = "./inc.rs" ]
mod inc;
#[ doc( inline ) ]
pub use inc::*;
