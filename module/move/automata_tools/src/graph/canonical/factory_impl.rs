
macro_rules! NODE_ID
{
  () => { < < Self as GraphNodesNominalInterface >::NodeHandle as HasId >::Id };
}

macro_rules! EDGE_ID
{
  () => { < < Self as GraphEdgesNominalInterface >::EdgeHandle as HasId >::Id };
}

impls3!
{

  //

  fn node< IntoId >( &self, id : IntoId ) -> &Self::NodeHandle
  where
    IntoId : Into< NODE_ID!() >,
  {
    let id = id.into();
    let got = self.id_to_node_map.get( &id );
    if got.is_some()
    {
      let result : &Self::NodeHandle = got.unwrap();
      return result;
    }
    unreachable!( "No node with id {:?} found", id );
  }

  //

  fn nodes< 'a, 'b >( &'a self )
  ->
  Box< dyn Iterator< Item = ( NODE_ID!(), &< Self as GraphNodesNominalInterface >::NodeHandle ) > + 'b >
  // core::slice::Iter< 'a, ( NODE_ID!(), &'b < Self as GraphNodesNominalInterface >::NodeHandle ) >
  where
    'a : 'b,
  {
    Box::new( self.id_to_node_map.iter().map( | el | ( *el.0, el.1) ) )
  }

  //

  fn nnodes( &self ) -> usize
  {
    self.id_to_node_map.len()
  }

  //

  fn edge< IntoId >( &self, id : IntoId ) -> &Self::EdgeHandle
  where
    IntoId : Into< EDGE_ID!() >,
  {
    let id = id.into();
    let got = self.id_to_edge_map.get( &id );
    if got.is_some()
    {
      let result : &Self::EdgeHandle = got.unwrap();
      return result;
    }
    unreachable!( "No edge with id {:?} found", id );
  }

  //

  fn edges< 'a, 'b >( &'a self )
  ->
  Box< dyn Iterator< Item = ( EDGE_ID!(), &Self::EdgeHandle ) > + 'b >
  where
    'a : 'b,
  {
    Box::new( self.id_to_edge_map.iter().map( | el | ( *el.0, el.1) ) )
  }

  //

  fn nedges( &self ) -> usize
  {
    self.id_to_edge_map.len()
  }

  //

  ? fn node_mut< IntoId >( &mut self, id : IntoId ) -> &mut Self::NodeHandle
  where
    IntoId : Into< NODE_ID!() >
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

  ? fn node_making< IntoId >( &mut self, id : IntoId ) -> NODE_ID!()
  where
    IntoId : Into< NODE_ID!() >,
  {
    let id = id.into();

    let result = self.id_to_node_map
    .entry( id )
    .or_insert_with( || canonical::Node::_make_with_id( id ).into() )
    // .or_insert_with( || canonical::Node::make_with_id( id ).into() )
    ;
    result.id()
  }

  //

  // fn _edge_id_generate( &mut self, _in_node : NODE_ID!(), _out_node : NODE_ID!() ) -> EDGE_ID!()
  // {
  //   while self.id_to_edge_map.contains_key( &self._current_edge_id )
  //   {
  //     self._current_edge_id = self._current_edge_id.next();
  //     assert!( self._current_edge_id.is_valid(), "Not more space for ids" );
  //   }
  //   self._current_edge_id
  // }

  //

  fn _edge_add( &mut self, in_node : NODE_ID!(), out_node : NODE_ID!() ) -> EDGE_ID!()
  {
    let edge_id = self._edge_id_generator.id_next();

    self.id_to_edge_map
    .entry( edge_id )
    .and_modify( | _ | { panic!( "Edge {:?} already exists", edge_id ) } )
    .or_insert_with( ||
    {
      canonical::Edge
      {
        id : edge_id,
        in_node,
        out_node,
        // kind : Default::default(),
      }
    });

    edge_id
  }

  //

  // fn from_0() -> Self
  // {
  //   let id_to_node_map = IndexMap::new();
  //   let id_to_edge_map = IndexMap::new();
  //   let _node_id_generator = Default::default();
  //   let _edge_id_generator = Default::default();
  //   // let _current_edge_id = EdgeId::first();
  //   Self
  //   {
  //     id_to_node_map,
  //     id_to_edge_map,
  //     _node_id_generator,
  //     _edge_id_generator,
  //     // ..default()
  //     // _current_edge_id,
  //     // _p : core::marker::PhantomData,
  //   }
  // }

  //

  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    f.write_fmt( format_args!( "GenerativeNodeFactory\n" ) )?;
    let mut first = true;
    for ( _id, node ) in self.nodes()
    {
      if !first
      {
        f.write_str( "\n" )?;
      }
      first = false;
      f.write_str( &wtools::string::indentation( "  ", format!( "{:?}", node ), "" ) )?;
    }
    f.write_str( "" )
  }

  ?

  ///
  /// Iterate output nodes of the node.
  ///

  fn node_add_out_nodes< IntoId1, IntoId2, Iter >
  (
    &mut self,
    in_node_id : IntoId1,
    out_nodes_iter : Iter,
  )
  where
    IntoId1 : Into< NODE_ID!() >,
    IntoId2 : Into< NODE_ID!() >,
    Iter : IntoIterator< Item = IntoId2 >,
    Iter::IntoIter : Clone,
  {

    let in_node_id = in_node_id.into();
    let iter = out_nodes_iter.into_iter();

    let out_ids : Vec< _ > = iter
    .map( | out_node_id |
    {
      let out_node_id = out_node_id.into();
      #[ cfg( debug_assertions ) ]
      let _ = self.node( out_node_id );
      let out_edge_id = self._edge_make_for_nodes( in_node_id, out_node_id );
      ( out_edge_id, out_node_id )
    })
    .collect()
    ;

    let in_node = self.node_mut( in_node_id );

    for out_id in out_ids
    {
      in_node.out_edges.insert( out_id.0 );
      in_node.out_nodes.insert( out_id.1 );
    }

  }

  //

  fn out_nodes_ids< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
  ->
  Box< dyn Iterator< Item = NODE_ID!() > + 'b >
  where
    IntoId : Into< NODE_ID!() >,
    'a : 'b,
  {
    let node = self.node( node_id );
    let iterator
      : Box< dyn Iterator< Item = NODE_ID!() > >
      = Box::new( node.out_nodes.iter().cloned() );
    iterator
  }

  //

  fn out_edges_ids< 'a, 'b, IntoId >( &'a self, node_id : IntoId )
  ->
  Box< dyn Iterator< Item = EDGE_ID!() > + 'b >
  where
    IntoId : Into< NODE_ID!() >,
    'a : 'b,
  {
    let node = self.node( node_id );
    let iterator
      : Box< dyn Iterator< Item = EDGE_ID!() > >
      = Box::new( node.out_edges.iter().cloned() );
    iterator
  }

}
