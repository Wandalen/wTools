// use super::*;

//

tests_impls!
{
  fn basic()
  {

    {
      // a_id!( protected::mod_private::has_private(), true );
      a_id!( mod_protected::has_protected(), true );
      a_id!( mod_orphan::has_orphan(), true );
      a_id!( mod_exposed::has_exposed(), true );
      a_id!( mod_prelude::has_prelude(), true );
    }

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
