#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "reflect_newtype" ) ]
// #[ path = "fundamental" ]
mod fundamental
{
  #[ allow( unused_imports ) ]
  use super::*;

  mod fields_test;
  mod fields_vec;
  mod fields_hmap;
  mod fields_bmap;

}

#[ cfg( feature = "reflect_newtype" ) ]
// #[ path = "group1" ]
mod group1
{
  #[ allow( unused_imports ) ]
  use super::*;

  mod newtype_experiment;

  mod common_test;
  mod primitive_test;
  mod struct_manual_test;
  mod struct_in_struct_manual_test;
  mod struct_with_lifetime_manual_test;
  mod slice_test;
  mod vec_test;
  mod hashset_test;
  mod hashmap_test;
  mod array_test;

}
