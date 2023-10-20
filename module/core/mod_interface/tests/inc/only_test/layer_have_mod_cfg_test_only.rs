tests_impls!
{
  fn mod_cfg()
  {

    a_true!( mod_a::fn_a() );
    a_true!( mod_b::fn_b() );
    // a_true!( mod_c::fn_c() );

  }
}

//

tests_index!
{
  mod_cfg,
}
