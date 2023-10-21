/// Internal namespace.
pub( crate ) mod private
{
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::private::
  {
  };
  #[ cfg( all( feature = "string_parse_number" ) ) ]
  #[ doc( inline ) ]
  pub use lexical::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
  pub use super::private::
  {
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::private::
  {
  };
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
}
