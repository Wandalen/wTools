#[ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::{ attr, diag, item_struct, Result };

///
/// Generates an implementation of the `From< T >` trait for a custom struct, enabling
/// type-based conversion from `T` to the struct. This function parses the given
/// `TokenStream` representing a struct, and produces code that allows for its
/// fields to be initialized from an instance of type `T`, assuming `T` can be
/// converted into each of the struct's field types.
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
///

#[ inline ]
pub fn from_components( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;

  // Struct name
  let item_name = &parsed.ident;

  // Generate snipets
  let trait_bounds = trait_bounds( item_struct::field_types( &parsed ) );
  let field_assigns = field_assign( parsed.fields.iter() );
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
    let about = format!( "derive : FromComponents\nstructure : {0}", &parsed.ident );
    diag::report_print( about, &original_input, &result );
  }

  // if has_debug
  // {
  //   diag::report_print( "derive : FromComponents", original_input, &result );
  // }

  Ok( result )
}

/// Generates trait bounds for the `From< T >` implementation, ensuring that `T`
/// can be converted into each of the struct's field types. This function
/// constructs a sequence of trait bounds necessary for the `From< T >`
/// implementation to compile.
///
/// # Example of generated code
///
/// Given field types `[i32, String]`, this function generates:
///
/// ```ignore
/// T : Into< i32 >,
/// T : Into< String >,
/// ```
///
/// These trait bounds are then used in the `From<T>` implementation to ensure type compatibility.

#[ inline ]
// fn trait_bounds( field_types : &[ &syn::Type ] ) -> Vec< proc_macro2::TokenStream >
fn trait_bounds< 'a >( field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type > ) -> Vec< proc_macro2::TokenStream >
{
  field_types.map( | field_type |
  {
    qt!
    {
      T : Into< #field_type >,
    }
  }).collect()
}

/// Generates code snippets for converting `T` into each of the struct's fields
/// inside the `from` function of the `From<T>` trait implementation. This function
/// creates a series of statements that clone the source `T`, convert it into the
/// appropriate field type, and assign it to the corresponding field of the struct.
///
/// # Example of generated code
///
/// For a struct with fields `field1: i32` and `field2: String`, this function generates:
///
/// ```ignore
/// let field1 = Into::< i32 >::into( src.clone() );
/// let field2 = Into::< String >::into( src.clone() );
/// ```
///

#[ inline ]
fn field_assign< 'a >( fields : impl Iterator< Item = &'a syn::Field > ) -> Vec< proc_macro2::TokenStream >
{
  fields.map( | field |
  {
    let field_ident = &field.ident;
    let field_type = &field.ty;
    qt!
    {
      let #field_ident = Into::< #field_type >::into( src.clone() );
    }
  }).collect()
}
