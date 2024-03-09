
use super::*;
use macro_tools::{ type_struct, Result };
use iter::{ IterExt, Itertools };

//

pub fn variadic_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let item_name = parsed.item_name;

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
      ) : ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > ) = parsed.fields.iter().map_result( | field |
      {
        let ident = field.ident.clone().ok_or_else( || syn_err!( parsed.item.span(), "Fields should be named" ) )?;
        let ty = field.ty.clone();
        Result::Ok
        ((
          qt!{ #ty, },
          qt!{ #ident : #ty, },
          qt!{ let #ident = core::convert::Into::into( #ident ); },
          qt!{ #ident, },
        ))
      })?
      .into_iter().multiunzip();

      let l = format!( "{}", parsed.fields.len() );
      let from_trait = macro_tools::format_ident!( "From_{l}" );
      let from_method = macro_tools::format_ident!( "from_{l}" );

      qt!
      {

        #[ automatically_derived ]
        // impl wtools::From_2< i32 > for StructNamedFields
        impl wtools::#from_trait< #( #types )* > for #item_name
        {
          // fn from_1( a : i32, b : i32 ) -> Self
          fn #from_method
          (
            #( #fn_params )*
          ) -> Self
          {
            #( #src_into_vars )*
            // let a = core::convert::Into::into( a );
            // let b = core::convert::Into::into( b );
            Self
            {
              #( #vars )*
              // a,
              // b,
            }
          }
        }

        impl From< ( i32, i32 ) > for StructNamedFields
        {
          /// Returns the argument unchanged.
          #[ inline( always ) ]
          fn from( src : ( i32, i32 ) ) -> Self
          {
            Self::from_1( src )
          }
        }

      }

    }
    syn::Fields::Unnamed( _ ) =>
    {

      let mut counter = 0;
      let
      (
        vars_assing_default,
        src_into_vars,
        vars
      ) : ( Vec< _ >, Vec< _ >, Vec< _ > ) = parsed.fields.iter().map_result( | _field |
      {
        let ident = macro_tools::format_ident!( "_{}", format!( "{counter}" ) );
        counter += 1;
        Result::Ok
        ((
          qt!{ let #ident = core::default::Default::default(); },
          qt!{ let #ident = src.into(); },
          qt!{ #ident, },
        ))
      })?
      .into_iter().multiunzip();

      qt!
      {
        #[ automatically_derived ]
        impl wtools::From_0 for #item_name
        {
          fn from_0() -> Self
          {
            #( #vars_assing_default )*
            // let a = Default::default();
            // let b = Default::default();
            // let c = Default::default();
            // let d = Default::default();
            Self
            (
              #( #vars )*
              // a,
              // b,
              // c,
              // d,
            )
          }
        }

        #[ automatically_derived ]
        impl wtools::From_1< i32 > for #item_name
        {
          fn from_1( src : i32 ) -> Self
          {
            #( #src_into_vars )*
            // let a = src.into();
            // let b = src.into();
            // let c = src.into();
            // let d = src.into();
            Self
            (
              #( #vars )*
              // a,
              // b,
              // c,
              // d,
            )
          }
        }

      }

    }
    _ => return Err( syn_err!( parsed.fields.span(), "Expects fields" ) ),
  };

  Ok( result )
}
