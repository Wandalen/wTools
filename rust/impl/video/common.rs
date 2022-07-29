/// Private namespace.
pub( crate ) mod private
{
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

  impl Default for EncoderType
  {
    fn default() -> Self
    {
      EncoderType::Mp4
    }
  }

  /// Select color encoding.
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum ColorType
  {
    /// RGB color encoding.
    Rgb,
    /// RGB color encoding.
    Rgba,
    // qqq : extend
  }

  impl Default for ColorType
  {
    fn default() -> Self
    {
      ColorType::Rgb
    }
  }

  /// Trait for encoders.
  pub trait EncodeData
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : &[ u8 ] ) -> Result< (), Box<dyn std::error::Error > >;
    /// Finish encoding. It is recommended to flush data at the end of encoding, because the data can be loosed.
    fn flush( &mut self ) -> Result< (), Box<dyn std::error::Error > >;
  }
}

//

wtools::meta::mod_interface!
{
  prelude use EncoderType;
  prelude use ColorType;
  prelude use EncodeData;
}
