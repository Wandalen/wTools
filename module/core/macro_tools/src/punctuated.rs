// ! xxx : write description

/// Internal namespace.
pub( crate ) mod private
{

  /// Ensures that a `syn::punctuated::Punctuated` collection ends with a comma if it contains elements.
  pub fn ensure_trailing_comma< T : Clone >
  ( punctuated : &mut syn::punctuated::Punctuated< T, syn::token::Comma > )
  {
    if !punctuated.empty_or_trailing()
    {
      punctuated.push_punct( syn::token::Comma::default() );
    }
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    ensure_trailing_comma,
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as punctuated;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
