// #![allow( unused_imports )]
#![ warn( missing_docs ) ]

use quote::quote;
use syn::{ DeriveInput };
// use proc_macro_error::{ abort };
use unzip_n::unzip_n;

use crate::wproc_macro;
// #[macro_use]
// extern crate wproc_macro;

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
  pub type_container_kind : wproc_macro::ContainerKind,
}

//

fn is_option( ty : &syn::Type ) -> bool
{
  wproc_macro::type_rightmost( ty ) == Some( "Option".to_string() )
}

//

fn parameter_internal_first( ty : &syn::Type ) -> &syn::Type
{
  wproc_macro::type_parameters( ty, 0 ..= 0 )
  .first()
  .or_else( || panic!( "Expect at least one parameter here:\n  {}", quote!{ #ty } ) ).unwrap()
}

//

fn parameter_internal_first_two( ty : &syn::Type ) -> Result< ( &syn::Type, &syn::Type ), syn::Error >
{
  let on_err = ||
  {
    syn::Error::new( proc_macro2::Span::call_site(), format!( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) )
  };
  let result = wproc_macro::type_parameters( ty, 0 ..= 1 );
  let mut iter = result.iter();
  Ok
  ((
    iter.next().ok_or_else( on_err )?,
    iter.next().ok_or_else( on_err )?,
  ),)
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
    vis : syn::Visibility::Inherited,
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
fn field_setter_map( field : &FormerField, former_name : &syn::Ident ) -> Result< syn::Stmt, syn::Error >
{
  let ident = field.ident.clone();

  let tokens = match &field.type_container_kind
  {
    wproc_macro::ContainerKind::No =>
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
    wproc_macro::ContainerKind::Vector =>
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
    wproc_macro::ContainerKind::HashMap =>
    {
      let ty = &field.ty;
      let ( k_ty, e_ty ) = parameter_internal_first_two( ty )?;
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
  Ok( stmt )

  // pub fn int_1< Src >( mut self, src : Src ) -> Self
  // where Src : Into< i32 >,
  // {
  //   debug_assert!( self.int_1.is_none() );
  //   self.int_1 = Some( src.into() );
  //   self
  // }

}

//

// use syn::spanned::Spanned;
// use wproc_macro::DataWrapped;

pub fn former( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream, syn::Error >
{
  // let ast = parse_macro_input!( input as DeriveInput );

  let ast = match syn::parse::< DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  let name_ident = &ast.ident;
  let former_name = format!( "{}Former", name_ident );
  let former_ident = syn::Ident::new( &former_name, name_ident.span() );
  let fields = if let syn::Data::Struct( syn::DataStruct { fields : syn::Fields::Named( syn::FieldsNamed { ref named, .. } ), .. } ) = ast.data
  {
    named
  }
  else
  {
    let data = &ast.data;
    return Err( syn::Error::new( wproc_macro::span_of( data ), format!( "Expects struct, found :\n{}", "xxx"/*quote!{ #data } */ ) ) );
    // return Err( syn::Error::new( wproc_macro::DataWrapped( data ).span(), format!( "Expects struct, found :\n{}", "xxx"/*quote!{ #data } */ ) ) );
    // return Err( syn::Error::new( data.span(), format!( "Expects struct, found :\n{}", "xxx"/*quote!{ #data } */ ) ) );
    // return Err( syn::Error::new( proc_macro2::Span::call_site(), format!( "Expects struct, found :\n{}", "xxx"/*quote!{ #data } */ ) ) );
  };

  let( fields_none, fields_optional, fields_form, fields_names, fields_setter ) = fields.iter().map( | field |
  {
    let attrs = &field.attrs;
    let vis = &field.vis;
    let ident = &field.ident;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_option = is_option( &ty );
    let type_container_kind = wproc_macro::type_container_kind( &ty );
    let non_optional_ty : &syn::Type = if is_option { parameter_internal_first( ty ) } else { ty };
    let former_field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_option, type_container_kind };
    (
      field_none_map( &former_field ),
      field_optional_map( &former_field ),
      field_form_map( &former_field ),
      field_name_map( &former_field ),
      field_setter_map( &former_field, &former_ident ),
    )
  }).unzip_n_vec();

  if let Some( Err( err ) ) = fields_setter.iter().find( | e | e.is_err() )
  {
    return Err( err.clone() );
  }

  let fields_setter = fields_setter.into_iter().map( | e | e.unwrap() ).collect::< Vec< syn::Stmt > >();

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
  // let result = proc_macro2::TokenStream::new();
  // result.into()
  Ok( result )
}
