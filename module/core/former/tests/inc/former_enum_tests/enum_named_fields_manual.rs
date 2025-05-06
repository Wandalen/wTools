// File: module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs
use super::*;
use former::
{
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator,
};
use std::marker::PhantomData; // Added PhantomData

// Define the inner struct needed for subform tests directly in this file
#[derive(Debug, PartialEq, Default, Clone)] // No Former derive needed for manual test
pub struct InnerForSubform {
    pub value: i64,
}

// --- Manual Former for InnerForSubform ---
// ... (Keep the existing manual former for InnerForSubform as it was correct) ...
#[derive(Debug, Default)]
pub struct InnerForSubformFormerStorage { pub value: Option<i64> }
impl Storage for InnerForSubformFormerStorage { type Preformed = InnerForSubform; }
impl StoragePreform for InnerForSubformFormerStorage {
    fn preform(mut self) -> Self::Preformed { InnerForSubform { value: self.value.take().unwrap_or_default() } }
}
#[derive(Default, Debug)]
pub struct InnerForSubformFormerDefinitionTypes<C = (), F = InnerForSubform> { _p: PhantomData<(C, F)> }
impl<C, F> FormerDefinitionTypes for InnerForSubformFormerDefinitionTypes<C, F> {
    type Storage = InnerForSubformFormerStorage; type Context = C; type Formed = F;
}
impl<C, F> FormerMutator for InnerForSubformFormerDefinitionTypes<C, F> {}
#[derive(Default, Debug)]
pub struct InnerForSubformFormerDefinition<C = (), F = InnerForSubform, E = ReturnPreformed> { _p: PhantomData<(C, F, E)> }
impl<C, F, E> FormerDefinition for InnerForSubformFormerDefinition<C, F, E>
where E: FormingEnd<InnerForSubformFormerDefinitionTypes<C, F>> {
    type Storage = InnerForSubformFormerStorage; type Context = C; type Formed = F;
    type Types = InnerForSubformFormerDefinitionTypes<C, F>; type End = E;
}
pub struct InnerForSubformFormer<Definition = InnerForSubformFormerDefinition>
where Definition: FormerDefinition<Storage = InnerForSubformFormerStorage> {
    storage: Definition::Storage, context: Option<Definition::Context>, on_end: Option<Definition::End>,
}
impl<Definition> InnerForSubformFormer<Definition>
where Definition: FormerDefinition<Storage = InnerForSubformFormerStorage> {
    #[inline(always)] pub fn form(self) -> <Definition::Types as FormerDefinitionTypes>::Formed { self.end() }
    #[inline(always)] pub fn end(mut self) -> <Definition::Types as FormerDefinitionTypes>::Formed {
        let on_end = self.on_end.take().unwrap(); let context = self.context.take();
        <Definition::Types as FormerMutator>::form_mutation(&mut self.storage, &mut self.context);
        on_end.call(self.storage, context)
    }
    #[inline(always)] pub fn begin(storage: Option<Definition::Storage>, context: Option<Definition::Context>, on_end: Definition::End) -> Self {
        Self { storage: storage.unwrap_or_default(), context, on_end: Some(on_end) }
    }
    #[inline(always)] pub fn _new(on_end: Definition::End) -> Self { Self::begin(None, None, on_end) }
    #[inline] pub fn value(mut self, src: impl Into<i64>) -> Self { self.storage.value = Some(src.into()); self }
}
// --- End Manual Former for InnerForSubform ---


// Define the enum without the derive macro
#[ derive( Debug, PartialEq ) ]
pub enum EnumWithNamedFields // Renamed enum for clarity
{
  // Reordered to match derive file
  UnitVariantScalar, // New
  UnitVariantDefault, // Renamed

  VariantZeroScalar {},
  // VariantZeroDefault {}, // Error case - no manual impl needed

  VariantZeroUnnamedScalar(), // New
  VariantZeroUnnamedDefault(), // New

  VariantOneScalar { field_a : String },
  VariantOneSubform { field_b : InnerForSubform },
  VariantOneDefault { field_c : InnerForSubform },

  VariantTwoScalar { field_d : i32, field_e : bool },
  // VariantTwoDefault { field_f : i32, field_g : bool }, // Error case - no manual impl needed
}

// --- Manual Former Implementation ---

// --- Components for VariantOneSubform ---
#[derive(Default, Debug)] pub struct EnumWithNamedFieldsVariantOneSubformEnd;
impl FormingEnd<InnerForSubformFormerDefinitionTypes<(), EnumWithNamedFields>> for EnumWithNamedFieldsVariantOneSubformEnd {
    #[inline(always)] fn call(&self, sub_storage: InnerForSubformFormerStorage, _context: Option<()>) -> EnumWithNamedFields {
        EnumWithNamedFields::VariantOneSubform { field_b: sub_storage.preform() }
    }
}

// --- Components for VariantOneDefault ---
#[derive(Default, Debug)] pub struct EnumWithNamedFieldsVariantOneDefaultEnd;
impl FormingEnd<InnerForSubformFormerDefinitionTypes<(), EnumWithNamedFields>> for EnumWithNamedFieldsVariantOneDefaultEnd {
    #[inline(always)] fn call(&self, sub_storage: InnerForSubformFormerStorage, _context: Option<()>) -> EnumWithNamedFields {
        EnumWithNamedFields::VariantOneDefault { field_c: sub_storage.preform() }
    }
}

// --- Static Methods on the Enum ---
impl EnumWithNamedFields
{
  // --- Unit Variant ---
  #[ inline( always ) ]
  pub fn unit_variant_scalar() -> Self { Self::UnitVariantScalar } // New
  #[ inline( always ) ]
  pub fn unit_variant_default() -> Self { Self::UnitVariantDefault } // Renamed (Default is scalar)

  // --- Zero Fields (Named - Struct-like) ---
  #[ inline( always ) ]
  pub fn variant_zero_scalar() -> Self { Self::VariantZeroScalar {} }
  // No method for VariantZeroDefault (error case)

  // --- Zero Fields (Unnamed - Tuple-like) ---
  #[ inline( always ) ]
  pub fn variant_zero_unnamed_scalar() -> Self { Self::VariantZeroUnnamedScalar() } // New
  #[ inline( always ) ]
  pub fn variant_zero_unnamed_default() -> Self { Self::VariantZeroUnnamedDefault() } // New (Default is scalar)

  // --- One Field (Named - Struct-like) ---
  #[ inline( always ) ]
  pub fn variant_one_scalar( field_a : impl Into< String > ) -> Self { Self::VariantOneScalar { field_a: field_a.into() } }

  #[ inline( always ) ]
  pub fn variant_one_subform() -> InnerForSubformFormer<InnerForSubformFormerDefinition<(), Self, EnumWithNamedFieldsVariantOneSubformEnd>> {
      InnerForSubformFormer::begin(None, None, EnumWithNamedFieldsVariantOneSubformEnd::default())
  }

  #[ inline( always ) ]
  pub fn variant_one_default() -> InnerForSubformFormer<InnerForSubformFormerDefinition<(), Self, EnumWithNamedFieldsVariantOneDefaultEnd>> {
      InnerForSubformFormer::begin(None, None, EnumWithNamedFieldsVariantOneDefaultEnd::default())
  }

  // --- Two Fields (Named - Struct-like) ---
  #[ inline( always ) ]
  pub fn variant_two_scalar( field_d : impl Into< i32 >, field_e : impl Into< bool > ) -> Self {
      Self::VariantTwoScalar { field_d: field_d.into(), field_e: field_e.into() }
  }
  // No method for VariantTwoDefault (error case)

}

// Include the test logic file
include!( "enum_named_fields_only_test.rs" );