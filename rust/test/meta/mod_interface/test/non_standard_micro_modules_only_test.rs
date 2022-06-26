#[ allow( unused_imports ) ]
use test_tools::
{
  tests_impls,
  tests_index,
  a_id,
};

//

tests_impls!
{
  fn basic()
  {

    // {
    //   a_id!( private::has_private(), false );
    //   a_id!( private::has_protected(), false );
    //   a_id!( private::has_orphan(), false );
    //   a_id!( private::has_exposed(), false );
    //   a_id!( private::has_prelude(), false );
    // }

    {
      // a_id!( protected::mod_private::has_private(), true );
      a_id!( protected::mod_protected::has_protected(), true );
      a_id!( protected::mod_orphan::has_orphan(), true );
      a_id!( protected::mod_exposed::has_exposed(), true );
      a_id!( protected::mod_prelude::has_prelude(), true );
    }

    {
      // a_id!( orphan::mod_private::has_private(), true );
      // a_id!( orphan::mod_protected::has_protected(), true );
      a_id!( orphan::mod_orphan::has_orphan(), true );
      a_id!( orphan::mod_exposed::has_exposed(), true );
      a_id!( orphan::mod_prelude::has_prelude(), true );
    }

    {
      // a_id!( exposed::mod_private::has_private(), true );
      // a_id!( exposed::mod_protected::has_protected(), true );
      // a_id!( exposed::mod_orphan::has_orphan(), true );
      a_id!( exposed::mod_exposed::has_exposed(), true );
      a_id!( exposed::mod_prelude::has_prelude(), true );
    }

    {
      // a_id!( prelude::mod_private::has_private(), true );
      // a_id!( prelude::mod_protected::has_protected(), true );
      // a_id!( prelude::mod_orphan::has_orphan(), true );
      // a_id!( prelude::mod_exposed::has_exposed(), true );
      a_id!( prelude::mod_prelude::has_prelude(), true );
    }

  }
}

//

tests_index!
{
  basic,
}
