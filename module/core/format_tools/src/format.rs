//!
//! Collection of mechanisms for formatting and serialization into string.
//!

/// Internal namespace.
pub( crate ) mod private
{

  #[ macro_export( local_inner_macros ) ]
  macro_rules! field_with_key
  {
    (
      $src : expr,
      $name : ident $(,)?
    )
    =>
    {{
      (
        ::core::stringify!( $name ),
        $crate::MaybeAs::< 'a, str, WithDisplay >::from
        (
          $crate::to_string_with_fallback!( WithDisplay, WithDebug, $src )
        ),
      )
    }};
  }

  #[ macro_export( local_inner_macros ) ]
  macro_rules! field
  {

    ( & $path:ident.$( $field:ident )+ ) =>
    {{
      field!( # ( & $path . ) ( $( $field )+ ) )
    }};

    ( $path:ident.$( $field:ident )+ ) =>
    {{
      field!( # ( $path . ) ( $( $field )+ ) )
    }};

    ( & $field:ident ) =>
    {{
      field!( # () ( $field ) )
    }};

    ( $field:ident ) =>
    {{
      field!( # () ( $field ) )
    }};

    // private

    (
      #
      ( $( $prefix:tt )* )
      ( $name:ident.$( $field:ident )+ )
    ) =>
    {{
      field!( # ( $( $prefix )* $name . ) ( $( $field )+ ) )
    }};

    (
      #
      ( $( $prefix:tt )* )
      ( $name:ident )
    ) =>
    {{
      field!( # # ( $( $prefix )* ) ( $name ) )
    }};

    (
      # #
      ( $( $prefix:tt )* )
      ( $name:ident )
    ) =>
    {{
      $crate::field_with_key!( $( $prefix )* $name, $name )
    }};

  }

  pub use field_with_key;
  pub use field;
}

pub mod to_string;
pub mod to_string_with_fallback;
pub mod wrapper;

pub mod as_table;
pub mod print;
pub mod table;

// xxx2 : continue

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use super::
  {
    to_string::orphan::*,
    to_string_with_fallback::orphan::*,
    wrapper::orphan::*,
    as_table::orphan::*,
    print::orphan::*,
    table::orphan::*,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use private::
  {
    field_with_key,
    field,
  };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use super::
  {
    to_string::exposed::*,
    to_string_with_fallback::exposed::*,
    wrapper::exposed::*,
    as_table::exposed::*,
    print::exposed::*,
    table::exposed::*,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use super::
  {
    to_string::prelude::*,
    to_string_with_fallback::prelude::*,
    wrapper::prelude::*,
    as_table::prelude::*,
    print::prelude::*,
    table::prelude::*,
  };

}
