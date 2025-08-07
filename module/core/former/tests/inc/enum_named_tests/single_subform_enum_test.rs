//! Test for single subform enum (should work without trait conflicts)
use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

#[derive(Debug, PartialEq, Default, Clone, Former)]
pub struct InnerStruct {
    pub value: i64,
}

#[derive(Debug, PartialEq, Former)]
pub enum SingleSubformEnum
{
  #[subform_scalar]
  OnlySubform { field: InnerStruct },
}

#[test]
fn single_subform_enum_test()
{
  let got = SingleSubformEnum::only_subform()
    .field(InnerStruct { value: 42 })
    .form();
  let expected = SingleSubformEnum::OnlySubform { field: InnerStruct { value: 42 } };
  assert_eq!(got, expected);
}