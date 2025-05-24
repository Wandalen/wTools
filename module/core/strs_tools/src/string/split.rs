/// Private namespace.
mod private
{
  use crate::string::parse_request::OpType;

  ///
  /// Either delimeter or delimeted with the slice on its string.
  ///
  #[ allow( dead_code ) ]
  #[derive(Debug, Clone)]
  pub struct Split< 'a >
  {
    /// The string slice representing the split segment or delimiter.
    pub string : &'a str,
    /// The type of split: either Delimeted (content between delimiters) or Delimeter (the delimiter itself).
    pub typ : SplitType,
    /// The starting byte index of the split segment or delimiter in the original source string.
    pub start : usize,
    /// The ending byte index (exclusive) of the split segment or delimiter in the original source string.
    pub end : usize,
  }

  impl From< Split< '_ > > for String
  {
    fn from( src : Split< '_ > ) -> Self
    {
      src.string.into()
    }
  }

  /// Defines the type of a split segment, either a delimited part or the delimiter itself.
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum SplitType
  {
    /// Substring of the original string with text inbetween delimeters.
    Delimeted,
    /// Delimiter,
    Delimiter,
  }

  /// Trait for finding the position of a delimiter pattern within a string.
  pub trait Searcher
  {
    /// Finds the first occurrence of the pattern in `src`. Returns a tuple of (start, end) byte indices if found.
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

  /// A fast, low-level iterator for splitting strings based on a delimiter. Alternates between delimited segments and delimiters.
  #[ derive( Debug ) ]
  pub struct SplitFastIterator< 'a, D >
  where
    D : Searcher
  {
    iterable : &'a str,
    current_offset : usize,
    counter : i32,
    delimeter : D,
  }

  impl< 'a, D : Searcher + Clone > SplitFastIterator< 'a, D >
  {
    /// Creates a new `SplitFastIterator` with the given options.
    #[ allow( dead_code, clippy::needless_pass_by_value ) ]
    fn new( o : impl SplitOptionsAdapter< 'a, D > ) -> Self
    {
      Self
      {
        iterable : o.src(),
        current_offset : 0,
        delimeter : o.delimeter(),
        counter : 0,
      }
    }
  }

  impl< 'a, D > Iterator for SplitFastIterator< 'a, D >
  where
    D : Searcher
  {
    type Item = Split< 'a >;

    fn next( &mut self ) -> Option< Self::Item >
    {
      // println!( "SFI - START - ctr:{}, off:{}, iter:'{}'", self.counter, self.current_offset, self.iterable );
      if self.iterable.is_empty() && self.counter > 0 { return None; }
      self.counter += 1;

      if self.counter % 2 == 1 // ODD: Delimeted segment
      {
        if let Some( ( d_start, _d_end ) ) = self.delimeter.pos( self.iterable ) // _d_end to silence warning
        {
          if d_start == 0
          {
            let split = Split { string: "", typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset };
            // Not advancing state here; EVEN counter will consume the delimiter at current position.
            // println!( "SFI - ODD - YIELD empty seg (delim at start): {:?}", split);
            return Some( split );
          }
          let segment_str = &self.iterable[ ..d_start ];
          let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += segment_str.len();
          self.iterable = &self.iterable[ d_start.. ];
          // println!( "SFI - ODD - YIELD seg: {:?}, new_off:{}, new_iter:'{}'", split, self.current_offset, self.iterable );
          return Some( split );
        }
        let segment_str = self.iterable;
        let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
        self.current_offset += segment_str.len();
        self.iterable = "";
        // println!( "SFI - ODD - YIELD last seg: {:?}", split );
        return Some( split );
      }
      // EVEN: Delimiter
      if let Some( ( d_start, d_end ) ) = self.delimeter.pos( self.iterable )
      {
        if d_start > 0 { self.iterable = ""; return None; }

        let delimiter_str = &self.iterable[ ..d_end ];
        let split = Split { string: delimiter_str, typ: SplitType::Delimiter, start: self.current_offset, end: self.current_offset + delimiter_str.len() };
        self.current_offset += delimiter_str.len();
        self.iterable = &self.iterable[ d_end.. ];
        // println!( "SFI - EVEN - YIELD delim: {:?}, new_off:{}, new_iter:'{}'", split, self.current_offset, self.iterable );
        return Some( split );
      }
      None
    }
  }

  /// An iterator for splitting strings with advanced options like stripping, preserving empty segments, and handling quotes.
  #[ derive( Debug ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct SplitIterator< 'a >
  {
    iterator : SplitFastIterator< 'a, Vec< &'a str > >,
    src : &'a str,
    stripping : bool,
    preserving_empty : bool,
    preserving_delimeters : bool,
    preserving_quoting : bool,
    quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  impl< 'a > SplitIterator< 'a >
  {
    /// Creates a new `SplitIterator` with the given options.
    #[ allow( clippy::needless_pass_by_value ) ]
    fn new( o : impl SplitOptionsAdapter< 'a, Vec< &'a str > > ) -> Self
    {
      let mut delimeter_list_for_fast_iterator;
      if o.quoting()
      {
        delimeter_list_for_fast_iterator = o.quoting_prefixes().clone();
        delimeter_list_for_fast_iterator.extend( o.quoting_postfixes().clone() );
        delimeter_list_for_fast_iterator.extend( o.delimeter() );
      }
      else
      {
        delimeter_list_for_fast_iterator = o.delimeter();
      }
      delimeter_list_for_fast_iterator.retain(|&pat| !pat.is_empty());

      let iterator = SplitFastIterator
      {
        iterable : o.src(),
        current_offset : 0,
        delimeter : delimeter_list_for_fast_iterator,
        counter : 0,
      };
      // println!("SI::new - Initialized with PE:{}, PD:{}, S:{}, Q:{}", o.preserving_empty(), o.preserving_delimeters(), o.stripping(), o.quoting());
      Self
      {
        iterator,
        src : o.src(),
        stripping : o.stripping(),
        preserving_empty : o.preserving_empty(),
        preserving_delimeters : o.preserving_delimeters(),
        preserving_quoting : o.preserving_quoting(),
        quoting : o.quoting(),
        quoting_prefixes : o.quoting_prefixes().clone(),
        quoting_postfixes : o.quoting_postfixes().clone(),
      }
    }
  }

  impl< 'a > Iterator for SplitIterator< 'a >
  {
    type Item = Split< 'a >;

    fn next( &mut self ) -> Option< Self::Item >
    {
      // println!( "SI::next() CALLED. Options: PE:{}, PD:{}, S:{}, Q:{}", self.preserving_empty, self.preserving_delimeters, self.stripping, self.quoting );
      while let Some( raw_split_val ) = self.iterator.next()
      {
        let mut current_split = raw_split_val;
        // println!( "SI - Raw from SFI: {:?}", current_split );

        if self.quoting
        && current_split.typ == SplitType::Delimiter // Corrected from Delimeted
        && self.quoting_prefixes.contains( &current_split.string )
        {
          // println!( "SI - >>> Calling HQS for: {:?}", current_split );
          current_split = self.handle_quoted_section( current_split );
          // println!( "SI - <<< Returned from HQS: {:?}", current_split );
        }

        if self.stripping && current_split.typ == SplitType::Delimeted
        {
          let original_string_ptr = current_split.string.as_ptr();
          let original_len = current_split.string.len();
          let trimmed_string = current_split.string.trim();
          if trimmed_string.len() < original_len || (trimmed_string.is_empty() && original_len > 0)
          {
            let leading_whitespace_len = trimmed_string.as_ptr() as usize - original_string_ptr as usize;
            current_split.start += leading_whitespace_len;
            current_split.string = trimmed_string;
            current_split.end = current_split.start + current_split.string.len();
          }
        }

        let mut skip = false;
        // println!( "SI - Filtering: Split: {:?}, Type: {:?}, Options: PE:{}, PD:{}", current_split.string, current_split.typ, self.preserving_empty, self.preserving_delimeters );
        if current_split.typ == SplitType::Delimeted && current_split.string.is_empty() && !self.preserving_empty { skip = true; /*println!("SI - SKIP empty Dmd");*/ }
        if current_split.typ == SplitType::Delimiter && !self.preserving_delimeters { skip = true; /*println!("SI - SKIP Dlr");*/ }
        // println!( "SI - Filtering: Split: {:?}, Type: {:?}, Options: PE:{}, PD:{}", current_split.string, current_split.typ, self.preserving_empty, self.preserving_delimeters );

        if skip { continue; }

        // println!( "SI - YIELDING: {:?}", current_split );
        return Some( current_split );
      }
      // println!( "SI - SFI exhausted" );
      None
    }
  }

  impl< 'a > SplitIterator< 'a >
  {
    /// Handles a quoted section, consuming the content until the matching postfix.
    ///
    /// # Panics
    ///
    /// Panics if the `prefix_split.string` is not found in `self.quoting_prefixes`.
    fn handle_quoted_section( &mut self, prefix_split : Split< 'a > ) -> Split< 'a >
    {
      let prefix_str = prefix_split.string;
      let prefix_start_abs = prefix_split.start;
      // println!( "HQS --- START --- prefix_split: {:?}, SFI.iter: '{}', SFI.offset: {}", prefix_split, self.iterator.iterable, self.iterator.current_offset );

      let prefix_idx = self.quoting_prefixes.iter().position( |&p| p == prefix_str ).unwrap();
      let expected_postfix = self.quoting_postfixes[prefix_idx];

      let search_space = self.iterator.iterable;
      let search_offset_abs = self.iterator.current_offset;

      // println!("HQS - Searching for postfix '{}' in search_space '{}' (abs_offset: {})", expected_postfix, search_space, search_offset_abs);

      let mut current_search_offset = 0;
      let mut found_postfix_pos : Option< ( usize, usize ) > = None;

      while let Some( ( pos, end_pos ) ) = expected_postfix.pos( &search_space[ current_search_offset.. ] )
      {
        let abs_pos = current_search_offset + pos;
        if abs_pos > 0 && search_space.as_bytes()[ abs_pos - 1 ] == b'\\'
        {
          // It's an escaped postfix, skip it
          current_search_offset = end_pos; // Move past the escaped postfix
          continue;
        }
        // Found unescaped postfix
        found_postfix_pos = Some( ( abs_pos, abs_pos + expected_postfix.len() ) );
        break; // Re-added break to terminate after finding the first unescaped postfix
      }

      if let Some( (postfix_rel_start, postfix_rel_end) ) = found_postfix_pos
      {
        // println!( "HQS - Found postfix '{}' at rel ({},{}) in '{}'", expected_postfix, postfix_rel_start, postfix_rel_end, search_space );
        let content_in_search_space = &search_space[ ..postfix_rel_start ];
        // println!( "HQS - content_in_search_space: '{}'", content_in_search_space);

        let final_str;
        let final_start_abs;
        let final_end_abs;

        if self.preserving_quoting
        {
          final_start_abs = prefix_start_abs;
          final_end_abs = search_offset_abs + postfix_rel_end;
          if final_end_abs > self.src.len() || final_start_abs > final_end_abs { /*println!("HQS - Bounds error PQ=true"); */ return prefix_split; }
          final_str = &self.src[ final_start_abs .. final_end_abs ];
          // println!( "HQS - Preserving quotes: final_str='{}', final_start_abs={}, final_end_abs={}", final_str, final_start_abs, final_end_abs);
        }
        else
        {
          final_start_abs = search_offset_abs;
          final_end_abs = search_offset_abs + content_in_search_space.len();
          if final_end_abs > self.src.len() || final_start_abs > final_end_abs { /*println!("HQS - Bounds error PQ=false"); */ return prefix_split; }
          final_str = content_in_search_space;
          // println!( "HQS - Stripping quotes: final_str='{}', final_start_abs={}, final_end_abs={}", final_str, final_start_abs, final_end_abs);
        }

        let consumed_len_in_iterable = postfix_rel_end;
        // println!( "HQS - Advancing SFI: current_offset was {}, iterable was '{}'", self.iterator.current_offset, self.iterator.iterable );
        // println!( "HQS - Advancing SFI by: {}", consumed_len_in_iterable );
        self.iterator.current_offset += consumed_len_in_iterable;
        self.iterator.iterable = &self.iterator.iterable[ consumed_len_in_iterable.. ];
        self.iterator.counter += 1; // Account for consuming the content and the postfix
        // println!( "HQS - SFI state after advance: offset:{}, iter:'{}', counter:{}", self.iterator.current_offset, self.iterator.iterable, self.iterator.counter );
        Split { string: final_str, typ: SplitType::Delimeted, start: final_start_abs, end: final_end_abs }
      }
      else
      {
        // println!( "HQS --- END (postfix NOT found) --- Prefix as literal: {:?}, SFI.iter: '{}', SFI.offset: {}", prefix_split, self.iterator.iterable, self.iterator.current_offset );
        prefix_split
      }
    }
  }

  /// Options for configuring string splitting behavior for `SplitIterator` and `SplitFastIterator` generic over delimiter type.
  #[ derive( Debug ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct SplitOptions< 'a, D >
  {
    src : &'a str,
    delimeter : D,
    preserving_empty : bool,
    preserving_delimeters : bool,
    preserving_quoting : bool,
    stripping : bool,
    quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  impl< 'a > SplitOptions< 'a, Vec< &'a str > >
  {
    /// Consumes the options and returns a `SplitIterator` for splitting with a `Vec<&str>` delimiter.
    #[ must_use ]
    pub fn split( self ) -> SplitIterator< 'a > { SplitIterator::new( self ) }
  }

  impl< 'a, D > SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone
  {
    /// Consumes the options and returns a `SplitFastIterator` for splitting.
    pub fn split_fast( self ) -> SplitFastIterator< 'a, D > { SplitFastIterator::new( self ) }
  }

  /// Adapter trait to provide a consistent interface for split options.
  pub trait SplitOptionsAdapter< 'a, D > where D : Clone
  {
    /// The source string to be split.
    fn src( &self ) -> &'a str;
    /// The delimiter(s) to split the string by.
    fn delimeter( &self ) -> D;
    /// Whether to preserve empty segments.
    fn preserving_empty( &self ) -> bool;
    /// Whether to preserve delimiters as part of the iteration.
    fn preserving_delimeters( &self ) -> bool;
    /// Whether to preserve quoting characters in the output segments.
    fn preserving_quoting( &self ) -> bool;
    /// Whether to strip leading/trailing whitespace from delimited segments.
    fn stripping( &self ) -> bool;
    /// Whether to enable quote handling.
    fn quoting( &self ) -> bool;
    /// Prefixes that start a quoted section.
    fn quoting_prefixes( &self ) -> &Vec< &'a str >;
    /// Postfixes that end a quoted section.
    fn quoting_postfixes( &self ) -> &Vec< &'a str >;
  }

  impl< 'a, D : Searcher + Clone + Default > SplitOptionsAdapter< 'a, D > for SplitOptions< 'a, D >
  {
    fn src( &self ) -> &'a str { self.src }
    fn delimeter( &self ) -> D { self.delimeter.clone() }
    fn preserving_empty( &self ) -> bool { self.preserving_empty }
    fn preserving_delimeters( &self ) -> bool { self.preserving_delimeters }
    fn preserving_quoting( &self ) -> bool { self.preserving_quoting }
    fn stripping( &self ) -> bool { self.stripping }
    fn quoting( &self ) -> bool { self.quoting }
    fn quoting_prefixes( &self ) -> &Vec< &'a str > { &self.quoting_prefixes }
    fn quoting_postfixes( &self ) -> &Vec< &'a str > { &self.quoting_postfixes }
  }

  /*
  macro_rules! builder_impls_from
  {
    ( $name : ident, $( ( $field : ident, $type : ty ) ),* $( , )? ) =>
    {
      impl< 'a > $name< 'a >
      {
        $( pub fn $field( &mut self, value : $type ) -> &mut $name< 'a > { self.$field = value; self } )*
        pub fn form( &mut self ) -> SplitOptions< 'a, Vec< &'a str > >
        {
          if self.quoting
          {
            if self.quoting_prefixes.is_empty() { self.quoting_prefixes = vec![ "\"", "`", "'" ]; }
            if self.quoting_postfixes.is_empty() { self.quoting_postfixes = vec![ "\"", "`", "'" ]; }
          }
          SplitOptions
          {
            src : self.src,
            delimeter : self.delimeter.clone().vector().unwrap(),
            preserving_empty : self.preserving_empty,
            preserving_delimeters : self.preserving_delimeters,
            preserving_quoting : self.preserving_quoting,
            stripping : self.stripping,
            quoting : self.quoting,
            quoting_prefixes : self.quoting_prefixes.clone(),
            quoting_postfixes : self.quoting_postfixes.clone(),
          }
        }
      }
    }
  }
  */

  /// A builder for `SplitOptions` to configure string splitting.
  #[ allow( clippy::struct_excessive_bools ) ]
  #[ derive( Debug ) ]
  pub struct SplitOptionsFormer< 'a >
  {
    src : &'a str,
    delimeter : OpType< &'a str >,
    preserving_empty : bool,
    preserving_delimeters : bool,
    preserving_quoting : bool,
    stripping : bool,
    quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }
  // builder_impls_from!
  // (
  //   SplitOptionsFormer,
  //   ( preserving_empty, bool ), ( preserving_delimeters, bool ), ( preserving_quoting, bool ),
  //   ( stripping, bool ), ( quoting, bool ),
  //   ( quoting_prefixes, Vec< &'a str > ), ( quoting_postfixes, Vec< &'a str > ),
  // );

  impl< 'a > SplitOptionsFormer< 'a >
  {
    /// Creates a new `SplitOptionsFormer` with a default delimiter.
    pub fn new< D : Into< OpType< &'a str > > >( delimeter : D ) -> SplitOptionsFormer< 'a >
    {
      Self
      {
        src : "", delimeter : OpType::Vector( vec![] ).append( delimeter.into() ),
        preserving_empty : false,
        preserving_delimeters : true, // Changed default to true
        preserving_quoting : false,
        stripping : false, quoting : false,
        quoting_prefixes : vec![], quoting_postfixes : vec![],
      }
    }

    // Manually added setters
    /// Sets whether to preserve empty segments.
    pub fn preserving_empty( &mut self, value : bool ) -> &mut Self { self.preserving_empty = value; self }
    /// Sets whether to preserve delimiters.
    pub fn preserving_delimeters( &mut self, value : bool ) -> &mut Self { self.preserving_delimeters = value; self }
    /// Sets whether to preserve quoting characters.
    pub fn preserving_quoting( &mut self, value : bool ) -> &mut Self { self.preserving_quoting = value; self }
    /// Sets whether to strip whitespace from segments.
    pub fn stripping( &mut self, value : bool ) -> &mut Self { self.stripping = value; self }
    /// Sets whether to enable quote handling.
    pub fn quoting( &mut self, value : bool ) -> &mut Self { self.quoting = value; self }
    /// Sets the quoting prefixes.
    pub fn quoting_prefixes( &mut self, value : Vec< &'a str > ) -> &mut Self { self.quoting_prefixes = value; self }
    /// Sets the quoting postfixes.
    pub fn quoting_postfixes( &mut self, value : Vec< &'a str > ) -> &mut Self { self.quoting_postfixes = value; self }

    // Existing methods that were likely part of the manual impl before, or should be retained
    /// Sets the source string to split.
    pub fn src( &mut self, value : &'a str ) -> &mut Self { self.src = value; self }
    /// Sets the delimiter(s).
    pub fn delimeter< D : Into< OpType< &'a str > > >( &mut self, value : D ) -> &mut Self
    { self.delimeter = OpType::Vector( vec![] ).append( value.into() ); self }

    // Manually added form method
    /// Consumes the builder and returns `SplitOptions` configured for `Vec<&str>` delimiter.
    ///
    /// # Panics
    ///
    /// Panics if the delimiter cannot be converted to a vector.
    pub fn form( &mut self ) -> SplitOptions< 'a, Vec< &'a str > >
    {
      if self.quoting
      {
        if self.quoting_prefixes.is_empty() { self.quoting_prefixes = vec![ "\"", "`", "'" ]; }
        if self.quoting_postfixes.is_empty() { self.quoting_postfixes = vec![ "\"", "`", "'" ]; }
      }
      SplitOptions
      {
        src : self.src,
        delimeter : self.delimeter.clone().vector().unwrap(),
        preserving_empty : self.preserving_empty,
        preserving_delimeters : self.preserving_delimeters,
        preserving_quoting : self.preserving_quoting,
        stripping : self.stripping,
        quoting : self.quoting,
        quoting_prefixes : self.quoting_prefixes.clone(),
        quoting_postfixes : self.quoting_postfixes.clone(),
      }
    }

    // Existing perform method
    /// Consumes the builder, creates `SplitOptions`, and returns a `SplitIterator` for `Vec<&str>` delimiter.
    pub fn perform( &mut self ) -> SplitIterator< 'a > { self.form().split() }
  }

  /// Creates a new `SplitOptionsFormer` for configuring string splitting with default options.
  #[ must_use ]
  pub fn split< 'a >() -> SplitOptionsFormer< 'a > { SplitOptionsFormer::new( <&str>::default() ) }
}

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
    SplitFastIterator,
    SplitIterator,
    split,
    SplitOptionsFormer,
  };
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
  pub use super::own as split; // Alias for the 'own' module itself
  pub use private::
  {
    Split,
    SplitType,
    SplitFastIterator,
    SplitIterator,
    split, // The function
    SplitOptionsFormer,
  };
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( unused_imports ) ] use super::*;
  pub use private::
  {
    SplitOptionsFormer,
    split,
  };
}