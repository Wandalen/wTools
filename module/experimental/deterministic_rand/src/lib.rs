#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https: //docs.rs/deterministic_rand/latest/deterministic_rand/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Deterministic random number generation" ) ]

use mod_interface::mod_interface;

#[ cfg(not(feature = "no_std")) ]
#[ cfg(feature = "determinism") ]
pub mod hrng_deterministic;
#[ cfg(any(not(feature = "determinism"), feature = "no_std")) ]
pub mod hrng_non_deterministic;

#[ cfg(not(feature = "no_std")) ]
#[ cfg(feature = "determinism") ]
pub use hrng_deterministic as hrng;
#[ cfg(any(not(feature = "determinism"), feature = "no_std")) ]
pub use hrng_non_deterministic as hrng;

pub use rand;
pub use rand::Rng;
pub use rand::distributions;

// xxx: mod_interface v0.61.0 regression: `use super::hrng` inside mod_interface! tries to
// access inaccessible `private` sub-modules of the aliased module. Re-export directly until
// mod_interface supports aliased-module imports.
pub use hrng::Hrng;

/// Private namespace — required by `mod_interface` for crate-root invocations.
mod private {}

mod_interface! {

  // xxx: make it working — feature-conditional use inside mod_interface! not yet supported
  // #[ cfg( feature = "determinism" ) ]
  // use super ::hrng_deterministic as hrng;
  // #[ cfg( not( feature = "determinism" ) ) ]
  // use super ::hrng_non_deterministic as hrng;

  // xxx: make it working — aliased layer import not yet supported
  // #[ cfg( feature = "determinism" ) ]
  // layer hrng_deterministic as hrng;
  // #[ cfg( not( feature = "determinism" ) ) ]
  // layer hrng_non_deterministic as hrng;

  layer iter;
  #[ cfg( not( feature = "no_std" ) ) ]
  layer seed;
}
