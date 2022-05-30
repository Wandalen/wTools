
macro_rules! NODE_ID
{
  () => { < < Self as GraphNodesInterface >::NodeHandle as HasId >::Id };
}

impls!
{

  //

  fn nodes< 'a, 'b >( &'a self )
  ->
  Box< dyn Iterator< Item = ( &NODE_ID!(), &Self::NodeHandle ) > + 'b >
  where
    'a : 'b,
  {
    let iterator
      : Box< dyn Iterator< Item = ( &NODE_ID!(), &Self::NodeHandle ) > >
      = Box::new( self.id_to_node_map.iter() )
    ;
    iterator
  }

  //

  fn node< IntoId >( &self, id : IntoId ) -> &Self::NodeHandle
  where
    IntoId : Into< NODE_ID!() >,
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
    .or_insert_with( || Node::make_with_id( id ).into() )
    ;
    result.id()
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
