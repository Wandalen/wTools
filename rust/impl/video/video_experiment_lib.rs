#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/video_experiment/latest/video_experiment/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Aggregates animation modules and provide common strategy to convert byte buffers to animated
//! formats.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.
pub mod dependency
{
  #[ doc( inline ) ]
  pub use ::gif;
  #[ doc( inline ) ]
  pub use ::apng;
  #[ doc( inline ) ]
  pub use ::png;
  #[ doc( inline ) ]
  pub use ::ac_ffmpeg;
  #[ doc( inline ) ]
  pub use ::openh264;
}

//

wtools::meta::mod_interface!
{
  /// Common types and interfaces.
  layer common;
  /// Encoders.
  layer encoders;
  /// Universal interface for animation.
  layer encoder_strategy;
  /// Yuv tools
  layer yuv;
}
