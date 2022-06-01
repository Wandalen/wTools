/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Implementation of trait From to vectorize into/from.
  ///

  pub trait VectorizedFrom< T > : Sized
  {
    /// Performs the conversion.
    fn vectorized_from( src : T ) -> Self;
  }

  ///
  /// Implementation of trait Into to vectorize into/from.
  ///

  pub trait VectorizedInto< T > : Sized
  {
    /// Performs the conversion.
    fn vectorized_into( self ) -> T;
  }

  //

  impl< Target, Original > VectorizedInto< Target > for Original
  where
    Target : VectorizedFrom< Original >,
  {
    fn vectorized_into( self ) -> Target
    {
      Target::vectorized_from( self )
    }
  }

  //

  impl< Into1, VectorizedInto, Id >
  VectorizedFrom< ( Into1, VectorizedInto ) >
  for ( Id, Id )
  where
    Into1 : Into< Id >,
    VectorizedInto : Into< Id >,
  {
    fn vectorized_from( src : ( Into1, VectorizedInto ) ) -> Self
    {
      ( src.0.into(), src.1.into() )
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::private::
  {
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    VectorizedFrom,
    VectorizedInto,
  };
}
