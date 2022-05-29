
macro_rules! ID
{
  () => { < < Self as NodeFactoryInterface >::NodeHandle as HasId >::Id };
}

impls!
{

  //

  fn node_making< Id >( &mut self, id : Id ) -> ID!()
  where
    Id : Into< ID!() >,
  {
    let id = id.into();

    let result = self.id_to_node_map
    .entry( id )
    .or_insert_with( || Node::make_named( id ).into() )
    ;
    result.id()
  }

  //

  fn nodes< 'a, 'b >( &'a self )
  ->
  Box< dyn Iterator< Item = ( &ID!(), &Self::NodeHandle ) > + 'b >
  where
    'a : 'b,
  {
    let iterator
      : Box< dyn Iterator< Item = ( &ID!(), &Self::NodeHandle ) > >
      = Box::new( self.id_to_node_map.iter() )
    ;
    iterator
  }

  //

  // fn node< Id >( &self, id : Id ) -> &Self::NodeHandle
  // where
  //   Id : Into< ID!() >,
  // {
  //   let id = id.into();
  //   let got = self.id_to_node_map.get( &id );
  //   if got.is_some()
  //   {
  //     let result : &Self::NodeHandle = got.unwrap().clone();
  //     return result;
  //   }
  //   unreachable!( "No node with id {:?} found", id );
  // }

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

}
