// use super::*;

//

tests_impls!
{
  fn basic()
  {

    {
      // a_id!( own::mod_private::has_private(), true );
      a_id!( mod_own::has_own(), true );
      a_id!( mod_orphan::has_orphan(), true );
      a_id!( mod_exposed::has_exposed(), true );
      a_id!( mod_prelude::has_prelude(), true );
    }

    {
      // a_id!( own::mod_private::has_private(), true );
      a_id!( own::mod_own::has_own(), true );
      a_id!( own::mod_orphan::has_orphan(), true );
      a_id!( own::mod_exposed::has_exposed(), true );
      a_id!( own::mod_prelude::has_prelude(), true );
    }

    {
      // a_id!( orphan::mod_private::has_private(), true );
      // a_id!( orphan::mod_own::has_own(), true );
      a_id!( orphan::mod_orphan::has_orphan(), true );
      a_id!( orphan::mod_exposed::has_exposed(), true );
      a_id!( orphan::mod_prelude::has_prelude(), true );
    }

    {
      // a_id!( exposed::mod_private::has_private(), true );
      // a_id!( exposed::mod_own::has_own(), true );
      // a_id!( exposed::mod_orphan::has_orphan(), true );
      a_id!( exposed::mod_exposed::has_exposed(), true );
      a_id!( exposed::mod_prelude::has_prelude(), true );
    }

    {
      // a_id!( prelude::mod_private::has_private(), true );
      // a_id!( prelude::mod_own::has_own(), true );
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
