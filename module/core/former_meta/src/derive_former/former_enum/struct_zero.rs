
// File: module/core/former_meta/src/derive_former/former_enum/struct_zero.rs
#![ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::
{
  Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  // diag, // Added for report_print // Removed unused import
  generic_params, // Added for decompose
  // ident, // Removed unused import // Removed unused import
  // phantom, // Added for phantom::tuple // Removed unused import
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing }; // Space before ;

/// Handles the generation of code for zero-field Struct enum variants.
#[ allow( clippy::too_many_lines ) ] // qqq : eliminate this
pub fn handle_struct_zero_variant< 'a > // Added explicit lifetime 'a
(
  _ast : &'a syn::DeriveInput, // Added lifetime 'a, Prefixed with _
  variant : &'a syn::Variant, // Added lifetime 'a
  struct_attrs : &'a ItemAttributes, // Added lifetime 'a
  enum_name : &'a syn::Ident, // Added lifetime 'a
  vis : &'a syn::Visibility, // Added lifetime 'a
  generics : &'a syn::Generics, // Added lifetime 'a
  _original_input : &'a proc_macro::TokenStream, // Added lifetime 'a, Prefixed with _
  _has_debug : bool, // Prefixed with _
  methods : &mut Vec<TokenStream>,
  _end_impls : &mut Vec<TokenStream>, // Prefixed with _
  standalone_constructors : &mut Vec<TokenStream>,
  variant_attrs : &'a FieldAttributes, // Added lifetime 'a
  _variant_field_info : &'a Vec<EnumVariantFieldInfo>, // Added lifetime 'a, Prefixed with _
  // Accept Option<&WhereClause> directly
  merged_where_clause : Option< &'a syn::WhereClause >,
) -> Result< () >
{
  println!( "DEBUG: Entering handle_struct_zero_variant for variant: {}", variant.ident ); // Debug print
  let variant_ident = &variant.ident;

  // Decompose generics within the function
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated ) // Use _ for unused where punctuated
  = generic_params::decompose( generics );
  // Use the passed Option<&WhereClause>
  let enum_generics_where = merged_where_clause;

  // Generate the snake_case method name, handling potential keywords
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
  let method_name = macro_tools::ident::ident_maybe_raw( &method_name_ident_temp ); // Use fully qualified path

  let wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter();
  let wants_subform_scalar = variant_attrs.subform_scalar.is_some();

  if wants_subform_scalar
  {
      return Err( syn::Error::new_spanned( variant, "#[subform_scalar] cannot be used on zero-field struct variants." ) );
  }
  else if wants_scalar // Default for Struct(0) is now an error, only #[scalar] works
  {
      // --- Scalar Struct(0) Variant ---
      // --- Standalone Constructor (Scalar Struct(0)) ---
      if struct_attrs.standalone_constructors.value( false )
      {
          let constructor_params : Vec<_> = _variant_field_info.iter().filter( |f| f.is_constructor_arg ).map( |f| { let pn = &f.ident; let ty = &f.ty; quote! { #pn : impl Into<#ty> } } ).collect();
          let return_type = quote! { #enum_name< #enum_generics_ty > };
          let constructor = quote!
          {
              /// Standalone constructor for the #variant_ident zero-field struct variant (scalar style).
              #[ inline( always ) ]
              #vis fn #method_name < #enum_generics_impl > ( #( #constructor_params ),* ) -> #return_type where #enum_generics_where
              { Self::#variant_ident {} }
          };
          standalone_constructors.push( constructor );
      }
      // --- End Standalone Constructor ---

      // Associated method (direct constructor)
      let static_method = quote!
      {
          /// Constructor for the #variant_ident zero-field struct variant (scalar style).
          #[ inline( always ) ]
          #vis fn #method_name() -> Self
          { Self::#variant_ident {} }
      };
      methods.push( static_method );
  }
  else // Default: Error
  {
     return Err( syn::Error::new_spanned( variant, "Former derive requires `#[scalar]` attribute for zero-field struct-like variants." ) );
  }

  Ok( () )
}
