use super::*;
use quote::ToTokens;

#[ test ]
fn attr_prop_test()
{
  use the_module::{ AttributePropertyComponent, AttributePropertyBoolean, AttributePropertyOptionalSingletone };

  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct DebugMarker;

  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct EnabledMarker;

  // pub trait AttributePropertyComponent
  // {
  //   const KEYWORD : &'static str;
  // }

  impl AttributePropertyComponent for DebugMarker
  {
    const KEYWORD : &'static str = "debug";
  }

  impl AttributePropertyComponent for EnabledMarker
  {
    const KEYWORD : &'static str = "enabled";
  }

  #[ derive( Debug, Default ) ]
  struct MyAttributes
  {
    pub debug : AttributePropertyBoolean< DebugMarker >,
    pub enabled : AttributePropertyBoolean< EnabledMarker >,
  }

  impl syn::parse::Parse for MyAttributes
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let mut debug = AttributePropertyBoolean::< DebugMarker >::default();
      let mut enabled = AttributePropertyBoolean::< EnabledMarker >::default();

      while !input.is_empty()
      {
        let lookahead = input.lookahead1();
        if lookahead.peek( syn::Ident )
        {
          let ident : syn::Ident = input.parse()?;
          match ident.to_string().as_str()
          {
            DebugMarker::KEYWORD => debug = input.parse()?,
            EnabledMarker::KEYWORD => enabled = input.parse()?,
            _ => return Err( lookahead.error() ),
          }
        }
        else
        {
          return Err( lookahead.error() );
        }

        // Optional comma handling
        if input.peek( syn::Token![,] )
        {
          input.parse::< syn::Token![,] >()?;
        }
      }

      Ok( MyAttributes { debug, enabled } )
    }
  }

  let input : syn::Attribute = syn::parse_quote!( #[ attribute( enabled = true ) ] );
  let syn::Meta::List( meta ) = input.meta else { panic!( "Expected a Meta::List" ) };

  let nested_meta_stream : proc_macro2::TokenStream = meta.tokens;
  let attrs : MyAttributes = syn::parse2( nested_meta_stream ).unwrap();
  println!( "{attrs:?}" );

  let attr : AttributePropertyBoolean< DebugMarker > = AttributePropertyBoolean::default();
  assert!( !attr.internal() );
  let attr : AttributePropertyBoolean< DebugMarker > = true.into();
  assert!( attr.internal() );
  let attr : AttributePropertyBoolean< DebugMarker > = false.into();
  assert!( !attr.internal() );

  let input : syn::Attribute = syn::parse_quote!( #[ attribute( enabled = true ) ] );
  let syn::Meta::List( meta ) = input.meta else { panic!( "Expected a Meta::List" ) };

  let nested_meta_stream : proc_macro2::TokenStream = meta.tokens;
  let parsed : MyAttributes = syn::parse2( nested_meta_stream ).unwrap();
  assert!( parsed.enabled.internal() );
  assert!( !parsed.debug.internal() );

}

#[ test ]
fn attribute_property_enabled()
{
  use the_module::AttributePropertyOptionalSingletone;

  // Test default value
  let attr : AttributePropertyOptionalSingletone = AttributePropertyOptionalSingletone::default();
  assert_eq!( attr.internal(), None );
  assert!( attr.value( true ) );
  assert!( !attr.value( false ) );

}
