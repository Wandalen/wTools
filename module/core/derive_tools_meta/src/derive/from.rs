use super::*;
use macro_tools::
{
  attr,
  diag,
  generic_params,
  item_struct,
  struct_like::StructLike,
  Result,
  AttributeComponent,
  AttributePropertyComponent,
  // AttributePropertySingletone,
  AttributePropertyOptionalSingletone,
};

use former_types::ComponentAssign;

//

pub fn from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  use macro_tools::quote::ToTokens;

  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref item ) | StructLike::Struct( ref item ) =>
    {

      let mut field_types = item_struct::field_types( &item );
      let field_names = item_struct::field_names( &item );

      match ( field_types.len(), field_names )
      {
        ( 0, _ ) =>
        generate_unit
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
        ),
        ( 1, Some( mut field_names ) ) =>
        generate_from_single_field_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names.next().unwrap(),
          &field_types.next().unwrap(),
        ),
        ( 1, None ) =>
        generate_from_single_field
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          &field_types.next().unwrap(),
        ),
        ( _, Some( field_names ) ) =>
        generate_from_multiple_fields_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names,
          field_types,
        ),
        ( _, None ) =>
        generate_from_multiple_fields
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_types,
        ),
      }

    },
    StructLike::Enum( ref item ) =>
    {

      let mut map = std::collections::HashMap::new();
      item.variants.iter().for_each( | variant |
      {
        map
        .entry( variant.fields.to_token_stream().to_string() )
        .and_modify( | e | *e += 1 )
        .or_insert( 1 );
      });

      let variants_result : Result< Vec< proc_macro2::TokenStream > > = item.variants.iter().map( | variant |
      {
        // don't do automatic off
        // if map[ & variant.fields.to_token_stream().to_string() ] <= 1
        if true
        {
          variant_generate
          (
            item_name,
            &item_attrs,
            &generics_impl,
            &generics_ty,
            &generics_where,
            variant,
            &original_input,
          )
        }
        else
        {
          Ok( qt!{} )
        }
      }).collect();

      let variants = variants_result?;

      qt!
      {
        #( #variants )*
      }
    },
  };

  if has_debug
  {
    let about = format!( "derive : From\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// qqq  : document, add example of generated code
fn variant_generate
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant : &syn::Variant,
  original_input : &proc_macro::TokenStream,
)
-> Result< proc_macro2::TokenStream >
{
  let variant_name = &variant.ident;
  let fields = &variant.fields;
  let attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;

  if !attrs.config.enabled.value( item_attrs.config.enabled.value( true ) )
  {
    return Ok( qt!{} )
  }

  if fields.len() <= 0
  {
    return Ok( qt!{} )
  }

  let ( args, use_src ) = if fields.len() == 1
  {
    let field = fields.iter().next().unwrap();
    (
      qt!{ #field },
      qt!{ src },
    )
  }
  else
  {
    let src_i = ( 0..fields.len() ).map( | e |
    {
      let i = syn::Index::from( e );
      qt!{ src.#i, }
    });
    (
      qt!{ #fields },
      qt!{ #( #src_i )* },
      // qt!{ src.0, src.1 },
    )
  };

  if attrs.config.debug.value( false )
  {
    let debug = format!
    (
      r#"
#[ automatically_derived ]
impl< {0} > From< {args} > for {item_name}< {1} >
where
  {2}
{{
  #[ inline ]
  fn from( src : {args} ) -> Self
  {{
    Self::{variant_name}( {use_src} )
  }}
}}
      "#,
      format!( "{}", qt!{ #generics_impl } ),
      format!( "{}", qt!{ #generics_ty } ),
      format!( "{}", qt!{ #generics_where } ),
    );
    let about = format!
    (
r#"derive : From
item : {item_name}
field : {variant_name}"#,
    );
    diag::report_print( about, original_input, debug );
  }

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > From< #args > for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline ]
        fn from( src : #args ) -> Self
        {
          Self::#variant_name( #use_src )
        }
      }
    }
  )

}

// qqq  : document, add example of generated code
fn generate_from_single_field_named
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_name : &syn::Ident,
  field_type : &syn::Type,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    // impl From < i32 > for MyStruct
    impl< #generics_impl > From< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: i32 ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self { a: src }
        Self { #field_name: src }
      }
    }
  }
}

// qqq  : document, add example of generated code
fn generate_from_single_field
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_type : &syn::Type,
)
-> proc_macro2::TokenStream
{

  qt!
  {
    #[automatically_derived]
    // impl From< bool > for IsTransparent
    impl< #generics_impl > From< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: bool ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self(src)
        Self(src)
      }
    }
  }
}

// qqq : for Petro : document, add example of generated code
fn generate_from_multiple_fields_named< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_names : Box< dyn macro_tools::IterTrait< 'a, &'a syn::Ident > + '_ >,
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
)
-> proc_macro2::TokenStream
{

  let params : Vec< proc_macro2::TokenStream > = field_names
  .enumerate()
  .map(| ( index, field_name ) |
  {
    let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
    qt! { #field_name : src.#index }
  })
  .collect();

  let field_types : Vec< _ > = field_types.collect();
  qt!
  {
    // impl From< (i32, bool) > for StructNamedFields
    impl< #generics_impl > From< (# ( #field_types ),* ) > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        // StructNamedFields{ a: src.0, b: src.1 }
        #item_name { #(#params),* }
      }
    }
  }

}

// qqq  : document, add example of generated code
fn generate_from_multiple_fields< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_types : impl macro_tools::IterTrait< 'a, &'a macro_tools::syn::Type >,
)
-> proc_macro2::TokenStream
{

  let params : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
  .map( | index |
  {
    let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
    qt!( src.#index )
  })
  .collect();

  let field_types : Vec< _ > = field_types.collect();

  qt!
  {
    // impl From< (i32, bool) > for StructWithManyFields
    impl< #generics_impl > From< (# ( #field_types ),* ) > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        // StructWithManyFields( src.0, src.1 )
        #item_name( #( #params ),* )
      }
    }
  }
}


// qqq  : document, add example of generated code
fn generate_unit
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    // impl From< () > for UnitStruct
    impl< #generics_impl > From< () > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn from( src : () ) -> Self
      {
        Self
      }
    }
  }
}

// == item attributes

///
/// Attributes of the whole tiem
///

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct ItemAttributes
{
  /// Attribute for customizing generated code.
  pub config : ItemAttributeConfig,
}

impl ItemAttributes
{

  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();

    let error = | attr : &syn::Attribute | -> syn::Error
    {
      let known_attributes = const_format::concatcp!
      (
        "Known attirbutes are : ",
        "debug",
        ", ", ItemAttributeConfig::KEYWORD,
        ".",
      );
      syn_err!
      (
        attr,
        "Expects an attribute of format '#[ attribute( key1 = val1, key2 = val2 ) ]'\n  {known_attributes}\n  But got: '{}'",
        qt!{ #attr }
      )
    };

    for attr in attrs
    {

      let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
      let key_str = format!( "{}", key_ident );

      if attr::is_standard( &key_str )
      {
        continue;
      }

      match key_str.as_ref()
      {
        ItemAttributeConfig::KEYWORD => result.assign( ItemAttributeConfig::from_meta( attr )? ),
        "debug" => {}
        _ => {},
        // _ => return Err( error( attr ) ),
        // attributes does not have to be known
      }
    }

    Ok( result )
  }

}

///
/// Attribute to hold parameters of forming for a specific field or variant.
/// For example to avoid code From generation for it.
///
/// `#[ from( on ) ]`
///

#[ derive( Debug, Default ) ]
pub struct ItemAttributeConfig
{
  /// Specifies whether `From` implementation for fields/variants should be generated by default.
  /// Can be altered using `on` and `off` attributes. But default it's `on`.
  /// `#[ from( on ) ]` - `From` is generated unless `off` for the field/variant is explicitly specified.
  /// `#[ from( off ) ]` - `From` is not generated unless `on` for the field/variant is explicitly specified.
  pub enabled : AttributePropertyEnabled,
}

impl AttributeComponent for ItemAttributeConfig
{
  const KEYWORD : &'static str = "from";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        return syn::parse2::< ItemAttributeConfig >( meta_list.tokens.clone() );
      },
      syn::Meta::Path( ref _path ) =>
      {
        return Ok( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ from( on ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > ComponentAssign< ItemAttributeConfig, IntoT > for ItemAttributes
where
  IntoT : Into< ItemAttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.config = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyEnabled, IntoT > for ItemAttributeConfig
where
  IntoT : Into< AttributePropertyEnabled >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.enabled = component.into();
  }
}

impl syn::parse::Parse for ItemAttributeConfig
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = const_format::concatcp!
      (
        "Known entries of attribute ", ItemAttributeConfig::KEYWORD, " are : ",
        EnabledMarker::KEYWORD_ON,
        ", ", EnabledMarker::KEYWORD_OFF,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ from( off ) ]'
  {known}
  But got: '{}'
"#,
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
          EnabledMarker::KEYWORD_ON => result.assign( AttributePropertyEnabled::from( true ) ),
          EnabledMarker::KEYWORD_OFF => result.assign( AttributePropertyEnabled::from( false ) ),
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

// == field attributes

///
/// Attributes of a field / variant
///

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct FieldAttributes
{
  /// Attribute for customizing generated code.
  pub config : FieldAttributeConfig,
}

impl FieldAttributes
{

  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();

    let error = | attr : &syn::Attribute | -> syn::Error
    {
      let known_attributes = const_format::concatcp!
      (
        "Known attirbutes are : ",
        "debug",
        ", ", FieldAttributeConfig::KEYWORD,
        ".",
      );
      syn_err!
      (
        attr,
        "Expects an attribute of format '#[ attribute( key1 = val1, key2 = val2 ) ]'\n  {known_attributes}\n  But got: '{}'",
        qt!{ #attr }
      )
    };

    for attr in attrs
    {

      let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
      let key_str = format!( "{}", key_ident );

      if attr::is_standard( &key_str )
      {
        continue;
      }

      match key_str.as_ref()
      {
        FieldAttributeConfig::KEYWORD => result.assign( FieldAttributeConfig::from_meta( attr )? ),
        "debug" => {}
        _ => return Err( error( attr ) ),
      }
    }

    Ok( result )
  }

}

///
/// Attribute to hold parameters of forming for a specific field or variant.
/// For example to avoid code From generation for it.
///
/// `#[ from( on ) ]`
///

#[ derive( Debug, Default ) ]
pub struct FieldAttributeConfig
{
  /// Specifies whether we should generate From implementation for the field.
  /// Can be altered using `on` and `off` attributes
  pub enabled : AttributePropertyEnabled,
  /// Specifies whether to print a sketch of generated `From` or not.
  /// Defaults to `false`, which means no code is printed unless explicitly requested.
  pub debug : AttributePropertyDebug,
  // qqq : apply debug properties to all brenches, not only enums
}

impl AttributeComponent for FieldAttributeConfig
{
  const KEYWORD : &'static str = "from";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        return syn::parse2::< FieldAttributeConfig >( meta_list.tokens.clone() );
      },
      syn::Meta::Path( ref _path ) =>
      {
        return Ok( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ from( on ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > ComponentAssign< FieldAttributeConfig, IntoT > for FieldAttributes
where
  IntoT : Into< FieldAttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.config = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyEnabled, IntoT > for FieldAttributeConfig
where
  IntoT : Into< AttributePropertyEnabled >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.enabled = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyDebug, IntoT > for FieldAttributeConfig
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for FieldAttributeConfig
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = const_format::concatcp!
      (
        "Known entries of attribute ", FieldAttributeConfig::KEYWORD, " are : ",
        AttributePropertyDebug::KEYWORD,
        ", ", EnabledMarker::KEYWORD_ON,
        ", ", EnabledMarker::KEYWORD_OFF,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ from( on ) ]'
  {known}
  But got: '{}'
"#,
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
          EnabledMarker::KEYWORD_ON => result.assign( AttributePropertyEnabled::from( true ) ),
          EnabledMarker::KEYWORD_OFF => result.assign( AttributePropertyEnabled::from( false ) ),
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

// == attribute properties

/// Marker type for attribute property to specify whether to provide a generated code as a hint.
/// Defaults to `false`, which means no debug is provided unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyDebugMarker;

impl AttributePropertyComponent for AttributePropertyDebugMarker
{
  const KEYWORD : &'static str = "debug";
}

/// Specifies whether to provide a generated code as a hint.
/// Defaults to `false`, which means no debug is provided unless explicitly requested.
pub type AttributePropertyDebug = AttributePropertyOptionalSingletone< AttributePropertyDebugMarker >;

// =

/// Marker type for attribute property to indicates whether `From` implementation for fields/variants should be generated.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct EnabledMarker;

impl EnabledMarker
{
  /// Keywords for parsing this attribute property.
  pub const KEYWORD_OFF : &'static str = "off";
  /// Keywords for parsing this attribute property.
  pub const KEYWORD_ON : &'static str = "on";
}

/// Specifies whether `From` implementation for fields/variants should be generated.
/// Can be altered using `on` and `off` attributes. But default it's `on`.
pub type AttributePropertyEnabled = AttributePropertyOptionalSingletone< EnabledMarker >;

// ==
