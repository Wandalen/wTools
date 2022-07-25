
use iter_tools::{ Itertools, process_results };
use proc_macro_tools::*;

pub type Result< T > = std::result::Result< T, syn::Error >;

///
/// Descripotr of a field.
///

#[allow( dead_code )]
struct FormerField< 'a >
{
  pub attrs : &'a Vec< syn::Attribute >,
  pub vis : &'a syn::Visibility,
  pub ident : &'a syn::Ident,
  pub colon_token : &'a Option< syn::token::Colon >,
  pub ty : &'a syn::Type,
  pub non_optional_ty : &'a syn::Type,
  pub is_optional : bool,
  pub type_container_kind : proc_macro_tools::ContainerKind,
}

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform = ( fn after1< 'a >() -> Option< &'a str > ) ]`
///

#[allow( dead_code )]
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

#[allow( dead_code )]
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

///
/// Is type under Option.
///

fn is_optional( ty : &syn::Type ) -> bool
{
  proc_macro_tools::type_rightmost( ty ) == Some( "Option".to_string() )
}

///
/// Extract the first parameter of the type if such exist.
///

fn parameter_internal_first( ty : &syn::Type ) -> Result< &syn::Type >
{
  proc_macro_tools::type_parameters( ty, 0 ..= 0 )
  .first()
  .map( | e | *e )
  .ok_or_else( || syn_err!( ty, "Expects at least one parameter here:\n  {}", qt!{ #ty } ) )
}

///
/// Extract the first and the second parameters of the type if such exist.
///

fn parameter_internal_first_two( ty : &syn::Type ) -> Result< ( &syn::Type, &syn::Type ) >
{
  let on_err = ||
  {
    syn_err!( ty, "Expects at least two parameters here:\n  {}", qt!{ #ty } )
  };
  let result = proc_macro_tools::type_parameters( ty, 0 ..= 1 );
  let mut iter = result.iter();
  Ok
  ((
    iter.next().ok_or_else( on_err )?,
    iter.next().ok_or_else( on_err )?,
  ),)
}

///
/// Generate fields for initializer of a struct setting each field to `None`.
///
/// ### Sample of output
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
/// ### Sample of output
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

  let ty2 = if is_optional( &ty )
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
/// ### Sample of output
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
  let mut default = None;

  for attr in field.attrs.iter()
  {
    let key_ident = attr.path.get_ident()
    .ok_or_else( || syn_err!( attr, "Expects simple key of an attirbute, but got:\n  {}", qt!{ #attr } ) )?;
    let key_str = format!( "{}", key_ident );
    match key_str.as_ref()
    {
      "default" =>
      {
        let attr_default = syn::parse2::< AttributeDefault >( attr.tokens.clone() )?;
        default = Some( attr_default.expr );
      }
      _ =>
      {
        return Err( syn_err!( attr, "Unknown attribute {}", qt!{ #attr } ) );
      }
    }
  }

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
      qt!
      {
        let val : #ty = ::core::default::Default::default();
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
  let ident = field.ident.clone();
  ident
}

///
/// Generate a fomer setter for the field.
///
/// ### Sample of output
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

#[inline]
fn field_setter_map( field : &FormerField< '_ >, former_name_ident : &syn::Ident ) -> Result< proc_macro2::TokenStream >
{
  let ident = &field.ident;

  let tokens = match &field.type_container_kind
  {
    proc_macro_tools::ContainerKind::No =>
    {
      let non_optional_ty = &field.non_optional_ty;
      qt!
      {
        #[inline]
        pub fn #ident< Src >( mut self, src : Src ) -> Self
        where Src : ::core::convert::Into< #non_optional_ty >,
        {
          debug_assert!( self.#ident.is_none() );
          self.#ident = ::core::option::Option::Some( src.into() );
          self
        }
      }
    },
    proc_macro_tools::ContainerKind::Vector =>
    {
      let ty = &field.ty;
      let internal_ty = parameter_internal_first( ty )?;
      qt!
      {
        #[inline]
        pub fn #ident( mut self ) -> former::runtime::VectorFormer
        <
          #internal_ty,
          #ty,
          #former_name_ident,
          impl Fn( &mut #former_name_ident, ::core::option::Option< #ty > )
        >
        {
          let container = self.#ident.take();
          let on_end = | former : &mut #former_name_ident, container : ::core::option::Option< #ty > |
          {
            former.#ident = container;
          };
          former::runtime::VectorFormer::new( self, container, on_end )
        }
      }
    },
    proc_macro_tools::ContainerKind::HashMap =>
    {
      let ty = &field.ty;
      let ( k_ty, e_ty ) = parameter_internal_first_two( ty )?;
      qt!
      {
        #[inline]
        pub fn #ident( mut self ) -> former::runtime::HashMapFormer
        <
          #k_ty,
          #e_ty,
          #ty,
          #former_name_ident,
          impl Fn( &mut #former_name_ident, ::core::option::Option< #ty > )
        >
        {
          let container = self.#ident.take();
          let on_end = | former : &mut #former_name_ident, container : ::core::option::Option< #ty > |
          {
            former.#ident = container;
          };
          former::runtime::HashMapFormer::new( self, container, on_end )
        }
      }
    },
    proc_macro_tools::ContainerKind::HashSet =>
    {
      let ty = &field.ty;
      let internal_ty = parameter_internal_first( ty )?;
      qt!
      {
        #[inline]
        pub fn #ident( mut self ) -> former::runtime::HashSetFormer
        <
          #internal_ty,
          #ty,
          #former_name_ident,
          impl Fn( &mut #former_name_ident, ::core::option::Option< #ty > )
        >
        {
          let container = self.#ident.take();
          let on_end = | former : &mut #former_name_ident, container : ::core::option::Option< #ty > |
          {
            former.#ident = container;
          };
          former::runtime::HashSetFormer::new( self, container, on_end )
        }
      }
    },
  };

  Ok( tokens )
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
        match output
        {
          syn::ReturnType::Type( _, boxed_type ) =>
          {
            perform_output = qt!{ #boxed_type };
          },
          _ => {},
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
    let attrs = &field.attrs;
    let vis = &field.vis;
    let ident = field.ident.as_ref()
    .ok_or_else( || syn_err!( field, "Expected that each field has key, but some does not:\n  {}", qt!{ #field } ) )?;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_optional = is_optional( &ty );
    let type_container_kind = proc_macro_tools::type_container_kind( &ty );
    let non_optional_ty : &syn::Type = if is_optional { parameter_internal_first( ty )? } else { ty };
    let former_field = FormerField { attrs, vis, ident, colon_token, ty, non_optional_ty, is_optional, type_container_kind };
    Ok( former_field )
  }).collect();

  let former_fields : Vec< _ > = process_results( former_fields, | iter | iter.collect() )?;

  let ( fields_none, fields_optional, fields_form, fields_names, fields_setter )
  : ( Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ >, Vec< _ > )
  = former_fields.iter().map( | former_field |
  {(
    field_none_map( &former_field ),
    field_optional_map( &former_field ),
    field_form_map( &former_field ),
    field_name_map( &former_field ),
    field_setter_map( &former_field, &former_name_ident ),
  )}).multiunzip();

  let ( _doc_former_mod, doc_former_struct ) = doc_generate( &name_ident );
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

    // #[doc = #doc_former_mod]
    // mod #former_mod_ident
    // {
      // use;
      // use super::*;
      // use super::#name_ident;
      // #[cfg( feature = "in_wtools" )]
      // use wtools::former;

      #[doc = #doc_former_struct]
      #[derive( Debug )]
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

    // }
    // pub use #former_mod_ident::#former_name_ident;

  };

  Ok( result )
}

//
// = Input :
//
// #[derive( Debug, PartialEq )]
// pub struct Struct1
// {
//   pub int_1 : i32,
//   string_1 : String,
//   int_optional_1 : core::option::Option< i32 >,
//   string_optional_1 : Option< String >,
//   vec_1 : Vec< String >,
//   hashmap_strings_1 : std::collections::HashMap< String, String >,
//   hashset_strings_1 : std::collections::HashSet< String >,
// }

//
// = Output :
//
// impl Struct1
// {
//   pub fn former() -> Struct1Former
//   {
//     Struct1Former
//     {
//       int_1 : core::option::Option::None,
//       string_1 : core::option::Option::None,
//       int_optional_1 : core::option::Option::None,
//       string_optional_1 : core::option::Option::None,
//       vec_1 : core::option::Option::None,
//       hashmap_strings_1 : core::option::Option::None,
//       hashset_strings_1 : core::option::Option::None,
//     }
//   }
// }
//
// //
//
// #[derive( Debug )]
// pub struct Struct1Former
// {
//   pub int_1 : core::option::Option< i32 >,
//   pub string_1 : core::option::Option< String >,
//   pub int_optional_1 :  core::option::Option< i32 >,
//   pub string_optional_1 : core::option::Option< String >,
//   pub vec_1 : core::option::Option< Vec< String > >,
//   pub hashmap_strings_1 : core::option::Option< std::collections::HashMap< String, String > >,
//   pub hashset_strings_1 : core::option::Option< std::collections::HashSet< String > >,
// }
//
// //
//
// impl Struct1Former
// {
//   fn form( mut self ) -> Struct1
//   {
//
//     let int_1 = if self.int_1.is_some()
//     {
//       self.int_1.take().unwrap()
//     }
//     else
//     {
//       let val : i32 = Default::default();
//       val
//     };
//
//     let string_1 = if self.string_1.is_some()
//     {
//       self.string_1.take().unwrap()
//     }
//     else
//     {
//       let val : String = Default::default();
//       val
//     };
//
//     let int_optional_1 = if self.int_optional_1.is_some()
//     {
//       Some( self.int_optional_1.take().unwrap() )
//     }
//     else
//     {
//       None
//     };
//
//     let string_optional_1 = if self.string_optional_1.is_some()
//     {
//       Some( self.string_optional_1.take().unwrap() )
//     }
//     else
//     {
//       None
//     };
//
//     let vec_1 = if self.vec_1.is_some()
//     {
//       self.vec_1.take().unwrap()
//     }
//     else
//     {
//       let val : Vec< String > = Default::default();
//       val
//     };
//
//     let hashmap_strings_1 = if self.hashmap_strings_1.is_some()
//     {
//       self.hashmap_strings_1.take().unwrap()
//     }
//     else
//     {
//       let val : std::collections::HashMap< String, String > = Default::default();
//       val
//     };
//
//     let hashset_strings_1 = if self.hashset_strings_1.is_some()
//     {
//       self.hashset_strings_1.take().unwrap()
//     }
//     else
//     {
//       let val : std::collections::HashSet< String > = Default::default();
//       val
//     };
//
//     Struct1
//     {
//       int_1,
//       string_1,
//       int_optional_1,
//       string_optional_1,
//       vec_1,
//       hashmap_strings_1,
//       hashset_strings_1,
//     }
//
//   }
//
//   pub fn int_1< Src >( mut self, src : Src ) -> Self
//   where Src : core::convert::Into< i32 >,
//   {
//     debug_assert!( self.int_1.is_none() );
//     self.int_1 = Some( src.into() );
//     self
//   }
//
//   pub fn string_1< Src >( mut self, src : Src ) -> Self
//   where Src : core::convert::Into< String >,
//   {
//     debug_assert!( self.string_1.is_none() );
//     self.string_1 = Some( src.into() );
//     self
//   }
//
//   pub fn string_optional_1< Src >( mut self, src : Src ) -> Self
//   where Src : core::convert::Into< String >
//   {
//     debug_assert!( self.string_optional_1.is_none() );
//     self.string_optional_1 = Some( src.into() );
//     self
//   }
//
//   pub fn vec_1( mut self ) -> former::runtime::VectorFormer
//   <
//     String,
//     Vec< String >,
//     Struct1Former,
//     impl Fn( &mut Struct1Former, core::option::Option< Vec< String > > )
//   >
//   {
//     let container = self.vec_1.take();
//     let on_end = | former : &mut Struct1Former, container : core::option::Option< Vec< String > > |
//     {
//       former.vec_1 = container;
//     };
//     former::runtime::VectorFormer::new( self, container, on_end )
//   }
//
//   pub fn hashmap_strings_1( mut self ) -> former::runtime::HashMapFormer
//   <
//     String,
//     String,
//     std::collections::HashMap< String, String >,
//     Struct1Former,
//     impl Fn( &mut Struct1Former, core::option::Option< std::collections::HashMap< String, String > > )
//   >
//   {
//     let container = self.hashmap_strings_1.take();
//     let on_end = | former : &mut Struct1Former, container : core::option::Option< std::collections::HashMap< String, String > > |
//     {
//       former.hashmap_strings_1 = container;
//     };
//     former::runtime::HashMapFormer::new( self, container, on_end )
//   }
//
//   pub fn hashset_strings_1( mut self ) -> former::runtime::HashSetFormer
//   <
//     String,
//     std::collections::HashSet< String >,
//     Struct1Former,
//     impl Fn( &mut Struct1Former, core::option::Option< std::collections::HashSet< String > > )
//   >
//   {
//     let container = self.hashset_strings_1.take();
//     let on_end = | former : &mut Struct1Former, container : core::option::Option< std::collections::HashSet< String > > |
//     {
//       former.hashset_strings_1 = container;
//     };
//     former::runtime::HashSetFormer::new( self, container, on_end )
//   }
//
// }
//
