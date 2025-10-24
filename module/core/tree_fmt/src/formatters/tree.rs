//! `TreeFormatter` for hierarchical tree display with 2 formatting methods
//!
//! ## Available Methods
//!
//! ### Hierarchical Tree
//! ```
//! # use tree_fmt::{ TreeBuilder, TreeFormatter };
//! # let tree = TreeBuilder::new("project")
//! #   .insert(&["src", "main.rs"], 1024)
//! #   .insert(&["src", "lib.rs"], 2048)
//! #   .build();
//! let formatter = TreeFormatter::new();
//! let output = formatter.format(&tree, |bytes| format!("({} bytes)", bytes));
//! // project
//! // ├── src
//! // │   ├── main.rs (1024 bytes)
//! // │   └── lib.rs (2048 bytes)
//! ```
//!
//! ### Column-Aligned Tree
//! ```
//! # use tree_fmt::{ TreeNode, ColumnData, TreeFormatter };
//! # let mut root = TreeNode::new("root".to_string(), None);
//! # root.children.push(TreeNode::new("api_ollama".to_string(),
//! #   Some(ColumnData::new(vec!["api_ollama".to_string(), "v0.1.0".to_string(), "(api/ollama)".to_string()]))));
//! # root.children.push(TreeNode::new("as_curl".to_string(),
//! #   Some(ColumnData::new(vec!["as_curl".to_string(), "v0.1.0".to_string(), "(module/as_curl)".to_string()]))));
//! let formatter = TreeFormatter::new();
//! let output = formatter.format_aligned(&root);
//! // ├── api_ollama  v0.1.0  (api/ollama)
//! // └── as_curl     v0.1.0  (module/as_curl)
//! ```
//!
//! ## Column Alignment Algorithm Design (`format_aligned` method)
//!
//! ### Problem: Ragged Column Output
//!
//! Tree symbols (`├──`, `│`, `└──`) vary by depth, causing ragged multi-column output:
//!
//! ```text
//! ├── short_name  v1.0  (path1)
//! │   ├── dep1  v2.0  (path/to/dep1)
//! │   └── dep2  v1.5  (path/to/dep2)
//! └── very_long_name  v0.1  (path2)
//! ```
//!
//! Notice how the version and path columns don't align vertically, making the tree
//! hard to scan. The problem is that deeper nodes have longer prefixes (more `│` characters).
//!
//! ### Solution: Two-Pass Algorithm
//!
//! **Pass 1: Width Calculation**
//! - Traverse entire tree with DFS
//! - For each column index, track maximum width needed
//! - **Column 0 special handling**: Include prefix length in width calculation
//!   - Prefix length = `(depth - 1) * indent_size + branch_connector_len + 1`
//!   - Example: Depth 2 node has prefix "│   ├── " (8 chars)
//! - Columns 1+ use raw `visual_len()` of content
//!
//! **Pass 2: Formatting**
//! - Render each node with calculated column widths
//! - Pad each column to its maximum width for alignment
//! - Use `visual_len()` and `pad_to_width()` for ANSI-aware padding
//!
//! ### Prefix Length Calculation
//!
//! The prefix length formula accounts for tree structure:
//!
//! ```rust,ignore
//! fn calculate_prefix_len(depth: usize, indent_size: usize, symbols: &TreeSymbols) -> usize {
//!   if depth == 0 { return 0; }
//!   let branch_connector_len = visual_len(symbols.branch) + 1; // "├── " or "└── "
//!   (depth - 1) * indent_size + branch_connector_len
//! }
//! ```
//!
//! Examples with default config (`indent_size=4`):
//! - Depth 0: 0 chars
//! - Depth 1: 0*4 + 4 = 4 chars (`├── `)
//! - Depth 2: 1*4 + 4 = 8 chars (`│   ├── `)
//! - Depth 3: 2*4 + 4 = 12 chars (`│   │   ├── `)
//!
//! ### Edge Cases Handled
//!
//! 1. **Empty trees**: Return root name only
//! 2. **Single column nodes**: No alignment needed, fast path
//! 3. **Mixed column counts**: Use max column count across all nodes
//! 4. **ANSI color codes**: Handled by `visual_len()` throughout
//! 5. **Unicode characters**: Multi-byte chars counted correctly
//!
//! ### Why This Works
//!
//! By including the tree prefix length in column 0 width calculation during pass 1,
//! we ensure all column 1+ start positions align vertically regardless of tree depth.
//!
//! **Result (with alignment)**:
//! ```text
//! ├── short_name       v1.0  (path1)
//! │   ├── dep1         v2.0  (path/to/dep1)
//! │   └── dep2         v1.5  (path/to/dep2)
//! └── very_long_name   v0.1  (path2)
//! ```
//!
//! ### Historical Context
//!
//! - **v0.1.0**: Only basic tree formatting, no column alignment
//! - **v0.2.0**: Added `AlignedTreeFormatter` with two-pass algorithm
//! - **v0.4.0**: Merged into `TreeFormatter` as `format_aligned()` method
//!
//! See `tests/reproduce_alignment_problem.rs` for visual demonstration comparing
//! aligned vs unaligned output.

use crate::{ TreeNode, TreeConfig, TreeSymbols, ColumnData, helpers::{ visual_len, pad_to_width } };

/// Initial string capacity for tree output
const INITIAL_CAPACITY : usize = 1024;


/// Formatter for rendering tree structures as strings
///
/// Provides configurable tree rendering with custom symbols, indentation,
/// and display options.
#[ derive( Debug ) ]
pub struct TreeFormatter
{
  config : TreeConfig,
  symbols : TreeSymbols,
}

impl TreeFormatter
{
  /// Create a new tree formatter with default formatter parameters
  pub fn new() -> Self
  {
    Self
    {
      config : TreeConfig::default(),
      symbols : TreeSymbols::default(),
    }
  }

  /// Create a tree formatter with custom formatter parameters
  pub fn with_config( config : TreeConfig ) -> Self
  {
    Self
    {
      config,
      symbols : TreeSymbols::default(),
    }
  }

  /// Create a tree formatter with custom symbols
  pub fn with_symbols( symbols : TreeSymbols ) -> Self
  {
    Self
    {
      config : TreeConfig::default(),
      symbols,
    }
  }

  /// Format tree with custom item renderer
  ///
  /// # Arguments
  ///
  /// * `tree` - Root node of the tree
  /// * `render_item` - Function to render item data as string
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ TreeBuilder, TreeFormatter };
  ///
  /// let tree = TreeBuilder::new( "root" ).insert( &[ "file.txt" ], 100 ).build();
  /// let formatter = TreeFormatter::new();
  /// let output = formatter.format( &tree, | size | format!( "{} bytes", size ) );
  /// ```
  pub fn format< T, F >( &self, tree : &TreeNode< T >, render_item : F ) -> String
  where
    F : Fn( &T ) -> String,
  {
    let mut output = String::with_capacity( INITIAL_CAPACITY );

    // If the root has data and no children, format it as a leaf node
    if tree.children.is_empty() && tree.data.is_some()
    {
      output.push_str( &tree.name );
      if let Some( ref data ) = tree.data
      {
        let rendered = render_item( data );
        if !rendered.is_empty()
        {
          output.push( ' ' );
          output.push_str( &rendered );
        }
      }
      output.push( '\n' );
      return output;
    }

    // If the root has no data and no children, always show root name
    // (even if show_root is false) to avoid empty output
    if tree.children.is_empty()
    {
      output.push_str( &tree.name );
      output.push( '\n' );
      return output;
    }

    // Standard tree formatting
    if self.config.show_root
    {
      output.push_str( &tree.name );
      output.push( '\n' );
    }

    self.format_node( tree, &mut output, "", true, 0, &render_item );

    output
  }

  /// Format tree with column-aligned data
  ///
  /// Renders hierarchical tree structures where each node can have multiple
  /// columns of data that are vertically aligned across all nodes.
  ///
  /// # Arguments
  ///
  /// * `tree` - Root node of the tree with `ColumnData`
  ///
  /// # Returns
  ///
  /// Formatted string with aligned columns and tree structure
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ TreeNode, ColumnData, TreeFormatter };
  ///
  /// let mut root = TreeNode::new( "root".to_string(), None );
  ///
  /// root.children.push( TreeNode::new(
  ///   "child1".to_string(),
  ///   Some( ColumnData::new( vec![
  ///     "name1".to_string(),
  ///     "v1.0".to_string(),
  ///     "(path/to/1)".to_string()
  ///   ]))
  /// ));
  ///
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
        let width = if i == 0
        {
          let prefix_len = self.calculate_prefix_len( depth );
          visual_len( col ) + prefix_len
        }
        else
        {
          visual_len( col )
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
  #[ allow( clippy::too_many_arguments ) ]
  fn format_aligned_node(
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
        if i == 0
        {
          // First column - just add the value
          output.push_str( col );

          // Calculate padding needed for first column
          if !column_widths.is_empty()
          {
            let current_len = visual_len( prefix ) + visual_len( connector ) + 1 + visual_len( col ); // +1 for space after connector
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
            let padded = pad_to_width( col, column_widths[ i ], false ); // Left-align
            output.push_str( &padded );
          }
          else
          {
            // No width constraint for this column
            output.push_str( col );
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

  /// Format a tree node recursively
  fn format_node< T, F >(
    &self,
    node : &TreeNode< T >,
    output : &mut String,
    prefix : &str,
    is_last : bool,
    depth : usize,
    render_item : &F,
  )
  where
    F : Fn( &T ) -> String,
  {
    // Check max depth
    if let Some( max_depth ) = self.config.max_depth
    {
      if depth >= max_depth
      {
        return;
      }
    }

    // Format children
    for ( idx, child ) in node.children.iter().enumerate()
    {
      let is_last_child = idx == node.children.len() - 1;

      // Determine symbol and new prefix
      let ( symbol, new_prefix ) = if self.config.show_branches
      {
        let sym = if is_last_child
        {
          self.symbols.last_branch
        }
        else
        {
          self.symbols.branch
        };

        let continuation = if node.name == "." || !self.config.show_root
        {
          // Root level - no prefix
          ""
        }
        else if is_last
        {
          self.symbols.space
        }
        else
        {
          self.symbols.vertical
        };

        let np = if node.name == "." || !self.config.show_root
        {
          prefix.to_string()
        }
        else
        {
          format!( "{prefix}{continuation}" )
        };

        ( sym, np )
      }
      else
      {
        let indent = " ".repeat( self.config.indent_size );
        ( "", format!( "{prefix}{indent}" ) )
      };

      // Format current child
      if self.config.show_branches
      {
        output.push_str( &new_prefix );
        output.push_str( symbol );
        output.push( ' ' );
      }
      else
      {
        output.push_str( &new_prefix );
      }

      output.push_str( &child.name );

      if let Some( ref data ) = child.data
      {
        let rendered = render_item( data );
        if !rendered.is_empty()
        {
          output.push( ' ' );
          output.push_str( &rendered );
        }
      }

      output.push( '\n' );

      // Recurse for children
      let child_prefix = if self.config.show_branches
      {
        if is_last_child
        {
          format!( "{}{}", new_prefix, self.symbols.space )
        }
        else
        {
          format!( "{}{}   ", new_prefix, self.symbols.vertical )
        }
      }
      else
      {
        new_prefix.clone()
      };

      self.format_node( child, output, &child_prefix, is_last_child, depth + 1, render_item );
    }
  }

  /// Format tree with aggregated directory totals
  ///
  /// This method calculates totals for each directory node (sum of all descendant values)
  /// and renders both files and directories with custom formatting.
  ///
  /// # Arguments
  ///
  /// * `tree` - Root node of the tree
  /// * `aggregate_fn` - Function to extract value from item for aggregation
  /// * `render_file` - Function to render file nodes
  /// * `render_directory` - Function to render directory nodes
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ TreeBuilder, TreeFormatter };
  ///
  /// #[ derive( Clone ) ]
  /// struct FileInfo { lines: u64, lang: String }
  ///
  /// let tree = TreeBuilder::new( "root" )
  ///   .insert( &[ "src", "main.rs" ], FileInfo { lines: 100, lang: "Rust".into() } )
  ///   .build();
  ///
  /// let formatter = TreeFormatter::new();
  /// let output = formatter.format_with_aggregation(
  ///   &tree,
  ///   1000u64, // grand total
  ///   | f | f.lines,
  ///   | v | v as f64,
  ///   | f, total, pct | format!( "({} lines, {:.1}%, {})", f.lines, pct, f.lang ),
  ///   | _name, total, pct | format!( "({} lines, {:.1}%)", total, pct ),
  /// );
  /// ```
  pub fn format_with_aggregation< T, V, A, F, D, C >(
    &self,
    tree : &TreeNode< T >,
    grand_total : V,
    aggregate_fn : A,
    convert_to_f64 : C,
    render_file : F,
    render_directory : D,
  ) -> String
  where
    V : Copy + std::ops::Add< Output = V > + Default + std::iter::Sum,
    A : Fn( &T ) -> V,
    C : Fn( V ) -> f64,
    F : Fn( &T, V, f64 ) -> String,
    D : Fn( &str, V, f64 ) -> String,
  {
    let mut output = String::with_capacity( INITIAL_CAPACITY );
    self.format_node_with_aggregation(
      tree,
      &mut output,
      "",
      true,
      0,
      grand_total,
      true,
      &aggregate_fn,
      &convert_to_f64,
      &render_file,
      &render_directory,
    );
    output
  }

  /// Calculate aggregated total for a node and all descendants
  fn calculate_aggregate< T, V, A >( node : &TreeNode< T >, aggregate_fn : &A ) -> V
  where
    V : Copy + std::ops::Add< Output = V > + Default + std::iter::Sum,
    A : Fn( &T ) -> V,
  {
    let direct = node.data.as_ref().map_or( V::default(), aggregate_fn );
    let children_total : V = node.children.iter()
      .map( | c | Self::calculate_aggregate( c, aggregate_fn ) )
      .sum();
    direct + children_total
  }

  /// Format node with aggregation support (recursive)
  #[ allow( clippy::too_many_arguments ) ]
  fn format_node_with_aggregation< T, V, A, C, F, D >(
    &self,
    node : &TreeNode< T >,
    output : &mut String,
    prefix : &str,
    is_last : bool,
    depth : usize,
    grand_total : V,
    is_root : bool,
    aggregate_fn : &A,
    convert_to_f64 : &C,
    render_file : &F,
    render_directory : &D,
  )
  where
    V : Copy + std::ops::Add< Output = V > + Default + std::iter::Sum,
    A : Fn( &T ) -> V,
    C : Fn( V ) -> f64,
    F : Fn( &T, V, f64 ) -> String,
    D : Fn( &str, V, f64 ) -> String,
  {
    // Check max depth
    if let Some( max_depth ) = self.config.max_depth
    {
      if depth >= max_depth
      {
        return;
      }
    }

    // For directory nodes (no data), show directory name with aggregated total
    if node.data.is_none() && !is_root
    {
      let node_total = Self::calculate_aggregate( node, aggregate_fn );
      let percentage = convert_to_f64( node_total ) / convert_to_f64( grand_total ) * 100.0;

      output.push_str( &format!(
        "{}/ {}",
        node.name,
        render_directory( &node.name, node_total, percentage )
      ) );
      output.push( '\n' );
    }

    // Calculate child prefix
    let child_prefix = if is_root
    {
      String::new()
    }
    else if is_last
    {
      format!( "{prefix}    " )
    }
    else
    {
      format!( "{prefix}│   " )
    };

    // Display file data if present
    if let Some( ref file_data ) = node.data
    {
      let value = aggregate_fn( file_data );
      let percentage = convert_to_f64( value ) / convert_to_f64( grand_total ) * 100.0;

      let symbol = if is_last { "└──" } else { "├──" };
      output.push_str( &format!(
        "{}{} {} {}",
        prefix,
        symbol,
        node.name,
        render_file( file_data, value, percentage )
      ) );
      output.push( '\n' );
    }

    // Recursively format children
    for ( idx, child ) in node.children.iter().enumerate()
    {
      let is_last_child = idx == node.children.len() - 1;
      self.format_node_with_aggregation(
        child,
        output,
        &child_prefix,
        is_last_child,
        depth + 1,
        grand_total,
        false,
        aggregate_fn,
        convert_to_f64,
        render_file,
        render_directory,
      );
    }
  }

  /// Write formatted tree directly to a writer
  ///
  /// # Errors
  ///
  /// Returns an error if writing to the provided writer fails
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ TreeBuilder, TreeFormatter };
  /// use std::io::Cursor;
  ///
  /// let tree = TreeBuilder::new( "root" )
  ///   .insert( &[ "file.txt" ], 100 )
  ///   .build();
  ///
  /// let formatter = TreeFormatter::new();
  /// let mut buffer = Cursor::new( Vec::new() );
  /// formatter.write_to( &tree, &mut buffer, | lines | format!( "{} lines", lines ) ).unwrap();
  ///
  /// let output = String::from_utf8( buffer.into_inner() ).unwrap();
  /// assert!( output.contains( "file.txt" ) );
  /// ```
  pub fn write_to< T, F, W >(
    &self,
    tree : &TreeNode< T >,
    writer : &mut W,
    render_item : F
  )
  -> std::io::Result< () >
  where
    F : Fn( &T ) -> String,
    W : std::io::Write,
  {
    let output = self.format( tree, render_item );
    writer.write_all( output.as_bytes() )
  }

}

impl Default for TreeFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

