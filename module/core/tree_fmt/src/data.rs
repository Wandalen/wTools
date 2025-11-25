//! Core data structures for `tree_fmt`

/// Generic tree node that can hold any data type
///
/// Represents a node in a hierarchical tree structure. Each node has a name,
/// optional data, and a list of child nodes.
#[ derive( Debug, Clone ) ]
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub struct TreeNode< T >
{
  /// Name of the node (e.g., file name, directory name)
  pub name : String,
  /// Optional data associated with the node (None for directory nodes)
  pub data : Option< T >,
  /// Child nodes
  pub children : Vec< TreeNode< T > >,
}

impl< T > TreeNode< T >
{
  /// Create a new tree node
  ///
  /// # Arguments
  ///
  /// * `name` - Name of the node
  /// * `data` - Optional data associated with the node
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::TreeNode;
  ///
  /// // Create a file node with data
  /// let file = TreeNode::new( "main.rs".to_string(), Some( 100u64 ) );
  ///
  /// // Create a directory node without data
  /// let dir : TreeNode< u64 > = TreeNode::new( "src".to_string(), None );
  /// ```
  pub const fn new( name : String, data : Option< T > ) -> Self
  {
    Self
    {
      name,
      data,
      children : Vec::new(),
    }
  }
}

/// Multi-column data for aligned tree formatting
///
/// Represents a tree node with multiple columns that should be aligned
/// vertically when rendered. Each column is a separate string value.
///
/// # Examples
///
/// ```
/// use tree_fmt::ColumnData;
///
/// // Create node with 3 columns: name, version, path
/// let data = ColumnData::new( vec![
///   "api_ollama".to_string(),
///   "v0.1.0".to_string(),
///   "(api/ollama)".to_string()
/// ]);
///
/// assert_eq!( data.columns.len(), 3 );
/// ```
#[ derive( Debug, Clone ) ]
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub struct ColumnData
{
  /// Column values for this node
  pub columns : Vec< String >,
}

impl ColumnData
{
  /// Create new column data from vector of strings
  ///
  /// # Arguments
  ///
  /// * `columns` - Vector of column values
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::ColumnData;
  ///
  /// let data = ColumnData::new( vec![
  ///   "name".to_string(),
  ///   "value".to_string()
  /// ]);
  /// ```
  pub fn new( columns : Vec< String > ) -> Self
  {
    Self { columns }
  }

  /// Create column data from key-value pairs
  ///
  /// Only the values are stored, keys are discarded. Useful for
  /// self-documenting code when building column data.
  ///
  /// # Arguments
  ///
  /// * `pairs` - Vector of (key, value) pairs
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::ColumnData;
  ///
  /// let data = ColumnData::from_pairs( vec![
  ///   ( "name", "api_ollama" ),
  ///   ( "version", "v0.1.0" ),
  ///   ( "path", "(api/ollama)" )
  /// ]);
  ///
  /// assert_eq!( data.columns.len(), 3 );
  /// assert_eq!( data.columns[ 0 ], "api_ollama" );
  /// ```
  pub fn from_pairs( pairs : Vec< ( &str, &str ) > ) -> Self
  {
    Self
    {
      columns : pairs.into_iter().map( | ( _, v ) | v.to_string() ).collect()
    }
  }

  /// Get number of columns
  pub fn len( &self ) -> usize
  {
    self.columns.len()
  }

  /// Check if column data is empty
  pub fn is_empty( &self ) -> bool
  {
    self.columns.is_empty()
  }
}

impl std::fmt::Display for ColumnData
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    write!( f, "{}", self.columns.join( " | " ) )
  }
}

/// Trait for working with table-shaped trees
///
/// Table-shaped trees have structure:
/// ```text
/// root
/// ├── row_1
/// │   ├── column_a: "value"
/// │   └── column_b: "value"
/// └── row_2
///     ├── column_a: "value"
///     └── column_b: "value"
/// ```
pub trait TableShapedView
{
  /// Extract column headers from the first row's children
  ///
  /// Returns `None` if tree is empty or not table-shaped.
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, TableShapedView };
  ///
  /// let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  ///   .add_row( vec![ "Alice".into(), "30".into() ] )
  ///   .build();
  ///
  /// let headers = tree.extract_headers().unwrap();
  /// assert_eq!( headers, vec![ "Name", "Age" ] );
  /// ```
  fn extract_headers( &self ) -> Option< Vec< String > >;

  /// Check if tree has table structure (all rows have same child names)
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, TableShapedView };
  ///
  /// let tree = RowBuilder::new( vec![ "Name".into() ] )
  ///   .add_row( vec![ "Alice".into() ] )
  ///   .build();
  ///
  /// assert!( tree.is_table_shaped() );
  /// ```
  fn is_table_shaped( &self ) -> bool;

  /// Extract row data as Vec<Vec<String>>
  ///
  /// Each row becomes a vector of cell values.
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, TableShapedView };
  ///
  /// let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  ///   .add_row( vec![ "Alice".into(), "30".into() ] )
  ///   .build();
  ///
  /// let rows = tree.to_rows();
  /// assert_eq!( rows.len(), 1 );
  /// assert_eq!( rows[ 0 ], vec![ "Alice", "30" ] );
  /// ```
  fn to_rows( &self ) -> Vec< Vec< String > >;
}

impl< T : std::fmt::Display > TableShapedView for TreeNode< T >
{
  fn extract_headers( &self ) -> Option< Vec< String > >
  {
    self.children.first().map( | row |
      row.children.iter().map( | child | child.name.clone() ).collect()
    )
  }

  fn is_table_shaped( &self ) -> bool
  {
    if self.children.is_empty()
    {
      return true; // Empty tree is trivially table-shaped
    }

    // Get expected column names from first row
    let expected_headers : Vec< String > = match self.children.first()
    {
      Some( first_row ) => first_row.children.iter().map( | c | c.name.clone() ).collect(),
      None => return true,
    };

    // Check all rows have same structure
    self.children.iter().all( | row |
    {
      if row.children.len() != expected_headers.len()
      {
        return false;
      }

      row.children.iter().zip( &expected_headers ).all( | ( child, expected_name ) |
        child.name == *expected_name
      )
    })
  }

  fn to_rows( &self ) -> Vec< Vec< String > >
  {
    self.children.iter().map( | row |
      row.children.iter().map( | cell |
        cell.data.as_ref().map( std::string::ToString::to_string ).unwrap_or_default()
      ).collect()
    ).collect()
  }
}

// ============================================================================
// Canonical Data Format for Unified Formatting
// ============================================================================

/// Data type classification for table columns
///
/// Used to provide semantic meaning to column data and enable
/// type-aware formatting in different output formats.
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub enum DataType
{
  /// String data
  #[ default ]
  String,
  /// Integer number
  Integer,
  /// Boolean value
  Boolean,
  /// File system path
  Path,
}

/// Metadata describing table structure and column types
///
/// Provides semantic information about table columns that can be used
/// by formatters to produce richer output (e.g., type-aware JSON schemas).
#[ derive( Debug, Clone ) ]
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub struct TableMetadata
{
  /// Column names (headers)
  pub column_names : Vec< String >,
  /// Column data types
  pub column_types : Vec< DataType >,
}

impl TableMetadata
{
  /// Create new metadata with column names and default String types
  pub fn new( column_names : Vec< String > ) -> Self
  {
    let count = column_names.len();
    Self
    {
      column_names,
      column_types : vec![ DataType::String; count ],
    }
  }

  /// Create metadata with explicit column types
  pub fn with_types( column_names : Vec< String >, column_types : Vec< DataType > ) -> Self
  {
    Self { column_names, column_types }
  }
}

/// Canonical table data structure for unified formatting
///
/// This is the universal data format that all formatters accept.
/// Build once using `RowBuilder`, then format to any output format.
///
/// # Examples
///
/// ```
/// use tree_fmt::{ TableView, TableMetadata };
///
/// let view = TableView {
///   metadata: TableMetadata::new( vec![ "Name".into(), "Age".into() ] ),
///   rows: vec![
///     vec![ "Alice".into(), "30".into() ],
///     vec![ "Bob".into(), "25".into() ],
///   ],
/// };
///
/// assert_eq!( view.metadata.column_names.len(), 2 );
/// assert_eq!( view.rows.len(), 2 );
/// ```
#[ derive( Debug, Clone ) ]
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub struct TableView
{
  /// Table metadata (column names and types)
  pub metadata : TableMetadata,
  /// Table rows (data)
  pub rows : Vec< Vec< String > >,
}

impl TableView
{
  /// Create new table view
  pub fn new( metadata : TableMetadata, rows : Vec< Vec< String > > ) -> Self
  {
    Self { metadata, rows }
  }

  /// Convert to `TreeNode` for backward compatibility with visual formatters
  pub fn to_tree_node( &self ) -> TreeNode< Vec< String > >
  {
    let mut root = TreeNode::new( "table".to_string(), None );

    // Add header row
    let mut header_row = TreeNode::new( "row_0".to_string(), None );
    for col_name in &self.metadata.column_names
    {
      let col_node = TreeNode::new(
        col_name.clone(),
        Some( vec![ col_name.clone() ] )
      );
      header_row.children.push( col_node );
    }
    root.children.push( header_row );

    // Add data rows
    for ( row_idx, row_data ) in self.rows.iter().enumerate()
    {
      let mut row_node = TreeNode::new( format!( "row_{}", row_idx + 1 ), None );
      for ( col_idx, cell_value ) in row_data.iter().enumerate()
      {
        let col_name = self.metadata.column_names.get( col_idx )
          .map_or( "", std::string::String::as_str );
        let col_node = TreeNode::new(
          col_name.to_string(),
          Some( vec![ cell_value.clone() ] )
        );
        row_node.children.push( col_node );
      }
      root.children.push( row_node );
    }

    root
  }
}
