//!
//! Attributes of the whole item.
//!
#[ allow( clippy::wildcard_imports ) ]
use super::*;

use macro_tools::
{
  ct,
  Result,
  AttributeComponent,
  AttributePropertyComponent,
  AttributePropertyOptionalSingletone,
};

use former_types::{ Assign, OptionExt };

/// Represents the attributes of a struct, including storage fields, mutator, and perform attributes.

#[ derive( Debug, Default ) ]
pub struct ItemAttributes
{
  /// Optional attribute for storage-specific fields.
  /// This field is used to specify fields that should be part of the storage but not the final formed structure.
  pub storage_fields : Option< AttributeStorageFields >,

  /// Attribute for customizing the mutation process in a forming operation.
  /// The `mutator` attribute allows for specifying whether a custom mutator should be used or if a sketch should be provided as a hint.
  pub mutator : AttributeMutator,

  /// Optional attribute for specifying a method to call after forming.
  /// This attribute can hold information about a method that should be invoked after the form operation is complete.
  pub perform : Option< AttributePerform >,
}

impl ItemAttributes
{

  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();

    let error = | attr : &syn::Attribute | -> syn::Error
    {
      let known_attributes = ct::concatcp!
      (
        "Known attirbutes are : ",
        "debug",
        ", ", AttributeStorageFields::KEYWORD,
        ", ", AttributeMutator::KEYWORD,
        ", ", AttributePerform::KEYWORD,
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
      let key_str = format!( "{key_ident}" );

      // attributes does not have to be known
      // if attr::is_standard( &key_str )
      // {
      //   continue;
      // }

      match key_str.as_ref()
      {
        AttributeStorageFields::KEYWORD => result.assign( AttributeStorageFields::from_meta( attr )? ),
        AttributeMutator::KEYWORD => result.assign( AttributeMutator::from_meta( attr )? ),
        AttributePerform::KEYWORD => result.assign( AttributePerform::from_meta( attr )? ),
        // "debug" => {}
        _ => {},
        // _ => return Err( error( attr ) ),
        // attributes does not have to be known
      }
    }

    Ok( result )
  }

  ///
  /// Generate parts, used for generating `perform()` method.
  ///
  /// Similar to `form()`, but will also invoke function from `perform` attribute, if specified.
  ///
  /// # Example of returned tokens :
  ///
  /// ## perform :
  /// return result;
  ///
  /// ## `perform_output` :
  /// < T : `::core::default::Default` >
  ///
  /// ## `perform_generics` :
  /// Vec< T >
  ///
  #[ allow( clippy::unnecessary_wraps ) ]
  pub fn performer( &self )
  -> Result< ( TokenStream, TokenStream, TokenStream ) >
  {

    let mut perform = qt!
    {
      return result;
    };
    let mut perform_output = qt!{ Definition::Formed };
    let mut perform_generics = qt!{};

    if let Some( ref attr ) = self.perform
    {

      // let attr_perform = syn::parse2::< AttributePerform >( meta_list.tokens.clone() )?;
      let signature = &attr.signature;
      let generics = &signature.generics;
      perform_generics = qt!{ #generics };
      let perform_ident = &signature.ident;
      let output = &signature.output;
      if let syn::ReturnType::Type( _, boxed_type ) = output
      {
        perform_output = qt!{ #boxed_type };
      }
      perform = qt!
      {
        return result.#perform_ident();
      };

    }

    Ok( ( perform, perform_output, perform_generics ) )
  }

  /// Returns an iterator over the fields defined in the `storage_fields` attribute.
  ///
  /// This function provides an iterator that yields `syn::Field` objects. If `storage_fields` is set,
  /// it clones and iterates over its fields. If `storage_fields` is `None`, it returns an empty iterator.
  ///

  // pub fn storage_fields( &self ) -> impl Iterator< Item = syn::Field >
  pub fn storage_fields( &self ) -> &syn::punctuated::Punctuated< syn::Field, syn::token::Comma >
  {

    self.storage_fields.as_ref().map_or_else
    (
      || &*Box::leak( Box::new( syn::punctuated::Punctuated::new() ) ),
      | attr | &attr.fields
    )

    // qqq : find better solutioin

    // self.storage_fields
    // .as_ref()
    // .map_or_else(
    //   || syn::punctuated::Punctuated::< syn::Field, syn::token::Comma >::new().into_iter(),
    //   | attr | attr.fields.clone().into_iter()
    //   // Clone and create an iterator when storage_fields is Some
    // )
  }

}

///
/// Attribute to hold storage-specific fields.
/// Useful if formed structure should not have such fields.
///
/// `#[ storage_fields( a : i32, b : Option< String > ) ]`
///

#[ derive( Debug, Default ) ]
pub struct AttributeStorageFields
{
  pub fields : syn::punctuated::Punctuated< syn::Field, syn::token::Comma >,
}

impl AttributeComponent for AttributeStorageFields
{

  const KEYWORD : &'static str = "storage_fields";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeStorageFields >( meta_list.tokens.clone() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format #[ storage_fields( a : i32, b : Option< String > ) ]
.\nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< AttributeStorageFields, IntoT > for ItemAttributes
where
  IntoT : Into< AttributeStorageFields >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.storage_fields.option_assign( component );
  }
}

impl< IntoT > Assign< AttributeStorageFields, IntoT > for AttributeStorageFields
where
  IntoT : Into< AttributeStorageFields >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.fields = component.fields;
  }
}

impl syn::parse::Parse for AttributeStorageFields
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {

    let fields : syn::punctuated::Punctuated< syn::Field, syn::Token![ , ] > =
    input.parse_terminated( syn::Field::parse_named, Token![ , ] )?;

    Ok( Self
    {
      fields,
    })
  }
}

/// Represents attributes for customizing the mutation process in a forming operation.
///
/// `AttributeMutator` allows specifying whether a custom mutator should be used or a sketch should be provided
/// as a hint for developing a custom mutator. This is crucial for advanced scenarios where the entity's state
/// might require conditional modifications which are not handled by the standard `FormingEnd`.
///
/// ## Example of code
/// ```ignore
/// custom, debug
/// ```

#[ derive( Debug, Default ) ]
pub struct AttributeMutator
{
  /// Indicates whether a custom mutator should be generated.
  /// Defaults to `false`, meaning no custom mutator is generated unless explicitly requested.
  pub custom : AttributePropertyCustom,
  /// Specifies whether to provide a sketch of the mutator as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug : AttributePropertyDebug,
}

#[ allow( clippy::match_wildcard_for_single_variants ) ]
impl AttributeComponent for AttributeMutator
{
  const KEYWORD : &'static str = "mutator";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeMutator >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        Ok( AttributeMutator::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ mutator( custom ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< AttributeMutator, IntoT > for ItemAttributes
where
  IntoT : Into< AttributeMutator >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.mutator.assign( component );
  }
}

impl< IntoT > Assign< AttributeMutator, IntoT > for AttributeMutator
where
  IntoT : Into< AttributeMutator >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.custom.assign( component.custom );
    self.debug.assign( component.debug );
  }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for AttributeMutator
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}

impl< IntoT > Assign< AttributePropertyCustom, IntoT > for AttributeMutator
where
  IntoT : Into< AttributePropertyCustom >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.custom = component.into();
  }
}

impl syn::parse::Parse for AttributeMutator
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", AttributeMutator::KEYWORD, " are : ",
        AttributePropertyCustom::KEYWORD,
        ", ", AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ mutator( custom ) ]'
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
          AttributePropertyCustom::KEYWORD => result.assign( AttributePropertyCustom::from( true ) ),
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

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform( fn after1< 'a >() -> Option< &'a str > ) ]`
///

#[ derive( Debug ) ]
pub struct AttributePerform
{
  pub signature : syn::Signature,
}

impl AttributeComponent for AttributePerform
{
  const KEYWORD : &'static str = "perform";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {

    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributePerform >( meta_list.tokens.clone() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format #[ perform( fn parse( mut self ) -> Request ) ]
.\nGot: {}", qt!{ #attr } ),
    }
  }

}

impl syn::parse::Parse for AttributePerform
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    Ok( Self
    {
      signature : input.parse()?,
    })
  }
}

impl< IntoT > Assign< AttributePerform, IntoT > for ItemAttributes
where
  IntoT : Into< AttributePerform >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.perform.option_assign( component );
  }
}

impl< IntoT > Assign< AttributePerform, IntoT > for AttributePerform
where
  IntoT : Into< AttributePerform >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.signature = component.signature;
  }
}

// == attribute properties

/// Marker type for attribute property to specify whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct DebugMarker;

impl AttributePropertyComponent for DebugMarker
{
  const KEYWORD : &'static str = "debug";
}

/// Specifies whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
pub type AttributePropertyDebug = AttributePropertyOptionalSingletone< DebugMarker >;

// =

/// Marker type for attribute property to indicates whether a custom code should be generated.
/// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct CustomMarker;

impl AttributePropertyComponent for CustomMarker
{
  const KEYWORD : &'static str = "custom";
}

/// Indicates whether a custom code should be generated.
/// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
pub type AttributePropertyCustom = AttributePropertyOptionalSingletone< CustomMarker >;
