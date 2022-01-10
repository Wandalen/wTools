#![ allow( unused_imports ) ]
#![ allow( unused_mut ) ]
#![ allow( dead_code ) ]
#![ allow( unused_variables ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

use meta_tools::*;
use quote::{ quote };
use syn::parse::*;
use wproc_macro::*;
use std::collections::HashMap;

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
  // elements : syn::punctuated::Punctuated< Element, syn::Token!{ ; } >,
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
      // elements,
      methods_map,
      signatures_map,
      fields_map,
    };
    Ok( result )
  }
}

//

pub fn options( attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  // let ast = parse_macro_input!( input as DeriveInput );

  let options_descriptor = match syn::parse::< OptionsDescriptor >( item )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  let generics = &options_descriptor.generics;
  let attrs = &options_descriptor.attrs;
  let mut fields_define = Vec::< &syn::Field >::new();

  for ( name, field ) in options_descriptor.fields_map.iter()
  {
    fields_define.push( field );
  }
  let mut attr_form_after = quote!{};
  if let Some( perform_fn ) = options_descriptor.methods_map.get( "perform" )
  {
    let sig = &perform_fn.sig;
    attr_form_after = quote!{ #[ form_after( #sig ) ] };
  }

  let result = quote!
  {

    mod split
    {

      use woptions::*;

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
        #[ inline ]
        fn perform( self ) -> std::str::Split< 'a, &'a str >
        where
          Self : Sized,
        {
          self.src().split( self.delimeter() )
        }
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
      }

      #[ inline ]
      pub fn former #generics() -> OptionsFormer #generics
      {
        Options::#generics::former()
      }

    }

    #[ inline ]
    fn split< 'a >() -> split::OptionsFormer< 'a >
    {
      split::former::< 'a >()
    }

  };

  // println!( "{:#?}", ast );
  // println!( "{:#?}", result );
  // let result = proc_macro2::TokenStream::new();
  Ok( result )
}
