#[ allow( unused_imports ) ]
use clone_dyn::clone_dyn;

fn main()
{

  #[ clone_dyn ]
  trait Trait1
  {
  }

  let vec = Vec::< Box< dyn Trait1 > >::new();
  let _vec2 = vec.clone(); /* <- it does not work without `clone_dyn` */

}
