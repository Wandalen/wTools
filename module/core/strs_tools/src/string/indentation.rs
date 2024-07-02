/// Internal namespace.
pub( crate ) mod private
{

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
  /// use strs_tools::exposed::*;
  ///
  /// let input = "Line 1\nLine 2\nLine 3";
  /// let indented = indentation( "  ", input, ";" );
  /// assert_eq!( indented, "  Line 1;\n  Line 2;\n  Line 3;" );
  ///
  /// // Demonstrating the function's handling of trailing newlines
  /// let input_with_newline = "Line 1\nLine 2\nLine 3\n";
  /// let indented_with_newline = indentation( "  ", input_with_newline, ";" );
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

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  pub use super::orphan::*;
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  pub use super::exposed::*;
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::protected as indentation;

  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    indentation,
  };
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
}
