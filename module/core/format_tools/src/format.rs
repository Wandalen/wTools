//!
//! Collection of mechanisms for formatting and serialization into string.
//!

/// Internal namespace.
pub( crate ) mod private
{

  #[ macro_export ]
  macro_rules! _field_with_key
  {
    (
      $src : expr,
      $name : ident,
      $how : ty,
      $fallback : ty
      $(,)?
    )
    =>
    {{
      (
        ::core::stringify!( $name ),
        $crate::MaybeAs::< 'a, str, $how >::from
        (
          $crate::to_string_with_fallback!( $how, $fallback, $src )
        ),
      )
    }};
  }

  pub mod ref_or_display_or_debug
  {

    #[ macro_export ]
    // #[ macro_use ]
    macro_rules! field_with_key
    {
      (
        $src : expr,
        $name : ident $(,)?
      )
      =>
      {{
        $crate::_field_with_key!( $src, $name, $crate::WithDisplay, $crate::WithDebug )
      }};
    }

    #[ macro_export ]
    // #[ macro_use ]
    macro_rules! field
    {

      ( & $path:ident.$( $field:ident )+ ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # ( & $path . ) ( $( $field )+ ) )
      }};

      ( $path:ident.$( $field:ident )+ ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # ( $path . ) ( $( $field )+ ) )
      }};

      ( & $field:ident ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # () ( $field ) )
      }};

      ( $field:ident ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # () ( $field ) )
      }};

      // private

      (
        #
        ( $( $prefix:tt )* )
        ( $name:ident.$( $field:ident )+ )
      ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # ( $( $prefix )* $name . ) ( $( $field )+ ) )
      }};

      (
        #
        ( $( $prefix:tt )* )
        ( $name:ident )
      ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # # ( $( $prefix )* ) ( $name ) )
      }};

      (
        # #
        ( $( $prefix:tt )* )
        ( $name:ident )
      ) =>
      {{
        $crate::ref_or_display_or_debug::field_with_key!( $( $prefix )* $name, $name )
      }};

    }

    pub use field_with_key;
    pub use field;

  }

//   pub mod debug
//   {
//
//     // #[ macro_export( local_inner_macros ) ]
//     macro_rules! field_with_key
//     {
//       (
//         $src : expr,
//         $name : ident $(,)?
//       )
//       =>
//       {{
//         $crate::_field_with_key!( $src, $name, $crate::WithDebug, $crate::WithDebug )
//       }};
//     }
//
//     // #[ macro_export( local_inner_macros ) ]
//     macro_rules! field
//     {
//
//       ( & $path:ident.$( $field:ident )+ ) =>
//       {{
//         $crate::debug::field!( # ( & $path . ) ( $( $field )+ ) )
//       }};
//
//       ( $path:ident.$( $field:ident )+ ) =>
//       {{
//         $crate::debug::field!( # ( $path . ) ( $( $field )+ ) )
//       }};
//
//       ( & $field:ident ) =>
//       {{
//         $crate::debug::field!( # () ( $field ) )
//       }};
//
//       ( $field:ident ) =>
//       {{
//         $crate::debug::field!( # () ( $field ) )
//       }};
//
//       // private
//
//       (
//         #
//         ( $( $prefix:tt )* )
//         ( $name:ident.$( $field:ident )+ )
//       ) =>
//       {{
//         $crate::debug::field!( # ( $( $prefix )* $name . ) ( $( $field )+ ) )
//       }};
//
//       (
//         #
//         ( $( $prefix:tt )* )
//         ( $name:ident )
//       ) =>
//       {{
//         $crate::debug::field!( # # ( $( $prefix )* ) ( $name ) )
//       }};
//
//       (
//         # #
//         ( $( $prefix:tt )* )
//         ( $name:ident )
//       ) =>
//       {{
//         $crate::debug::field_with_key!( $( $prefix )* $name, $name )
//       }};
//
//     }
//
//     pub use field_with_key;
//     pub use field;
//
//   }

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
    ref_or_display_or_debug,
    // field_with_key,
    // field,
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
