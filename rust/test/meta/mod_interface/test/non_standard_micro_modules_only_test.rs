
use test_tools::*;

//

fn basic_test()
{

  assert_eq!( private::slim_private(), false );
  assert_eq!( private::slim_protected(), false );
  assert_eq!( private::slim_orphan(), false );
  assert_eq!( private::slim_exposed(), false );
  assert_eq!( private::slim_prelude(), false );

  // assert_eq!( protected::slim_private::slim_private(), "slim_private" );
  assert_eq!( protected::slim_protected::slim_protected(), "slim_protected" );
  assert_eq!( protected::slim_orphan::slim_orphan(), "slim_orphan" );
  assert_eq!( protected::slim_exposed::slim_exposed(), "slim_exposed" );
  assert_eq!( protected::slim_prelude::slim_prelude(), "slim_prelude" );

  // assert_eq!( orphan::slim_private::slim_private(), "slim_private" );
  // assert_eq!( orphan::slim_protected::slim_protected(), "slim_protected" );
  assert_eq!( orphan::slim_orphan::slim_orphan(), "slim_orphan" );
  assert_eq!( orphan::slim_exposed::slim_exposed(), "slim_exposed" );
  assert_eq!( orphan::slim_prelude::slim_prelude(), "slim_prelude" );

  // assert_eq!( exposed::slim_private::slim_private(), "slim_private" );
  // assert_eq!( exposed::slim_protected::slim_protected(), "slim_protected" );
  // assert_eq!( exposed::slim_orphan::slim_orphan(), "slim_orphan" );
  assert_eq!( exposed::slim_exposed::slim_exposed(), "slim_exposed" );
  assert_eq!( exposed::slim_prelude::slim_prelude(), "slim_prelude" );

  // assert_eq!( prelude::slim_private::slim_private(), "slim_private" );
  // assert_eq!( prelude::slim_protected::slim_protected(), "slim_protected" );
  // assert_eq!( prelude::slim_orphan::slim_orphan(), "slim_orphan" );
  // assert_eq!( prelude::slim_exposed::slim_exposed(), "slim_exposed" );
  assert_eq!( prelude::slim_prelude::slim_prelude(), "slim_prelude" );

}

//

test_suite!
{
  basic,
}
