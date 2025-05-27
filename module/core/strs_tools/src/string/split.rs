// TEMPORARILY making private public for diagnostics
pub mod private // Changed from cfg-gated to simple pub mod
{
  use crate::string::parse_request::OpType;
  use bitflags::bitflags;

  bitflags! {
      #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
      pub struct SplitFlags: u8 {
          const PRESERVING_EMPTY      = 1 << 0;
          const PRESERVING_DELIMITERS = 1 << 1;
          const PRESERVING_QUOTING    = 1 << 2;
          const STRIPPING             = 1 << 3;
          const QUOTING               = 1 << 4;
      }
  }

  #[derive(Debug, Clone)]
  pub struct Split< 'a >
  {
    pub string : &'a str,
    pub typ : SplitType,
    pub start : usize,
    pub end : usize,
  }

  impl From< Split< '_ > > for String
  {
    fn from( src : Split< '_ > ) -> Self
    {
      src.string.into()
    }
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum SplitType
  {
    Delimeted,
    Delimiter,
  }

  pub trait Searcher
  {
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

    pub fn get_test_iterable(&self) -> &'a str { self.iterable }
    pub fn get_test_current_offset(&self) -> usize { self.current_offset }
    pub fn get_test_active_quote_char(&self) -> Option<char> { self.active_quote_char }
    pub fn get_test_counter(&self) -> i32 { self.counter }
  }

  impl< 'a, D > Iterator for SplitFastIterator< 'a, D >
  where
    D : Searcher
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
        let mut prev_char_is_escape = false;
        for ( char_idx, ch ) in self.iterable.char_indices()
        {
          if prev_char_is_escape { prev_char_is_escape = false; continue; }
          if ch == '\\' { prev_char_is_escape = true; continue; }
          if ch == current_quote_char { end_of_quote_idx = Some( char_idx + ch.len_utf8() ); break; }
        }
        let ( segment_str, consumed_len ) = if let Some( end_idx ) = end_of_quote_idx
          { ( &self.iterable[ ..end_idx ], end_idx ) } else { ( self.iterable, self.iterable.len() ) };
        let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
        self.current_offset += consumed_len; self.iterable = &self.iterable[ consumed_len.. ]; return Some( split );
      }
      if self.iterable.is_empty() && self.counter > 0 { return None; }
      self.counter += 1;
      if self.counter % 2 == 1 {
        if let Some( ( d_start, _d_end ) ) = self.delimeter.pos( self.iterable ) {
          if d_start == 0 { return Some( Split { string: "", typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset } ); }
          let segment_str = &self.iterable[ ..d_start ];
          let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += segment_str.len(); self.iterable = &self.iterable[ d_start.. ]; Some( split ) 
        } else {
          if self.iterable.is_empty() { return None; } 
          let segment_str = self.iterable;
          let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += segment_str.len(); self.iterable = ""; Some( split ) 
        }
      } else if let Some( ( d_start, d_end ) ) = self.delimeter.pos( self.iterable ) {
        if d_start > 0 { self.iterable = ""; return None; } 
        let delimiter_str = &self.iterable[ ..d_end ];
        let split = Split { string: delimiter_str, typ: SplitType::Delimiter, start: self.current_offset, end: self.current_offset + delimiter_str.len() };
        self.current_offset += delimiter_str.len(); self.iterable = &self.iterable[ d_end.. ]; Some( split ) 
      } else { None }
    }
  }

  #[derive(Debug)]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct SplitIterator< 'a >
  {
    iterator : SplitFastIterator< 'a, Vec< &'a str > >,
    src : &'a str,
    // stripping : bool,
    // preserving_empty : bool,
    // preserving_delimeters : bool,
    // preserving_quoting : bool,
    // quoting : bool,
    flags : SplitFlags,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
    pending_opening_quote_delimiter : Option< Split< 'a > >,
    last_yielded_token_was_delimiter : bool,
    just_finished_peeked_quote_end_offset : Option< usize >,
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
        // stripping : flags.contains(SplitFlags::STRIPPING), preserving_empty : flags.contains(SplitFlags::PRESERVING_EMPTY),
        // preserving_delimeters : flags.contains(SplitFlags::PRESERVING_DELIMITERS), preserving_quoting : flags.contains(SplitFlags::PRESERVING_QUOTING),
        // quoting : flags.contains(SplitFlags::QUOTING),
        quoting_prefixes : o.quoting_prefixes().clone(),
        quoting_postfixes : o.quoting_postfixes().clone(), pending_opening_quote_delimiter : None,
        last_yielded_token_was_delimiter : false, just_finished_peeked_quote_end_offset : None,
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
        let mut just_finished_quote_offset_cache = None;
        if let Some(offset) = self.just_finished_peeked_quote_end_offset.take() { just_finished_quote_offset_cache = Some(offset); }
        if let Some( pending_split ) = self.pending_opening_quote_delimiter.take() {
          if pending_split.typ != SplitType::Delimiter || self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) {
            if self.flags.contains(SplitFlags::QUOTING) && self.quoting_prefixes.contains(&pending_split.string) {
              if let Some(fcoq) = pending_split.string.chars().next() { self.iterator.active_quote_char = Some(fcoq); }
            }
            self.last_yielded_token_was_delimiter = pending_split.typ == SplitType::Delimiter; return Some( pending_split );
          }
          if self.flags.contains(SplitFlags::QUOTING) && self.quoting_prefixes.contains(&pending_split.string) {
            if let Some(fcoq) = pending_split.string.chars().next() { self.iterator.active_quote_char = Some(fcoq); }
          }
        }
        if self.last_yielded_token_was_delimiter && self.flags.contains(SplitFlags::PRESERVING_EMPTY) && self.flags.contains(SplitFlags::QUOTING) &&
           self.iterator.active_quote_char.is_none() && self.quoting_prefixes.iter().any(|p| self.iterator.iterable.starts_with(p)) &&
           self.iterator.delimeter.pos(self.iterator.iterable).is_none_or(|(ds, _)| ds != 0) {
          let current_sfi_offset = self.iterator.current_offset;
          let empty_token = Split { string: "", typ: SplitType::Delimeted, start: current_sfi_offset, end: current_sfi_offset };
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
              quote_handled_by_peek = true; let prefix_str = self.quoting_prefixes[ prefix_idx ];
              let opening_quote_original_start = self.iterator.current_offset; let prefix_len = prefix_str.len();
              let expected_postfix = self.quoting_postfixes[ prefix_idx ];
              self.iterator.current_offset += prefix_len; self.iterator.iterable = &self.iterator.iterable[ prefix_len.. ];
              self.iterator.active_quote_char = Some( first_char_iterable );
              let quoted_segment_from_sfi_opt = self.iterator.next(); self.iterator.active_quote_char = None;
              if let Some( mut quoted_segment ) = quoted_segment_from_sfi_opt {
                self.just_finished_peeked_quote_end_offset = Some(quoted_segment.end); 
                if quoted_segment.string.ends_with( expected_postfix ) {
                  if self.flags.contains(SplitFlags::PRESERVING_QUOTING) {
                    quoted_segment.start = opening_quote_original_start; 
                    let full_quoted_len = prefix_len + quoted_segment.string.len();
                    if quoted_segment.start + full_quoted_len <= self.src.len() { quoted_segment.string = &self.src[ quoted_segment.start .. ( quoted_segment.start + full_quoted_len ) ]; }
                    else { quoted_segment.string = ""; }
                    quoted_segment.end = quoted_segment.start + quoted_segment.string.len();
                  } else {
                    quoted_segment.start = opening_quote_original_start + prefix_len; 
                    if quoted_segment.string.len() >= expected_postfix.len() {
                      let content_len = quoted_segment.string.len() - expected_postfix.len();
                      quoted_segment.string = &quoted_segment.string[0 .. content_len];
                    } else { quoted_segment.string = ""; }
                    quoted_segment.end = quoted_segment.start + quoted_segment.string.len(); 
                  }
                } else { // Unclosed quote
                  if self.flags.contains(SplitFlags::PRESERVING_QUOTING) {
                    quoted_segment.start = opening_quote_original_start;
                    let full_quoted_len = prefix_len + quoted_segment.string.len();
                    if quoted_segment.start + full_quoted_len <= self.src.len() { quoted_segment.string = &self.src[ quoted_segment.start .. ( quoted_segment.start + full_quoted_len ) ]; }
                    else { quoted_segment.string = ""; }
                    quoted_segment.end = quoted_segment.start + quoted_segment.string.len();
                  }
                }
                quoted_segment.typ = SplitType::Delimeted; effective_split_opt = Some( quoted_segment );
              } else { // SFI returned None
                let mut prefix_as_token = Split { string: prefix_str, typ: SplitType::Delimeted, start: opening_quote_original_start, end: opening_quote_original_start + prefix_len };
                if !self.flags.contains(SplitFlags::PRESERVING_QUOTING) {
                  prefix_as_token.string = ""; prefix_as_token.start = opening_quote_original_start + prefix_len; prefix_as_token.end = prefix_as_token.start; 
                }
                effective_split_opt = Some( prefix_as_token );
                if effective_split_opt.is_some() { self.just_finished_peeked_quote_end_offset = Some(opening_quote_original_start + prefix_len); }
              }
              if effective_split_opt.is_some() { self.last_yielded_token_was_delimiter = false; }
            } else { effective_split_opt = self.iterator.next(); }
          } else { effective_split_opt = self.iterator.next(); }
        } else { effective_split_opt = self.iterator.next(); }
        let mut current_split = effective_split_opt?;
        if let Some(peeked_quote_end) = just_finished_quote_offset_cache {
          if current_split.typ == SplitType::Delimeted && current_split.string.is_empty() && current_split.start == peeked_quote_end && self.flags.contains(SplitFlags::PRESERVING_EMPTY) && peeked_quote_end < self.src.len() {
            let char_after_quote = &self.src[peeked_quote_end..];
            if self.iterator.delimeter.pos(char_after_quote).is_some_and(|(ds, _)| ds == 0) {
              self.last_yielded_token_was_delimiter = false; continue;
            }
          }
        }
        if !quote_handled_by_peek && self.flags.contains(SplitFlags::QUOTING) && current_split.typ == SplitType::Delimiter && self.iterator.active_quote_char.is_none() {
          if let Some(_prefix_idx) = self.quoting_prefixes.iter().position(|p| *p == current_split.string) {
            let opening_quote_delimiter = current_split.clone();
            if self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) { self.pending_opening_quote_delimiter = Some(opening_quote_delimiter.clone()); }
            if let Some(fcoq) = opening_quote_delimiter.string.chars().next() { self.iterator.active_quote_char = Some(fcoq); }
            if !self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) { continue; }
          }
        }
        if self.flags.contains(SplitFlags::STRIPPING) && current_split.typ == SplitType::Delimeted {
          let original_string_ptr = current_split.string.as_ptr(); let original_len = current_split.string.len();
          let trimmed_string = current_split.string.trim();
          if trimmed_string.len() < original_len || (trimmed_string.is_empty() && original_len > 0) {
            let leading_whitespace_len = trimmed_string.as_ptr() as usize - original_string_ptr as usize;
            current_split.start += leading_whitespace_len; current_split.string = trimmed_string;
            current_split.end = current_split.start + current_split.string.len();
          }
        }
        let mut skip = false;
        if current_split.typ == SplitType::Delimeted && current_split.string.is_empty() && !self.flags.contains(SplitFlags::PRESERVING_EMPTY) { skip = true; }
        if current_split.typ == SplitType::Delimiter && !self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) { skip = true; }
        if !skip {
          if current_split.typ == SplitType::Delimiter { self.last_yielded_token_was_delimiter = true; }
          return Some( current_split );
        }
      } 
    } 
  } 

  #[derive(Debug, Clone)] 
  pub struct SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone,
  {
    src : &'a str,
    delimeter : D,
    flags : SplitFlags,
    // preserving_empty : bool,
    // preserving_delimeters : bool,
    // preserving_quoting : bool,
    // stripping : bool,
    // quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  impl< 'a > SplitOptions< 'a, Vec< &'a str > >
  {
    #[ must_use ]
    pub fn split( self ) -> SplitIterator< 'a > { SplitIterator::new( &self ) }
  }

  impl< 'a, D > SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone
  {
    // This is inside pub mod private, so pub fn makes it pub
    pub fn split_fast( self ) -> SplitFastIterator< 'a, D > { SplitFastIterator::new( &self ) }
  }

  pub trait SplitOptionsAdapter< 'a, D > where D : Searcher + Default + Clone 
  {
    fn src( &self ) -> &'a str;
    fn delimeter( &self ) -> D;
    // fn preserving_empty( &self ) -> bool;
    // fn preserving_delimeters( &self ) -> bool;
    // fn preserving_quoting( &self ) -> bool;
    // fn stripping( &self ) -> bool;
    // fn quoting( &self ) -> bool;
    fn flags( &self ) -> SplitFlags;
    fn quoting_prefixes( &self ) -> &Vec< &'a str >;
    fn quoting_postfixes( &self ) -> &Vec< &'a str >;
    fn clone_options_for_sfi( &self ) -> SplitOptions< 'a, D >;
  }

  impl< 'a, D : Searcher + Clone + Default > SplitOptionsAdapter< 'a, D > for SplitOptions< 'a, D >
  {
    fn src( &self ) -> &'a str { self.src }
    fn delimeter( &self ) -> D { self.delimeter.clone() }
    // fn preserving_empty( &self ) -> bool { self.flags.contains(SplitFlags::PRESERVING_EMPTY) }
    // fn preserving_delimeters( &self ) -> bool { self.flags.contains(SplitFlags::PRESERVING_DELIMITERS) }
    // fn preserving_quoting( &self ) -> bool { self.flags.contains(SplitFlags::PRESERVING_QUOTING) }
    // fn stripping( &self ) -> bool { self.flags.contains(SplitFlags::STRIPPING) }
    // fn quoting( &self ) -> bool { self.flags.contains(SplitFlags::QUOTING) }
    fn flags( &self ) -> SplitFlags { self.flags }
    fn quoting_prefixes( &self ) -> &Vec< &'a str > { &self.quoting_prefixes }
    fn quoting_postfixes( &self ) -> &Vec< &'a str > { &self.quoting_postfixes }
    fn clone_options_for_sfi( &self ) -> SplitOptions< 'a, D > { self.clone() }
  }

  #[ allow( clippy::struct_excessive_bools ) ] #[ derive( Debug ) ]
  pub struct SplitOptionsFormer< 'a >
  {
    src : &'a str,
    delimeter : OpType< &'a str >,
    flags : SplitFlags,
    // preserving_empty : bool,
    // preserving_delimeters : bool,
    // preserving_quoting : bool,
    // stripping : bool,
    // quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  impl< 'a > SplitOptionsFormer< 'a >
  {
    pub fn new< D : Into< OpType< &'a str > > >( delimeter : D ) -> SplitOptionsFormer< 'a >
    {
      Self
      {
        src : "", delimeter : OpType::Vector( vec![] ).append( delimeter.into() ),
        flags : SplitFlags::PRESERVING_DELIMITERS, // Default
        // preserving_empty : false,
        // preserving_delimeters : true,
        // preserving_quoting : false,
        // stripping : false, quoting : false,
        quoting_prefixes : vec![], quoting_postfixes : vec![],
      }
    }
    pub fn preserving_empty( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::PRESERVING_EMPTY); } else { self.flags.remove(SplitFlags::PRESERVING_EMPTY); } self }
    pub fn preserving_delimeters( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::PRESERVING_DELIMITERS); } else { self.flags.remove(SplitFlags::PRESERVING_DELIMITERS); } self }
    pub fn preserving_quoting( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::PRESERVING_QUOTING); } else { self.flags.remove(SplitFlags::PRESERVING_QUOTING); } self }
    pub fn stripping( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::STRIPPING); } else { self.flags.remove(SplitFlags::STRIPPING); } self }
    pub fn quoting( &mut self, value : bool ) -> &mut Self { if value { self.flags.insert(SplitFlags::QUOTING); } else { self.flags.remove(SplitFlags::QUOTING); } self }
    pub fn quoting_prefixes( &mut self, value : Vec< &'a str > ) -> &mut Self { self.quoting_prefixes = value; self }
    pub fn quoting_postfixes( &mut self, value : Vec< &'a str > ) -> &mut Self { self.quoting_postfixes = value; self }
    pub fn src( &mut self, value : &'a str ) -> &mut Self { self.src = value; self }
    pub fn delimeter< D : Into< OpType< &'a str > > >( &mut self, value : D ) -> &mut Self
    { self.delimeter = OpType::Vector( vec![] ).append( value.into() ); self }
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
        // preserving_empty : self.preserving_empty,
        // preserving_delimeters : self.preserving_delimeters,
        // preserving_quoting : self.preserving_quoting,
        // stripping : self.stripping,
        // quoting : self.quoting,
        quoting_prefixes : self.quoting_prefixes.clone(),
        quoting_postfixes : self.quoting_postfixes.clone(),
      }
    }
    pub fn perform( &mut self ) -> SplitIterator< 'a > { self.form().split() }
  }
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