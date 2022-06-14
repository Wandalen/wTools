/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Has length and indexed access.
  ///

  pub trait Enumerable
  {
    /// Item.
    type Item;
    /// Length.
    fn len( &self ) -> usize;
    /// Get element.
    fn element( &self, index : usize ) -> &Self::Item;
    /// Get element.
    fn element_take( &self, index : usize ) -> Self::Item;
  }

//     impl IntoIterator for Pair
//     {
//       type Item = < Pair as Enumerable >::Item;
//       type IntoIter = TheModule::EnumerableIteratorConsumable< Self >;
//       fn into_iter( self ) -> Self::IntoIter
//       {
//         TheModule::EnumerableIteratorConsumable::new( self )
//       }
//     }
//
//     impl< 'a > IntoIterator for &'a Pair
//     {
//       type Item = &'a < Pair as Enumerable >::Item;
//       type IntoIter = TheModule::EnumerableIteratorNonConsumable< 'a, Pair >;
//       fn into_iter( self ) -> Self::IntoIter
//       {
//         TheModule::EnumerableIteratorNonConsumable::new( self )
//       }
//     }

  /// Iterator for enumerable.

  #[ derive( Debug ) ]
  pub struct EnumerableIteratorConsumable< En >
  where
    En : Enumerable,
  {
    ins : En,
    last_index : usize,
  }

  impl< En > EnumerableIteratorConsumable< En >
  where
    En : Enumerable,
  {
    /// Constructor.
    pub fn new( ins : En ) -> Self
    {
      Self { ins, last_index : 0 }
    }
  }

  impl< En > Iterator
  for EnumerableIteratorConsumable< En >
  where
    En : Enumerable,
  {
    type Item = En::Item;
    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.last_index < self.ins.len()
      {
        self.last_index += 1;
        Some( self.ins.element_take( self.last_index - 1 ) )
      }
      else
      {
        None
      }
    }
  }

  /// Iterator for enumerable.

  #[ derive( Debug ) ]
  pub struct EnumerableIteratorNonConsumable< 'a, En >
  where
    En : Enumerable,
  {
    ins : &'a En,
    last_index : usize,
  }

  impl< 'a, En > EnumerableIteratorNonConsumable< 'a, En >
  where
    En : Enumerable,
  {
    /// Constructor.
    pub fn new( ins : &'a En ) -> Self
    {
      Self { ins, last_index : 0 }
    }
  }

  impl< 'a, En > Iterator
  for EnumerableIteratorNonConsumable< 'a, En >
  where
    En : Enumerable,
  {
    type Item = &'a En::Item;
    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.last_index < self.ins.len()
      {
        self.last_index += 1;
        Some( self.ins.element( self.last_index - 1 ) )
      }
      else
      {
        None
      }
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
    EnumerableIteratorConsumable,
    EnumerableIteratorNonConsumable,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    Enumerable,
  };
}
