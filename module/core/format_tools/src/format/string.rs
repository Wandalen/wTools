//!
//! String tools.
//!

// xxx : move to crate string_tools

/// Define a private namespace for all its items.
mod private
{

  // use crate::*;

  /// Returns the size of the text in `src` as a `[ width, height ]` array.
  ///
  /// This function calculates the dimensions of the input text, where the width is defined
  /// as the length of the longest line, and the height is the total number of lines. It
  /// handles various edge cases, including empty strings and strings with trailing newlines,
  /// to ensure accurate dimension calculation.
  ///
  /// # Arguments
  ///
  /// * `src` - A string slice or any type that can be referenced as a string. This allows
  ///   for flexibility in passing different string-like types.
  ///
  /// # Returns
  ///
  /// A `[usize; 2]` array representing the dimensions of the text:
  /// - `width`: The length of the longest line in the text.
  /// - `height`: The total number of lines in the text.
  ///
  /// # Nuances
  ///
  /// - **Empty Strings**: If the input string is empty, the function returns `[0, 1]`
  ///   because there is one line with a width of zero.
  /// - **Trailing Newlines**: If the input string ends with a newline character, it is
  ///   treated as having an additional empty line at the end.
  /// - **Empty Lines**: Empty lines within the text are counted as lines with a width of zero.
  ///
  /// # Examples
  ///
  /// ```
  /// let text = "Hello\nWorld\nThis is a test";
  /// let dimensions = format_tools::string::size( text );
  /// assert_eq!( dimensions, [ 14, 3 ] );
  /// ```
  ///
  /// In this example, the function returns `[ 14, 3 ]` because the longest line ( "This is a test" )
  /// has 14 characters, and there are 3 lines in total.
  ///
  /// ```
  /// let text = "";
  /// let dimensions = format_tools::string::size( text );
  /// assert_eq!( dimensions, [ 0, 1 ] );
  /// ```
  ///
  /// Here, the function returns `[0, 1]` because the input is an empty string, which is considered
  /// as a single line with zero width.
  ///
  /// ```
  /// let text = "Line 1\n\nLine 3\n";
  /// let dimensions = format_tools::string::size( text );
  /// assert_eq!( dimensions, [ 6, 4 ] );
  /// ```
  ///
  /// In this example, the function returns `[ 6, 4 ]` because the longest line ( "Line 1" or "Line 3" )
  /// has 6 characters, there are 4 lines in total, including the empty line and the trailing newline.

  pub fn size< S : AsRef< str > >( src : S ) -> [ usize ; 2 ]
  {
    let text = src.as_ref();
    let mut height = 0;
    let mut width = 0;

    for line in lines( text )
    {
      height += 1;
      let line_length = line.chars().count();
      if line_length > width
      {
        width = line_length;
      }
    }

    [ width, height ]
  }

  /// Returns an iterator over the lines of a string slice.
  ///
  /// This function provides an iterator that yields each line of the input string slice.
  /// It is an enhancement over the standard `str::lines()` method, as it handles trailing
  /// newlines by returning an additional empty line if the input string ends with a newline.
  ///
  /// # Arguments
  ///
  /// * `src` - A reference to a type that can be converted to a string slice. This allows
  ///   for flexibility in passing various string-like types.
  ///
  /// # Returns
  ///
  /// An iterator of type `Lines` that yields each line as a `&str`.
  ///
  /// # Examples
  ///
  /// ```
  /// let text = "Hello\nWorld\n";
  /// let mut lines = format_tools::string::lines( text );
  /// assert_eq!( lines.next(), Some( "Hello" ) );
  /// assert_eq!( lines.next(), Some( "World" ) );
  /// assert_eq!( lines.next(), Some( "" ) );
  /// assert_eq!( lines.next(), None );
  /// ```
  pub fn lines< S : AsRef< str > + ?Sized >( src : & S ) -> Lines< '_ >
  {
    Lines::new( src.as_ref() )
  }

  /// An iterator over the lines of a string slice.
  ///
  /// This struct implements the `Iterator` trait, allowing you to iterate over the lines
  /// of a string. It enhances the standard `str::Lines` iterator by handling trailing
  /// newlines, ensuring that an additional empty line is returned if the input string
  /// ends with a newline character.
  /// ```
  #[ derive( Debug ) ]
  pub struct Lines< 'a >
  {
    lines : std::str::Lines< 'a >,
    has_trailing_newline : bool,
    finished : bool,
  }
  impl< 'a > Lines< 'a >
  {
    fn new( input : &'a str ) -> Self
    {
      let has_trailing_newline = input.len() == 0 || input.ends_with( '\n' );
      Lines
      {
        lines : input.lines(),
        has_trailing_newline,
        finished : false,
      }
    }
  }

  impl< 'a > Iterator for Lines< 'a >
  {
    type Item = &'a str;

    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.finished
      {
        return None;
      }

      match self.lines.next()
      {
        Some( line ) => Some( line ),
        None =>
        {
          if self.has_trailing_newline
          {
            self.finished = true;
            Some( "" )
          }
          else
          {
            None
          }
        }
      }
    }
  }

}

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use private::
  {
    size,
    lines,
    Lines,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::string;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
