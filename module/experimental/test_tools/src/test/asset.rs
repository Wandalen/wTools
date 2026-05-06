//!
//! Test asset helper.
//!

/// Define a private namespace for all its items.
mod private
{
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own 
{
  use super :: *;

  #[ doc( inline ) ]
  pub use { };
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan 
{
  use super :: *;

  #[ doc( inline ) ]
  pub use exposed :: *;

  pub use super ::super ::asset;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed 
{
  use super :: *;

  #[ doc( inline ) ]
  pub use prelude :: *;

  #[ doc( inline ) ]
  pub use { };
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ allow( unused_imports ) ]
pub mod prelude 
{
  use super :: *;

  #[ doc( inline ) ]
  pub use { };
}
