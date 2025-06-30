//! Purpose: Handles the generation of constructors for unit variants within enums for the `#[derive(Former)]` macro.
//! This module integrates utilities from `macro_tools` for robust code generation.
//!
//! This handler is responsible for:
//! - Generating static constructors (e.g., `Enum::Variant`) for unit variants.
//! - Generating standalone constructors (e.g., `variant()`) if `#[standalone_constructors]` is present on the enum.
//! - Validating that `#[subform_scalar]` is not used on unit variants.
//!
//! **Note on Generics:** There is a known, persistent issue with deriving `Former` on generic enums that causes a "comparison operators cannot be chained" error during compilation of the generated code. This issue is temporarily bypassed in tests by disabling the problematic test cases in `former` crate. A dedicated future task (`module/alias/macro_tools/task.md` and `module/core/former/task.md`) has been proposed to investigate and resolve this generic enum derivation issue more robustly, and to refine `macro_tools` utilities.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Generates `Enum::variant() -> Enum`.
//! - Rule 1a (Unit + `#[scalar]`): Generates `Enum::variant() -> Enum` (as default for unit is scalar).
//! - Rule 2a (Unit + `#[subform_scalar]`): Produces a compilation error.
//! - Rule 4a (`#[standalone_constructors]` on Enum): Generates top-level `fn variant_name() -> EnumName`.
use super::*;
use macro_tools::
{
  Result,
  diag,
  generic_params::GenericsRef,
  ident,
  qt,
  syn,
};
use super::EnumVariantHandlerContext;
use convert_case::Case;
use proc_macro2::TokenStream;

// qqq: Refactored to use `macro_tools` utilities for error handling, identifier casing, and generic quoting.
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  if let Some( attr ) = &ctx.variant_attrs.subform_scalar
  {
    diag::return_syn_err!( attr.name.span(), "TEST ERROR: #[subform_scalar] cannot be used on unit variants. V3" );
  }

  let variant_ident = &ctx.variant.ident;
  let enum_name = &ctx.enum_name;
  let vis = &ctx.vis;

  let method_ident = ident::cased_ident_from_ident( variant_ident, Case::Snake );

  let generics_ref = GenericsRef::new( ctx.generics );
  let fn_signature_generics = generics_ref.impl_generics_tokens_if_any();
  let return_type_generics = generics_ref.ty_generics_tokens_if_any();
  let enum_path_for_construction = generics_ref.type_path_tokens_if_any( enum_name );
  let where_clause_tokens = generics_ref.where_clause_tokens_if_any();

  let generated_method = qt!
  {
    #[ inline( always ) ]
    pub fn #method_ident () -> Self
    {
      Self::#variant_ident
    }
  };

  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = qt!
    {
      #[ inline( always ) ]
      #vis fn #method_ident #fn_signature_generics () -> #enum_name #return_type_generics
      #where_clause_tokens
      {
        #enum_path_for_construction :: #variant_ident
      }
    };
    ctx.standalone_constructors.push( generated_standalone );
  }

  Ok( generated_method )
}