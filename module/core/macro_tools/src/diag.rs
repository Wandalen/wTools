//!
//! Macro helpers.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

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

  /// Formats a debugging report for code transformation processes, detailing both the original and generated code for easy comparison and review.
  ///
  /// This function creates a structured report comprising the initial input code, the resulting generated code, and an explanatory context. It is designed to facilitate debugging and documentation of code transformations, such as those performed in procedural macros or similar code generation tasks. The report categorizes the information into labeled sections to enhance readability and traceability.
  ///
  /// This function helps visualize the changes from the original to the generated code, assisting developers in verifying and understanding the transformations applied during code generation processes.
  ///
  /// # Parameters
  ///
  /// - `about` : A description or context explaining the purpose or nature of the transformation. This information is displayed at the beginning of the report to provide an overview of the code transformation context.
  /// - `input` : The original code before transformation. This is typically the code that is subject to processing by macros or other code generation tools.
  /// - `output` : The code generated as a result of the transformation. This reflects the changes or enhancements made to the original code.
  ///
  /// # Type Parameters
  ///
  /// - `IntoAbout` : A type that can be converted into a string representation, providing a descriptive context for the report.
  /// - `IntoInput` : A type representing the original code, which can be converted into a string format for display.
  /// - `IntoOutput` : A type representing the generated code, which can be converted into a string format for display.
  ///
  /// # Returns
  ///
  /// A string containing the formatted debug report, organized into sections with appropriate labels and indentation to distinguish between the original and generated code segments.
  ///
  /// # Examples
  ///
  /// ```
  /// use macro_tools::exposed::*;
  ///
  /// let original_input : proc_macro2::TokenStream = quote!
  /// {
  ///   #[derive(Debug, PartialEq)]
  ///   pub struct MyStruct
  ///   {
  ///     pub field : i32,
  ///   }
  /// };
  ///
  /// let generated_code : proc_macro2::TokenStream = quote!
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
  /// let formatted_report = report_format( "Code Transformation for MyStruct", original_input, generated_code );
  /// println!( "{}", formatted_report );
  /// ```
  ///

  pub fn report_format< IntoAbout, IntoInput, IntoOutput >
  (
    about : IntoAbout, input : IntoInput, output : IntoOutput
  ) -> String
  where
    IntoAbout : ToString,
    IntoInput : ToString,
    IntoOutput : ToString,
  {
    format!( "\n" ) +
    &format!( " = context\n\n{}\n\n", indentation( "  ", about.to_string(), "" ) ) +
    &format!( " = original\n\n{}\n\n", indentation( "  ", input.to_string(), "" ) ) +
    &format!( " = generated\n\n{}\n", indentation( "  ", output.to_string(), "" ) )
  }

  /// Prints a debugging report for a pair of token streams to the standard output.
  ///
  /// This function acts as a utility for debugging transformations in procedural macros or other code generation scenarios.
  /// It provides an immediate visual comparison of the original code versus the generated code by utilizing the `report_format`
  /// function to format the output and then printing it directly to the standard output. This can be particularly helpful for
  /// real-time debugging and quick assessments without requiring additional output management.
  ///
  /// # Parameters and Type Parameters
  /// - `about` : A description of the code transformation context or operation. This is used to headline the generated report.
  /// - `input` : The original code or token stream before transformation. This is what the code looked like prior to any procedural manipulations.
  /// - `output` : The transformed or generated code or token stream as a result of the macro or code transformation process.
  ///
  /// The types for these parameters are expected to be convertible to strings, matching the `report_format` function's requirements.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use macro_tools::exposed::*;
  ///
  /// let original_input : proc_macro2::TokenStream = quote!
  /// {
  ///   #[derive(Debug, PartialEq)]
  ///   pub struct MyStruct
  ///   {
  ///     pub field : i32,
  ///   }
  /// };
  ///
  /// let generated_code : proc_macro2::TokenStream = quote!
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
  /// report_print( "Code Transformation for MyStruct", original_input, generated_code );
  /// ```
  ///
  /// The above example demonstrates how the `report_print` function can be used to visualize the changes from original input code to the generated code,
  /// helping developers to verify and understand the modifications made during code generation processes. The output is formatted to show clear distinctions
  /// between the 'original' and 'generated' sections, providing an easy-to-follow comparison.

  pub fn report_print< IntoAbout, IntoInput, IntoOutput >
  (
    about : IntoAbout, input : IntoInput, output : IntoOutput
  )
  where
    IntoAbout : ToString,
    IntoInput : ToString,
    IntoOutput : ToString,
  {
    println!( "{}", report_format( about, input, output ) );
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
    ( $src :expr ) =>
    {{
      let result = $crate::tree_diagnostics_str!( $src );
      println!( "{}", result );
      result
    }};
    ( $( $src :expr ),+ $(,)? ) =>
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
    ( $src :expr ) =>
    {{
      let result = $crate::code_diagnostics_str!( $src );
      println!( "{}", result );
      result
    }};
    ( $( $src :expr ),+ $(,)? ) =>
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
    ( $src :expr ) =>
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
    ( $src :expr ) =>
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
    ( $src :expr ) =>
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
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;

}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use private::
  // {
  //   Result,
  // };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::diag;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::
  {
    indentation,
    report_format,
    report_print,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::
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
  // pub use private::Result;
}
