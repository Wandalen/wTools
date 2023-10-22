
use super::*;
use iter::{ IterExt, Itertools };

//

pub fn variadic_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let item_name = parsed.item_name;

  let result = match &parsed.fields
  {
    syn::Fields::Named( _ ) =>
    {

      let
      (
        vars_assing_default,
        src_into_vars,
        vars
      ) : ( Vec< _ >, Vec< _ >, Vec< _ > ) = parsed.fields.iter().map_result( | field |
      {
        let ident = field.ident.clone().ok_or_else( || syn_err!( parsed.item.span(), "Fields should be named" ) )?;
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
            {
              #( #vars )*
              // a,
              // b,
              // c,
              // d,
            }
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
            {
              #( #vars )*
              // a,
              // b,
              // c,
              // d,
            }
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
