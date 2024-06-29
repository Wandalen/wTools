//!
//! Keywords
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  // qqq : zzz : cover by tests
  /// Check is string a keyword.
  pub fn is( src : &str ) -> bool
  {
    // Create an identifier from the string
    let ident = Ident::new( src, Span::call_site() );

    // Check if it is a keyword or reserved word
    ident.is_reserved() || ident.is_keyword()
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
  use super::*;
  pub use super::super::tokens;
  // pub use super::protected as tokens;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    is,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

