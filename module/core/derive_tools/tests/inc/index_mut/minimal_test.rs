use super::*;
use test_tools::prelude::*;
use core::ops::{Index, IndexMut};
use derive_tools::IndexMut;

#[derive(IndexMut)]
pub struct TupleStruct1(#[index_mut] pub i32);

#[test]
fn test_tuple_struct1() {
  let mut instance = TupleStruct1(123);
  assert_eq!(instance[0], 123);
  instance[0] = 456;
  assert_eq!(instance[0], 456);
}
