use core::default::Default;

/// Private implementation details for the isolate module.
pub mod private {
  use super::*;

  /// Newtype for the source string slice.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct Src<'a>(pub &'a str);

  /// Newtype for the delimiter string slice.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct Delimeter<'a>(pub &'a str);

  /// Newtype for the quote boolean flag.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct Quote(pub bool);

  /// Newtype for the left boolean flag.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct Left(pub bool);

  /// Newtype for the none boolean flag.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct NoneFlag(pub bool);

  ///
  /// Options for isolate.
  ///
  #[ allow( dead_code ) ]
  #[ derive( Debug ) ] // Removed Assign derive
  pub struct IsolateOptions<'a> {
    /// Source string slice.
    pub src: Src<'a>,
    /// Delimiter string slice.
    pub delimeter: Delimeter<'a>,
    /// Quote boolean flag.
    pub quote: Quote,
    /// Left boolean flag.
    pub left: Left,
    /// Number of times to isolate.
    pub times: u8,
    /// None boolean flag.
    pub none: NoneFlag,
  }

  impl Default for IsolateOptions<'_> {
    fn default() -> Self {
      Self {
        src: Src::default(),
        delimeter: Delimeter::default(),
        quote: Quote::default(),
        left: Left::default(),
        times: 1,
        none: NoneFlag::default(),
      }
    }
  }

  impl<'a> IsolateOptions<'a> {
    /// Do isolate.
    #[ must_use ]
    pub fn isolate(&self) -> (&'a str, Option< &'a str >, &'a str) {
      let times = self.times + 1;
      let result;

      /* */

      let left_none_result = |src: &'a str| -> (&'a str, Option< &'a str >, &'a str) {
        if self.none.0 {
          ("", None, src)
        } else {
          (src, None, "")
        }
      };

      /* */

      let right_none_result = |src: &'a str| -> (&'a str, Option< &'a str >, &'a str) {
        if self.none.0 {
          (src, None, "")
        } else {
          ("", None, src)
        }
      };

      /* */

      let count_parts_len = |parts: &Vec< &str >| -> usize {
        let mut len = 0;
        for i in 0..self.times {
          let i = i as usize;
          if i > 0 {
            len += self.delimeter.0.len();
          }
          len += parts[i].len();
        }
        len
      };

      if self.left.0 {
        let parts: Vec< &str > = self.src.0.trim().splitn(times.into(), self.delimeter.0).collect();
        if parts.len() == 1 {
          result = left_none_result(parts[0]);
        } else {
          let len = count_parts_len(&parts);
          let max_len = len + self.delimeter.0.len();
          if max_len <= self.src.0.len() {
            let delim_opt = if self.delimeter.0.is_empty() {
              None
            } else {
              Some(self.delimeter.0)
            };
            result = (&self.src.0[0..len], delim_opt, &self.src.0[max_len..]);
          } else {
            result = left_none_result(self.src.0);
          }
        }
      } else {
        let parts: Vec< &str > = self.src.0.trim().rsplitn(times.into(), self.delimeter.0).collect();
        if parts.len() == 1 {
          result = right_none_result(parts[0]);
        } else {
          let len = count_parts_len(&parts);
          if len + self.delimeter.0.len() <= self.src.0.len() {
            let delim_opt = if self.delimeter.0.is_empty() {
              None
            } else {
              Some(self.delimeter.0)
            };
            result = (parts[parts.len() - 1], delim_opt, &self.src.0[self.src.0.len() - len..]);
          } else {
            result = right_none_result(self.src.0);
          }
        }
      }

      result
    }
  }

  ///
  /// Function to split a string with some delimeter.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///
  ///
  ///
  #[ must_use ]
  pub fn isolate<'a>() -> IsolateOptions<'a> {
    IsolateOptions::default()
  }

  ///
  /// Function to split a string with some delimeter. Routine splits string from left.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///
  ///
  ///
  #[ must_use ]
  pub fn isolate_left<'a>() -> IsolateOptions<'a> {
    IsolateOptions {
      left: Left(true),
      ..IsolateOptions::default()
    }
  }

  ///
  /// Function to split a string with some delimeter. Routine splits string from right.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///
  ///
  ///
  #[ must_use ]
  pub fn isolate_right<'a>() -> IsolateOptions<'a> {
    IsolateOptions {
      left: Left(false),
      ..IsolateOptions::default()
    }
  }
}

/// Owned namespace of the module.
#[ allow( unused_imports ) ]
pub mod own {
  #[ allow( unused_imports ) ]
  use super::*;
  use super::private as i;

  pub use orphan::*; // Added
  pub use i::IsolateOptions;
  // pub use i::IsolateOptionsAdapter; // Removed
  pub use i::isolate;
  pub use i::isolate_left;
  pub use i::isolate_right;
}

pub use own::*;

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*; // Added
  pub use super::own as isolate;

  use super::private as i;

  // pub use i::IsolateOptionsAdapter; // Removed
  pub use i::isolate;
  pub use i::isolate_left;
  pub use i::isolate_right;
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude {
  #[ allow( unused_imports ) ]
  use super::*;
  use super::private as i;

  // pub use i::IsolateOptionsAdapter; // Removed
}
