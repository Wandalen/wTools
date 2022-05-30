
/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use ::inspect_type::orphan::*;
  pub use ::is_slice::orphan::*;
  pub use ::implements::orphan::*;
}

#[ doc( inline ) ]
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
  pub use ::inspect_type::exposed::*;
  pub use ::is_slice::exposed::*;
  pub use ::implements::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use ::inspect_type::prelude::*;
  pub use ::is_slice::prelude::*;
  pub use ::implements::prelude::*;
}
