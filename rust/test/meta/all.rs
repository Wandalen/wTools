
mod for_each_test
{
  use super::TheModule as TheModule;
  include!( "./all/for_each_test.rs" );
}

#[ path = "./all/generator_test.rs" ]
mod generator_test;

#[ path = "./impls_index/mod.rs" ]
mod impls;
