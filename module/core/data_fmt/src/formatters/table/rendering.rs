//! Row and border rendering primitives for `TableFormatter`

use super::TableFormatter;
use crate::ansi_str::pad_unicode_width;

impl TableFormatter
{
  /// Format a single-line row (no multiline cells)
  pub( super ) fn format_single_line_row(
    &self,
    output : &mut String,
    cells : &[ String ],
    column_widths : &[ usize ],
    should_pad : bool
  )
  {
    use crate::config::HeaderSeparatorVariant;

    // Add leading border pipe if header separator variant uses pipes.
    // This maintains consistency with header separator formatting - if the separator
    // line has pipes, all rows must also have pipes for proper alignment.
    let needs_border_pipes = matches!(
      self.config.header_sep_variant(),
      HeaderSeparatorVariant::AsciiGrid | HeaderSeparatorVariant::Markdown | HeaderSeparatorVariant::Unicode
    );

    if needs_border_pipes
    {
      let border_char = if matches!( self.config.header_sep_variant(), HeaderSeparatorVariant::Unicode )
      {
        '│'
      }
      else
      {
        '|'
      };
      output.push( border_char );
    }

    for ( idx, cell ) in cells.iter().enumerate()
    {
      let width = column_widths.get( idx ).copied().unwrap_or( 10 );
      let align_right = self.config.col_align_right().get( idx ).copied().unwrap_or( false );

      // Add padding before cell if outer_padding enabled (skip for CSV/TSV)
      if idx == 0 && self.config.has_outer_padding() && should_pad
      {
        output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
      }

      // Apply truncation if max_column_width is set
      //
      // Key Decision: Truncate BEFORE padding (not after)
      // - Rationale: pad_unicode_width expects final content, not pre-padded
      // - Order: truncate → pad → render
      // - Multiline: truncate_cell handles per-line truncation internally
      //
      // Pitfall: DON'T truncate based on `width` (column width from calculate_column_widths)
      // - `width` reflects CURRENT content, may be wider than max_column_width
      // - Always use max_column_width for truncation limit, not calculated width
      // - Example: If content is 50 chars and max_column_width=20, truncate to 20
      //   (not to calculated width of 50)
      let cell_content = if let Some( max_width ) = self.config.max_col_width()
      {
        crate::ansi_str::truncate_cell( cell, max_width, self.config.trunc_marker() )
      }
      else
      {
        cell.clone()
      };

      // Render cell content (padded for aligned formats, raw for CSV/TSV)
      if should_pad
      {
        output.push_str( &pad_unicode_width( &cell_content, width, align_right ) );
      }
      else
      {
        output.push_str( &cell_content );
      }

      // Add column separator (except after last column)
      if idx < cells.len() - 1
      {
        self.append_column_separator( output );
      }

      // Add padding after last cell if outer_padding enabled (skip for CSV/TSV)
      if idx == cells.len() - 1 && self.config.has_outer_padding() && should_pad
      {
        output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
      }
    }

    // Add trailing border pipe if header separator style uses pipes
    if needs_border_pipes
    {
      let border_char = if matches!( self.config.header_sep_variant(), HeaderSeparatorVariant::Unicode )
      {
        '│'
      }
      else
      {
        '|'
      };
      output.push( border_char );
    }

    output.push( '\n' );
  }

  /// Format a multiline row using two-pass algorithm (spec lines 1777-1810)
  ///
  /// Algorithm:
  /// 1. Split all cells into lines and find max line count (row height)
  /// 2. Render each line of the row with borders and separators
  /// 3. Pad shorter cells with empty strings to match row height
  pub( super ) fn format_multiline_row(
    &self,
    output : &mut String,
    cells : &[ String ],
    column_widths : &[ usize ]
  )
  {
    use crate::config::HeaderSeparatorVariant;

    // Pass 1: Split all cells into lines and find maximum line count
    let split_cells : Vec<Vec<&str>> = cells
      .iter()
      .map( |cell| cell.lines().collect() )
      .collect();

    let row_height = split_cells
      .iter()
      .map( std::vec::Vec::len )
      .max()
      .unwrap_or( 1 );

    let needs_border_pipes = matches!(
      self.config.header_sep_variant(),
      HeaderSeparatorVariant::AsciiGrid | HeaderSeparatorVariant::Markdown | HeaderSeparatorVariant::Unicode
    );

    // Pass 2: Render each line of the row
    for line_idx in 0..row_height
    {
      // Add leading border pipe if needed
      if needs_border_pipes
      {
        let border_char = if matches!( self.config.header_sep_variant(), HeaderSeparatorVariant::Unicode )
        {
          '│'
        }
        else
        {
          '|'
        };
        output.push( border_char );
      }

      for ( col_idx, cell_lines ) in split_cells.iter().enumerate()
      {
        let line = cell_lines.get( line_idx ).unwrap_or( &"" );
        let width = column_widths.get( col_idx ).copied().unwrap_or( 10 );
        let align_right = self.config.col_align_right().get( col_idx ).copied().unwrap_or( false );

        // Add padding before cell if outer_padding enabled
        if col_idx == 0 && self.config.has_outer_padding()
        {
          output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
        }

        // Apply truncation to individual line if max_column_width is set
        let line_content = if let Some( max_width ) = self.config.max_col_width()
        {
          crate::ansi_str::truncate_cell( line, max_width, self.config.trunc_marker() )
        }
        else
        {
          (*line).to_string()
        };

        // Pad and render line
        output.push_str( &pad_unicode_width( &line_content, width, align_right ) );

        // Add column separator (except after last column)
        if col_idx < cells.len() - 1
        {
          self.append_column_separator( output );
        }

        // Add padding after last cell if outer_padding enabled
        if col_idx == cells.len() - 1 && self.config.has_outer_padding()
        {
          output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
        }
      }

      // Add trailing border pipe if needed
      if needs_border_pipes
      {
        let border_char = if matches!( self.config.header_sep_variant(), HeaderSeparatorVariant::Unicode )
        {
          '│'
        }
        else
        {
          '|'
        };
        output.push( border_char );
      }

      output.push( '\n' );
    }
  }

  /// Append column separator based on formatter parameters
  fn append_column_separator( &self, output : &mut String )
  {
    match self.config.col_sep()
    {
      crate::config::ColumnSeparator::Spaces( n ) =>
      {
        output.push_str( &" ".repeat( *n ) );
      }
      crate::config::ColumnSeparator::Character( ch ) =>
      {
        output.push( *ch );
      }
      crate::config::ColumnSeparator::String( s ) =>
      {
        output.push_str( s );
      }
    }
  }

  /// Format header separator line
  pub( super ) fn format_header_separator( &self, output : &mut String, column_widths : &[ usize ] )
  {
    use crate::config::HeaderSeparatorVariant;

    match self.config.header_sep_variant()
    {
      HeaderSeparatorVariant::None =>
      {
        // No separator
      }
      HeaderSeparatorVariant::Dash =>
      {
        // Plain dashes under each column
        for ( idx, &width ) in column_widths.iter().enumerate()
        {
          if idx == 0 && self.config.has_outer_padding()
          {
            output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
          }

          output.push_str( &"-".repeat( width ) );

          if idx < column_widths.len() - 1
          {
            self.append_column_separator( output );
          }

          if idx == column_widths.len() - 1 && self.config.has_outer_padding()
          {
            output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
          }
        }
        output.push( '\n' );
      }
      HeaderSeparatorVariant::AsciiGrid =>
      {
        // Grid-style separator matching border rule format: +-----|-----+
        // Fix(issue-014): corners changed from '|' to '+' for AsciiGrid consistency.
        // Root cause: '|' was hardcoded, mismatching the '+' used in border rules.
        // Pitfall: only change the corner/junction chars here; data row pipes stay '|'.
        output.push( '+' );
        for ( idx, &width ) in column_widths.iter().enumerate()
        {
          // Leading padding for first column
          if idx == 0 && self.config.has_outer_padding()
          {
            output.push_str( &"-".repeat( self.config.cell_inner_padding() ) );
          }

          // Dashes for content width
          output.push_str( &"-".repeat( width ) );

          // Trailing padding for last column (before the plus!)
          if idx == column_widths.len() - 1 && self.config.has_outer_padding()
          {
            output.push_str( &"-".repeat( self.config.cell_inner_padding() ) );
          }

          // Column junction as '+' (after all content)
          output.push( '+' );
        }
        output.push( '\n' );
      }
      HeaderSeparatorVariant::Unicode =>
      {
        // Fix(issue-align): delegate to format_unicode_horizontal_rule so outer
        // padding is added only at the two outer edges — matching data row layout.
        // Root cause: `width + 2` added padding around every column junction,
        //   producing separators that were 2*(N-1) chars wider than data rows.
        // Pitfall: never replicate the padding logic inline here; always delegate
        //   to format_unicode_horizontal_rule to keep both paths in sync.
        self.format_unicode_horizontal_rule( output, column_widths, '├', '─', '┼', '┤' );
      }
      HeaderSeparatorVariant::Markdown =>
      {
        // Fix(issue-align): delegate to format_ascii_horizontal_rule for the same
        // outer-edge-only padding reason as the Unicode branch above.
        self.format_ascii_horizontal_rule( output, column_widths, '|', '-', '|', '|' );
      }
    }
  }

  /// Render one ASCII horizontal rule line with parameterized corner/fill/mid chars.
  ///
  /// Used for top border, bottom border, header separator, and inter-row separators
  /// in `BorderVariant::AsciiGrid`. Outer padding (`cell_inner_padding` spaces) is
  /// replaced by `fill` characters at the table's outer edges.
  ///
  /// Example: `widths=[1,1]`, `outer_padding=1`, left='+', fill='-', mid='+', right='+'
  /// → `+--+--+`
  fn format_ascii_horizontal_rule(
    &self,
    output : &mut String,
    widths : &[ usize ],
    left : char,
    fill : char,
    mid : char,
    right : char
  )
  {
    output.push( left );
    for ( idx, &width ) in widths.iter().enumerate()
    {
      if idx == 0 && self.config.has_outer_padding()
      {
        output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
      }
      output.push_str( &fill.to_string().repeat( width ) );
      if idx == widths.len() - 1 && self.config.has_outer_padding()
      {
        output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
      }
      output.push( if idx < widths.len() - 1 { mid } else { right } );
    }
    output.push( '\n' );
  }

  /// Render one Unicode box-drawing horizontal rule line with parameterized chars.
  ///
  /// Same structure as `format_ascii_horizontal_rule`; `'─'` is multi-byte but
  /// `fill.to_string().repeat(n)` counts chars, not bytes, so it works correctly.
  ///
  /// Example: top border → `┌──┬──┐`, bottom → `└──┴──┘`
  fn format_unicode_horizontal_rule(
    &self,
    output : &mut String,
    widths : &[ usize ],
    left : char,
    fill : char,
    mid : char,
    right : char
  )
  {
    output.push( left );
    for ( idx, &width ) in widths.iter().enumerate()
    {
      if idx == 0 && self.config.has_outer_padding()
      {
        output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
      }
      output.push_str( &fill.to_string().repeat( width ) );
      if idx == widths.len() - 1 && self.config.has_outer_padding()
      {
        output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
      }
      output.push( if idx < widths.len() - 1 { mid } else { right } );
    }
    output.push( '\n' );
  }

  /// Emit top border if the border variant requires one.
  ///
  /// `AsciiGrid` → `+---+---+`  (ASCII horizontal rule)
  /// `Unicode`   → `┌───┬───┐`  (Unicode box-drawing top)
  /// Others      → no-op
  pub( super ) fn format_top_border_if_needed( &self, output : &mut String, widths : &[ usize ] )
  {
    use crate::config::BorderVariant;
    match self.config.bdr_variant()
    {
      BorderVariant::AsciiGrid =>
      {
        self.format_ascii_horizontal_rule( output, widths, '+', '-', '+', '+' );
      }
      BorderVariant::Unicode =>
      {
        self.format_unicode_horizontal_rule( output, widths, '┌', '─', '┬', '┐' );
      }
      _ => {}
    }
  }

  /// Emit bottom border if the border variant requires one.
  ///
  /// `AsciiGrid` → `+---+---+`
  /// `Unicode`   → `└───┴───┘`
  /// Others      → no-op
  pub( super ) fn format_bottom_border_if_needed( &self, output : &mut String, widths : &[ usize ] )
  {
    use crate::config::BorderVariant;
    match self.config.bdr_variant()
    {
      BorderVariant::AsciiGrid =>
      {
        self.format_ascii_horizontal_rule( output, widths, '+', '-', '+', '+' );
      }
      BorderVariant::Unicode =>
      {
        self.format_unicode_horizontal_rule( output, widths, '└', '─', '┴', '┘' );
      }
      _ => {}
    }
  }

  /// Emit an inter-row separator if the border variant requires one.
  ///
  /// `AsciiGrid` → `+---+---+`
  /// `Unicode`   → `├───┼───┤`  (same character set as header separator)
  /// Others      → no-op
  pub( super ) fn format_inter_row_sep_if_needed( &self, output : &mut String, widths : &[ usize ] )
  {
    use crate::config::BorderVariant;
    match self.config.bdr_variant()
    {
      BorderVariant::AsciiGrid =>
      {
        self.format_ascii_horizontal_rule( output, widths, '+', '-', '+', '+' );
      }
      BorderVariant::Unicode =>
      {
        self.format_unicode_horizontal_rule( output, widths, '├', '─', '┼', '┤' );
      }
      _ => {}
    }
  }
}
