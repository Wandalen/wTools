
use super::*;
use macro_tools::{ attr, Result };
use former_types::{ ComponentAssign };

// xxx : document

///
/// Attributes of a field.
///

#[ derive( Debug, Default ) ]
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

  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();
    // let known_attributes = "Known structure attirbutes are : `storage_fields`, `mutator`, `perform`, `debug`.";
    let known_attributes = const_format::concatcp!
    (
      "Known attirbutes are : ",
      "debug",
      ", ", AttributeConfig::KEYWORD,
      ", ", AttributeScalarSetter::KEYWORD,
      ", ", AttributeSubformScalarSetter::KEYWORD,
      ", ", AttributeSubformCollectionSetter::KEYWORD,
      ", ", AttributeSubformEntrySetter::KEYWORD,
      ".",
    );

    let error = | attr : &syn::Attribute | -> syn::Error
    {
      syn_err!
      (
        attr,
        "Expects an attribute of format `#[ attribute( key1 = val1, key2 = val2 ) ]`\n  {known_attributes}\n  But got:\n    `{}`",
        qt!{ #attr }
      )
    };

    for attr in attrs
    {

      // return Err( error( attr ) );

      let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
      let key_str = format!( "{}", key_ident );

      if attr::is_standard( &key_str )
      {
        continue;
      }

      match key_str.as_ref()
      {
        AttributeConfig::KEYWORD => result.assign( AttributeConfig::from_meta( attr )? ),
        AttributeScalarSetter::KEYWORD => result.assign( AttributeScalarSetter::from_meta( attr )? ),
        AttributeSubformScalarSetter::KEYWORD => result.assign( AttributeSubformScalarSetter::from_meta( attr )? ),
        AttributeSubformCollectionSetter::KEYWORD => result.assign( AttributeSubformCollectionSetter::from_meta( attr )? ),
        AttributeSubformEntrySetter::KEYWORD => result.assign( AttributeSubformEntrySetter::from_meta( attr )? ),
        "debug" => {}
        _ => return Err( error( attr ) ),
      }
    }

    Ok( result )
  }

}

// impl FieldAttributes
// {
//   pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
//   {
//     let mut config = None;
//     let mut scalar = None;
//     let mut subform_scalar = None;
//     let mut subform_collection = None;
//     let mut subform_entry = None;
//
//     for attr in attrs
//     {
//       let key_ident = attr.path().get_ident()
//       .ok_or_else( || syn_err!( attr, "Expects an attribute of format #[ attribute( key1 = val1, key2 = val2 ) ], but got:\n  {}", qt!{ #attr } ) )?;
//       let key_str = format!( "{}", key_ident );
//
//       if attr::is_standard( &key_str )
//       {
//         continue;
//       }
//
//       match key_str.as_ref()
//       {
//         AttributeConfig::KEYWORD =>
//         {
//           config.replace( AttributeConfig::from_meta( &attr )? );
//         }
//         AttributeScalarSetter::KEYWORD =>
//         {
//           scalar.replace( AttributeScalarSetter::from_meta( &attr )? );
//         }
//         AttributeSubformScalarSetter::KEYWORD =>
//         {
//           subform_scalar.replace( AttributeSubformScalarSetter::from_meta( &attr )? );
//         }
//         AttributeSubformCollectionSetter::KEYWORD =>
//         {
//           subform_collection.replace( AttributeSubformCollectionSetter::from_meta( &attr )? );
//         }
//         AttributeSubformEntrySetter::KEYWORD =>
//         {
//           subform_entry.replace( AttributeSubformEntrySetter::from_meta( &attr )? );
//         }
//         _ =>
//         {
//           return Err( syn_err!( attr, "Unknown field attribute {}", qt!{ #attr } ) );
//         }
//       }
//     }
//
//     Ok( FieldAttributes { config, scalar, subform_scalar, subform_collection, subform_entry } )
//   }
// }

///
/// Attribute to hold configuration information about the field such as default value.
///
/// `#[ default( 13 ) ]`
///

#[ derive( Debug, Default ) ]
pub struct AttributeConfig
{

  /// Default value to use for a field.
  pub default : AttributePropertyOptionalSyn< syn::Expr >,
  // pub default : Option< syn::Expr >,

}

impl AttributeConfig
{

  const KEYWORD : &'static str = "former";

  pub fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeConfig >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeConfig >( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format #[ former( default = 13 ) ].\nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > ComponentAssign< AttributeConfig, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.config = Some( component.into() );
  }
}

impl< IntoT > ComponentAssign< AttributePropertyOptionalSyn< syn::Expr >, IntoT > for AttributeConfig
where
  IntoT : Into< AttributePropertyOptionalSyn< syn::Expr > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.default = component.into();
  }
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
        match ident.to_string().as_str()
        {
          "default" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            default = Some( input.parse()? );
          }
          _ =>
          {
            return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'default'. For example: `former( default = 13 )`", ident ) ) );
          }
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

    Ok( Self { default : default.into() } )
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

#[ derive( Debug, Default ) ]
pub struct AttributeScalarSetter
{
  /// Optional identifier for naming the setter.
  pub name : AttributePropertyOptionalSyn< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub hint : AttributePropertyHint,
}

#[ allow( dead_code ) ]
impl AttributeScalarSetter
{

  const KEYWORD : &'static str = "scalar";

  pub fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeScalarSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeScalarSetter >( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ scalar( setter = false ) ]` or `#[ scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl< IntoT > ComponentAssign< AttributeScalarSetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeScalarSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.scalar = Some( component.into() );
  }
}

impl< IntoT > ComponentAssign< AttributePropertyOptionalSyn< syn::Ident >, IntoT > for AttributeScalarSetter
where
  IntoT : Into< AttributePropertyOptionalSyn< syn::Ident > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertySetter, IntoT > for AttributeScalarSetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyHint, IntoT > for AttributeScalarSetter
where
  IntoT : Into< AttributePropertyHint >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.hint = component.into();
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
        match ident.to_string().as_str()
        {
          "name" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            name = Some( input.parse()? );
          }
          "setter" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            let value : syn::LitBool = input.parse()?;
            setter = Some( value.value() );
          }
          "hint" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            let value : syn::LitBool = input.parse()?;
            hint = value.value;
          }
          _ =>
          {
            return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `scalar( name = myName, setter = true )`", ident ) ) );
          }
        }
      }
      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `scalar( name = myName, setter = true )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![ , ] )
      {
        input.parse::< syn::Token![ , ] >()?;
      }
    }

    Ok( Self { name : name.into(), setter : setter.into(), hint : hint.into() } )
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

#[ derive( Debug, Default ) ]

pub struct AttributeSubformScalarSetter
{
  /// Optional identifier for naming the setter.
  pub name : AttributePropertyOptionalSyn< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub hint : AttributePropertyHint,
}

#[ allow( dead_code ) ]
impl AttributeSubformScalarSetter
{

  const KEYWORD : &'static str = "subform_scalar";

  pub fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformScalarSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformScalarSetter >( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_scalar( setter = false ) ]` or `#[ subform_scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl< IntoT > ComponentAssign< AttributeSubformScalarSetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeSubformScalarSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.subform_scalar = Some( component.into() );
  }
}

impl< IntoT > ComponentAssign< AttributePropertyOptionalSyn< syn::Ident >, IntoT > for AttributeSubformScalarSetter
where
  IntoT : Into< AttributePropertyOptionalSyn< syn::Ident > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertySetter, IntoT > for AttributeSubformScalarSetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyHint, IntoT > for AttributeSubformScalarSetter
where
  IntoT : Into< AttributePropertyHint >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.hint = component.into();
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
        match ident.to_string().as_str()
        {
          "name" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            name = Some( input.parse()? );
          }
          "setter" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            let value : syn::LitBool = input.parse()?;
            setter = Some( value.value() );
          }
          "hint" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            let value : syn::LitBool = input.parse()?;
            hint = value.value;
          }
          _ =>
          {
            return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `subform_scalar( name = myName, setter = true )`", ident ) ) );
          }
        }
      }
      else
      {
        return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `subform_scalar( name = myName, setter = true )`" ) );
      }

      // Optional comma handling
      if input.peek( syn::Token![ , ] )
      {
        input.parse::< syn::Token![ , ] >()?;
      }
    }

    Ok( Self { name : name.into(), setter : setter.into(), hint : hint.into() } )
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

#[ derive( Debug, Default ) ]
pub struct AttributeSubformCollectionSetter
{
  /// Optional identifier for naming the setter.
  pub name : AttributePropertyOptionalSyn< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter : AttributePropertySetter,
  /// Definition of the collection former to use, e.g., `former::VectorFormer`.
  pub definition : AttributePropertyOptionalSyn< syn::Type >,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub hint : AttributePropertyHint,
}

// xxx
impl AttributeSubformCollectionSetter
{

  const KEYWORD : &'static str = "subform_collection";

  pub fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformCollectionSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformCollectionSetter >( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_collection ]` or `#[ subform_collection( definition = former::VectorDefinition ) ]` if you want to use default collection defition. \nGot: {}", qt!{ #attr } ),
    }
  }

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.is_none() || self.setter.unwrap()
  }

}

impl< IntoT > ComponentAssign< AttributeSubformCollectionSetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeSubformCollectionSetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.subform_collection = Some( component.into() );
  }
}

impl< IntoT > ComponentAssign< AttributePropertyOptionalSyn< syn::Ident >, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertyOptionalSyn< syn::Ident > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertySetter, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyOptionalSyn< syn::Type >, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertyOptionalSyn< syn::Type > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.definition = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyHint, IntoT > for AttributeSubformCollectionSetter
where
  IntoT : Into< AttributePropertyHint >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.hint = component.into();
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
        match ident.to_string().as_str()
        {
          "name" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            name = Some( input.parse()? );
          }
          "setter" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            let value : syn::LitBool = input.parse()?;
            setter = Some( value.value );
          }
          "hint" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            let value : syn::LitBool = input.parse()?;
            hint = value.value;
          }
          "definition" =>
          {
            input.parse::< syn::Token![ = ] >()?;
            definition = Some( input.parse()? );
          }
          _ =>
          {
            return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `collection( name = myName, setter = true, definition = MyDefinition )`", ident ) ) );
          }
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

    Ok( Self { name : name.into(), setter : setter.into(), hint : hint.into(), definition : definition.into() } )
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

#[ derive( Debug, Default ) ]
pub struct AttributeSubformEntrySetter
{
  /// An optional identifier that names the setter. It is parsed from inputs
  /// like `name = my_field`.
  // pub name : Option< syn::Ident >,
  pub name : AttributePropertyOptionalSyn< syn::Ident >,
  /// Disable generation of setter.
  /// It still generate `_field_subform_entry` method, so it could be used to make a setter with custom arguments.
  // pub setter : Option< bool >,
  pub setter : AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  // pub hint : bool,
  pub hint : AttributePropertyHint,
}

impl AttributeSubformEntrySetter
{

  const KEYWORD : &'static str = "subform_entry";

  pub fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformEntrySetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformEntrySetter >( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_entry ]` or `#[ subform_entry( name : child )` ], \nGot: {}", qt!{ #attr } ),
    }
  }

  /// Should setter be generated or not?
  pub fn setter( &self ) -> bool
  {
    self.setter.as_ref().is_none() || self.setter.as_ref().unwrap()
  }

}

impl< IntoT > ComponentAssign< AttributeSubformEntrySetter, IntoT > for FieldAttributes
where
  IntoT : Into< AttributeSubformEntrySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.subform_entry = Some( component.into() );
  }
}

impl< IntoT > ComponentAssign< AttributePropertyOptionalSyn< syn::Ident >, IntoT > for AttributeSubformEntrySetter
where
  IntoT : Into< AttributePropertyOptionalSyn< syn::Ident > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertySetter, IntoT > for AttributeSubformEntrySetter
where
  IntoT : Into< AttributePropertySetter >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.setter = component.into();
  }
}

impl< IntoT > ComponentAssign< AttributePropertyHint, IntoT > for AttributeSubformEntrySetter
where
  IntoT : Into< AttributePropertyHint >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.hint = component.into();
  }
}

impl syn::parse::Parse for AttributeSubformEntrySetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = const_format::concatcp!
      (
        "Known entries of attribute ", AttributeSubformEntrySetter::KEYWORD, " are : ",
        AttributePropertyOptionalSyn::< syn::Ident >::KEYWORD,
        ", ", AttributePropertySetter::KEYWORD,
        ", ", AttributePropertyHint::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ subform( name = myName, setter = true ) ]'
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

        input.parse::< syn::Token![=] >()?;
        match ident.to_string().as_str()
        {
          // xxx
          AttributePropertyOptionalSyn::< syn::Ident >::KEYWORD => result.assign( AttributePropertyOptionalSyn::< syn::Ident >::parse( input )? ),
          AttributePropertySetter::KEYWORD => result.assign( AttributePropertySetter::parse( input )? ),
          AttributePropertyHint::KEYWORD => result.assign( AttributePropertyHint::parse( input )? ),
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

// impl syn::parse::Parse for AttributeSubformEntrySetter
// {
//   fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
//   {
//     let mut name : Option< syn::Ident > = None;
//     let mut setter : Option< bool > = None;
//     let mut hint = false;
//
//     while !input.is_empty()
//     {
//       let lookahead = input.lookahead1();
//       if lookahead.peek( syn::Ident )
//       {
//         let ident : syn::Ident = input.parse()?;
//         match ident.to_string().as_str()
//         {
//           "name" =>
//           {
//             input.parse::< syn::Token![ = ] >()?;
//             name = Some( input.parse()? );
//           }
//           "setter" =>
//           {
//             input.parse::< syn::Token![ = ] >()?;
//             let value : syn::LitBool = input.parse()?;
//             setter = Some( value.value() );
//           }
//           "hint" =>
//           {
//             input.parse::< syn::Token![ = ] >()?;
//             let value : syn::LitBool = input.parse()?;
//             hint = value.value;
//           }
//           _ =>
//           {
//             return Err( syn::Error::new_spanned( &ident, format!( "Unexpected identifier '{}'. Expected 'name', 'setter', or 'definition'. For example: `subform( name = myName, setter = true )`", ident ) ) );
//           }
//         }
//       }
//       else
//       {
//         return Err( syn::Error::new( input.span(), "Expected 'name', 'setter', or 'definition' identifier. For example: `subform( name = myName, setter = true )`" ) );
//       }
//
//       // Optional comma handling
//       if input.peek( syn::Token![ , ] )
//       {
//         input.parse::< syn::Token![ , ] >()?;
//       }
//     }
//
//     Ok( Self { name, setter, hint } )
//   }
// }

// == attribute entries

// = AttributePropertyHint

/// Specifies whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyHint( bool );

impl AttributePropertyHint
{
  const KEYWORD : &'static str = "hint";

  /// Just unwrap, returning internal data.
  pub fn internal( self ) -> bool
  {
    self.0
  }

}

impl syn::parse::Parse for AttributePropertyHint
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let value : syn::LitBool = input.parse()?;
    Ok( value.value.into() )
  }
}

impl From< bool > for AttributePropertyHint
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src )
  }
}

impl From< AttributePropertyHint > for bool
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyHint ) -> Self
  {
    src.0
  }
}

// = AttributePropertySetter

/// Disable generation of setter.
/// Attributes still might generate some helper methods to reuse by custom setter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertySetter( Option< bool > );

impl AttributePropertySetter
{
  const KEYWORD : &'static str = "setter";

  /// Just unwrap, returning internal data.
  pub fn internal( self ) -> Option< bool >
  {
    self.0
  }

  /// Returns Option< &bool > instead of &Option< bool >
  pub fn ref_internal( &self ) -> Option< &bool >
  {
    self.0.as_ref()
  }

}

impl syn::parse::Parse for AttributePropertySetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let value : syn::LitBool = input.parse()?;
    Ok( value.value.into() )
  }
}

impl core::ops::Deref for AttributePropertySetter
{
  type Target = Option< bool >;
  #[ inline( always ) ]
  fn deref( &self ) -> &Option< bool >
  {
    &self.0
  }
}

impl AsRef< Option< bool > > for AttributePropertySetter
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &Option< bool >
  {
    &self.0
  }
}

impl From< bool > for AttributePropertySetter
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( Some( src ) )
  }
}

impl From< Option< bool > > for AttributePropertySetter
{
  #[ inline( always ) ]
  fn from( src : Option< bool > ) -> Self
  {
    Self( src )
  }
}

impl From< AttributePropertySetter > for Option< bool >
{
  #[ inline( always ) ]
  fn from( src : AttributePropertySetter ) -> Self
  {
    src.0
  }
}

// = AttributePropertyOptionalSyn

/// Property of an attribute which simply wrap one of standard of `syn` type and keep it optional.
#[ derive( Debug, Clone ) ]
pub struct AttributePropertyOptionalSyn< T >( Option< T > )
where T : syn::parse::Parse + quote::ToTokens;

impl< T > Default for AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  fn default() -> Self
  {
    Self( None )
  }
}

impl< T > AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  /// Just unwrap, returning internal data.
  pub fn internal( self ) -> Option< T >
  {
    self.0
  }
  /// Returns Option< &T > instead of &Option< T >
  pub fn ref_internal( &self ) -> Option< &T >
  {
    self.0.as_ref()
  }
}

impl< T > AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  const KEYWORD : &'static str = "name";
}

impl< T > syn::parse::Parse for AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let value : T = input.parse()?;
    Ok( value.into() )
  }
}

// xxx
impl< T > quote::ToTokens for AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.0.to_tokens( tokens );
  }
}

impl< T > core::ops::Deref for AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  type Target = Option< T >;
  #[ inline( always ) ]
  fn deref( &self ) -> &Option< T >
  {
    &self.0
  }
}

impl< T > AsRef< Option< T > > for AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &Option< T >
  {
    &self.0
  }
}

impl< T > From< T > for AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : T ) -> Self
  {
    Self( Some( src ) )
  }
}

impl< T > From< Option< T > > for AttributePropertyOptionalSyn< T >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : Option< T > ) -> Self
  {
    Self( src )
  }
}

impl< T > From< AttributePropertyOptionalSyn< T > > for Option< T >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyOptionalSyn< T > ) -> Self
  {
    src.0
  }
}

impl< 'a, T > From< &'a AttributePropertyOptionalSyn< T > > for Option< &'a T >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : &'a AttributePropertyOptionalSyn< T > ) -> Self
  {
    src.0.as_ref()
  }
}

// xxx2 : continue
