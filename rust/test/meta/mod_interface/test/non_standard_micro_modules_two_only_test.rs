#[ allow( unused_imports ) ]
use test_tools::*;

//

tests_impls!
{
  #[ test ]
  fn basic()
  {

    // {
    //   assert_eq!( private::has_private(), false );
    //   assert_eq!( private::has_protected(), false );
    //   assert_eq!( private::has_orphan(), false );
    //   assert_eq!( private::has_exposed(), false );
    //   assert_eq!( private::has_prelude(), false );
    // }

    {
      // assert_eq!( protected::mod_private1::has_private1(), true );
      // assert_eq!( protected::mod_private2::has_private2(), true );
      assert_eq!( protected::mod_protected1::has_protected1(), true );
      assert_eq!( protected::mod_protected2::has_protected2(), true );
      assert_eq!( protected::mod_orphan1::has_orphan1(), true );
      assert_eq!( protected::mod_orphan2::has_orphan2(), true );
      assert_eq!( protected::mod_exposed1::has_exposed1(), true );
      assert_eq!( protected::mod_exposed2::has_exposed2(), true );
      assert_eq!( protected::mod_prelude1::has_prelude1(), true );
      assert_eq!( protected::mod_prelude2::has_prelude2(), true );
    }

    {
      // assert_eq!( protected::mod_private1::has_private1(), true );
      // assert_eq!( protected::mod_private2::has_private2(), true );
      // assert_eq!( protected::mod_protected1::has_protected1(), true );
      // assert_eq!( protected::mod_protected2::has_protected2(), true );
      assert_eq!( protected::mod_orphan1::has_orphan1(), true );
      assert_eq!( protected::mod_orphan2::has_orphan2(), true );
      assert_eq!( protected::mod_exposed1::has_exposed1(), true );
      assert_eq!( protected::mod_exposed2::has_exposed2(), true );
      assert_eq!( protected::mod_prelude1::has_prelude1(), true );
      assert_eq!( protected::mod_prelude2::has_prelude2(), true );
    }

    {
      // assert_eq!( protected::mod_private1::has_private1(), true );
      // assert_eq!( protected::mod_private2::has_private2(), true );
      // assert_eq!( protected::mod_protected1::has_protected1(), true );
      // assert_eq!( protected::mod_protected2::has_protected2(), true );
      // assert_eq!( protected::mod_orphan1::has_orphan1(), true );
      // assert_eq!( protected::mod_orphan2::has_orphan2(), true );
      assert_eq!( protected::mod_exposed1::has_exposed1(), true );
      assert_eq!( protected::mod_exposed2::has_exposed2(), true );
      assert_eq!( protected::mod_prelude1::has_prelude1(), true );
      assert_eq!( protected::mod_prelude2::has_prelude2(), true );
    }

    {
      // assert_eq!( protected::mod_private1::has_private1(), true );
      // assert_eq!( protected::mod_private2::has_private2(), true );
      // assert_eq!( protected::mod_protected1::has_protected1(), true );
      // assert_eq!( protected::mod_protected2::has_protected2(), true );
      // assert_eq!( protected::mod_orphan1::has_orphan1(), true );
      // assert_eq!( protected::mod_orphan2::has_orphan2(), true );
      // assert_eq!( protected::mod_exposed1::has_exposed1(), true );
      // assert_eq!( protected::mod_exposed2::has_exposed2(), true );
      assert_eq!( protected::mod_prelude1::has_prelude1(), true );
      assert_eq!( protected::mod_prelude2::has_prelude2(), true );
    }

  }
}

//

tests_index!
{
  basic,
}
