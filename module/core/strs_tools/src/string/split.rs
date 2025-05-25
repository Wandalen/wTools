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
    active_quote_char : Option< char >,
  }

  impl< 'a, D : Searcher + Clone > SplitFastIterator< 'a, D >
  {
    #[ allow( dead_code, clippy::needless_pass_by_value ) ]
    fn new( o : impl SplitOptionsAdapter< 'a, D > ) -> Self
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
  }

  impl< 'a, D > Iterator for SplitFastIterator< 'a, D >
  where
    D : Searcher
  {
    type Item = Split< 'a >;

    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.iterable.is_empty() && ( self.counter > 0 || self.active_quote_char.is_some() ) { return None; }

      if let Some( current_quote_char ) = self.active_quote_char
      {
        let mut end_of_quote_idx : Option< usize > = None;
        let mut prev_char_is_escape = false;
        for ( char_idx, ch ) in self.iterable.char_indices()
        {
          if prev_char_is_escape
          {
            prev_char_is_escape = false;
            continue;
          }
          if ch == '\\'
          {
            prev_char_is_escape = true;
            continue;
          }
          if ch == current_quote_char
          {
            end_of_quote_idx = Some( char_idx + ch.len_utf8() );
            break;
          }
        }

        let ( segment_str, consumed_len ) = if let Some( end_idx ) = end_of_quote_idx
        {
          ( &self.iterable[ ..end_idx ], end_idx )
        }
        else
        {
          ( self.iterable, self.iterable.len() )
        };
        
        let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
        self.current_offset += consumed_len;
        self.iterable = &self.iterable[ consumed_len.. ];
        self.counter += 1; 
        return Some( split ); 
      }
      
      if self.iterable.is_empty() && self.counter > 0 { return None; }
      self.counter += 1;

      if self.counter % 2 == 1 // ODD: Delimeted segment
      {
        if let Some( ( d_start, _d_end ) ) = self.delimeter.pos( self.iterable )
        {
          if d_start == 0 
          {
            let split = Split { string: "", typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset };
            return Some( split ); 
          }
          let segment_str = &self.iterable[ ..d_start ];
          let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += segment_str.len();
          self.iterable = &self.iterable[ d_start.. ];
          Some( split ) 
        }
        else 
        {
          if self.iterable.is_empty() { return None; } 
          let segment_str = self.iterable;
          let split = Split { string: segment_str, typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += segment_str.len();
          self.iterable = "";
          Some( split ) 
        }
      }
      // EVEN: Delimiter (No preceding else needed as ODD branch always returns or this is the only path)
      else if let Some( ( d_start, d_end ) ) = self.delimeter.pos( self.iterable )
      {
        if d_start > 0 { self.iterable = ""; return None; } 
        let delimiter_str = &self.iterable[ ..d_end ];
        let split = Split { string: delimiter_str, typ: SplitType::Delimiter, start: self.current_offset, end: self.current_offset + delimiter_str.len() };
        self.current_offset += delimiter_str.len();
        self.iterable = &self.iterable[ d_end.. ];
        Some( split ) 
      }
      else
      {
        None
      }
    }
  }

  /// An iterator for splitting strings with advanced options like stripping,
  /// preserving empty segments, and handling quotes.
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
      let mut delimeter_list_for_fast_iterator = o.delimeter();
      delimeter_list_for_fast_iterator.retain(|&pat| !pat.is_empty());
      let iterator = SplitFastIterator
      {
        iterable : o.src(),
        current_offset : 0,
        delimeter : delimeter_list_for_fast_iterator,
        counter : 0,
        active_quote_char : None,
      };
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
      loop
      {
        let effective_split_opt : Option<Split<'a>>;

        if self.quoting && self.iterator.active_quote_char.is_none()
        {
          if let Some( first_char_iterable ) = self.iterator.iterable.chars().next()
          {
            if let Some( prefix_idx ) = self.quoting_prefixes.iter().position( |p| self.iterator.iterable.starts_with( p ) )
            {
              let prefix_str = self.quoting_prefixes[ prefix_idx ];
              let opening_quote_original_start = self.iterator.current_offset;
              let prefix_len = prefix_str.len();
              let expected_postfix = self.quoting_postfixes[ prefix_idx ];

              self.iterator.current_offset += prefix_len;
              self.iterator.iterable = &self.iterator.iterable[ prefix_len.. ];
              self.iterator.active_quote_char = Some( first_char_iterable );

              let quoted_segment_from_sfi_opt = self.iterator.next();
              self.iterator.active_quote_char = None; 

              if let Some( mut quoted_segment ) = quoted_segment_from_sfi_opt
              {
                if quoted_segment.string.ends_with( expected_postfix )
                {
                  if self.preserving_quoting
                  {
                    quoted_segment.start = opening_quote_original_start;
                    if quoted_segment.end <= self.src.len() && quoted_segment.start < quoted_segment.end
                    {
                      quoted_segment.string = &self.src[ quoted_segment.start .. quoted_segment.end ];
                    }
                  }
                  else 
                  {
                    quoted_segment.string = &quoted_segment.string[ ..quoted_segment.string.len() - expected_postfix.len() ];
                    quoted_segment.end -= expected_postfix.len();
                  }
                }
                else if self.preserving_quoting { 
                   quoted_segment.start = opening_quote_original_start;
                   if quoted_segment.end <= self.src.len() && quoted_segment.start < quoted_segment.end {
                       quoted_segment.string = &self.src[ quoted_segment.start .. quoted_segment.end ];
                   }
                }
                quoted_segment.typ = SplitType::Delimeted;
                effective_split_opt = Some( quoted_segment );
              }
              else 
              {
                let mut prefix_as_token = Split
                {
                  string: prefix_str,
                  typ: SplitType::Delimeted,
                  start: opening_quote_original_start,
                  end: opening_quote_original_start + prefix_len,
                };
                if !self.preserving_quoting && prefix_str == expected_postfix { 
                    prefix_as_token.string = "";
                    prefix_as_token.end = prefix_as_token.start;
                }
                effective_split_opt = Some( prefix_as_token );
              }
            } else { effective_split_opt = self.iterator.next(); } 
          } else { effective_split_opt = self.iterator.next(); } 
        }
        else 
        {
          effective_split_opt = self.iterator.next();
        }

        let mut current_split = effective_split_opt?; 

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
        if current_split.typ == SplitType::Delimeted && current_split.string.is_empty() && !self.preserving_empty { skip = true; }
        if current_split.typ == SplitType::Delimiter && !self.preserving_delimeters { skip = true; }

        if !skip
        {
          return Some( current_split );
        }
      }
    }
  }

  /// Options for configuring string splitting behavior.
  #[ derive( Debug ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone,
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
    /// Consumes the options and returns a `SplitIterator`.
    #[ must_use ]
    pub fn split( self ) -> SplitIterator< 'a > { SplitIterator::new( self ) }
  }

  impl< 'a, D > SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone
  {
    /// Consumes the options and returns a `SplitFastIterator`.
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

  impl< 'a > SplitOptionsFormer< 'a >
  {
    /// Creates a new `SplitOptionsFormer` with default delimiters.
    pub fn new< D : Into< OpType< &'a str > > >( delimeter : D ) -> SplitOptionsFormer< 'a >
    {
      Self
      {
        src : "", delimeter : OpType::Vector( vec![] ).append( delimeter.into() ),
        preserving_empty : false,
        preserving_delimeters : true,
        preserving_quoting : false,
        stripping : false, quoting : false,
        quoting_prefixes : vec![], quoting_postfixes : vec![],
      }
    }

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
    /// Sets the source string to split.
    pub fn src( &mut self, value : &'a str ) -> &mut Self { self.src = value; self }
    /// Sets the delimiter(s).
    pub fn delimeter< D : Into< OpType< &'a str > > >( &mut self, value : D ) -> &mut Self
    { self.delimeter = OpType::Vector( vec![] ).append( value.into() ); self }

    /// Consumes the builder and returns `SplitOptions`.
    ///
    /// # Panics
    ///
    /// Panics if the delimiter cannot be converted to a vector (internal error).
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
    /// Consumes the builder, creates `SplitOptions`, and returns a `SplitIterator`.
    pub fn perform( &mut self ) -> SplitIterator< 'a > { self.form().split() }
  }

  /// Creates a new `SplitOptionsFormer` for string splitting.
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
    Searcher,
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
  pub use super::own as split;
  pub use private::
  {
    Split,
    SplitType,
    SplitFastIterator,
    SplitIterator,
    split,
    SplitOptionsFormer,
    Searcher,
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
    Searcher,
  };
}