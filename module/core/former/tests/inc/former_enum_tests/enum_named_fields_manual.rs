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

  // Manual implementation of standalone constructor for S0.4
  // #[ inline( always ) ]
  // pub fn standalone_variant_zero_scalar() -> Self { Self::VariantZeroScalar {} }

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

  // Manual implementation of standalone constructor for S1.4
  // #[ inline( always ) ]
  // pub fn standalone_variant_one_default() -> InnerForSubformFormer<InnerForSubformFormerDefinition<(), Self, EnumWithNamedFieldsVariantOneDefaultEnd>> {
  //     InnerForSubformFormer::begin(None, None, EnumWithNamedFieldsVariantOneDefaultEnd::default())
  // }

  // Manual implementation of standalone constructor for S1.5
  // #[ inline( always ) ]
  // pub fn standalone_variant_one_scalar( field_a : impl Into< String > ) -> Self { Self::VariantOneScalar { field_a: field_a.into() } }

  // Manual implementation of standalone constructor for S1.6
  // #[ inline( always ) ]
  // pub fn standalone_variant_one_subform() -> InnerForSubformFormer<InnerForSubformFormerDefinition<(), Self, EnumWithNamedFieldsVariantOneSubformEnd>> {
  //     InnerForSubformFormer::begin(None, None, EnumWithNamedFieldsVariantOneSubformEnd::default())
  // }

  // Manual implementation of standalone constructor for S1.7 (assuming #[arg_for_constructor] on field_a)
  // This case is tricky for manual implementation as it depends on the macro's arg_for_constructor logic.
  // A simplified manual equivalent might be a direct constructor.
  // Let's add a direct constructor as a placeholder, noting it might differ from macro output.
  // qqq : Manual implementation for S1.7 might not perfectly match macro output due to arg_for_constructor complexity.
  // #[ inline( always ) ]
  // pub fn standalone_variant_one_default_with_arg( field_c : impl Into< InnerForSubform > ) -> Self {
  //     Self::VariantOneDefault { field_c: field_c.into() }
  // }


  // --- Two Fields (Named - Struct-like) ---
  #[ inline( always ) ]
  pub fn variant_two_scalar( field_d : impl Into< i32 >, field_e : impl Into< bool > ) -> Self {
      Self::VariantTwoScalar { field_d: field_d.into(), field_e: field_e.into() }
  }
  // No method for VariantTwoDefault (error case)

  // Manual implementation of standalone constructor for SN.4
  // #[ inline( always ) ]
  // pub fn standalone_variant_two_default() -> InnerForSubformFormer<InnerForSubformFormerDefinition<(), Self, EnumWithNamedFieldsVariantTwoDefaultEnd>> {
  //     // qqq : Need to define EnumWithNamedFieldsVariantTwoDefaultEnd for this manual impl
  //     // For now, using InnerForSubformFormerDefinition as a placeholder.
  //     // This will likely cause a compilation error until the correct End struct is defined.
  //     InnerForSubformFormer::begin(None, None, InnerForSubformFormerDefinition::<(), Self, EnumWithNamedFieldsVariantTwoDefaultEnd>::default())
  // }

  // Manual implementation of standalone constructor for SN.5
  // #[ inline( always ) ]
  // pub fn standalone_variant_two_scalar( field_d : impl Into< i32 >, field_e : impl Into< bool > ) -> Self {
  //     Self::VariantTwoScalar { field_d: field_d.into(), field_e: field_e.into() }
  // }

  // Manual implementation of standalone constructor for SN.6
  // #[ inline( always ) ]
  // pub fn standalone_variant_two_subform() -> InnerForSubformFormer<InnerForSubformFormerDefinition<(), Self, EnumWithNamedFieldsVariantTwoSubformEnd>> {
  //     // qqq : Need to define EnumWithNamedFieldsVariantTwoSubformEnd for this manual impl
  //     // For now, using InnerForSubformFormerDefinition as a placeholder.
  //     // This will likely cause a compilation error until the correct End struct is defined.
  //     InnerForSubformFormer::begin(None, None, EnumWithNamedFieldsVariantTwoSubformEnd::default())
  // }

  // Manual implementation of standalone constructor for SN.7 (assuming #[arg_for_constructor] on some fields)
  // Similar to S1.7, this is complex for manual implementation.
  // Let's add a direct constructor with all fields as args as a placeholder.
  // qqq : Manual implementation for SN.7 might not perfectly match macro output due to arg_for_constructor complexity.
  // #[ inline( always ) ]
  // pub fn standalone_variant_two_default_with_args( field_d : impl Into< i32 >, field_e : impl Into< bool > ) -> Self {
  //     Self::VariantOneDefault { field_d: field_d.into(), field_e: field_e.into() }
  // }


}

// qqq : Need to define EnumWithNamedFieldsVariantTwoDefaultEnd and EnumWithNamedFieldsVariantTwoSubformEnd for manual impls
// Placeholder definitions to avoid immediate compilation errors
// #[derive(Default, Debug)] pub struct EnumWithNamedFieldsVariantTwoDefaultEnd;
// impl FormingEnd<InnerForSubformFormerDefinitionTypes<(), EnumWithNamedFields>> for EnumWithNamedFieldsVariantTwoDefaultEnd {
//     #[inline(always)] fn call(&self, sub_storage: InnerForSubformFormerStorage, _context: Option<()>) -> EnumWithNamedFields {
//         // qqq : This implementation is incorrect, needs to handle the actual fields of VariantTwoDefault
//         // This will likely require a different approach or a dedicated manual struct for VariantTwoDefault's former.
//         // For now, returning a placeholder variant.
//         EnumWithNamedFields::UnitVariantScalar // Placeholder
//     }
// }

// #[derive(Default, Debug)] pub struct EnumWithNamedFieldsVariantTwoSubformEnd;
// impl FormingEnd<InnerForSubformFormerDefinitionTypes<(), EnumWithNamedFields>> for EnumWithNamedFieldsVariantTwoSubformEnd {
//     #[inline(always)] fn call(&self, sub_storage: InnerForSubformFormerStorage, _context: Option<()>) -> EnumWithNamedFields {
//         // qqq : This implementation is incorrect, needs to handle the actual fields of VariantTwoSubform
//         // This will likely require a different approach or a dedicated manual struct for VariantTwoSubform's former.
//         // For now, returning a placeholder variant.
//         EnumWithNamedFields::UnitVariantScalar // Placeholder
//     }
// }


// Include the test logic file
include!( "enum_named_fields_only_test.rs" );