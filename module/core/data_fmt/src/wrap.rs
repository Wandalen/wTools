//! Word-wrap utility for `data_fmt`.
//!
//! Provides `WrapFormatter` and `WrapConfig` for wrapping strings to a
//! configured column width with independent first-line and continuation-line
//! indent strings, configurable break strategies, and overflow handling.

/// Controls how lines are broken when content exceeds `WrapConfig::width`.
#[ derive( Debug, Clone, Default, PartialEq ) ]
pub enum BreakStrategy
{
  /// Break at the last word boundary before the limit.
  /// Overlong tokens (longer than available width) are handled by `break_long_words`.
  Word,
  /// Break at exactly `width` chars, ignoring word boundaries.
  Hard,
  /// Word-boundary first; hard-break only when a single token exceeds available width.
  #[ default ]
  WordThenHard,
}

/// Controls output behavior when `max_lines` is exceeded.
#[ derive( Debug, Clone, PartialEq ) ]
pub enum Overflow
{
  /// Drop excess lines silently.
  Truncate,
  /// Append a suffix to the last kept line, truncating content so line length ≤ `width`.
  Ellipsis( String ),
}

/// Configuration for word-wrap behavior.
///
/// All fields have documented defaults; use builder methods to customize.
#[ derive( Debug, Clone ) ]
pub struct WrapConfig
{
  /// Maximum line width measured in Unicode chars. Default: `80`.
  pub width : usize,
  /// Prefix prepended to line 0 of the output. Default: `""`.
  pub initial_indent : String,
  /// Prefix prepended to all output lines after the first. Default: `""`.
  pub subsequent_indent : String,
  /// Line-breaking strategy. Default: `BreakStrategy::WordThenHard`.
  pub break_strategy : BreakStrategy,
  /// Hard-break words that exceed available width (used by `Word` strategy). Default: `true`.
  pub break_long_words : bool,
  /// Treat `\n` in input as a hard line break. Default: `true`.
  pub preserve_newlines : bool,
  /// Maximum number of output lines; `None` means unlimited. Default: `None`.
  pub max_lines : Option< usize >,
  /// Behavior when output exceeds `max_lines`. Default: `Overflow::Truncate`.
  pub overflow : Overflow,
  /// Number of spaces each `\t` expands to before processing. Default: `4`.
  pub tab_width : usize,
}

impl WrapConfig
{
  /// Create a `WrapConfig` with all default values.
  pub fn new() -> Self
  {
    Self
    {
      width : 80,
      initial_indent : String::new(),
      subsequent_indent : String::new(),
      break_strategy : BreakStrategy::WordThenHard,
      break_long_words : true,
      preserve_newlines : true,
      max_lines : None,
      overflow : Overflow::Truncate,
      tab_width : 4,
    }
  }

  /// Set the target line width in Unicode chars.
  #[ must_use ]
  pub fn width( mut self, width : usize ) -> Self
  {
    self.width = width;
    self
  }

  /// Set the prefix for the first output line.
  #[ must_use ]
  pub fn initial_indent( mut self, indent : String ) -> Self
  {
    self.initial_indent = indent;
    self
  }

  /// Set the prefix for continuation lines (lines 1+).
  #[ must_use ]
  pub fn subsequent_indent( mut self, indent : String ) -> Self
  {
    self.subsequent_indent = indent;
    self
  }

  /// Set both `initial_indent` and `subsequent_indent` to the same value.
  #[ must_use ]
  pub fn indent( mut self, indent : String ) -> Self
  {
    self.subsequent_indent.clone_from( &indent );
    self.initial_indent = indent;
    self
  }

  /// Set the break strategy.
  #[ must_use ]
  pub fn break_strategy( mut self, strategy : BreakStrategy ) -> Self
  {
    self.break_strategy = strategy;
    self
  }

  /// Whether to hard-break words that exceed available width (for `Word` strategy).
  #[ must_use ]
  pub fn break_long_words( mut self, val : bool ) -> Self
  {
    self.break_long_words = val;
    self
  }

  /// Whether to treat `\n` in input as a hard line break.
  #[ must_use ]
  pub fn preserve_newlines( mut self, val : bool ) -> Self
  {
    self.preserve_newlines = val;
    self
  }

  /// Set maximum number of output lines.
  #[ must_use ]
  pub fn max_lines( mut self, n : usize ) -> Self
  {
    self.max_lines = Some( n );
    self
  }

  /// Set overflow behavior when output exceeds `max_lines`.
  #[ must_use ]
  pub fn overflow( mut self, overflow : Overflow ) -> Self
  {
    self.overflow = overflow;
    self
  }

  /// Set the number of spaces each `\t` expands to.
  #[ must_use ]
  pub fn tab_width( mut self, width : usize ) -> Self
  {
    self.tab_width = width;
    self
  }
}

impl Default for WrapConfig
{
  fn default() -> Self
  {
    Self::new()
  }
}

/// Word-wrap formatter; wraps strings to configured width using `WrapConfig`.
#[ derive( Debug, Clone ) ]
pub struct WrapFormatter
{
  config : WrapConfig,
}

impl WrapFormatter
{
  /// Create a `WrapFormatter` with default `WrapConfig` (width=80, no indent).
  pub fn new() -> Self
  {
    Self { config : WrapConfig::new() }
  }

  /// Create a `WrapFormatter` with the given configuration.
  pub fn with_config( config : WrapConfig ) -> Self
  {
    Self { config }
  }

  /// Wrap `text` according to the current config.
  ///
  /// Returns one `String` per output line; each entry has the appropriate
  /// indent prepended. An empty input returns an empty `Vec`.
  pub fn wrap( &self, text : &str ) -> Vec< String >
  {
    if text.is_empty() { return Vec::new(); }
    let expanded = expand_tabs( text, self.config.tab_width );
    let segments : Vec< String > = if self.config.preserve_newlines
    {
      expanded.split( '\n' ).map( String::from ).collect()
    }
    else
    {
      vec![ expanded.replace( '\n', " " ) ]
    };
    let mut lines : Vec< String > = Vec::new();
    for segment in &segments
    {
      let seg_lines = self.wrap_segment( segment, lines.len() );
      lines.extend( seg_lines );
    }
    if let Some( max ) = self.config.max_lines
    {
      if lines.len() > max
      {
        lines.truncate( max );
        apply_overflow( &mut lines, &self.config );
      }
    }
    lines
  }

  /// Wrap `text` and join the resulting lines with `"\n"`.
  ///
  /// Equivalent to `self.wrap(text).join("\n")`.
  pub fn wrap_joined( &self, text : &str ) -> String
  {
    self.wrap( text ).join( "\n" )
  }

  fn wrap_segment( &self, text : &str, first_line_idx : usize ) -> Vec< String >
  {
    let words : Vec< &str > = text.split_whitespace().collect();
    if words.is_empty() { return vec![ String::new() ]; }
    if matches!( self.config.break_strategy, BreakStrategy::Hard )
    {
      return hard_break_str( &words.join( " " ), &self.config, first_line_idx );
    }
    wrap_words( &words, &self.config, first_line_idx )
  }
}

impl Default for WrapFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

// --- private helpers ---

fn char_count( s : &str ) -> usize
{
  s.chars().count()
}

fn indent_for( line_idx : usize, config : &WrapConfig ) -> &str
{
  if line_idx == 0 { &config.initial_indent } else { &config.subsequent_indent }
}

fn available_for( line_idx : usize, config : &WrapConfig ) -> usize
{
  config.width.saturating_sub( char_count( indent_for( line_idx, config ) ) )
}

fn expand_tabs( text : &str, tab_width : usize ) -> String
{
  // Fix( issue-004a ): removed early return for tab_width==0.
  // Root cause: returning `text.to_string()` kept literal `\t` instead of
  // deleting it; `tab_width=0` means "replace with 0 spaces" (i.e. delete).
  // Pitfall: `split_whitespace` masks the bug for Word/WordThenHard strategies,
  // so the incorrect behaviour only surfaces if whitespace is preserved.
  text.replace( '\t', &" ".repeat( tab_width ) )
}

/// Hard-break `text` at `width` chars per line, prepending the appropriate indent.
fn hard_break_str( text : &str, config : &WrapConfig, first_line_idx : usize ) -> Vec< String >
{
  let mut lines = Vec::new();
  let mut remaining = text;
  while !remaining.is_empty()
  {
    let li = first_line_idx + lines.len();
    let indent = indent_for( li, config );
    let avail = config.width.saturating_sub( char_count( indent ) );
    if avail == 0
    {
      lines.push( indent.to_string() );
      break;
    }
    let byte_end = remaining
      .char_indices()
      .nth( avail )
      .map_or( remaining.len(), | ( i, _ ) | i );
    lines.push( format!( "{indent}{}", &remaining[ ..byte_end ] ) );
    // Fix( issue-004b ): trim leading spaces after extracting each chunk.
    // Root cause: `words.join(" ")` inserts inter-word spaces; after slicing out
    // `avail` chars the leftover may start with a space from the separator,
    // causing the next chunk to begin with that space (e.g. " worl" instead of "world").
    // Pitfall: only manifests when the chunk boundary falls on a word-separator
    // space, not at the middle of a word.
    remaining = remaining[ byte_end.. ].trim_start_matches( ' ' );
  }
  lines
}

/// Place `word` on its own line, hard-breaking if the strategy and `break_long_words` require it.
fn push_overlong_word(
  word : &str,
  lines : &mut Vec< String >,
  config : &WrapConfig,
  first_line_idx : usize,
)
{
  let hard_break = matches!( config.break_strategy, BreakStrategy::WordThenHard )
    || config.break_long_words;
  if hard_break
  {
    // Fix( issue-004c ): compute `avail` per output line inside the loop.
    // Root cause: the old code computed `avail` once from `line_idx` (the first line)
    // then passed the same chunk size to `hard_chunks`; continuation chunks landed on
    // lines with `subsequent_indent`, so `indent + chunk` exceeded `width`.
    // Pitfall: only visible when `initial_indent != subsequent_indent`.
    let mut remaining = word;
    while !remaining.is_empty()
    {
      let li = first_line_idx + lines.len();
      let indent = indent_for( li, config );
      let avail = config.width.saturating_sub( char_count( indent ) );
      if avail == 0
      {
        lines.push( indent.to_string() );
        break;
      }
      let byte_end = remaining
        .char_indices()
        .nth( avail )
        .map_or( remaining.len(), | ( i, _ ) | i );
      lines.push( format!( "{indent}{}", &remaining[ ..byte_end ] ) );
      remaining = &remaining[ byte_end.. ];
    }
  }
  else
  {
    let line_idx = first_line_idx + lines.len();
    let indent = indent_for( line_idx, config );
    lines.push( format!( "{indent}{word}" ) );
  }
}

fn flush_pending(
  lines : &mut Vec< String >,
  pending : &mut Vec< &str >,
  pending_width : &mut usize,
  line_idx : usize,
  config : &WrapConfig,
)
{
  if !pending.is_empty()
  {
    let indent = indent_for( line_idx, config );
    let joined = pending.join( " " );
    lines.push( format!( "{indent}{joined}" ) );
    pending.clear();
    *pending_width = 0;
  }
}

/// Word-wrap `words` into lines, prepending the appropriate indent to each line.
fn wrap_words( words : &[ &str ], config : &WrapConfig, first_line_idx : usize ) -> Vec< String >
{
  let mut lines : Vec< String > = Vec::new();
  let mut pending : Vec< &str > = Vec::new();
  let mut pending_width = 0_usize;
  for &word in words
  {
    let word_width = char_count( word );
    let line_idx = first_line_idx + lines.len();
    let avail = available_for( line_idx, config );
    if pending.is_empty()
    {
      if word_width <= avail
      {
        pending.push( word );
        pending_width = word_width;
      }
      else
      {
        push_overlong_word( word, &mut lines, config, first_line_idx );
      }
    }
    else if pending_width + 1 + word_width <= avail
    {
      pending.push( word );
      pending_width += 1 + word_width;
    }
    else
    {
      flush_pending( &mut lines, &mut pending, &mut pending_width, line_idx, config );
      let new_idx = first_line_idx + lines.len();
      let new_avail = available_for( new_idx, config );
      if word_width <= new_avail
      {
        pending.push( word );
        pending_width = word_width;
      }
      else
      {
        push_overlong_word( word, &mut lines, config, first_line_idx );
      }
    }
  }
  if !pending.is_empty()
  {
    let line_idx = first_line_idx + lines.len();
    let indent = indent_for( line_idx, config );
    let joined = pending.join( " " );
    lines.push( format!( "{indent}{joined}" ) );
  }
  lines
}

/// Append an ellipsis suffix to the last line in `lines`, truncating content to keep ≤ `width`.
fn apply_overflow( lines : &mut [ String ], config : &WrapConfig )
{
  if let Overflow::Ellipsis( ellipsis ) = &config.overflow
  {
    let line_idx = lines.len().saturating_sub( 1 );
    let indent = indent_for( line_idx, config );
    let indent_len = char_count( indent );
    let ellipsis_len = char_count( ellipsis );
    let avail = config.width.saturating_sub( indent_len + ellipsis_len );
    let indent_owned = indent.to_string();
    let ellipsis_owned = ellipsis.clone();
    if let Some( last ) = lines.last_mut()
    {
      let content : String = last.chars().skip( indent_len ).take( avail ).collect();
      *last = format!( "{indent_owned}{content}{ellipsis_owned}" );
    }
  }
}
