
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/asbytes/latest/asbytes/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::bytemuck;
}

/// Define a private namespace for all its items.
#[ cfg( feature = "enabled" ) ]
mod private
{

  pub use bytemuck::
  {
    Pod,
  };

  /// Trait for converting data to byte slices.
  /// This trait abstracts the conversion of types that implement Pod into their raw byte representation.
  #[ cfg( feature = "as_bytes" ) ]
  pub trait AsBytes
  {

    /// Returns the underlying byte slice of the data.
    fn as_bytes( &self ) -> &[ u8 ]
    ;

    /// Returns the size in bytes of the data.
    #[ inline ]
    fn byte_size( &self ) -> usize
    {
      self.as_bytes().len()
    }

    /// Returns the count of scalar elements contained in the data.
    /// For flat structures, this corresponds to the number of elements.
    /// For multidimensional data, this value may differ from the total number of components.
    fn len( &self ) -> usize;

  }

  /// Implementation for any single POD type.
  impl< T : Pod > AsBytes for ( T, )
  {

    #[ inline ]
    fn as_bytes( &self ) -> &[ u8 ]
    {
      // Use bytes_of to get the byte slice of a single POD item
      bytemuck::bytes_of( &self.0 )
    }

    #[ inline ]
    fn byte_size( &self ) -> usize
    {
      // The size is simply the size of the type itself
      std::mem::size_of::< T >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      // A single item has a length of 1 element
      1
    }

  }

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
      self.len() * std::mem::size_of::< T >() / std::mem::size_of::< u8 >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      self.len()
    }

  }

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
      self.len() * std::mem::size_of::< T >() / std::mem::size_of::< u8 >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      self.len()
    }

  }

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
      self.len() * std::mem::size_of::< T >() / std::mem::size_of::< u8 >()
    }

    #[ inline ]
    fn len( &self ) -> usize
    {
      N
    }

  }

}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use bytemuck::
  {
    checked,
    offset_of,
    bytes_of,
    bytes_of_mut,
    cast,
    cast_mut,
    cast_ref,
    cast_slice,
    cast_slice_mut,
    fill_zeroes,
    from_bytes,
    from_bytes_mut,
    pod_align_to,
    pod_align_to_mut,
    pod_read_unaligned,
    try_cast,
    try_cast_mut,
    try_cast_ref,
    try_cast_slice,
    try_cast_slice_mut,
    try_from_bytes,
    try_from_bytes_mut,
    try_pod_read_unaligned,
    write_zeroes,
    CheckedBitPattern,
    PodCastError,
    AnyBitPattern,
    Contiguous,
    NoUninit,
    Pod,
    PodInOption,
    TransparentWrapper,
    Zeroable,
    ZeroableInOption,
  };
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ cfg( feature = "as_bytes" ) ]
  pub use private::
  {
    AsBytes,
    Pod,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
