
use super::*;
use iter_tools::{ Itertools, process_results };
use macro_tools::{ attr, diag, generics, container_kind, typ, Result };
use proc_macro2::TokenStream;

///
/// Descriptor of a field.
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
  // #[ allow( dead_code ) ]
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
/// In simple terms, used on `form()` call to unwrap contained values from the former's container.
/// Will try to use default values if no values supplied by the former and the type implements `Default` trait.
///
/// ### Example of generated code for an optional field
///
/// ```ignore
/// let int_1 : i32 = if self.container.int_1.is_some()
/// {
///   Some( self.container.int_1.take().unwrap() )
/// }
/// else
/// {
///   None
/// };
/// ```
///
/// ### Example of generated code for a non-optional field
/// 
/// ```ignore
/// let int_1 : i32 = if self.container.int_1.is_some()
/// {
///   self.container.int_1.unwrap()
/// }
/// else
/// {
///   i32::default() // oversimplified
/// }
/// ```

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
          ::core::option::Option::Some( ( #default_val ).into() )
        }
      }
    };

    qt!
    {
      let #ident = if self.container.#ident.is_some()
      {
        ::core::option::Option::Some( self.container.#ident.take().unwrap() )
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
          ( #default_val ).into()
        }
      }
    };

    qt!
    {
      let #ident = if self.container.#ident.is_some()
      {
        self.container.#ident.take().unwrap()
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
/// # Example of output
/// ```ignore
/// #[ doc = "Setter for the 'int_1' field." ]
/// #[ inline ]
/// pub fn int_1< Src >( mut self, src : Src ) -> Self
/// where
///   Src : ::core::convert::Into< i32 >,
/// {
///   debug_assert!( self.int_1.is_none() );
///   self.container.int_1 = ::core::option::Option::Some( src.into() );
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
///   self.container.int_1 = ::core::option::Option::Some( src.into() );
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
    subformer_field_setter( ident, ident, non_optional_ty, &subformer_ty.expr )
    // field_setter( ident, ident, non_optional_ty )
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
/// # Example of output
/// ```ignore
/// #[ doc = "Setter for the 'int_1' field." ]
/// #[ inline ]
/// pub fn int_1< Src >( mut self, src : Src ) -> Self
/// where
///   Src : ::core::convert::Into< i32 >,
/// {
///   debug_assert!( self.int_1.is_none() );
///   self.container.int_1 = ::core::option::Option::Some( src.into() );
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
      debug_assert!( self.container.#field_ident.is_none() );
      self.container.#field_ident = ::core::option::Option::Some( src.into() );
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
///   let container = self.hashmap_strings_1.take();
///   let on_end = | container : std::collections::HashMap< String, String >, mut former : core::option::Option< Self > | -> Self
///   {
///     former.hashmap_strings_1 = Some( container );
///     former
///   };
///   former::HashMapSubformer::begin( self, container, on_end )
/// }
/// ```

#[ inline ]
fn subformer_field_setter
(
  field_ident : &syn::Ident,
  setter_name : &syn::Ident,
  non_optional_type : &syn::Type,
  subformer_type : &syn::Type,
)
-> TokenStream
{
  let doc = format!
  (
    "Subformer setter for the '{}' field.",
    field_ident
  );

  // tree_print!( non_optional_type );
  // code_print!( non_optional_type );
  let params = typ::type_parameters( &non_optional_type, .. );
  // params.iter().for_each( | e | println!( "{}", qt!( #e ) ) );

  qt!
  {
    #[ doc = #doc ]
    #[ inline ]
    pub fn #setter_name( mut self ) -> #subformer_type
    <
      #( #params, )*
      #non_optional_type,
      Self,
      impl Fn( #non_optional_type, core::option::Option< Self > ) -> Self,
    >
    {
      let container = self.container.#setter_name.take();
      let on_end = | container : #non_optional_type, former : core::option::Option< Self > | -> Self
      {
        let mut former = former.unwrap();
        former.container.#setter_name = Some( container );
        former
      };
      #subformer_type::begin( Some( self ), container, on_end )
    }
  }

}

///
/// Generate documentation for the former.
///

fn doc_generate( name_ident : &syn::Ident ) -> ( String, String )
{

  let doc_former_mod = format!
  (
r#" Implementation of former for [{}].
"#,
    name_ident
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
    name_ident, doc_example1
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
/// <T: Default>
///
/// ## perform_generics :
/// Vec<T>

pub fn performer< 'a >
(
  name_ident : &syn::Ident,
  generics_ty : &syn::TypeGenerics< '_ >,
  attrs : impl Iterator< Item = &'a syn::Attribute >,
)
-> Result< ( TokenStream, TokenStream, TokenStream ) >
{

  let mut perform = qt!
  {
    return result;
  };
  let mut perform_output = qt!{ #name_ident #generics_ty };
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


  /* names */

  let name_ident = &ast.ident;
  let former_name = format!( "{}Former", name_ident );
  let former_name_ident = syn::Ident::new( &former_name, name_ident.span() );
  let former_container_name = format!( "{}FormerContainer", name_ident );
  let former_container_name_ident = syn::Ident::new( &former_container_name, name_ident.span() );

  /* generic parameters */

  let generics = &ast.generics;
  let ( generics_impl, generics_ty, generics_where ) = generics.split_for_impl();
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
  let mut extra_generics : syn::Generics = parse_quote!{ < __FormerContext = #name_ident #generics_ty, __FormerEnd = former::ReturnContainer > };
  extra_generics.where_clause = parse_quote!{ where __FormerEnd : former::ToSuperFormer< #name_ident #generics_ty, __FormerContext >, };
  // xxx : write helper to fix the bug
  let generics_of_former = generics::merge( &generics, &extra_generics );
  let ( generics_of_former_impl, generics_of_former_ty, generics_of_former_where ) = generics_of_former.split_for_impl();
  let generics_of_former_with_defaults = generics_of_former.params.clone();
  // macro_tools::code_print!( generics_of_former_with_defaults );
  // macro_tools::code_print!( extra_generics );

  /* structure attribute */

  let ( perform, perform_output, perform_generics ) = performer
  (
    &name_ident,
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

  let ( fields_none, fields_optional, fields_form, fields_names, fields_setter )
  : ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = former_fields.iter().map( | former_field |
  {(
    field_none_map( former_field ),
    field_optional_map( former_field ),
    field_form_map( former_field ),
    field_name_map( former_field ),
    field_setter_map( former_field ),
  )}).multiunzip();

  let ( _doc_former_mod, doc_former_struct ) = doc_generate( name_ident );
  let fields_setter : Vec< _ > = process_results( fields_setter, | iter | iter.collect() )?;
  let fields_form : Vec< _ > = process_results( fields_form, | iter | iter.collect() )?;

  let result = qt!
  {

    #[ automatically_derived ]
    impl #generics_impl #name_ident #generics_ty
    #generics_where
    {
      ///
      /// Make former, variation of builder pattern to form structure defining values of fields step by step.
      ///
      #[ inline( always ) ]
      pub fn former() -> #former_name_ident < #generics_params #name_ident #generics_ty, former::ReturnContainer >
      {
        #former_name_ident :: < #generics_params #name_ident #generics_ty, former::ReturnContainer > :: new()
      }
    }

    #[ doc = "Container of a corresponding former." ]
    pub struct #former_container_name_ident #generics_ty
    #generics_where
    {
      #(
        /// A field
        #fields_optional,
      )*
    }

    impl #generics_impl core::default::Default for #former_container_name_ident #generics_ty
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

    #[ doc = #doc_former_struct ]
    #[ automatically_derived ]
    pub struct #former_name_ident < #generics_of_former_with_defaults >
    #generics_of_former_where
    {
      container : #former_container_name_ident #generics_ty,
      context : core::option::Option< __FormerContext >,
      on_end : core::option::Option< __FormerEnd >,
    }

    #[ automatically_derived ]
    impl #generics_of_former_impl #former_name_ident #generics_of_former_ty
    #generics_of_former_where
    {

      ///
      /// Finish setting options and return formed entity.
      ///
      /// `perform` has no effect on method `form`, but change behavior and returned type of method `perform`.
      ///
      #[ inline( always ) ]
      pub fn form( mut self ) -> #name_ident #generics_ty
      {
        #( #fields_form )*
        let result = #name_ident
        {
          #( #fields_names, )*
        };
        return result;
      }

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
      #[ inline( always ) ]
      pub fn new() -> #former_name_ident < #generics_params #name_ident #generics_ty, former::ReturnContainer >
      {
        #former_name_ident :: < #generics_params #name_ident #generics_ty, former::ReturnContainer > :: begin
        (
          None,
          former::ReturnContainer,
        )
      }

      ///
      /// Begin the process of forming. Expects context of forming to return it after forming.
      ///
      #[ inline( always ) ]
      pub fn begin
      (
        context :  core::option::Option< __FormerContext >,
        on_end : __FormerEnd,
      ) -> Self
      {
        Self
        {
          container : core::default::Default::default(),
          context : context,
          on_end : ::core::option::Option::Some( on_end ),
        }
      }

      ///
      /// End the process of forming returning original context of forming.
      ///
      #[ inline( always ) ]
      pub fn end( mut self ) -> __FormerContext
      {
        let on_end = self.on_end.take().unwrap();
        let context = self.context.take();
        let container = self.form();
        on_end.call( container, context )
      }

      #(
        #fields_setter
      )*

    }

  };

  if has_debug
  {
    diag::debug_report_print( "derive : Former",original_input, &result );
  }

  Ok( result )
}
