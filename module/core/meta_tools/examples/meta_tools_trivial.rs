//! This example showcases the usage of the `hmap!` macro from the `meta_tools` crate to create a hashmap and compares it with a hashmap created using `std::collections::HashMap`.
use meta_tools::*;

fn main()
{
  let meta_map = hmap! { 3 => 13 };
  let mut std_map = std::collections::HashMap::new();
  std_map.insert( 3, 13 );
  assert_eq!( meta_map, std_map );
}
