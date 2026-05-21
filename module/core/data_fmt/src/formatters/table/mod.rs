//! `TableFormatter` for rendering tabular data with 9 distinct styles
//!
//! ## Available Styles
//!
//! ### Plain (default)
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .build_view();
//! let formatter = TableFormatter::new();
//! // Name   Age
//! // ----   ---
//! // Alice  30
//! ```
//!
//! ### Minimal (no separator)
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::minimal() );
//! // Name
//! // Alice
//! ```
//!
//! ### Bordered
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::bordered() );
//! //  Name  | Age
//! // -------+-----
//! //  Alice |  30
//! ```
//!
//! ### Markdown
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::markdown() );
//! // | Name  | Age |
//! // |-------|-----|
//! // | Alice | 30  |
//! ```
//!
//! ### Grid (ASCII box)
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::grid() );
//! // +-------+-----+
//! // | Name  | Age |
//! // +-------+-----+
//! // | Alice | 30  |
//! // +-------+-----+
//! ```
//!
//! ### Unicode Box
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::unicode_box() );
//! // ┌───────┬─────┐
//! // │ Name  │ Age │
//! // ├───────┼─────┤
//! // │ Alice │ 30  │
//! // └───────┴─────┘
//! ```
//!
//! ### CSV
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::csv() );
//! // Name,Age
//! // Alice,30
//! ```
//!
//! ### TSV
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::tsv() );
//! // Name    Age
//! // Alice    30
//! ```
//!
//! ### Compact
//! ```
//! # use data_fmt::{ RowBuilder, TableFormatter, TableConfig };
//! # let view = RowBuilder::new(vec!["Name".into()]).add_row(vec!["Alice".into()]).build_view();
//! let formatter = TableFormatter::with_config( TableConfig::compact() );
//! // Name  Age
//! // Alice  30
//! ```

use crate::{ TreeNode, TableConfig };
use crate::ansi_str::{ unicode_visual_len, pad_unicode_width };
use color_tools::DecoratedText;

/// Initial string capacity for table output
const INITIAL_CAPACITY : usize = 512;

/// Formatter for rendering tabular data as strings
///
/// Provides table rendering with configurable borders, column widths,
/// and alignment. Automatically handles ANSI color codes for proper alignment.
#[ derive( Debug ) ]
pub struct TableFormatter
{
  pub( super ) config : TableConfig,
}

impl Default for TableFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl TableFormatter
{
  /// Create a new table formatter with default formatter parameters
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::TableFormatter;
  ///
  /// let formatter = TableFormatter::new();
  /// ```
  pub fn new() -> Self
  {
    Self
    {
      config : TableConfig::default(),
    }
  }

  /// Create a table formatter with custom formatter parameters
  pub const fn with_config( config : TableConfig ) -> Self
  {
    Self
    {
      config,
    }
  }

  /// Format hierarchical tree as table (flattened)
  ///
  /// Flattens hierarchical tree to table with columns: path, name, depth, data.
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::{ TreeBuilder, TableFormatter };
  ///
  /// let tree = TreeBuilder::new( "root" )
  ///   .insert( &[ "file.txt" ], 100 )
  ///   .build();
  ///
  /// let formatter = TableFormatter::new();
  /// let output = formatter.format_tree( &tree );
  ///
  /// assert!( output.contains( "path" ) );
  /// assert!( output.contains( "file.txt" ) );
  /// ```
  pub fn format_tree< T : std::fmt::Display >( &self, tree : &TreeNode< T > ) -> String
  {
    let flattened = crate::conversions::flatten_to_table_tree( tree );
    super::Format::format( self, &flattened ).unwrap_or_default()
  }

  /// Internal implementation of table formatting
  fn format_internal
  (
    &self,
    headers : &[ String ],
    rows : &[ Vec< DecoratedText > ],
    row_details : &[ Option< DecoratedText > ],
  )
  -> String
  {
    // Fix( issue-empty-table ): return empty string only when no columns are defined.
    // Root cause: format_single_line_row unconditionally appends '\n' for zero-column
    // slices, producing bare newlines → "\n\n" for a table with zero columns.
    // Pitfall: guarding on rows.is_empty() is too aggressive — headers-only tables
    // should render header + separator as a useful "empty state" display.
    // IC-3: no columns → empty string; columns + no rows → header + separator only.
    if headers.is_empty() { return String::new(); }
    let mut output = String::with_capacity( INITIAL_CAPACITY );

    // Calculate natural column widths (uses .text for ANSI-free measurement)
    let column_widths = self.calculate_column_widths_for_rows( headers, rows );

    // Auto-wrap: compute budgets and pre-wrap flex cells at budget boundary
    let wrapped_rows_storage;
    let ( rows, column_widths ) = if self.should_auto_wrap( &column_widths )
    {
      let ( wr, cw ) = self.apply_auto_wrap( headers, rows, &column_widths );
      wrapped_rows_storage = wr;
      ( wrapped_rows_storage.as_slice(), cw )
    }
    else
    {
      ( rows, column_widths )
    };

    // Fold: first column where cumulative width exceeds terminal becomes fold_point.
    // Columns 0..fold_point render in the primary table; fold_point..N appear as continuation lines.
    // fold_point == column_widths.len() when all columns fit (no fold) or fold is disabled.
    let fold_point = if self.should_auto_fold()
    {
      self.determine_fold_point( &column_widths )
    }
    else
    {
      column_widths.len()
    };
    let primary_widths : &[ usize ] = &column_widths[ ..fold_point ];
    let primary_headers : &[ String ] = &headers[ ..fold_point ];

    // Top border (AsciiGrid / Unicode only)
    self.format_top_border_if_needed( &mut output, primary_widths );

    // Header row + separator (only primary columns; overflow headers appear as fold labels)
    self.format_header_with_color( &mut output, primary_headers, primary_widths );
    self.format_header_separator( &mut output, primary_widths );

    // Data rows — optionally alternating color, with inter-row separators
    for ( idx, row ) in rows.iter().enumerate()
    {
      let color = if self.config.alternating_rows_enabled()
      {
        if idx % 2 == 0 { self.config.row_color1_str() } else { self.config.row_color2_str() }
      }
      else
      {
        ""
      };

      let primary_row = &row[ ..fold_point ];

      if color.is_empty()
      {
        self.format_row_colored( &mut output, primary_row, primary_widths, false );
      }
      else
      {
        // Row-level color: strip cell colors (cell RESET would clear row background)
        let plain_cells : Vec< DecoratedText > = primary_row.iter()
          .map( | ct | DecoratedText::from( ct.text.as_str() ) )
          .collect();
        let mut row_buf = String::new();
        self.format_row( &mut row_buf, &plain_cells, primary_widths, false );
        // Fix(issue-multiline-color): wrap each output line with row color individually.
        let reset = self.config.color_reset_str();
        for line in row_buf.lines()
        {
          output.push_str( color );
          output.push_str( line );
          output.push_str( reset );
          output.push( '\n' );
        }
      }

      // Fold continuation lines — overflow columns rendered below primary row.
      // Emitted before sub-row detail lines per algorithm/005 rendering order.
      if fold_point < column_widths.len()
      {
        let overflow_hdrs : Vec< &str > = headers[ fold_point.. ]
          .iter()
          .map( String::as_str )
          .collect();
        let overflow_vals : Vec< &str > = row[ fold_point.. ]
          .iter()
          .map( | ct | ct.text.as_str() )
          .collect();
        let continuation = self.render_fold_continuation( &overflow_hdrs, &overflow_vals );
        output.push_str( &continuation );
      }

      // Sub-row detail line(s) — indent every line; apply per-line ANSI color when set.
      //
      // Fix(issue-ansi-color-per-line): iterate ct.text.lines() and wrap each line
      //   individually with color + line + RESET.
      // Root cause: calling ct.render() then .lines() would place the ANSI RESET
      //   at the very end of the whole block; any intermediate \n would cause terminal
      //   background-color bleed across line boundaries.
      // Pitfall: never call .render() and then .lines() on the result — always iterate
      //   .text.lines() and emit color/RESET per output line.
      if let Some( Some( ct ) ) = row_details.get( idx )
      {
        if !ct.text.is_empty()
        {
          let indent = self.config.detail_indent();
          for line in ct.text.lines()
          {
            output.push_str( indent );
            if let Some( ref color ) = ct.color
            {
              output.push_str( &DecoratedText::from( line.to_string() ).with_color( color.clone() ).render() );
            }
            else
            {
              output.push_str( line );
            }
            output.push( '\n' );
          }
        }
      }

      if idx < rows.len() - 1
      {
        self.format_inter_row_sep_if_needed( &mut output, primary_widths );
      }
    }

    // Bottom border (AsciiGrid / Unicode only)
    self.format_bottom_border_if_needed( &mut output, primary_widths );

    output
  }

  /// Format the header row, applying ANSI color per-line when header colorization is enabled.
  ///
  /// # Fix(issue-multiline-color)
  ///
  /// Iterates `.lines()` instead of single-pair wrap to avoid background-color bleed.
  /// Root cause: `trim_end_matches('\n')` + single wrap left intermediate `\n` chars inside
  /// the color sequence without RESET, causing bleed on each sub-line boundary.
  /// Pitfall: never use single color/RESET wrap on output that may contain intermediate newlines.
  fn format_header_with_color
  (
    &self,
    output : &mut String,
    primary_headers : &[ String ],
    primary_widths : &[ usize ],
  )
  {
    let header_cells : Vec< DecoratedText > = primary_headers.iter()
      .map( | h | DecoratedText::from( h.as_str() ) )
      .collect();
    let header_color = self.config.header_color_str();
    if self.config.colorize_header_enabled() && !header_color.is_empty()
    {
      let mut row_buf = String::new();
      self.format_row( &mut row_buf, &header_cells, primary_widths, true );
      let reset = self.config.color_reset_str();
      for line in row_buf.lines()
      {
        output.push_str( header_color );
        output.push_str( line );
        output.push_str( reset );
        output.push( '\n' );
      }
    }
    else
    {
      self.format_row( output, &header_cells, primary_widths, true );
    }
  }

  /// Format a single row (header or data)
  ///
  /// ## Border Consistency with Header Separator
  ///
  /// This function ensures row formatting is consistent with the header separator style.
  /// When `header_separator` is `AsciiGrid` or `Markdown`, the separator line includes
  /// leading and trailing pipes (`|...|`). To maintain visual consistency, data rows
  /// MUST also have leading and trailing pipes.
  ///
  /// **Bug History**: Previously, rows only had column separators between cells, while
  /// the header separator had border pipes. This created misaligned output:
  /// ```text
  ///  Crate   |Type   |Name        (no leading/trailing pipes)
  /// |---------|--------|---------  (has pipes!)
  ///  mindful |Binary |mindful     (no leading/trailing pipes)
  /// ```
  ///
  /// **Fix**: Check `header_separator` style and add border pipes when needed:
  /// ```text
  /// | Crate   |Type   |Name       |  (has pipes)
  /// |---------|--------|---------  |  (has pipes)
  /// | mindful |Binary |mindful    |  (has pipes)
  /// ```
  ///
  /// This ensures consistency across all table styles without breaking CSV/TSV formats.
  ///
  /// ## Multiline Cell Support
  ///
  /// When any cell contains `\n` characters, the row is rendered using multiline algorithm:
  /// - Each cell split into lines via `str::lines()`
  /// - Row height = max lines across all cells
  /// - Each line rendered separately with borders/separators
  /// - Shorter cells padded with empty strings
  ///
  /// **CSV/TSV Exception**: Multiline disabled per spec (line 1861) - newlines kept as literal "\n"
  /// Format a row using plain text only (no cell-level ANSI).
  ///
  /// Used for header rows and for data rows when row-level color is applied externally.
  fn format_row(
    &self,
    output : &mut String,
    cells : &[ DecoratedText ],
    column_widths : &[ usize ],
    _is_header : bool
  )
  {
    let is_csv_or_tsv = matches!(
      self.config.col_sep(),
      crate::config::ColumnSeparator::Character( ',' | '\t' )
    );
    let should_pad = !is_csv_or_tsv;

    // Extract plain text; CSV/TSV escapes newlines
    let cells_prepared : Vec< String > = cells
      .iter()
      .map( | ct | if is_csv_or_tsv { ct.text.replace( '\n', "\\n" ) } else { ct.text.clone() } )
      .collect();

    let has_multiline = !is_csv_or_tsv && cells_prepared.iter().any( | cell | cell.contains( '\n' ) );

    if has_multiline
    {
      self.format_multiline_row( output, &cells_prepared, column_widths );
    }
    else
    {
      self.format_single_line_row( output, &cells_prepared, column_widths, should_pad );
    }
  }

  /// Format a row preserving per-cell ANSI color codes.
  ///
  /// For single-line cells: calls `ct.render()` (color + text + RESET) before padding.
  /// For multi-line cells: per-line iteration — `color + line + RESET` per line — preventing
  ///   background-color bleed across `\n` boundaries.
  ///
  /// # Fix(issue-ansi-color-per-line)
  ///
  /// Root cause: calling `ct.render()` on a multi-line colored cell produces
  ///   `color + "line_a\nline_b" + RESET`. The `\n` appears inside the color sequence,
  ///   causing background-color bleed on every sub-line boundary.
  /// Pitfall: never call `.render()` and then `.lines()` on the result — always iterate
  ///   `.text.lines()` and emit `color + line + RESET` per output line.
  fn format_row_colored(
    &self,
    output : &mut String,
    cells : &[ DecoratedText ],
    column_widths : &[ usize ],
    _is_header : bool
  )
  {
    let is_csv_or_tsv = matches!(
      self.config.col_sep(),
      crate::config::ColumnSeparator::Character( ',' | '\t' )
    );

    if is_csv_or_tsv
    {
      // CSV/TSV: plain text, no ANSI
      let cells_plain : Vec< String > = cells
        .iter()
        .map( | ct | ct.text.replace( '\n', "\\n" ) )
        .collect();
      self.format_single_line_row( output, &cells_plain, column_widths, false );
      return;
    }

    // Check for multiline (based on text content, not rendered ANSI)
    let has_multiline = cells.iter().any( | ct | ct.text.contains( '\n' ) );

    if has_multiline
    {
      // Per-line color wrapping: emit color+line+RESET for each sub-line to prevent
      // background-color bleed across \n boundaries (Fix issue-ansi-color-per-line).
      let cells_colored : Vec< String > = cells.iter()
        .map( | ct |
        {
          if let Some( ref c ) = ct.color
          {
            ct.text.lines()
              .map( | line | DecoratedText::from( line.to_string() ).with_color( c.clone() ).render() )
              .collect::< Vec< _ > >()
              .join( "\n" )
          }
          else
          {
            ct.text.clone()
          }
        } )
        .collect();
      self.format_multiline_row( output, &cells_colored, column_widths );
    }
    else
    {
      // Single-line: render each cell with its color (ct.render() is safe — no internal \n)
      let cells_rendered : Vec< String > = cells.iter().map( DecoratedText::render ).collect();
      self.format_single_line_row( output, &cells_rendered, column_widths, true );
    }
  }

  /// Format a single-line row (no multiline cells)
  fn format_single_line_row(
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
      if self.config.has_outer_padding() && should_pad
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

      // Add padding after cell if outer_padding enabled (skip for CSV/TSV)
      if self.config.has_outer_padding() && should_pad
      {
        output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
      }

      // Add column separator (except after last column)
      if idx < cells.len() - 1
      {
        self.append_column_separator( output );
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
  fn format_multiline_row(
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
        if self.config.has_outer_padding()
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

        // Add padding after cell if outer_padding enabled
        if self.config.has_outer_padding()
        {
          output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
        }

        // Add column separator (except after last column)
        if col_idx < cells.len() - 1
        {
          self.append_column_separator( output );
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
  fn format_header_separator( &self, output : &mut String, column_widths : &[ usize ] )
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
          if self.config.has_outer_padding()
          {
            output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
          }

          output.push_str( &"-".repeat( width ) );

          if self.config.has_outer_padding()
          {
            output.push_str( &" ".repeat( self.config.cell_inner_padding() ) );
          }

          if idx < column_widths.len() - 1
          {
            self.append_column_separator( output );
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
        for ( _, &width ) in column_widths.iter().enumerate()
        {
          if self.config.has_outer_padding()
          {
            output.push_str( &"-".repeat( self.config.cell_inner_padding() ) );
          }

          output.push_str( &"-".repeat( width ) );

          if self.config.has_outer_padding()
          {
            output.push_str( &"-".repeat( self.config.cell_inner_padding() ) );
          }

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
      if self.config.has_outer_padding()
      {
        output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
      }
      output.push_str( &fill.to_string().repeat( width ) );
      if self.config.has_outer_padding()
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
      if self.config.has_outer_padding()
      {
        output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
      }
      output.push_str( &fill.to_string().repeat( width ) );
      if self.config.has_outer_padding()
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
  fn format_top_border_if_needed( &self, output : &mut String, widths : &[ usize ] )
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
  fn format_bottom_border_if_needed( &self, output : &mut String, widths : &[ usize ] )
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
  fn format_inter_row_sep_if_needed( &self, output : &mut String, widths : &[ usize ] )
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

mod auto_fit;

impl TableFormatter
{
  /// Calculate column widths based on content
  ///
  /// Uses `unicode_visual_len()` for display-width-aware, ANSI-stripping measurement.
  fn calculate_column_widths_for_rows(
    &self,
    headers : &[ String ],
    rows : &[ Vec< DecoratedText > ]
  )
  -> Vec< usize >
  {
    // Use provided widths if available
    if !self.config.col_widths_override().is_empty()
    {
      return self.config.col_widths_override().to_vec();
    }

    // Auto-calculate based on content
    let mut widths = vec![ 0; headers.len() ];

    // Consider header widths (unicode display-width, ANSI-stripped)
    //
    // Fix(issue-multiline-width): use max single-line width, not total string width.
    // Root cause: `unicode_visual_len(cell)` on a multiline string counts `\n` as
    //   1 display column (via `ch.width().unwrap_or(1)`), producing a column that is
    //   wider than its widest single line (e.g., "Line1\nLine2" → 11 instead of 5).
    // Pitfall: never call `unicode_visual_len` on strings that may contain `\n`;
    //   always split by lines and take the per-line maximum.
    for ( idx, header ) in headers.iter().enumerate()
    {
      let header_width = header
        .lines()
        .map( unicode_visual_len )
        .max()
        .unwrap_or( 0 );
      widths[ idx ] = header_width;
    }

    // Consider row widths (unicode display-width, ANSI-stripped; use .text to skip ANSI bytes)
    for row in rows
    {
      for ( idx, cell ) in row.iter().enumerate()
      {
        if idx < widths.len()
        {
          let cell_width = cell
            .text
            .lines()
            .map( unicode_visual_len )
            .max()
            .unwrap_or( 0 );
          widths[ idx ] = widths[ idx ].max( cell_width );
        }
      }
    }

    // Cap column widths at max_column_width if configured
    // This ensures truncated columns don't get padded back to original size
    if let Some( max_width ) = self.config.max_col_width()
    {
      for width in &mut widths
      {
        *width = (*width).min( max_width );
      }
    }

    // Enforce min_column_width floor (applied after max cap so min can override max)
    // Guard: default min is 0, which is a no-op; skip the loop in that case
    let min = self.config.min_col_width();
    if min > 0
    {
      for width in &mut widths
      {
        *width = ( *width ).max( min );
      }
    }

    widths
  }

  /// Write formatted table directly to a writer
  ///
  /// # Errors
  ///
  /// Returns an error if writing to the provided writer fails
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::{ RowBuilder, TableFormatter };
  /// use std::io::Cursor;
  ///
  /// let view = RowBuilder::new( vec![ "Name".into() ] )
  ///   .add_row( vec![ "Alice".into() ] )
  ///   .build_view();
  ///
  /// let formatter = TableFormatter::new();
  /// let mut buffer = Cursor::new( Vec::new() );
  /// formatter.write_to( &view, &mut buffer ).unwrap();
  ///
  /// let output = String::from_utf8( buffer.into_inner() ).unwrap();
  /// assert!( output.contains( "Alice" ) );
  /// ```
  pub fn write_to< W : std::io::Write >(
    &self,
    data : &crate::TableView,
    writer : &mut W
  )
  -> std::io::Result< () >
  {
    let output = super::Format::format( self, data ).unwrap_or_default();
    writer.write_all( output.as_bytes() )
  }
}

// Implement unified Format trait for TableView
impl super::Format for TableFormatter
{
  fn format( &self, data : &crate::TableView ) -> Result< String, super::FormatError >
  {
    Ok( self.format_internal( &data.metadata.column_names, &data.rows, &data.row_details ) )
  }
}
