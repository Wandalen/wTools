crate::mod_interface!
{
  /// Implements canonical factory where each node in a cell.
  #[ cfg( feature = "cell_factory" ) ]
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
  #[ cfg( feature = "cell_factory" ) ]
  layer node_cell;
}
