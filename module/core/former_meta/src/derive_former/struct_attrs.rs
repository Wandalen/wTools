
use super::*;
use macro_tools::{ attr, Result };

///
/// Definition of a field.
///

///
/// Attributes of a struct.
///

pub struct StructAttributes
{
  pub perform : Option< AttributePerform >,
  pub storage_fields : Option< AttributeStorageFields >,
}

impl StructAttributes
{
  // fn from_attrs( attributes : & Vec< syn::Attribute > ) -> Result< Self >
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut perform = None;
    let mut storage_fields = None;

    for attr in attrs
    {
      let key_ident = attr.path().get_ident()
      .ok_or_else( || syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ) )?;
      let key_str = format!( "{}", key_ident );

      if attr::is_standard( &key_str )
      {
        continue;
      }

      match key_str.as_ref()
      {
        "storage_fields" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              storage_fields.replace( syn::parse2::< AttributeStorageFields >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ storage_fields( a : i32, b : Option< String > ) ]
.\nGot: {}", qt!{ #attr } ),
          }
        }
        "perform" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              perform.replace( syn::parse2::< AttributePerform >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ perform( fn parse( mut self ) -> Request ) ]
.\nGot: {}", qt!{ #attr } ),
          }
        }
        "debug" =>
        {
        }
        _ =>
        {
          return Err( syn_err!( attr, "Known structure attirbutes are : `storage_fields`, `perform`, `debug`.\nUnknown structure attribute : {}", qt!{ #attr } ) );
        }
      }
    }

    Ok( StructAttributes { perform, storage_fields } )
  }
}

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform( fn after1< 'a >() -> Option< &'a str > ) ]`
///

pub struct AttributePerform
{
  pub signature : syn::Signature,
}

impl syn::parse::Parse for AttributePerform
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    // let input2;
    Ok( Self
    {
      // paren_token : syn::parenthesized!( input2 in input ),
      // signature : input2.parse()?,
      signature : input.parse()?,
    })
  }
}

///
/// Attribute to hold storage-specific fields.
/// Useful if formed structure should not have such fields.
///
/// `#[ storage_fields( a : i32, b : Option< String > ) ]`
///

pub struct AttributeStorageFields
{
  pub fields : syn::punctuated::Punctuated< syn::Field, syn::token::Comma >,
}

impl syn::parse::Parse for AttributeStorageFields
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {

    let fields : syn::punctuated::Punctuated< syn::Field, syn::Token![,] > =
    input.parse_terminated( syn::Field::parse_named, Token![,] )?;

    Ok( Self
    {
      fields,
      // fields : syn::Fields::Named( syn::FieldsNamed
      // {
      //   brace_token : Default::default(),
      //   named : fields,
      // }),
    })
  }
}


//

///
/// Generate parts, used for generating `perform()`` method.
///
/// Similar to `form()`, but will also invoke function from `perform` attribute, if specified.
///
/// # Example of returned tokens :
///
/// ## perform :
/// return result;
///
/// ## perform_output :
/// < T : ::core::default::Default >
///
/// ## perform_generics :
/// Vec< T >
///

impl StructAttributes
{

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

  pub fn storage_fields( &self ) -> impl Iterator< Item = syn::Field >
  {
    self.storage_fields
    .as_ref()
    .map_or_else(
      || syn::punctuated::Punctuated::< syn::Field, syn::token::Comma >::new().into_iter(),
      | attr | attr.fields.clone().into_iter() // Clone and create an iterator when storage_fields is Some
    )
  }

  /// xxx : write documentation. provide example of generated code

  pub fn storage_fields_code( &self )
  -> Result< TokenStream >
  {

    let mut result = qt!
    {
    };

    if let Some( ref attr ) = self.storage_fields
    {
      let storage_fields = &attr.fields;
      result = qt! { #storage_fields }
    }

    Ok( result )
  }

}
