/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Alternative implementation of trait From.
  ///

  pub trait CollectionFrom< T > : Sized
  {
    /// Performs the conversion.
    fn collection_from( src : T ) -> Self;
  }

  ///
  /// Alternative implementation of trait Into.
  ///

  pub trait CollectionInto< T > : Sized
  {
    /// Performs the conversion.
    fn collection_into( self ) -> T;
  }

  impl< Target, Original > CollectionInto< Target > for Original
  where
    Target : CollectionFrom< Original >,
  {
    fn collection_into( self ) -> Target
    {
      Target::collection_from( self )
    }
  }

  //

  impl< Into1, CollectionInto, Id >
  CollectionFrom< ( Into1, CollectionInto ) >
  for ( Id, Id )
  where
    Into1 : Into< Id >,
    CollectionInto : Into< Id >,
  {
    fn collection_from( src : ( Into1, CollectionInto ) ) -> Self
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
    CollectionFrom,
    CollectionInto,
  };
}
