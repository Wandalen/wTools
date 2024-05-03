
use super::*;
use macro_tools::{ attr, Result };

///
/// Attributes of a field.
///

pub struct FieldAttributes
{
  pub config : Option< AttributeConfig >,
  pub scalar : Option< AttributeScalarSetter >,
  pub container : Option< AttributeContainerSetter >,
  pub subform : Option< AttributeSubformSetter >,
}

impl FieldAttributes
{
  // fn from_attrs( attributes : & Vec< syn::Attribute > ) -> Result< Self >
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut config = None;
    let mut scalar = None;
    let mut container = None;
    let mut subform = None;
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
            // syn::Meta::List( ref meta_list ) =>
            // {
            //   config.replace( syn::parse2::< AttributeConfig >( meta_list.tokens.clone() )? );
            // },
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
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ scalar( setter = false ) ]` or `#[ scalar( setter = false, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
          }
        }
        "container" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              container.replace( syn::parse2::< AttributeContainerSetter >( meta_list.tokens.clone() )? );
            },
            syn::Meta::Path( ref _path ) =>
            {
              container.replace( syn::parse2::< AttributeContainerSetter >( Default::default() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ container ]` or `#[ container( definition = former::VectorDefinition ) ]` if you want to use default container defition. \nGot: {}", qt!{ #attr } ),
          }
        }
        "subform" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              subform.replace( syn::parse2::< AttributeSubformSetter >( meta_list.tokens.clone() )? );
            },
            syn::Meta::Path( ref _path ) =>
            {
              subform.replace( syn::parse2::< AttributeSubformSetter >( Default::default() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform ]` or `#[ subform( name : child )` ], \nGot: {}", qt!{ #attr } ),
          }
        }
        _ =>
        {
          return Err( syn_err!( attr, "Unknown field attribute {}", qt!{ #attr } ) );
        }
      }
    }

    Ok( FieldAttributes { config, scalar, container, subform } )
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
  /// Such field should be present only in storage and should not be present in structure itself.
  /// That might be useful for parametrization of forming process.
  pub only_storage : Option< bool >,

}

impl AttributeConfig
{
}

impl syn::parse::Parse for AttributeConfig
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut default : Option< syn::Expr > = None;
    let mut only_storage : Option< bool > = None;

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
        else if ident == "only_storage"
        {
          input.parse::< syn::Token![ = ] >()?;
          let value : syn::LitBool = input.parse()?;
          only_storage = Some( value.value() );
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

    Ok( Self { default, only_storage } )
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

    Ok( Self { name, setter } )
  }
}

/// Represents an attribute for configuring container setter generation.
///
/// This struct is part of a meta-programming approach to enable detailed configuration of nested structs or collections such as `Vec< E >, HashMap< K, E >` and so on.
/// It allows the customization of setter methods and the specification of the container's behavior through meta attributes.
///
/// ## Example Input
///
/// The following is an example of a token stream that this struct can parse:
/// ```ignore
/// name = "custom_setter", setter = true, definition = former::VectorDefinition
/// ```
///

pub struct AttributeContainerSetter
{
  /// Optional identifier for naming the setter.
  pub name : Option< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : Option< bool >,
  /// Definition of the container former to use, e.g., `former::VectorSubformer`.
  pub definition : Option< syn::Type >,
}

impl AttributeContainerSetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl syn::parse::Parse for AttributeContainerSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut name : Option< syn::Ident > = None;
    let mut setter : Option< bool > = None; // Default is to generate a setter
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
        else if ident == "definition"
        {
          input.parse::< syn::Token![ = ] >()?;
          definition = Some( input.parse()? );
        }
        else
        {
          return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `container( name = myName, setter = true, definition = MyDefinition )`", ident ) ) );
        }
      }
      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `container( name = myName, setter = true, definition = MyDefinition )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![ , ] )
      {
        input.parse::< syn::Token![ , ] >()?;
      }
    }

    Ok( Self { name, setter, definition } )
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

pub struct AttributeSubformSetter
{
  /// An optional identifier that names the setter. It is parsed from inputs
  /// like `name = my_field`.
  pub name : Option< syn::Ident >,
  /// Disable generation of setter.
  /// It still generate `_field_add` method, so it could be used to make a setter with custom arguments.
  pub setter : Option< bool >,
}

impl AttributeSubformSetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl syn::parse::Parse for AttributeSubformSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut name : Option< syn::Ident > = None;
    let mut setter : Option< bool > = None;

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

    Ok( Self { name, setter } )
  }
}
