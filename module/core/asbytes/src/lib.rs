#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/asbytes/latest/asbytes/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Byte conversion utilities" ) ]

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  // Only include bytemuck if either as_bytes or into_bytes is enabled
  #[ cfg(any(feature = "as_bytes", feature = "into_bytes")) ]
  pub use ::bytemuck;
}

/// Define a private namespace for all its items.
#[ cfg( feature = "enabled" ) ]
mod private
{
}

#[ cfg( feature = "as_bytes" ) ]
mod as_bytes;
#[ cfg( feature = "into_bytes" ) ]
mod into_bytes;

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
  #[ cfg( feature = "as_bytes" ) ]
  pub use as_bytes::orphan::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "into_bytes" ) ]
  pub use into_bytes::orphan::*;

  // Re-export bytemuck items only if a feature needing it is enabled
  #[ cfg(any(feature = "as_bytes", feature = "into_bytes")) ]
  #[ doc( inline ) ]
  pub use bytemuck :: {
  checked, offset_of, bytes_of, bytes_of_mut, cast, cast_mut, cast_ref, cast_slice, cast_slice_mut, fill_zeroes, from_bytes,
  from_bytes_mut, pod_align_to, pod_align_to_mut, pod_read_unaligned, try_cast, try_cast_mut, try_cast_ref, try_cast_slice,
  try_cast_slice_mut, try_from_bytes, try_from_bytes_mut, try_pod_read_unaligned, write_zeroes, CheckedBitPattern,
  PodCastError, AnyBitPattern, Contiguous, NoUninit, Pod, PodInOption, TransparentWrapper, Zeroable, ZeroableInOption,
 };

  // Expose allocation submodule if into_bytes and extern_crate_alloc are enabled
  #[ cfg(all(feature = "into_bytes", feature = "extern_crate_alloc")) ]
  pub use bytemuck ::allocation;
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
  pub use exposed :: *;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed 
{
  use super::*;

  #[ doc( inline ) ]
  #[ cfg( feature = "as_bytes" ) ]
  pub use as_bytes ::exposed :: *;
  #[ doc( inline ) ]
  #[ cfg( feature = "into_bytes" ) ]
  pub use into_bytes ::exposed :: *;

  #[ doc( inline ) ]
  pub use prelude :: *;
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude 
{
  use super::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "as_bytes" ) ]
  pub use as_bytes ::prelude :: *;
  #[ doc( inline ) ]
  #[ cfg( feature = "into_bytes" ) ]
  pub use into_bytes ::prelude :: *;
}
