//! Table-shaped tree construction helpers

use crate::TreeNode;
use color_tools::DecoratedText;

/// Builder for constructing table-shaped trees
///
/// Creates trees where root has row nodes, and each row node
/// has column-named children containing cell data.
///
/// # Examples
///
/// ```
/// use data_fmt::RowBuilder;
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
  rows : Vec< Vec< DecoratedText > >,
  row_details : Vec< Option< DecoratedText > >,
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
      row_details : Vec::new(),
    }
  }

  /// Validate row length matches headers
  fn validate_row_length( &self, len : usize )
  {
    assert!(
      len == self.headers.len(),
      "row length {} doesnt match headers length {}",
      len,
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
  /// use data_fmt::RowBuilder;
  ///
  /// let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
  ///   .add_row( vec![ "1", "2" ] )
  ///   .add_row( vec![ "3", "4" ] )
  ///   .build();
  ///
  /// assert_eq!( tree.children.len(), 2 );
  /// ```
  #[ must_use ]
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row( mut self, row : Vec< DecoratedText > ) -> Self
  {
    self.validate_row_length( row.len() );
    self.row_count += 1;
    let row_name = self.row_count.to_string();
    self.add_row_internal( row_name, row, None );
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
  pub fn add_row_mut( &mut self, row : Vec< DecoratedText > )
  {
    self.validate_row_length( row.len() );
    self.row_count += 1;
    let row_name = self.row_count.to_string();
    self.add_row_internal( row_name, row, None );
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
  /// use data_fmt::RowBuilder;
  ///
  /// let tree = RowBuilder::new( vec![ "Name".into() ] )
  ///   .add_row_with_name( "Alice".into(), vec![ "30" ] )
  ///   .add_row_with_name( "Bob".into(), vec![ "25" ] )
  ///   .build();
  ///
  /// assert_eq!( tree.children[ 0 ].name, "Alice" );
  /// ```
  #[ must_use ]
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row_with_name( mut self, row_name : String, row : Vec< DecoratedText > ) -> Self
  {
    self.validate_row_length( row.len() );
    self.add_row_internal( row_name, row, None );
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
  pub fn add_row_with_name_mut( &mut self, row_name : String, row : Vec< DecoratedText > )
  {
    self.validate_row_length( row.len() );
    self.add_row_internal( row_name, row, None );
  }

  /// Add a row with an optional detail annotation line
  ///
  /// Consumes and returns `self` for method chaining.
  /// The detail appears as an indented line below the row in table output.
  ///
  /// # Panics
  ///
  /// Panics if row length doesnt match headers length
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::RowBuilder;
  ///
  /// let view = RowBuilder::new( vec![ "Name".into() ] )
  ///   .add_row_with_detail( vec![ "Alice" ], Some( "note".into() ) )
  ///   .build_view();
  ///
  /// assert_eq!( view.row_details[ 0 ], Some( data_fmt::DecoratedText::from( "note" ) ) );
  /// ```
  #[ must_use ]
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row_with_detail
  (
    mut self,
    row : Vec< DecoratedText >,
    detail : Option< DecoratedText >,
  )
  -> Self
  {
    self.validate_row_length( row.len() );
    self.row_count += 1;
    let row_name = self.row_count.to_string();
    self.add_row_internal( row_name, row, detail );
    self
  }

  /// Add a row with an optional detail annotation line (non-consuming)
  ///
  /// This method takes `&mut self` for use in loops or when you need to keep
  /// the builder mutable.
  ///
  /// # Panics
  ///
  /// Panics if row length doesnt match headers length
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::RowBuilder;
  ///
  /// let mut builder = RowBuilder::new( vec![ "Name".into() ] );
  /// builder.add_row_with_detail_mut( vec![ "Alice" ], Some( "note".into() ) );
  /// let view = builder.build_view();
  ///
  /// assert_eq!( view.row_details[ 0 ], Some( data_fmt::DecoratedText::from( "note" ) ) );
  /// ```
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn add_row_with_detail_mut
  (
    &mut self,
    row : Vec< DecoratedText >,
    detail : Option< DecoratedText >,
  )
  {
    self.validate_row_length( row.len() );
    self.row_count += 1;
    let row_name = self.row_count.to_string();
    self.add_row_internal( row_name, row, detail );
  }

  /// Internal row addition (no validation)
  fn add_row_internal( &mut self, row_name : String, row : Vec< DecoratedText >, detail : Option< DecoratedText > )
  {
    self.row_details.push( detail );

    // Build TreeNode structure for backward compatibility
    let mut row_node = TreeNode::new( row_name, None );

    for ( header, value ) in self.headers.iter().zip( row.iter() )
    {
      let cell_node = TreeNode::new( header.clone(), Some( value.render() ) );
      row_node.children.push( cell_node );
    }

    self.root.children.push( row_node );

    // Store row data for TableView (after tree nodes are built)
    self.rows.push( row );
  }

  /// Build as canonical `TableView` for use with Format trait
  ///
  /// Creates a `TableView` that can be formatted using any formatter
  /// implementing the `Format` trait (json, yaml, table, text, etc).
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::{ RowBuilder, TableFormatter, Format, TableConfig };
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
    crate::TableView::with_details(
      crate::TableMetadata::new( self.headers ),
      self.rows,
      self.row_details,
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
