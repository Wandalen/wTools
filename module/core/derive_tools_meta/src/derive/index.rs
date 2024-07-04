use super::*;
use macro_tools::
{
  attr, 
  diag, 
  generic_params, 
  struct_like::StructLike, 
  Result
};


/// Generates [Index](core::ops::Index) trait implementation.
pub fn index( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where ) 
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed 
  {
    StructLike::Struct( ref item ) => 
    generate_struct
      ( 
        item_name, 
        &generics_impl, 
        &generics_ty, 
        &generics_where, 
        &item.fields 
      ),
    StructLike::Enum( ref item ) =>
      generate_enum
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &item.variants,
      ),
    StructLike::Unit( ref item ) => Err( 
      syn::Error::new(
        item.fields.span(),
        "cannot infer type: Empty type cannot be indexed",
      ) 
    ),
  }?;

  if has_debug 
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// An aggregator function to generate `Index` implementation for tuple and named structs 
fn generate_struct
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &syn::Fields,
) 
-> Result< proc_macro2::TokenStream > 
{
  match fields 
  {
    syn::Fields::Named( fields ) => 
    generate_struct_named_fields
    (
      item_name, 
      generics_impl, 
      generics_ty, 
      generics_where, 
      fields
    ),
    
    syn::Fields::Unnamed( fields ) => 
    generate_struct_tuple_fields
    (
      item_name, 
      generics_impl, 
      generics_ty, 
      generics_where, 
      fields
    ),
  
    syn::Fields::Unit => Err( 
      syn::Error::new(
        fields.span(),
        "cannot infer type: Empty type cannot be indexed",
      ) 
    ),
  }
}



/// Generates `Index` implementation for structs with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Index;
/// #[ derive( Index ) ]
/// pub struct Struct< T >( T );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct< T >( T );
/// #[ automatically_derived ]
/// impl< T > ::core::ops::Index< usize > for Struct< T >
/// {
///   type Output = T;
///   #[ inline( always ) ]
///   fn index( &self, index: usize ) -> &Self::Output
///   {
///     match index 
///     {
///       0 => &self.0,
///       _ => panic!( "Index out of bounds" ),
///     }
///   }
/// }
/// ```
///

fn generate_struct_tuple_fields
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &syn::FieldsUnnamed,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = fields.unnamed.clone();

  // Generate match arms for each field
  let match_arms = fields.iter().enumerate().map(|( index, _field )| 
    {
      let index = syn::Index::from( index );
      qt! 
      {
        #index => &self.#index
      }
    }
  );

  Ok
  (
    qt! 
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = T;
        #[ inline( always ) ]
        fn index( &self, index: usize ) -> &Self::Output
        {
          match index 
          {
            #(#match_arms,)*
            _ => panic!( "Index out of bounds" ),
          }
        }
      }
    }
  )
}

/// Generates `Index` implementation for structs with named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Index;
/// #[ derive( Index ) ]
/// pub struct Struct< T > 
/// {
///   a: T,    
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct< T >
/// {
///   a: T,
/// };
///
/// #[ automatically_derived ]
/// impl< T > ::core::ops::Index< usize > for Struct< T >
/// {
///   type Output = T;
///   #[ inline( always ) ]
///   fn index( &self, index: usize ) -> &Self::Output
///   {
///   match index 
///     {
///        0 => &self.a,
///        _ => panic!( "Index out of bounds" ),
///     }
///   }
/// }
/// ```
///

fn generate_struct_named_fields
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &syn::FieldsNamed,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = fields.named.clone();

  // Generate match arms for each field
  let match_arms = fields.iter().enumerate().map(|( index, field )| 
    {
      let index = syn::Index::from( index );
      let field_name = &field.ident;
      qt! 
      {
        #index => &self.#field_name
      }
    }
  );

  Ok
  (
    qt! 
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = T;
        #[ inline( always ) ]
        fn index( &self, index: usize ) -> &Self::Output
        {
          match index 
          {
            #(#match_arms,)*
            _ => panic!( "Index out of bounds" ),
          }
        }
      }
    }
  )
}



/// An aggregator function to generate `Index` implementation for Enum
fn generate_enum
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variants : &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = match variants.first()
  {
    Some( variant ) => &variant.fields,
    None => return Err( 
      syn::Error::new(
        generics_ty.span(),
        "cannot infer type: Empty type cannot be indexed",
      ) 
    ),
  };

  let idents = variants.iter().map( | v | v.ident.clone() ).collect::< Vec< _ > >();


  match fields 
  {
    syn::Fields::Named( ref item) => 
    generate_enum_named_fields
    (
      item_name, 
      generics_impl, 
      generics_ty, 
      generics_where, 
      &idents,
      item
    ),
    syn::Fields::Unnamed( ref item ) => 
    generate_enum_tuple_fields
    (
      item_name, 
      generics_impl, 
      generics_ty, 
      generics_where, 
      &idents,
      item
    ),  
    syn::Fields::Unit => Err( 
      syn::Error::new(
        fields.span(),
        "cannot infer type: Empty type cannot be indexed",
      ) 
    ),
  }
}


/// Generates `Index` implementation for enums with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Index;
/// #[ derive( Index ) ]
/// pub enum EnumTuple< T > 
/// {
///   A( T ),
///   B( T ),    
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub enum EnumTuple< T > 
/// {
///   A( T ),
///   B( T ),    
/// }
///
/// #[ automatically_derived ]
/// impl< T > ::core::ops::Index< usize > for EnumTuple< T >
/// {
///   type Output = T;
///   #[ inline( always ) ]
///   fn index( &self, index: usize ) -> &Self::Output
///   {
///   match index 
///     {
///       0 => match self
///       {
///         EnumTuple::A( a ) | EnumTuple::B( a ) => a, 
///       },
///       _ => panic!( "Index out of bounds" ),
///     }
///   }
/// }
/// ```
///

fn generate_enum_tuple_fields
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant_idents : &[ syn::Ident ],
  fields: &syn::FieldsUnnamed,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = fields.unnamed.clone();

  // Generate match arms for each field
  let match_arms = fields.iter().enumerate().map(|( index, _field )| 
    {
      let index = syn::Index::from( index );
      qt! 
      {
        #index => match self 
        {
          #( #item_name::#variant_idents( v ) )|* => v
        }
      }
    }
  );

  Ok
  (
    qt! 
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = T;
        #[ inline( always ) ]
        fn index( &self, index: usize ) -> &Self::Output
        {
          match index 
          {
            #(#match_arms)*
            _ => panic!( "Index out of bounds" ),
          }
        }
      }
    }
  )
}





/// Generates `Index` implementation for enums with named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Index;
/// #[ derive( Index ) ]
/// pub enum EnumNamed< T > 
/// {
///   A { a: T, b: T },
///   B { a: T, b: T },    
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub enum EnumNamed< T > 
/// {
///   A { a: T, b: T },
///   B { a: T, b: T },    
/// }
///
/// #[ automatically_derived ]
/// impl< T > ::core::ops::Index< usize > for EnumNamed< T >
/// {
///   type Output = T;
///   #[ inline( always ) ]
///   fn index( &self, index: usize ) -> &Self::Output
///   {
///       match index 
///       {
///         0 => match self 
///         {
///            EnumNamed::A { a, .. } | EnumNamed::B { a, .. } => a,
///         },
///         1 => match self 
//          {
///            EnumNamed::A { b, .. } | EnumNamed::B { b, .. } => b,
///         },
///         _ => panic!( "Index out of bounds" ),
///     }
///   }
/// }
/// ```
///
fn generate_enum_named_fields
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant_idents : &[ syn::Ident ],
  fields: &syn::FieldsNamed,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = fields.named.clone();

  // Generate match arms for each field
  let match_arms = fields.iter().enumerate().map(|( index, field )| 
    {
      let index = syn::Index::from( index );
      let field_name = &field.ident;
      qt! 
      {
        #index => match self 
        {
          #( #item_name::#variant_idents { #field_name: v, .. } )|* => v,   
        }
      }
    }
  );

  Ok
  (
    qt! 
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = T;
        #[ inline( always ) ]
        fn index( &self, index: usize ) -> &Self::Output
        {
          match index 
          {
            #(#match_arms)*
            _ => panic!( "Index out of bounds" ),
          }
        }
      }
    }
  )
}

  
