
macro_rules! ID
{
  () => { < < Self as NodeFactoryInterface >::NodeHandle as HasId >::Id };
}

impls!
{

  ///
  /// Constructor.
  ///

  pub fn make() -> Self
  {
    let id_to_node_map = HashMap::new();
    Self
    {
      id_to_node_map,
    }
  }

  //

  fn node_making< Id >( &mut self, id : Id ) -> ID!()
  where
    Id : Into< ID!() >
  {
    let id = id.into();

    let result = self.id_to_node_map
    .entry( id )
    .or_insert_with( || Node::make_named( id ) )
    ;
    result.id()
  }

  ///
  /// Iterate output nodes of the node.
  ///

  fn node_extend_out_nodes< Iter >
  (
    &mut self,
    node_id : ID!(),
    out_nodes_iter : Iter,
  )
  where
    Iter : IntoIterator< Item = ID!() >,
  {
    self.node_mut( node_id ).extend( out_nodes_iter );
  }

  //

  fn node( &self, id : ID!() ) -> &Self::NodeHandle
  {
    let id = id.into();
    let got = self.id_to_node_map.get( &id );
    if got.is_some()
    {
      let result : &Self::NodeHandle = got.unwrap().clone();
      return result;
    }
    unreachable!( "No node with id {:?} found", id );
  }

  //

  fn node_mut( &mut self, id : ID!() ) -> &mut Self::NodeHandle
  {
    let id = id.into();
    let got = self.id_to_node_map.get_mut( &id );
    if got.is_some()
    {
      let result : &mut Self::NodeHandle = got.unwrap();
      return result;
    }
    unreachable!( "No node with id {:?} found", id );
  }

  //

  fn out_nodes< 'a, 'b >( &'a self, node_id : ID!() )
  ->
  Box< dyn Iterator< Item = ID!() > + 'b >
  where
    'a : 'b,
  {
    let node = self.node( node_id );
    let iterator
      : Box< dyn Iterator< Item = ID!() > >
      = Box::new( node.out_nodes.iter().cloned() );
    iterator
  }

}
