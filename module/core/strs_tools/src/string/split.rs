//! Provides tools for splitting strings with advanced options including quoting.

use bitflags::bitflags;

bitflags! {
    /// Flags to control the behavior of the split iterators.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct SplitFlags: u8 {
        /// Preserves empty segments.
        const PRESERVING_EMPTY      = 1 << 0;
        /// Preserves delimiter segments.
        const PRESERVING_DELIMITERS = 1 << 1;
        /// Preserves quoting characters in the output.
        const PRESERVING_QUOTING    = 1 << 2;
        /// Strips leading/trailing whitespace from delimited segments.
        const STRIPPING             = 1 << 3;
        /// Enables handling of quoted sections.
        const QUOTING               = 1 << 4;
    }
}

/// Internal implementation details for string splitting.
mod private
{
  use std::borrow::Cow;
  use crate::string::parse_request::OpType;
  use super::SplitFlags; // Import SplitFlags from parent module

  /// Represents a segment of a string after splitting.
  #[derive(Debug, Clone)]
  pub struct Split< 'a >
  {
    /// The string content of the segment.
    pub string : Cow< 'a, str >,
    /// The type of the segment (delimited or delimiter).
    pub typ : SplitType,
    /// The starting byte index of the segment in the original string.
    pub start : usize,
    /// The ending byte index of the segment in the original string.
    pub end : usize,
  }

  impl<'a> From< Split<'a> > for String
  {
    fn from( src : Split<'a> ) -> Self
    {
      src.string.into_owned()
    }
  }

  /// Defines the type of a split segment.
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum SplitType
  {
    /// A segment of delimited content.
    Delimeted,
    /// A segment representing a delimiter.
    Delimiter,
  }

  /// Trait for finding the position of a delimiter pattern in a string.
  pub trait Searcher
  {
    /// Finds the first occurrence of the delimiter pattern in `src`.
    /// Returns `Some((start_index, end_index))` if found, `None` otherwise.
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >;
  }

  impl Searcher for &str
  {
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >
    {
      if self.is_empty() { return None; }
      src.find( self ).map( | start | ( start, start + self.len() ) )
    }
  }

  impl Searcher for String
  {
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >
    {
      if self.is_empty() { return None; }
      src.find( self ).map( | start | ( start, start + self.len() ) )
    }
  }

  impl Searcher for Vec<&str>
  {
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >
    {
      let mut r = vec![];
      for pat in self
      {
        if pat.is_empty() { continue; }
        if let Some( x ) =  src.find( pat )
        {
          r.push( ( x, x + pat.len() ) );
        }
      }
      if r.is_empty() { return None; }
      r.sort_by( |a, b| a.0.cmp( &b.0 ).then_with( || (a.1 - a.0).cmp( &(b.1 - b.0) ) ) );
      r.first().copied()
    }
  }

  /// An iterator that quickly splits a string based on a delimiter, without advanced options.
  #[derive(Debug)]
  pub struct SplitFastIterator< 'a, D >
  where
    D : Searcher
  {
    iterable : &'a str,
    current_offset : usize,
    counter : i32,
    delimeter : D,
    active_quote_char : Option< char >,
  }

  impl< 'a, D : Searcher + Default + Clone > SplitFastIterator< 'a, D >
  {
    fn new( o : &impl SplitOptionsAdapter< 'a, D > ) -> Self
    {
      Self
      {
        iterable : o.src(),
        current_offset : 0,
        delimeter : o.delimeter(),
        counter : 0,
        active_quote_char : None,
      }
    }

    /// Sets the internal state of the iterator, for testing purposes.
    // Test helper methods are pub
    pub fn set_test_state(
        &mut self,
        iterable: &'a str,
        current_offset: usize,
        active_quote_char: Option<char>,
        counter: i32,
    ) {
        self.iterable = iterable;
        self.current_offset = current_offset;
        self.active_quote_char = active_quote_char;
        self.counter = counter;
    }

    /// Gets the current iterable string, for testing purposes.
    pub fn get_test_iterable(&self) -> &'a str { self.iterable }
    /// Gets the current offset within the original string, for testing purposes.
    pub fn get_test_current_offset(&self) -> usize { self.current_offset }
    /// Gets the currently active quote character, if any, for testing purposes.
    pub fn get_test_active_quote_char(&self) -> Option<char> { self.active_quote_char }
    /// Gets the internal counter value, for testing purposes.
    pub fn get_test_counter(&self) -> i32 { self.counter }
  }

  impl< 'a, D : Searcher > Iterator for SplitFastIterator< 'a, D >
  {
    type Item = Split< 'a >;
    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.iterable.is_empty() && ( self.counter > 0 || self.active_quote_char.is_some() )
      {
        return None;
      }
      if let Some( current_quote_char ) = self.active_quote_char
      {
        let mut end_of_quote_idx : Option< usize > = None;
        let mut temp_iterable = self.iterable;
        let mut search_offset = 0;
        loop
        {
          if let Some( pos ) = temp_iterable.find( current_quote_char )
          {
            let mut backslashes = 0;
            for c in temp_iterable[ ..pos ].chars().rev()
            {
              if c == '\\' { backslashes += 1; } else { break; }
            }

            if backslashes % 2 == 1
            {
              let new_start = pos + 1;
              temp_iterable = &temp_iterable[ new_start.. ];
              search_offset += new_start;
              continue;
            }
            else
            {
              let end_idx = search_offset + pos + current_quote_char.len_utf8();
              end_of_quote_idx = Some( end_idx );
              break;
            }
          }
          else
          {
            break;
          }
        }
        let ( segment_str, consumed_len ) = if let Some( end_idx ) = end_of_quote_idx
          { ( &self.iterable[ ..end_idx ], end_idx ) } else { ( self.iterable, self.iterable.len() ) };
        let split = Split { string: Cow::Borrowed( segment_str ), typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
        self.current_offset += consumed_len; self.iterable = &self.iterable[ consumed_len.. ]; return Some( split );
      }
      if self.iterable.is_empty() && self.counter > 0 { return None; }
      self.counter += 1;
      if self.counter % 2 == 1 {
        if let Some( ( d_start, _d_end ) ) = self.delimeter.pos( self.iterable ) {
          if d_start == 0 { return Some( Split { string: Cow::Borrowed(""), typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset } ); }
          let segment_str = &self.iterable[ ..d_start ];
          let split = Split { string: Cow::Borrowed( segment_str ), typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += segment_str.len(); self.iterable = &self.iterable[ d_start.. ]; Some( split )
        } else {
          if self.iterable.is_empty() { return None; }
          let segment_str = self.iterable;
          let split = Split { string: Cow::Borrowed( segment_str ), typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += segment_str.len(); self.iterable = ""; Some( split )
        }
      } else if let Some( ( d_start, d_end ) ) = self.delimeter.pos( self.iterable ) {
        if d_start > 0 { self.iterable = ""; return None; }
        let delimiter_str = &self.iterable[ ..d_end ];
        let split = Split { string: Cow::Borrowed( delimiter_str ), typ: SplitType::Delimiter, start: self.current_offset, end: self.current_offset + delimiter_str.len() };
        self.current_offset += delimiter_str.len(); self.iterable = &self.iterable[ d_end.. ]; Some( split )
      } else { None }
    }
  }

  /// Helper function to unescape common escape sequences in a string.
  /// Returns a `Cow::Borrowed` if no unescaping is needed, otherwise `Cow::Owned`.
  fn unescape_str( input : &str ) -> Cow< '_, str >
  {
    if !input.contains( '\\' )
    {
      return Cow::Borrowed( input );
    }

    let mut output = String::with_capacity( input.len() );
    let mut chars = input.chars();

    while let Some( ch ) = chars.next()
    {
      if ch == '\\'
      {
        if let Some( next_ch ) = chars.next()
        {
          match next_ch
          {
            '"' => output.push( '"' ),
            '\\' => output.push( '\\' ),
            'n' => output.push( '\n' ),
            't' => output.push( '\t' ),
            'r' => output.push( '\r' ),
            _ =>
            {
              output.push( '\\' );
              output.push( next_ch );
            }
          }
        }
        else
        {
          output.push( '\\' );
        }
      }
      else
      {
        output.push( ch );
      }
    }

    Cow::Owned( output )
  }

  #[cfg(test)]
  mod unescape_tests
  {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn no_escapes()
    {
      let input = "hello world";
      let result = unescape_str( input );
      assert!( matches!( result, Cow::Borrowed( _ ) ) );
      assert_eq!( result, "hello world" );
    }

    #[test]
    fn valid_escapes()
    {
      let input = r#"hello \"world\\, \n\t\r end"#;
      let expected = "hello \"world\\, \n\t\r end";
      let result = unescape_str( input );
      assert!( matches!( result, Cow::Owned( _ ) ) );
      assert_eq!( result, expected );
    }

    #[test]
    fn mixed_escapes()
    {
      let input = r#"a\"b\\c\nd"#;
      let expected = "a\"b\\c\nd";
      let result = unescape_str( input );
      assert!( matches!( result, Cow::Owned( _ ) ) );
      assert_eq!( result, expected );
    }

    #[test]
    fn unrecognized_escape()
    {
      let input = r"hello \z world";
      let result = unescape_str( input );
      assert!( matches!( result, Cow::Owned( _ ) ) );
      assert_eq!( result, r"hello \z world" );
    }

    #[test]
    fn empty_string()
    {
      let input = "";
      let result = unescape_str( input );
      assert!( matches!( result, Cow::Borrowed( _ ) ) );
      assert_eq!( result, "" );
    }

    #[test]
    fn trailing_backslash()
    {
      let input = r"hello\";
      let result = unescape_str( input );
      assert!( matches!( result, Cow::Owned( _ ) ) );
      assert_eq!( result, r"hello\" );
    }
  }

  /// An iterator that splits a string with advanced options like quoting and preservation.
  #[derive(Debug)]
  #[ allow( clippy::struct_excessive_bools ) ] // This lint is addressed by using SplitFlags
  pub struct SplitIterator< 'a >
  {
    iterator : SplitFastIterator< 'a, Vec< &'a str > >,
    src : &'a str,
    flags : SplitFlags,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
    pending_opening_quote_delimiter : Option< Split< 'a > >,
    last_yielded_token_was_delimiter : bool,
    just_finished_peeked_quote_end_offset : Option< usize >,
    skip_next_spurious_empty : bool,
  }

  impl< 'a > SplitIterator< 'a >
  {
    fn new( o : &impl SplitOptionsAdapter< 'a, Vec< &'a str > > ) -> Self
    {
      let mut delimeter_list_for_fast_iterator = o.delimeter();
      delimeter_list_for_fast_iterator.retain(|&pat| !pat.is_empty());
      let iterator = SplitFastIterator::new( &o.clone_options_for_sfi() );
      let flags = o.flags();
      Self {
        iterator, src : o.src(), flags,
        quoting_prefixes : o.quoting_prefixes().clone(),
        quoting_postfixes : o.quoting_postfixes().clone(), pending_opening_quote_delimiter : None,
        last_yielded_token_was_delimiter : false, just_finished_peeked_quote_end_offset : None,
        skip_next_spurious_empty : false,
      }
    }
  }

  impl< 'a > Iterator for SplitIterator< 'a >
  {
    type Item = Split< 'a >;
    #[allow(clippy::too_many_lines)]
    fn next( &mut self ) -> Option< Self::Item >
    {
      loop {
        if let Some(offset) = self.just_finished_peeked_quote_end_offset.take() {
            if self.iterator.current_offset < offset {
                self.iterator.iterable = &self.iterator.iterable[offset - self.iterator.current_offset..];
                self.iterator.current_offset = offset;
            }
        }
        if let Some( pending_split ) = self.pending_opening_quote_delimiter.take() {
          if pending_split.typ != SplitType::Delimiter || self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) {
            if self.flags.contains(SplitFlags::QUOTING) && self.quoting_prefixes.contains(&pending_split.string.as_ref()) {
              if let Some(fcoq) = pending_split.string.chars().next() { self.iterator.active_quote_char = Some(fcoq); }
            }
            self.last_yielded_token_was_delimiter = pending_split.typ == SplitType::Delimiter; return Some( pending_split );
          }
          if self.flags.contains(SplitFlags::QUOTING) && self.quoting_prefixes.contains(&pending_split.string.as_ref()) {
            if let Some(fcoq) = pending_split.string.chars().next() { self.iterator.active_quote_char = Some(fcoq); }
          }
        }
        if self.last_yielded_token_was_delimiter && self.flags.contains(SplitFlags::PRESERVING_EMPTY) && self.flags.contains(SplitFlags::QUOTING) &&
           self.iterator.active_quote_char.is_none() && self.quoting_prefixes.iter().any(|p| self.iterator.iterable.starts_with(p)) &&
           self.iterator.delimeter.pos(self.iterator.iterable).is_none_or(|(ds, _)| ds != 0) {
          let current_sfi_offset = self.iterator.current_offset;
          let empty_token = Split { string: Cow::Borrowed(""), typ: SplitType::Delimeted, start: current_sfi_offset, end: current_sfi_offset };
          self.last_yielded_token_was_delimiter = false; return Some(empty_token);
        }
        self.last_yielded_token_was_delimiter = false;
        let sfi_next_internal_counter_will_be_odd = self.iterator.counter % 2 == 0;
        let sfi_iterable_starts_with_delimiter = self.iterator.delimeter.pos( self.iterator.iterable ).is_some_and( |(d_start, _)| d_start == 0 );
        let sfi_should_yield_empty_now = self.flags.contains(SplitFlags::PRESERVING_EMPTY) && sfi_next_internal_counter_will_be_odd && sfi_iterable_starts_with_delimiter;
        let effective_split_opt : Option<Split<'a>>; let mut quote_handled_by_peek = false;
        if self.flags.contains(SplitFlags::QUOTING) && self.iterator.active_quote_char.is_none() && !sfi_should_yield_empty_now {
          if let Some( first_char_iterable ) = self.iterator.iterable.chars().next() {
            if let Some( prefix_idx ) = self.quoting_prefixes.iter().position( |p| self.iterator.iterable.starts_with( p ) ) {
              quote_handled_by_peek = true;
              let prefix_str = self.quoting_prefixes[ prefix_idx ];
              let opening_quote_original_start = self.iterator.current_offset; let prefix_len = prefix_str.len();
              let expected_postfix = self.quoting_postfixes[ prefix_idx ];
              self.iterator.current_offset += prefix_len; self.iterator.iterable = &self.iterator.iterable[ prefix_len.. ];
              self.iterator.active_quote_char = Some( first_char_iterable );
              let quoted_segment_from_sfi_opt = self.iterator.next(); self.iterator.active_quote_char = None;
              if let Some( mut quoted_segment ) = quoted_segment_from_sfi_opt {
                self.just_finished_peeked_quote_end_offset = Some(quoted_segment.end);
                if quoted_segment.string.ends_with( expected_postfix ) {
                  if self.flags.contains(SplitFlags::PRESERVING_QUOTING) {
                    let new_start = opening_quote_original_start;
                    let full_quoted_len = prefix_len + quoted_segment.string.len();
                    let new_string = if new_start + full_quoted_len <= self.src.len() { Cow::Borrowed(&self.src[ new_start .. ( new_start + full_quoted_len ) ]) }
                    else { Cow::Borrowed("") };
                    let new_end = new_start + new_string.len();
                    effective_split_opt = Some(Split { string: new_string, typ: SplitType::Delimeted, start: new_start, end: new_end });
                  } else {
                    let new_start = opening_quote_original_start + prefix_len;
                    let content_len = quoted_segment.string.len() - expected_postfix.len();
                    let sliced_str : &str = &quoted_segment.string.as_ref()[0 .. content_len];
                    let unescaped_string : Cow<'a, str> = unescape_str( sliced_str ).into_owned().into();
                    let new_end = new_start + unescaped_string.len();
                    effective_split_opt = Some(Split
                    {
                      string: unescaped_string,
                      typ: SplitType::Delimeted,
                      start: new_start,
                      end: new_end,
                    });
                  }
                } else { // Unclosed quote
                  if self.flags.contains(SplitFlags::PRESERVING_QUOTING) {
                    let new_start = opening_quote_original_start;
                    let full_quoted_len = prefix_len + quoted_segment.string.len();
                    let new_string = if new_start + full_quoted_len <= self.src.len() { Cow::Borrowed(&self.src[ new_start .. ( new_start + full_quoted_len ) ]) }
                    else { Cow::Borrowed("") };
                    let new_end = new_start + new_string.len();
                    effective_split_opt = Some(Split { string: new_string, typ: SplitType::Delimeted, start: new_start, end: new_end });
                  } else {
                    quoted_segment.string = unescape_str( &quoted_segment.string ).into_owned().into();
                    effective_split_opt = Some(quoted_segment);
                  }
                }
              } else { // SFI returned None
                let mut prefix_as_token = Split { string: Cow::Borrowed(prefix_str), typ: SplitType::Delimeted, start: opening_quote_original_start, end: opening_quote_original_start + prefix_len };
                if !self.flags.contains(SplitFlags::PRESERVING_QUOTING) {
                  prefix_as_token.string = Cow::Borrowed(""); prefix_as_token.start = opening_quote_original_start + prefix_len; prefix_as_token.end = prefix_as_token.start;
                }
                effective_split_opt = Some( prefix_as_token );
                if effective_split_opt.is_some() { self.just_finished_peeked_quote_end_offset = Some(opening_quote_original_start + prefix_len); }
              }
              if effective_split_opt.is_some() { self.last_yielded_token_was_delimiter = false; }
            } else { effective_split_opt = self.iterator.next(); }
          } else { effective_split_opt = self.iterator.next(); }
        } else { effective_split_opt = self.iterator.next(); }
        let mut current_split = effective_split_opt?;

        if quote_handled_by_peek
        {
          self.skip_next_spurious_empty = true;
        }

        if self.skip_next_spurious_empty && current_split.typ == SplitType::Delimeted && current_split.string.is_empty()
        {
          self.skip_next_spurious_empty = false;
          continue;
        }

        let skip = ( current_split.typ == SplitType::Delimeted && current_split.string.is_empty() && !self.flags.contains( SplitFlags::PRESERVING_EMPTY ) )
        || ( current_split.typ == SplitType::Delimiter && !self.flags.contains( SplitFlags::PRESERVING_DELIMITERS ) );

        if skip
        {
          continue;
        }

        if !quote_handled_by_peek && self.flags.contains(SplitFlags::QUOTING) && current_split.typ == SplitType::Delimiter && self.iterator.active_quote_char.is_none() {
          if let Some(_prefix_idx) = self.quoting_prefixes.iter().position(|p| *p == current_split.string.as_ref()) {
            let opening_quote_delimiter = current_split.clone();
            if self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) { self.pending_opening_quote_delimiter = Some(opening_quote_delimiter.clone()); }
            if let Some(fcoq) = opening_quote_delimiter.string.chars().next() { self.iterator.active_quote_char = Some(fcoq); }
            if !self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) { continue; }
          }
        }
        if self.flags.contains(SplitFlags::STRIPPING) && current_split.typ == SplitType::Delimeted {
          let original_len = current_split.string.len();
          let trimmed_string = current_split.string.trim();
          if trimmed_string.len() < original_len {
            let leading_whitespace_len = trimmed_string.as_ptr() as usize - current_split.string.as_ptr() as usize;
            current_split.start += leading_whitespace_len;
            current_split.string = Cow::Owned(trimmed_string.to_string());
            current_split.end = current_split.start + current_split.string.len();
          }
        }
        if current_split.typ == SplitType::Delimiter { self.last_yielded_token_was_delimiter = true; }
        return Some( current_split );
      }
    }
  }

  /// Options to configure the behavior of split iterators.
  #[derive(Debug, Clone)]
  pub struct SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone,
  {
    src : &'a str,
    delimeter : D,
    flags : SplitFlags,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  impl< 'a > SplitOptions< 'a, Vec< &'a str > >
  {
    /// Consumes the options and returns a `SplitIterator`.
    #[ must_use ]
    pub fn split( self ) -> SplitIterator< 'a > { SplitIterator::new( &self ) }
  }

  impl< 'a, D : Searcher + Default + Clone > SplitOptions< 'a, D >
  {
    /// Consumes the options and returns a `SplitFastIterator`.
    // This is inside pub mod private, so pub fn makes it pub
    pub fn split_fast( self ) -> SplitFastIterator< 'a, D > { SplitFastIterator::new( &self ) }
  }
  impl< 'a > core::iter::IntoIterator for SplitOptions< 'a, Vec< &'a str > >
  {
    type Item = Split< 'a >;
    type IntoIter = SplitIterator< 'a >;

    fn into_iter( self ) -> Self::IntoIter
    {
      SplitIterator::new( &self )
    }
  }

  /// Adapter trait to provide split options to iterators.
  pub trait SplitOptionsAdapter< 'a, D > where D : Searcher + Default + Clone
  {
    /// Gets the source string to be split.
    fn src( &self ) -> &'a str;
    /// Gets the delimiter(s) to use for splitting.
    fn delimeter( &self ) -> D;
    /// Gets the behavior flags for splitting.
    fn flags( &self ) -> SplitFlags;
    /// Gets the prefixes that denote the start of a quoted section.
    fn quoting_prefixes( &self ) -> &Vec< &'a str >;
    /// Gets the postfixes that denote the end of a quoted section.
    fn quoting_postfixes( &self ) -> &Vec< &'a str >;
    /// Clones the options, specifically for initializing a `SplitFastIterator`.
    fn clone_options_for_sfi( &self ) -> SplitOptions< 'a, D >;
  }

  impl< 'a, D : Searcher + Clone + Default > SplitOptionsAdapter< 'a, D > for SplitOptions< 'a, D >
  {
    fn src( &self ) -> &'a str { self.src }
    fn delimeter( &self ) -> D { self.delimeter.clone() }
    fn flags( &self ) -> SplitFlags { self.flags }
    fn quoting_prefixes( &self ) -> &Vec< &'a str > { &self.quoting_prefixes }
    fn quoting_postfixes( &self ) -> &Vec< &'a str > { &self.quoting_postfixes }
    fn clone_options_for_sfi( &self ) -> SplitOptions< 'a, D > { self.clone() }
  }

  /// Former (builder) for creating `SplitOptions`.
  #[ allow( clippy::struct_excessive_bools ) ] // This lint is addressed by using SplitFlags
  #[ derive( Debug ) ]
  pub struct SplitOptionsFormer< 'a >
  {
    src : &'a str,
    delimeter : OpType< &'a str >,
    flags : SplitFlags,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  impl< 'a > SplitOptionsFormer< 'a >
  {
    /// Creates a new `SplitOptionsFormer` with the given delimiter(s).
    pub fn new< D : Into< OpType< &'a str > > >( delimeter : D ) -> SplitOptionsFormer< 'a >
    {
      Self
      {
        src : "", delimeter : OpType::Vector( vec![] ).append( delimeter.into() ),
        flags : SplitFlags::PRESERVING_DELIMITERS, // Default
        quoting_prefixes : vec![], quoting_postfixes : vec![],
      }
    }
    /// Sets whether to preserve empty segments.
    pub fn preserving_empty( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::PRESERVING_EMPTY); } else { self.flags.remove(SplitFlags::PRESERVING_EMPTY); } self }
    /// Sets whether to preserve delimiter segments.
    pub fn preserving_delimeters( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::PRESERVING_DELIMITERS); } else { self.flags.remove(SplitFlags::PRESERVING_DELIMITERS); } self }
    /// Sets whether to preserve quoting characters in the output.
    pub fn preserving_quoting( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::PRESERVING_QUOTING); } else { self.flags.remove(SplitFlags::PRESERVING_QUOTING); } self }
    /// Sets whether to strip leading/trailing whitespace from delimited segments.
    pub fn stripping( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::STRIPPING); } else { self.flags.remove(SplitFlags::STRIPPING); } self }
    /// Sets whether to enable handling of quoted sections.
    pub fn quoting( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::QUOTING); } else { self.flags.remove(SplitFlags::QUOTING); } self }
    /// Sets the prefixes that denote the start of a quoted section.
    pub fn quoting_prefixes( &mut self, value : Vec< &'a str > ) -> &mut Self { self.quoting_prefixes = value; self }
    /// Sets the postfixes that denote the end of a quoted section.
    pub fn quoting_postfixes( &mut self, value : Vec< &'a str > ) -> &mut Self { self.quoting_postfixes = value; self }
    /// Sets the source string to be split.
    pub fn src( &mut self, value : &'a str ) -> &mut Self { self.src = value; self }
    /// Sets the delimiter(s) to use for splitting.
    pub fn delimeter< D : Into< OpType< &'a str > > >( &mut self, value : D ) -> &mut Self
    { self.delimeter = OpType::Vector( vec![] ).append( value.into() ); self }
    /// Consumes the former and returns configured `SplitOptions`.
    ///
    /// # Panics
    /// Panics if `delimeter` field contains an `OpType::Primitive(None)` which results from `<&str>::default()`,
    /// and `vector()` method on `OpType` is not robust enough to handle it (currently it would unwrap a None).
    pub fn form( &mut self ) -> SplitOptions< 'a, Vec< &'a str > >
    {
      if self.flags.contains(SplitFlags::QUOTING)
      {
        if self.quoting_prefixes.is_empty() { self.quoting_prefixes = vec![ "\"", "`", "'" ]; }
        if self.quoting_postfixes.is_empty() { self.quoting_postfixes = vec![ "\"", "`", "'" ]; }
      }
      SplitOptions
      {
        src : self.src,
        delimeter : self.delimeter.clone().vector().unwrap(),
        flags : self.flags,
        quoting_prefixes : self.quoting_prefixes.clone(),
        quoting_postfixes : self.quoting_postfixes.clone(),
      }
    }
    /// Consumes the former, builds `SplitOptions`, and returns a `SplitIterator`.
    pub fn perform( &mut self ) -> SplitIterator< 'a > { self.form().split() }
  }
  /// Creates a new `SplitOptionsFormer` to build `SplitOptions` for splitting a string.
  /// This is the main entry point for using the string splitting functionality.
  #[ must_use ] pub fn split< 'a >() -> SplitOptionsFormer< 'a > { SplitOptionsFormer::new( <&str>::default() ) }
}
// NOTE: The #[cfg(not(test))] mod private block was removed as part of the simplification.
// All definitions are now in the single `pub mod private` block above,
// with test-specific items/visibilities handled by #[cfg(test)] attributes.

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( unused_imports ) ] use super::*;
  pub use orphan::*;
  pub use private::
  {
    Split,
    SplitType,
    SplitIterator,
    split,
    SplitOptionsFormer,
    Searcher,
  };
  #[cfg(test)] // Conditionally export SplitFastIterator for tests
  pub use private::SplitFastIterator;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( unused_imports ) ] use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( unused_imports ) ] use super::*;
  pub use prelude::*;
  pub use super::own::split; // Expose the function `split` from `own`

  // Re-export other necessary items from `own` or `private` as needed for the public API
  pub use super::own::
  {
    Split,
    SplitType,
    SplitIterator,
    SplitOptionsFormer,
    Searcher,
  };
  #[cfg(test)]
  pub use super::own::SplitFastIterator;
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( unused_imports ) ] use super::*;
  pub use private:: // Items from private are now directly accessible if private is pub
  {
    SplitOptionsFormer,
    split,
    Searcher,
  };
  #[cfg(test)]
  pub use private::SplitFastIterator;
}