// File: module/core/former_meta/src/derive_former/former_enum/unit.rs

use macro_tools::{ Result, proc_macro2::TokenStream, quote::quote, syn, diag };
use convert_case::{ Case, Casing };
use super::ident;
use syn::{ DeriveInput, Variant, Visibility, Generics, WhereClause, parse_quote }; // Added necessary syn items
use super::{ ItemAttributes, FieldAttributes, EnumVariantFieldInfo }; // Import types from super

#[ allow( clippy::too_many_arguments ) ] // Allow many arguments for handler functions
pub( super ) fn handle_unit_variant
(
  _ast : &DeriveInput,
  variant : &Variant,
  struct_attrs : &ItemAttributes,
  enum_name : &syn::Ident,
  vis : &Visibility,
  generics : &Generics,
  original_input : &proc_macro::TokenStream,
  has_debug : bool,
  methods : &mut Vec<TokenStream>,
  _end_impls : &mut Vec<TokenStream>, // Added end_impls
  standalone_constructors : &mut Vec<TokenStream>,
  variant_attrs : &FieldAttributes,
  _variant_field_info : &Vec<EnumVariantFieldInfo>,
  merged_where_clause : Option<&WhereClause>,
)
->
Result< () >
{
  let variant_ident = &variant.ident;
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  let method_name_ident_temp = parse_quote!( #method_name_snake_str );
  let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  // Check for #[subform_scalar] attribute
  if variant_attrs.subform_scalar.is_some()
  {
    return Err( syn::Error::new_spanned( variant, "#[subform_scalar] is not allowed on unit variants" ) );
  }

  // Generate the static method for the unit variant
  let method = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name #generics #merged_where_clause () -> #enum_name #generics.ty
    {
      #enum_name :: #variant_ident
    }
  };

  methods.push( method.clone() ); // Add to methods for the impl block

  // If #[standalone_constructors] is present on the struct, add the method to standalone constructors
  if struct_attrs.standalone_constructors.is_some()
  {
      standalone_constructors.push( method );
  }


  // Debug print if #[debug] is present on the enum
  if has_debug
  {
    let about = format!( "derive : Former\nenum : {enum_name}\nvariant : {variant_name_str}\nhandler : unit" );
    diag::report_print( about, original_input, &methods.last().unwrap() ); // Print the generated method
  }

  Ok( () )
}
