use macro_tools::
{
  diag,
  generic_params,
  struct_like::StructLike,
  Result,
  qt,
  attr,
  syn,
  proc_macro2,
  return_syn_err,
  Spanned,
};
use crate::derive::syn::Variant;
use super::FieldAccess;
use super::item_attributes::{ ItemAttributes };

pub fn add( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > 
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

  let result = match parsed 
  {
    StructLike::Unit( ref item ) =>
    {
      return_syn_err!( item.span(), "Add cannot be derived for unit structs. It is only applicable to structs with at least one field." );
    },
    
    StructLike::Struct( ref item ) =>
    {

    let fields_result: Result< Vec < ( FieldAccess, syn::Type ) > > = item
    .fields
    .iter()
    .enumerate()
    .map( | ( index, field ) | {
        let access = match &field.ident 
        {
            Some( ident ) => FieldAccess::Named( ident.clone() ),
            None => FieldAccess::Unnamed( syn::Index::from( index ) ),
        };
        let field_type = field.ty.clone();
        Ok( ( access, field_type ) )
    })
    .collect();

      let fields = fields_result?;
      if fields.is_empty() 
      {
        return_syn_err!( item.span(), "Add requires at least one field in the struct" );
      }

      generate_struct
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &fields,
      )
      
    },
    StructLike::Enum( ref item ) =>
    {
        let non_empty_variants: Vec< &Variant > = item.variants.iter().filter( | variant| !variant.fields.is_empty() ).collect();
        if non_empty_variants.is_empty() 
        {
          return_syn_err!
          (
            item.span(),
            "Add cannot be derived for enums without any variants containing fields. At least one variant must have fields."
          );
        }
        
        generate_enum(
          item_name,
          &item_attrs,
          &generics_impl,
          &generics_ty,
          &generics_where,
          &non_empty_variants,
        )
    }
  };

  if has_debug
  {
    let about = format!( "derive : Add\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }
  
  Ok( result )
}

fn generate_enum
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl : & syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : & syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : & syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variants : &[ &syn::Variant ],
)
-> proc_macro2::TokenStream 
{
  let arms = variants.iter().map( | v |
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
        qt! { #a + #b }
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
    qt! 
    {
        ( #pat_a, #pat_b ) => Ok( #construct ),
    }
  }).collect::< Vec< _ > >();

  let body = qt!
  {
    match ( self, other )
    {
      #( #arms )*
      ( a, b ) => Err( format!( "Cannot add different variants" ).into() ), //: {a:?} and {b:?}")),  // TODO: Include variant names in error, e.g., "Cannot add VariantA and VariantB"
    }
  };
  
  let error_type: proc_macro2::TokenStream = if let Some( ty ) = &item_attrs.error 
  {
    qt! { #ty }
  } 
  else 
  {
    qt! { String }
  };
   qt! 
   {
      #[ automatically_derived ]
      impl< #generics_impl > std::ops::Add for #item_name< #generics_ty >
      where #generics_where
      {

      type Output = Result< Self, #error_type >;

      # [ inline ( always ) ]
      fn add( self, other: Self ) -> Self::Output 
      {
        #body
      }
    }
  }
}

fn generate_struct
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &[ ( FieldAccess, syn::Type ) ],
) 
-> proc_macro2::TokenStream 
{
  let additions = fields.iter().map( | ( access, _ty ) | 
  {
    match access 
    {
      FieldAccess::Named( ident ) => qt! { #ident: self.#ident + other.#ident },
      FieldAccess::Unnamed( index ) => qt! { self.#index + other.#index },
    }
  });

  let body = if matches!( fields.first(), Some( ( FieldAccess::Named( _ ), _ ) ) ) 
  {
    qt! { Self { #( #additions ),* } }
  } 
  else 
  {
    qt! { Self ( #( #additions ),* ) }
  };

  qt! 
  {
    #[ automatically_derived ]
    impl< #generics_impl > std::ops::Add for #item_name< #generics_ty >
    where #generics_where
    {
      type Output = Self;

      # [ inline ( always ) ]
      fn add( self, other: Self ) -> Self::Output 
      {
        #body
      }
    }
  }
}