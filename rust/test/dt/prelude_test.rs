use super::*;

//

fn basic_test()
{
  use TheModule::prelude::*;

  /* test.case( "HashMap" ) */
  let src = HashMap::< i32, i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "Map" ) */
  let src = Map::< i32, i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "HashSet" ) */
  let src = HashSet::< i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "Set" ) */
  let src = Set::< i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "BTreeMap" ) */
  let src = BTreeMap::< i32, i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "BTreeSet" ) */
  let src = BTreeSet::< i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "BinaryHeap" ) */
  let src = BinaryHeap::< i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "LinkedList" ) */
  let src = LinkedList::< i32 >::new();
  a_true!( src.is_empty() );

  /* test.case( "VecDeque" ) */
  let src = VecDeque::< i32 >::new();
  a_true!( src.is_empty() );

}

//

test_suite!
{
  basic,
}
