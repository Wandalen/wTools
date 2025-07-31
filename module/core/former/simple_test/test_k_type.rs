#[allow(unused_imports)]
use super::*;

#[derive(Debug, PartialEq, Default)]
pub struct Property<Name> {
  name: Name,
  code: isize,
}

#[derive(Debug, PartialEq, former::Former)]
pub struct Child<K: core::hash::Hash + core::cmp::Eq> {
  pub name: String,
  pub properties: collection_tools::HashMap<K, Property<K>>,
}

fn main() {
    // Test compilation
}