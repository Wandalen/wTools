#![allow( unused_imports )]

use quote::quote;
use syn::{ parse_macro_input, parse_quote, DeriveInput };
use proc_macro_error::{ abort, abort_call_site };
use unzip_n::unzip_n;
use crate::tpm;

unzip_n!( 0 );
unzip_n!( 3 );
unzip_n!( 4 );
unzip_n!( 5 );

/* xxx :
- remove unwraps
- add documentation generation
- add documentation
*/

//

#[allow( dead_code )]
struct FormerField< 'a >
{
  pub attrs : &'a Vec< syn::Attribute >,
  pub vis : &'a syn::Visibility,
  pub ident : &'a Option< syn::Ident >,
  pub colon_token : &'a Option< syn::token::Colon >,
  pub ty : &'a syn::Type,
  pub non_optional_ty : &'a syn::Type,
  pub is_option : bool,
  pub container_kind : tpm::ContainerKind,
}

//

fn is_option( ty : &syn::Type ) -> bool
{
  tpm::rightmost_is( ty, "Option" )
}

//

fn parameter_internal_first( ty : &syn::Type ) -> &syn::Type
{
  tpm::parameters_internal( ty, 0 ..= 0 )
  .first()
  .or_else( || panic!( "Expect at least one parameter here:\n  {}", quote!{ #ty } ) ).unwrap()
}

//

fn parameter_internal_first_two( ty : &syn::Type ) -> Result< ( &syn::Type, &syn::Type ), Error >
{
  let result = tpm::parameters_internal( ty, 0 ..= 1 );
  let mut iter = result.iter();
  (
    iter.next().ok_or_else( || Error::new( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) )?,
    iter.next().ok_or_else( || Error::new( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) )?,
  )
}

//

#[inline]
fn field_none_map( field : &FormerField ) -> syn::Field
{
  let ident = field.ident.clone();
  let colon_token = field.colon_token.clone();
  let tokens = quote! { ::core::option::Option::None };
  let ty2 : syn::Type = syn::parse2( tokens ).unwrap();
  syn::Field
  {
    attrs : vec![],
    vis : syn::Visibility::Inherited,
    ident,
    colon_token,
    ty : ty2,
  }
}

//

#[inline]
fn field_optional_map( field : &FormerField ) -> syn::Field
{
  // let attrs = field.attrs.clone();
  // let _vis = field.vis.clone();
  let ident = field.ident.clone();
  let colon_token = field.colon_token.clone();
  let ty = field.ty.clone();

  let tokens = if is_option( &ty )
  {
    quote! { #ty }
  }
  else
  {
    quote! { ::core::option::Option< #ty > }
  };

  let ty2 : syn::Type = syn::parse2( tokens ).unwrap();

  syn::Field
  {
    attrs : vec![],
    vis : syn::parse2( quote!{} ).unwrap(),
    ident,
    colon_token,
    ty : ty2,
  }
}

//

#[inline]
fn field_form_map( field : &FormerField ) -> syn::Stmt
{
  let ident = field.ident;
  let ty = field.ty;

  let tokens = if field.is_option
  {
    quote!
    {
      let #ident = if self.#ident.is_some()
      {
        ::core::option::Option::Some( self.#ident.take().unwrap() )
      }
      else
      {
        ::core::option::Option::None
      };
    }
  }
  else
  {
    quote!
    {
      let #ident = if self.#ident.is_some()
      {
        self.#ident.take().unwrap()
      }
      else
      {
        let val : #ty = ::core::default::Default::default();
        val
      };
    }
  };

  let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
  stmt
}

//

#[inline]
fn field_name_map( field : &FormerField ) -> syn::Ident
{
  let ident = field.ident.clone();
  if let Some( ident ) = ident
  {
    ident
  }
  else
  {
    syn::Ident::new( "?? no name_ident ??", proc_macro2::Span::call_site() )
  }
}

//

#[inline]
fn field_setter_map( field : &FormerField, former_name : &syn::Ident ) -> syn::Stmt
{
  let ident = field.ident.clone();

  let tokens = match &field.container_kind
  {
    tpm::ContainerKind::No =>
    {
      let non_optional_ty = &field.non_optional_ty;
      quote!
      {
        #[inline]
        pub fn #ident< Src >( mut self, src : Src ) -> Self
        where Src : ::core::convert::Into< #non_optional_ty >,
        {
          debug_assert!( self.#ident.is_none() );
          self.#ident = ::core::option::Option::Some( src.into() );
          self
        }
      }
    },
    tpm::ContainerKind::Vector =>
    {
      let ty = &field.ty;
      let internal_ty = parameter_internal_first( ty );
      quote!
      {
        #[inline]
        pub fn #ident( mut self ) -> former::runtime::VectorFormer
        <
          #internal_ty,
          #ty,
          #former_name,
          impl Fn( &mut #former_name, ::core::option::Option< #ty > )
        >
        {
          let container = self.#ident.take();
          let on_end = | former : &mut #former_name, container : ::core::option::Option< #ty > |
          {
            former.#ident = container;
          };
          former::runtime::VectorFormer::new( self, container, on_end )
        }
      }
    },
    tpm::ContainerKind::HashMap =>
    {
      let ty = &field.ty;
      let ( k_ty, e_ty ) = parameter_internal_first_two( ty );
      quote!
      {
        #[inline]
        pub fn #ident( mut self ) -> former::runtime::HashmapFormer
        <
          #k_ty,
          #e_ty,
          #ty,
          #former_name,
          impl Fn( &mut #former_name, ::core::option::Option< #ty > )
        >
        {
          let container = self.hashmap_strings_1.take();
          let on_end = | former : &mut #former_name, container : ::core::option::Option< #ty > |
          {
            former.hashmap_strings_1 = container;
          };
          former::runtime::HashmapFormer::new( self, container, on_end )
        }
      }
    },
  };

  let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
  stmt

  // pub fn int_1< Src >( mut self, src : Src ) -> Self
  // where Src : Into< i32 >,
  // {
  //   debug_assert!( self.int_1.is_none() );
  //   self.int_1 = Some( src.into() );
  //   self
  // }

}

//

pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let ast = parse_macro_input!( input as DeriveInput );
  let name_ident = &ast.ident;

  let former_name = format!( "{}Former", name_ident );
  let former_ident = syn::Ident::new( &former_name, name_ident.span() );
  let fields = if let syn::Data::Struct( syn::DataStruct { fields : syn::Fields::Named( syn::FieldsNamed { ref named, .. } ), .. } ) = ast.data
  {
    named
  }
  else
  {
    abort!( ast.ident.span(), "Expects struct" );
  };

  let( fields_none, fields_optional, fields_form, fields_names, fields_setter ) = fields.iter().map( | field |
  {
    let attrs = &field.attrs;
    let vis = &field.vis;
    let ident = &field.ident;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_option = is_option( &ty );
    let container_kind = tpm::container_kind( &ty );
    let non_optional_ty : &syn::Type = if is_option { parameter_internal_first( ty ) } else { ty };

    let former_field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_option, container_kind };
    (
      field_none_map( &former_field ),
      field_optional_map( &former_field ),
      field_form_map( &former_field ),
      field_name_map( &former_field ),
      field_setter_map( &former_field, &former_ident ),
    )
  }).unzip_n_vec();

  let result = quote!
  {

    impl #name_ident
    {
      #[inline]
      pub fn former() -> #former_ident
      {
        #former_ident
        {
          #( #fields_none, )*
        }
      }
    }

    /* qqq : pub is optional here */
    #[derive( Debug )]
    pub struct #former_ident
    {
      #( #fields_optional, )*
    }

    impl #former_ident
    {
      #[inline]
      pub fn form( mut self ) -> #name_ident
      {
        #( #fields_form )*
        #name_ident
        {
          #( #fields_names, )*
        }
      }
      #( #fields_setter )*
    }

  };

  // println!( "{:#?}", ast );
  // println!( "{:#?}", result );
  // proc_macro::TokenStream::new()
  result.into()
}
