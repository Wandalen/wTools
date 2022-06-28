// use super::*;

//

tests_impls!
{
  fn basic()
  {

    {
      // a_id!( mod_private1::has_private1(), true );
      // a_id!( mod_private2::has_private2(), true );
      a_id!( mod_protected1::has_protected1(), true );
      a_id!( mod_protected2::has_protected2(), true );
      a_id!( mod_orphan1::has_orphan1(), true );
      a_id!( mod_orphan2::has_orphan2(), true );
      a_id!( mod_exposed1::has_exposed1(), true );
      a_id!( mod_exposed2::has_exposed2(), true );
      a_id!( mod_prelude1::has_prelude1(), true );
      a_id!( mod_prelude2::has_prelude2(), true );
    }

    {
      // a_id!( protected::mod_private1::has_private1(), true );
      // a_id!( protected::mod_private2::has_private2(), true );
      a_id!( protected::mod_protected1::has_protected1(), true );
      a_id!( protected::mod_protected2::has_protected2(), true );
      a_id!( protected::mod_orphan1::has_orphan1(), true );
      a_id!( protected::mod_orphan2::has_orphan2(), true );
      a_id!( protected::mod_exposed1::has_exposed1(), true );
      a_id!( protected::mod_exposed2::has_exposed2(), true );
      a_id!( protected::mod_prelude1::has_prelude1(), true );
      a_id!( protected::mod_prelude2::has_prelude2(), true );
    }

    {
      // a_id!( orphan::mod_private1::has_private1(), true );
      // a_id!( orphan::mod_private2::has_private2(), true );
      // a_id!( orphan::mod_protected1::has_protected1(), true );
      // a_id!( orphan::mod_protected2::has_protected2(), true );
      a_id!( orphan::mod_orphan1::has_orphan1(), true );
      a_id!( orphan::mod_orphan2::has_orphan2(), true );
      a_id!( orphan::mod_exposed1::has_exposed1(), true );
      a_id!( orphan::mod_exposed2::has_exposed2(), true );
      a_id!( orphan::mod_prelude1::has_prelude1(), true );
      a_id!( orphan::mod_prelude2::has_prelude2(), true );
    }

    {
      // a_id!( exposed::mod_private1::has_private1(), true );
      // a_id!( exposed::mod_private2::has_private2(), true );
      // a_id!( exposed::mod_protected1::has_protected1(), true );
      // a_id!( exposed::mod_protected2::has_protected2(), true );
      // a_id!( exposed::mod_orphan1::has_orphan1(), true );
      // a_id!( exposed::mod_orphan2::has_orphan2(), true );
      a_id!( exposed::mod_exposed1::has_exposed1(), true );
      a_id!( exposed::mod_exposed2::has_exposed2(), true );
      a_id!( exposed::mod_prelude1::has_prelude1(), true );
      a_id!( exposed::mod_prelude2::has_prelude2(), true );
    }

    {
      // a_id!( prelude::mod_private1::has_private1(), true );
      // a_id!( prelude::mod_private2::has_private2(), true );
      // a_id!( prelude::mod_protected1::has_protected1(), true );
      // a_id!( prelude::mod_protected2::has_protected2(), true );
      // a_id!( prelude::mod_orphan1::has_orphan1(), true );
      // a_id!( prelude::mod_orphan2::has_orphan2(), true );
      // a_id!( prelude::mod_exposed1::has_exposed1(), true );
      // a_id!( prelude::mod_exposed2::has_exposed2(), true );
      a_id!( prelude::mod_prelude1::has_prelude1(), true );
      a_id!( prelude::mod_prelude2::has_prelude2(), true );
    }

  }
}

//

tests_index!
{
  basic,
}
