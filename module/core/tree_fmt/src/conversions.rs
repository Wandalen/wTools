//! Data conversion utilities between tree representations

use crate::{ TreeNode, RowBuilder };

/// Formatter parameters for tree flattening
#[ derive( Debug, Clone ) ]
#[ allow( clippy::struct_excessive_bools ) ]
pub struct FlattenConfig
{
  /// Include path column
  pub include_path : bool,
  /// Include name column
  pub include_name : bool,
  /// Include depth column
  pub include_depth : bool,
  /// Include data column
  pub include_data : bool,
  /// Custom column names (path, name, depth, data)
  pub column_names : Option< ( String, String, String, String ) >,
}

impl Default for FlattenConfig
{
  fn default() -> Self
  {
    Self
    {
      include_path : true,
      include_name : true,
      include_depth : true,
      include_data : true,
      column_names : None,
    }
  }
}

impl FlattenConfig
{
  /// Create new config with defaults (all columns included)
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set whether to include path column
  #[ must_use ]
  pub fn include_path( mut self, include : bool ) -> Self
  {
    self.include_path = include;
    self
  }

  /// Set whether to include name column
  #[ must_use ]
  pub fn include_name( mut self, include : bool ) -> Self
  {
    self.include_name = include;
    self
  }

  /// Set whether to include depth column
  #[ must_use ]
  pub fn include_depth( mut self, include : bool ) -> Self
  {
    self.include_depth = include;
    self
  }

  /// Set whether to include data column
  #[ must_use ]
  pub fn include_data( mut self, include : bool ) -> Self
  {
    self.include_data = include;
    self
  }

  /// Set custom column names
  #[ must_use ]
  pub fn column_names( mut self, path : String, name : String, depth : String, data : String ) -> Self
  {
    self.column_names = Some( ( path, name, depth, data ) );
    self
  }
}

/// Flatten hierarchical tree to table-shaped tree
///
/// Creates table with columns: path, name, depth, data.
/// Each node in the tree becomes a row in the resulting table-shaped tree.
///
/// # Examples
///
/// ```
/// use tree_fmt::{ TreeBuilder, conversions };
///
/// let tree = TreeBuilder::new( "root" )
///   .insert( &[ "src", "main.rs" ], 100 )
///   .build();
///
/// let flattened = conversions::flatten_to_table_tree( &tree );
/// // Returns table-shaped tree with path/name/depth/data columns
/// ```
pub fn flatten_to_table_tree< T : std::fmt::Display >( tree : &TreeNode< T > ) -> TreeNode< String >
{
  flatten_to_table_tree_with_config( tree, &FlattenConfig::default() )
}

/// Flatten hierarchical tree with custom formatter parameters
///
/// # Examples
///
/// ```
/// use tree_fmt::{ TreeBuilder, conversions::{ flatten_to_table_tree_with_config, FlattenConfig } };
///
/// let tree = TreeBuilder::new( "root" ).insert( &[ "file.txt" ], 100 ).build();
///
/// let config = FlattenConfig::new()
///   .include_path( false )
///   .include_depth( false );
///
/// let flattened = flatten_to_table_tree_with_config( &tree, &config );
/// // Returns table with only name and data columns
/// ```
pub fn flatten_to_table_tree_with_config< T : std::fmt::Display >(
  tree : &TreeNode< T >,
  config : &FlattenConfig
)
-> TreeNode< String >
{
  let ( path_name, name_name, depth_name, data_name ) = config.column_names.clone().unwrap_or
  (
    ( "path".into(), "name".into(), "depth".into(), "data".into() )
  );

  let mut headers = Vec::new();
  if config.include_path { headers.push( path_name ); }
  if config.include_name { headers.push( name_name ); }
  if config.include_depth { headers.push( depth_name ); }
  if config.include_data { headers.push( data_name ); }

  let mut builder = RowBuilder::new( headers );

  traverse_and_flatten_with_config( tree, &mut builder, "", 0, config );
  builder.build()
}

/// DFS traversal with custom config
fn traverse_and_flatten_with_config< T : std::fmt::Display >(
  node : &TreeNode< T >,
  builder : &mut RowBuilder,
  path : &str,
  depth : usize,
  config : &FlattenConfig,
)
{
  let current_path = if path.is_empty()
  {
    node.name.clone()
  }
  else
  {
    format!( "{}/{}", path, node.name )
  };

  let data_str = node.data.as_ref()
    .map( ToString::to_string )
    .unwrap_or_default();

  let mut row = Vec::new();
  if config.include_path { row.push( current_path.clone() ); }
  if config.include_name { row.push( node.name.clone() ); }
  if config.include_depth { row.push( depth.to_string() ); }
  if config.include_data { row.push( data_str ); }

  builder.add_row_mut( row );

  for child in &node.children
  {
    traverse_and_flatten_with_config( child, builder, &current_path, depth + 1, config );
  }
}
