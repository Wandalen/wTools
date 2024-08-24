#[ allow( unused_imports ) ]
use super::*;

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "prelude" ) ]
tests_impls!
{
  fn basic()
  {
    use the_module::prelude::*;

    /* test.case( "Vec" ) */
    let src = Vec::< i32 >::new();
    a_true!( src.is_empty() );

    /* test.case( "DynList" ) */
    let src = DynList::< i32 >::new();
    a_true!( src.is_empty() );

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
}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "prelude" ) ]
tests_index!
{
  basic,
}
