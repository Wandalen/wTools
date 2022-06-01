
macro_rules! NODE_ID
{
  () => { < < Self as GraphNodesNominalInterface >::NodeHandle as HasId >::Id };
}

macro_rules! EDGE_ID
{
  () => { < < Self as GraphEdgesNominalInterface >::EdgeHandle as HasId >::Id };
}

impls!
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
  Box< dyn Iterator< Item = ( NODE_ID!(), &Self::NodeHandle ) > + 'b >
  where
    'a : 'b,
  {
    Box::new( self.id_to_node_map.iter().map( | el | ( *el.0, el.1 ) ) )
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
    Box::new( self.id_to_edge_map.iter().map( | el | ( *el.0, el.1 ) ) )
  }

  //

  fn nedges( &self ) -> usize
  {
    self.id_to_edge_map.len()
  }

  //

  fn node_mut< IntoId >( &mut self, id : IntoId ) -> &mut Self::NodeHandle
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

  fn node_making< IntoId >( &mut self, id : IntoId ) -> NODE_ID!()
  where
    IntoId : Into< NODE_ID!() >,
  {
    let id = id.into();

    let result = self.id_to_node_map
    .entry( id )
    .or_insert_with( || canonical::Node::make_with_id( id ).into() )
    ;
    result.id()
  }

  //

  fn _edge_id_generate( &mut self, _in_node : NODE_ID!(), _out_node : NODE_ID!() ) -> EDGE_ID!()
  {
    while self.id_to_edge_map.contains_key( &self._current_edge_id )
    {
      self._current_edge_id = self._current_edge_id.next();
      assert!( self._current_edge_id.is_valid(), "Not more space for ids" );
    }
    self._current_edge_id
  }

  //

  fn _edge_add( &mut self, edge_id : EDGE_ID!(), in_node : NODE_ID!(), out_node : NODE_ID!() )
  {

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
        kind : Default::default(),
      }
    });

  }

  //

  fn make_0() -> Self
  {
    let id_to_node_map = IndexMap::new();
    let id_to_edge_map = IndexMap::new();
    let _current_edge_id = EdgeId::first();
    Self
    {
      id_to_node_map,
      id_to_edge_map,
      _current_edge_id,
    }
  }

  //

  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    f.write_fmt( format_args!( "NodeFactory\n" ) )?;
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

}
