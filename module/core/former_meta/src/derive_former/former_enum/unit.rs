#![ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::
{
  Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote },
  // diag, // Added for report_print // Removed unused import
  generic_params, // Added for decompose
  // ident, // Removed unused import
  // phantom, // Added for phantom::tuple // Removed unused import
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing }; // Space before ;

/// Handles the generation of code for Unit enum variants.
#[ allow( clippy::too_many_lines ) ] // qqq : eliminate this
pub fn handle_unit_variant
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
  variant_field_info : &Vec<EnumVariantFieldInfo>,
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
  let method_name = macro_tools::ident::ident_maybe_raw( &method_name_ident_temp ); // Use fully qualified path

  let _wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter(); // Prefixed with _
  let wants_subform_scalar = variant_attrs.subform_scalar.is_some();

  // --- Error Handling ---
  if wants_subform_scalar
  {
    return Err( syn::Error::new_spanned( variant, "#[subform_scalar] cannot be used on unit variants." ) );
  }
  // #[scalar] is redundant but allowed, default is scalar.

  // --- Standalone Constructor (Unit) ---
  if struct_attrs.standalone_constructors.value( false )
  {
    if variant_attrs.arg_for_constructor.value( false )
    {
      return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied to a unit enum variant." ) );
    }
    // <<< Use collected info (empty for unit) to generate params >>>
    let _constructor_params : Vec<_> = variant_field_info // Will be empty // <<< Prefixed with _
      .iter()
      .filter( |f_info| f_info.is_constructor_arg )
      .map( |f_info| {
        let param_name = &f_info.ident; // Should not happen for unit
        let ty = &f_info.ty;
        quote! { #param_name : impl Into< #ty > }
      })
      .collect(); // <<< Added collect()
    // <<< End Use >>>

    // <<< Determine Return Type (Always Self for Unit) >>>
    let return_type = quote! { #enum_name< #enum_generics_ty > }; // qqq : check generics
    // <<< End Determine >>>

    let constructor = quote!
    {
      /// Standalone constructor for the #variant_ident unit variant.
      #[ inline( always ) ]
      #vis fn #method_name < #enum_generics_impl >() // qqq : check generics
      -> // Return type on new line
      #return_type // <<< Use determined return type
      where // Where clause on new line
        #enum_generics_where // qqq : check generics
      { // Brace on new line
        #enum_name::#variant_ident
      } // Brace on new line
    };
    standalone_constructors.push( constructor );
  }
  // --- End Standalone Constructor ---

  // Associated method (Default is scalar for Unit)
  let static_method = quote!
  {
    /// Constructor for the #variant_ident unit variant.
    #[ inline( always ) ]
    #vis fn #method_name() -> Self
    {
      Self::#variant_ident
    }
  };
  methods.push( static_method );

  Ok( () )
}