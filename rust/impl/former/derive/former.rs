// #![allow( unused_imports )]
#![ warn( missing_docs ) ]

use quote::quote;
use syn::{ DeriveInput };
use itertools::{ MultiUnzip, process_results };
// use itertools::{ Itertools };
// use itertools::*;

use wproc_macro::*;
use syn::spanned::*;

pub type Result< T > = std::result::Result< T, syn::Error >;

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
  pub ident : &'a Option< syn::Ident >, /* xxx : make it nonoptional? */
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

fn parameter_internal_first( ty : &syn::Type ) -> Result< &syn::Type >
{
  wproc_macro::type_parameters( ty, 0 ..= 0 )
  .first()
  .map( | e | *e )
  .ok_or_else( || syn::Error::new( ty.span(), format!( "Expect at least one parameter here:\n  {}", quote!{ #ty } ) ) )
  /* xxx : write failing test */
}

//

fn parameter_internal_first_two( ty : &syn::Type ) -> Result< ( &syn::Type, &syn::Type ) >
{
  let on_err = ||
  {
    syn::Error::new( ty.span(), format!( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) )
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
fn field_form_map( field : &FormerField ) -> Result< syn::Stmt >
{
  let ident = field.ident;
  let ty = field.ty;

  let mut default = None;
  if field.attrs.len() == 1
  {
    let ( key, val, _meta ) = attr_pair_single
    (
      field.attrs.first().ok_or_else( || syn_err!( field.ident.as_ref().unwrap(), "No attr" ) )?
    )?;
    if key != "default".to_string()
    {
      return Err( syn_err!( field.ident.as_ref().unwrap(), format!( "Unknown attribute key {}", key ) ) ); /* xxx : negative test? */
    }
    default = Some( val );
  }

  let tokens = if field.is_option
  {

    let _else = if default == None
    {
      quote!
      {
        ::core::option::Option::None
      }
    }
    else
    {
      let default_val = default.unwrap();
      quote!
      {
        ::core::option::Option::Some( ( #default_val ).into() )
      }
    };

    quote!
    {
      let #ident = if self.#ident.is_some()
      {
        ::core::option::Option::Some( self.#ident.take().unwrap() )
      }
      else
      {
        #_else
      };
    }

  }
  else
  {

    let _else = if default == None
    {
      quote!
      {
        let val : #ty = ::core::default::Default::default();
      }
    }
    else
    {
      let default_val = default.unwrap();
      quote!
      {
        let val : #ty = ( #default_val ).into();
      }
    };

    quote!
    {
      let #ident = if self.#ident.is_some()
      {
        self.#ident.take().unwrap()
      }
      else
      {
        #_else
        val
      };
    }

  };

  let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
  Ok( stmt )
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
fn field_setter_map( field : &FormerField, former_name_ident : &syn::Ident ) -> Result< syn::Stmt >
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
      let internal_ty = parameter_internal_first( ty )?;
      quote!
      {
        #[inline]
        pub fn #ident( mut self ) -> former::runtime::VectorFormer
        <
          #internal_ty,
          #ty,
          #former_name_ident,
          impl Fn( &mut #former_name_ident, ::core::option::Option< #ty > )
        >
        {
          let container = self.#ident.take();
          let on_end = | former : &mut #former_name_ident, container : ::core::option::Option< #ty > |
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
          #former_name_ident,
          impl Fn( &mut #former_name_ident, ::core::option::Option< #ty > )
        >
        {
          let container = self.hashmap_strings_1.take();
          let on_end = | former : &mut #former_name_ident, container : ::core::option::Option< #ty > |
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

pub fn former( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  // let ast = parse_macro_input!( input as DeriveInput );

  let ast = match syn::parse::< DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  let name_ident = &ast.ident;
  let generics = &ast.generics;
  let former_name = format!( "{}Former", name_ident );
  let former_name_ident = syn::Ident::new( &former_name, name_ident.span() );

  let fields = match ast.data
  {
    syn::Data::Struct( ref data_struct ) => match data_struct.fields
    {
      syn::Fields::Named( ref fields_named ) =>
      {
        &fields_named.named
      },
      _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Fields::Named( ref fields_named )" ) ),
    },
    _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Data::Struct( ref data_struct )" ) ),
  };

  let former_fields : Vec< Result< FormerField > > = fields.iter().map( | field |
  {
    let attrs = &field.attrs;
    let vis = &field.vis;
    let ident = &field.ident;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_option = is_option( &ty );
    let type_container_kind = wproc_macro::type_container_kind( &ty );
    let non_optional_ty : &syn::Type = if is_option { parameter_internal_first( ty )? } else { ty };
    let former_field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_option, type_container_kind };
    Ok( former_field )
  }).collect();

  let former_fields : Vec< _ > = process_results( former_fields, | iter | iter.collect() )?;

  let( fields_none, fields_optional, fields_form, fields_names, fields_setter )
  : ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = former_fields.iter().map( | former_field |
  {(
    field_none_map( &former_field ),
    field_optional_map( &former_field ),
    field_form_map( &former_field ),
    field_name_map( &former_field ),
    field_setter_map( &former_field, &former_name_ident ),
  )}).multiunzip();

  let fields_setter : Vec< _ > = process_results( fields_setter, | iter | iter.collect() )?;
  let fields_form : Vec< _ > = process_results( fields_form, | iter | iter.collect() )?;

  let result = quote!
  {

    impl #generics #name_ident #generics
    {
      #[inline]
      pub fn former() -> #former_name_ident
      {
        #former_name_ident
        {
          #( #fields_none, )*
        }
      }
    }

    /* xxx : qqq : pub is optional here */
    #[derive( Debug )]
    pub struct #former_name_ident #generics
    {
      #( #fields_optional, )*
    }

    impl #generics #former_name_ident #generics
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
  Ok( result )
}
