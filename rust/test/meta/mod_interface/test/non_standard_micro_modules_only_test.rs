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
      // assert_eq!( protected::mod_private::has_private(), true );
      assert_eq!( protected::mod_protected::has_protected(), true );
      assert_eq!( protected::mod_orphan::has_orphan(), true );
      assert_eq!( protected::mod_exposed::has_exposed(), true );
      assert_eq!( protected::mod_prelude::has_prelude(), true );
    }

    {
      // assert_eq!( orphan::mod_private::has_private(), true );
      // assert_eq!( orphan::mod_protected::has_protected(), true );
      assert_eq!( orphan::mod_orphan::has_orphan(), true );
      assert_eq!( orphan::mod_exposed::has_exposed(), true );
      assert_eq!( orphan::mod_prelude::has_prelude(), true );
    }

    {
      // assert_eq!( exposed::mod_private::has_private(), true );
      // assert_eq!( exposed::mod_protected::has_protected(), true );
      // assert_eq!( exposed::mod_orphan::has_orphan(), true );
      assert_eq!( exposed::mod_exposed::has_exposed(), true );
      assert_eq!( exposed::mod_prelude::has_prelude(), true );
    }

    {
      // assert_eq!( prelude::mod_private::has_private(), true );
      // assert_eq!( prelude::mod_protected::has_protected(), true );
      // assert_eq!( prelude::mod_orphan::has_orphan(), true );
      // assert_eq!( prelude::mod_exposed::has_exposed(), true );
      assert_eq!( prelude::mod_prelude::has_prelude(), true );
    }

  }
}

//

tests_index!
{
  basic,
}
