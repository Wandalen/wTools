//! Core types for string splitting.

#[ cfg( feature = "std" ) ]
use std::{ borrow::Cow, vec, vec::Vec, string::String };
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::{ borrow::Cow, vec, vec::Vec, string::String };
use super::SplitFlags;

/// Helper function to unescape common escape sequences in a string.
/// Returns a `Cow::Borrowed` if no unescaping is needed, otherwise `Cow::Owned`.
#[ allow( clippy::elidable_lifetime_names ) ] // Design Rulebook requires explicit lifetimes
pub( super ) fn unescape_str< 'a >( input : &'a str ) -> Cow< 'a, str >
{
  if !input.contains( '\\' )
  {
    return Cow::Borrowed( input );
  }

  let mut output = String::with_capacity( input.len() );
  let mut chars = input.chars();

  while let Some(ch) = chars.next() {
    if ch == '\\' {
      if let Some(next_ch) = chars.next() {
        match next_ch {
          '"' => output.push('"'),
          '\\' => output.push('\\'),
          'n' => output.push('\n'),
          't' => output.push('\t'),
          'r' => output.push('\r'),
          '\'' => output.push('\''),
          _ => {
            output.push('\\');
            output.push(next_ch);
          }
        }
      } else {
        output.push('\\');
      }
    } else {
      output.push(ch);
    }
  }

  Cow::Owned(output)
}

/// Represents a segment of a string after splitting.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Split<'a> {
  /// The string content of the segment.
  pub string: Cow<'a, str>,
  /// The type of the segment (delimited or delimiter).
  pub typ: SplitType,
  /// The starting byte index of the segment in the original string.
  pub start: usize,
  /// The ending byte index of the segment in the original string.
  pub end: usize,
  /// Indicates if the original segment was quoted.
  pub was_quoted: bool,
}

impl<'a> From<Split<'a>> for String {
  fn from(src: Split<'a>) -> Self {
    src.string.into_owned()
  }
}

/// Defines the type of a split segment.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum SplitType {
  /// A segment of delimited content.
  Delimited,
  /// A segment representing a delimiter.
  Delimiter,
}

/// Trait for finding the position of a delimiter pattern in a string.
pub trait Searcher {
  /// Finds the first occurrence of the delimiter pattern in `src`.
  /// Returns `Some((start_index, end_index))` if found, `None` otherwise.
  fn pos(&self, src: &str) -> Option< (usize, usize) >;
}

impl Searcher for &str {
  fn pos(&self, src: &str) -> Option< (usize, usize) > {
    if self.is_empty() {
      return None;
    }
    src.find(self).map(|start| (start, start + self.len()))
  }
}

impl Searcher for String {
  fn pos(&self, src: &str) -> Option< (usize, usize) > {
    if self.is_empty() {
      return None;
    }
    src.find(self).map(|start| (start, start + self.len()))
  }
}

impl Searcher for Vec< &str > {
  fn pos(&self, src: &str) -> Option< (usize, usize) > {
    let mut r = vec![];
    for pat in self {
      if pat.is_empty() {
        continue;
      }
      if let Some(x) = src.find(pat) {
        r.push((x, x + pat.len()));
      }
    }
    if r.is_empty() {
      return None;
    }
    r.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| (a.1 - a.0).cmp(&(b.1 - b.0))));
    r.first().copied()
  }
}

/// Options to configure the behavior of split iterators.
#[ derive( Debug, Clone ) ]
pub struct SplitOptions<'a, D>
where
  D: Searcher + Default + Clone,
{
  pub( super ) src: &'a str,
  pub( super ) delimiter: D,
  pub( super ) flags: SplitFlags,
  pub( super ) quoting_prefixes: Vec< &'a str >,
  pub( super ) quoting_postfixes: Vec< &'a str >,
}

/// Adapter trait to provide split options to iterators.
pub trait SplitOptionsAdapter<'a, D>
where
  D: Searcher + Default + Clone,
{
  /// Gets the source string to be split.
  fn src(&self) -> &'a str;
  /// Gets the delimiter(s) to use for splitting.
  fn delimiter(&self) -> D;
  /// Gets the behavior flags for splitting.
  fn flags(&self) -> SplitFlags;
  /// Gets the prefixes that denote the start of a quoted section.
  fn quoting_prefixes(&self) -> &Vec< &'a str >;
  /// Gets the postfixes that denote the end of a quoted section.
  fn quoting_postfixes(&self) -> &Vec< &'a str >;
  /// Clones the options, specifically for initializing a `SplitFastIterator`.
  fn clone_options_for_sfi(&self) -> SplitOptions<'a, D>;
}

impl<'a, D: Searcher + Clone + Default> SplitOptionsAdapter<'a, D> for SplitOptions<'a, D> {
  fn src(&self) -> &'a str {
    self.src
  }
  fn delimiter(&self) -> D {
    self.delimiter.clone()
  }
  fn flags(&self) -> SplitFlags {
    self.flags
  }
  fn quoting_prefixes(&self) -> &Vec< &'a str > {
    &self.quoting_prefixes
  }
  fn quoting_postfixes(&self) -> &Vec< &'a str > {
    &self.quoting_postfixes
  }
  fn clone_options_for_sfi(&self) -> SplitOptions<'a, D> {
    self.clone()
  }
}
