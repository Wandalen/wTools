use super::*;
use macro_tools::
{
  attr, 
  diag, 
  generic_params,
  struct_like::StructLike, 
  Result
};

#[ path = "index/item_attributes.rs" ]
mod item_attributes;
use item_attributes::*;
#[ path = "index/field_attributes.rs" ]
mod field_attributes;
use field_attributes::*;


pub fn index_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > 
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();
 
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where ) 
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed 
  {
    StructLike::Struct( ref item ) => 
    generate_struct
    (
      item_name,
      &item_attrs,
      &generics_impl,
      &generics_ty,
      &generics_where,
      &item.fields,

    ),
    StructLike::Enum( _ ) => 
    unimplemented!( "IndexMut not implemented for Enum" ),
    StructLike::Unit( _ ) => 
    unimplemented!( "IndexMut not implemented for Unit" ),
  }?;

  if has_debug 
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// An aggregator function to generate `IndexMut` implementation for tuple and named structs 
fn generate_struct
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::Fields,
) 
-> Result< proc_macro2::TokenStream > 
{

  match fields 
  {
    syn::Fields::Named( fields ) => 
    generate_struct_named_fields
    (
      item_name, 
      &item_attrs,
      generics_impl, 
      generics_ty, 
      generics_where, 
      fields
    ),
    
    syn::Fields::Unnamed( fields ) => 
    generate_struct_tuple_fields
    (
      item_name, 
      generics_impl, 
      generics_ty, 
      generics_where, 
      fields
    ),

    syn::Fields::Unit => 
    unimplemented!( "IndexMut not implemented for Unit" ),
  }
}


fn generate_struct_named_fields
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::FieldsNamed,
) 
-> Result< proc_macro2::TokenStream > 
{

  let fields = fields.named.clone();
  let attr_name = &item_attrs.index.name.clone().internal();

  let field_attrs: Vec<&syn::Field> = fields
    .iter()
    .filter
    (
      |field| 
      {
        FieldAttributes::from_attrs( field.attrs.iter() ).map_or
        ( 
          false, 
          | attrs | attrs.index.value( false ) 
        )
      }
    )
    .collect();

  let generate = | is_mut : bool | 
  -> Result< proc_macro2::TokenStream >
  {
    if let Some( attr_name ) = attr_name 
    {
      Ok
      (
        qt! 
        {
          &self.#attr_name[index]
        }
      )
    } 
    else 
    {
      match field_attrs.len() 
      {
        0 | 1 => 
        {
          let field_name = 
            match field_attrs
              .first()
              .map(|&field| field)
              .or_else
              (
                || fields.first()
              ) 
            {
              Some( field ) => 
              field.ident.as_ref().unwrap(),
              None => 
              unimplemented!("IndexMut not implemented for Unit"),
            };
          
          Ok
          (
            if is_mut 
            {
              qt! 
              {
                &mut self.#field_name[ index ]
              }
            } 
            else 
            {
              qt! 
              {
                &self.#field_name[ index ]
              }
            }
          )
        }
        _ => 
        Err
        (
          syn::Error::new_spanned
          (
            &fields,
            "Only one field can include #[ index ] derive macro",
          )
        ),
      }
    } 
  };


  let generated_index = generate(false)?;
  let generated_index_mut = generate(true)?;
  
  Ok
  (
    qt! 
    {
       #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = T;
        #[ inline( always ) ]
        fn index( &self, index : usize ) -> &Self::Output
        {
          #generated_index
        }
      }
      
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::IndexMut< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn index_mut( &mut self, index : usize ) -> &mut Self::Output
        {
          #generated_index_mut 
        }
      }
    }
  )
}

fn generate_struct_tuple_fields
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::FieldsUnnamed,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = fields.unnamed.clone();
  let non_empty_attrs : Vec< &syn::Field > = fields
    .iter()
    .filter( | field | !field.attrs.is_empty() )
    .collect();
  
  let generated = match non_empty_attrs.len() 
  {
    0 =>
    { 
      return Err
      (
        syn::Error::new_spanned
        ( 
          &fields, 
          "No attributes specified. You must to specify #[ index ] for fields or name for #[ index ( name = field_name ) ] for item derive macro" 
        )
      );
    },
    1 => fields.iter().enumerate().map
    (
      | ( i, field ) | 
      { 
        let i = syn::Index::from( i );  
        if !field.attrs.is_empty() 
        {
          qt! 
          {
            &mut self.#i[ index ] 
          }
        } 
        else 
        {
          qt!{ }
        }
      }
    ),  
    _ => 
    {
      return Err
      (
        syn::Error::new_spanned
        ( 
          &fields, 
          "Only one field can include #[ index ] derive macro" 
        )
      );
    }
  };
  
  Ok
  (
    qt! 
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::IndexMut< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn index_mut( &mut self, index : usize ) -> &mut Self::Output
        {
          #( #generated )*
        }
      }
    }
  )
}
