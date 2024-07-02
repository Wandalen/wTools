/// Internal namespace.
pub( crate ) mod private
{
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  use super::*;
  pub use orphan::*;
  #[ allow( unused_imports ) ]
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
  #[ allow( unused_imports ) ]
  pub use private::
  {
  };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::protected as number;

  #[ allow( unused_imports ) ]
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
