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
      $key : ident,
      $src : expr,
      $how : ty,
      $fallback : ty
      $(,)?
    )
    =>
    {{
      (
        ::core::stringify!( $key ),
        $crate::MaybeAs::< 'a, str, $how >::from
        (
          $crate::to_string_with_fallback!( $how, $fallback, $src )
        ),
      )
    }};
  }

  // #[macro_use]
  pub mod ref_or_display_or_debug
  {

    #[ macro_export ]
    // #[ macro_use ]
    macro_rules! field_with_key
    {
      (
        $key : ident,
        $src : expr
        $(,)?
      )
      =>
      {{
        $crate::_field_with_key!( $key, $src, $crate::WithDisplay, $crate::WithDebug )
      }};
    }

    #[ macro_export ]
    // #[ macro_use ]
    macro_rules! field
    {

      ( & $path:ident.$( $key:ident )+ ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # ( & $path . ) ( $( $key )+ ) )
      }};

      ( $path:ident.$( $key:ident )+ ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # ( $path . ) ( $( $key )+ ) )
      }};

      ( & $key:ident ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # () ( $key ) )
      }};

      ( $key:ident ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # () ( $key ) )
      }};

      // private

      (
        #
        ( $( $prefix:tt )* )
        ( $prekey:ident.$( $field:ident )+ )
      ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # ( $( $prefix )* $prekey . ) ( $( $field )+ ) )
      }};

      (
        #
        ( $( $prefix:tt )* )
        ( $key:ident )
      ) =>
      {{
        $crate::ref_or_display_or_debug::field!( # # ( $( $prefix )* ) ( $key ) )
      }};

      (
        # #
        ( $( $prefix:tt )* )
        ( $key:ident )
      ) =>
      {{
        $crate::ref_or_display_or_debug::field_with_key!( $key, $( $prefix )* $key )
      }};

    }

    pub use field_with_key;
    pub use field;

  }

//   pub mod debug
//   {
//
//     #[ macro_export ]
//     macro_rules! field_with_key
//     {
//       (
//         $src : expr,
//         $key : ident $(,)?
//       )
//       =>
//       {{
//         $crate::_field_with_key!( $src, $key, $crate::WithDebug, $crate::WithDebug )
//       }};
//     }
//
//     #[ macro_export ]
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
//         ( $key:ident.$( $field:ident )+ )
//       ) =>
//       {{
//         $crate::debug::field!( # ( $( $prefix )* $key . ) ( $( $field )+ ) )
//       }};
//
//       (
//         #
//         ( $( $prefix:tt )* )
//         ( $key:ident )
//       ) =>
//       {{
//         $crate::debug::field!( # # ( $( $prefix )* ) ( $key ) )
//       }};
//
//       (
//         # #
//         ( $( $prefix:tt )* )
//         ( $key:ident )
//       ) =>
//       {{
//         $crate::debug::field_with_key!( $( $prefix )* $key, $key )
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

// use private::ref_or_display_or_debug::field as xxx;
// use private::field as xxx;

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
