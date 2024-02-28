
use iter_tools::{ Itertools, process_results };
use macro_tools::*;

pub type Result< T > = std::result::Result< T, syn::Error >;

///
/// Descripotr of a field.
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
  pub type_container_kind : macro_tools::ContainerKind,
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
      let key_ident = attr.path.get_ident()
      .ok_or_else( || syn_err!( attr, "Expects simple key of an attirbute, but got:\n  {}", qt!{ #attr } ) )?;
      let key_str = format!( "{}", key_ident );
      match key_str.as_ref()
      {
        "default" =>
        {
          let attr_default = syn::parse2::< AttributeDefault >( attr.tokens.clone() )?;
          default.replace( attr_default );
        }
        "setter" =>
        {
          let attr_setter = syn::parse2::< AttributeSetter >( attr.tokens.clone() )?;
          setter.replace( attr_setter );
        }
        "subformer" =>
        {
          let attr_former = syn::parse2::< AttributeFormer >( attr.tokens.clone() )?;
          subformer.replace( attr_former );
        }
        "alias" =>
        {
          let attr_alias = syn::parse2::< AttributeAlias >( attr.tokens.clone() )?;
          alias.replace( attr_alias );
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
/// `#[ perform = ( fn after1< 'a >() -> Option< &'a str > ) ]`
///

#[ allow( dead_code ) ]
struct AttributeFormAfter
{
  paren_token : syn::token::Paren,
  signature : syn::Signature,
}

impl syn::parse::Parse for AttributeFormAfter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    let input2;
    Ok( Self
    {
      paren_token : syn::parenthesized!( input2 in input ),
      signature : input2.parse()?,
    })
  }
}

///
/// Attribute to hold information about default value.
///
/// `#[ default = 13 ]`
///

#[ allow( dead_code ) ]
struct AttributeDefault
{
  // eq_token : syn::Token!{ = },
  paren_token : syn::token::Paren,
  expr : syn::Expr,
}

impl syn::parse::Parse for AttributeDefault
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    let input2;
    Ok( Self
    {
      paren_token : syn::parenthesized!( input2 in input ),
      // eq_token : input.parse()?,
      expr : input2.parse()?,
    })
  }
}

// qqq : xxx : implement test for setter

///
/// Attribute to enable/disable setter generation.
///
/// `#[ setter = false ]`
///
#[ allow( dead_code ) ]
struct AttributeSetter
{
  paren_token : syn::token::Paren,
  condition : syn::LitBool,
}

impl syn::parse::Parse for AttributeSetter
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    let input2;
    Ok( Self
    {
      paren_token : syn::parenthesized!( input2 in input ),
      condition : input2.parse()?,
    })
  }
}

///
/// Attribute to enable/disable former generation.
///
/// `#[ former( former::runtime::VectorSubformer ) ]`
///

#[ allow( dead_code ) ]
struct AttributeFormer
{
  paren_token : syn::token::Paren,
  expr : syn::Type,
}

impl syn::parse::Parse for AttributeFormer
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    let input2;
    Ok( Self
    {
      paren_token : syn::parenthesized!( input2 in input ),
      expr : input2.parse()?,
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
  paren_token : syn::token::Paren,
  alias : syn::Ident,
}

impl syn::parse::Parse for AttributeAlias
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    let input2;
    Ok( Self
    {
      paren_token : syn::parenthesized!( input2 in input ),
      alias : input2.parse()?,
    })
  }
}

///
/// Is type under Option.
///

fn is_optional( ty : &syn::Type ) -> bool
{
  macro_tools::type_rightmost( ty ) == Some( "Option".to_string() )
}

///
/// Extract the first parameter of the type if such exist.
///

fn parameter_internal_first( ty : &syn::Type ) -> Result< &syn::Type >
{
  macro_tools::type_parameters( ty, 0 ..= 0 )
  .first()
  .copied()
  .ok_or_else( || syn_err!( ty, "Expects at least one parameter here:\n  {}", qt!{ #ty } ) )
}

///
/// Generate fields for initializer of a struct setting each field to `None`.
///
/// ### Basic use-case. of output
///
/// ```compile_fail
/// int_1 : core::option::Option::None,
/// string_1 : core::option::Option::None,
/// int_optional_1 : core::option::Option::None,
/// ```
///

#[inline]
fn field_none_map( field : &FormerField< '_ > ) -> proc_macro2::TokenStream
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
/// ### Basic use-case. of output
///
/// ```compile_fail
/// pub int_1 : core::option::Option< i32 >,
/// pub string_1 : core::option::Option< String >,
/// pub int_optional_1 :  core::option::Option< i32 >,
/// pub string_optional_1 : core::option::Option< String >,
/// ```
///

#[inline]
fn field_optional_map( field : &FormerField< '_ > ) -> proc_macro2::TokenStream
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
/// ### Basic use-case. of output
///
/// ```compile_fail
/// let int_1 = if self.int_1.is_some()
/// {
///   self.int_1.take().unwrap()
/// }
/// else
/// {
///   let val : i32 = Default::default();
///   val
/// };
/// ```
///

#[inline]
fn field_form_map( field : &FormerField< '_ > ) -> Result< proc_macro2::TokenStream >
{
  let ident = field.ident;
  let ty = field.ty;
  let default = field.attrs.default.as_ref()
  .map( | attr_default | &attr_default.expr );

  let tokens = if field.is_optional
  {

    let _else = if default == None
    {
      qt!
      {
        ::core::option::Option::None
      }
    }
    else
    {
      let default_val = default.unwrap();
      qt!
      {
        ::core::option::Option::Some( ( #default_val ).into() )
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

    let _else = if default == None
    {
      let panic_msg = format!( "Field '{}' isn't initialized", ident );
      qt!
      {
        let val : #ty =
        {
          // Autoref specialization
          trait NotDefault< T >
          {
            fn maybe_default( self : &Self ) -> T { panic!( #panic_msg ) }
          }

          trait WithDefault< T >
          {
            fn maybe_default( self : &Self ) -> T;
          }

          impl< T > NotDefault< T >
          for & ::core::marker::PhantomData< T >
          {}

          impl< T > WithDefault< T >
          for ::core::marker::PhantomData< T >
          where T : ::core::default::Default,
          {
            fn maybe_default( self : &Self ) -> T
            {
              T::default()
            }
          }

          ( &::core::marker::PhantomData::< #ty > ).maybe_default()
        };
      }
    }
    else
    {
      let default_val = default.unwrap();
      qt!
      {
        let val : #ty = ( #default_val ).into();
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
        val
      };
    }

  };

  Ok( tokens )
}

///
/// Extract name of a field out.
///

#[inline]
fn field_name_map( field : &FormerField< '_ > ) -> syn::Ident
{
  field.ident.clone()
}

///
/// Generate a former setter for the field.
///
/// ### Basic use-case. of output
///
/// ```compile_fail
/// pub fn int_1< Src >( mut self, src : Src ) -> Self
/// where Src : Into< i32 >,
/// {
///   debug_assert!( self.int_1.is_none() );
///   self.int_1 = Some( src.into() );
///   self
/// }
/// ```
///

#[ inline ]
fn field_setter_map( field : &FormerField< '_ > ) -> Result< proc_macro2::TokenStream >
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

  if let Some( alias_attr ) = &field.attrs.alias
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
  }

}

///
/// Generate a setter for the 'field_ident' with the 'setter_name' name.
///

#[ inline ]
fn field_setter
(
  field_ident : &syn::Ident,
  setter_name : &syn::Ident,
  non_optional_type : &syn::Type,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ doc = "Setter for the '#field_ident' field." ]
    #[ inline ]
    pub fn #setter_name< Src >( mut self, src : Src ) -> Self
    where Src : ::core::convert::Into< #non_optional_type >,
    {
      debug_assert!( self.#field_ident.is_none() );
      self.#field_ident = ::core::option::Option::Some( src.into() );
      self
    }
  }
}

///
/// Generate a sub-former setter for the 'field_ident' with the 'setter_name' name.
///

#[ inline ]
fn subformer_field_setter
(
  field_ident : &syn::Ident,
  setter_name : &syn::Ident,
  non_optional_type : &syn::Type,
  subformer_type : &syn::Type,
)
-> proc_macro2::TokenStream
{
  let doc = format!
  (
    "Subformer setter for the '{}' field.",
    field_ident
  );

  // tree_print!( non_optional_type );
  // code_print!( non_optional_type );
  let params = type_parameters( &non_optional_type, .. );
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
      impl Fn( #non_optional_type, Self ) -> Self,
    >
    {
      let container = self.#setter_name.take();
      let on_end = | container : #non_optional_type, mut former : Self | -> Self
      {
        former.#setter_name = Some( container );
        former
      };
      #subformer_type::begin( Some( self ), container, on_end )
    }
  }

  // pub fn hashmap_strings_1( mut self ) -> former::runtime::HashMapSubformer
  // <
  //   String,
  //   String,
  //   std::collections::HashMap< String, String >,
  //   Struct1Former,
  //   impl Fn( std::collections::HashMap< String, String >, Self ) -> Self
  // >
  // {
  //   let container = self.hashmap_strings_1.take();
  //   let on_end = | container : std::collections::HashMap< String, String >, mut former : Self | -> Self
  //   {
  //     former.hashmap_strings_1 = Some( container );
  //     former
  //   };
  //   former::runtime::HashMapSubformer::begin( self, container, on_end )
  // }

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
#[derive( Former )]
pub struct Struct1
{
  #[default( 31 )]
  field1 : i32,
}
"#;

  let doc_former_struct = format!
  (
r#" Object to form [{}]. If field's values is not set then default value of the field is set.

For specifing custom default value use attribute `default`. For example:
```
{}
```
"#,
    name_ident, doc_example1
  );

  ( doc_former_mod, doc_former_struct )
}

//

pub fn former( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let ast = match syn::parse::< syn::DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  let name_ident = &ast.ident;
  let generics = &ast.generics;
  let former_name = format!( "{}Former", name_ident );
  let former_name_ident = syn::Ident::new( &former_name, name_ident.span() );

  // use heck::ToSnakeCase;
  // let former_snake = name_ident.to_string().to_snake_case();
  // let former_mod = format!( "{}_former", former_snake );
  // let former_mod_ident = syn::Ident::new( &former_mod, name_ident.span() );

  /* structure attribute */

  let mut perform = qt!
  {
    return result;
  };
  let mut perform_output = qt!{ #name_ident #generics };
  let mut perform_generics = qt!{};
  for attr in ast.attrs.iter()
  {
    if let Some( ident ) = attr.path.get_ident()
    {
      let ident_string = format!( "{}", ident );
      if ident_string == "perform"
      {
        let attr_perform = syn::parse2::< AttributeFormAfter >( attr.tokens.clone() )?;
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
      }
    }
    else
    {
      return Err( syn_err!( "Unknown structure attribute:\n{}", qt!{ attr } ) );
    }
  }

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
    let type_container_kind = macro_tools::type_optional_container_kind( ty ).0;
    let non_optional_ty : &syn::Type = if is_optional { parameter_internal_first( ty )? } else { ty };
    let former_field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_optional, type_container_kind };
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

    impl #generics #name_ident #generics
    {
      ///
      /// Make former, variation of builder pattern to form structure defining values of fields step by step.
      ///
      #[inline]
      pub fn former() -> #former_name_ident #generics
      {
        #former_name_ident
        {
          #( #fields_none, )*
        }
      }
    }

    #[ doc = #doc_former_struct ]
    #[ automatically_derived ]
    pub struct #former_name_ident #generics
    {
      #(
        /// A field
        #fields_optional,
      )*
    }

    impl #generics #former_name_ident #generics
    {
      ///
      /// Finish setting options and call perform on formed entity.
      ///
      /// If `perform` defined then associated method is called and its result returned instead of entity.
      /// For example `perform()` of structure with : `#[ perform( fn after1< 'a >() -> Option< &'a str > )` returns `Option< &'a str >`.
      ///
      #[inline]
      pub fn perform #perform_generics ( self ) -> #perform_output
      {
        let result = self.form();
        #perform
      }

      ///
      /// Finish setting options and return formed entity.
      ///
      /// `perform` has no effect on method `form`, but change behavior and returned type of mehod `perform`.
      ///
      #[inline]
      pub fn form( mut self ) -> #name_ident #generics
      {
        #( #fields_form )*
        let result = #name_ident
        {
          #( #fields_names, )*
        };
        return result;
      }

      #(
        #fields_setter
      )*

    }

  };

  Ok( result )
}
