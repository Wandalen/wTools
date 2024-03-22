//!
//! Macro helpers.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  ///
  /// Result with syn::Error.
  ///

  pub type Result< T > = std::result::Result< T, syn::Error >;

  /// Adds indentation and optional prefix/postfix to each line of the given string.
  ///
  /// This function iterates over each line in the input string and applies the specified
  /// prefix and postfix to it, effectively indenting the string and optionally wrapping
  /// each line with additional content.
  ///
  /// # Parameters
  /// - `prefix` : The string to prepend to each line, typically used for indentation.
  /// - `src` : The source string to be indented and modified.
  /// - `postfix` : The string to append to each line, can be used for line terminators or other suffixes.
  ///
  /// # Type Parameters
  /// - `Prefix` : A type that can be referenced as a string slice, for the prefix.
  /// - `Src` : A type that can be referenced as a string slice, for the source string.
  /// - `Postfix` : A type that can be referenced as a string slice, for the postfix.
  ///
  /// # Returns
  /// A `String` that represents the original `src` string with `prefix` and `postfix` applied to each line.
  ///
  /// # Example
  /// ```
  /// use macro_tools::diag;
  ///
  /// let input = "Line 1\nLine 2\nLine 3";
  /// let indented = diag::indentation( "  ", input, ";" );
  /// assert_eq!( indented, "  Line 1;\n  Line 2;\n  Line 3;" );
  ///
  /// // Demonstrating the function's handling of trailing newlines
  /// let input_with_newline = "Line 1\nLine 2\nLine 3\n";
  /// let indented_with_newline = diag::indentation( "  ", input_with_newline, ";" );
  /// assert_eq!( indented_with_newline, "  Line 1;\n  Line 2;\n  Line 3;\n  ;" );
  /// ```
  ///
  /// In the example above, `indentation` is used to add two spaces before each line
  /// and a semicolon at the end of each line. The function also demonstrates handling
  /// of input strings that end with a newline character by appending an additional line
  /// consisting only of the prefix and postfix.

  pub fn indentation< Prefix, Src, Postfix >( prefix : Prefix, src : Src, postfix : Postfix ) -> String
  where
    Prefix : AsRef< str >,
    Src : AsRef< str >,
    Postfix : AsRef< str >,
  {
    let prefix = prefix.as_ref();
    let postfix = postfix.as_ref();
    let src = src.as_ref();

    let mut result = src
    .lines()
    .enumerate()
    .fold( String::new(), | mut a, b |
    {
      if b.0 > 0
      {
        a.push_str( "\n" );
      }
      a.push_str( prefix );
      a.push_str( &b.1 );
      a.push_str( postfix );
      a
    });

    if src.ends_with( "\n" ) || src.ends_with( "\n\r" ) || src.ends_with( "\r\n" )
    {
      result.push_str( "\n" );
      result.push_str( prefix );
      result.push_str( postfix );
    }

    result
  }

  /// Formats a debugging report for a pair of token streams, showing the original and generated code.
  ///
  /// This function takes two inputs: the original code as an `IntoTokens` (which can be converted into a `proc_macro2::TokenStream`),
  /// and the generated code as a `proc_macro2::TokenStream`. It formats both inputs with indentation for better readability,
  /// labeling them as "original" and "generated" respectively.
  ///
  /// Ensure the correct conversion of `proc_macro::TokenStream` to `proc_macro2::TokenStream` where necessary,
  /// especially when interfacing with procedural macros' `input` parameter
  ///
  /// # Parameters
  /// - `input`: The original input code that can be converted into a `proc_macro2::TokenStream`.
  /// - `output`: The generated code as a `proc_macro2::TokenStream`.
  ///
  /// # Returns
  /// A `String` containing the formatted debug report.
  ///
  /// # Type Parameters
  /// - `IntoTokens`: A type that can be converted into a `proc_macro2::TokenStream`.
  ///
  /// # Examples
  /// ```
  /// use macro_tools::exposed::*;
  ///
  /// let original_input : proc_macro2::TokenStream = qt!
  /// {
  ///   #[ derive( Debug, PartialEq ) ]
  ///   pub struct MyStruct
  ///   {
  ///     pub field : i32,
  ///   }
  /// };
  ///
  /// let generated_code : proc_macro2::TokenStream = qt!
  /// {
  ///   impl MyStruct
  ///   {
  ///     pub fn new( field : i32 ) -> Self
  ///     {
  ///       MyStruct { field }
  ///     }
  ///   }
  /// };
  ///
  /// // Format the debug report for printing or logging
  /// let formatted_report = debug_report_format( "derive :: MyDerive", original_input, &generated_code );
  /// println!( "{}", formatted_report );
  /// ```
  ///
  /// This will output a formatted report showing the original input code and the generated code side by side,
  /// each line indented for clarity.
  ///
  pub fn debug_report_format< IntoAbout, IntoTokens >
  (
    about : IntoAbout, input : IntoTokens, output : &proc_macro2::TokenStream
  ) -> String
  where
    IntoAbout : Into< String >,
    // xxx : qqq : use AsRef<>
    IntoTokens : Into< proc_macro2::TokenStream >,
  {
    format!( "\n" ) +
    &format!( " = context\n\n{}\n\n", indentation( "  ", about.into(), "" ) ) +
    &format!( " = original\n\n{}\n\n", indentation( "  ", input.into().to_string(), "" ) ) +
    &format!( " = generated\n\n{}\n", indentation( "  ", qt!{ #output }.to_string(), "" ) )
  }

  /// Prints a debugging report for a pair of token streams to the standard output.
  ///
  /// This convenience function wraps `debug_report_format`, directly printing the formatted report to stdout.
  /// It serves as a utility for debugging procedural macros, providing a clear comparison between original
  /// and generated code.
  ///
  /// # Parameters and Type Parameters
  /// - Same as `debug_report_format`.
  ///
  /// # Examples
  ///
  /// ```
  /// use macro_tools::exposed::*;
  ///
  /// let original_input : proc_macro2::TokenStream = qt!
  /// {
  ///   #[ derive( Debug, PartialEq ) ]
  ///   pub struct MyStruct
  ///   {
  ///     pub field : i32,
  ///   }
  /// };
  ///
  /// let generated_code : proc_macro2::TokenStream = qt!
  /// {
  ///   impl MyStruct
  ///   {
  ///     pub fn new( field : i32 ) -> Self
  ///     {
  ///       MyStruct { field }
  ///     }
  ///   }
  /// };
  ///
  /// // Directly print the debug report
  /// debug_report_print( "derive :: MyDerive", original_input, &generated_code );
  /// ```
  ///
  /// This will output a formatted report showing the original input code and the generated code side by side,
  /// each line indented for clarity.

  pub fn debug_report_print< IntoAbout, IntoTokens >
  (
    about : IntoAbout, input : IntoTokens, output : &proc_macro2::TokenStream
  )
  where
    IntoAbout : Into< String >,
    IntoTokens : Into< proc_macro2::TokenStream >,
  {
    println!( "{}", debug_report_format( about, input, output ) );
  }

  ///
  /// Macro for diagnostics purpose to print both syntax tree and source code behind it with syntax tree.
  ///
  /// ### Basic use-case.
  /// ```
  /// use macro_tools::prelude::*;
  ///
  /// let code = qt!( std::collections::HashMap< i32, i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// tree_print!( tree_type );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! tree_print
  {
    ( $src:expr ) =>
    {{
      let result = $crate::tree_diagnostics_str!( $src );
      println!( "{}", result );
      result
    }};
    ( $( $src:expr ),+ $(,)? ) =>
    {{
      $( $crate::tree_print!( $src ) );+
    }};
  }

  ///
  /// Macro for diagnostics purpose to print both syntax tree and source code behind it without syntax tree.
  ///
  /// ### Basic use-case.
  /// ```
  /// use macro_tools::prelude::*;
  ///
  /// let code = qt!( std::collections::HashMap< i32, i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// tree_print!( tree_type );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! code_print
  {
    ( $src:expr ) =>
    {{
      let result = $crate::code_diagnostics_str!( $src );
      println!( "{}", result );
      result
    }};
    ( $( $src:expr ),+ $(,)? ) =>
    {{
      $( $crate::code_print!( $src ) );+
    }};
  }

  ///
  /// Macro for diagnostics purpose to export both syntax tree and source code behind it into a string.
  ///

  #[ macro_export ]
  macro_rules! tree_diagnostics_str
  {
    ( $src:expr ) =>
    {{
      let src2 = &$src;
      format!( "{} : {} :\n{:#?}", stringify!( $src ), $crate::qt!{ #src2 }, $src )
    }};
  }

  ///
  /// Macro for diagnostics purpose to diagnose source code behind it and export it into a string.
  ///

  #[ macro_export ]
  macro_rules! code_diagnostics_str
  {
    ( $src:expr ) =>
    {{
      let src2 = &$src;
      format!( "{} : {}", stringify!( $src ), $crate::qt!{ #src2 } )
    }};
  }

  ///
  /// Macro to export source code behind a syntax tree into a string.
  ///

  #[ macro_export ]
  macro_rules! code_to_str
  {
    ( $src:expr ) =>
    {{
      let src2 = &$src;
      format!( "{}", $crate::qt!{ #src2 } )
    }};
  }

  ///
  /// Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.
  ///
  /// ### Basic use-case.
  /// ```
  /// # use macro_tools::exposed::*;
  /// syn_err!( "No attr" );
  /// # ()
  /// ```
  ///

  #[ macro_export ]
  macro_rules! syn_err
  {

    ( $msg:expr $(,)? ) =>
    {
      $crate::syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( _, $msg:expr $(,)? ) =>
    {
      $crate::syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( $span:expr, $msg:expr $(,)? ) =>
    {
      $crate::syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), $msg )
    };
    ( $span:expr, $msg:expr, $( $arg:expr ),+ $(,)? ) =>
    {
      $crate::syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), format!( $msg, $( $arg ),+ ) )
    };
    ( _, $msg:expr, $( $arg:expr ),+ $(,)? ) =>
    {
      $crate::syn::Error::new( proc_macro2::Span::call_site(), format!( $msg, $( $arg ),+ ) )
    };

  }

  ///
  /// Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.
  ///
  /// ### Basic use-case.
  /// ```
  /// # use macro_tools::exposed::*;
  /// syn_err!( "No attr" );
  /// # ()
  /// ```
  ///

  #[ macro_export ]
  macro_rules! return_syn_err
  {
    ( $( $Arg : tt )* ) =>
    {
      return Result::Err( $crate::syn_err!( $( $Arg )* ) )
    };
  }

  pub use
  {
    tree_print,
    code_print,
    tree_diagnostics_str,
    code_diagnostics_str,
    code_to_str,
    syn_err,
    return_syn_err,
  };

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

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as diag;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    Result,
    indentation,
    debug_report_format,
    debug_report_print,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    tree_print,
    code_print,
    tree_diagnostics_str,
    code_diagnostics_str,
    code_to_str,
    syn_err,
    return_syn_err,
  };

  // #[ doc( inline ) ]
  // pub use super::private::Result;
}
