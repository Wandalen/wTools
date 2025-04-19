/// Define a private namespace for all its items.
mod private
{

  pub use bytemuck::
  {
    Pod,
  };

  /// Trait for borrowing data as byte slices.
  /// This trait abstracts the conversion of types that implement Pod (or collections thereof)
  /// into their raw byte representation as a slice (`&[u8]`).

  pub trait AsBytes
  {

    /// Returns the underlying byte slice of the data.
    fn as_bytes( &self ) -> &[ u8 ]
    ;

    /// Returns an owned vector containing a copy of the bytes of the data.
    /// The default implementation clones the bytes from `as_bytes()`.
    #[ inline ]
    fn to_bytes_vec( &self ) -> Vec< u8 >
    {
      self.as_bytes().to_vec()
    }

    /// Returns the size in bytes of the data.
    #[ inline ]
    fn byte_size( &self ) -> usize
    {
      self.as_bytes().len()
    }

    /// Returns the count of elements contained in the data.
    /// For single-element tuples `(T,)`, this is 1.
    /// For collections (`Vec<T>`, `&[T]`, `[T; N]`), this is the number of `T` items.
    fn len( &self ) -> usize;

  }

  /// Implementation for single POD types wrapped in a tuple `(T,)`.

  impl< T : Pod > AsBytes for ( T, )
  {

    #[ inline ]
    fn as_bytes( &self ) -> &[ u8 ]
    {
      bytemuck::bytes_of( &self.0 )
    }

    #[ inline ]
    fn byte_size( &self ) -> usize
    {
      std::mem::size_of::< T >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      1
    }

  }

  /// Implementation for Vec<T> where T is POD.

  impl< T : Pod > AsBytes for Vec< T >
  {

    #[ inline ]
    fn as_bytes( &self ) -> &[ u8 ]
    {
      bytemuck::cast_slice( self )
    }

    #[ inline ]
    fn byte_size( &self ) -> usize
    {
      self.len() * std::mem::size_of::< T >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      self.len()
    }

  }

  /// Implementation for [T] where T is POD.

  impl< T : Pod > AsBytes for [ T ]
  {

    #[ inline ]
    fn as_bytes( &self ) -> &[ u8 ]
    {
      bytemuck::cast_slice( self )
    }

    #[ inline ]
    fn byte_size( &self ) -> usize
    {
      self.len() * std::mem::size_of::< T >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      self.len()
    }

  }

  /// Implementation for [T; N] where T is POD.

  impl< T : Pod, const N : usize > AsBytes for [ T ; N ]
  {

    #[ inline ]
    fn as_bytes( &self ) -> &[ u8 ]
    {
      bytemuck::cast_slice( self )
    }

    #[ inline ]
    fn byte_size( &self ) -> usize
    {
      N * std::mem::size_of::< T >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      N
    }

  }

}


#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.

#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;

}


#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.

#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

}

/// Exposed namespace of the module.

#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.

#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  pub use private::AsBytes;
}