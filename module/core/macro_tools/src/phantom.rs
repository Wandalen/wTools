//!
//! Responsible for generating marker `PhantomData` fields to avoid the rule requiring the usage of all generic parameters in a struct. This is often necessary to ensure that Rust's type system correctly tracks the ownership and lifetimes of these parameters without needing them to be explicitly used in the struct's fields.
//!
//! Functions and structures to handle and manipulate `PhantomData` fields in structs using the `syn` crate. These utilities ensure that generic parameters are correctly accounted for in type checking, even if they are not directly used in the struct's fields.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Adds a `PhantomData` field to a struct to manage generic parameter usage.
  ///
  /// This function clones a given `syn::ItemStruct`, calculates the appropriate `PhantomData` usage
  /// based on the struct's generic parameters, and adds a corresponding `PhantomData` field. This field
  /// helps in handling ownership and lifetime indications for generic parameters, ensuring that they
  /// are correctly accounted for in type checking, even if they are not directly used in the struct's
  /// fields.
  ///
  /// # Parameters
  /// - `input`: A reference to the `syn::ItemStruct` which describes the structure to which the
  ///   `PhantomData` field will be added.
  ///
  /// # Returns
  /// Returns a new `syn::ItemStruct` with the `PhantomData` field added to its list of fields.
  ///
  /// # Examples
  /// ```rust
  /// use syn::{ parse_quote, ItemStruct };
  ///
  /// let input_struct : ItemStruct = parse_quote!
  /// {
  ///   pub struct MyStruct< T, U >
  ///   {
  ///     data : T,
  ///   }
  /// };
  ///
  /// let modified_struct = macro_tools::phantom::add_to_item( &input_struct );
  /// println!( "{:#?}", modified_struct );
  ///
  /// // Output will include a _phantom field of type `PhantomData< ( T, U ) >`
  /// ```
  ///

  pub fn add_to_item( input : &syn::ItemStruct ) -> syn::ItemStruct
  {

    // Only proceed if there are generics
    if input.generics.params.is_empty()
    {
      return item::ensure_comma( input );
    }

    // Clone the input struct to work on a modifiable copy
    let mut input = input.clone();

    // Prepare the tuple type for PhantomData based on the struct's generics
    let phantom = tuple( &input.generics.params );

    // Handle different field types: Named, Unnamed, or Unit
    match &mut input.fields
    {
      syn::Fields::Named( fields ) =>
      {
        let phantom_field : syn::Field = syn::parse_quote!
        {
          _phantom : #phantom
        };

        // Ensure there is a trailing comma if fields are already present
        if !fields.named.empty_or_trailing()
        {
          fields.named.push_punct( Default::default() );
        }
        fields.named.push( phantom_field );
        fields.named.push_punct( Default::default() ); // Add trailing comma after adding PhantomData
      },
      syn::Fields::Unnamed( fields ) =>
      {
        let phantom_field : syn::Field = syn::parse_quote!
        {
          #phantom
        };

        // Ensure there is a trailing comma if fields are already present
        if !fields.unnamed.empty_or_trailing()
        {
          fields.unnamed.push_punct( Default::default() );
        }
        fields.unnamed.push_value( phantom_field );
        fields.unnamed.push_punct( Default::default() ); // Ensure to add the trailing comma after PhantomData
      },
      syn::Fields::Unit =>
      {
        let phantom_field : syn::Field = syn::parse_quote!
        {
          #phantom
        };

        // Replace syn::Fields::Unit to syn::Fields::Unnamed
        input.fields = syn::Fields::Unnamed
          (
            syn::FieldsUnnamed
            {
              paren_token : Default::default(),
              unnamed : syn::punctuated::Punctuated::from_iter( vec![phantom_field] )
            }
          )
      }
    };

    input
  }

  /// Constructs a `PhantomData` type tuple from the generic parameters of a struct.
  ///
  /// This function generates a tuple type for `PhantomData` using the given generic parameters,
  /// which includes types, lifetimes, and const generics. It ensures that the generated tuple
  /// use all parameters.
  ///
  /// # Parameters
  /// - `input`: A reference to a `Punctuated< GenericParam,  Comma>` containing the generic parameters.
  ///
  /// # Returns
  /// Returns a `syn::Type` that represents a `PhantomData` tuple incorporating all the generic parameters.
  ///
  /// # Examples
  /// ```rust
  /// use syn::{parse_quote, punctuated::Punctuated, GenericParam, token::Comma};
  /// use macro_tools::phantom::tuple;
  ///
  /// let generics: Punctuated< GenericParam, Comma > = parse_quote! { 'a, T, const N : usize };
  /// let phantom_type = tuple( &generics );
  /// println!( "{}", quote::quote! { #phantom_type } );
  /// // Output : ::core::marker::PhantomData< ( &'a (), *const T, N ) >
  /// ```
  ///
  pub fn tuple( input : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma > ) -> syn::Type
  {
    use proc_macro2::Span;
    use syn::{ GenericParam, Type };

    // Prepare the tuple type for PhantomData based on the struct's generics
    let generics_tuple_type =
    {
      let generics_list = input.iter().map( | param |
      {
        match param
        {
          GenericParam::Type( type_param ) =>
          {
            let path = &type_param.ident;
            let path2 : syn::Type = parse_quote!{ *const #path };
            path2
          },
          GenericParam::Lifetime( lifetime_param ) => Type::Reference( syn::TypeReference
          {
            and_token : Default::default(),
            lifetime : Some( lifetime_param.lifetime.clone() ),
            mutability : None,
            elem : Box::new( Type::Tuple( syn::TypeTuple
            {
              paren_token : syn::token::Paren( Span::call_site() ),
              elems : syn::punctuated::Punctuated::new(),
            })),
          }),
          GenericParam::Const( const_param ) => Type::Path( syn::TypePath
          {
            qself : None,
            path : const_param.ident.clone().into(),
          }),
        }
      }).collect::< syn::punctuated::Punctuated< _, syn::token::Comma > >();

      Type::Tuple( syn::TypeTuple
      {
        paren_token : syn::token::Paren( Span::call_site() ),
        elems : generics_list,
      })
    };

    let result : syn::Type = syn::parse_quote!
    {
      ::core::marker::PhantomData< #generics_tuple_type >
    };

    result
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

#[ allow( unused_imports ) ]
pub mod protected
{

  //!
  //! Responsible for generating marker `PhantomData` fields to avoid the rule requiring the usage of all generic parameters in a struct. This is often necessary to ensure that Rust's type system correctly tracks the ownership and lifetimes of these parameters without needing them to be explicitly used in the struct's fields.
  //!
  //! Functions and structures to handle and manipulate `PhantomData` fields in structs using the `syn` crate. These utilities ensure that generic parameters are correctly accounted for in type checking, even if they are not directly used in the struct's fields.
  //!

  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    add_to_item,
    tuple,
  };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  pub use super::super::phantom;
  // pub use super::protected as phantom;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
}
