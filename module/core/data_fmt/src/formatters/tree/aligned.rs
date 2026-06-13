//! Column-aligned tree rendering for `TreeFormatter`
//!
//! Two-pass algorithm: (1) traverse the tree to find maximum column widths,
//! (2) render with alignment padding so all columns line up across rows.

use crate::{ TreeNode, ColumnData, ansi_str::{ visual_len, pad_to_width } };

/// Output buffer capacity hint
const INITIAL_CAPACITY : usize = 1024;

impl super::TreeFormatter
{
  /// Format a tree with all columns width-aligned
  ///
  /// Two-pass algorithm: first traverses the entire tree to find the maximum
  /// display width per column, then formats each node with padding so columns
  /// align across rows.
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::{ TreeBuilder, TreeFormatter, ColumnData };
  ///
  /// let root = TreeBuilder::new( "root" ).build();
  /// let formatter = TreeFormatter::new();
  /// let output = formatter.format_aligned( &root );
  /// println!( "{}", output );
  /// ```
  pub fn format_aligned( &self, tree : &TreeNode< ColumnData > ) -> String
  {
    let mut output = String::with_capacity( INITIAL_CAPACITY );

    // Handle empty tree
    if tree.children.is_empty() && tree.data.is_none()
    {
      output.push_str( &tree.name );
      output.push( '\n' );
      return output;
    }

    // Phase 1: Calculate column widths across entire tree
    let column_widths = self.calculate_column_widths( tree );

    // Phase 2: Format tree with alignment
    if self.config.show_root
    {
      // Show root node
      output.push_str( &tree.name );
      output.push( '\n' );

      // Format children
      for ( i, child ) in tree.children.iter().enumerate()
      {
        let is_last = i == tree.children.len() - 1;
        self.format_aligned_node( child, &mut output, &column_widths, "", "", is_last, 1 );
      }
    }
    else
    {
      // Skip root, format children directly
      for ( i, child ) in tree.children.iter().enumerate()
      {
        let is_last = i == tree.children.len() - 1;
        self.format_aligned_node( child, &mut output, &column_widths, "", "", is_last, 0 );
      }
    }

    output
  }

  /// Calculate maximum width needed for each column across entire tree
  fn calculate_column_widths( &self, tree : &TreeNode< ColumnData > ) -> Vec< usize >
  {
    let mut max_widths = Vec::new();
    let start_depth = usize::from( self.config.show_root );
    self.traverse_for_widths( tree, &mut max_widths, start_depth );

    // Apply minimum column width
    if self.config.min_column_width > 0
    {
      for width in &mut max_widths
      {
        if *width < self.config.min_column_width
        {
          *width = self.config.min_column_width;
        }
      }
    }

    max_widths
  }

  /// Recursively traverse tree to find maximum column widths
  fn traverse_for_widths(
    &self,
    node : &TreeNode< ColumnData >,
    max_widths : &mut Vec< usize >,
    depth : usize
  )
  {
    // Check max_depth constraint
    if let Some( max_depth ) = self.config.max_depth
    {
      if depth > max_depth
      {
        return;
      }
    }

    if let Some( data ) = &node.data
    {
      for ( i, col ) in data.columns.iter().enumerate()
      {
        // First column includes tree symbols and indentation
        let col_rendered = col.render();
        let width = if i == 0
        {
          let prefix_len = self.calculate_prefix_len( depth );
          visual_len( &col_rendered ) + prefix_len
        }
        else
        {
          visual_len( &col_rendered )
        };

        // Update max width for this column
        if i >= max_widths.len()
        {
          max_widths.push( width );
        }
        else if width > max_widths[ i ]
        {
          max_widths[ i ] = width;
        }
      }
    }

    // Recurse to children
    for child in &node.children
    {
      self.traverse_for_widths( child, max_widths, depth + 1 );
    }
  }

  /// Calculate length of tree prefix (symbols + indentation) for a given depth
  fn calculate_prefix_len( &self, depth : usize ) -> usize
  {
    if depth == 0
    {
      return 0;
    }

    // Each level adds:
    // - indent_size spaces (4 by default)
    // - branch connector length (├── or └──) + 1 space after connector
    let branch_connector_len = visual_len( self.symbols.branch ) + 1; // +1 for space

    ( depth - 1 ) * self.config.indent_size + branch_connector_len
  }

  /// Format a single node with column alignment
  // Each parameter is a distinct rendering input (node, buffer, column_widths, prefix,
  // child_prefix, is_last, depth); no natural grouping exists without adding allocation
  // overhead on this hot recursive path.
  #[ allow( clippy::too_many_arguments ) ]
  pub( super ) fn format_aligned_node(
    &self,
    node : &TreeNode< ColumnData >,
    output : &mut String,
    column_widths : &[ usize ],
    prefix : &str,
    _child_prefix : &str,
    is_last : bool,
    depth : usize
  )
  {
    // Check max_depth constraint
    if let Some( max_depth ) = self.config.max_depth
    {
      if depth > max_depth
      {
        return;
      }
    }

    // Add prefix (indentation from parent levels)
    output.push_str( prefix );

    // Add tree connector symbol
    let connector = if is_last
    {
      self.symbols.last_branch
    }
    else
    {
      self.symbols.branch
    };
    output.push_str( connector );
    output.push( ' ' ); // Space after connector

    // Format columns with alignment
    if let Some( data ) = &node.data
    {
      for ( i, col ) in data.columns.iter().enumerate()
      {
        let col_rendered = col.render();
        if i == 0
        {
          // First column - just add the value
          output.push_str( &col_rendered );

          // Calculate padding needed for first column
          if !column_widths.is_empty()
          {
            let current_len = visual_len( prefix ) + visual_len( connector ) + 1 + visual_len( &col_rendered ); // +1 for space after connector
            let target_len = column_widths[ 0 ];
            if current_len < target_len
            {
              output.push_str( &" ".repeat( target_len - current_len ) );
            }
          }
        }
        else
        {
          // Subsequent columns - add separator then padded value
          output.push_str( &self.config.column_separator );

          if i < column_widths.len()
          {
            let padded = pad_to_width( &col_rendered, column_widths[ i ], false ); // Left-align
            output.push_str( &padded );
          }
          else
          {
            // No width constraint for this column
            output.push_str( &col_rendered );
          }
        }
      }
    }
    else
    {
      // Node has no data, just show the name
      output.push_str( &node.name );
    }

    output.push( '\n' );

    // Format children recursively
    if !node.children.is_empty() && self.config.show_branches
    {
      let new_prefix = format!(
        "{}{}",
        prefix,
        if is_last
        {
          " ".repeat( self.config.indent_size )
        }
        else
        {
          format!( "{}{}", self.symbols.vertical, " ".repeat( self.config.indent_size - visual_len( self.symbols.vertical ) ) )
        }
      );

      for ( i, child ) in node.children.iter().enumerate()
      {
        let is_last_child = i == node.children.len() - 1;
        self.format_aligned_node( child, output, column_widths, &new_prefix, &new_prefix, is_last_child, depth + 1 );
      }
    }
  }

}
