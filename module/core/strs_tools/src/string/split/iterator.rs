//! Iterator implementations for string splitting.

#[ cfg( feature = "std" ) ]
use std::{ borrow::Cow, vec::Vec, string::ToString };
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::{ borrow::Cow, vec::Vec, string::ToString };
use super::SplitFlags;
use super::types::
{
  unescape_str,
  Split,
  SplitType,
  Searcher,
  SplitOptionsAdapter,
};

/// An iterator that quickly splits a string based on a delimiter, without advanced options.
#[ derive( Debug ) ]
pub( super ) struct SplitFastIterator<'a, D>
where
  D: Searcher,
{
  iterable: &'a str,
  current_offset: usize,
  counter: i32,
  delimiter: D,
  done: bool,
}

impl<'a, D: Searcher + Default + Clone> SplitFastIterator<'a, D> {
  pub( super ) fn new(o: &impl SplitOptionsAdapter<'a, D>) -> Self {
    Self {
      iterable: o.src(),
      current_offset: 0,
      delimiter: o.delimiter(),
      counter: 0,
      done: false,
    }
  }
}

impl<'a, D: Searcher> Iterator for SplitFastIterator<'a, D> {
  type Item = Split<'a>;
  #[ allow( clippy::too_many_lines ) ]
  fn next(&mut self) -> Option< Self::Item > {
    if self.done
    {
      return None;
    }
    // Fix(BUG-002): When iterable is exhausted after a delimiter phase (even counter),
    // yield one trailing empty content segment before stopping.
    // Root cause: the early return unconditionally stopped when iterable was empty,
    // even when the last yielded item was a delimiter that should be followed by
    // an empty content segment (matching str::split behavior).
    // Pitfall: alternating content/delimiter phases mean empty iterable after
    // delimiter phase requires one more empty content yield.
    if self.iterable.is_empty() && self.counter > 0 {
      self.done = true;
      if self.counter % 2 == 0
      {
        return Some( Split {
          string: Cow::Borrowed( "" ),
          typ: SplitType::Delimited,
          start: self.current_offset,
          end: self.current_offset,
          was_quoted: false,
        } );
      }
      return None;
    }
    self.counter += 1;
    if self.counter % 2 == 1 {
      if let Some((d_start, _d_end)) = self.delimiter.pos(self.iterable) {
        if d_start == 0 {
          return Some(Split {
            string: Cow::Borrowed(""),
            typ: SplitType::Delimited,
            start: self.current_offset,
            end: self.current_offset,
            was_quoted: false,
          });
        }
        let segment_str = &self.iterable[..d_start];
        let split = Split {
          string: Cow::Borrowed(segment_str),
          typ: SplitType::Delimited,
          start: self.current_offset,
          end: self.current_offset + segment_str.len(),
          was_quoted: false,
        };
        self.current_offset += segment_str.len();
        self.iterable = &self.iterable[d_start..];
        Some(split)
      } else {
        if self.iterable.is_empty() && self.counter > 1 {
          return None;
        }
        let segment_str = self.iterable;
        let split = Split {
          string: Cow::Borrowed(segment_str),
          typ: SplitType::Delimited,
          start: self.current_offset,
          end: self.current_offset + segment_str.len(),
          was_quoted: false,
        };
        self.current_offset += segment_str.len();
        self.iterable = "";
        Some(split)
      }
    } else if let Some((d_start, d_end)) = self.delimiter.pos(self.iterable) {
      if d_start > 0 {
        self.iterable = "";
        return None;
      }
      let delimiter_str = &self.iterable[..d_end];
      let split = Split {
        string: Cow::Borrowed(delimiter_str),
        typ: SplitType::Delimiter,
        start: self.current_offset,
        end: self.current_offset + delimiter_str.len(),
        was_quoted: false,
      };
      self.current_offset += delimiter_str.len();
      self.iterable = &self.iterable[d_end..];
      Some(split)
    } else {
      None
    }
  }
}

/// An iterator that splits a string with advanced options like quoting and preservation.
#[ allow( clippy::struct_excessive_bools ) ]
#[ derive( Debug ) ]
// This lint is addressed by using SplitFlags
pub struct SplitIterator<'a> {
  iterator: SplitFastIterator<'a, Vec< &'a str >>,
  src: &'a str,
  flags: SplitFlags,
  quoting_prefixes: Vec< &'a str >,
  quoting_postfixes: Vec< &'a str >,
  pending_opening_quote_delimiter: Option<Split<'a>>,
  last_yielded_token_was_delimiter: bool,
  just_finished_peeked_quote_end_offset: Option< usize >,
  skip_next_spurious_empty: bool,
  active_quote_char: Option< char >,
  just_processed_quote: bool,
}

impl<'a> SplitIterator<'a> {
  pub( super ) fn new(o: &impl SplitOptionsAdapter<'a, Vec< &'a str >>) -> Self {
    let mut delimiter_list_for_fast_iterator = o.delimiter();
    delimiter_list_for_fast_iterator.retain(|&pat| !pat.is_empty());
    let iterator = SplitFastIterator::new(&o.clone_options_for_sfi());
    let flags = o.flags();
    Self {
      iterator,
      src: o.src(),
      flags,
      quoting_prefixes: o.quoting_prefixes().clone(),
      quoting_postfixes: o.quoting_postfixes().clone(),
      pending_opening_quote_delimiter: None,
      last_yielded_token_was_delimiter: false,
      just_finished_peeked_quote_end_offset: None,
      skip_next_spurious_empty: false,
      active_quote_char: None, // No active quote at iteration start
      just_processed_quote: false,
    }
  }
}

impl<'a> Iterator for SplitIterator<'a> {
  type Item = Split<'a>;
  #[ allow( clippy::too_many_lines ) ]
  fn next(&mut self) -> Option< Self::Item > {
    loop {
      if let Some(offset) = self.just_finished_peeked_quote_end_offset.take() {
        if self.iterator.current_offset != offset {
          if offset > self.iterator.current_offset {
            // Move forward
            self.iterator.iterable = &self.iterator.iterable[offset - self.iterator.current_offset..];
          } else {
            // Move backward - need to recalculate from source
            let src_len = self.src.len();
            if offset < src_len {
              self.iterator.iterable = &self.src[offset..];
            }
          }
          self.iterator.current_offset = offset;
        }
      }
      if let Some(pending_split) = self.pending_opening_quote_delimiter.take() {
        if pending_split.typ != SplitType::Delimiter || self.flags.contains( SplitFlags::PRESERVING_DELIMITERS )
        {
          self.last_yielded_token_was_delimiter = pending_split.typ == SplitType::Delimiter;
          return Some(pending_split);
        }
      }

      let about_to_process_quote = self.flags.contains(SplitFlags::QUOTING)
        && self.active_quote_char.is_none()
        && self.quoting_prefixes.iter().any(|p| self.iterator.iterable.starts_with(p));
      // Special case: don't generate preserving_empty tokens when the last yielded token was quoted content (empty or not)
      // and we're not about to process a quote. This prevents spurious empty tokens after empty quoted sections.
      let last_was_quoted_content = self.just_processed_quote;
      // For now, focus on the core case: consecutive delimiters only
      // Generate preserving_empty tokens for consecutive delimiters OR before quotes (but not for quoted empty content)
      let has_consecutive_delimiters = self
        .iterator
        .delimiter
        .pos(self.iterator.iterable)
        .is_some_and(|(ds, _)| ds == 0);
      let preserving_empty_check = self.last_yielded_token_was_delimiter
        && self.flags.contains(SplitFlags::PRESERVING_EMPTY)
        && !last_was_quoted_content
        && (has_consecutive_delimiters
          || (about_to_process_quote
            && !self.iterator.iterable.starts_with("\"\"")
            && !self.iterator.iterable.starts_with("''")
            && !self.iterator.iterable.starts_with("``")));

      if preserving_empty_check {
        let current_sfi_offset = self.iterator.current_offset;
        let empty_token = Split {
          string: Cow::Borrowed(""),
          typ: SplitType::Delimited,
          start: current_sfi_offset,
          end: current_sfi_offset,
          was_quoted: false,
        };
        // Prevent duplicate empty tokens after delimiter processing
        self.last_yielded_token_was_delimiter = false;
        // Advance the iterator's counter to skip the empty content that would naturally be returned next
        self.iterator.counter += 1;
        return Some(empty_token);
      }

      self.last_yielded_token_was_delimiter = false;
      let sfi_next_internal_counter_will_be_odd = self.iterator.counter % 2 == 0;
      let sfi_iterable_starts_with_delimiter = self
        .iterator
        .delimiter
        .pos(self.iterator.iterable)
        .is_some_and(|(d_start, _)| d_start == 0);
      let sfi_should_yield_empty_now = self.flags.contains(SplitFlags::PRESERVING_EMPTY)
        && sfi_next_internal_counter_will_be_odd
        && sfi_iterable_starts_with_delimiter;
      let effective_split_opt: Option<Split<'a>>;
      let mut quote_handled_by_peek = false;

      // Simplified quoting logic
      if self.flags.contains(SplitFlags::QUOTING) && self.active_quote_char.is_none() && !sfi_should_yield_empty_now {
        if let Some(first_char_iterable) = self.iterator.iterable.chars().next() {
          if let Some(prefix_idx) = self
            .quoting_prefixes
            .iter()
            .position(|p| self.iterator.iterable.starts_with(p))
          {
            quote_handled_by_peek = true;
            let prefix_str = self.quoting_prefixes[prefix_idx];
            let opening_quote_original_start = self.iterator.current_offset;
            let prefix_len = prefix_str.len();
            let expected_postfix = self.quoting_postfixes[prefix_idx];

            // Consume the opening quote
            self.iterator.current_offset += prefix_len;
            self.iterator.iterable = &self.iterator.iterable[prefix_len..];
            self.active_quote_char = Some(first_char_iterable); // Set active quote char in SplitIterator

            let mut end_of_quote_idx: Option< usize > = None;
            let mut chars = self.iterator.iterable.chars();
            let mut current_char_offset = 0;
            let mut escaped = false;

            // Simple quote parsing: find the closing quote, respecting escape sequences
            while let Some(c) = chars.next() {
              if escaped {
                escaped = false;
                current_char_offset += c.len_utf8();
              } else if c == '\\' {
                escaped = true;
                current_char_offset += c.len_utf8();
              } else if c == first_char_iterable
              // Found unescaped quote
              {
                // Check if this is truly a closing quote or the start of an adjacent quoted section
                let remaining_chars = chars.as_str();
                if !remaining_chars.is_empty() {
                  let next_char = remaining_chars.chars().next().unwrap();
                  // If the next character is alphanumeric (part of content), this might be an adjacent quote
                  if next_char.is_alphanumeric() && current_char_offset > 0 {
                    // Check if the previous character is non-whitespace (meaning no delimiter)
                    let content_so_far = &self.iterator.iterable[..current_char_offset];
                    if let Some(last_char) = content_so_far.chars().last() {
                      if !last_char.is_whitespace() {
                        // This is an adjacent quote - treat it as the end of this section
                        end_of_quote_idx = Some(current_char_offset);
                        break;
                      }
                    }
                  }
                }
                // Normal closing quote
                end_of_quote_idx = Some(current_char_offset);
                break;
              } else {
                current_char_offset += c.len_utf8();
              }
            }

            let (quoted_content_str, consumed_len_in_sfi_iterable) = if let Some(end_idx) = end_of_quote_idx {
              // Content is from start of current iterable to end_idx (before the closing quote)
              let content = &self.iterator.iterable[..end_idx];

              // Check if this is an adjacent quote scenario (no delimiter follows)
              let remaining_chars = &self.iterator.iterable[end_idx..];
              let is_adjacent = if remaining_chars.len() > 1 {
                let chars_after_quote: Vec< char > = remaining_chars.chars().take(2).collect();
                if chars_after_quote.len() >= 2 {
                  chars_after_quote[0] == '"' && chars_after_quote[1].is_alphanumeric()
                } else {
                  false
                }
              } else {
                false
              };

              let consumed = if is_adjacent {
                end_idx // Don't consume the quote - it's the start of the next section
              } else {
                end_idx + expected_postfix.len() // Normal case - consume the closing quote
              };

              (content, consumed)
            } else {
              // No closing quote found, consume the rest of the iterable
              (self.iterator.iterable, self.iterator.iterable.len())
            };

            if quoted_content_str.is_empty() && end_of_quote_idx.is_some() {
              self.last_yielded_token_was_delimiter = false;
            }

            // Advance SFI's internal state based on what was consumed
            self.iterator.current_offset += consumed_len_in_sfi_iterable;
            self.iterator.iterable = &self.iterator.iterable[consumed_len_in_sfi_iterable..];
            self.active_quote_char = None; // Reset active quote char

            if self.flags.contains(SplitFlags::PRESERVING_QUOTING) {
              let full_quoted_len = prefix_len
                + quoted_content_str.len()
                + if end_of_quote_idx.is_some() {
                  expected_postfix.len()
                } else {
                  0
                };
              let new_string = if opening_quote_original_start + full_quoted_len <= self.src.len() {
                Cow::Borrowed(&self.src[opening_quote_original_start..(opening_quote_original_start + full_quoted_len)])
              } else {
                Cow::Borrowed("")
              };
              let new_end = opening_quote_original_start + new_string.len();
              effective_split_opt = Some(Split {
                string: new_string,
                typ: SplitType::Delimited,
                start: opening_quote_original_start,
                end: new_end,
                was_quoted: true,
              });
            } else {
              let unescaped_string: Cow<'a, str> = unescape_str(quoted_content_str).into_owned().into();
              let new_start = opening_quote_original_start + prefix_len;
              let new_end = new_start + unescaped_string.len();
              effective_split_opt = Some(Split {
                string: unescaped_string,
                typ: SplitType::Delimited,
                start: new_start,
                end: new_end,
                was_quoted: true,
              });
            }
            if effective_split_opt.is_some() {
              self.last_yielded_token_was_delimiter = false;
              self.just_processed_quote = true;
            }
          } else {
            effective_split_opt = self.iterator.next();
          }
        } else {
          effective_split_opt = self.iterator.next();
        }
      } else {
        effective_split_opt = self.iterator.next();
      }

      let mut current_split = effective_split_opt?;
      if quote_handled_by_peek {
        self.skip_next_spurious_empty = true;
      }
      if self.skip_next_spurious_empty && current_split.typ == SplitType::Delimited && current_split.string.is_empty() {
        self.skip_next_spurious_empty = false;
        continue;
      }

      if !quote_handled_by_peek
        && self.flags.contains(SplitFlags::QUOTING)
        && current_split.typ == SplitType::Delimiter
        && self.active_quote_char.is_none()
      {
        if let Some(_prefix_idx) = self.quoting_prefixes.iter().position(|p| *p == current_split.string.as_ref()) {
          let opening_quote_delimiter = current_split.clone();
          if self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) {
            self.pending_opening_quote_delimiter = Some(opening_quote_delimiter.clone());
          }
          if let Some(fcoq) = opening_quote_delimiter.string.chars().next() {
            self.active_quote_char = Some(fcoq);
          }
          if !self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) {
            continue;
          }
        }
      }
      if self.flags.contains(SplitFlags::STRIPPING) && current_split.typ == SplitType::Delimited {
        let original_len = current_split.string.len();
        let trimmed_string = current_split.string.trim();
        if trimmed_string.len() < original_len {
          let leading_whitespace_len = trimmed_string.as_ptr() as usize - current_split.string.as_ptr() as usize;
          current_split.start += leading_whitespace_len;
          current_split.string = Cow::Owned(trimmed_string.to_string());
          current_split.end = current_split.start + current_split.string.len();
        }
      }
      let skip = (current_split.typ == SplitType::Delimited
        && current_split.string.is_empty()
        && !self.flags.contains(SplitFlags::PRESERVING_EMPTY))
        || (current_split.typ == SplitType::Delimiter && !self.flags.contains(SplitFlags::PRESERVING_DELIMITERS));
      if current_split.typ == SplitType::Delimiter {
        // Don't set this flag if we just processed a quote, as the quoted content was the last yielded token
        if !self.just_processed_quote {
          self.last_yielded_token_was_delimiter = true;
        }
      }
      if skip {
        continue;
      }
      // Reset the quote flag when returning any token
      self.just_processed_quote = false;
      return Some(current_split);
    }
  }
}
