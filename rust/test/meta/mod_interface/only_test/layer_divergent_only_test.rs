// use super::*;

//

tests_impls!
{
  fn divergent()
  {

    /* test.case( "layer_b" ); */
    {
      a_id!( layer_b::protected::Vec::< i32 >::new(), layer_b::protected::Vec::< i32 >::new() );
      a_id!( layer_b::protected::SuperStruct1{}, layer_b::protected::SuperStruct1{} );
      a_id!( layer_b::protected::SuperStruct2{}, layer_b::protected::SuperStruct2{} );
    }

    /* test.case( "root" ); */
    {
      // a_id!( layer_a_orphan(), true );
    }

    /* test.case( "protected" ); */
    {
      // a_id!( protected::layer_a_orphan(), true );
    }

    /* test.case( "orphan" ); */
    {
    }

    /* test.case( "exposed" ); */
    {
    }

    /* test.case( "prelude" ); */
    {
    }

  }
}

//

tests_index!
{
  divergent,
}
