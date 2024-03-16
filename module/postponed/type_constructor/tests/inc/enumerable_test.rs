#[ allow( unused_imports ) ]
use super::*;

//

macro_rules! PairDefine
{

  ()
  =>
  {

    struct Pair1( i32, i32 );
    impl the_module::Enumerable for Pair1
    {
      type Element = i32;
      fn len( &self ) -> usize
      {
        2
      }
      fn element_ref( &self, index : usize ) -> &Self::Element
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
      fn element_copy( &self, index : usize ) -> Self::Element
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
    // impl the_module::EnumerableMut for Pair1
    // {
    //   fn element_mut< 'slf, 'element >( &'slf mut self, index : usize ) -> &'element mut Self::Element
    //   where
    //     'element : 'slf,
    //   {
    //     debug_assert!( index < 2 );
    //     if index == 0
    //     {
    //       &mut self.0
    //     }
    //     else
    //     {
    //       &mut self.1
    //     }
    //   }
    // }

  };

}

//

tests_impls!
{

  fn basic()
  {
    use the_module::prelude::*;
    PairDefine!();

    /* test.case( "basic" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    a_id!( pair.element_copy( 0 ), 13 );
    a_id!( pair.element_copy( 1 ), 31 );
    a_id!( pair.element( 0 ), &13 );
    a_id!( pair.element( 1 ), &31 );

  }

  //

  fn manual_into_iter()
  {
    use the_module::prelude::*;
    PairDefine!();

    impl IntoIterator for Pair1
    {
      type Item = < Pair1 as Enumerable >::Element;
      type IntoIter = the_module::EnumerableIteratorCopy< Self >;
      fn into_iter( self ) -> Self::IntoIter
      {
        the_module::EnumerableIteratorCopy::new( self )
      }
    }

    impl< 'a > IntoIterator for &'a Pair1
    {
      type Item = &'a < Pair1 as Enumerable >::Element;
      type IntoIter = the_module::EnumerableIteratorRef< 'a, Pair1 >;
      fn into_iter( self ) -> Self::IntoIter
      {
        the_module::EnumerableIteratorRef::new( self )
      }
    }

    /* test.case( "consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    for e in pair
    {
      println!( "{}", e );
    }
    // a_id!( pair.len(), 2 );

    /* test.case( "consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    let got : Vec< _ > = pair.into_iter().collect();
    let exp = vec![ 13, 31 ];
    a_id!( got, exp );

    /* test.case( "non-consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    for e in &pair
    {
      println!( "{}", e );
    }
    a_id!( pair.len(), 2 );

    /* test.case( "non-consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    let got : Vec< _ > = ( &pair ).into_iter().cloned().collect();
    let exp = vec![ 13, 31 ];
    a_id!( got, exp );
    a_id!( pair.len(), 2 );

  }

  //

  fn enumerable_iterate_trait()
  {
    use the_module::prelude::*;
    PairDefine!();

    /* test.case( "consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    for e in pair.enumerable_iterate_consuming()
    {
      println!( "{}", e );
    }
    // a_id!( pair.len(), 2 );

    /* test.case( "consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    let got : Vec< _ > = pair.enumerable_iterate_consuming().collect();
    let exp = vec![ 13, 31 ];
    a_id!( got, exp );

    /* test.case( "non-consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    for e in pair.enumerable_iterate()
    {
      println!( "{}", e );
    }
    a_id!( pair.len(), 2 );

    /* test.case( "non-consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    let got : Vec< _ > = pair.enumerable_iterate().cloned().collect();
    let exp = vec![ 13, 31 ];
    a_id!( got, exp );
    a_id!( pair.len(), 2 );

  }

  //

  fn into_iterate_enumerable_iterate_trait()
  {
    use the_module::prelude::*;
    PairDefine!();

    impl IntoIterator for Pair1
    {
      type Item = < Pair1 as Enumerable >::Element;
      type IntoIter = the_module::EnumerableIteratorCopy< Self >;
      fn into_iter( self ) -> Self::IntoIter
      {
        the_module::EnumerableIteratorCopy::new( self )
      }
    }

    impl< 'a > IntoIterator for &'a Pair1
    {
      type Item = &'a < Pair1 as Enumerable >::Element;
      type IntoIter = the_module::EnumerableIteratorRef< 'a, Pair1 >;
      fn into_iter( self ) -> Self::IntoIter
      {
        the_module::EnumerableIteratorRef::new( self )
      }
    }

    /* test.case( "consumable iterator" ); */
    let pair = Pair1( 13, 31 );
    a_id!( pair.len(), 2 );
    for e in pair
    {
      println!( "{}", e );
    }
    // a_id!( pair.len(), 2 );

//     /* test.case( "consumable iterator" ); */
//     let pair = Pair1( 13, 31 );
//     a_id!( pair.len(), 2 );
//     let got : Vec< _ > = pair.into_iter().collect();
//     let exp = vec![ 13, 31 ];
//     a_id!( got, exp );
//
//     /* test.case( "non-consumable iterator" ); */
//     let pair = Pair1( 13, 31 );
//     a_id!( pair.len(), 2 );
//     for e in &pair
//     {
//       println!( "{}", e );
//     }
//     a_id!( pair.len(), 2 );
//
//     /* test.case( "non-consumable iterator" ); */
//     let pair = Pair1( 13, 31 );
//     a_id!( pair.len(), 2 );
//     let got : Vec< _ > = ( &pair ).into_iter().cloned().collect();
//     let exp = vec![ 13, 31 ];
//     a_id!( got, exp );
//     a_id!( pair.len(), 2 );

  }

}

//

tests_index!
{
  basic,
  manual_into_iter,
  enumerable_iterate_trait,
  into_iterate_enumerable_iterate_trait,
}
