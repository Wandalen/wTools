use super::*;
use macro_tools::{ attr, diag, type_struct, Result };

/// Implement `From< T >` for the entity structure, implying that `T` implement `Into< Field >` for each field and that each field has its unique type. It's part of process of type-based forming.
///
/// # Example of generated code
///
/// ```ignore
/// impl< T > From< T > for Options2
/// where
///   T : Into< i32 >,
///   T : Into< String >,
///   T : Clone,
/// {
///   #[ inline( always ) ]
///   fn from( src : T ) -> Self
///   {
///     let field1 = Into::< i32 >::into( src.clone() );
///     let field2 = Into::< String >::into( src.clone() );
///     Options2
///     {
///       field1,
///       field2,
///     }
///   }
/// }
/// ```

#[ inline ]
pub fn from_components( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let has_debug = attr::has_debug( parsed.item.attrs.iter() )?;

  // Struct name
  let item_name = parsed.item_name.clone();

  // Fields
  // let field_types : Vec< _ > = parsed.fields.iter().map( | field | &field.ty ).collect();

  // Generate snipets
  let trait_bounds = generate_trait_bounds( &parsed.field_types()[ .. ] );
  let field_assigns = generate_field_assigns( &parsed.fields_many() );

  // Struct initialization
  let field_names : Vec< _ > = parsed.fields.iter().map( | field | &field.ident ).collect();

  // Generate the From<T> trait implementation
  let result = qt!
  {
    impl< T > From< T > for #item_name
    where
      T : Clone,
      #( #trait_bounds )*
    {
      #[ inline( always ) ]
      fn from( src : T ) -> Self
      {
        #( #field_assigns )*
        Self
        {
          #( #field_names, )*
        }
      }
    }
  };

  if has_debug
  {
    diag::debug_report_print( "derive : FromComponents", original_input, &result );
  }
  Ok( result.into() )
}

#[ inline ]
fn generate_trait_bounds( field_types : &[ &syn::Type ] ) -> Vec< proc_macro2::TokenStream >
{
  field_types.iter().map( | field_type |
  {
    qt!
    {
      T : Into< #field_type >,
    }
  }).collect()
}

#[ inline ]
fn generate_field_assigns( fields : &[ &syn::Field ] ) -> Vec< proc_macro2::TokenStream >
{
  fields.iter().map( | field |
  {
    let field_ident = &field.ident;
    let field_type = &field.ty;
    qt!
    {
      let #field_ident = Into::< #field_type >::into( src.clone() );
    }
  }).collect()
}
