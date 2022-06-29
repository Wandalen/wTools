/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use num_traits::{ Zero }; /* xxx : consider for wtools */

  /// Convertable into RGBA.
  pub trait RgbaInterface< T >
  where
    T : Zero + fmt::Debug + Clone + Copy,
  {
    /// Convert into RGBA.
    fn into_rgba( self ) -> Rgba< T >;
  }

  // xxx : here type_constructor's derives should be used

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

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
  };
  pub use ::rgb::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::private::Rgba;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "use_std" ) ]
  pub use super::private::RgbaInterface;
}
