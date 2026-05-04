//! Text buffer with cursor management and Unicode support

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Text buffer managing lines of text with cursor position
#[derive( Debug, Clone )]
pub struct TextBuffer
{
  /// Lines of text
  pub lines: Vec< String >,

  /// Current cursor line (0-indexed)
  pub cursor_line: usize,

  /// Current cursor column in grapheme clusters (0-indexed)
  pub cursor_col: usize,
}

impl TextBuffer
{
  /// Create new empty buffer
  pub fn new() -> Self
  {
    Self
    {
      lines: vec![ String::new() ],
      cursor_line: 0,
      cursor_col: 0,
    }
  }

  /// Create buffer with initial text
  pub fn with_text( text: &str ) -> Self
  {
    let lines: Vec< String > = text.lines().map( |s| s.to_string() ).collect();
    let lines = if lines.is_empty()
    {
      vec![ String::new() ]
    }
    else
    {
      lines
    };

    Self
    {
      lines,
      cursor_line: 0,
      cursor_col: 0,
    }
  }

  /// Get all text as single string
  pub fn text( &self ) -> String
  {
    self.lines.join( "\n" )
  }

  /// Get current line count
  pub fn line_count( &self ) -> usize
  {
    self.lines.len()
  }

  /// Get character count (total length)
  pub fn char_count( &self ) -> usize
  {
    self.text().chars().count()
  }

  /// Get current cursor position (line, column)
  pub fn cursor_position( &self ) -> ( usize, usize )
  {
    ( self.cursor_line, self.cursor_col )
  }

  /// Get current line
  pub fn current_line( &self ) -> &str
  {
    &self.lines[ self.cursor_line ]
  }

  /// Get line at index
  pub fn line( &self, index: usize ) -> Option< &str >
  {
    self.lines.get( index ).map( |s| s.as_str() )
  }

  /// Get all lines
  pub fn lines( &self ) -> &[ String ]
  {
    &self.lines
  }

  /// Insert character at cursor position
  pub fn insert_char( &mut self, ch: char )
  {
    if ch == '\n'
    {
      self.insert_newline();
    }
    else
    {
      // Calculate byte position first
      let byte_pos = self.grapheme_to_byte_index( &self.lines[ self.cursor_line ], self.cursor_col );

      // Now modify the line
      let line = &mut self.lines[ self.cursor_line ];
      line.insert( byte_pos, ch );
      self.cursor_col += 1;
    }
  }

  /// Insert newline at cursor position
  pub fn insert_newline( &mut self )
  {
    let line = &self.lines[ self.cursor_line ];
    let byte_pos = self.grapheme_to_byte_index( line, self.cursor_col );

    let rest = line[ byte_pos.. ].to_string();
    self.lines[ self.cursor_line ].truncate( byte_pos );

    self.cursor_line += 1;
    self.lines.insert( self.cursor_line, rest );
    self.cursor_col = 0;
  }

  /// Delete character before cursor (backspace)
  pub fn delete_char_before( &mut self )
  {
    if self.cursor_col > 0
    {
      // Delete within line
      // Calculate byte positions first
      let byte_pos = self.grapheme_to_byte_index( &self.lines[ self.cursor_line ], self.cursor_col );
      let prev_byte_pos = self.grapheme_to_byte_index( &self.lines[ self.cursor_line ], self.cursor_col - 1 );

      // Now modify the line
      let line = &mut self.lines[ self.cursor_line ];
      line.drain( prev_byte_pos..byte_pos );
      self.cursor_col -= 1;
    }
    else if self.cursor_line > 0
    {
      // Delete newline, merge with previous line
      let current_line = self.lines.remove( self.cursor_line );
      self.cursor_line -= 1;

      let prev_line = &self.lines[ self.cursor_line ];
      self.cursor_col = prev_line.graphemes( true ).count();

      self.lines[ self.cursor_line ].push_str( &current_line );
    }
  }

  /// Delete character at cursor (delete key)
  pub fn delete_char_at( &mut self )
  {
    let line_len = self.lines[ self.cursor_line ].graphemes( true ).count();

    if self.cursor_col < line_len
    {
      // Delete within line
      // Calculate byte positions first
      let byte_pos = self.grapheme_to_byte_index( &self.lines[ self.cursor_line ], self.cursor_col );
      let next_byte_pos = self.grapheme_to_byte_index( &self.lines[ self.cursor_line ], self.cursor_col + 1 );

      // Now modify the line
      let line = &mut self.lines[ self.cursor_line ];
      line.drain( byte_pos..next_byte_pos );
    }
    else if self.cursor_line < self.lines.len() - 1
    {
      // Delete newline, merge with next line
      let next_line = self.lines.remove( self.cursor_line + 1 );
      self.lines[ self.cursor_line ].push_str( &next_line );
    }
  }

  /// Move cursor left
  pub fn move_left( &mut self )
  {
    if self.cursor_col > 0
    {
      self.cursor_col -= 1;
    }
    else if self.cursor_line > 0
    {
      self.cursor_line -= 1;
      let line = &self.lines[ self.cursor_line ];
      self.cursor_col = line.graphemes( true ).count();
    }
  }

  /// Move cursor right
  pub fn move_right( &mut self )
  {
    let line = &self.lines[ self.cursor_line ];
    let line_len = line.graphemes( true ).count();

    if self.cursor_col < line_len
    {
      self.cursor_col += 1;
    }
    else if self.cursor_line < self.lines.len() - 1
    {
      self.cursor_line += 1;
      self.cursor_col = 0;
    }
  }

  /// Move cursor up
  pub fn move_up( &mut self )
  {
    if self.cursor_line > 0
    {
      self.cursor_line -= 1;
      self.clamp_cursor_to_line();
    }
  }

  /// Move cursor down
  pub fn move_down( &mut self )
  {
    if self.cursor_line < self.lines.len() - 1
    {
      self.cursor_line += 1;
      self.clamp_cursor_to_line();
    }
  }

  /// Move cursor to start of line
  pub fn move_to_line_start( &mut self )
  {
    self.cursor_col = 0;
  }

  /// Move cursor to end of line
  pub fn move_to_line_end( &mut self )
  {
    let line = &self.lines[ self.cursor_line ];
    self.cursor_col = line.graphemes( true ).count();
  }

  /// Move cursor to start of text
  pub fn move_to_text_start( &mut self )
  {
    self.cursor_line = 0;
    self.cursor_col = 0;
  }

  /// Move cursor to end of text
  pub fn move_to_text_end( &mut self )
  {
    self.cursor_line = self.lines.len() - 1;
    self.move_to_line_end();
  }

  /// Clamp cursor column to current line length
  fn clamp_cursor_to_line( &mut self )
  {
    let line = &self.lines[ self.cursor_line ];
    let line_len = line.graphemes( true ).count();
    if self.cursor_col > line_len
    {
      self.cursor_col = line_len;
    }
  }

  /// Convert grapheme cluster index to byte index
  fn grapheme_to_byte_index( &self, s: &str, grapheme_index: usize ) -> usize
  {
    s.graphemes( true )
      .take( grapheme_index )
      .map( |g| g.len() )
      .sum()
  }

  /// Get display width of line (accounts for wide chars)
  pub fn line_display_width( &self, line_index: usize ) -> usize
  {
    self.lines.get( line_index )
      .map( |line| UnicodeWidthStr::width( line.as_str() ) )
      .unwrap_or( 0 )
  }

  /// Get display width of current line
  pub fn current_line_display_width( &self ) -> usize
  {
    self.line_display_width( self.cursor_line )
  }
}

impl Default for TextBuffer
{
  fn default() -> Self
  {
    Self::new()
  }
}
