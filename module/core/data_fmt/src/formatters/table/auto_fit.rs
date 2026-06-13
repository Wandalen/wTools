//! Auto-fit and fold helpers for `TableFormatter`

use super::TableFormatter;
use crate::ansi_str::{ unicode_visual_len };
use crate::wrap::{ WrapConfig, WrapFormatter };
use crate::config::{ ColumnFlex, FoldStyle };
use color_tools::DecoratedText;

impl TableFormatter
{
  // --- Auto-wrap helpers ---

  /// Resolve effective terminal width from config or fallback
  pub( super ) fn resolve_terminal_width( &self ) -> usize
  {
    // Tier 0: explicit override via `terminal_width(Some(w))` — takes priority over all
    if let Some( w ) = self.config.term_width()
    {
      return if w == 0 { 1 } else { w };
    }
    // Tier 1: $COLUMNS environment variable — unset or unparseable falls through
    if let Ok( val ) = std::env::var( "COLUMNS" )
    {
      if let Ok( n ) = val.trim().parse::< usize >()
      {
        if n > 0 { return n; }
      }
    }
    // Tier 2: terminal_size crate (runtime detection) — feature-gated
    #[ cfg( feature = "terminal_size" ) ]
    {
      if let Some( ( terminal_size::Width( w ), _ ) ) = terminal_size::terminal_size()
      {
        return w as usize;
      }
    }
    // Tier 3: hardcoded fallback
    120
  }

  /// Visual width of one column separator
  pub( super ) fn separator_visual_width( &self ) -> usize
  {
    match self.config.col_sep()
    {
      crate::config::ColumnSeparator::Spaces( n ) => *n,
      crate::config::ColumnSeparator::Character( _ ) => 1,
      crate::config::ColumnSeparator::String( s ) => unicode_visual_len( s ),
    }
  }

  /// Whether the current style uses border pipe characters at row edges
  pub( super ) fn needs_border_pipes( &self ) -> bool
  {
    use crate::config::HeaderSeparatorVariant;
    matches!(
      self.config.header_sep_variant(),
      HeaderSeparatorVariant::AsciiGrid | HeaderSeparatorVariant::Markdown | HeaderSeparatorVariant::Unicode
    )
  }

  /// Compute the total display width of a row given column widths
  pub( super ) fn compute_total_row_width( &self, column_widths : &[ usize ] ) -> usize
  {
    if column_widths.is_empty() { return 0; }
    let content_width : usize = column_widths.iter().sum();
    let sep_count = column_widths.len() - 1;
    let sep_total = self.separator_visual_width() * sep_count;
    let outer = if self.config.has_outer_padding()
    {
      self.config.cell_inner_padding() * 2
    }
    else
    {
      0
    };
    let border = if self.needs_border_pipes() { 2 } else { 0 };
    content_width + sep_total + outer + border
  }

  /// Determine if auto-wrapping should be applied
  pub( super ) fn should_auto_wrap( &self, column_widths : &[ usize ] ) -> bool
  {
    if !self.config.is_auto_wrap() { return false; }
    if !self.config.col_widths_override().is_empty() { return false; }
    if column_widths.is_empty() { return false; }
    if self.config.is_csv_or_tsv() { return false; }
    let total = self.compute_total_row_width( column_widths );
    let terminal = self.resolve_terminal_width();
    total > terminal
  }

  /// Determine if column folding should be applied.
  ///
  /// Mirrors `should_auto_wrap` guard logic: fold is a form of terminal adaptation
  /// and is disabled by the same conditions that disable wrapping.
  pub( super ) fn should_auto_fold( &self ) -> bool
  {
    if !self.config.is_auto_fold() { return false; }
    if !self.config.is_auto_wrap() { return false; }
    if !self.config.col_widths_override().is_empty() { return false; }
    !self.config.is_csv_or_tsv()
  }

  /// Classify columns as Fixed or Flex using explicit config or auto-heuristic
  pub( super ) fn classify_columns( &self, column_widths : &[ usize ] ) -> Vec< ColumnFlex >
  {
    let explicit = self.config.col_flex();
    if !explicit.is_empty()
    {
      let mut result = explicit.to_vec();
      result.resize( column_widths.len(), ColumnFlex::Flex );
      return result;
    }
    column_widths
      .iter()
      .map( | &w | if w <= 12 { ColumnFlex::Fixed } else { ColumnFlex::Flex } )
      .collect()
  }

  /// Compute per-column budget widths based on terminal width and flex classification
  pub( super ) fn compute_column_budgets(
    &self,
    column_widths : &[ usize ],
    flex_map : &[ ColumnFlex ],
  ) -> Vec< usize >
  {
    let terminal = self.resolve_terminal_width();
    let sep_count = if column_widths.len() > 1 { column_widths.len() - 1 } else { 0 };
    let overhead = self.separator_visual_width() * sep_count
      + if self.config.has_outer_padding() { self.config.cell_inner_padding() * 2 } else { 0 }
      + if self.needs_border_pipes() { 2 } else { 0 };

    let fixed_total : usize = column_widths
      .iter()
      .zip( flex_map.iter() )
      .filter( | ( _, f ) | **f == ColumnFlex::Fixed )
      .map( | ( w, _ ) | *w )
      .sum();

    let flex_count = flex_map.iter().filter( | f | **f == ColumnFlex::Flex ).count();
    if flex_count == 0
    {
      return column_widths.to_vec();
    }

    let budget = terminal.saturating_sub( fixed_total + overhead );
    let base = budget / flex_count;
    let remainder = budget % flex_count;
    let min = self.config.min_col_width();

    let mut budgets = Vec::with_capacity( column_widths.len() );
    let mut flex_idx = 0;
    for ( i, &w ) in column_widths.iter().enumerate()
    {
      if flex_map[ i ] == ColumnFlex::Fixed
      {
        budgets.push( w );
      }
      else
      {
        let extra = usize::from( flex_idx < remainder );
        let b = ( base + extra ).max( if min > 0 { min } else { 1 } );
        budgets.push( b.min( w ) );
        flex_idx += 1;
      }
    }
    budgets
  }

  /// Pre-wrap flex-column cells at their budget widths, returning modified rows
  /// and recalculated column widths
  pub( super ) fn apply_auto_wrap(
    &self,
    headers : &[ String ],
    rows : &[ Vec< DecoratedText > ],
    column_widths : &[ usize ],
  ) -> ( Vec< Vec< DecoratedText > >, Vec< usize > )
  {
    let flex_map = self.classify_columns( column_widths );
    let budgets = self.compute_column_budgets( column_widths, &flex_map );

    let mut wrapped_rows : Vec< Vec< DecoratedText > > = rows.to_vec();

    for ( col_idx, ( &flex, &budget ) ) in flex_map.iter().zip( budgets.iter() ).enumerate()
    {
      if flex != ColumnFlex::Flex { continue; }
      if budget >= column_widths[ col_idx ] { continue; }

      let wrapper = WrapFormatter::with_config(
        WrapConfig::new().width( budget )
      );

      for row in &mut wrapped_rows
      {
        if col_idx < row.len()
        {
          let cell_width = row[ col_idx ]
            .text
            .lines()
            .map( unicode_visual_len )
            .max()
            .unwrap_or( 0 );
          if cell_width > budget
          {
            let wrapped_text = wrapper.wrap_joined( &row[ col_idx ].text );
            let original_color = row[ col_idx ].color.clone();
            let mut new_cell = DecoratedText::from( wrapped_text );
            if let Some( color ) = original_color
            {
              new_cell = new_cell.with_color( color );
            }
            row[ col_idx ] = new_cell;
          }
        }
      }
    }

    let new_widths = self.calculate_column_widths_for_rows( headers, &wrapped_rows );
    ( wrapped_rows, new_widths )
  }

  // --- Fold helpers ---

  /// Find the first column index where including it would cause the row to exceed the terminal.
  ///
  /// Returns `column_widths.len()` when all columns fit (no fold needed).
  /// Uses the same overhead accounting as `compute_total_row_width` so fold detection
  /// is consistent with the auto-wrap trigger condition.
  pub( super ) fn determine_fold_point( &self, column_widths : &[ usize ] ) -> usize
  {
    let terminal = self.resolve_terminal_width();
    let sep_width = self.separator_visual_width();
    let outer = if self.config.has_outer_padding()
    {
      self.config.cell_inner_padding() * 2
    }
    else
    {
      0
    };
    let border = if self.needs_border_pipes() { 2 } else { 0 };
    let overhead = outer + border;

    let mut content_so_far = 0usize;
    for ( i, &w ) in column_widths.iter().enumerate()
    {
      content_so_far += w;
      let sep_total = i.saturating_mul( sep_width );
      if content_so_far + sep_total + overhead > terminal
      {
        // Fix(BUG-007): clamp to 1 so the first column always stays
        // in the primary table (Invariant 1: header row must never be empty).
        // Root cause: when col[0] alone exceeded terminal, fold_point=0 produced an
        //   empty primary header row with no visible column names.
        // Pitfall: never return 0 here — always return at least 1 so the formatter
        //   emits at least one column in the primary table row.
        return i.max( 1 );
      }
    }
    column_widths.len()
  }

  /// Render continuation lines for overflow columns below a data row.
  ///
  /// Supports three `FoldStyle` variants:
  /// - `Labeled` (default): all overflow columns joined as "Col: val  Col2: val2" on one line,
  ///   wrapped at terminal width when the joined line exceeds the available budget.
  /// - `Stacked`: each overflow column on its own indented labeled line, wrapped per-column.
  /// - `Bare`: all overflow values joined on one line without labels.
  pub( super ) fn render_fold_continuation(
    &self,
    overflow_headers : &[ &str ],
    overflow_values : &[ &str ],
  ) -> String
  {
    let indent = self.config.fold_indent_val();
    let terminal = self.resolve_terminal_width();
    let indent_width = unicode_visual_len( indent );
    let available = terminal.saturating_sub( indent_width );

    let mut lines : Vec< String > = Vec::new();

    match self.config.fold_style_val()
    {
      FoldStyle::Labeled =>
      {
        // All overflow columns joined as "Col: val  Col2: val2" on one continuation line.
        // Wrapped at available width when the joined content is too long.
        let pairs : Vec< String > = overflow_headers.iter()
          .zip( overflow_values.iter() )
          .filter( | ( _, v ) | !v.is_empty() )
          .map( | ( h, v ) | format!( "{}: {}", h, v.trim_end() ) )
          .collect();
        if !pairs.is_empty()
        {
          let joined = pairs.join( "  " );
          let full_line = format!( "{indent}{joined}" );
          if unicode_visual_len( &full_line ) > terminal && available > 0
          {
            let fmt = WrapFormatter::with_config( WrapConfig::new().width( available ) );
            let output_wrapped = fmt.wrap_joined( &joined );
            for line in output_wrapped.lines()
            {
              lines.push( format!( "{indent}{line}" ) );
            }
          }
          else
          {
            lines.push( full_line );
          }
        }
      }
      FoldStyle::Stacked =>
      {
        // Each overflow column on its own indented labeled line.
        for ( header, value ) in overflow_headers.iter().zip( overflow_values.iter() )
        {
          if value.is_empty() { continue; }
          let label = format!( "{header}: " );
          let label_width = unicode_visual_len( &label );
          let value_str = value.trim_end();
          let full_line = format!( "{indent}{label}{value_str}" );
          if unicode_visual_len( &full_line ) > terminal && available > label_width
          {
            let value_available = available.saturating_sub( label_width );
            let fmt = WrapFormatter::with_config( WrapConfig::new().width( value_available ) );
            let output_wrapped = fmt.wrap_joined( value_str );
            let mut it = output_wrapped.lines();
            if let Some( first ) = it.next()
            {
              lines.push( format!( "{indent}{label}{first}" ) );
              for rest in it
              {
                lines.push( format!( "{indent}{rest}" ) );
              }
            }
          }
          else
          {
            lines.push( full_line );
          }
        }
      }
      FoldStyle::Bare =>
      {
        // All overflow values on one line without column labels.
        // Fix(BUG-006): wrap when the joined line exceeds terminal,
        //   mirroring the Labeled and Stacked wrapping guards.
        // Root cause: the Bare branch emitted joined values unconditionally without
        //   checking unicode_visual_len vs terminal, unlike the other two styles.
        // Pitfall: Bare has no label prefix, so wrapped continuation lines carry only
        //   value fragments — ensure tests verify wrapping produces ≤ terminal width.
        let values : Vec< &str > = overflow_values.iter()
          .copied()
          .filter( | v | !v.is_empty() )
          .collect();
        if !values.is_empty()
        {
          let vals_joined = values.join( "  " );
          let full_line = format!( "{indent}{vals_joined}" );
          if unicode_visual_len( &full_line ) > terminal && available > 0
          {
            let fmt = WrapFormatter::with_config( WrapConfig::new().width( available ) );
            let output_wrapped = fmt.wrap_joined( &vals_joined );
            for line in output_wrapped.lines()
            {
              lines.push( format!( "{indent}{line}" ) );
            }
          }
          else
          {
            lines.push( full_line );
          }
        }
      }
    }

    if lines.is_empty()
    {
      String::new()
    }
    else
    {
      let mut result = lines.join( "\n" );
      result.push( '\n' );
      result
    }
  }
}
