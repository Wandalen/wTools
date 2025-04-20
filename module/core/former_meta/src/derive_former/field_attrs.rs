//! Attributes of a field.
#[ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::
{
  ct,
  Result,
  AttributeComponent,
  AttributePropertyComponent,
  AttributePropertyOptionalBoolean,
  AttributePropertyOptionalSyn,
  AttributePropertyOptionalSingletone,
};
use former_types::{ Assign, OptionExt };

///
/// Attributes of a field.
///

#[ derive( Debug, Default ) ]
pub struct FieldAttributes
{
  /// Configuration attribute for a field.
  pub config : Option< AttributeConfig >,

  /// Scalar setter attribute for a field.
  pub scalar : Option< AttributeScalarSetter >,

  /// Subform scalar setter attribute for a field.
  pub subform_scalar : Option< AttributeSubformScalarSetter >,

  /// Subform collection setter attribute for a field.
  pub subform_collection : Option< AttributeSubformCollectionSetter >,

  /// Subform entry setter attribute for a field.
  pub subform_entry : Option< AttributeSubformEntrySetter >,

  /// Marks a field as a required argument for standalone constructors.
  pub arg_for_constructor : AttributePropertyArgForConstructor,
}

impl FieldAttributes
{

  /// Creates an instance of `FieldAttributes` from a list of attributes.
  ///
  /// # Parameters
  ///
  /// * `attrs`: An iterator over references to `syn::Attribute`.
  ///
  /// # Returns
  ///
  /// * `Result< Self >`: A result containing an instance of `FieldAttributes` on success,
  ///   or a `syn::Error` on failure.
  ///
  /// This function processes each attribute in the provided iterator and assigns the
  /// appropriate attribute type to the respective field in the `FieldAttributes` struct.
  ///
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();
    // Known attributes for error reporting
    let known_attributes = ct::concatcp!
    (
      "Known field attributes are : ",
      "debug", // Assuming debug might be handled elsewhere
      ", ", AttributeConfig::KEYWORD,
      ", ", AttributeScalarSetter::KEYWORD,
      ", ", AttributeSubformScalarSetter::KEYWORD,
      ", ", AttributeSubformCollectionSetter::KEYWORD,
      ", ", AttributeSubformEntrySetter::KEYWORD,
      ", ", AttributePropertyArgForConstructor::KEYWORD,
      ".",
    );

    // Helper closure to create a syn::Error for unknown attributes
    let error = | attr : &syn::Attribute | -> syn::Error
    {
      syn_err!
      (
        attr,
        "Expects an attribute of format `#[ attribute( key1 = val1, key2 = val2 ) ]`\n  {known_attributes}\n  But got:\n    `{}`",
        qt!{ #attr }
      )
    };

    // Iterate over the provided attributes
    for attr in attrs
    {
      // Get the attribute key as a string
      let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
      let key_str = format!( "{key_ident}" );

      // attributes does not have to be known
      // if attr::is_standard( &key_str )
      // {
      //   continue;
      // }

      // Match the attribute key and assign to the appropriate field
      match key_str.as_ref()
      {
        AttributeConfig::KEYWORD => result.assign( AttributeConfig::from_meta( attr )? ),
        AttributeScalarSetter::KEYWORD => result.assign( AttributeScalarSetter::from_meta( attr )? ),
        AttributeSubformScalarSetter::KEYWORD => result.assign( AttributeSubformScalarSetter::from_meta( attr )? ),
        AttributeSubformCollectionSetter::KEYWORD => result.assign( AttributeSubformCollectionSetter::from_meta( attr )? ),
        AttributeSubformEntrySetter::KEYWORD => result.assign( AttributeSubformEntrySetter::from_meta( attr )? ),
        AttributePropertyArgForConstructor::KEYWORD => result.assign( AttributePropertyArgForConstructor::from( true ) ),
        // "debug" => {}, // Assuming debug is handled elsewhere or implicitly
        _ => {}, // Allow unknown attributes
        // _ => return Err( error( attr ) ),
      }
    }

    Ok( result )
  }

}

// = Assign implementations for FieldAttributes =

impl< IntoT > Assign< AttributeConfig, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component : AttributeConfig = component.into();
    self.config.option_assign( component );
  }
}

impl< IntoT > Assign< AttributeScalarSetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeScalarSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.scalar.option_assign( component );
  }
}

impl< IntoT > Assign< AttributeSubformScalarSetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeSubformScalarSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.subform_scalar.option_assign( component );
  }
}

impl< IntoT > Assign< AttributeSubformCollectionSetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeSubformCollectionSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.subform_collection.option_assign( component );
  }
}

impl< IntoT > Assign< AttributeSubformEntrySetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeSubformEntrySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.subform_entry.option_assign( component );
  }
}

impl< IntoT > Assign< AttributePropertyArgForConstructor, IntoT > for FieldAttributes
where
  IntoT : Into< AttributePropertyArgForConstructor >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.arg_for_constructor.assign( component );
  }
}


///
/// Attribute to hold configuration information about the field such as default value.
///
/// `#[ default( 13 ) ]`
///

#[ derive( Debug, Default ) ]
pub struct AttributeConfig
{

  /// Default value to use for a field.
  pub default : AttributePropertyDefault,

}

impl AttributeComponent for AttributeConfig
{

  const KEYWORD : &'static str = "former";

  #[ allow( clippy::match_wildcard_for_single_variants ) ]
  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeConfig >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeConfig >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format #[ former( default = 13 ) ].\nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< AttributeConfig, IntoT > for AttributeConfig
where
  IntoT : Into< AttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.default.assign( component.default );
  }
}

impl< IntoT > Assign< AttributePropertyDefault, IntoT > for AttributeConfig
where
  IntoT : Into< AttributePropertyDefault >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.default.assign( component.into() );
  }
}

impl syn::parse::Parse for AttributeConfig
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", AttributeConfig::KEYWORD, " are : ",
        AttributePropertyDefault::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r"Expects an attribute of format '#[ former( default = 13 ) ]'
  {known}
  But got: '{}'
",
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
          AttributePropertyDefault::KEYWORD => result.assign( AttributePropertyDefault::parse( input )? ),
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

/// Attribute for scalar setters.
#[ derive( Debug, Default ) ]
pub struct AttributeScalarSetter
{
  /// Optional identifier for naming the setter.
  pub name : AttributePropertyName,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug : AttributePropertyDebug,
}

impl AttributeScalarSetter
{

  /// Should setter be generated or not?
  #[ allow( dead_code ) ]
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl AttributeComponent for AttributeScalarSetter
{

  const KEYWORD : &'static str = "scalar";

  #[ allow( clippy::match_wildcard_for_single_variants ) ]
  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeScalarSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeScalarSetter >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ scalar( setter = false ) ]` or `#[ scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< AttributeScalarSetter, IntoT > for AttributeScalarSetter
where
  IntoT : Into< AttributeScalarSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.name.assign( component.name );
    self.setter.assign( component.setter );
    self.debug.assign( component.debug );
  }
}

impl< IntoT > Assign< AttributePropertyName, IntoT > for AttributeScalarSetter
where
  IntoT : Into< AttributePropertyName >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > Assign< AttributePropertySetter, IntoT > for AttributeScalarSetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for AttributeScalarSetter
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeScalarSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", AttributeScalarSetter::KEYWORD, " are : ",
        AttributePropertyName::KEYWORD,
        ", ", AttributePropertySetter::KEYWORD,
        ", ", AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r"Expects an attribute of format '#[ scalar( name = myName, setter = true ) ]'
  {known}
  But got: '{}'
",
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
          AttributePropertyName::KEYWORD => result.assign( AttributePropertyName::parse( input )? ),
          AttributePropertySetter::KEYWORD => result.assign( AttributePropertySetter::parse( input )? ),
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

/// Attribute for subform scalar setters.
#[ derive( Debug, Default ) ]
pub struct AttributeSubformScalarSetter
{
  /// Optional identifier for naming the setter.
  pub name : AttributePropertyName,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug : AttributePropertyDebug,
}

impl AttributeSubformScalarSetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl AttributeComponent for AttributeSubformScalarSetter
{

  const KEYWORD : &'static str = "subform_scalar";

  #[ allow( clippy::match_wildcard_for_single_variants ) ]
  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformScalarSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformScalarSetter >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_scalar( setter = false ) ]` or `#[ subform_scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< AttributeSubformScalarSetter, IntoT > for AttributeSubformScalarSetter
where
  IntoT : Into< AttributeSubformScalarSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.name.assign( component.name );
    self.setter.assign( component.setter );
    self.debug.assign( component.debug );
  }
}

impl< IntoT > Assign< AttributePropertyName, IntoT > for AttributeSubformScalarSetter
where
  IntoT : Into< AttributePropertyName >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > Assign< AttributePropertySetter, IntoT > for AttributeSubformScalarSetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for AttributeSubformScalarSetter
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeSubformScalarSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", AttributeSubformScalarSetter::KEYWORD, " are : ",
        AttributePropertyName::KEYWORD,
        ", ", AttributePropertySetter::KEYWORD,
        ", ", AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r"Expects an attribute of format '#[ subform_scalar( name = myName, setter = true ) ]'
  {known}
  But got: '{}'
",
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
          AttributePropertyName::KEYWORD => result.assign( AttributePropertyName::parse( input )? ),
          AttributePropertySetter::KEYWORD => result.assign( AttributePropertySetter::parse( input )? ),
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

/// Attribute for subform collection setters.
#[ derive( Debug, Default ) ]
pub struct AttributeSubformCollectionSetter
{
  /// Optional identifier for naming the setter.
  pub name : AttributePropertyName,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug : AttributePropertyDebug,
  /// Definition of the collection former to use, e.g., `former::VectorFormer`.
  pub definition : AttributePropertyDefinition,
}

impl AttributeSubformCollectionSetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl AttributeComponent for AttributeSubformCollectionSetter
{

  const KEYWORD : &'static str = "subform_collection";

  #[ allow( clippy::match_wildcard_for_single_variants ) ]
  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformCollectionSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformCollectionSetter >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_collection ]` or `#[ subform_collection( definition = former::VectorDefinition ) ]` if you want to use default collection defition. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< AttributeSubformCollectionSetter, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributeSubformCollectionSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.name.assign( component.name );
    self.setter.assign( component.setter );
    self.debug.assign( component.debug );
    self.definition.assign( component.definition );
  }
}

impl< IntoT > Assign< AttributePropertyName, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertyName >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > Assign< AttributePropertySetter, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > Assign< AttributePropertyDefinition, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertyDefinition >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.definition = component.into();
  }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeSubformCollectionSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", AttributeSubformCollectionSetter::KEYWORD, " are : ",
        AttributePropertyName::KEYWORD,
        ", ", AttributePropertySetter::KEYWORD,
        ", ", AttributePropertyDebug::KEYWORD,
        ", ", AttributePropertyDefinition::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r"Expects an attribute of format '#[ subform_collection( name = myName, setter = true, debug, definition = MyDefinition ) ]'
  {known}
  But got: '{}'
",
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
          AttributePropertyName::KEYWORD => result.assign( AttributePropertyName::parse( input )? ),
          AttributePropertySetter::KEYWORD => result.assign( AttributePropertySetter::parse( input )? ),
          AttributePropertyDebug::KEYWORD => result.assign( AttributePropertyDebug::from( true ) ),
          AttributePropertyDefinition::KEYWORD => result.assign( AttributePropertyDefinition::parse( input )? ),
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

/// Attribute for subform entry setters.
#[ derive( Debug, Default ) ]
pub struct AttributeSubformEntrySetter
{
  /// An optional identifier that names the setter. It is parsed from inputs
  /// like `name = my_field`.
  pub name : AttributePropertyName,
  /// Disable generation of setter.
  /// It still generate `_field_subform_entry` method, so it could be used to make a setter with custom arguments.
  pub setter : AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug : AttributePropertyDebug,
}

impl AttributeSubformEntrySetter
{

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.as_ref().is_none() || self.setter.as_ref().unwrap()
  }

}

impl AttributeComponent for AttributeSubformEntrySetter
{

  const KEYWORD : &'static str = "subform_entry";

  #[ allow( clippy::match_wildcard_for_single_variants ) ]
  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformEntrySetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformEntrySetter >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_entry ]` or `#[ subform_entry( name : child )` ], \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< AttributeSubformEntrySetter, IntoT > for AttributeSubformEntrySetter
where
  IntoT : Into< AttributeSubformEntrySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.name.assign( component.name );
    self.setter.assign( component.setter );
    self.debug.assign( component.debug );
  }
}

impl< IntoT > Assign< AttributePropertyName, IntoT > for AttributeSubformEntrySetter
where
  IntoT : Into< AttributePropertyName >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > Assign< AttributePropertySetter, IntoT > for AttributeSubformEntrySetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for AttributeSubformEntrySetter
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeSubformEntrySetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", AttributeSubformEntrySetter::KEYWORD, " are : ",
        AttributePropertyName::KEYWORD,
        ", ", AttributePropertySetter::KEYWORD,
        ", ", AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r"Expects an attribute of format '#[ subform( name = myName, setter = true ) ]'
  {known}
  But got: '{}'
",
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
          AttributePropertyName::KEYWORD => result.assign( AttributePropertyName::parse( input )? ),
          AttributePropertySetter::KEYWORD => result.assign( AttributePropertySetter::parse( input )? ),
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

// == attribute properties ==

// =

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

/// Disable generation of setter.
/// Attributes still might generate some helper methods to reuse by custom setter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct SetterMarker;

impl AttributePropertyComponent for SetterMarker
{
  const KEYWORD : &'static str = "setter";
}

/// Disable generation of setter.
/// Attributes still might generate some helper methods to reuse by custom setter.
pub type AttributePropertySetter = AttributePropertyOptionalBoolean< SetterMarker >;

// =

/// Marker type for attribute property of optional identifier that names the setter. It is parsed from inputs
/// like `name = my_field`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct NameMarker;

impl AttributePropertyComponent for NameMarker
{
  const KEYWORD : &'static str = "name";
}

/// An optional identifier that names the setter. It is parsed from inputs
/// like `name = my_field`.
pub type AttributePropertyName = AttributePropertyOptionalSyn< syn::Ident, NameMarker >;

// =

/// Marker type for default value to use for a field.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct DefaultMarker;

impl AttributePropertyComponent for DefaultMarker
{
  const KEYWORD : &'static str = "default";
}

/// An optional identifier that names the setter. It is parsed from inputs
/// like `name = my_field`.
pub type AttributePropertyDefault = AttributePropertyOptionalSyn< syn::Expr, DefaultMarker >;

// =

/// Marker type for definition of the collection former to use, e.g., `former::VectorFormer`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct DefinitionMarker;

impl AttributePropertyComponent for DefinitionMarker
{
  const KEYWORD : &'static str = "definition";
}

/// Definition of the collection former to use, e.g., `former::VectorFormer`.
pub type AttributePropertyDefinition = AttributePropertyOptionalSyn< syn::Type, DefinitionMarker >;

// =

/// Marker type for attribute property marking a field as a constructor argument.
/// Defaults to `false`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct ArgForConstructorMarker;

impl AttributePropertyComponent for ArgForConstructorMarker
{
  const KEYWORD : &'static str = "arg_for_constructor";
}

/// Indicates whether a field should be an argument for standalone constructors.
/// Defaults to `false`. Parsed as a singletone attribute (`#[arg_for_constructor]`).
pub type AttributePropertyArgForConstructor = AttributePropertyOptionalSingletone< ArgForConstructorMarker >;