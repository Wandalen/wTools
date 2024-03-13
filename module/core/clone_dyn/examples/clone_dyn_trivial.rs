//! Demonstrates the usage of `clone_dyn` to enable cloning for trait objects.
//!
//! By default, Rust does not support cloning for trait objects due to the `Clone` trait
//! requiring compile-time knowledge of the type's size. The `clone_dyn` crate addresses
//! this limitation through procedural macros, allowing for cloning collections of trait objects.

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
