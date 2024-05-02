
use super::*;
use iter_tools::{ Itertools, process_results };
use macro_tools::{ attr, diag, generic_params, generic_args, container_kind, typ, Result };
use proc_macro2::TokenStream;

///
/// Definition of a field.
///

#[ allow( dead_code ) ]
struct FormerField< 'a >
{
  pub attrs : Attributes,
  pub vis : &'a syn::Visibility,
  pub ident : &'a syn::Ident,
  pub colon_token : &'a Option< syn::token::Colon >,
  pub ty : &'a syn::Type,
  pub non_optional_ty : &'a syn::Type,
  pub is_optional : bool,
  pub of_type : container_kind::ContainerKind,
}

// xxx
impl< 'a > FormerField< 'a >
{

  /// Get name of setter for container if such setter should be generated.
  pub fn container_setter_name( &self ) -> Option< &syn::Ident >
  {

    if let Some( ref attr ) = self.attrs.container
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name
        {
          return Some( &name )
        }
        else
        {
          return Some( &self.ident )
        }
      }
    }

    return None;
  }

  /// Get name of setter for subform if such setter should be generated.
  pub fn subform_setter_name( &self ) -> Option< &syn::Ident >
  {

    if let Some( ref attr ) = self.attrs.subform
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name
        {
          return Some( &name )
        }
        else
        {
          return Some( &self.ident )
        }
      }
    }

    return None;
  }

  /// Is scalar setter required.
  pub fn scalar_setter_enabled( &self ) -> bool
  {

    let mut explicit = false;
    if let Some( ref attr ) = self.attrs.scalar
    {
      if let Some( setter ) = attr.setter
      {
        if setter == false
        {
          return false
        }
        explicit = true;
      }
    }

    if self.attrs.container.is_some() && !explicit
    {
      return false;
    }

    if self.attrs.subform.is_some() && !explicit
    {
      return false;
    }

    return true;
  }

}

///
/// Attributes of the field.
///

struct Attributes
{
  default : Option< AttributeDefault >,
  scalar : Option< AttributeScalarSetter >,
  container : Option< AttributeContainerSetter >,
  subform : Option< AttributeSubformSetter >,
  alias : Option< AttributeAlias >, // xxx : remove
}

impl Attributes
{
  fn parse( attributes : & Vec< syn::Attribute > ) -> Result< Self >
  {
    let mut default = None;
    let mut scalar = None;
    let mut container = None;
    let mut subform = None;
    let mut alias = None;
    for attr in attributes
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
        "default" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              default.replace( syn::parse2::< AttributeDefault >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ default( val ) ], but got:\n  {}", qt!{ #attr } ),
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
        "alias" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              alias.replace( syn::parse2::< AttributeAlias >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format `#[ alias( val ) ]`. \nGot: {}", qt!{ #attr } ),
          }
          // let attr_alias = syn::parse2::< AttributeAlias >( attr.tokens.clone() )?;
          // alias.replace( attr_alias );
        }
        _ =>
        {
          return Err( syn_err!( attr, "Unknown attribute {}", qt!{ #attr } ) );
        }
      }
    }

    Ok( Attributes { default, scalar, container, subform, alias } )
  }
}

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform( fn after1< 'a >() -> Option< &'a str > ) ]`
///


struct AttributeFormAfter
{
  // paren_token : syn::token::Paren,
  signature : syn::Signature,
}

impl syn::parse::Parse for AttributeFormAfter
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
/// Attribute to hold information about default value.
///
/// `#[ default( 13 ) ]`
///


struct AttributeDefault
{
  // eq_token : syn::Token!{ = },
  // paren_token : syn::token::Paren,
  expr : syn::Expr,
}

impl syn::parse::Parse for AttributeDefault
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    // let input2;
    Ok( Self
    {
      // paren_token : syn::parenthesized!( input2 in input ),
      // eq_token : input.parse()?,
      // expr : input2.parse()?,
      expr : input.parse()?,
    })
  }
}

///
/// Attribute to enable/disable scalar setter generation.
///
/// `#[ scalar( false ) ]`
///


/// Represents an attribute for configuring setter generation in container-like structures.
///
/// Used to specify extra options for primitive scalar setter generation.
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

struct AttributeScalarSetter
{
  /// Optional identifier for naming the setter.
  name : Option< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  setter : Option< bool >,
}

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

struct AttributeContainerSetter
{
  /// Optional identifier for naming the setter.
  name : Option< syn::Ident >,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  setter : Option< bool >,
  /// Definition of the container former to use, e.g., `former::VectorSubformer`.
  definition : Option< syn::Type >,
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

struct AttributeSubformSetter
{
  /// An optional identifier that names the setter. It is parsed from inputs
  /// like `name = my_field`.
  name : Option< syn::Ident >,
  /// Disable generation of setter.
  /// It still generate `_field_add` method, so it could be used to make a setter with custom arguments.
  setter : Option< bool >,
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

///
/// Attribute to create alias.
///
/// `#[ alias( name ) ]`
///


struct AttributeAlias
{
  // paren_token : syn::token::Paren,
  alias : syn::Ident,
}

impl syn::parse::Parse for AttributeAlias
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    // let input2;
    Ok( Self
    {
      // paren_token : syn::parenthesized!( input2 in input ),
      // alias : input2.parse()?,
      alias : input.parse()?,
    })
  }
}

///
/// Is type under Option.
///

fn is_optional( ty : &syn::Type ) -> bool
{
  typ::type_rightmost( ty ) == Some( "Option".to_string() )
}

///
/// Extract the first parameter of the type if such exist.
///

fn parameter_internal_first( ty : &syn::Type ) -> Result< &syn::Type >
{
  typ::type_parameters( ty, 0 ..= 0 )
  .first()
  .copied()
  .ok_or_else( || syn_err!( ty, "Expects at least one parameter here:\n  {}", qt!{ #ty } ) )
}

///
/// Generate fields for initializer of a struct setting each field to `None`.
///
/// Used for initializing a Container, where on initialization all fields are None. User can alter them through builder pattern
///
/// ### Basic use-case. of output
///
/// ```ignore
/// int_1 : core::option::Option::None,
/// string_1 : core::option::Option::None,
/// int_optional_1 : core::option::Option::None,
/// ```
///

#[ inline( always ) ]
fn field_none_map( field : &FormerField< '_ > ) -> TokenStream
{
  let ident = Some( field.ident.clone() );
  let tokens = qt! { ::core::option::Option::None };
  let ty2 : syn::Type = syn::parse2( tokens ).unwrap();

  qt!
  {
    #ident : #ty2
  }
}

///
/// Generate field of the former for a field of the structure
///
/// Used to generate a Container
///
/// ### Basic use-case. of output
///
/// ```ignore
/// pub int_1 : core::option::Option< i32 >,
/// pub string_1 : core::option::Option< String >,
/// pub int_optional_1 :  core::option::Option< i32 >,
/// pub string_optional_1 : core::option::Option< String >,
/// ```
///

#[ inline( always ) ]
fn field_optional_map( field : &FormerField< '_ > ) -> TokenStream
{
  let ident = Some( field.ident.clone() );
  let ty = field.ty.clone();

  // let ty2 = if is_optional( &ty )
  let ty2 = if field.is_optional
  {
    qt! { #ty }
  }
  else
  {
    qt! { ::core::option::Option< #ty > }
  };

  qt!
  {
    pub #ident : #ty2
  }

}

///
/// Generate code converting a field of the former to the field of the structure.
///
/// In simple terms, used on `form()` call to unwrap contained values from the former's storage.
/// Will try to use default values if no values supplied by the former and the type implements `Default` trait.
///
/// ### Generated code will look similar to this :
///
/// ```ignore
/// let int_1 : i32 = if self.storage.int_1.is_some()
/// {
///   // if int_1 is optional
///   Some( self.storage.int_1.take().unwrap() )
///
///   // if int_1 isn't optional
///   self.storage.int_1.take().unwrap()
/// }
/// else
/// {
///   // if int_1 is optional and has default
///   Some( i32::default().into() )
///
///   // if int_1 is optional and doesn't have default
///   None
///
///   // if int_1 isn't optional and has default
///   i32::default().into()
///
///   // if int_1 isn't optional and hasn't default
///   panic!( "Field 'int_1' isn't initialized" )
/// };
/// ```
///

#[ inline( always ) ]
fn field_form_map( field : &FormerField< '_ > ) -> Result< TokenStream >
{
  let ident = field.ident;
  let ty = field.ty;
  let default = field.attrs.default.as_ref()
  .map( | attr_default | &attr_default.expr );

  let tokens = if field.is_optional
  {

    let _else = match default
    {
      None =>
      {
        qt!
        {
          ::core::option::Option::None
        }
      }

      Some( default_val ) =>
      {
        qt!
        {
          ::core::option::Option::Some( ::core::convert::Into::into( #default_val ) )
        }
      }
    };

    qt!
    {
      let #ident = if self.#ident.is_some()
      {
        ::core::option::Option::Some( self.#ident.take().unwrap() )
      }
      else
      {
        #_else
      };
    }

  }
  else
  {

    let _else = match default
    {
      None =>
      {
        let panic_msg = format!( "Field '{}' isn't initialized", ident );
        qt!
        {
          {
            // By hardly utilizing deref coercion, we achieve conditional trait implementation
            trait MaybeDefault< T >
            {
              fn maybe_default( self : &Self ) -> T { panic!( #panic_msg ) }
            }

            // Panic on non-`Default` types
            impl< T > MaybeDefault< T >
            for &::core::marker::PhantomData< T >
            {}

            // Return default value on `Default`` types
            impl< T > MaybeDefault< T >
            for ::core::marker::PhantomData< T >
            where T : ::core::default::Default,
            {
              fn maybe_default( self : &Self ) -> T
              {
                T::default()
              }
            }

            // default if `impl Default`, otherwise - panic
            ( &::core::marker::PhantomData::< #ty > ).maybe_default()
          }
        }
      }
      Some( default_val ) =>
      {
        qt!
        {
          ::core::convert::Into::into( #default_val )
        }
      }
    };

    qt!
    {
      let #ident = if self.#ident.is_some()
      {
        self.#ident.take().unwrap()
      }
      else
      {
        #_else
      };
    }

  };

  Ok( tokens )
}

///
/// Extract name of a field out.
///

#[ inline( always ) ]
fn field_name_map( field : &FormerField< '_ > ) -> syn::Ident
{
  field.ident.clone()
}

///
/// Generate a former setter for the field.
///
/// If aliases provided, also generate aliases
///
/// # Example of generated code
///
/// ```ignore
/// #[ doc = "Setter for the 'int_1' field." ]
/// #[ inline ]
/// pub fn int_1< Src >( mut self, src : Src ) -> Self
/// where
///   Src : ::core::convert::Into< i32 >,
/// {
///   debug_assert!( self.int_1.is_none() );
///   self.storage.int_1 = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
///   self
/// }
///
/// /// #[ doc = "Setter for the 'int_1' field." ]
/// #[ inline ]
/// pub fn int_1_alias< Src >( mut self, src : Src ) -> Self
/// where
///   Src : ::core::convert::Into< i32 >,
/// {
///   debug_assert!( self.int_1.is_none() );
///   self.storage.int_1 = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
///   self
/// }
/// ```

#[ inline ]
fn field_setter_map
(
  field : &FormerField< '_ >,
  stru : &syn::Ident,
)
-> Result< TokenStream >
{
  let ident = &field.ident;
  let non_optional_ty = &field.non_optional_ty;
  let r = qt!{};

  // xxx : write test for interoperability of 3 attributes: scalar, subform, container

  // scalar setter
  let r = if field.scalar_setter_enabled()
  {
    let r2 = field_scalar_setter( ident, ident, non_optional_ty );
    qt!
    {
      #r
      #r2
    }
  }
  else
  {
    r
  };

  // alias trival setter
  let r = if let Some( alias_attr ) = &field.attrs.alias
  {
    let r2 = field_scalar_setter( ident, &alias_attr.alias, non_optional_ty );
    qt!
    {
      #r
      #r2
    }
  }
  else
  {
    r
  };

  // container setter
  let r = if let Some( _ ) = &field.attrs.container
  {
    let r2 = field_container_setter( field, stru );
    qt!
    {
      #r
      #r2
    }
  }
  else
  {
    r
  };

  // subform setter
  let r = if field.attrs.subform.is_some()
  {
    let r2 = field_subform_add_setter_map( field, stru )?;
    qt!
    {
      #r
      #r2
    }
  }
  else
  {
    r
  };

  // tree_print!( r.as_ref().unwrap() );
  Ok( r )
}

/// zzz : write documentation
#[ inline ]
fn field_subform_add_setter_map
(
  field : &FormerField< '_ >,
  stru : &syn::Ident,
)
-> Result< TokenStream >
{

  if field.attrs.subform.is_none()
  {
    return Ok( qt!{ } );
  }

  use convert_case::{ Case, Casing };
  let field_ident = field.ident;
  let field_ty = field.non_optional_ty;
  let attr = field.attrs.subform.as_ref().unwrap();
  // let params = typ::type_parameters( &field.non_optional_ty, .. );

  // example : `child`
  let setter_name = field.subform_setter_name();

  // example : `ParentFormerAddChildrenEnd``
  let former_add_end_name = format!( "{}FormerAdd{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
  let former_add_end = syn::Ident::new( &former_add_end_name, field_ident.span() );

  // example : `_children_former`
  let field_add_name = format!( "_{}_add", field_ident );
  let field_add = syn::Ident::new( &field_add_name, field_ident.span() );

  let r = qt!
  {

    // zzz : improve documentation
    /// Setter returning former of element of container of the field as subformer.
    #[ inline( always ) ]
    pub fn #field_add< Former2, Definition2 >( self ) -> Former2
    where
      Definition2 : former::FormerDefinition
      <
        End = #former_add_end< Definition >,
        Storage = < < #field_ty as former::Container >::Val as former::EntityToStorage >::Storage,
        Formed = Self,
        Context = Self,
      >,
      Definition2::Types : former::FormerDefinitionTypes
      <
        Storage = < < #field_ty as former::Container >::Val as former::EntityToStorage >::Storage,
        Formed = Self,
        Context = Self,
      >,
      Former2 : former::FormerBegin< Definition2 >,
    {
      Former2::former_begin( None, Some( self ), #former_add_end::default() )
    }

  };

  // xxx : it should be printed by hint also
  let r = if attr.setter()
  {
    qt!
    {
      #r

      #[ inline( always ) ]
      pub fn #setter_name( self ) ->
      < < #field_ty as former::Container >::Val as former::EntityToFormer
        <
          <
            < #field_ty as former::Container >::Val as former::EntityToDefinition< Self, Self, #former_add_end < Definition > >
          >::Definition,
        >
      >::Former
      // #as_subformer< Self, impl #as_subformer_end< Self > >
      {
        self.#field_add
        ::< < < #field_ty as former::Container >::Val as former::EntityToFormer< _ > >::Former, _, >()
        // ::< #former< _ >, _, >()
      }
    }

    // #[ inline( always ) ]
    // pub fn child( self ) ->
    // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
    // {
    //   self._children_add
    //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
    // }

  }
  else
  {
    r
  };

  // tree_print!( r.as_ref().unwrap() );
  Ok( r )
}

///
/// Generate a container setter for the 'field_ident' with the 'setter_name' name.
///
/// # Example of generated code
///
/// ```ignore
/// pub fn hashmap_strings_1( mut self ) -> former::HashMapSubformer
/// <
///   String,
///   String,
///   std::collections::HashMap< String, String >,
///   Struct1Former,
///   impl Fn( std::collections::HashMap< String, String >, core::option::Option< Self > ) -> Self
/// >
/// {
///   let formed = self.hashmap_strings_1.take();
///   let on_end = | formed : std::collections::HashMap< String, String >, mut former : core::option::Option< Self > | -> Self
///   {
///     former.hashmap_strings_1 = Some( formed );
///     former
///   };
///   former::HashMapSubformer::begin_coercing( formed, self, on_end )
/// }
/// ```
/// zzz : update example

#[ inline ]
fn field_container_setter
(
  field : &FormerField< '_ >,
  stru : &syn::Ident,
)
-> TokenStream
{
  let field_ident = &field.ident;
  let non_optional_ty = &field.non_optional_ty;
  let params = typ::type_parameters( &non_optional_ty, .. );

  use convert_case::{ Case, Casing };
  let former_assign_end_name = format!( "{}FormerAssign{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
  let former_assign_end = syn::Ident::new( &former_assign_end_name, field_ident.span() );
  let field_assign_name = format!( "_{}_assign", field_ident );
  let field_assign = syn::Ident::new( &field_assign_name, field_ident.span() );

  // example : `former::VectorDefinition`
  let subformer_definition = &field.attrs.container.as_ref().unwrap().definition;
  let subformer_definition = if subformer_definition.is_some()
  {
    qt!
    {
      #subformer_definition
      <
        #( #params, )*
        Self,
        Self,
        #former_assign_end< Definition >,
      >
    }
    // former::VectorDefinition< String, Self, Self, Struct1FormerAssignVec1End, >
  }
  else
  {
    qt!
    {
      < #non_optional_ty as former::EntityToDefinition< Self, Self, #former_assign_end< Definition > > >::Definition
    }
    // < Vec< String > as former::EntityToDefinition< Self, Self, Struct1FormerAssignVec1End > >::Definition
  };

  let doc = format!
  (
    "Container setter for the '{}' field. Method {} unlike method {} accept custom container subformer.",
    field_ident,
    field_assign_name,
    field_ident,
  );

  let setter1 =
  qt!
  {
    #[ doc = #doc ]
    #[ inline( always ) ]
    pub fn #field_assign< Former2 >( self ) -> Former2
    where
      Former2 : former::FormerBegin
      <
        #subformer_definition
      >,
    {
      Former2::former_begin( None, Some( self ), #former_assign_end::< Definition >::default() )
    }
  };

  let setter_name = field.container_setter_name();
  let setter2 = if let Some( setter_name ) = setter_name
  {
    if params.len() > 1
    {
      qt!
      {

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        former::ContainerSubformer::
        <
          ( #( #params, )* ), #subformer_definition
        >
        {
          self.#field_assign::< former::ContainerSubformer::
          <
            ( #( #params, )* ), #subformer_definition
          > >()
        }

      }
    }
    else
    {
      qt!
      {

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        former::ContainerSubformer::
        <
          #( #params, )* #subformer_definition
        >
        {
          self.#field_assign::< former::ContainerSubformer::
          <
            #( #params, )* #subformer_definition
          > >()
        }

      }
    }
  }
  else
  {
    qt!{}
  };

  qt!
  {
    #setter1
    #setter2
  }

//   #[ inline( always ) ]
//   pub fn vec_1_assign< Former2 >( self ) -> Former2
//   where
//     Former2 : former::FormerBegin
//     <
//       former::VectorDefinition
//       <
//         String,
//         Self,
//         Self,
//         Struct1FormerVec_1End,
//       >
//     >,
//   {
//     Former2::_begin( None, Some( self ), Struct1FormerVec_1End )
//   }
//
//   pub fn vec_1( self ) ->
//   former::ContainerSubformer::
//   <
//     String, former::VectorDefinition< String, Self, Self, Struct1FormerVec_1End >
//   >
//   {
//     self.vec_1_assign::< former::ContainerSubformer::
//     <
//       String, former::VectorDefinition< String, Self, Self, Struct1FormerVec_1End >
//     >>()
//   }

}

///
/// Generate a single scalar setter for the 'field_ident' with the 'setter_name' name.
///
/// Used as a helper function for field_setter_map(), which generates alias setters
///
/// # Example of generated code
/// ```ignore
/// #[ doc = "Setter for the 'int_1' field." ]
/// #[ inline ]
/// pub fn int_1< Src >( mut self, src : Src ) -> Self
/// where
///   Src : ::core::convert::Into< i32 >,
/// {
///   debug_assert!( self.int_1.is_none() );
///   self.storage.int_1 = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
///   self
/// }
/// ```

#[ inline ]
fn field_scalar_setter
(
  field_ident : &syn::Ident,
  setter_name : &syn::Ident,
  non_optional_type : &syn::Type,
)
-> TokenStream
{
  let doc = format!
  (
    "Setter for the '{}' field.",
    field_ident,
  );

  qt!
  {
    #[ doc = #doc ]
    #[ inline ]
    pub fn #setter_name< Src >( mut self, src : Src ) -> Self
    where Src : ::core::convert::Into< #non_optional_type >,
    {
      debug_assert!( self.storage.#field_ident.is_none() );
      self.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
      self
    }
  }
}

// zzz : description and exmaple
/// Generate unit struct which is descriptor of callback which should be called after subforming process of a specific field. Childs are used insted of closures to inline code and let optimizer play with optimization.
///
/// # Example of generated code
///
/// ```rust, ignore
/// pub struct Struct1FormerVec_1End;
/// #[ automatically_derived ]
/// impl< Definition > former::FormingEnd
/// <
///   former::VectorDefinition< String, Struct1Former< Definition >, Struct1Former< Definition >, former::NoEnd >,
/// >
/// for Struct1FormerVec_1End
/// where
///   Definition : former::FormerDefinition,
///   Definition::Types : former::FormerDefinitionTypes
///   <
///     Storage = Struct1FormerStorage
///   >,
/// {
///   #[ inline( always ) ]
///   fn call
///   (
///     &self, storage : Vec< String >,
///     super_former : Option< Struct1Former< Definition > >,
///   )
///   -> Struct1Former< Definition >
///   {
///     let mut super_former = super_former.unwrap();
///     if let Some( ref mut field ) = super_former.storage.vec_1
///     {
///       former::ContainerAssign::assign( field, storage );
///     }
///     else
///     {
///       super_former.storage.vec_1 = Some( storage );
///     }
///     super_former
///   }
/// }
/// ```

#[ inline ]
fn field_former_assign_end_map
(
  field : &FormerField< '_ >,
  stru : &syn::Ident,
  former : &syn::Ident,
  former_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
->
Result< TokenStream >
{

  if field.attrs.container.is_none()
  {
    return Ok( qt!{ } );
  }

  use convert_case::{ Case, Casing };
  let field_ident = field.ident;
  let field_ty = field.non_optional_ty;
  let params = typ::type_parameters( field_ty, .. );

  // example : `ParentFormerAssignChildsEnd``
  let former_assign_end_name = format!( "{}FormerAssign{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
  let former_assign_end = syn::Ident::new( &former_assign_end_name, field_ident.span() );

  // example : `former::VectorDefinition``
  let subformer_definition = &field.attrs.container.as_ref().unwrap().definition;

  // zzz : improve description
  let former_assign_end_doc = format!
  (
r#"Callback to return original former after forming of container for `${stru}` is done.#

Callback replace content of container assigning new content from subformer's storage."#
  );

  let subformer_definition = if subformer_definition.is_some()
  {
    qt!
    {
      #subformer_definition < #( #params, )* #former< #former_generics_ty >, #former< #former_generics_ty >, former::NoEnd >
    }
    // former::VectorDefinition< String, Struct1Former< Definition, >, Struct1Former< Definition, >, former::NoEnd >
  }
  else
  {
    qt!
    {
      < #field_ty as former::EntityToDefinition< #former< #former_generics_ty >, #former< #former_generics_ty >, former::NoEnd > >::Definition
    }
    // < Vec< String > as former::EntityToDefinition< Struct1Former< Definition, >, Struct1Former< Definition, >, former::NoEnd > >::Definition
  };

  let r = qt!
  {

    // zzz : improve description
    #[ doc = #former_assign_end_doc ]
    pub struct #former_assign_end< Definition >
    {
      _phantom : core::marker::PhantomData< ( Definition, ) >,
    }

    impl< Definition > Default
    for #former_assign_end< Definition >
    {

      #[ inline( always ) ]
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }

    }

    #[ automatically_derived ]
    impl< #former_generics_impl > former::FormingEnd
    <
      #subformer_definition,
    >
    for #former_assign_end< Definition >
    where
      #former_generics_where
    {
      #[ inline( always ) ]
      fn call
      (
        &self,
        storage : #field_ty,
        super_former : Option< #former< #former_generics_ty > >,
      )
      -> #former< #former_generics_ty >
      {
        let mut super_former = super_former.unwrap();
        if let Some( ref mut field ) = super_former.storage.#field_ident
        {
          former::ContainerAssign::assign( field, storage );
        }
        else
        {
          super_former.storage.#field_ident = Some( storage );
        }
        super_former
      }
    }

  };

  // tree_print!( r.as_ref().unwrap() );
  Ok( r )
}

/// zzz : write documentation

#[ inline ]
fn field_former_add_end_map
(
  field : &FormerField< '_ >,
  stru : &syn::Ident,
  former : &syn::Ident,
  _former_storage : &syn::Ident,
  _former_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  _former_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  struct_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  struct_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  struct_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
->
Result< TokenStream >
{

  if field.attrs.subform.is_none()
  {
    return Ok( qt!{ } );
  }

  use convert_case::{ Case, Casing };
  let field_ident = field.ident;
  let field_ty = field.non_optional_ty;
  // let params = typ::type_parameters( &field.non_optional_ty, .. );

  // example : `ParentFormerAddChildrenEnd``
  let former_add_end_name = format!( "{}FormerAdd{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
  let former_add_end = syn::Ident::new( &former_add_end_name, field_ident.span() );

  let r = qt!
  {

    // zzz : improve description
    /// Handles the completion of an element of subformer's container.
    pub struct #former_add_end< Definition >
    {
      _phantom : core::marker::PhantomData< fn( Definition ) >,
    }

    impl< Definition > Default
    for #former_add_end< Definition >
    {
      #[ inline( always ) ]
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }
    }

    impl< #struct_generics_impl Types2, Definition > former::FormingEnd< Types2, >
    for #former_add_end< Definition >
    where
      Definition : former::FormerDefinition
      <
        Storage = < #stru < #struct_generics_ty > as former::EntityToStorage >::Storage,
      >,
      Types2 : former::FormerDefinitionTypes
      <
        Storage = < < #field_ty as former::Container >::Val as former::EntityToStorage >::Storage,
        Formed = #former< #former_generics_ty >,
        Context = #former< #former_generics_ty >,
      >,
      #struct_generics_where
    {
      #[ inline( always ) ]
      fn call
      (
        &self,
        substorage : Types2::Storage,
        super_former : core::option::Option< Types2::Context >,
      )
      -> Types2::Formed
      {
        let mut super_former = super_former.unwrap();
        if super_former.storage.#field_ident.is_none()
        {
          super_former.storage.#field_ident = Some( Default::default() );
        }
        if let Some( ref mut field ) = super_former.storage.#field_ident
        {
          former::ContainerAdd::add
          (
            field,
            < < #field_ty as former::Container >::Val as former::ValToElement< #field_ty > >
            ::val_to_element( former::StoragePreform::preform( substorage ) ),
          );
          // former::ContainerAdd::add( field, former::StoragePreform::preform( substorage ) );
        }
        super_former
      }
    }

  };

  // tree_print!( r.as_ref().unwrap() );
  Ok( r )
}

///
/// Generate documentation for the former.
///

fn doc_generate( stru : &syn::Ident ) -> ( String, String )
{

  let doc_former_mod = format!
  (
r#" Implementation of former for [{}].
"#,
    stru
  );

  let doc_example1 =
r#"
use former::Former;
#[ derive( Former ) ]
pub struct Struct1
{
  #[default( 31 ) ]
  field1 : i32,
}
"#;

  let doc_former_struct = format!
  (
r#" Object to form [{}]. If field's values is not set then default value of the field is set.

For specifying custom default value use attribute `default`. For example:
```
{}
```
"#,
    stru, doc_example1
  );

  ( doc_former_mod, doc_former_struct )
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

pub fn performer< 'a >
(
  _struct_name : &syn::Ident,
  // _former_definition : &syn::Ident,
  // _generics_ty : &syn::TypeGenerics< '_ >,
  attrs : impl Iterator< Item = &'a syn::Attribute >,
)
-> Result< ( TokenStream, TokenStream, TokenStream ) >
{

  let mut perform = qt!
  {
    return result;
  };
  // let mut perform_output = qt!{ #stru #generics_ty_ };
  let mut perform_output = qt!{ < Definition::Types as former::FormerDefinitionTypes >::Formed };

  let mut perform_generics = qt!{};
  for attr in attrs
  {
    if let Some( ident ) = attr.path().get_ident()
    {
      let ident_string = format!( "{}", ident );
      if ident_string == "perform"
      {
        match attr.meta
        {
          syn::Meta::List( ref meta_list ) =>
          {
            // default.replace( syn::parse2::< AttributeDefault >( meta_list.tokens.clone() )? );
            // let attr_perform = syn::parse2::< AttributeFormAfter >( attr.tokens.clone() )?;
            let attr_perform = syn::parse2::< AttributeFormAfter >( meta_list.tokens.clone() )?;
            let signature = &attr_perform.signature;
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
          },
          _ => return_syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ),
        }
      }
    }
    else
    {
      return_syn_err!( "Unknown structure attribute:\n{}", qt!{ attr } );
    }
  }

  Ok( ( perform, perform_output, perform_generics ) )
}

//

///
/// Generate the whole Former ecosystem
///
/// Output examples can be found in [docs to former crate](https://docs.rs/former/latest/former/)
///

pub fn former( input : proc_macro::TokenStream ) -> Result< TokenStream >
{
  use macro_tools::IntoGenericArgs;

  let original_input = input.clone();
  let ast = match syn::parse::< syn::DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };
  let has_debug = attr::has_debug( ast.attrs.iter() )?;
  let example_of_custom_setter = false;

  /* names */

  let stru = &ast.ident;
  let former_name = format!( "{}Former", stru );
  let former = syn::Ident::new( &former_name, stru.span() );
  let former_storage_name = format!( "{}FormerStorage", stru );
  let former_storage = syn::Ident::new( &former_storage_name, stru.span() );
  let former_definition_name = format!( "{}FormerDefinition", stru );
  let former_definition = syn::Ident::new( &former_definition_name, stru.span() );
  let former_definition_types_name = format!( "{}FormerDefinitionTypes", stru );
  let former_definition_types = syn::Ident::new( &former_definition_types_name, stru.span() );
  let as_subformer_name = format!( "{}AsSubformer", stru );
  let as_subformer = syn::Ident::new( &as_subformer_name, stru.span() );
  let as_subformer_end_name = format!( "{}AsSubformerEnd", stru );
  let as_subformer_end = syn::Ident::new( &as_subformer_end_name, stru.span() );

  // zzz : improve
  let as_subformer_end_doc = format!( "Alias for trait former::FormingEnd with context and formed the same type and definition of structure [`$(stru)`]. Use as subformer end of a field during process of forming of super structure." );

  /* parameters for structure */

  let generics = &ast.generics;
  let ( struct_generics_with_defaults, struct_generics_impl, struct_generics_ty, struct_generics_where )
  = generic_params::decompose( generics );

  /* parameters for definition */

  let extra : macro_tools::syn::AngleBracketedGenericArguments = parse_quote!
  {
    < (), #stru < #struct_generics_ty >, former::ReturnPreformed >
  };
  let former_definition_args = generic_args::merge( &generics.into_generic_args(), &extra.into() ).args;

  /* parameters for former */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #former_definition < #former_definition_args > >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage < #struct_generics_ty > >,
  };
  let extra = generic_params::merge( &generics, &extra.into() );

  let ( former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where )
  = generic_params::decompose( &extra );

  /* parameters for former perform */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < Definition = #former_definition < #former_definition_args > >
    where
      Definition : former::FormerDefinition
      <
        Storage = #former_storage < #struct_generics_ty >,
        Formed = #stru < #struct_generics_ty >,
      >,
      Definition::Types : former::FormerDefinitionTypes
      <
        Storage = #former_storage < #struct_generics_ty >,
        Formed = #stru < #struct_generics_ty >,
      >,
  };
  let extra = generic_params::merge( &generics, &extra.into() );

  let ( _former_perform_generics_with_defaults, former_perform_generics_impl, former_perform_generics_ty, former_perform_generics_where )
  = generic_params::decompose( &extra );

  /* parameters for definition types */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #stru < #struct_generics_ty > >
  };
  let former_definition_type_generics = generic_params::merge( &generics, &extra.into() );
  let ( former_definition_type_generics_with_defaults, former_definition_type_generics_impl, former_definition_type_generics_ty, former_definition_type_generics_where )
  = generic_params::decompose( &former_definition_type_generics );

  let former_definition_type_phantom = macro_tools::phantom::tuple( &former_definition_type_generics_impl );

  /* parameters for definition */

  let extra : macro_tools::GenericsWithWhere = parse_quote!
  {
    < __Context = (), __Formed = #stru < #struct_generics_ty >, __End = former::ReturnPreformed >
  };
  let generics_of_definition = generic_params::merge( &generics, &extra.into() );
  let ( former_definition_generics_with_defaults, former_definition_generics_impl, former_definition_generics_ty, former_definition_generics_where )
  = generic_params::decompose( &generics_of_definition );

  let former_definition_phantom = macro_tools::phantom::tuple( &former_definition_generics_impl );

  /* structure attribute */

  let ( perform, perform_output, perform_generics ) = performer
  (
    &stru,
    // &former_definition,
    // &struct_generics_ty,
    ast.attrs.iter(),
  )?;

  /* */

  let fields = match ast.data
  {
    syn::Data::Struct( ref data_struct ) => match data_struct.fields
    {
      syn::Fields::Named( ref fields_named ) =>
      {
        &fields_named.named
      },
      _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Fields::Named( ref fields_named )\n  {}", qt!{ #ast } ) ),
    },
    _ => return Err( syn_err!( ast, "Unknown format of data, expected syn::Data::Struct( ref data_struct )\n  {}", qt!{ #ast } ) ),
  };

  let former_fields : Vec< Result< FormerField< '_ > > > = fields.iter().map( | field |
  {
    let attrs = Attributes::parse( &field.attrs )?;
    let vis = &field.vis;
    let ident = field.ident.as_ref()
    .ok_or_else( || syn_err!( field, "Expected that each field has key, but some does not:\n  {}", qt!{ #field } ) )?;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_optional = is_optional( ty );
    let of_type = container_kind::of_optional( ty ).0;
    let non_optional_ty : &syn::Type = if is_optional { parameter_internal_first( ty )? } else { ty };
    let former_field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_optional, of_type };
    Ok( former_field )
  }).collect();

  let former_fields : Vec< _ > = process_results( former_fields, | iter | iter.collect() )?;

  let
  (
    fields_none,
    fields_optional,
    fields_form,
    fields_names,
    fields_setter,
    fields_former_assign,
    fields_former_add,
  )
  :
  ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = former_fields.iter().map( | former_field |
  {(
    field_none_map( former_field ),
    field_optional_map( former_field ),
    field_form_map( former_field ),
    field_name_map( former_field ),
    field_setter_map( former_field, &stru ),
    field_former_assign_end_map
    (
      former_field,
      &stru,
      &former,
      &former_generics_impl,
      &former_generics_ty,
      &former_generics_where,
    ),
    field_former_add_end_map
    (
      former_field,
      &stru,
      &former,
      &former_storage,
      &former_generics_impl,
      &former_generics_ty,
      &former_generics_where,
      &struct_generics_impl,
      &struct_generics_ty,
      &struct_generics_where,
    ),
  )}).multiunzip();

  let ( _doc_former_mod, doc_former_struct ) = doc_generate( stru );
  let fields_setter : Vec< _ > = process_results( fields_setter, | iter | iter.collect() )?;
  let fields_form : Vec< _ > = process_results( fields_form, | iter | iter.collect() )?;
  let fields_former_assign : Vec< _ > = process_results( fields_former_assign, | iter | iter.collect() )?;
  let fields_former_add : Vec< _ > = process_results( fields_former_add, | iter | iter.collect() )?;

  let result = qt!
  {

    // = formed

    #[ automatically_derived ]
    impl < #struct_generics_impl > #stru < #struct_generics_ty >
    where
      #struct_generics_where
    {

      ///
      /// Make former, variation of builder pattern to form structure defining values of fields step by step.
      ///

      #[ inline( always ) ]
      pub fn former() -> #former < #struct_generics_ty #former_definition< #former_definition_args > >
      {
        #former :: < #struct_generics_ty #former_definition< #former_definition_args > > :: new_coercing( former::ReturnPreformed )
      }

    }

    // = entity to former

    impl< #struct_generics_impl Definition > former::EntityToFormer< Definition >
    for #stru < #struct_generics_ty >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      #struct_generics_where
    {
      type Former = #former < #struct_generics_ty Definition > ;
    }

    impl< #struct_generics_impl > former::EntityToStorage
    for #stru < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Storage = #former_storage < #struct_generics_ty >;
    }

    impl< #struct_generics_impl __Context, __Formed, __End > former::EntityToDefinition< __Context, __Formed, __End >
    for #stru < #struct_generics_ty >
    where
      __End : former::FormingEnd< #former_definition_types < #struct_generics_ty __Context, __Formed > >,
      #struct_generics_where
    {
      type Definition = #former_definition < #struct_generics_ty __Context, __Formed, __End >;
    }

    // = definition types

    #[ derive( Debug ) ]
    // pub struct #former_definition_types < #former_definition_type_generics_impl >
    pub struct #former_definition_types < #former_definition_type_generics_with_defaults >
    where
      #former_definition_type_generics_where
    {
      // _phantom : core::marker::PhantomData< ( __Context, __Formed ) >,
      _phantom : #former_definition_type_phantom,
    }

    impl < #former_definition_type_generics_impl > ::core::default::Default
    for #former_definition_types < #former_definition_type_generics_ty >
    where
      #former_definition_type_generics_where
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }
    }

    impl < #former_definition_type_generics_impl > former::FormerDefinitionTypes
    for #former_definition_types < #former_definition_type_generics_ty >
    where
      #former_definition_type_generics_where
    {
      type Storage = #former_storage < #struct_generics_ty >;
      type Formed = __Formed;
      type Context = __Context;
    }

    // = definition

    #[ derive( Debug ) ]
    // pub struct #former_definition < #former_definition_generics_impl >
    pub struct #former_definition < #former_definition_generics_with_defaults >
    where
      #former_definition_generics_where
    {
      // _phantom : core::marker::PhantomData< ( __Context, __Formed, __End ) >,
      _phantom : #former_definition_phantom,
    }

    impl < #former_definition_generics_impl > ::core::default::Default
    for #former_definition < #former_definition_generics_ty >
    where
      #former_definition_generics_where
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }
    }

    impl < #former_definition_generics_impl > former::FormerDefinition
    for #former_definition < #former_definition_generics_ty >
    where
      __End : former::FormingEnd< #former_definition_types < #former_definition_type_generics_ty > >,
      #former_definition_generics_where
    {
      type Types = #former_definition_types < #former_definition_type_generics_ty >;
      type End = __End;
      type Storage = #former_storage < #struct_generics_ty >;
      type Formed = __Formed;
      type Context = __Context;
    }

    // = storage

    #[ doc = "Container of a corresponding former." ]
    #[ allow( explicit_outlives_requirements ) ]
    // pub struct #former_storage < #struct_generics_ty >
    pub struct #former_storage < #struct_generics_with_defaults >
    where
      #struct_generics_where
    {
      #(
        /// A field
        #fields_optional,
      )*
    }

    impl < #struct_generics_impl > ::core::default::Default
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {

      #[ inline( always ) ]
      fn default() -> Self
      {
        Self
        {
          #( #fields_none, )*
        }
      }

    }

    impl < #struct_generics_impl > former::Storage
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Formed = #stru < #struct_generics_ty >;
    }

    impl < #struct_generics_impl > former::StoragePreform
    for #former_storage < #struct_generics_ty >
    where
      #struct_generics_where
    {
      type Preformed = #stru < #struct_generics_ty >;

      // fn preform( mut self ) -> < Self as former::Storage >::Formed
      fn preform( mut self ) -> Self::Preformed
      {
        #( #fields_form )*
        // Rust does not support that, yet
        // let result = < Definition::Types as former::FormerDefinitionTypes >::Formed
        let result = #stru :: < #struct_generics_ty >
        {
          #( #fields_names, )*
        };
        return result;
      }

    }

    // = former

    #[ doc = #doc_former_struct ]
    pub struct #former < #former_generics_with_defaults >
    where
      #former_generics_where
    {
      storage : Definition::Storage,
      context : core::option::Option< Definition::Context >,
      on_end : core::option::Option< Definition::End >,
      // zzz : should on_end be optional?
    }

    #[ automatically_derived ]
    impl < #former_generics_impl > #former < #former_generics_ty >
    where
      #former_generics_where
    {

      ///
      /// Construct new instance of former with default parameters.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn new( on_end : Definition::End ) -> Self
      {
        Self::begin_coercing( None, None, on_end )
      }

      ///
      /// Construct new instance of former with default parameters.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self
      where
        IntoEnd : Into< Definition::End >,
      {
        Self::begin_coercing
        (
          None,
          None,
          end,
        )
      }

      ///
      /// Begin the process of forming. Expects context of forming to return it after forming.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn begin
      (
        mut storage : core::option::Option< Definition::Storage >,
        context : core::option::Option< Definition::Context >,
        on_end : < Definition as former::FormerDefinition >::End,
      )
      -> Self
      {
        if storage.is_none()
        {
          storage = Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( on_end ),
        }
      }

      ///
      /// Begin the process of forming. Expects context of forming to return it after forming.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn begin_coercing< IntoEnd >
      (
        mut storage : core::option::Option< Definition::Storage >,
        context : core::option::Option< Definition::Context >,
        on_end : IntoEnd,
      ) -> Self
      where
        IntoEnd : ::core::convert::Into< < Definition as former::FormerDefinition >::End >,
      {
        if storage.is_none()
        {
          storage = Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( ::core::convert::Into::into( on_end ) ),
        }
      }

      ///
      /// End the process of forming returning original context of forming.
      ///
      #[ inline( always ) ]
      pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        self.end()
      }

      ///
      /// End the process of forming returning original context of forming.
      ///
      #[ inline( always ) ]
      pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        let on_end = self.on_end.take().unwrap();
        let context = self.context.take();
        former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
      }

      #(
        #fields_setter
      )*

    }

    // = former :: preform

    impl< #former_generics_impl > #former< #former_generics_ty >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty >, Formed = #stru < #struct_generics_ty > >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage < #struct_generics_ty >, Formed = #stru < #struct_generics_ty > >,
      #former_generics_where
    {

      pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        former::StoragePreform::preform( self.storage )
      }

    }

    // = former :: perform

    #[ automatically_derived ]
    impl < #former_perform_generics_impl > #former < #former_perform_generics_ty >
    where
      #former_perform_generics_where
    {

      ///
      /// Finish setting options and call perform on formed entity.
      ///
      /// If `perform` defined then associated method is called and its result returned instead of entity.
      /// For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`.
      ///
      #[ inline( always ) ]
      pub fn perform #perform_generics ( self ) -> #perform_output
      {
        let result = self.form();
        #perform
      }

    }

    // = former begin

    impl< #struct_generics_impl Definition > former::FormerBegin< Definition >
    // for ChildFormer< Definition >
    for #former
    <
      #struct_generics_ty
      Definition,
    >
    where
      Definition : former::FormerDefinition< Storage = #former_storage < #struct_generics_ty > >,
      #struct_generics_where
    {

      #[ inline( always ) ]
      fn former_begin
      (
        storage : core::option::Option< Definition::Storage >,
        context : core::option::Option< Definition::Context >,
        on_end : Definition::End,
      )
      -> Self
      {
        debug_assert!( storage.is_none() );
        Self::begin( None, context, on_end )
      }

    }

    // = subformer

    // zzz : improve description
    /// Use as subformer of a field during process of forming of super structure.
    pub type #as_subformer < #struct_generics_ty __Superformer, __End > = #former
    <
      #struct_generics_ty
      #former_definition
      <
        #struct_generics_ty
        __Superformer,
        __Superformer,
        __End,
        // impl former::FormingEnd< CommandFormerDefinitionTypes< K, __Superformer, __Superformer > >,
      >,
    >;

    // = as subformer end

    // zzz : imporove documentation
    #[ doc = #as_subformer_end_doc ]
    pub trait #as_subformer_end < #struct_generics_impl SuperFormer >
    where
      #struct_generics_where
      Self : former::FormingEnd
      <
        #former_definition_types < #struct_generics_ty SuperFormer, SuperFormer >,
      >,
    {
    }

    impl< #struct_generics_impl SuperFormer, __T > #as_subformer_end < #struct_generics_ty SuperFormer >
    for __T
    where
      #struct_generics_where
      Self : former::FormingEnd
      <
        #former_definition_types < #struct_generics_ty SuperFormer, SuperFormer >,
      >,
    {
    }

    // = container assign callbacks

    #(
      #fields_former_assign
    )*

    // = container add callbacks

    #(
      #fields_former_add
    )*

  };

  if has_debug
  {
    diag::debug_report_print( "derive : Former", original_input, &result );
  }

  // zzz : implement hints, rewrite
  if example_of_custom_setter
  {
    let _example =
r#"
impl< Context, End > UserProfileFormer< Context, End >
where
  End : former::FormingEnd< UserProfile, Context >,
{
  pub fn age< Src >( mut self, src : Src ) -> Self
  where
    Src : Into< i32 >,
  {
    debug_assert!( self.age.is_none() );
    self.storage.age = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
    self
  }
}
"#;
  }

  Ok( result )
}

// zzz : explain concept of Storage
