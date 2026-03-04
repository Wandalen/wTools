//! Filtering utilities for Claude Code storage
//!
//! Zero-dependency filtering with case-insensitive substring matching.

/// Zero-dependency case-insensitive substring matcher
///
/// ## Design
///
/// Uses `to_lowercase()` for Unicode-aware case-insensitive matching.
/// Pattern is stored in lowercase form to avoid repeated conversions.
///
/// ## Examples
///
/// ```rust
/// use claude_storage_core::StringMatcher;
///
/// let matcher = StringMatcher::new( "WillBe" );
/// assert!( matcher.matches( "claude_storage/willbe/src" ) );
/// assert!( matcher.matches( "CLAUDE_STORAGE/WILLBE/SRC" ) );
/// assert!( !matcher.matches( "claude_storage/wplan/src" ) );
/// ```
///
/// ## Empty Pattern
///
/// Empty pattern matches all text:
///
/// ```rust
/// use claude_storage_core::StringMatcher;
///
/// let matcher = StringMatcher::new( "" );
/// assert!( matcher.matches( "anything" ) );
/// assert!( matcher.matches( "" ) );
/// ```
pub struct StringMatcher
{
  pattern : String, // Lowercased for case-insensitive matching
}

impl StringMatcher
{
  /// Create new matcher with case-insensitive pattern
  ///
  /// Pattern is converted to lowercase for Unicode-aware matching.
  ///
  /// ## Examples
  ///
  /// ```rust
  /// use claude_storage_core::StringMatcher;
  ///
  /// let matcher = StringMatcher::new( "WillBe" );
  /// assert!( matcher.matches( "willbe" ) );
  /// ```
  pub fn new( pattern : impl Into< String > ) -> Self
  {
    let pattern = pattern.into().to_lowercase();
    Self { pattern }
  }

  /// Check if text matches pattern (case-insensitive)
  ///
  /// ## Behavior
  ///
  /// - Empty pattern matches all text
  /// - Case-insensitive substring match
  /// - Unicode-aware with `to_lowercase()`
  ///
  /// ## Examples
  ///
  /// ```rust
  /// use claude_storage_core::StringMatcher;
  ///
  /// let matcher = StringMatcher::new( "storage" );
  /// assert!( matcher.matches( "claude_storage_core" ) );
  /// assert!( matcher.matches( "CLAUDE_STORAGE_CORE" ) );
  /// assert!( !matcher.matches( "claude_session" ) );
  /// ```
  pub fn matches( &self, text : &str ) -> bool
  {
    if self.pattern.is_empty()
    {
      return true; // Empty pattern matches all
    }

    text.to_lowercase().contains( &self.pattern )
  }
}

/// Session-level filtering
///
/// ## Examples
///
/// ```rust
/// use claude_storage_core::SessionFilter;
///
/// // Filter for agent sessions with 10+ entries
/// let filter = SessionFilter
/// {
///   agent_only : Some( true ),
///   min_entries : Some( 10 ),
///   session_id_substring : None,
/// };
/// ```
#[derive( Debug, Clone, Default )]
pub struct SessionFilter
{
  /// Filter by agent session type
  /// - None: No filtering (show all)
  /// - Some(true): Only agent sessions
  /// - Some(false): Only main sessions
  pub agent_only : Option< bool >,

  /// Minimum entry count (inclusive)
  pub min_entries : Option< usize >,

  /// Session ID substring match (case-insensitive)
  pub session_id_substring : Option< String >,
}

impl SessionFilter
{
  /// Create default filter (no filtering)
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Check if filter has any active conditions
  pub fn is_default( &self ) -> bool
  {
    self.agent_only.is_none()
      && self.min_entries.is_none()
      && self.session_id_substring.is_none()
  }
}

/// Project-level filtering
///
/// ## Examples
///
/// ```rust
/// use claude_storage_core::ProjectFilter;
///
/// // Filter for projects with "willbe" in path and 5+ sessions
/// let filter = ProjectFilter
/// {
///   path_substring : Some( "willbe".to_string() ),
///   min_entries : None,
///   min_sessions : Some( 5 ),
/// };
/// ```
#[derive( Debug, Clone, Default )]
pub struct ProjectFilter
{
  /// Path substring match (case-insensitive)
  pub path_substring : Option< String >,

  /// Minimum total entries across all sessions
  pub min_entries : Option< usize >,

  /// Minimum session count
  pub min_sessions : Option< usize >,
}

impl ProjectFilter
{
  /// Create default filter (no filtering)
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Check if filter has any active conditions
  pub fn is_default( &self ) -> bool
  {
    self.path_substring.is_none()
      && self.min_entries.is_none()
      && self.min_sessions.is_none()
  }
}
