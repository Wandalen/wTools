//!
//! String tools.
//!

// xxx : move to crate string_tools

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;

  /// Returns the size of the text in `src` as a `[ width, height ]` array.
  ///
  /// The width is the length of the longest line, and the height is the number of lines.
  ///
  /// # Arguments
  ///
  /// * `src` - A string slice or any type that can be referenced as a string.
  ///
  /// # Examples
  ///
  /// ```
  /// let text = "Hello\nWorld\nThis is a test";
  /// let dimensions = format_tools::string::size( text );
  /// assert_eq!( dimensions, [14, 3 ] );
  /// ```
  ///
  /// The function returns `[ 14, 3 ]` because the longest line ("This is a test") has 14 characters,
  /// and there are 3 lines in total.

  pub fn size< S : AsRef< str > >( src : S ) -> [ usize ; 2 ]
  {
    let text = src.as_ref();
    let mut height = 0;
    let mut width = 0;

    for line in text.lines()
    {
      height += 1;
      let line_length = line.chars().count();
      if line_length > width
      {
        width = line_length;
      }
    }

    // Handle the case where the input ends with a newline character
    if text.ends_with( '\n' )
    {
      height += 1;
    }

    // Special case for an empty string
    if text.is_empty()
    {
      height = 1;
    }

    [ width, height ]
  }

//   pub fn size< S : AsRef< str > >( src : S ) -> [ usize; 2 ]
//   {
//     let text = src.as_ref();
//     let mut height = 0;
//     let mut width = 0;
//
//     for line in text.lines()
//     {
//       height += 1;
//       let line_length = line.len();
//       if line_length > width
//       {
//         width = line_length;
//       }
//     }
//
//     [ width, height ]
//   }

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
