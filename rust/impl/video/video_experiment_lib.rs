
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
  pub use ::minimp4;
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
}
