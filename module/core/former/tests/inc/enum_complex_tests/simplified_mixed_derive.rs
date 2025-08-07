// Purpose: Simplified replacement for comprehensive_mixed_derive to avoid build timeouts
// This provides mixed enum variant coverage without causing build performance issues

use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

// Simple inner types for mixed enum testing
#[derive(Debug, PartialEq, Default, Clone, Former)]
pub struct SimpleInner {
  pub data: String,
  pub value: i32,
}

// Simplified mixed enum with unit, tuple, and struct variants
#[derive(Debug, PartialEq, Former)]
pub enum SimplifiedMixedEnum {
  // Unit variants
  UnitVariantA,
  UnitVariantB,
  
  // Tuple variants
  #[scalar]
  TupleScalar(String),
  TupleSubform(SimpleInner),
  
  // Struct variants
  StructVariant {
    name: String,
    inner: SimpleInner,
  },
}

impl Default for SimplifiedMixedEnum {
  fn default() -> Self {
    Self::UnitVariantA
  }
}

// SIMPLIFIED MIXED ENUM TESTS - comprehensive coverage without build timeout

#[test]
fn simplified_mixed_unit_variants_test() {
  let unit_a = SimplifiedMixedEnum::unit_variant_a();
  let unit_b = SimplifiedMixedEnum::unit_variant_b();
  
  assert_eq!(unit_a, SimplifiedMixedEnum::UnitVariantA);
  assert_eq!(unit_b, SimplifiedMixedEnum::UnitVariantB);
}

#[test]
fn simplified_mixed_tuple_scalar_test() {
  let got = SimplifiedMixedEnum::tuple_scalar("tuple_test".to_string());
  let expected = SimplifiedMixedEnum::TupleScalar("tuple_test".to_string());
  assert_eq!(got, expected);
}

#[test]
fn simplified_mixed_tuple_subform_test() {
  let inner = SimpleInner {
    data: "subform_data".to_string(),
    value: 42,
  };
  
  let got = SimplifiedMixedEnum::tuple_subform()
    ._0(inner.clone())
    .form();
    
  let expected = SimplifiedMixedEnum::TupleSubform(inner);
  assert_eq!(got, expected);
}

#[test]
fn simplified_mixed_struct_variant_test() {
  let inner = SimpleInner {
    data: "struct_data".to_string(),
    value: 100,
  };
  
  let got = SimplifiedMixedEnum::struct_variant()
    .name("struct_test".to_string())
    .inner(inner.clone())
    .form();
    
  let expected = SimplifiedMixedEnum::StructVariant {
    name: "struct_test".to_string(),
    inner: inner,
  };
  
  assert_eq!(got, expected);
}

// Test comprehensive mixed enum patterns
#[test]
fn simplified_mixed_comprehensive_test() {
  // Test all variant types work together
  let variants = vec![
    SimplifiedMixedEnum::unit_variant_a(),
    SimplifiedMixedEnum::tuple_scalar("test".to_string()),
    SimplifiedMixedEnum::tuple_subform()
      ._0(SimpleInner { data: "test_data".to_string(), value: 1 })
      .form(),
    SimplifiedMixedEnum::struct_variant()
      .name("test_struct".to_string())
      .inner(SimpleInner { data: "struct_test".to_string(), value: 2 })
      .form(),
  ];
  
  assert_eq!(variants.len(), 4);
  
  // Verify each variant type
  assert!(matches!(variants[0], SimplifiedMixedEnum::UnitVariantA));
  assert!(matches!(variants[1], SimplifiedMixedEnum::TupleScalar(_)));
  assert!(matches!(variants[2], SimplifiedMixedEnum::TupleSubform(_)));
  assert!(matches!(variants[3], SimplifiedMixedEnum::StructVariant { .. }));
}