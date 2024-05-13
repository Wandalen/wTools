use super::*;
use macro_tools::{ attr, diag, type_struct, Result };

///
/// Generates implementations of the `ComponentAssign` trait for each field of a struct.
///
pub fn component_assign( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let has_debug = attr::has_debug( parsed.item.attrs.iter() )?;

  let for_field = parsed.fields_many().iter().map( | field |
  {
    for_each_field( field, &parsed.item_name )
  })
  .collect::< Result< Vec< _ > > >()?;

  let result = qt!
  {
    #( #for_field )*
  };

  if has_debug
  {
    let about = format!( "derive : ComponentAssign\nstructure : {0}", &parsed.item_name );
    diag::report_print( about, &original_input, &result );
  }

  // if has_debug
  // {
  //   diag::report_print( "derive : ComponentAssign", original_input, &result );
  // }

  Ok( result )
}

/// Generates an implementation of the `ComponentAssign` trait for a specific field of a struct.
///
/// This function creates the trait implementation that enables setting a struct's field value
/// with a type that can be converted into the field's type. It dynamically generates code
/// during the macro execution to provide `ComponentAssign` trait implementations for each field
/// of the struct, facilitating an ergonomic API for modifying struct instances.
///
/// # Parameters
///
/// - `field`: Reference to the struct field's metadata.
/// - `item_name`: The name of the struct.
///
/// # Example of generated code
///
/// ```rust, ignore
/// impl< IntoT > former::ComponentAssign< i32, IntoT > for Options1
/// where
///   IntoT : Into< i32 >,
/// {
///   #[ inline( always ) ]
///   fn assign( &mut self, component : IntoT )
///   {
///     self.field1 = component.into().clone();
///   }
/// }
/// ```
fn for_each_field( field : &syn::Field, item_name : &syn::Ident ) -> Result< proc_macro2::TokenStream >
{
  let field_name = field.ident.as_ref()
  .ok_or_else( || syn::Error::new( field.span(), "Field without a name" ) )?;
  let field_type = &field.ty;

  Ok( qt!
  {
    #[ allow( non_snake_case ) ]
    impl< IntoT > ComponentAssign< #field_type, IntoT > for #item_name
    where
      IntoT : Into< #field_type >,
    {
      #[ inline( always ) ]
      fn assign( &mut self, component : IntoT )
      {
        self.#field_name = component.into();
      }
    }
  })
}
