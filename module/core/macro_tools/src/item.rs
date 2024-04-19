//! xxx : update documentation of file

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  /// Ensures the last field in a struct has a trailing comma.
  ///
  /// This function checks and modifies the fields of a given struct, `input`, ensuring that the last field, whether in
  /// named or unnamed structs, ends with a trailing comma. This adjustment is commonly needed in macro-generated
  /// code to maintain consistency and syntactical correctness across different struct types, including unit structs
  /// which are unaffected as they do not contain fields.
  ///
  /// # Arguments
  ///
  /// * `input` - A reference to the struct (`syn::ItemStruct`) whose fields are to be checked and modified.
  ///
  /// # Returns
  ///
  /// Returns a modified clone of the input struct (`syn::ItemStruct`) where the last field in named or unnamed
  /// structs has a trailing comma. Unit structs remain unchanged as they do not contain fields.
  ///
  /// # Examples
  ///
  /// ```
  /// use macro_tools::
  /// {
  ///   syn::{ parse_quote, ItemStruct },
  ///   quote::quote,
  /// };
  ///
  /// // Create a struct using `parse_quote!` macro
  /// let input_struct : ItemStruct = parse_quote!
  /// {
  ///   struct Example
  ///   {
  ///     field1 : i32,
  ///     field2 : String
  ///   }
  /// };
  ///
  /// // Apply `ensure_comma` to ensure the last field has a trailing comma
  /// let modified_struct = macro_tools::item::ensure_comma( &input_struct );
  ///
  /// // Now `modified_struct` will have a trailing comma after `field2`
  /// assert_eq!( quote!( #modified_struct ).to_string(), quote!
  /// {
  ///   struct Example
  ///   {
  ///     field1 : i32,
  ///     field2 : String,
  ///   }
  /// }.to_string() );
  /// ```

  pub fn ensure_comma( input : &syn::ItemStruct ) -> syn::ItemStruct
  {
    let mut new_input = input.clone(); // Clone the input to modify it

    match &mut new_input.fields
    {
      // Handle named fields
      syn::Fields::Named( syn::FieldsNamed { named, .. } ) =>
      {
        punctuated::ensure_trailing_comma( named )
      },
      // Handle unnamed fields (tuples)
      syn::Fields::Unnamed( syn::FieldsUnnamed { unnamed, .. } ) =>
      {
        punctuated::ensure_trailing_comma( unnamed )
      },
      // Do nothing for unit structs
      syn::Fields::Unit => {}
    }

    new_input
  }

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
  /// use macro_tools::item::phantom_add;
  ///
  /// let input_struct: ItemStruct = parse_quote!
  /// {
  ///   pub struct MyStruct< T, U >
  ///   {
  ///     data : T,
  ///   }
  /// };
  ///
  /// let modified_struct = phantom_add(&input_struct);
  /// println!( "{:#?}", modified_struct );
  ///
  /// // Output will include a _phantom field of type `PhantomData<(T, U)>`
  /// ```
  ///

  pub fn phantom_add( input : &syn::ItemStruct ) -> syn::ItemStruct
  {
    use proc_macro2::Span;
    use syn::{ GenericParam, Type };

    // Only proceed if there are generics
    if input.generics.params.is_empty()
    {
      return ensure_comma( input );
    }

    // Clone the input struct to work on a modifiable copy
    let mut input = input.clone();

    // Prepare the tuple type for PhantomData based on the struct's generics
    let generics_tuple_type =
    {
      let generics_list = input.generics.params.iter().map( | param |
      {
        match param
        {
          GenericParam::Type( type_param ) => Type::Path( syn::TypePath
          {
            qself : None,
            path : type_param.ident.clone().into(),
          }),
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
      }).collect::<syn::punctuated::Punctuated< _, syn::token::Comma>>();

      Type::Tuple( syn::TypeTuple
      {
        paren_token : syn::token::Paren( Span::call_site() ),
        elems : generics_list,
      })
    };

    // Handle different field types: Named, Unnamed, or Unit
    match &mut input.fields
    {
      syn::Fields::Named( fields ) =>
      {
        let phantom_field : syn::Field = syn::parse_quote!
        {
          _phantom : core::marker::PhantomData< #generics_tuple_type >
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
          core::marker::PhantomData< #generics_tuple_type >
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
        // No fields to modify in a unit struct
      }
    };

    input
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    ensure_comma,
    phantom_add,
  };
}

// xxx : external attr instead of internal?
/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as item;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
