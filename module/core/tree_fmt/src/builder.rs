//! `TreeBuilder` for constructing trees from flat data

use crate::TreeNode;

/// Builder for constructing tree structures from flat data
///
/// Provides methods to build a hierarchical tree from flat lists of items
/// with path-based insertion.
#[ derive( Debug ) ]
pub struct TreeBuilder< T >
{
  root : TreeNode< T >,
}

impl< T > TreeBuilder< T >
{
  /// Create a new tree builder with a root node name
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::TreeBuilder;
  ///
  /// let builder : TreeBuilder< i32 > = TreeBuilder::new( "root" );
  /// ```
  pub fn new( root_name : impl Into< String > ) -> Self
  {
    Self
    {
      root : TreeNode::new( root_name.into(), None ),
    }
  }

  /// Insert an item at the specified path
  ///
  /// # Arguments
  ///
  /// * `path` - Array of path components (e.g., `&["src", "main.rs"]`)
  /// * `data` - Data to associate with the leaf node
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::TreeBuilder;
  ///
  /// let tree = TreeBuilder::new( "root" )
  ///   .insert( &[ "src", "main.rs" ], 100 )
  ///   .insert( &[ "src", "lib.rs" ], 200 )
  ///   .build();
  /// ```
  #[ must_use ]
  pub fn insert( mut self, path : &[ &str ], data : T ) -> Self
  {
    if path.is_empty()
    {
      return self;
    }

    // Filter out empty components first
    let filtered_path : Vec< &str > = path.iter().filter( | c | !c.is_empty() ).copied().collect();

    if filtered_path.is_empty()
    {
      return self;
    }

    let mut current = &mut self.root;

    // Navigate/create directory structure for all components except the last
    for component in filtered_path.iter().take( filtered_path.len() - 1 )
    {
      let component_str = (*component).to_string();

      // Intermediate directory node
      let child_idx = current.children.iter().position( | c | c.name == component_str );

      if let Some( idx ) = child_idx
      {
        current = &mut current.children[ idx ];
      }
      else
      {
        current.children.push( TreeNode::new( component_str.clone(), None ) );
        let last_idx = current.children.len() - 1;
        current = &mut current.children[ last_idx ];
      }
    }

    // Insert the final component as a leaf with data
    let leaf_name = filtered_path[ filtered_path.len() - 1 ].to_string();
    current.children.push( TreeNode::new( leaf_name, Some( data ) ) );

    self
  }

  /// Build the final tree
  ///
  /// Consumes the builder and returns the root node.
  pub fn build( self ) -> TreeNode< T >
  {
    self.root
  }
}

impl< T : Clone > TreeBuilder< T >
{
  /// Build tree from items with path extractor and data extractor
  ///
  /// # Arguments
  ///
  /// * `items` - Slice of items to build tree from
  /// * `extract_path` - Function to extract path components from an item
  /// * `extract_data` - Function to extract data from an item
  ///
  /// # Examples
  ///
  /// ```
  /// use tree_fmt::TreeBuilder;
  ///
  /// #[ derive( Clone ) ]
  /// struct FileItem { path : String, size : u64 }
  ///
  /// let items = vec![
  ///   FileItem { path : "src/main.rs".to_string(), size : 100 },
  ///   FileItem { path : "src/lib.rs".to_string(), size : 200 },
  /// ];
  ///
  /// let tree = TreeBuilder::from_items( &items, | item | {
  ///   item.path.split( '/' ).map( | s | s.to_string() ).collect()
  /// }, | item | item.clone() );
  /// ```
  pub fn from_items< F, G >( items : &[ T ], extract_path : F, extract_data : G ) -> TreeNode< T >
  where
    F : Fn( &T ) -> Vec< String >,
    G : Fn( &T ) -> T,
  {
    let mut builder = Self::new( "root" );

    for item in items
    {
      let path = extract_path( item );
      let data = extract_data( item );

      if !path.is_empty()
      {
        let path_refs : Vec< &str > = path.iter().map( String::as_str ).collect();
        builder = builder.insert( &path_refs, data );
      }
    }

    builder.build()
  }
}
