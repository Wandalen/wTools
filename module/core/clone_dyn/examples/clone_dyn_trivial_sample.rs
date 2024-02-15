//! example

#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
use clone_dyn::clone_dyn;

fn main()
{

  #[ cfg( feature = "enabled" ) ]
  {

    #[ clone_dyn ]
    trait Trait1
    {
    }

    let vec = Vec::< Box< dyn Trait1 > >::new();
    let _vec2 = vec.clone(); /* <- it does not work without `clone_dyn` */

  }

}
