
use super::*;
// Use re-exports from macro_tools
use macro_tools::{qt, attr, diag, Result, proc_macro2::TokenStream, syn::Index};

///
/// Generates implementations of the `Assign` trait for each field of a struct.
///
pub fn component_assign(input: proc_macro::TokenStream) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::<syn::ItemStruct>(input)?;
  let has_debug = attr::has_debug(parsed.attrs.iter())?;
  let item_name = &parsed.ident.clone();

  // Directly iterate over fields and handle named/unnamed cases
  let for_fields = match &parsed.fields {
    syn::Fields::Named(fields_named) => {
      fields_named.named.iter()
      .map( | field | for_each_field( field, None, item_name ) ) // Pass None for index
      .collect::< Result< Vec<  _  > > >()?
    }
    syn::Fields::Unnamed(fields_unnamed) => {
      fields_unnamed.unnamed.iter().enumerate()
      .map( |( index, field )| for_each_field( field, Some( index ), item_name ) ) // Pass Some(index)
      .collect::< Result< Vec<  _  > > >()?
    }
    syn::Fields::Unit => {
      // No fields to generate Assign for
      vec![]
    }
  };

  let result = qt! {
    #( #for_fields )*
  };

  if has_debug {
    let about = format!("derive : Assign\nstructure : {item_name}");
    diag::report_print(about, &original_input, &result);
  }

  Ok(result)
}

/// Generates an implementation of the `Assign` trait for a specific field of a struct.
///
/// This function creates the trait implementation that enables setting a struct's field value
/// with a type that can be converted into the field's type. It dynamically generates code
/// during the macro execution to provide `Assign` trait implementations for each field
/// of the struct, facilitating an ergonomic API for modifying struct instances.
///
/// # Parameters
///
/// - `field`: Reference to the struct field's metadata.
/// - `index`: `Some(usize)` for tuple fields, `None` for named fields.
/// - `item_name`: The name of the struct.
///
/// # Example of generated code for a tuple struct field
///
/// ```rust, ignore
/// impl< IntoT > Assign< i32, IntoT > for TupleStruct
/// where
///   IntoT : Into< i32 >,
/// {
///   #[ inline( always ) ]
///   fn assign( &mut self, component : IntoT )
///   {
///     self.0 = component.into(); // Uses index
///   }
/// }
/// ```
fn for_each_field(
  field: &syn::Field,
  index: Option< usize >, // Added index parameter
  item_name: &syn::Ident,
) -> Result< proc_macro2::TokenStream > {
  let field_type = &field.ty;

  // Construct the field accessor based on whether it's named or tuple
  let field_accessor: TokenStream = if let Some(ident) = &field.ident {
    // Named field: self.field_name
    quote! { #ident }
  } else if let Some(idx) = index {
    // Tuple field: self.0, self.1, etc.
    let index_lit = Index::from(idx);
    quote! { #index_lit }
  } else {
    // Should not happen if called correctly from `component_assign`
    return Err(syn::Error::new_spanned(field, "Field has neither ident nor index"));
  };

  Ok(qt! {
    #[ allow( non_snake_case ) ] // Still useful for named fields that might not be snake_case
    impl< IntoT > Assign< #field_type, IntoT > for #item_name
    where
      IntoT : Into< #field_type >,
    {
      #[ inline( always ) ]
      fn assign( &mut self, component : IntoT )
      {
        self.#field_accessor = component.into(); // Use the accessor
      }
    }
  })
}
