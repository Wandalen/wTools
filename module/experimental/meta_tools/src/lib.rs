#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https: //docs.rs/meta_tools/latest/meta_tools/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Meta programming utilities" ) ]

#![ warn( dead_code ) ]

// Declare the top-level modules
/// Dependencies namespace.
pub mod dependency;
/// Meta programming namespace.
pub mod meta;
/// Own namespace.
pub mod own;
/// Orphan namespace.
pub mod orphan;
/// Exposed namespace.
#[ allow( clippy::module_inception ) ]
pub mod exposed;
/// Prelude namespace.
pub mod prelude;

// Re-export the exposed parts of these modules directly
#[ cfg( feature = "enabled" ) ]
pub use dependency::exposed::*;
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use meta::exposed::*;
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use own::exposed::*;
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use orphan::exposed::*;
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use exposed::exposed::*;
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use prelude::exposed::*;
