/// Internal namespace.
pub( crate ) mod private
{

  // xxx : use type_constructor::Enumberable for indexed access to color components

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
    fn element_copy( &self, index : usize ) -> Self::Item;
  }

  ///
  /// Has length and indexed access, including mutable access.
  ///

  pub trait EnumerableMut
  where
    Self : Enumerable,
  {
    /// Length.
    fn element_mut< 'a, 'b >( &'a mut self, index : usize ) -> &'b mut < Self as Enumerable >::Item
    where
      'b : 'a;
  }

//     impl IntoIterator for Pair
//     {
//       type Item = < Pair as Enumerable >::Item;
//       type IntoIter = TheModule::EnumerableIteratorCopy< Self >;
//       fn into_iter( self ) -> Self::IntoIter
//       {
//         TheModule::EnumerableIteratorCopy::new( self )
//       }
//     }
//
//     impl< 'a > IntoIterator for &'a Pair
//     {
//       type Item = &'a < Pair as Enumerable >::Item;
//       type IntoIter = TheModule::EnumerableIteratorRef< 'a, Pair >;
//       fn into_iter( self ) -> Self::IntoIter
//       {
//         TheModule::EnumerableIteratorRef::new( self )
//       }
//     }

  /// Iterator for enumerable.

  #[ derive( Debug ) ]
  pub struct EnumerableIteratorCopy< En >
  where
    En : Enumerable,
  {
    ins : En,
    last_index : usize,
  }

  impl< En > EnumerableIteratorCopy< En >
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
  for EnumerableIteratorCopy< En >
  where
    En : Enumerable,
  {
    type Item = En::Item;
    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.last_index < self.ins.len()
      {
        self.last_index += 1;
        Some( self.ins.element_copy( self.last_index - 1 ) )
      }
      else
      {
        None
      }
    }
  }

  ///
  /// Ref iterator for enumerable.
  ///

  #[ derive( Debug ) ]
  pub struct EnumerableIteratorRef< 'a, En >
  where
    En : Enumerable,
  {
    ins : &'a En,
    last_index : usize,
  }

  impl< 'a, En > EnumerableIteratorRef< 'a, En >
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
  for EnumerableIteratorRef< 'a, En >
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

  ///
  /// Mut iterator for enumerable.
  ///

  #[ derive( Debug ) ]
  pub struct EnumerableIteratorMut< 'a, En >
  where
    En : EnumerableMut,
  {
    ins : &'a mut En,
    last_index : usize,
  }

  impl< 'a, En > EnumerableIteratorMut< 'a, En >
  where
    En : EnumerableMut,
  {
    /// Constructor.
    pub fn new( ins : &'a mut En ) -> Self
    {
      Self { ins, last_index : 0 }
    }
  }

  impl< 'a, En > Iterator
  for EnumerableIteratorMut< 'a, En >
  where
    En : EnumerableMut,
  {
    type Item = &'a mut < En as Enumerable >::Item;
    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.last_index < self.ins.len()
      {
        self.last_index += 1;
        Some( self.ins.element_mut( self.last_index - 1 ) )
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
    EnumerableIteratorCopy,
    EnumerableIteratorRef,
    EnumerableIteratorMut,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    Enumerable,
    EnumerableMut,
  };
}
