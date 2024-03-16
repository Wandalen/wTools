
// use super::*;
// use macro_tools::{ attr, diag, type_struct, Result };

// /// Generates `From` implementations for each unique component (field) of the structure.
// pub fn component_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
// {
//   let original_input = input.clone();
//   let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
//   let has_debug = attr::has_debug( parsed.item.attrs.iter() )?;
//
//   let for_field = parsed.fields_many().iter().map( | field |
//   {
//     for_each_field( field, &parsed.item_name )
//   })
//   .collect::< Result< Vec< _ >  > >()?;
//
//   let result = qt!
//   {
//     #( #for_field )*
//   };
//
//   if has_debug
//   {
//     diag::debug_report_print( "derive : ComponentFrom", original_input, &result );
//   }
//
//   Ok( result )
// }
//
// /// Generates a `From` implementation for a specific field of a struct.
//
// fn for_each_field( field : &syn::Field, item_name : &syn::Ident ) -> Result< proc_macro2::TokenStream >
// {
//   let field_name = field.ident.as_ref()
//   .ok_or_else( || syn::Error::new( field.span(), "Field without a name" ) )?;
//   let field_type = &field.ty;
//
//   Ok( qt!
//   {
//     #[ allow( non_local_definitions ) ]
//     impl From< &#item_name > for #field_type
//     {
//       #[ inline( always ) ]
//       fn from( src : &#item_name ) -> Self
//       {
//         src.#field_name.clone()
//       }
//     }
//   })
// }
