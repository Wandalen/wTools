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
      $fallback1 : ty,
      $fallback2 : ty
      $(,)?
    )
    =>
    {{
      (
        ::core::stringify!( $key ),
        $crate::MaybeAs::< 'a, str, $how >::from
        (
          $crate::to_string_with_fallback!( $how, $fallback1, $fallback2, $src )
        ),
      )
    }};
  }

  #[ macro_export ]
  macro_rules! _field
  {

    ( & $path:ident.$( $key:ident )+, $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # ( & $path . ) ( $( $key )+ ) ( $how, $fallback1, $fallback2 ) )
    }};

    ( $path:ident.$( $key:ident )+, $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # ( $path . ) ( $( $key )+ ) ( $how, $fallback1, $fallback2 ) )
    }};

    ( & $key:ident, $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # () ( $key ) ( $how, $fallback1, $fallback2 ) )
    }};

    ( $key:ident, $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # () ( $key ) ( $how, $fallback1, $fallback2 ) )
    }};

    // private

    (
      #
      ( $( $prefix:tt )* )
      ( $prekey:ident.$( $field:ident )+ )
      ( $how : ty, $fallback1 : ty, $fallback2 : ty )
    )
    =>
    {{
      $crate::_field!( # ( $( $prefix )* $prekey . ) ( $( $field )+ ) ( $how, $fallback1, $fallback2 ) )
    }};

    (
      #
      ( $( $prefix:tt )* )
      ( $key:ident )
      ( $how : ty, $fallback1 : ty, $fallback2 : ty )
    )
    =>
    {{
      $crate::_field!( # # ( $( $prefix )* ) ( $key ) ( $how, $fallback1, $fallback2 ) )
    }};

    (
      # #
      ( $( $prefix:tt )* )
      ( $key:ident )
      ( $how : ty, $fallback1 : ty, $fallback2 : ty )
    )
    =>
    {{
      $crate::_field_with_key!( $key, $( $prefix )* $key, $how, $fallback1, $fallback2 )
    }};

  }

  pub mod ref_or_display_or_debug
  {

    #[ macro_export ]
    macro_rules! ref_or_display_or_debug_field_with_key
    {
      (
        $key : ident,
        $src : expr
        $(,)?
      )
      =>
      {{
        $crate::_field_with_key!( $key, $src, $crate::WithRef, $crate::WithDisplay, $crate::WithDebug )
      }};
    }

    #[ macro_export ]
    macro_rules! ref_or_display_or_debug_field
    {
      ( $( $t:tt )+ )
      =>
      {{
        $crate::_field!( $( $t )+, $crate::WithRef, $crate::WithDisplay, $crate::WithDebug )
      }}
    }

    pub use ref_or_display_or_debug_field_with_key as field_with_key;
    pub use ref_or_display_or_debug_field as field;

  }

  pub mod ref_or_debug
  {

    #[ macro_export ]
    macro_rules! ref_or_debug_field_with_key
    {
      (
        $key : ident,
        $src : expr
        $(,)?
      )
      =>
      {{
        $crate::_field_with_key!( $key, $src, $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
      }};
    }

    #[ macro_export ]
    macro_rules! ref_or_debug_field
    {
      ( $( $t:tt )+ )
      =>
      {{
        $crate::_field!( $( $t )+, $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
      }}
    }

    pub use ref_or_debug_field_with_key as field_with_key;
    pub use ref_or_debug_field as field;

  }

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
    ref_or_debug,
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
