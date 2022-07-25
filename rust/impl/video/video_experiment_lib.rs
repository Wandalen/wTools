
/// Namespace with dependencies.
pub mod dependency
{
  pub use ::gif;
  pub use ::apng;
  pub use ::png;
  pub use ::minimp4;
  pub use ::ac_ffmpeg;
  pub use ::openh264;
}

//

wtools::meta::mod_interface!
{
  /// Encoders.
  layer encoders;
  /// Common interface for animation.
  layer encoder_strategy;
}
