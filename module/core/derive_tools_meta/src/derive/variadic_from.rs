use super::*;
use macro_tools::{ Result, format_ident, attr, diag, qt };

#[ path = "from/field_attributes.rs" ]
mod field_attributes;
use field_attributes::*;
#[ path = "from/item_attributes.rs" ]
mod item_attributes;
use item_attributes::*;

/// The `derive_variadic_from` macro is designed to provide a way to implement the `From`-like
/// traits for structs with a variable number of fields, allowing them to be constructed from
/// tuples of different lengths or from individual arguments.
pub fn variadic_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let fields_parsed = match parsed.fields
  {
    syn::Fields::Named( ref fields ) => fields.named.clone(),
    syn::Fields::Unnamed( ref fields ) => fields.unnamed.clone(),
    syn::Fields::Unit => return_syn_err!( parsed.span(), "Expects a structure with fields" ),
  };

  if fields_parsed.len() > 3
  {
    return Ok( qt!{} );
  }

  let mut result = proc_macro2::TokenStream::new();

  // from!()
  if fields_parsed.is_empty()
  {
    result.extend( generate_empty( item_name ) );
  }

  // from!( 13 )
  if fields_parsed.len() == 1
  {
    let f1 = fields_parsed.iter().next().unwrap();
    let field_type1 = &f1.ty;
    result.extend( generate_single( item_name, field_type1 ) );
  }

  // from!( 13, 14 )
  if fields_parsed.len() == 2
  {
    let f1 = fields_parsed.iter().next().unwrap();
    let f2 = fields_parsed.iter().skip( 1 ).next().unwrap();
    let field_type1 = &f1.ty;
    let field_type2 = &f2.ty;
    result.extend( generate_two( item_name, field_type1, field_type2 ) );
  }

  // from!( 13, 14, 15 )
  if fields_parsed.len() == 3
  {
    let f1 = fields_parsed.iter().next().unwrap();
    let f2 = fields_parsed.iter().skip( 1 ).next().unwrap();
    let f3 = fields_parsed.iter().skip( 2 ).next().unwrap();
    let field_type1 = &f1.ty;
    let field_type2 = &f2.ty;
    let field_type3 = &f3.ty;
    result.extend( generate_three( item_name, field_type1, field_type2, field_type3 ) );
  }

  if has_debug
  {
    let about = format!( "derive : VariadicFrom\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `From` implementation for empty tuple.
fn generate_empty( item_name : &syn::Ident ) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl From< () > for #item_name
    {
      #[ inline( always ) ]
      fn from( _src : () ) -> Self
      {
        Self::default()
      }
    }
  }
}

/// Generates `From` implementation for a single field.
fn generate_single( item_name : &syn::Ident, field_type : &syn::Type ) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : #field_type ) -> Self
      {
        Self { a : src, ..Default::default() }
      }
    }
  }
}

/// Generates `From` implementation for two fields.
fn generate_two( item_name : &syn::Ident, field_type1 : &syn::Type, field_type2 : &syn::Type ) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl From< ( #field_type1, #field_type2 ) > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : ( #field_type1, #field_type2 ) ) -> Self
      {
        Self { a : src.0, b : src.1, ..Default::default() }
      }
    }
  }
}

/// Generates `From` implementation for three fields.
fn generate_three( item_name : &syn::Ident, field_type1 : &syn::Type, field_type2 : &syn::Type, field_type3 : &syn::Type ) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl From< ( #field_type1, #field_type2, #field_type3 ) > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : ( #field_type1, #field_type2, #field_type3 ) ) -> Self
      {
        Self { a : src.0, b : src.1, c : src.2, ..Default::default() }
      }
    }
  }
}
