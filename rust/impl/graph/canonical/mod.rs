#[ cfg( feature = "cell_factory" ) ]
wtools::meta::mod_interface!
{
  /// Implements canonical factory where each node in a cell.
  layer cell_factory;
  /// Implements canonical edge.
  layer edge;
  /// Implements canonical factory.
  layer factory;
  /// Implements several identities.
  layer identity;
  /// Implements canonical node.
  layer node;
  /// Implements node cell.
  layer node_cell;
}
#[ cfg( not( feature = "cell_factory" ) ) ]
wtools::meta::mod_interface!
{
  /// Implements canonical edge.
  layer edge;
  /// Implements canonical factory.
  layer factory;
  /// Implements several identities.
  layer identity;
  /// Implements canonical node.
  layer node;
}
