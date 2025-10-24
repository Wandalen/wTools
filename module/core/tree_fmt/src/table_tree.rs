//! Table-shaped tree construction helpers

use crate::TreeNode;

/// Builder for constructing table-shaped trees
///
/// Creates trees where root has row nodes, and each row node
/// has column-named children containing cell data.
///
/// # Examples
///
/// ```
/// use tree_fmt::RowBuilder;
///
/// let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .add_row( vec![ "Bob".into(), "25".into() ] )
///   .build();
///
/// assert_eq!( tree.children.len(), 2 );
/// assert_eq!( tree.children[ 0 ].name, "1" );
/// ```
#[ derive( Debug ) ]
pub struct RowBuilder
{
  root : TreeNode< String >,
  headers : Vec< String >,
  row_count : usize,
  rows : Vec< Vec< String > >,
}

impl RowBuilder
{
  /// Create a new table tree builder with column headers
  pub fn new( headers : Vec< String > ) -> Self
  {
    Self
    {
      root : TreeNode::new( "root".to_string(), None ),
      headers,
      row_count : 0,
      rows : Vec::new(),
    }
  }

  /// Validate row length matches headers
  fn validate_row_length( &self, row : &[ String ] )
  {
    assert!(
      row.len() == self.headers.len(),
      "row length {} doesnt match headers length {}",
      row.len(),
      self.headers.len()
    );
  }

  /// Add a row with automatic numeric naming (1, 2, 3, ...)
  ///
  /// Consumes and returns `self` for method chaining.
  ///
  /// # Panics
  ///
  /// Panics if row length doesnt match headers length
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::RowBuilder;
  ///
  /// let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
  ///   .add_row( vec![ "1".into(), "2".into() ] )
  ///   .add_row( vec![ "3".into(), "4".into() ] )
  ///   .build();
  ///
  /// assert_eq!( tree.children.len(), 2 );
  /// ```
  #[ must_use ]
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row( mut self, row : Vec< String > ) -> Self
  {
    self.validate_row_length( &row );
    self.row_count += 1;
    let row_name = self.row_count.to_string();
    self.add_row_internal( row_name, &row );
    self
  }

  /// Add a row with automatic numeric naming (non-consuming, for programmatic use)
  ///
  /// This method takes `&mut self` for use in loops or when you need to keep
  /// the builder mutable.
  ///
  /// # Panics
  ///
  /// Panics if row length doesnt match headers length
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row_mut( &mut self, row : Vec< String > )
  {
    self.validate_row_length( &row );
    self.row_count += 1;
    let row_name = self.row_count.to_string();
    self.add_row_internal( row_name, &row );
  }

  /// Add a row with custom row name
  ///
  /// Consumes and returns `self` for method chaining.
  ///
  /// # Panics
  ///
  /// Panics if row length doesnt match headers length
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::RowBuilder;
  ///
  /// let tree = RowBuilder::new( vec![ "Name".into() ] )
  ///   .add_row_with_name( "Alice".into(), vec![ "30".into() ] )
  ///   .add_row_with_name( "Bob".into(), vec![ "25".into() ] )
  ///   .build();
  ///
  /// assert_eq!( tree.children[ 0 ].name, "Alice" );
  /// ```
  #[ must_use ]
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row_with_name( mut self, row_name : String, row : Vec< String > ) -> Self
  {
    self.validate_row_length( &row );
    self.add_row_internal( row_name, &row );
    self
  }

  /// Add a row with custom row name (non-consuming, for programmatic use)
  ///
  /// This method takes `&mut self` for use in loops or when you need to keep
  /// the builder mutable.
  ///
  /// # Panics
  ///
  /// Panics if row length doesnt match headers length
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row_with_name_mut( &mut self, row_name : String, row : Vec< String > )
  {
    self.validate_row_length( &row );
    self.add_row_internal( row_name, &row );
  }

  /// Internal row addition (no validation)
  fn add_row_internal( &mut self, row_name : String, row : &[ String ] )
  {
    // Store row data for TableView
    self.rows.push( row.to_vec() );

    // Build TreeNode structure for backward compatibility
    let mut row_node = TreeNode::new( row_name, None );

    for ( header, value ) in self.headers.iter().zip( row.iter() )
    {
      let cell_node = TreeNode::new( header.clone(), Some( value.clone() ) );
      row_node.children.push( cell_node );
    }

    self.root.children.push( row_node );
  }

  /// Build as canonical `TableView` for use with Format trait
  ///
  /// Creates a `TableView` that can be formatted using any formatter
  /// implementing the `Format` trait (json, yaml, table, text, etc).
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::{ RowBuilder, TableFormatter, Format, TableConfig };
  ///
  /// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  ///   .add_row( vec![ "Alice".into(), "30".into() ] )
  ///   .build_view();
  ///
  /// // Format as table using Format trait
  /// let formatter = TableFormatter::with_config( TableConfig::plain() );
  /// let output = Format::format( &formatter, &view ).unwrap();
  /// assert!( output.contains( "Alice" ) );
  /// ```
  pub fn build_view( self ) -> crate::TableView
  {
    crate::TableView::new(
      crate::TableMetadata::new( self.headers ),
      self.rows
    )
  }

  /// Build the final tree (for backward compatibility)
  ///
  /// Returns a `TreeNode<String>` structure for use with legacy formatters.
  /// For new code, prefer `build_view()` which returns the canonical `TableView`.
  pub fn build( self ) -> TreeNode< String >
  {
    self.root
  }
}
