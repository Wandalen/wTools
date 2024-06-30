use super::*;
use former_types::Assign;
use macro_tools::
{
  ct,
  diag,
  Result,
  phantom::add_to_item,
  quote::ToTokens,
  syn::ItemStruct,
  AttributePropertyComponent,
  AttributePropertyOptionalSingletone
};

pub fn phantom( _attr : proc_macro::TokenStream, input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let attrs = syn::parse::< ItemAttributes >( _attr )?;
  let original_input = input.clone();
  let item_parsed = syn::parse::< ItemStruct >( input )?;

  let has_debug = attrs.debug.value( false );
  let item_name = &item_parsed.ident;
  let result = add_to_item( &item_parsed ).to_token_stream();

  if has_debug
  {
    let about = format!( "derive : PhantomData\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// == attributes

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct ItemAttributes
{
  /// Attribute for customizing generated code.
  pub debug : AttributePropertyDebug,
}

impl syn::parse::Parse for ItemAttributes
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
        let known = ct::concatcp!
        (
        "Known properties of attribute `phantom` are : ",
        AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ phantom( {} ) ]'
  {known}
  But got: '{}'
"#,
        AttributePropertyDebug::KEYWORD,
        qt!{ #ident }
      )
    };

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        match ident.to_string().as_str()
        {
          AttributePropertyDebug::KEYWORD => result.assign( AttributePropertyDebug::from( true ) ),
          _ => return Err( error( &ident ) ),
        }
      }
      else
      {
        return Err( lookahead.error() );
      }

      // Optional comma handling
      if input.peek( syn::Token![ , ] )
      {
        input.parse::< syn::Token![ , ] >()?;
      }
    }

    Ok( result )
  }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for ItemAttributes
  where
    IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, prop : IntoT )
  {
    self.debug = prop.into();
  }
}

// == attribute properties

/// Marker type for attribute property to specify whether to provide a generated code as a hint.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyDebugMarker;

impl AttributePropertyComponent for AttributePropertyDebugMarker
{
  const KEYWORD : &'static str = "debug";
}

/// Specifies whether to provide a generated code as a hint.
/// Defaults to `false`, which means no debug is provided unless explicitly requested.
pub type AttributePropertyDebug = AttributePropertyOptionalSingletone< AttributePropertyDebugMarker >;
