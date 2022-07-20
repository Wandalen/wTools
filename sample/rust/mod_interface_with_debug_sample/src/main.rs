
mod_interface::mod_interface!
{
  #![ debug ]
  /// Inner.
  layer inner;
}

//

fn main()
{
  /* test public namespaces */
  assert_eq!( prelude::inner_is(), true );
  assert_eq!( exposed::inner_is(), true );
  assert_eq!( orphan::inner_is(), true );
  assert_eq!( protected::inner_is(), true );

  /* test public module `inner` */
  assert_eq!( inner::prelude::inner_is(), true );
  assert_eq!( inner::exposed::inner_is(), true );
  assert_eq!( inner::orphan::inner_is(), true );
  assert_eq!( inner::protected::inner_is(), true );
}

// qqq : rewrite sample
