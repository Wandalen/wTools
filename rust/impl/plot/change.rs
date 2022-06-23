/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Context.
  pub trait Change
  where
    Self :
      fmt::Debug +
      // Clone +
      // ?Sized +
    ,
  {
  }

  // impl Clone for Change
  // {
  //   fn clone( &self ) -> Self
  //   {
  //     Box::new(  )
  //   }
  // }

//   trait ChangeCloneInterface
//   {
//     fn clone_box( &self ) -> Box< dyn Change >;
//   }
//
//   impl< T > ChangeCloneInterface for T
//   where
//     // T : Change + Clone,
//     T : 'static + Change + Clone,
//   {
//     fn clone_box( &self ) -> Box< dyn Change >
//     {
//       Box::new( self.clone() )
//     }
//   }
//
//   impl Clone for Box< dyn Change >
//   {
//     fn clone( &self ) -> Box< dyn Change >
//     {
//       self.clone_box()
//     }
//   }

  // xxx : move

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
  };
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
  pub use super::
  {
    prelude::*,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    private::Change,
  };
}
