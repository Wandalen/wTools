
use meta_tools_min::*;
use proc_macro_tools::quote::{ ToTokens, TokenStreamExt };
use proc_macro_tools::syn::parse::*;
use proc_macro_tools::syn::spanned::Spanned;
use proc_macro_tools::*;
use std::collections::HashMap;
use iter_tools::{ /* Itertools, */ process_results };
use convert_case::{Case, Casing};

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
  #[ allow( dead_code ) ]
  Signature( FnQuick ),
  Field( syn::Field ),
}

impl Parse for Element
{
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
  {

    let attrs : Vec< syn::Attribute > = input.call( syn::Attribute::parse_outer )?;
    let vis : syn::Visibility = input.parse()?;

    // zzz : remove lookahead, use input
    let lookahead1 = input.lookahead1();
    if lookahead1.peek( syn::Token!{ fn } )
    {
      let sig : syn::Signature = input.parse()?;

      let lookahead2 = input.lookahead1();
      if lookahead2.peek( syn::token::Brace )
      {
        let input2;
        let _brace_token : syn::token::Brace = syn::braced!( input2 in input );
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
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
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
          let key = f.ident.as_ref().ok_or_else( || syn_err!( &f.clone(), "Field does not have name: {}", qt!{ #f } ) )?.to_string();
          fields_map.insert( key, f );
        },
      }
    }

    let result = OptionsDescriptor
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
pub struct AccessorDescriptor
{
  attr : proc_macro2::TokenStream,
  signature : proc_macro2::TokenStream,
  body : proc_macro2::TokenStream,
}

//

impl quote::ToTokens for AccessorDescriptor
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

fn getter_gen( name : &str, field : &syn::Field ) -> Result< AccessorDescriptor >
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
    qt!{ & #ty }
  };

  let attr = qt!
  {
    #[ inline ]
  };

  let signature = qt!
  {
    fn #name_ident( &self ) -> #ty2
  };

  let body = qt!
  {
    {
      &self.#name_ident
    }
  };

  let result = AccessorDescriptor
  {
    attr,
    signature,
    body,
  };

  Ok( result )
}

///
/// Generate a mutter for a field.
///

fn mutter_gen( name : &str, field : &syn::Field ) -> Result< AccessorDescriptor >
{

  let name_ident = syn::Ident::new( &name, field.span() );
  let name_mut_ident = syn::Ident::new( &format!( "{}_mut", name ), field.span() );
  let ty = &field.ty;

  // tree_print!( ty );

  let ty2 = qt!{ &mut #ty };

  let attr = qt!
  {
    #[ inline ]
  };

  let signature = qt!
  {
    fn #name_mut_ident( &mut self ) -> #ty2
  };

  let body = qt!
  {
    {
      &mut self.#name_ident
    }
  };

  let result = AccessorDescriptor
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

  let mut perform = qt!{};
  let mut attr_perform = qt!{};
  if let Some( perform_fn ) = options_descriptor.methods_map.get( "perform" )
  {
    let sig = &perform_fn.sig;
    attr_perform = qt!{ #[ perform( #sig ) ] };
    perform = qt!
    {
      #[ allow( unused_attributes ) ]
      #[ inline ]
      #perform_fn
    }
  }

  ( perform, attr_perform )
}

///
/// Options macro handler.
///

pub fn options( _attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let options_descriptor = match syn::parse::< OptionsDescriptor >( item )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  let name_ident = &options_descriptor.ident;
  let name_options_adapter_str = String::from( name_ident.to_string() ).from_case( Case::Snake ).to_case( Case::UpperCamel );
  let name_options_adapter_str = format!( "{}OptionsAdapter", name_options_adapter_str );
  let name_options_adapter_ident = syn::Ident::new( &name_options_adapter_str, name_ident.span() );
  let generics = &options_descriptor.generics;
  let attrs = &options_descriptor.attrs;

  let mut fields_define = Vec::< &syn::Field >::new();
  for ( _name, field ) in options_descriptor.fields_map.iter()
  {
    fields_define.push( field );
  }

  let ( perform, attr_perform ) = perform_gen( &options_descriptor );

  let getters = options_descriptor.fields_map.iter().map( | ( key, field ) | getter_gen( key, field ) );
  let getters : Vec< _ > = process_results( getters, | iter | iter.collect() )?;
  let getters_signatures : Vec< _ > = getters.iter().map( | e | e.signature.clone() ).collect();

  let mutters = options_descriptor.fields_map.iter().map( | ( key, field ) | mutter_gen( key, field ) );
  let mutters : Vec< _ > = process_results( mutters, | iter | iter.collect() )?;
  let mutters_signatures : Vec< _ > = mutters.iter().map( | e | e.signature.clone() ).collect();

  let result = qt!
  {

    pub mod #name_ident
    {

      // #[cfg( feature = "in_wtools" )]
      // use ::wtools::options::*;
      // #[cfg( not( feature = "in_wtools" ) )]
      use super::Former;

      #( #attrs )*
      #[ derive( Former, PartialEq, Debug ) ]
      #attr_perform
      pub struct Options #generics
      {
        #( #fields_define, )*
      }

      pub trait OptionsAdapter #generics
      {
        #( #getters_signatures ; )*
        #( #mutters_signatures ; )*
        #perform
      }

      impl #generics OptionsAdapter #generics for Options #generics
      {
        #( #getters )*
        #( #mutters )*
      }

      #[ inline ]
      pub fn former #generics() -> OptionsFormer #generics
      {
        Options::#generics::former()
      }

      /// Namespace of the module to include with `use module::*`.
      pub mod prelude
      {
        pub use super::OptionsAdapter as #name_options_adapter_ident;
      }

    }

    #[ inline ]
    pub fn #name_ident #generics () -> #name_ident::OptionsFormer #generics
    {
      #name_ident::former::#generics()
    }

  };

  Ok( result )
}
