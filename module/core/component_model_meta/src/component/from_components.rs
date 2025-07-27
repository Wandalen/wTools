
use super::*;
// Use re-exports from macro_tools
use macro_tools::{attr, diag, item_struct, Result, proc_macro2::TokenStream};

///
/// Generates an implementation of the `From< T >` trait for a custom struct, enabling
/// type-based conversion from `T` to the struct. This function parses the given
/// `TokenStream` representing a struct, and produces code that allows for its
/// fields to be initialized from an instance of type `T`, assuming `T` can be
/// converted into each of the struct's field types.
///
/// # Example of generated code for a tuple struct
///
/// ```ignore
/// impl< T > From< T > for TargetTuple
/// where
///   T : Clone,
///   T : Into< i32 >,
///   T : Into< String >,
/// {
///   #[ inline( always ) ]
///   fn from( src : T ) -> Self
///   {
///     let field_0 = Into::< i32 >::into( src.clone() );
///     let field_1 = Into::< String >::into( src.clone() );
///     Self( field_0, field_1 ) // Uses tuple construction
///   }
/// }
/// ```
///
#[inline]
pub fn from_components(input: proc_macro::TokenStream) -> Result<proc_macro2::TokenStream> {
  let original_input = input.clone();
  let parsed = syn::parse::<syn::ItemStruct>(input)?;
  let has_debug = attr::has_debug(parsed.attrs.iter())?;

  // Struct name
  let item_name = &parsed.ident;

  // Generate snippets based on whether fields are named or unnamed
  let (field_assigns, final_construction): (Vec<TokenStream>, TokenStream) = match &parsed.fields {
    syn::Fields::Named(fields_named) => {
      let assigns = field_assign_named(fields_named.named.iter());
      let names: Vec<_> = fields_named.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
      let construction = quote! { Self { #( #names, )* } };
      (assigns, construction)
    }
    syn::Fields::Unnamed(fields_unnamed) => {
      let (assigns, temp_names) = field_assign_unnamed(fields_unnamed.unnamed.iter().enumerate());
      let construction = quote! { Self ( #( #temp_names, )* ) };
      (assigns, construction)
    }
    syn::Fields::Unit => {
      // No fields to assign, construct directly
      (vec![], quote! { Self })
    }
  };

  // Extract field types for trait bounds
  let field_types = item_struct::field_types(&parsed);
  let trait_bounds = trait_bounds(field_types);

  // Generate the From<T> trait implementation
  let result = qt! {
    impl< T > From< T > for #item_name
    where
      T : Clone,
      #( #trait_bounds )*
    {
      #[ inline( always ) ]
      fn from( src : T ) -> Self
      {
        #( #field_assigns )*
        #final_construction // Use the determined construction syntax
      }
    }
  };

  if has_debug {
    let about = format!("derive : FromComponents\nstructure : {0}", &parsed.ident);
    diag::report_print(about, &original_input, &result);
  }

  Ok(result)
}

/// Generates trait bounds for the `From< T >` implementation. (Same as before)
#[inline]
fn trait_bounds<'a>(field_types: impl macro_tools::IterTrait<'a, &'a syn::Type>) -> Vec<proc_macro2::TokenStream> {
  field_types
    .map(|field_type| {
      qt! {
        T : Into< #field_type >,
      }
    })
    .collect()
}

/// Generates assignment snippets for named fields.
#[inline]
fn field_assign_named<'a>(fields: impl Iterator<Item = &'a syn::Field>) -> Vec<proc_macro2::TokenStream> {
  fields
    .map(|field| {
      let field_ident = field.ident.as_ref().unwrap(); // Safe because we are in Named fields
      let field_type = &field.ty;
      qt! {
        let #field_ident = Into::< #field_type >::into( src.clone() );
      }
    })
    .collect()
}

/// Generates assignment snippets for unnamed fields and returns temporary variable names.
#[inline]
fn field_assign_unnamed<'a>(
  fields: impl Iterator<Item = (usize, &'a syn::Field)>,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::Ident>) {
  fields
    .map(|(index, field)| {
      let temp_var_name = format_ident!("field_{}", index); // Create temp name like field_0
      let field_type = &field.ty;
      let assign_snippet = qt! {
        let #temp_var_name = Into::< #field_type >::into( src.clone() );
      };
      (assign_snippet, temp_var_name)
    })
    .unzip() // Unzip into two vectors: assignments and temp names
}
