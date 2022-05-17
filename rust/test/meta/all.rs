
mod for_each_test
{
  use super::TheModule as TheModule;
  include!( "./all/for_each_test.rs" );
}

mod generator_test
{
  use super::TheModule as TheModule;
  include!( "./all/generator_test.rs" );
}

#[ path = "./impls_index/mod.rs" ]
mod impls;
