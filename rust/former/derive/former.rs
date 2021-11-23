#![allow( unused_imports )]

use quote::quote;
use syn::{ parse_macro_input, parse_quote, DeriveInput };
use proc_macro_error::{ abort, abort_call_site };
use unzip_n::unzip_n;

unzip_n!( 3 );
unzip_n!( 4 );
unzip_n!( 5 );

//

#[derive( Debug )]
enum ContainerKind
{
  No,
  Vector,
  HashMap,
}

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
  pub is_container : ContainerKind,
}

//

fn is_option( ty : &syn::Type ) -> bool
{
  if let syn::Type::Path( path ) = ty
  {
    let last = &path.path.segments.last();
    if last.is_none()
    {
      return false;
    }
    return last.unwrap().ident == "Option";
  }
  false
}

//

fn parameters_internal( ty : &syn::Type, r : core::ops::RangeInclusive< usize > ) -> Vec< &syn::Type >
{
  // return vec![];
  if let syn::Type::Path( syn::TypePath{ path : syn::Path { ref segments, .. }, .. } ) = ty
  {
    let last = &segments.last();
    if last.is_none()
    {
      return vec![ &ty ]
    }
    let args = &last.unwrap().arguments;
    if let syn::PathArguments::AngleBracketed( ref args2 ) = args
    {
      let args3 = &args2.args;
      // trace_macros!( true );
      let selected : Vec< &syn::Type > = args3
      .iter()
      .skip_while( | e | if let syn::GenericArgument::Type( _ ) = e { false } else { true } )
      .skip( *r.start() )
      .take( *r.end() - *r.start() + 1 )
      .map( | e | if let syn::GenericArgument::Type( ty ) = e { ty } else { unreachable!( "Expects Type" ) } )
      .collect();
      // tree_print!( selected.first().unwrap() );
      return selected;
    }
  }
  vec![ &ty ]
}

//

fn parameter_internal_first( ty : &syn::Type ) -> &syn::Type
{
  parameters_internal( ty, 0 ..= 0 )
  .first()
  .or_else( || panic!( "Expect at least one parameter here:\n  {}", quote!{ #ty } ) ).unwrap()
}

//

fn parameter_internal_first_two( ty : &syn::Type ) -> ( &syn::Type, &syn::Type )
{
  let result = parameters_internal( ty, 0 ..= 1 );
  let mut iter = result.iter();
  (
    iter.next().or_else( || panic!( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) ).unwrap(),
    iter.next().or_else( || panic!( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) ).unwrap(),
  )
}

//

fn is_container( ty : &syn::Type ) -> ContainerKind
{
  if let syn::Type::Path( path ) = ty
  {
    let last = &path.path.segments.last();
    if last.is_none()
    {
      return ContainerKind::No
    }
    match last.unwrap().ident.to_string().as_ref()
    {
      "Vec" => { return ContainerKind::Vector }
      "HashMap" => { return ContainerKind::HashMap }
      _ => { return ContainerKind::No }
    }
  }
  ContainerKind::No
}

//

#[inline]
fn field_none_map( field : &FormerField ) -> syn::Field
{
  let attrs = field.attrs.clone();
  // let _vis = field.vis.clone();
  let ident = field.ident.clone();
  let colon_token = field.colon_token.clone();
  // let _ty = field.ty.clone();

  let tokens = quote! { core::option::Option::None };
  let ty2 : syn::Type = syn::parse2( tokens ).unwrap();
  syn::Field
  {
    attrs,
    vis : syn::parse2( quote!{} ).unwrap(),
    ident,
    colon_token,
    ty : ty2,
  }
}

//

#[inline]
fn field_optional_map( field : &FormerField ) -> syn::Field
{
  let attrs = field.attrs.clone();
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
    quote! { core::option::Option< #ty > }
  };

  let ty2 : syn::Type = syn::parse2( tokens ).unwrap();

  syn::Field
  {
    attrs,
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
        Some( self.#ident.take().unwrap() )
      }
      else
      {
        None
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
        let val : #ty = Default::default();
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
    syn::Ident::new( "?? no name ??", proc_macro2::Span::call_site() )
  }
}

//

#[inline]
fn field_setter_map( field : &FormerField ) -> syn::Stmt
{
  let ident = field.ident.clone();
  // let ty = field.ty;

  let tokens = match &field.is_container
  {
    ContainerKind::No =>
    {
      let non_optional_ty = &field.non_optional_ty;
      quote!
      {
        pub fn #ident< Src >( mut self, src : Src ) -> Self
        where Src : core::convert::Into< #non_optional_ty >,
        {
          debug_assert!( self.#ident.is_none() );
          self.#ident = Some( src.into() );
          self
        }
      }
    },
    ContainerKind::Vector =>
    {
      let ty = &field.ty;
      let internal_ty = parameter_internal_first( ty );
      // tree_print!( internal_ty );
      // let ident2 = syn::Ident::new( &format!( "{}_", ident.unwrap().to_string() ), proc_macro2::Span::call_site() );
      quote!
      {
        // pub fn #ident2() {}
        pub fn #ident( mut self ) -> former_runtime::VectorFormer
        <
          #internal_ty,
          #ty,
          CommandFormer,
          impl Fn( &mut CommandFormer, core::option::Option< #ty > )
        >
        {
          let container = self.#ident.take();
          let on_end = | former : &mut CommandFormer, container : core::option::Option< #ty > |
          {
            former.#ident = container;
          };
          former_runtime::VectorFormer::new( self, container, on_end )
        }
      }
    },
    ContainerKind::HashMap =>
    {
      // let k_ty = ty;
      // let e_ty = ty;
      let ty = &field.ty;
      let ( k_ty, e_ty ) = parameter_internal_first_two( ty );
      // tree_print!( k_ty, e_ty );
      // let ident2 = syn::Ident::new( &format!( "{}_", ident.unwrap().to_string() ), proc_macro2::Span::call_site() );
      quote!
      {
        // pub fn #ident2() {}
        pub fn #ident( mut self ) -> former_runtime::HashmapFormer
        <
          #k_ty,
          #e_ty,
          #ty,
          CommandFormer,
          impl Fn( &mut CommandFormer, core::option::Option< #ty > )
        >
        {
          let container = self.hashmap_strings_1.take();
          let on_end = | former : &mut CommandFormer, container : core::option::Option< #ty > |
          {
            former.hashmap_strings_1 = container;
          };
          former_runtime::HashmapFormer::new( self, container, on_end )
        }
      }
    },
  };

  let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
  stmt

  // pub fn int_1< Src >( mut self, src : Src ) -> Self
  // where Src : core::convert::Into< i32 >,
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
  let name = &ast.ident;

  let former_name = format!( "{}Former", name );
  let former_ident = syn::Ident::new( &former_name, name.span() );
  let fields = if let syn::Data::Struct( syn::DataStruct { fields : syn::Fields::Named( syn::FieldsNamed { ref named, .. } ), .. } ) = ast.data
  {
    named
  }
  else
  {
    abort!( ast.ident.span(), "Expects struct" );
  };

  let ( fields_none, fields_optional, fields_form, fields_names, fields_setter ) = fields.iter().map( | field |
  {
    let attrs = &field.attrs;
    let vis = &field.vis;
    let ident = &field.ident;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_option = is_option( &ty );
    let is_container = is_container( &ty );
    // println!( "is_container : {:?}", is_container );

    let non_optional_ty : &syn::Type = if is_option { parameter_internal_first( ty ) } else { ty };

    let former_field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_option, is_container };
    (
      field_none_map( &former_field ),
      field_optional_map( &former_field ),
      field_form_map( &former_field ),
      field_name_map( &former_field ),
      field_setter_map( &former_field ),
    )
  }).unzip_n_vec();

  let result = quote!
  {

    impl #name
    {
      pub fn former() -> #former_ident
      {
        #former_ident
        {
          #( #fields_none, )*
        }
      }
    }

    #[derive( Debug )]
    pub struct #former_ident
    {
      #( #fields_optional, )*
    }

    impl #former_ident
    {
      fn form( mut self ) -> #name
      {
        #( #fields_form )*
        Command
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
