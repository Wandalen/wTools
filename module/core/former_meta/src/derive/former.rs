
use super::*;
use iter_tools::{ Itertools, process_results };
use macro_tools::{ attr, diag, generics, container_kind, typ, Result };
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

///
/// Attributes of the field.
///

struct Attributes
{
  default : Option< AttributeDefault >,
  setter : Option< AttributeSetter >,
  subformer : Option< AttributeFormer >,
  alias : Option< AttributeAlias >,
}

impl Attributes
{
  fn parse( attributes : & Vec< syn::Attribute > ) -> Result< Self >
  {
    let mut default = None;
    let mut setter = None;
    let mut subformer = None;
    let mut alias = None;
    for attr in attributes
    {
      let key_ident = attr.path().get_ident()
      .ok_or_else( || syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ) )?;
      let key_str = format!( "{}", key_ident );
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
            _ => return_syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ),
          }
        }
        "setter" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              setter.replace( syn::parse2::< AttributeSetter >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ),
          }
          // let attr_setter = syn::parse2::< AttributeSetter >( attr.tokens.clone() )?;
          // setter.replace( attr_setter );
        }
        "subformer" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              subformer.replace( syn::parse2::< AttributeFormer >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ),
          }
          // let attr_former = syn::parse2::< AttributeFormer >( attr.tokens.clone() )?;
          // subformer.replace( attr_former );
        }
        "alias" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              alias.replace( syn::parse2::< AttributeAlias >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ),
          }
          // let attr_alias = syn::parse2::< AttributeAlias >( attr.tokens.clone() )?;
          // alias.replace( attr_alias );
        }
        "doc" =>
        {
        }
        _ =>
        {
          return Err( syn_err!( attr, "Unknown attribute {}", qt!{ #attr } ) );
        }
      }
    }

    Ok( Attributes { default, setter, subformer, alias } )
  }
}

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform( fn after1< 'a >() -> Option< &'a str > ) ]`
///

#[ allow( dead_code ) ]
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

#[ allow( dead_code ) ]
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
/// Attribute to enable/disable setter generation.
///
/// `#[ setter( false ) ]`
///

#[ allow( dead_code ) ]
struct AttributeSetter
{
  // paren_token : syn::token::Paren,
  condition : syn::LitBool,
}

impl syn::parse::Parse for AttributeSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    // let input2;
    Ok( Self
    {
      // paren_token : syn::parenthesized!( input2 in input ),
      // condition : input2.parse()?,
      condition : input.parse()?,
    })
  }
}

///
/// Attribute to enable/disable former generation.
/// Also known as subformers, used for aggregation relationship, when a struct holds another struct, which needs to be build by invoking multiple methods
/// Typical example is a struct holding a `Vec`
///
/// `#[ subformer( former::VectorSubformer ) ]`
///
// qqq : update documentation

#[ allow( dead_code ) ]
struct AttributeFormer
{
  // paren_token : syn::token::Paren,
  expr : syn::Type,
}

impl syn::parse::Parse for AttributeFormer
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    // let input2;
    Ok( Self
    {
      // paren_token : syn::parenthesized!( input2 in input ),
      // expr : input2.parse()?,
      expr : input.parse()?,
    })
  }
}

///
/// Attribute to create alias.
///
/// `#[ alias( name ) ]`
///

#[ allow( dead_code ) ]
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
fn field_setter_map( field : &FormerField< '_ > ) -> Result< TokenStream >
{
  let ident = &field.ident;

  if let Some( setter_attr ) = &field.attrs.setter
  {
    if !setter_attr.condition.value()
    {
      return Ok( qt!{ } );
    }
  }

  let non_optional_ty = &field.non_optional_ty;
  // Either subformer or ordinary setter.
  let setter_tokens = if let Some( subformer_ty ) = &field.attrs.subformer
  {
    // subformer_field_setter( ident, ident, non_optional_ty, &subformer_ty.expr )
    subformer_field_setter( field, &subformer_ty.expr )
  }
  else
  {
    field_setter( ident, ident, non_optional_ty )
  };

  let r = if let Some( alias_attr ) = &field.attrs.alias
  {
    let alias_tokens = field_setter( ident, &alias_attr.alias, non_optional_ty );
    let token = qt!
    {
      #setter_tokens
      #alias_tokens
    };
    Ok( token )
  }
  else
  {
    Ok( setter_tokens )
  };

  // tree_print!( r.as_ref().unwrap() );
  r
}


///
/// Generate a single setter for the 'field_ident' with the 'setter_name' name.
///
/// Used as a helper function for field_setter_map(), which generates all alias setters
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
fn field_setter
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

///
/// Generate a sub-former setter for the 'field_ident' with the 'setter_name' name.
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
///   former::HashMapSubformer::begin( formed, self, on_end )
/// }
/// ```
/// zzz : update example

#[ inline ]
fn subformer_field_setter
(
  field : &FormerField< '_ >,
  // field_ident : &syn::Ident,
  // setter_name : &syn::Ident,
  // non_optional_type : &syn::Type,
  subformer_type : &syn::Type,
)
-> TokenStream
{
  let field_ident = &field.ident;
  let doc = format!
  (
    "Subformer setter for the '{}' field.",
    field_ident
  );

  let non_optional_ty = &field.non_optional_ty;

  // tree_print!( non_optional_type );
  // code_print!( non_optional_type );
  let params = typ::type_parameters( &non_optional_ty, .. );
  // params.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
  // let genertic_params = typ::all_type_parameters( non_optional_type );
  // xxx : try `all_type_parameters`` instead

  let subformer_definition = &field.attrs.subformer.as_ref().unwrap().expr;
  // for example : former::VectorDefinition

  use convert_case::{ Case, Casing };
  // let ident = field_ident;
  let field_forming_end_name = format!( "former{}End", field_ident.to_string().to_case( Case::Camel ) );
  let field_forming_end = syn::Ident::new( &field_forming_end_name, field_ident.span() );
  let field_set_name = format!( "{}_set", field_ident );
  let field_set = syn::Ident::new( &field_forming_end_name, field_ident.span() );

  qt!
  {
    #[ inline( always ) ]
    pub fn #field_set< Former2 >( self ) -> Former2
    where
      Former2 : former::FormerBegin
      <
        #subformer_definition
        <
          #( #params, )*,
          Self,
          Self,
          #field_set,
        >
      >,
    {
      Former2::_begin( None, Some( self ), #field_set )
    }

    pub fn #field_ident( self ) ->
    former::ContainerSubformer::
    <
      #( #params, )*, #subformer_definition< #( #params, )*, Self, Self, #field_set >
    >
    {
      self.#field_set::< former::ContainerSubformer::
      <
        #( #params, )*, #subformer_definition< #( #params, )*, Self, Self, #field_set >
      >>()
    }

  }

  // qt!
  // {
  //   #[ doc = #doc ]
  //   #[ inline ]
  //   pub fn #setter_name( mut self ) -> #subformer_type
  //   <
  //     #( #params, )*
  //     #non_optional_type,
  //     Self,
  //     impl Fn( #non_optional_type, core::option::Option< Self > ) -> Self,
  //   >
  //   {
  //     let formed = self.storage.#setter_name.take();
  //     let on_end = | formed : #non_optional_type, former : core::option::Option< Self > | -> Self
  //     {
  //       let mut former = former.unwrap();
  //       former.storage.#setter_name = Some( formed );
  //       former
  //     };
  //     #subformer_type::begin( formed, Some( self ), on_end )
  //   }
  // }

//   #[ inline( always ) ]
//   pub fn vec_1_set< Former2 >( self ) -> Former2
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
//     self.vec_1_set::< former::ContainerSubformer::
//     <
//       String, former::VectorDefinition< String, Self, Self, Struct1FormerVec_1End >
//     >>()
//   }

}

// zzz : description and exmaple
/// Generate unit struct which is descriptor of callback which should be called after subforming process of a specific field. Descriptors are used insted of closures to inline code and let optimizer play with optimization.
///
/// # Example of generated code
///
/// ```rust, ignore
/// #[ allow( non_camel_case_types ) ]
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
fn fields_setter_callback_descriptor_map
(
  field : &FormerField< '_ >,
  former : &syn::Ident,
  former_storage : &syn::Ident,
  former_definition : &syn::Ident,
)
->
Result< TokenStream >
{

  if field.attrs.subformer.is_none()
  {
    return Ok( qt!{ } );
  }

  let subformer = field.attrs.subformer.as_ref().unwrap();
  // former::VectorDefinition
  // xxx

  use convert_case::{ Case, Casing };
  let ident = field.ident;
  let field_forming_end_name = format!( "former{}End", ident.to_string().to_case( Case::Camel ) );
  let field_forming_end = syn::Ident::new( &field_forming_end_name, ident.span() );

  // let field_ty = field.non_optional_ty;
  let params = typ::type_parameters( &field.non_optional_ty, .. );
  // let params = typ::all_type_parameters( field.non_optional_ty );
  // let xxx = field_ty;
  // let generics = field_ty.generics
  // let ( generics_impl, generics_ty, generics_where ) = generics.split_for_impl();

  let r = qt!
  {
    // xxx

    // zzz : description
    /// Return original former after subformer for `vec_1` is done.
    #[ allow( non_camel_case_types ) ]
    pub struct #field_forming_end;
    #[ automatically_derived ]
    impl< Definition > former::FormingEnd
    <
      former::VectorDefinition< #( #params, )* #former< Definition >, #former< Definition >, former::NoEnd >,
      // xxx : what is there is no generic parameters?
    >
    for #field_forming_end
    where
      Definition : former::FormerDefinition,
      Definition::Types : former::FormerDefinitionTypes
      <
        Storage = #former_storage
      >,
    {
      #[ inline( always ) ]
      fn call
      (
        &self,
        storage : field_ty,
        super_former : Option< #former< Definition > >,
      )
      -> #former< Definition >
      {
        let mut super_former = super_former.unwrap();
        if let Some( ref mut field ) = super_former.storage.#ident
        {
          former::ContainerAssign::assign( field, storage );
        }
        else
        {
          super_former.storage.#ident = Some( storage );
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

fn doc_generate( struct_name : &syn::Ident ) -> ( String, String )
{

  let doc_former_mod = format!
  (
r#" Implementation of former for [{}].
"#,
    struct_name
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
    struct_name, doc_example1
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
  _former_definition : &syn::Ident,
  _generics_ty : &syn::TypeGenerics< '_ >,
  attrs : impl Iterator< Item = &'a syn::Attribute >,
)
-> Result< ( TokenStream, TokenStream, TokenStream ) >
{

  let mut perform = qt!
  {
    return result;
  };
  // let mut perform_output = qt!{ #struct_name #generics_ty };
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

  let original_input = input.clone();
  let ast = match syn::parse::< syn::DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };
  let has_debug = attr::has_debug( ast.attrs.iter() )?;
  let example_of_custom_setter = false;


  /* names */

  let struct_name = &ast.ident;
  let former_name = format!( "{}Former", struct_name );
  let former = syn::Ident::new( &former_name, struct_name.span() );
  let former_storage_name = format!( "{}FormerStorage", struct_name );
  let former_storage = syn::Ident::new( &former_storage_name, struct_name.span() );
  let former_definition_name = format!( "{}FormerDefinition", struct_name );
  let former_definition = syn::Ident::new( &former_definition_name, struct_name.span() );
  let former_definition_types_name = format!( "{}FormerDefinitionTypes", struct_name );
  let former_definition_types = syn::Ident::new( &former_definition_types_name, struct_name.span() );
  let former_with_closure_name = format!( "{}FormerWithClosure", struct_name );
  let former_with_closure = syn::Ident::new( &former_with_closure_name, struct_name.span() );

  /* generic parameters */

  let generics = &ast.generics;
  let ( generics_impl, generics_ty, generics_where ) = generics.split_for_impl();
  // zzz : eliminate generics_params maybe
  let _generics_params = generics::params_names( generics ).params;
  let generics_params = if _generics_params.len() == 0
  {
    qt!{}
  }
  else
  {
    qt!{ #_generics_params, }
  };

  // add embedded generic parameters
  let mut extra_generics : syn::Generics = parse_quote!
  {
    < Definition = #former_definition #generics_ty >
  };
  extra_generics.where_clause = parse_quote!
  {
    where
      Definition : former::FormerDefinition,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage #generics_ty >,
      < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  };
  // zzz : write helper to fix bug with where
  let generics_of_former = generics::merge( &generics, &extra_generics );
  let ( generics_of_former_impl, generics_of_former_ty, generics_of_former_where ) = generics_of_former.split_for_impl();
  let generics_of_former_with_defaults = generics_of_former.params.clone();
  // macro_tools::code_print!( generics_of_former_with_defaults );
  // macro_tools::code_print!( extra_generics );

  /* structure attribute */

  let ( perform, perform_output, perform_generics ) = performer
  (
    &struct_name,
    &former_definition,
    &generics_ty,
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
    fields_setter_callback_descriptor,
  )
  :
  ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = former_fields.iter().map( | former_field |
  {(
    field_none_map( former_field ),
    field_optional_map( former_field ),
    field_form_map( former_field ),
    field_name_map( former_field ),
    field_setter_map( former_field ),
    fields_setter_callback_descriptor_map( former_field, &former, &former_storage, &former_definition ),
  )}).multiunzip();

  let ( _doc_former_mod, doc_former_struct ) = doc_generate( struct_name );
  let fields_setter : Vec< _ > = process_results( fields_setter, | iter | iter.collect() )?;
  let fields_form : Vec< _ > = process_results( fields_form, | iter | iter.collect() )?;
  let fields_setter_callback_descriptor : Vec< _ > = process_results( fields_setter_callback_descriptor, | iter | iter.collect() )?;

  let result = qt!
  {

    // = formed

    #[ automatically_derived ]
    impl #generics_impl #struct_name #generics_ty
    #generics_where
    {
      ///
      /// Make former, variation of builder pattern to form structure defining values of fields step by step.
      ///

      #[ inline( always ) ]
      pub fn former() -> #former < #generics_params >
      {
        #former :: < #generics_params > :: new( former::ReturnPreformed )
      }

    }

    // = definition

    #[ derive( Debug ) ]
    pub struct #former_definition_types< Context = (), Formed = #struct_name #generics_ty >
    {
      _phantom : core::marker::PhantomData< ( Context, Formed ) >,
    }

    impl< Context, Formed > Default
    for #former_definition_types< Context, Formed >
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }
    }

    #[ derive( Debug ) ]
    pub struct #former_definition< Context = (), Formed = #struct_name #generics_ty, End = former::ReturnPreformed >
    // where
    //   End : former::FormingEnd< #former_definition< Context, Formed, NoEnd > >,
    {
      _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
    }

    impl< Context, Formed, End > Default
    for #former_definition< Context, Formed, End >
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : core::marker::PhantomData,
        }
      }
    }

    impl< Context, Formed > former::FormerDefinitionTypes
    for #former_definition_types< Context, Formed >
    {
      type Storage = #former_storage #generics_ty;
      type Formed = Formed;
      type Context = Context;
    }

    impl< Context, Formed, End > former::FormerDefinition
    for #former_definition< Context, Formed, End >
    where
      End : former::FormingEnd< #former_definition_types< Context, Formed > >,
    {
      type Types = #former_definition_types< Context, Formed >;
      type End = End;
    }

    pub type #former_with_closure< Context, Formed > =
    #former_definition< Context, Formed, former::FormingEndClosure< #former_definition_types< Context, Formed > > >;

    // = storage

    #[ doc = "Container of a corresponding former." ]
    pub struct #former_storage #generics_ty
    #generics_where
    {
      #(
        /// A field
        #fields_optional,
      )*
    }

    impl #generics_impl ::core::default::Default for #former_storage #generics_ty
    #generics_where
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

    impl #generics_impl former::Storage
    for #former_storage #generics_ty
    #generics_where
    {
      // type Definition = Struct1FormerDefinition;
      // type Definition = #former_definition #generics_ty;
      type Formed = #struct_name #generics_ty;
    }
    // generics_impl, generics_ty, generics_where

    impl former::StoragePreform
    for #former_storage #generics_ty
    #generics_where
    {

      // fn preform( mut self ) -> #former_storage #generics_ty
      // fn preform( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      fn preform( mut self ) -> < Self as former::Storage >::Formed
      {
        #( #fields_form )*
        // Rust does not support that, yet
        // let result = < Definition::Types as former::FormerDefinitionTypes >::Formed
        let result = #struct_name #generics_ty
        {
          #( #fields_names, )*
        };
        return result;
      }

    }

    // = former

    #[ doc = #doc_former_struct ]
    pub struct #former < #generics_of_former_with_defaults >
    #generics_of_former_where
    {
      storage : < Definition::Types as former::FormerDefinitionTypes >::Storage,
      context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
      on_end : core::option::Option< Definition::End >,
      // zzz : should on_end be optional?
    }

    #[ automatically_derived ]
    impl #generics_of_former_impl #former #generics_of_former_ty
    #generics_of_former_where
    {

      // ///
      // /// Finish setting options and return formed entity.
      // ///
      // #[ inline( always ) ]
      // pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      // // #struct_name #generics_ty
      // {
      //   former::StoragePreform::preform( self.storage )
      //   // < #former_storage #generics_ty as former::StoragePreform >::preform( self.storage )
      //   // #( #fields_form )*
      //   // let result = #struct_name
      //   // {
      //   //   #( #fields_names, )*
      //   // };
      //   // return result;
      // }

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

      ///
      /// Construct new instance of former with default parameters.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn _new_precise( on_end : Definition::End ) -> Self
      {
        Self::begin( None, None, on_end )
      }

      ///
      /// Construct new instance of former with default parameters.
      ///
      // zzz : improve description
      #[ inline( always ) ]
      pub fn new< IntoEnd >( end : IntoEnd ) -> Self
      where
        IntoEnd : Into< Definition::End >,
      {
        Self::begin
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
      pub fn _begin_precise
      (
        mut storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
        context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
        on_end : < Definition as former::FormerDefinition >::End,
      ) -> Self
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
      pub fn begin< IntoEnd >
      (
        mut storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
        context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
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
        // let storage = self.form();
        // on_end.call( self.storage, context )
        // former::FormingEnd::< #former_definition #generics_ty >::call( &on_end, self.storage, context )
        former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
      }

      #(
        #fields_setter
      )*

    }

    impl< Definition > #former< Definition >
    where
      Definition : former::FormerDefinition,
      Definition::Types : former::FormerDefinitionTypes< Storage = #former_storage #generics_ty, Formed = #struct_name #generics_ty >,
      < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
    {

      pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        former::StoragePreform::preform( self.storage )
      }

    }

    #(
      #fields_setter_callback_descriptor
    )*

  };

  if has_debug
  {
    diag::debug_report_print( "derive : Former", original_input, &result );
  }

  // zzz : implement hints
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
