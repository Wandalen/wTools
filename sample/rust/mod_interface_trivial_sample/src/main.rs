
mod_interface::mod_interface!
{
  /// Inner.
  layer inner;
}

//

fn main()
{
  /* test public module `inner` */
  assert_eq!( inner::prelude::inner_is(), true );

  /* test namespace `prelude` */
  assert_eq!( prelude::inner_is(), true );
  /* test namespace `exposed` */
  assert_eq!( exposed::inner_is(), true );
  /* test namespace `orphan` */
  assert_eq!( orphan::inner_is(), true );
  /* test namespace `protected` */
  assert_eq!( protected::inner_is(), true );
}

// qqq : rewrite sample
