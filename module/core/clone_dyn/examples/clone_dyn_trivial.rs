//! qqq : write proper description

#[ cfg( any( not( feature = "enabled" ), all( feature = "no_std", not( feature = "use_alloc" ) ) ) ) ]
fn main() {}

#[ cfg( all( feature = "enabled", any( not( feature = "no_std" ), feature = "use_alloc" ) ) ) ]
fn main()
{

  use clone_dyn::clone_dyn;

  #[ clone_dyn ]
  trait Trait1
  {
  }

  let vec = Vec::< Box< dyn Trait1 > >::new();
  let _vec2 = vec.clone(); /* <- it does not work without `clone_dyn` */

}
