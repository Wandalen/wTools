mod add; 
mod sub;
mod mul;
mod div; 
mod item_attributes;

pub use add::add;
pub use sub::sub;
pub use mul::mul;
pub use div::div;

use item_attributes::ItemAttributes;
use proc_macro2::TokenStream;
use macro_tools::
{
  qt,
  syn,
  proc_macro2,
};
use super::FieldAccess;

#[ derive( Copy, Clone ) ]
pub enum OpKind 
{
  Add,
  Sub,
  Mul,
  Div,
}

fn generate_enum_match_arms< F >
( 
  item_name : &syn::Ident, 
  variants : &[ &syn::Variant ], 
  op_expression : F 
) 
-> Vec< TokenStream >
where F : Fn( &syn::Ident, &syn::Ident ) -> TokenStream,
{
  variants.iter().map( | v |
  {
    let variant_ident = &v.ident;
    let fields : Vec< FieldAccess > = v.fields.iter().enumerate().map( | ( i, f ) | 
    {
      match &f.ident 
      {
        Some( ident ) => FieldAccess::Named( ident.clone() ),
        None => FieldAccess::Unnamed( syn::Index::from( i ) ),
      }
    }).collect();

    let a_vars : Vec< syn::Ident > = fields.iter().enumerate().map( | ( i, _ ) | 
    {
      syn::Ident::new( &format!( "a{i}" ), proc_macro2::Span::call_site() )
    }).collect();
    let b_vars : Vec < syn::Ident > = fields.iter().enumerate().map( | (  i, _ ) | 
    {
      syn::Ident::new( &format!( "b{i}" ), proc_macro2::Span::call_site() )
    }).collect();

    let a_fields = fields.iter().zip( &a_vars ).map( | ( faccess, var ) | 
    {
      match faccess 
      {
        FieldAccess::Named( ident ) => qt! { #ident : #var },
        FieldAccess::Unnamed( _ ) => qt! { #var },
      }
    });

    let b_fields = fields.iter().zip( &b_vars ).map( | ( faccess, var ) | 
    {
      match faccess 
      {
        FieldAccess::Named( ident ) => qt! { #ident: #var },
        FieldAccess::Unnamed( _ ) => qt! { #var },
      }
    });
    
    let add_fields = a_vars.iter().zip( &b_vars ).map( | ( a, b ) | 
    {
      op_expression( a, b )
    });

    let pat_a = match v.fields 
    {
      syn::Fields::Named( _ ) => 
        {
          qt! { #item_name::#variant_ident { #( #a_fields ), * } }
        }
      _ => 
        {
          qt! { #item_name::#variant_ident( #( #a_fields ), * ) }
        }
    };

    let pat_b = match v.fields 
    {
      syn::Fields::Named( _ ) => 
        {
          qt! { #item_name::#variant_ident { #( #b_fields ), * } }
        }
      _ => 
        {
          qt! { #item_name::#variant_ident( #( #b_fields ), * ) }
        }
    };
    let construct = match v.fields 
    {
      syn::Fields::Named( _ ) => 
        {
          qt! { #item_name::#variant_ident { #( #add_fields ), * } }
        }
      _ => 
        {
          qt! { #item_name::#variant_ident( #( #add_fields ), * ) }
        }
    };
    qt! { ( #pat_a, #pat_b ) => Ok( #construct ), }
    }).collect::< Vec< _ > >()
}

fn generate_enum_match_body< F >
( 
  item_name : &syn::Ident, 
  variants : &[ &syn::Variant ], 
  item_attrs : &ItemAttributes, 
  op : OpKind, 
  op_expression : F 
) 
-> proc_macro2::TokenStream
where F : Fn( &syn::Ident, &syn::Ident ) -> TokenStream,
{
  let arms = generate_enum_match_arms( item_name, variants, op_expression );
  
  if let Some( error_expr ) = item_attrs.error_expr_for( op ) 
  {
    qt! 
    {
      match ( self, other ) 
      {
        #( #arms )*
        _ => Err( #error_expr ),
      }
    }
  } 
  else 
  {
    qt! 
    {
     match ( self, other ) 
      {
        #( #arms )*
        _ => Err( "Mismatched variants".into() ),
      }
    }
  }
}