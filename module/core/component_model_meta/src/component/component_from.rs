#[allow(clippy::wildcard_imports)]
use super::*;
use macro_tools::{attr, diag, Result, proc_macro2::TokenStream, syn::Index};

/// Generates `From` implementations for each unique component (field) of the structure.
pub fn component_from(input: proc_macro::TokenStream) -> Result<proc_macro2::TokenStream> {
  let original_input = input.clone();
  let parsed = syn::parse::<syn::ItemStruct>(input)?;
  let has_debug = attr::has_debug(parsed.attrs.iter())?;
  let item_name = &parsed.ident;

  // Directly iterate over fields and handle named/unnamed cases
  let for_fields = match &parsed.fields {
    syn::Fields::Named(fields_named) => {
      fields_named.named.iter()
      .map( | field | for_each_field( field, None, item_name ) ) // Pass None for index
      .collect::< Result< Vec< _ > > >()?
    }
    syn::Fields::Unnamed(fields_unnamed) => {
      fields_unnamed.unnamed.iter().enumerate()
      .map( |( index, field )| for_each_field( field, Some( index ), item_name ) ) // Pass Some(index)
      .collect::< Result< Vec< _ > > >()?
    }
    syn::Fields::Unit => {
      // No fields to generate From for
      vec![]
    }
  };

  let result = qt! {
    #( #for_fields )*
  };

  if has_debug {
    let about = format!("derive : ComponentFrom\nstructure : {item_name}");
    diag::report_print(about, &original_input, &result);
  }

  Ok(result)
}

/// Generates a `From` implementation for a specific field of a struct.
///
/// # Arguments
///
/// * `field` - A reference to the field for which to generate the `From` implementation.
/// * `index`: `Some(usize)` for tuple fields, `None` for named fields.
/// * `item_name` - The name of the structure containing the field.
///
/// # Example of generated code for a tuple struct field
///
/// ```rust, ignore
/// impl From< &TupleStruct > for i32
/// {
///   #[ inline( always ) ]
///   fn from( src : &TupleStruct ) -> Self
///   {
///     src.0.clone() // Uses index
///   }
/// }
/// ```
fn for_each_field(
  field: &syn::Field,
  index: Option<usize>, // Added index parameter
  item_name: &syn::Ident,
) -> Result<proc_macro2::TokenStream> {
  let field_type = &field.ty;

  // Construct the field accessor based on whether it's named or tuple
  let field_accessor: TokenStream = if let Some(ident) = &field.ident {
    // Named field: src.field_name
    quote! { #ident }
  } else if let Some(idx) = index {
    // Tuple field: src.0, src.1, etc.
    let index_lit = Index::from(idx);
    quote! { #index_lit }
  } else {
    // Should not happen if called correctly from `component_from`
    return Err(syn::Error::new_spanned(field, "Field has neither ident nor index"));
  };

  Ok(qt! {
    // Removed #[ allow( non_local_definitions ) ] as it seems unnecessary here
    impl From< &#item_name > for #field_type
    {
      #[ inline( always ) ]
      fn from( src : &#item_name ) -> Self
      {
        // Use src.#field_accessor instead of self.#field_accessor
        src.#field_accessor.clone()
      }
    }
  })
}
