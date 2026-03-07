//! Content search functionality for Claude Code storage
//!
//! Zero-dependency streaming search through session content.

use crate::{ EntryType, StringMatcher };

/// Search filter for session content
///
/// ## Examples
///
/// ```rust
/// use claude_storage_core::SearchFilter;
///
/// // Basic case-insensitive search
/// let filter = SearchFilter::new( "error_tools" );
///
/// // Case-sensitive search
/// let filter = SearchFilter::new( "ErrorTools" ).case_sensitive( true );
/// ```
#[derive( Debug, Clone )]
pub struct SearchFilter
{
  /// Search query
  pub query : String,

  /// Case-sensitive matching (default: false)
  pub case_sensitive : bool,

  /// Filter by entry type (user/assistant)
  pub match_entry_type : Option< EntryType >,
}

impl SearchFilter
{
  /// Create new search filter with query
  pub fn new( query : impl Into< String > ) -> Self
  {
    Self
    {
      query : query.into(),
      case_sensitive : false,
      match_entry_type : None,
    }
  }

  /// Set case-sensitive matching
  #[must_use] 
  pub fn case_sensitive( mut self, value : bool ) -> Self
  {
    self.case_sensitive = value;
    self
  }

  /// Filter by entry type
  #[must_use] 
  pub fn match_entry_type( mut self, entry_type : EntryType ) -> Self
  {
    self.match_entry_type = Some( entry_type );
    self
  }

  /// Check if text matches query
  #[must_use] 
  pub fn matches_text( &self, text : &str ) -> bool
  {
    if self.case_sensitive
    {
      text.contains( &self.query )
    }
    else
    {
      let matcher = StringMatcher::new( &self.query );
      matcher.matches( text )
    }
  }
}

/// Search match result with context
#[derive( Debug, Clone )]
pub struct SearchMatch
{
  /// Zero-based entry index in session
  pub entry_index : usize,

  /// Entry type (user/assistant)
  pub entry_type : EntryType,

  /// Line number within entry content (zero-based)
  pub line_number : usize,

  /// Matched line with surrounding context
  /// Format: "...{50 chars before}MATCH{50 chars after}..."
  pub excerpt : String,

  /// Full matched line (without truncation)
  pub full_line : String,
}

impl SearchMatch
{
  /// Create new search match
  #[must_use] 
  pub fn new
  (
    entry_index : usize,
    entry_type : EntryType,
    line_number : usize,
    full_line : String,
  )
  -> Self
  {
    // Create excerpt (context around match)
    // UTF-8 safe truncation using char indices
    let excerpt = if full_line.chars().count() > 150
    {
      // Truncate long lines, showing middle part
      let char_count = full_line.chars().count();
      let start_char = char_count.saturating_sub( 100 ) / 2;
      let end_char = start_char + 100;

      let excerpt_str : String = full_line
        .chars()
        .skip( start_char )
        .take( end_char - start_char )
        .collect();

      format!( "...{excerpt_str}..." )
    }
    else
    {
      full_line.clone()
    };

    Self
    {
      entry_index,
      entry_type,
      line_number,
      excerpt,
      full_line,
    }
  }

  /// Get entry index
  #[must_use] 
  pub fn entry_index( &self ) -> usize
  {
    self.entry_index
  }

  /// Get entry type
  #[must_use] 
  pub fn entry_type( &self ) -> EntryType
  {
    self.entry_type
  }

  /// Get line number
  #[must_use] 
  pub fn line_number( &self ) -> usize
  {
    self.line_number
  }

  /// Get excerpt
  #[must_use] 
  pub fn excerpt( &self ) -> &str
  {
    &self.excerpt
  }

  /// Get full line
  #[must_use] 
  pub fn full_line( &self ) -> &str
  {
    &self.full_line
  }
}
