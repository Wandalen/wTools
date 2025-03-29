// use super::*;

//

tests_impls!
{
  fn basic()
  {

    /* test.case( "root" ); */
    {
      a_id!( layer_a::layer_a_own(), true );
      a_id!( layer_a::layer_a_orphan(), true );
      a_id!( layer_a::layer_a_exposed(), true );
      a_id!( layer_a::layer_a_prelude(), true );
    }

    /* test.case( "root" ); */
    {
      // a_id!( layer_a_own(), true );
      a_id!( layer_a_orphan(), true );
      a_id!( layer_a_exposed(), true );
      a_id!( layer_a_prelude(), true );
    }

    /* test.case( "protected" ); */
    {
      // a_id!( own::layer_a_own(), true );
      a_id!( own::layer_a_orphan(), true );
      a_id!( own::layer_a_exposed(), true );
      a_id!( own::layer_a_prelude(), true );
    }

    /* test.case( "orphan" ); */
    {
      // a_id!( orphan::layer_a_own(), true );
      // a_id!( orphan::layer_a_orphan(), true );
      a_id!( orphan::layer_a_exposed(), true );
      a_id!( orphan::layer_a_prelude(), true );
    }

    /* test.case( "exposed" ); */
    {
      // a_id!( exposed::layer_a_own(), true );
      // a_id!( exposed::layer_a_orphan(), true );
      a_id!( exposed::layer_a_exposed(), true );
      a_id!( exposed::layer_a_prelude(), true );
    }

    /* test.case( "prelude" ); */
    {
      // a_id!( prelude::layer_a_own(), true );
      // a_id!( prelude::layer_a_orphan(), true );
      // a_id!( prelude::layer_a_exposed(), true );
      a_id!( prelude::layer_a_prelude(), true );
    }

  }
}

//

tests_index!
{
  basic,
}
