/// Define a private namespace for all its items.
mod private {}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own 
{
  #[ allow( unused_imports ) ]
  use super :: *;
  pub use orphan :: *;
  pub use private :: { };
  #[ cfg( feature = "string_parse_number" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports, clippy ::wildcard_imports ) ]
  pub use lexical :: *;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan 
{
  #[ allow( unused_imports ) ]
  use super :: *;
  pub use exposed :: *;
  pub use private :: { };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed 
{
  #[ allow( unused_imports ) ]
  use super :: *;
  pub use prelude :: *; // Added
  pub use super ::own as number;

  pub use private :: { };
}

/// Namespace of the module to include with `use module :: *`.
#[ allow( unused_imports ) ]
pub mod prelude 
{
  #[ allow( unused_imports ) ]
  use super :: *;
}
