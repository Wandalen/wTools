use super::*;

#[allow(dead_code)]
#[derive(the_module::Index)]
struct StructNamed<T>(T, T);

include!("./only_test/struct_tuple.rs");
