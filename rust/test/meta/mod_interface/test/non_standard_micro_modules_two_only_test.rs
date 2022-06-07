#[ allow( unused_imports ) ]
use test_tools::*;

//

tests_impls!
{
  #[ test ]
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
      // a_id!( protected::mod_private1::has_private1(), true );
      // a_id!( protected::mod_private2::has_private2(), true );
      // a_id!( protected::mod_protected1::has_protected1(), true );
      // a_id!( protected::mod_protected2::has_protected2(), true );
      a_id!( protected::mod_orphan1::has_orphan1(), true );
      a_id!( protected::mod_orphan2::has_orphan2(), true );
      a_id!( protected::mod_exposed1::has_exposed1(), true );
      a_id!( protected::mod_exposed2::has_exposed2(), true );
      a_id!( protected::mod_prelude1::has_prelude1(), true );
      a_id!( protected::mod_prelude2::has_prelude2(), true );
    }

    {
      // a_id!( protected::mod_private1::has_private1(), true );
      // a_id!( protected::mod_private2::has_private2(), true );
      // a_id!( protected::mod_protected1::has_protected1(), true );
      // a_id!( protected::mod_protected2::has_protected2(), true );
      // a_id!( protected::mod_orphan1::has_orphan1(), true );
      // a_id!( protected::mod_orphan2::has_orphan2(), true );
      a_id!( protected::mod_exposed1::has_exposed1(), true );
      a_id!( protected::mod_exposed2::has_exposed2(), true );
      a_id!( protected::mod_prelude1::has_prelude1(), true );
      a_id!( protected::mod_prelude2::has_prelude2(), true );
    }

    {
      // a_id!( protected::mod_private1::has_private1(), true );
      // a_id!( protected::mod_private2::has_private2(), true );
      // a_id!( protected::mod_protected1::has_protected1(), true );
      // a_id!( protected::mod_protected2::has_protected2(), true );
      // a_id!( protected::mod_orphan1::has_orphan1(), true );
      // a_id!( protected::mod_orphan2::has_orphan2(), true );
      // a_id!( protected::mod_exposed1::has_exposed1(), true );
      // a_id!( protected::mod_exposed2::has_exposed2(), true );
      a_id!( protected::mod_prelude1::has_prelude1(), true );
      a_id!( protected::mod_prelude2::has_prelude2(), true );
    }

  }
}

//

tests_index!
{
  basic,
}
