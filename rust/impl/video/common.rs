/// Private namespace.
pub( crate ) mod private
{
  use wtools::error::Result;

  /// Select strategy for the output format.
  #[ derive( Debug, PartialEq ) ]
  pub enum EncoderType
  {
    /// Convert to gif.
    Gif,
    /// Convert to apng.
    Png,
    /// Convert to mp4.
    Mp4,
  }

  /// Select color encoding.
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum ColorType
  {
    /// RGB color encoding.
    Rgb,
    // qqq : extend
  }

  /// Trait for encoders.
  pub trait EncodeData
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : impl AsRef< [ u8 ] > ) -> Result< () >;
    /// Finish encoding.
    fn flush( &self ) -> Result< () >;
  }
}

//

wtools::meta::mod_interface!
{
  prelude use EncoderType;
  prelude use ColorType;
  prelude use EncodeData;
}
