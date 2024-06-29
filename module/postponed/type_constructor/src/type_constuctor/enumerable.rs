/// Internal namespace.
pub( crate ) mod private
{

  // zzz : use type_constructor::Enumberable for indexed access to color components

  ///
  /// Has length and indexed access.
  ///

  pub trait Enumerable
  {
    /// Type of an element.
    type Element;
    /// Length.
    fn len( &self ) -> usize;
    /// Get element by reference.
    #[ inline ]
    fn element( &self, index : usize ) -> &Self::Element
    {
      self.element_ref( index )
    }
    /// Get element by reference.
    fn element_ref( &self, index : usize ) -> &Self::Element;
    /// Get element copying it.
    fn element_copy( &self, index : usize ) -> Self::Element;
  }

  ///
  /// Has length and indexed access, including mutable access.
  ///

  pub trait EnumerableMut
  where
    Self : Enumerable,
  {

    // fn element_mut2( &mut self, index : usize ) -> &mut < Self as Enumerable >::Element;

    /// Get element by mutable reference.
    // fn element_mut( &mut self, index : usize ) -> &mut < Self as Enumerable >::Element;
    // where
    //   Self : 'static
    // ;
    fn element_mut< 'slf, 'element >( &'slf mut self, index : usize ) -> &'element mut < Self as Enumerable >::Element
    where
      'element : 'slf
    ;

  }

  /// Iterate enumerable consuming it.
  pub trait IterateEnumerableConsuming
  {
    /// Type of an element.
    type Element;
    /// Type of iterator.
    type Iterator : Iterator< Item = Self::Element >;
    /// Iterate consuming.
    fn enumerable_iterate_consuming( self ) -> Self::Iterator;
  }

  /// Iterate enumerable consuming non-consuming it.
  pub trait IterateEnumerable
  {
    /// Type of an element.
    type Element;
    /// Type of iterator.
    type Iterator : Iterator< Item = Self::Element >;
    /// Iterate non-consuming.
    fn enumerable_iterate( self ) -> Self::Iterator;
  }

  impl< E > IterateEnumerableConsuming for E
  where
    E : Enumerable,
  {
    type Element = < E as Enumerable >::Element;
    type Iterator = EnumerableIteratorCopy< Self >;
    fn enumerable_iterate_consuming( self ) -> Self::Iterator
    {
      EnumerableIteratorCopy::new( self )
    }
  }

  impl< 'a, E > IterateEnumerable for &'a E
  where
    E : Enumerable,
  {
    type Element = &'a < E as Enumerable >::Element;
    type Iterator = EnumerableIteratorRef< 'a, E >;
    fn enumerable_iterate( self ) -> Self::Iterator
    {
      EnumerableIteratorRef::new( self )
    }
  }

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
    type Item = En::Element;
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
    type Item = &'a En::Element;
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
    En : EnumerableMut + 'static,
  {
    ins : &'a mut En,
    last_index : usize,
  }

  impl< 'a, En > EnumerableIteratorMut< 'a, En >
  where
    En : EnumerableMut + 'static,
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
    En : EnumerableMut + 'static,
  {
    type Item = &'a mut < En as Enumerable >::Element;
    fn next( &mut self ) -> Option< Self::Item >
    // where
    //   Self : 'a,
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
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    EnumerableIteratorCopy,
    EnumerableIteratorRef,
    EnumerableIteratorMut,
  };
}


/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    Enumerable,
    EnumerableMut,
    IterateEnumerableConsuming,
    IterateEnumerable,
  };
}
