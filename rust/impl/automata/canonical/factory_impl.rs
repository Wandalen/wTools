
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
    Id : Into< ID!() >,
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

  fn node_extend_out_nodes< Id, Iter >
  (
    &mut self,
    node_id : Id,
    out_nodes_iter : Iter,
  )
  where
    Iter : IntoIterator< Item = Id >,
    Iter::IntoIter : Clone,
    Id : Into< ID!() >
  {

    // let out_nodes_iter2 = out_nodes_iter.into_iter()
    // .map( | id |
    // {
    //   let id = id.into();
    //   self.node( id );
    //   id
    // })
    // // .collect()
    // ;
    // self.node_mut( node_id.into() ).extend( out_nodes_iter2 );

    // let out_nodes_iter2 : Vec< _ > = out_nodes_iter.into_iter()
    // .map( | id |
    // {
    //   let id = id.into();
    //   self.node( id );
    //   id
    // })
    // .collect()
    // ;
    // self.node_mut( node_id.into() ).extend( out_nodes_iter2 );

    let iter = out_nodes_iter.into_iter();
    let iter2 = iter.clone();

    #[ cfg( debug_assertions ) ]
    iter
    .map( | id |
    {
      let node = self.node( id );
    })
    ;

    let iter3 = iter2.into_iter()
    .map( | id |
    {
      let id = id.into();
      id
    })
    ;

    // xxx
    self.node_mut( node_id.into() ).extend( iter3 );
  }

  //

  fn node< Id >( &self, id : Id ) -> &Self::NodeHandle
  where
    Id : Into< ID!() >,
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

  fn node_mut< Id >( &mut self, id : Id ) -> &mut Self::NodeHandle
  where
    Id : Into< ID!() >
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

  fn out_nodes< 'a, 'b, Id >( &'a self, node_id : Id )
  ->
  Box< dyn Iterator< Item = ID!() > + 'b >
  where
    Id : Into< ID!() >,
    'a : 'b,
  {
    let node = self.node( node_id );
    let iterator
      : Box< dyn Iterator< Item = ID!() > >
      = Box::new( node.out_nodes.iter().cloned() );
    iterator
  }

}
