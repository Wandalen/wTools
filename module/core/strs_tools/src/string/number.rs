/// Internal namespace.
mod private
{
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use orphan::*;
  pub use private::
  {
  };
  #[ cfg( all( feature = "string_parse_number" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use lexical::*;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use exposed::*;
  pub use private::
  {
  };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::own as number;

  pub use private::
  {
  };
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
