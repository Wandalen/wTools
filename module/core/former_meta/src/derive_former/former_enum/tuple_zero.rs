#![ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::
{
  Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  // diag, // Added for report_print // Removed unused import
  generic_params, // Added for decompose
  ident, // Added for ident_maybe_raw
  // phantom, // Added for phantom::tuple // Removed unused import
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing }; // Space before ;

/// Handles the generation of code for zero-field Tuple enum variants.
#[ allow( clippy::too_many_lines ) ] // qqq : eliminate this
pub fn handle_tuple_zero_variant
(
  _ast : &syn::DeriveInput, // Prefixed with _
  variant : &syn::Variant,
  struct_attrs : &ItemAttributes,
  enum_name : &syn::Ident,
  vis : &syn::Visibility,
  generics : &syn::Generics,
  _original_input : &proc_macro::TokenStream, // Prefixed with _
  _has_debug : bool, // Prefixed with _
  methods : &mut Vec<TokenStream>,
  _end_impls : &mut Vec<TokenStream>, // Prefixed with _
  standalone_constructors : &mut Vec<TokenStream>,
  variant_attrs : &FieldAttributes,
  _variant_field_info : &Vec<EnumVariantFieldInfo>, // Prefixed with _
  _merged_where_clause : &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>, // Prefixed with _
) -> Result< () >
{
  let variant_ident = &variant.ident;

  // Decompose generics within the function
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, enum_generics_where )
  = generic_params::decompose( generics );

  // Generate the snake_case method name, handling potential keywords
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
  let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  let _wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter(); // Prefixed with _
  let wants_subform_scalar = variant_attrs.subform_scalar.is_some();

  // Default behavior is scalar (direct constructor)
  // #[scalar] attribute is redundant but allowed
  if wants_subform_scalar
  {
     return Err( syn::Error::new_spanned( variant, "#[subform_scalar] cannot be used on zero-field tuple variants." ) );
  }

  // --- Standalone Constructor (Zero Tuple) ---
  if struct_attrs.standalone_constructors.value( false )
  {
    // ... (logic similar to Unit variant standalone constructor) ...
    let return_type = quote! { #enum_name< #enum_generics_ty > };
    let constructor = quote!
    {
      /// Standalone constructor for the #variant_ident zero-field tuple variant.
      #[ inline( always ) ]
      #vis fn #method_name < #enum_generics_impl >()
      -> #return_type
      where #enum_generics_where
      { Self::#variant_ident() }
    };
    standalone_constructors.push( constructor );
  }
  // --- End Standalone Constructor ---

  // Associated method (direct constructor)
  let static_method = quote!
  {
    /// Constructor for the #variant_ident zero-field tuple variant.
    #[ inline( always ) ]
    #vis fn #method_name() -> Self
    {
      Self::#variant_ident()
    }
  };
  methods.push( static_method );

  Ok( () )
}