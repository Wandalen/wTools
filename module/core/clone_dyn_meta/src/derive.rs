
use macro_tools::prelude::*;
use macro_tools::
{
  Result,
  AttributePropertyOptionalSingletone,
  AttributePropertyComponent,
  diag,
  generic_params,
  ct,
};
use former_types::{ Assign };

//

pub fn clone_dyn( attr_input : proc_macro::TokenStream, item_input : proc_macro::TokenStream )
-> Result< proc_macro2::TokenStream >
{

  let attrs = syn::parse::< ItemAttributes >( attr_input )?;
  let original_input = item_input.clone();
  let mut item_parsed = match syn::parse::< syn::ItemTrait >( item_input )
  {
    Ok( original ) => original,
    Err( err ) => return Err( err ),
  };

  let has_debug = attrs.debug.value( false );
  let item_name = &item_parsed.ident;

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &item_parsed.generics );

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    where
      Self : clone_dyn::CloneDyn,
  };
  item_parsed.generics = generic_params::merge( &item_parsed.generics, &extra.into() );

  let result = qt!
  {
    #item_parsed

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generics_impl > Clone
    for Box< dyn #item_name< #generics_ty > + 'c >
    where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
    }

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generics_impl > Clone
    for Box< dyn #item_name< #generics_ty > + Send + 'c >
    where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
    }

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generics_impl > Clone
    for Box< dyn #item_name< #generics_ty > + Sync + 'c >
    where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
    }

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generics_impl > Clone
    for Box< dyn #item_name< #generics_ty > + Send + Sync + 'c >
    where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
    }

  };

  if has_debug
  {
    let about = format!( "macro : CloneDny\ntrait : {item_name}" );
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
        "Known properties of attribute `clone_dyn` are : ",
        AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ clone_dyn( {} ) ]'
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
