/// Internal namespace.
mod private
{
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super ::private ::
  {
 };
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed :: *;