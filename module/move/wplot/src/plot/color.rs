/// Internal namespace.
pub( crate ) mod private
{
  // use crate::own::*;
  use core::fmt;
  use num_traits::{ Zero }; /* zzz : consider as submodule for wtools */

  /// Convertable into RGBA.
  pub trait RgbaInterface< T >
  where
    T : Zero + fmt::Debug + Clone + Copy,
  {
    /// Convert into RGBA.
    fn into_rgba( self ) -> Rgba< T >;
  }

  // zzz : use type_constructor::Enumberable for indexed access to color components

  /// RGBA
  #[ derive( Debug, Clone ) ]
  pub struct Rgba< T = f32 >
  where
    T : Zero + fmt::Debug + Clone + Copy,
  {
    /// Red.
    pub r : T,
    /// Green.
    pub g : T,
    /// Blue.
    pub b : T,
    /// Alpha.
    pub a : T,
  }

  impl< T > Default for Rgba< T >
  where
    T : Zero + fmt::Debug + Clone + Copy,
  {
    fn default() -> Self
    {
      Self
      {
        r : Zero::zero(),
        g : Zero::zero(),
        b : Zero::zero(),
        a : Zero::zero(),
      }
    }
  }

  impl< T > RgbaInterface< T > for Rgba< T >
  where
    T : Zero + fmt::Debug + Clone + Copy,
  {
    fn into_rgba( self ) -> Rgba< T >
    {
      self
    }
  }

  impl RgbaInterface< f32 >
  for [ f32 ; 3 ]
  {
    fn into_rgba( self ) -> Rgba< f32 >
    {
      Rgba::< f32 >
      {
        r : self[ 0 ],
        g : self[ 1 ],
        b : self[ 2 ],
        a : 1.0,
      }
    }
  }

  impl RgbaInterface< f32 >
  for [ f32 ; 4 ]
  {
    fn into_rgba( self ) -> Rgba< f32 >
    {
      Rgba::< f32 >
      {
        r : self[ 0 ],
        g : self[ 1 ],
        b : self[ 2 ],
        a : self[ 3 ],
      }
    }
  }

}

::meta_tools::mod_interface!
{

  own use ::rgb::*;

  #[ cfg( not( feature = "no_std" ) ) ]
  exposed use Rgba;

  #[ cfg( not( feature = "no_std" ) ) ]
  prelude use RgbaInterface;

}
