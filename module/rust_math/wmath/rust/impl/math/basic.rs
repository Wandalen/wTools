/// Internal namespace.
pub mod internal
{
  use core::fmt::Debug;
  use core::hash::Hash;

  // #[ repr( C ) ]
  // struct ix2( u32, u32 );
  // fn as_tuple( &self ) -> &( u32 , u32 );
  // fn as_array( &self ) -> &[ u32 ; 2 ];

  ///
  #[ allow( non_camel_case_types ) ]
  pub type ix2_cgmath< E > = cgmath::Vector2< E >;

  ///
  #[ allow( non_camel_case_types ) ]
  pub type ix2_nalgebra< E > = nalgebra::Vector2< E >;

  ///
  #[ allow( non_camel_case_types ) ]
  pub type ix2_winit< E > = winit::dpi::PhysicalSize< E >;

  ///
  /// Vector ix2
  ///

  #[ allow( non_camel_case_types ) ]
  #[ repr( C ) ]
  #[ derive( Debug, PartialEq, Eq, Copy, Clone, Hash ) ]
  pub struct ix2< E : Debug + PartialEq + Eq + Copy + Clone + Hash + Sized >
  ( E, E );

  ///
  /// Interface of vector ix2.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait ix2_interface
  {
    /// Type of element.
    type Element : Debug + PartialEq + Eq + Copy + Clone + Hash + Sized;

    /// First element.
    fn _0( &self ) -> Self::Element;
    /// Second element.
    fn _1( &self ) -> Self::Element;
    /// First element.
    #[ inline ]
    fn x( &self ) -> Self::Element
    {
      self._0()
    }
    /// Second element.
    #[ inline ]
    fn y( &self ) -> Self::Element
    {
      self._1()
    }

    /// First element.
    fn _0_ref( &self ) -> &Self::Element;
    /// Second element.
    fn _1_ref( &self ) -> &Self::Element;
    /// First element.
    #[ inline ]
    fn x_ref( &self ) -> &Self::Element
    {
      self._0_ref()
    }
    /// Second element.
    #[ inline ]
    fn y_ref( &self ) -> &Self::Element
    {
      self._1_ref()
    }

    /// First element.
    fn _0_mut( &mut self ) -> &mut Self::Element;
    /// Second element.
    fn _1_mut( &mut self ) -> &mut Self::Element;
    /// First element.
    #[ inline ]
    fn x_mut( &mut self ) -> &mut Self::Element
    {
      self._0_mut()
    }
    /// Second element.
    #[ inline ]
    fn y_mut( &mut self ) -> &mut Self::Element
    {
      self._1_mut()
    }

    /// Clone as tuple.
    #[ inline ]
    fn clone_as_tuple( &self ) -> ( Self::Element , Self::Element )
    {
      ( self._0(), self._1() )
    }
    /// Clone as array.
    #[ inline ]
    fn clone_as_array( &self ) -> [ Self::Element ; 2 ]
    {
      [ self._0(), self._1() ]
    }
    /// Clone as canonical.
    #[ inline ]
    fn clone_as_canonical( &self ) -> ix2< Self::Element >
    {
      ix2::< Self::Element >( self._0(), self._1() )
    }

  }

  ///
  /// Interface of vector ix2 for structures with the canonical layout.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait ix2_canonical_interface : ix2_interface
  {

    /// Interpret as tuple.
    #[ inline ]
    fn as_tuple( &self ) -> &( Self::Element , Self::Element )
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical() )
      }
    }
    /// Interpret as array.
    #[ inline ]
    fn as_array( &self ) -> &[ Self::Element ; 2 ]
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical() )
      }
    }
    /// Interpret as slice.
    #[ inline ]
    fn as_slice( &self ) -> &[ Self::Element ]
    {
      &self.as_array()[ .. ]
    }

    /// Interpret as mutable tuple.
    #[ inline ]
    fn as_tuple_mut( &mut self ) -> &mut ( Self::Element , Self::Element )
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical_mut() )
      }
    }
    /// Interpret as mutable array.
    #[ inline ]
    fn as_array_mut( &mut self ) -> &mut [ Self::Element ; 2 ]
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical_mut() )
      }
    }
    /// Interpret as mutable slice.
    #[ inline ]
    fn as_slice_mut( &mut self ) -> &mut [ Self::Element ]
    {
      &mut self.as_array_mut()[ .. ]
    }

    /// Canonical representation of the vector.
    fn as_canonical( &self ) -> &ix2< Self::Element >;
    /// Mutable canonical representation of the vector.
    fn as_canonical_mut( &mut self ) -> &mut ix2< Self::Element >;

  }

  impl< E > ix2_interface for ix2< E >
  where
    E : Debug + PartialEq + Eq + Copy + Clone + Hash,
  {
    type Element = E;

    #[ inline ]
    fn _0( &self ) -> Self::Element
    {
      self.0
    }

    #[ inline ]
    fn _1( &self ) -> Self::Element
    {
      self.1
    }

    /* */

    #[ inline ]
    fn _0_ref( &self ) -> &Self::Element
    {
      &self.0
    }

    #[ inline ]
    fn _1_ref( &self ) -> &Self::Element
    {
      &self.1
    }

    /* */

    #[ inline ]
    fn _0_mut( &mut self ) -> &mut Self::Element
    {
      &mut self.0
    }

    #[ inline ]
    fn _1_mut( &mut self ) -> &mut Self::Element
    {
      &mut self.1
    }

    /* */

  }

  impl< E > ix2_canonical_interface for ix2< E >
  where
    E : Debug + PartialEq + Eq + Copy + Clone + Hash,
  {

    #[ inline ]
    fn as_canonical( &self ) -> &ix2< Self::Element >
    {
      self
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut ix2< Self::Element >
    {
      self
    }

  }

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::ix2_cgmath;
  pub use i::ix2_nalgebra;
  pub use i::ix2_winit;

  pub use i::ix2;
  pub use i::ix2_interface;
  pub use i::ix2_canonical_interface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::ix2_cgmath;
  pub use i::ix2_nalgebra;
  pub use i::ix2_winit;

  pub use i::ix2;
  pub use i::ix2_interface;
  pub use i::ix2_canonical_interface;

}
