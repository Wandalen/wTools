//! Header, row, and caption rendering for `TableFormatter`
//!
//! Extracted to keep `mod.rs` under the 500-line ceiling.
//! Contains the four dispatch functions that sit between the high-level
//! `format_internal` loop and the low-level primitives in `rendering.rs`.

use super::TableFormatter;
use color_tools::DecoratedText;

impl TableFormatter
{
  /// Format the header row, applying ANSI color wrapping line-by-line if enabled.
  ///
  /// # Fix(BUG-009)
  ///
  /// Iterates `.lines()` instead of single-pair wrap to avoid background-color bleed.
  /// Root cause: `trim_end_matches('\n')` + single wrap left intermediate `\n` chars inside
  /// the color sequence without RESET, causing bleed on each sub-line boundary.
  /// Pitfall: never use single color/RESET wrap on output that may contain intermediate newlines.
  pub( super ) fn format_header_with_color
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
  pub( super ) fn format_row(
    &self,
    output : &mut String,
    cells : &[ DecoratedText ],
    column_widths : &[ usize ],
    _is_header : bool
  )
  {
    let is_csv_or_tsv = self.config.is_csv_or_tsv();
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
  /// # Fix(BUG-010)
  ///
  /// Root cause: calling `ct.render()` on a multi-line colored cell produces
  ///   `color + "line_a\nline_b" + RESET`. The `\n` appears inside the color sequence,
  ///   causing background-color bleed on every sub-line boundary.
  /// Pitfall: never call `.render()` and then `.lines()` on the result — always iterate
  ///   `.text.lines()` and emit `color + line + RESET` per output line.
  pub( super ) fn format_row_colored(
    &self,
    output : &mut String,
    cells : &[ DecoratedText ],
    column_widths : &[ usize ],
    _is_header : bool
  )
  {
    let is_csv_or_tsv = self.config.is_csv_or_tsv();

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
      // background-color bleed across \n boundaries (Fix BUG-010).
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

  /// Render the caption titled rule into `output`, or return early if no caption is set.
  ///
  /// Format: `─── content ──────...` filling `resolve_terminal_width()` columns.
  /// Uses `.chars().count()` (not `.len()`) because `·` and `─` are multi-byte in UTF-8.
  pub( super ) fn render_caption_if_present( &self, output : &mut String )
  {
    let Some( caption ) = self.config.caption_ref() else { return };
    let content = caption.content_str();
    let tw = self.resolve_terminal_width();
    let lead_width = crate::config::CAPTION_LEAD_WIDTH;
    let rule_char  = crate::config::CAPTION_RULE_CHAR;
    // used = lead_width + 1 (space) + content chars + 1 (trailing space)
    let used = lead_width + 1 + content.chars().count() + 1;
    let trail = tw.saturating_sub( used );
    let lead  : String = std::iter::repeat_n( rule_char, lead_width ).collect();
    let trail_str : String = std::iter::repeat_n( rule_char, trail ).collect();
    output.push_str( &lead );
    output.push( ' ' );
    output.push_str( &content );
    output.push( ' ' );
    output.push_str( &trail_str );
    output.push( '\n' );
  }

}
