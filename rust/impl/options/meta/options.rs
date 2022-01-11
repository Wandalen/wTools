#![ allow( unused_imports ) ]
#![ allow( unused_mut ) ]
#![ allow( dead_code ) ]
#![ allow( unused_variables ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

use meta_tools::*;
use quote::{ quote, TokenStreamExt };
use syn::parse::*;
use wproc_macro::*;
use std::collections::HashMap;
// use itertools::{ MultiUnzip, process_results };
use iter_tools::{ Itertools, process_results };

pub type Result< T > = std::result::Result< T, syn::Error >;

///
/// Descriptor of a function with a body, body of which is not parsed.
///

pub struct FnQuick
{
  pub attrs : Vec< syn::Attribute >,
  pub vis : syn::Visibility,
  pub sig : syn::Signature,
  pub block : Option< proc_macro2::TokenStream >,
}

impl quote::ToTokens for FnQuick
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {

    for attr in self.attrs.iter()
    {
      attr.to_tokens( tokens );
    }

    self.sig.to_tokens( tokens );

    match &self.block
    {
      Some( block ) => tokens.append( proc_macro2::Group::new( proc_macro2::Delimiter::Brace, block.to_token_stream() ) ),
      None => tokens.append( proc_macro2::Punct::new( ';', proc_macro2::Spacing::Alone ) ),
    }

  }
}

///
/// Descriptor of element of options.
///

pub enum Element
{
  Fn( FnQuick ),
  Signature( FnQuick ),
  Field( syn::Field ),
}

impl Parse for Element
{
  fn parse( input : ParseStream ) -> Result< Self >
  {

    let attrs : Vec< syn::Attribute > = input.call( syn::Attribute::parse_outer )?;
    let vis : syn::Visibility = input.parse()?;

    let lookahead1 = input.lookahead1();
    if lookahead1.peek( syn::Token!{ fn } )
    {
      let sig : syn::Signature = input.parse()?;

      let lookahead2 = input.lookahead1();
      if lookahead2.peek( syn::token::Brace )
      {
        let input2;
        let brace_token : syn::token::Brace = syn::braced!( input2 in input );
        let block : proc_macro2::TokenStream = input2.parse()?;
        let fn_desc = FnQuick
        {
          attrs,
          vis,
          sig,
          block : Some( block ),
        };
        return Ok( Element::Fn( fn_desc ) );
      }
      else
      {
        let fn_desc = FnQuick
        {
          attrs,
          vis,
          sig,
          block : None,
        };
        return Ok( Element::Fn( fn_desc ) );
      }
    }
    else
    {
      input.call( syn::Field::parse_named ).map( | mut e |
      {
        e.vis = vis;
        e.attrs = attrs;
        Element::Field( e )
      })
    }
  }
}

///
/// Descriptor of attribute options.
///

#[allow( dead_code )]
struct OptionsDescriptor
{
  attrs : Vec< syn::Attribute >,
  vis : syn::Visibility,
  ident : syn::Ident,
  generics: syn::Generics,
  brace_token : syn::token::Brace,
  methods_map : HashMap< String, FnQuick >,
  signatures_map : HashMap< String, FnQuick >,
  fields_map : HashMap< String, syn::Field >,
}

impl Parse for OptionsDescriptor
{
  fn parse( input : ParseStream ) -> Result< Self >
  {
    let input2;
    let vis = input.parse()?;
    let ident = input.parse()?;
    let generics = input.parse()?;
    let brace_token = syn::braced!( input2 in input );
    let mut attrs = input2.call( syn::Attribute::parse_inner )?;
    let elements : syn::punctuated::Punctuated< Element, syn::Token!{ ; } > = input2.parse_terminated( Element::parse )?;
    let mut methods_map = hmap!{};
    let mut signatures_map = hmap!{};
    let mut fields_map = hmap!{};

    for attr in attrs.iter_mut()
    {
      attr.style = syn::AttrStyle::Outer;
    }

    for element in elements.into_iter()
    {
      match element
      {
        Element::Fn( f ) =>
        {
          methods_map.insert( f.sig.ident.to_string(), f );
        },
        Element::Signature( f ) =>
        {
          signatures_map.insert( f.sig.ident.to_string(), f );
        },
        Element::Field( f ) =>
        {
          let key = f.ident.as_ref().ok_or_else( || syn_err!( &f.clone(), "Field does not have name: {}", quote!{ #f } ) )?.to_string();
          fields_map.insert( key, f );
        },
      }
    }

    let mut result = OptionsDescriptor
    {
      vis,
      ident,
      generics,
      brace_token,
      attrs,
      methods_map,
      signatures_map,
      fields_map,
    };
    Ok( result )
  }
}

///
/// Generate a getter for a field.
///

// fn getter_gen( name : &str, field : &syn::Field ) -> Result< syn::Stmt >
fn getter_gen( name : &str, field : &syn::Field ) -> Result< proc_macro2::TokenStream >
{

  let tokens = quote!
  {
    fn #name ( &self ) -> &'a str;
  };

  Ok( tokens )
  // let stmt : syn::Stmt = syn::parse2( tokens )?;
}

///
/// Options macro handler.
///

pub fn options( attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  // let ast = parse_macro_input!( input as DeriveInput );

  let options_descriptor = match syn::parse::< OptionsDescriptor >( item )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  let name_ident = &options_descriptor.ident;
  let generics = &options_descriptor.generics;
  let attrs = &options_descriptor.attrs;
  let mut fields_define = Vec::< &syn::Field >::new();

  for ( name, field ) in options_descriptor.fields_map.iter()
  {
    fields_define.push( field );
  }
  let mut perform = quote!{};
  let mut attr_form_after = quote!{};
  if let Some( perform_fn ) = options_descriptor.methods_map.get( "perform" )
  {
    let sig = &perform_fn.sig;
    attr_form_after = quote!{ #[ form_after( #sig ) ] };
    perform = quote!
    {
      #[ allow( unused_attributes ) ]
      #[ inline ]
      #perform_fn
    }
  }

  // let ( fields_none, fields_optional, fields_form, fields_names, fields_setter )
  // : ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  // = former_fields.iter().map( | former_field |
  // {(
  //   field_none_map( &former_field ),
  //   field_optional_map( &former_field ),
  //   field_form_map( &former_field ),
  //   field_name_map( &former_field ),
  //   field_setter_map( &former_field, &former_name_ident ),
  // )}).multiunzip();

  let getters : Vec< _ > = options_descriptor.fields_map.iter().map( | ( key, field ) | getter_gen( key, field ) ).collect();
  let getters : Vec< _ > = process_results( getters, | iter | iter.collect() )?;

  // #[ inline ]
  // fn src( &self ) -> &'a str
  // {
  //   &self.src
  // }

  let result = quote!
  {

    mod #name_ident
    {

      use ::woptions::*;

      #( #attrs )*
      #[ derive( Former, PartialEq, Debug ) ]
      #attr_form_after
      pub struct Options #generics
      {
        #( #fields_define, )*
      }

      pub trait OptionsAdapter #generics
      {
        fn src( &self ) -> &'a str;
        fn delimeter( &self ) -> &'a str;
        fn left( &self ) -> &bool;

        #perform

      }

      impl #generics OptionsAdapter #generics for Options #generics
      {
        #[ inline ]
        fn src( &self ) -> &'a str
        {
          &self.src
        }
        #[ inline ]
        fn delimeter( &self ) -> &'a str
        {
          &self.delimeter
        }
        #[ inline ]
        fn left( &self ) -> &bool
        {
          &self.left
        }
      }

      #[ inline ]
      pub fn former #generics() -> OptionsFormer #generics
      {
        Options::#generics::former()
      }

    }

    #[ inline ]
    fn #name_ident #generics () -> #name_ident::OptionsFormer #generics
    {
      #name_ident::former::#generics()
    }

  };

  // println!( "{:#?}", ast );
  // println!( "{:#?}", result );
  // let result = proc_macro2::TokenStream::new();
  Ok( result )
}
