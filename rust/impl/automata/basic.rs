/// Internal namespace.
pub mod internal
{

}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  // pub use i::NanLikeInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  // pub use i::NanLikeInterface;
}
