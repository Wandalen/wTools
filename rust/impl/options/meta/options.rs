#![ allow( unused_imports ) ]
#![ allow( unused_mut ) ]
#![ allow( dead_code ) ]
#![ allow( unused_variables ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

use meta_tools::*;
use quote::{ quote, ToTokens, TokenStreamExt };
use syn::parse::*;
use syn::spanned::Spanned;
use wproc_macro::*;
use std::collections::HashMap;
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
/// Getter descriptor.
///

#[ derive( Debug ) ]
pub struct GetterDescriptor
{
  attr : proc_macro2::TokenStream,
  signature : proc_macro2::TokenStream,
  body : proc_macro2::TokenStream,
}

//

impl quote::ToTokens for GetterDescriptor
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.attr.to_tokens( tokens );
    self.signature.to_tokens( tokens );
    self.body.to_tokens( tokens );
  }
}

///
/// Generate a getter for a field.
///

fn getter_gen( name : &str, field : &syn::Field ) -> Result< GetterDescriptor >
{

  let name_ident = syn::Ident::new( &name, field.span() );
  let ty = &field.ty;

  // tree_print!( ty );

  let ty_is_ref = matches!( ty, syn::Type::Reference( _ ) );

  let ty2 = if ty_is_ref
  {
    ty.to_token_stream()
  }
  else
  {
    quote!{ & #ty }
  };

  let attr = quote!
  {
    #[ inline ]
  };

  let signature = quote!
  {
    fn #name_ident( &self ) -> #ty2
  };

  let body = quote!
  {
    {
      &self.#name_ident
    }
  };

  let result = GetterDescriptor
  {
    attr,
    signature,
    body,
  };

  Ok( result )
}

///
///
///

fn perform_gen( options_descriptor : &OptionsDescriptor ) -> ( proc_macro2::TokenStream, proc_macro2::TokenStream )
{

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

  ( perform, attr_form_after )
}

///
/// Options macro handler.
///

pub fn options( attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

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

  let ( perform, attr_form_after ) = perform_gen( &options_descriptor );

  let getters = options_descriptor.fields_map.iter().map( | ( key, field ) | getter_gen( key, field ) );
  let getters : Vec< _ > = process_results( getters, | iter | iter.collect() )?;
  let getters_signatures : Vec< _ > = getters.iter().map( | e | e.signature.clone() ).collect();

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
        #( #getters_signatures ; )*
        #perform
      }

      impl #generics OptionsAdapter #generics for Options #generics
      {
        #( #getters )*
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

  Ok( result )
}

//
// = Input :
//
// Options!{ split< 'a >
// {
//   #![ derive( PartialOrd ) ]
//
//   pub src : &'a str;
//   pub delimeter : &'a str;
//   #[ default( true ) ]
//   pub left : bool;
//
//   fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) >
//   where
//     Self : Sized,
//   {
//     if *self.left()
//     {
//       Box::new( self.src().split( self.delimeter() ) )
//     }
//     else
//     {
//       Box::new( self.src().rsplit( self.delimeter() ) )
//     }
//   }
//
// }}
//

//
// = Output:
//
// #[ derive( PartialOrd ) ]
// #[ derive( Former, PartialEq, Debug ) ]
// #[ form_after( fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) > ) ]
// pub struct Options< 'a >
// {
//   pub src : &'a str,
//   pub delimeter : &'a str,
//   #[ default( true ) ]
//   pub left : bool,
// }
//
// pub trait OptionsAdapter< 'a >
// {
//   fn src( &self ) -> &'a str;
//   fn delimeter( &self ) -> &'a str;
//   fn left( &self ) -> &bool;
//   #[ inline ]
//   fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) >
//   where
//     Self : Sized,
//   {
//     if *self.left()
//     {
//       Box::new( self.src().split( self.delimeter() ) )
//     }
//     else
//     {
//       Box::new( self.src().rsplit( self.delimeter() ) )
//     }
//   }
// }
//
// impl< 'a > OptionsAdapter< 'a > for Options< 'a >
// {
//   #[ inline ]
//   fn src( &self ) -> &'a str
//   {
//     &self.src
//   }
//   #[ inline ]
//   fn delimeter( &self ) -> &'a str
//   {
//     &self.delimeter
//   }
//   #[ inline ]
//   fn left( &self ) -> &bool
//   {
//     &self.left
//   }
// }
//
// #[ inline ]
// pub fn former< 'a >() -> OptionsFormer< 'a >
// {
//   Options::< 'a >::former()
// }
//
// }
//
// #[ inline ]
// fn split< 'a >() -> split::OptionsFormer< 'a >
// {
// split::former::< 'a >()
// }
//