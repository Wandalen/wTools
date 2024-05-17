
use super::*;
use macro_tools::{ attr, Result };

///
/// Attributes of a field.
///

pub struct FieldAttributes
{
  pub config : Option< AttributeConfig >,
  pub scalar : Option< AttributeScalarSetter >,
  pub subform_scalar : Option< AttributeSubformScalarSetter >,
  pub subform_collection : Option< AttributeSubformCollectionSetter >,
  pub subform_entry : Option< AttributeSubformEntrySetter >,
}

impl FieldAttributes
{
  // fn from_attrs( attributes : & Vec< syn::Attribute > ) -> Result< Self >
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut config = None;
    let mut scalar = None;
    let mut subform_scalar = None;
    let mut subform_collection = None;
    let mut subform_entry = None;
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
        "former" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              config.replace( syn::parse2::< AttributeConfig >( meta_list.tokens.clone() )? );
            },
            syn::Meta::Path( ref _path ) =>
            {
              config.replace( syn::parse2::< AttributeConfig >( Default::default() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ former( default = 13 ) ].\nGot: {}", qt!{ #attr } ),
          }
        }
        "scalar" =>
        {
          // qqq : move this part of parsing into attribute. do that for all attributes
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              scalar.replace( syn::parse2::< AttributeScalarSetter >( meta_list.tokens.clone() )? );
            },
            syn::Meta::Path( ref _path ) =>
            {
              scalar.replace( syn::parse2::< AttributeScalarSetter >( Default::default() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ scalar( setter = false ) ]` or `#[ scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
          }
        }
        "subform_scalar" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              subform_scalar.replace( syn::parse2::< AttributeSubformScalarSetter >( meta_list.tokens.clone() )? );
            },
            syn::Meta::Path( ref _path ) =>
            {
              subform_scalar.replace( syn::parse2::< AttributeSubformScalarSetter >( Default::default() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_scalar( setter = false ) ]` or `#[ subform_scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
          }
        }
        "subform_collection" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              subform_collection.replace( syn::parse2::< AttributeSubformCollectionSetter >( meta_list.tokens.clone() )? );
            },
            syn::Meta::Path( ref _path ) =>
            {
              subform_collection.replace( syn::parse2::< AttributeSubformCollectionSetter >( Default::default() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_collection ]` or `#[ subform_collection( definition = former::VectorDefinition ) ]` if you want to use default collection defition. \nGot: {}", qt!{ #attr } ),
          }
        }
        "subform_entry" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              subform_entry.replace( syn::parse2::< AttributeSubformEntrySetter >( meta_list.tokens.clone() )? );
            },
            syn::Meta::Path( ref _path ) =>
            {
              subform_entry.replace( syn::parse2::< AttributeSubformEntrySetter >( Default::default() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_entry ]` or `#[ subform_entry( name : child )` ], \nGot: {}", qt!{ #attr } ),
          }
        }
        _ =>
        {
          return Err( syn_err!( attr, "Unknown field attribute {}", qt!{ #attr } ) );
        }
      }
    }

    Ok( FieldAttributes { config, scalar, subform_scalar, subform_collection, subform_entry } )
  }
}

///
/// Attribute to hold configuration information about the field such as default value.
///
/// `#[ default( 13 ) ]`
///

pub struct AttributeConfig
{

  /// Default value to use for the field.
  pub default : Option< syn::Expr >,

}

impl AttributeConfig
{
}

impl syn::parse::Parse for AttributeConfig
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut default : Option< syn::Expr > = None;
    // let mut only_storage : Option< bool > = None;

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        if ident == "default"
        {
          input.parse::< syn::Token![ = ] >()?;
          default = Some( input.parse()? );
        }
        else
        {
          return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'default'. For example: `former( default = 13 )`", ident ) ) );
        }
      }

      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'default'. For example: `former( default = 13 )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![ , ] )
      {
        input.parse::< syn::Token![ , ] >()?;
      }
    }

    Ok( Self { default } )
  }
}

///
/// Attribute to enable/disable scalar setter generation.
///
/// ## Example Input
///
/// A typical input to parse might look like the following:
///
/// ```ignore
/// name = field_name, setter = true
/// ```
///

pub struct AttributeScalarSetter
{
  /// Optional identifier for naming the setter.
  pub name : Option< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : Option< bool >,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub hint : bool,
}

#[ allow( dead_code ) ]
impl AttributeScalarSetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl syn::parse::Parse for AttributeScalarSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut name : Option< syn::Ident > = None;
    let mut setter : Option< bool > = None;
    let mut hint = false;

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        if ident == "name"
        {
          input.parse::< syn::Token![ = ] >()?;
          name = Some( input.parse()? );
        }
        else if ident == "setter"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          setter = Some( value.value() );
        }
        else if ident == "hint"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          hint = value.value;
        }
        else
        {
          return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `scalar( name = myName, setter = true )`", ident ) ) );
        }
      }
      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `scalar( name = myName, setter = true )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![,] )
      {
        input.parse::< syn::Token![,] >()?;
      }
    }

    Ok( Self { name, setter, hint } )
  }
}

///
/// Attribute to enable/disable scalar setter generation.
///
/// ## Example Input
///
/// A typical input to parse might look like the following:
///
/// ```ignore
/// name = field_name, setter = true
/// ```
///

pub struct AttributeSubformScalarSetter
{
  /// Optional identifier for naming the setter.
  pub name : Option< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : Option< bool >,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub hint : bool,
}

#[ allow( dead_code ) ]
impl AttributeSubformScalarSetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl syn::parse::Parse for AttributeSubformScalarSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut name : Option< syn::Ident > = None;
    let mut setter : Option< bool > = None;
    let mut hint = false;

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        if ident == "name"
        {
          input.parse::< syn::Token![ = ] >()?;
          name = Some( input.parse()? );
        }
        else if ident == "setter"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          setter = Some( value.value() );
        }
        else if ident == "hint"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          hint = value.value;
        }
        else
        {
          return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `subform_scalar( name = myName, setter = true )`", ident ) ) );
        }
      }
      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `subform_scalar( name = myName, setter = true )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![,] )
      {
        input.parse::< syn::Token![,] >()?;
      }
    }

    Ok( Self { name, setter, hint } )
  }
}

/// Represents an attribute for configuring collection setter generation.
///
/// This struct is part of a meta-programming approach to enable detailed configuration of nested structs or collections such as `Vec< E >, HashMap< K, E >` and so on.
/// It allows the customization of setter methods and the specification of the collection's behavior through meta attributes.
///
/// ## Example Input
///
/// The following is an example of a token stream that this struct can parse:
/// ```ignore
/// name = "custom_setter", setter = true, definition = former::VectorDefinition
/// ```
///

pub struct AttributeSubformCollectionSetter
{
  /// Optional identifier for naming the setter.
  pub name : Option< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : Option< bool >,
  /// Definition of the collection former to use, e.g., `former::VectorFormer`.
  pub definition : Option< syn::Type >,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub hint : bool,
}

impl AttributeSubformCollectionSetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl syn::parse::Parse for AttributeSubformCollectionSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut name : Option< syn::Ident > = None;
    let mut setter : Option< bool > = None; // Default is to generate a setter
    let mut hint = false;
    let mut definition : Option< syn::Type > = None;

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        if ident == "name"
        {
          input.parse::< syn::Token![ = ] >()?;
          name = Some( input.parse()? );
        }
        else if ident == "setter"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          setter = Some( value.value );
        }
        else if ident == "hint"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          hint = value.value;
        }
        else if ident == "definition"
        {
          input.parse::< syn::Token![ = ] >()?;
          definition = Some( input.parse()? );
        }
        else
        {
          return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `collection( name = myName, setter = true, definition = MyDefinition )`", ident ) ) );
        }
      }
      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `collection( name = myName, setter = true, definition = MyDefinition )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![ , ] )
      {
        input.parse::< syn::Token![ , ] >()?;
      }
    }

    Ok( Self { name, setter, hint, definition } )
  }
}

/// Represents a subform attribute to control subform setter generation.
/// Used to specify extra options for using one former as subformer of another one.
/// For example name of setter could be customized.
///
/// ## Example Input
///
/// A typical input to parse might look like the following:
///
/// ```ignore
/// name = field_name, setter = true
/// ```
///
/// or simply:
///
/// ```ignore
/// mame = field_name
/// ```

pub struct AttributeSubformEntrySetter
{
  /// An optional identifier that names the setter. It is parsed from inputs
  /// like `name = my_field`.
  pub name : Option< syn::Ident >,
  /// Disable generation of setter.
  /// It still generate `_field_subform_entry` method, so it could be used to make a setter with custom arguments.
  pub setter : Option< bool >,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub hint : bool,
}

impl AttributeSubformEntrySetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl syn::parse::Parse for AttributeSubformEntrySetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut name : Option< syn::Ident > = None;
    let mut setter : Option< bool > = None;
    let mut hint = false;

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        if ident == "name"
        {
          input.parse::< syn::Token![ = ] >()?;
          name = Some( input.parse()? );
        }
        else if ident == "setter"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          setter = Some( value.value() );
        }
        else if ident == "hint"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          hint = value.value;
        }
        else
        {
          return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `subform( name = myName, setter = true )`", ident ) ) );
        }
      }
      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `subform( name = myName, setter = true )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![,] )
      {
        input.parse::< syn::Token![,] >()?;
      }
    }

    Ok( Self { name, setter, hint } )
  }
}
