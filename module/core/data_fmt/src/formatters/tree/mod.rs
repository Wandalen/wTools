//! `TreeFormatter` for hierarchical tree display with 2 formatting methods
//!
//! ## Available Methods
//!
//! ### Hierarchical Tree
//! ```
//! # use data_fmt::{ TreeBuilder, TreeFormatter };
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
//! # use data_fmt::{ TreeNode, ColumnData, TreeFormatter };
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
//! ```text
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
//! See `tests/regression_alignment_column.rs` for visual demonstration comparing
//! aligned vs unaligned output.

use crate::{ TreeNode, TreeConfig, TreeSymbols };

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
  /// use data_fmt::{ TreeBuilder, TreeFormatter };
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

  /// Write formatted tree directly to a writer
  ///
  /// # Errors
  ///
  /// Returns an error if writing to the provided writer fails
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::{ TreeBuilder, TreeFormatter };
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

mod aligned;
mod aggregated;
