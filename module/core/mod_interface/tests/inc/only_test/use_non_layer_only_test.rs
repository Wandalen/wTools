// use super::*;

//

tests_impls!
{
  fn divergent()
  {

    // test.case( "CrateStructForTesting1" );
    {
      a_id!( layer_a::CrateStructForTesting1{}, layer_a::CrateStructForTesting1{} );
      a_id!( layer_a::own::CrateStructForTesting1{}, layer_a::own::CrateStructForTesting1{} );
    }

    // test.case( "SuperStruct" );
    {
      a_id!( layer_a::SuperStruct1{}, layer_a::SuperStruct1{} );
      a_id!( layer_a::own::SuperStruct1{}, layer_a::own::SuperStruct1{} );
    }

    // test.case( "Vec" );
    {
      a_id!( layer_a::Vec::< i32 >::new(), layer_a::Vec::< i32 >::new() );
      a_id!( layer_a::own::Vec::< i32 >::new(), layer_a::own::Vec::< i32 >::new() );
      a_id!( layer_a::orphan::Vec::< i32 >::new(), layer_a::orphan::Vec::< i32 >::new() );
      // a_id!( layer_a::exposed::Vec::< i32 >::new(), layer_a::exposed::Vec::< i32 >::new() );
      a_id!( Vec::< i32 >::new(), Vec::< i32 >::new() );
      a_id!( own::Vec::< i32 >::new(), own::Vec::< i32 >::new() );
      // a_id!( orphan::Vec::< i32 >::new(), orphan::Vec::< i32 >::new() );
    }

    // test.case( "SubStruct2" );
    {
      a_id!( layer_a::SubStruct2{}, layer_a::SubStruct2{} );
      a_id!( layer_a::own::SubStruct2{}, layer_a::own::SubStruct2{} );
      a_id!( layer_a::orphan::SubStruct2{}, layer_a::orphan::SubStruct2{} );
      // a_id!( layer_a::exposed::SubStruct2{}, layer_a::exposed::SubStruct2{} );
      a_id!( SubStruct2{}, SubStruct2{} );
      a_id!( own::SubStruct2{}, own::SubStruct2{} );
      // a_id!( orphan::SubStruct2{}, orphan::SubStruct2{} );
    }

    // test.case( "SubStruct2" );
    {
      a_id!( layer_a::SubStruct3{}, layer_a::SubStruct3{} );
      a_id!( layer_a::own::SubStruct3{}, layer_a::own::SubStruct3{} );
      a_id!( layer_a::orphan::SubStruct3{}, layer_a::orphan::SubStruct3{} );
      // a_id!( layer_a::exposed::SubStruct3{}, layer_a::exposed::SubStruct3{} );
      a_id!( SubStruct3{}, SubStruct3{} );
      a_id!( own::SubStruct3{}, own::SubStruct3{} );
      // a_id!( orphan::SubStruct3{}, orphan::SubStruct3{} );
    }

    // test.case( "SubStruct2" );
    {
      a_id!( layer_a::SubStruct4{}, layer_a::SubStruct4{} );
      a_id!( layer_a::own::SubStruct4{}, layer_a::own::SubStruct4{} );
      a_id!( layer_a::orphan::SubStruct4{}, layer_a::orphan::SubStruct4{} );
      // a_id!( layer_a::exposed::SubStruct4{}, layer_a::exposed::SubStruct4{} );
      a_id!( SubStruct4{}, SubStruct4{} );
      a_id!( own::SubStruct4{}, own::SubStruct4{} );
      // a_id!( orphan::SubStruct4{}, orphan::SubStruct4{} );
    }

    // test.case( "SubStruct2" );
    {
      a_id!( layer_a::PrivateStruct1{}, layer_a::PrivateStruct1{} );
      a_id!( layer_a::own::PrivateStruct1{}, layer_a::own::PrivateStruct1{} );
      a_id!( layer_a::orphan::PrivateStruct1{}, layer_a::orphan::PrivateStruct1{} );
      // a_id!( layer_a::exposed::PrivateStruct1{}, layer_a::exposed::PrivateStruct1{} );
      a_id!( PrivateStruct1{}, PrivateStruct1{} );
      a_id!( own::PrivateStruct1{}, own::PrivateStruct1{} );
      // a_id!( orphan::PrivateStruct1{}, orphan::PrivateStruct1{} );
    }

  }
}

//

tests_index!
{
  divergent,
}
