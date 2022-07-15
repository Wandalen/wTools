#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{

  fn basic()
  {
    use TheModule::prelude::*;

    struct Pair( i32, i32 );

    impl Enumerable for Pair
    {
      type Item = i32;
      fn len( &self ) -> usize
      {
        2
      }
      fn element( &self, index : usize ) -> &Self::Item
      {
        debug_assert!( index < 2 );
        if index == 0
        {
          &self.0
        }
        else
        {
          &self.1
        }
      }
      fn element_copy( &self, index : usize ) -> Self::Item
      {
        debug_assert!( index < 2 );
        if index == 0
        {
          self.0
        }
        else
        {
          self.1
        }
      }
    }

    impl IntoIterator for Pair
    {
      type Item = < Pair as Enumerable >::Item;
      type IntoIter = TheModule::EnumerableIteratorCopy< Self >;
      fn into_iter( self ) -> Self::IntoIter
      {
        TheModule::EnumerableIteratorCopy::new( self )
      }
    }

    impl< 'a > IntoIterator for &'a Pair
    {
      type Item = &'a < Pair as Enumerable >::Item;
      type IntoIter = TheModule::EnumerableIteratorRef< 'a, Pair >;
      fn into_iter( self ) -> Self::IntoIter
      {
        TheModule::EnumerableIteratorRef::new( self )
      }
    }

    /* test.case( "basic" ); */
    let pair = Pair( 13, 31 );
    a_id!( pair.len(), 2 );
    a_id!( pair.element_copy( 0 ), 13 );
    a_id!( pair.element_copy( 1 ), 31 );
    a_id!( pair.element( 0 ), &13 );
    a_id!( pair.element( 1 ), &31 );

    /* test.case( "consumable iterator" ); */
    let pair = Pair( 13, 31 );
    a_id!( pair.len(), 2 );
    for e in pair
    {
      println!( "{}", e );
    }
    // a_id!( pair.len(), 2 );

    /* test.case( "non-consumable iterator" ); */
    let pair = Pair( 13, 31 );
    a_id!( pair.len(), 2 );
    for e in &pair
    {
      println!( "{}", e );
    }
    a_id!( pair.len(), 2 );

  }

}

//

tests_index!
{
  basic,
}
