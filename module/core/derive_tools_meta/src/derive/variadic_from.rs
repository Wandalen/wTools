
use super::*;
use macro_tools::{ Result, format_ident, attr, diag };
use iter::{ IterExt, Itertools };

/// This function generates an implementation of a variadic `From` trait for a given struct.
/// It handles both named and unnamed fields within the struct, generating appropriate code
/// for converting a tuple of fields into an instance of the struct.

pub fn variadic_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let len = parsed.fields.len();
  let from_trait = format_ident!( "From{len}",  );
  let from_method = format_ident!( "from{len}" );

  let
  (
    types,
    fn_params,
    src_into_vars,
    vars
  )
  :
  ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = parsed.fields.iter().enumerate().map_result( | ( i, field ) |
  {
    let ident = field.ident.clone().map_or_else( || format_ident!( "_{i}" ), | e | e );
    let ty = field.ty.clone();
    Result::Ok
    ((
      qt!{ #ty, },
      qt!{ #ident : #ty, },
      qt!{ let #ident = ::core::convert::Into::into( #ident ); },
      qt!{ #ident, },
    ))
  })?
  .into_iter()
  .multiunzip();

  let result = match &parsed.fields
  {
    syn::Fields::Named( _ ) =>
    {

      if 1 <= len && len <= 3
      {
        qt!
        {

          #[ automatically_derived ]
          // impl variadic_from::From2< i32 > for StructNamedFields
          impl variadic_from::#from_trait< #( #types )* > for #item_name
          {
            // fn from1( a : i32, b : i32 ) -> Self
            fn #from_method
            (
              #( #fn_params )*
            ) -> Self
            {
              #( #src_into_vars )*
              // let a = ::core::convert::Into::into( a );
              // let b = ::core::convert::Into::into( b );
              Self
              {
                #( #vars )*
                // a,
                // b,
              }
            }
          }

          impl From< ( #( #types )* ) > for #item_name
          {
            /// Reuse From1.
            #[ inline( always ) ]
            fn from( src : ( #( #types )* ) ) -> Self
            {
              Self::from1( src )
            }
          }

        }
      }
      else
      {
        qt!{}
      }

    }
    syn::Fields::Unnamed( _ ) =>
    {

      if 1 <= len && len <= 3
      {
        qt!
        {

          #[ automatically_derived ]
          // impl variadic_from::From2< i32 > for StructNamedFields
          impl variadic_from::#from_trait< #( #types )* > for #item_name
          {
            // fn from1( a : i32, b : i32 ) -> Self
            fn #from_method
            (
              #( #fn_params )*
            ) -> Self
            {
              #( #src_into_vars )*
              // let a = ::core::convert::Into::into( a );
              // let b = ::core::convert::Into::into( b );
              Self
              (
                #( #vars )*
                // a,
                // b,
              )
            }
          }

          impl From< ( #( #types )* ) > for #item_name
          {
            /// Reuse From1.
            #[ inline( always ) ]
            fn from( src : ( #( #types )* ) ) -> Self
            {
              Self::from1( src )
            }
          }

        }
      }
      else
      {
        qt!{}
      }

    }
    syn::Fields::Unit =>
    {

      qt!{}

    }
    // _ => return Err( syn_err!( parsed.fields.span(), "Expects fields" ) ),
  };

  if has_debug
  {
    let about = format!( "derive : VariadicForm\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
