/// Internal namespace.
pub mod internal
{

  // pub trait i32x1
  // {
  // }

}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;

}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;

}
