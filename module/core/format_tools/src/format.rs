//!
//! Collection of mechanisms for formatting and serialization into string.
//!

/// Define a private namespace for all its items.
mod private
{

  /// Macro to create a field with a key and formatted value.
  ///
  /// This macro helps to convert a field of a structure into one or another string representation
  /// depending on the parameters `how`, `fallback1`, and `fallback2`. Unlike `_field_with_key`,
  /// the key is the path of the expression and is deduced from the last part of the expression.
  /// For example, for `this.is.field`, the key is `field`.

  #[ macro_export ]
  macro_rules! _field_with_key
  {
    (
      $path : expr,
      $key : ident,
      $how : ty,
      $fallback1 : ty,
      $fallback2 : ty
      $(,)?
    )
    =>
    {{
      (
        ::core::stringify!( $key ),
        // $crate::OptionalCow::< '_, str, $how >::from
        Option::Some
        (
          $crate::to_string_with_fallback!( $how, $fallback1, $fallback2, $path )
        ),
      )
    }};
  }

  /// Macro to create a field with optional fallbacks.
  ///
  /// This macro helps to convert a field of a structure into one or another string representation
  /// depending on the parameters `how`, `fallback1`, and `fallback2`. Unlike `_field_with_key`,
  /// the key is the path of the expression and is deduced from the last part of the expression.
  /// For example, for `this.is.field`, the key is `field`.

  #[ macro_export ]
  macro_rules! _field
  {

    // dst.push( field!( &self.id ) );
    ( ( & $pre:ident.$( $key:tt )+ ), $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # ( & $pre . ) ( $( $key )+ ) ( $how, $fallback1, $fallback2 ) )
    }};

    // dst.push( field!( self.id ) );
    ( ( $pre:ident.$( $key:tt )+ ), $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # ( $pre . ) ( $( $key )+ ) ( $how, $fallback1, $fallback2 ) )
    }};

    // dst.push( field!( &tools ) );
    ( ( & $key:ident ), $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # () ( & $key ) ( $how, $fallback1, $fallback2 ) )
    }};

    // dst.push( field!( tools ) );
    ( ( $key:ident ), $how : ty, $fallback1 : ty, $fallback2 : ty $(,)? ) =>
    {{
      $crate::_field!( # () ( $key ) ( $how, $fallback1, $fallback2 ) )
    }};

    // private

    // ( a.b. )
    // ( c.d )
    // ( $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
    (
      #
      ( $( $prefix:tt )* )
      ( $prekey:ident.$( $field:tt )+ )
      ( $how : ty, $fallback1 : ty, $fallback2 : ty )
    )
    =>
    {{
      $crate::_field!( # ( $( $prefix )* $prekey . ) ( $( $field )+ ) ( $how, $fallback1, $fallback2 ) )
    }};

    // ( a.b. )
    // ( 0.d )
    // ( $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
    (
      #
      ( $( $prefix:tt )* )
      ( $prekey:tt.$( $field:tt )+ )
      ( $how : ty, $fallback1 : ty, $fallback2 : ty )
    )
    =>
    {{
      $crate::_field!( # ( $( $prefix )* $prekey . ) ( $( $field )+ ) ( $how, $fallback1, $fallback2 ) )
    }};

    // ( a.b.c. )
    // ( d )
    // ( $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
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

    // ( a.b.c )
    // ( d )
    // ( $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
    (
      # #
      ( $( $prefix:tt )* )
      ( $key:ident )
      ( $how : ty, $fallback1 : ty, $fallback2 : ty )
    )
    =>
    {{
      // _field_with_key!( id, &self. id, $crate::WithRef, $crate::WithDisplay, $crate::WithDebugMultiline )
      $crate::_field_with_key!( $( $prefix )* $key, $key, $how, $fallback1, $fallback2 )
    }};

  }

  /// Converting representations to a reference on a string slice,
  /// but if not possible, to a display string, and if that is also not possible, then to a debug string.
  ///
  /// Macros for converting fields to different string representations in a prioritized manner:
  /// 1. Reference to a string slice.
  /// 2. Display string.
  /// 3. Debug string with miltiline.
  pub mod ref_or_display_or_debug_multiline
  {

    /// Macro to create a field with key using reference, display, or debug formatting.
    ///
    /// This macro attempts to convert the field to a reference to a string slice.
    /// If that is not possible, it tries to use the Display trait for conversion.
    /// If that also fails, it falls back to using the Debug trait with multiline.
    #[ macro_export ]
    macro_rules! ref_or_display_or_debug_multiline_field_with_key
    {
      (
        $key : ident,
        $src : expr
        $(,)?
      )
      =>
      {{
        $crate::_field_with_key!( $src, $key, $crate::WithRef, $crate::WithDisplay, $crate::WithDebugMultiline )
      }};
    }

    /// Macro to create a field using reference, display, or debug formatting.
    ///
    /// This macro attempts to convert the field to a reference to a string slice.
    /// If that is not possible, it tries to use the Display trait for conversion.
    /// If that also fails, it falls back to using the Debug trait with multiline.
    #[ macro_export ]
    macro_rules! ref_or_display_or_debug_multiline_field
    {
      ( $( $t:tt )+ )
      =>
      {{
        $crate::_field!( ( $( $t )+ ), $crate::WithRef, $crate::WithDisplay, $crate::WithDebugMultiline )
      }}
    }

    pub use ref_or_display_or_debug_multiline_field_with_key as field_with_key;
    pub use ref_or_display_or_debug_multiline_field as field;

  }

  /// Converting representations to a reference on a string slice,
  /// but if not possible, to a display string, and if that is also not possible, then to a debug string.
  ///
  /// Macros for converting fields to different string representations in a prioritized manner:
  /// 1. Reference to a string slice.
  /// 2. Display string.
  /// 3. Debug string.
  pub mod ref_or_display_or_debug
  {

    /// Macro to create a field with key using reference, display, or debug formatting.
    ///
    /// This macro attempts to convert the field to a reference to a string slice.
    /// If that is not possible, it tries to use the Display trait for conversion.
    /// If that also fails, it falls back to using the Debug trait.
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
        $crate::_field_with_key!( $src, $key, $crate::WithRef, $crate::WithDisplay, $crate::WithDebug )
      }};
    }

    /// Macro to create a field using reference, display, or debug formatting.
    ///
    /// This macro attempts to convert the field to a reference to a string slice.
    /// If that is not possible, it tries to use the Display trait for conversion.
    /// If that also fails, it falls back to using the Debug trait.
    #[ macro_export ]
    macro_rules! ref_or_display_or_debug_field
    {
      ( $( $t:tt )+ )
      =>
      {{
        $crate::_field!( ( $( $t )+ ), $crate::WithRef, $crate::WithDisplay, $crate::WithDebug )
      }}
    }

    pub use ref_or_display_or_debug_field_with_key as field_with_key;
    pub use ref_or_display_or_debug_field as field;

  }

  /// Converting representations to a reference on a string slice,
  /// but if not possible, to a debug string.
  ///
  /// Macros for converting fields to different string representations in a prioritized manner:
  /// 1. Reference to a string slice.
  /// 2. Debug string.
  ///
  pub mod ref_or_debug
  {

    /// Macro to create a field with key using reference or debug formatting.
    ///
    /// This macro attempts to convert the field to a reference to a string slice.
    /// If that is not possible, it falls back to using the Debug trait.
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
        $crate::_field_with_key!( $src, $key, $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
      }};
    }

    /// Macro to create a field using reference or debug formatting.
    ///
    /// This macro attempts to convert the field to a reference to a string slice.
    /// If that is not possible, it falls back to using the Debug trait.
    #[ macro_export ]
    macro_rules! ref_or_debug_field
    {
      ( $( $t:tt )+ )
      =>
      {{
        $crate::_field!( ( $( $t )+ ), $crate::WithRef, $crate::WithDebug, $crate::WithDebug )
      }}
    }

    pub use ref_or_debug_field_with_key as field_with_key;
    pub use ref_or_debug_field as field;

  }

}

pub mod as_table;
pub mod filter;
pub mod md_math;
pub mod output_format;
pub mod print;
pub mod string;
pub mod table;
pub mod to_string;
pub mod to_string_with_fallback;

/// A strucutre for diagnostic and demonstration purpose.
#[ doc( hidden ) ]
#[ cfg( debug_assertions ) ]
pub mod test_object_without_impl;

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
    as_table::orphan::*,
    filter::orphan::*,
    md_math::orphan::*,
    output_format::orphan::*,
    print::orphan::*,
    string::orphan::*,
    table::orphan::*,
    to_string::orphan::*,
    to_string_with_fallback::orphan::*,
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
    ref_or_display_or_debug_multiline,
    ref_or_debug,
  };

  #[ doc( hidden ) ]
  #[ cfg( debug_assertions ) ]
  pub use test_object_without_impl::
  {
    TestObjectWithoutImpl,
    test_objects_gen,
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use reflect_tools::OptionalCow;

  #[ doc( inline ) ]
  pub use
  {
    as_table::exposed::*,
    filter::exposed::*,
    md_math::exposed::*,
    output_format::exposed::*,
    print::exposed::*,
    string::exposed::*,
    table::exposed::*,
    to_string::exposed::*,
    to_string_with_fallback::exposed::*,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
    as_table::prelude::*,
    filter::prelude::*,
    md_math::prelude::*,
    output_format::prelude::*,
    print::prelude::*,
    string::prelude::*,
    table::prelude::*,
    to_string::prelude::*,
    to_string_with_fallback::prelude::*,
  };

}
