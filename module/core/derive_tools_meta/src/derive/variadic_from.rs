
use super::*;
use macro_tools::{ Result, format_ident, attr, diag };
use iter::{ IterExt, Itertools };

//

// xxx : investigate
pub fn variadic_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let len = parsed.fields.len();
  let from_trait = format_ident!( "From_{len}",  );
  let from_method = format_ident!( "from_{len}" );

  let result = match &parsed.fields
  {
    syn::Fields::Named( _ ) =>
    {

      let
      (
        types,
        fn_params,
        src_into_vars,
        vars
      )
      :
      ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
      = parsed.fields.iter().map_result( | field |
      {
        let ident = field.ident.clone().ok_or_else( || syn_err!( parsed.span(), "Fields should be named" ) )?;
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

      if len <= 3
      {
        qt!
        {

          // xxx
          #[ automatically_derived ]
          // impl variadic_from::From_2< i32 > for StructNamedFields
          impl variadic_from::#from_trait< #( #types )* > for #item_name
          {
            // fn from_1( a : i32, b : i32 ) -> Self
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
            /// Reuse From_1.
            #[ inline( always ) ]
            fn from( src : ( #( #types )* ) ) -> Self
            {
              Self::from_1( src )
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
        // let ident = field.ident.clone().ok_or_else( || syn_err!( parsed.span(), "Fields should be named" ) )?;
        let ident = format_ident!( "_{i}" );
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
      // xxx : reduce maybe

      if len <= 3
      {
        qt!
        {

          // xxx
          #[ automatically_derived ]
          // impl variadic_from::From_2< i32 > for StructNamedFields
          impl variadic_from::#from_trait< #( #types )* > for #item_name
          {
            // fn from_1( a : i32, b : i32 ) -> Self
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
            /// Reuse From_1.
            #[ inline( always ) ]
            fn from( src : ( #( #types )* ) ) -> Self
            {
              Self::from_1( src )
            }
          }

        }
      }
      else
      {
        qt!{}
      }

//       qt!
//       {
//
//         // #[ automatically_derived ]
//         // impl variadic_from::From_0 for #item_name
//         // {
//         //   fn from_0() -> Self
//         //   {
//         //     #( #vars_assing_default )*
//         //     // let a = Default::default();
//         //     // let b = Default::default();
//         //     // let c = Default::default();
//         //     // let d = Default::default();
//         //     Self
//         //     (
//         //       #( #vars )*
//         //       // a,
//         //       // b,
//         //       // c,
//         //       // d,
//         //     )
//         //   }
//         // }
//
//         #[ automatically_derived ]
//         impl variadic_from::From_1< i32 > for #item_name
//         {
//           fn from_1( src : i32 ) -> Self
//           {
//             #( #src_into_vars )*
//             // let a = src.into();
//             // let b = src.into();
//             // let c = src.into();
//             // let d = src.into();
//             Self
//             (
//               #( #vars )*
//               // a,
//               // b,
//               // c,
//               // d,
//             )
//           }
//         }
//
//       }

    }
    _ => return Err( syn_err!( parsed.fields.span(), "Expects fields" ) ),
  };

  if has_debug
  {
    let about = format!( "derive : VariadicForm\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
