/// Define a private namespace for all its items.
mod private {
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
  ///   let iter = strs_tools::string::split()
  ///   .src( "abc def" )
  ///   .delimeter( " " )
  ///   .perform();
  /// ```
  ///
  /// In the example above, `indentation` is used to add two spaces before each line
  /// and a semicolon at the end of each line. The function also demonstrates handling
  /// of input strings that end with a newline character by appending an additional line
  /// consisting only of the prefix and postfix.
  pub fn indentation<Prefix, Src, Postfix>(prefix: Prefix, src: Src, postfix: Postfix) -> String
  where
    Prefix: AsRef<str>,
    Src: AsRef<str>,
    Postfix: AsRef<str>,
  {
    let prefix = prefix.as_ref();
    let postfix = postfix.as_ref();
    let src = src.as_ref();

    let mut result = src.lines().enumerate().fold(String::new(), |mut a, b| {
      if b.0 > 0 {
        a.push('\n');
      }
      a.push_str(prefix);
      a.push_str(b.1);
      a.push_str(postfix);
      a
    });

    if src.ends_with('\n') || src.ends_with("\n\r") || src.ends_with("\r\n") {
      result.push('\n');
      result.push_str(prefix);
      result.push_str(postfix);
    }

    result
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use orphan::*;
  pub use private::{};
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
  pub use private::{};
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*; // Added
  pub use super::own as indentation;

  pub use private::{indentation};
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude {
  #[ allow( unused_imports ) ]
  use super::*;
}
