
use super::*;
use macro_tools::{ attr, diag, generic_params, struct_like::StructLike, Result };

//

pub fn inner_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( _ ) =>
    {
      generate_unit
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
      )
    }
    StructLike::Struct( ref item ) =>
    {
      generate_struct
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &item.fields,
      )
    }
    StructLike::Enum( ref item ) =>
    {
      generate_enum
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &item.variants,
      )
    }
  }?;

  if has_debug
  {
    let about = format!( "derive : InnerFrom\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for the unit type regarding the bound type
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub struct Struct;
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct;
/// #[ allow( non_local_definitions ) ]
/// #[ allow( clippy::unused_imports ) ]
/// #[ automatically_derived]
/// impl From< Struct > for ()
/// {
///   #[ inline( always ) ]
///   fn from( src : Struct ) -> ()
///   {
///     ()
///   }
/// }
/// ```
///
fn generate_unit
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      #[ allow( non_local_definitions ) ]
      #[ allow( clippy::unused_imports ) ]
      #[ automatically_derived ]
      impl< #generics_impl > ::core::convert::From< #item_name< #generics_ty > > for ()
      where
        #generics_where
      {
        #[ inline( always ) ]
        // fn from( src : UnitStruct ) -> ()
        fn from( src : #item_name< #generics_ty > ) -> ()
        {
          ()
        }
      }
    }
  )
}

/// An aggregator function to generate `From` implementation for the inner type regarding unit, tuple structs and the ones with named fields
fn generate_struct
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::Fields,
)
-> Result< proc_macro2::TokenStream >
{
  match fields
  {

    syn::Fields::Unit =>
    generate_unit
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
    ),

    syn::Fields::Unnamed( fields ) =>
    generate_struct_tuple_fields
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
      fields,
    ),

    syn::Fields::Named( fields ) =>
    generate_struct_named_fields
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
      fields,
    ),

  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for the contained field types regarding the bounded type
/// Works with tuple structs
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub struct Struct( bool );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct( bool );
/// #[ allow( non_local_definitions ) ]
/// #[ automatically_derived ]
/// impl From< Struct > for bool
/// {
///   #[ inline( always ) ]
///   fn from( src : Struct ) -> Self
///   {
///     src.0
///   }
/// }
/// ```
///
fn generate_struct_tuple_fields
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::FieldsUnnamed,
)
-> Result< proc_macro2::TokenStream >
{
  let field_types = fields.unnamed.iter().map( | field | &field.ty );
  let captures : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
  .map( | index |
  {
    let index : proc_macro2::TokenStream = index.to_string().parse().unwrap();
    qt! { src.#index }
  })
  .collect();
  Ok
  (
    qt!
    {
      #[ allow( non_local_definitions ) ]
      #[ automatically_derived ]
      impl< #generics_impl > ::core::convert::From< #item_name< #generics_ty > > for ( #( #field_types ),* )
      where
        #generics_where
      {
        #[ inline( always ) ]
        // fn from( src : StructWithManyFields ) -> Self
        fn from( src : #item_name< #generics_ty > ) -> Self
        {
          ( #( #captures ), * )
        }
      }
    }
  )
}

// qqq  : document, add example of generated code
/// Generates `From` implementation for the inner type regarding bounded type
/// Works with structs named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub struct Struct
/// {
///   value : bool,
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct
/// {
///   value : bool,
/// }
/// #[ allow( non_local_definitions ) ]
/// #[ automatically_derived ]
/// impl From< Struct > for bool
/// {
///   #[ inline( always ) ]
///   fn from( src : Struct ) -> Self
///   {
///     src.value
///   }
/// }
/// ```
///
fn generate_struct_named_fields
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::FieldsNamed,
)
-> Result< proc_macro2::TokenStream >
{
  let field_names = fields.named.iter().map( | field | field.ident.as_ref().unwrap() );
  let field_types = fields.named.iter().map( | field | &field.ty );
  let params : Vec< proc_macro2::TokenStream > = field_names
  .map( | field_name | qt! { src.#field_name } )
  .collect();
  Ok
  (
    qt!
    {
      #[ allow( non_local_definitions ) ]
      #[ automatically_derived ]
      impl< #generics_impl > ::core::convert::From< #item_name< #generics_ty > > for ( #( #field_types ),* )
      where
        #generics_where
      {
        #[ inline( always ) ]
        // fn from( src : StructWithManyFields ) -> Self
        fn from( src : #item_name< #generics_ty > ) -> Self
        {
          ( #( #params ), * )
        }
      }
    }
  )
}

/// An aggregator function to generate `From` implementation for the inner type regrading unit, tuple enums and the ones with named fields
fn generate_enum
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variants : &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
)
-> Result< proc_macro2::TokenStream >
{
  let fields = match variants.first()
  {
    Some( variant ) => &variant.fields,
    None => return generate_unit
    (
      item_name,
      &generics_impl,
      &generics_ty,
      &generics_where,
    ),
  };

  // error if fields have different types
  if !variants.iter().skip(1).all(|v| &v.fields == fields)
  {
    return Err( syn::Error::new( variants.span(), "Variants must have the same type" ) );
  }

  let idents = variants.iter().map( | v | v.ident.clone() ).collect::< Vec< _ > >();

  match fields
  {

    syn::Fields::Unit =>
    generate_unit
    (
      item_name,
      &generics_impl,
      &generics_ty,
      &generics_where,
    ),

    syn::Fields::Unnamed( ref item ) =>
    generate_enum_tuple_variants
    (
      item_name,
      &generics_impl,
      &generics_ty,
      &generics_where,
      &idents,
      item,
    ),

    syn::Fields::Named( ref item ) =>
    generate_enum_named_variants
    (
      item_name,
      &generics_impl,
      &generics_ty,
      &generics_where,
      &idents,
      item,
    ),

  }
}

/// Generates `From` implementation for the inner types regarding enums with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub enum E
/// {
///   A ( i32, Vec< String > ),
///   B ( i32, Vec< String > ),
///   C ( i32, Vec< String > ),
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub enum E
/// {
///   A ( i32, Vec< String > ),
///   B ( i32, Vec< String > ),
///   C ( i32, Vec< String > ),
/// }
/// #[ automatically_derived ]
/// impl ::core::convert::From< E > for ( i32, Vec< String > )
/// {
///   #[ inline( always ) ]
///   fn from( other : E ) -> Self
///   {
///     match other
///     {
///       E::A( _1, _2 ) | E::B( _1, _2 ) | E::C( _1, _2 ) => (_1, _2),
///     }
///   }
/// }
/// ```
///
fn generate_enum_tuple_variants
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant_idents : &[ syn::Ident ],
  fields : &syn::FieldsUnnamed,
)
-> Result< proc_macro2::TokenStream >
{
  let field_types = fields.unnamed.iter().map( | field | &field.ty );
  let captures : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
  .map( | index |
  {
    let ident = format_ident!("_{}", index);
    qt! { #ident }
  })
  .collect();

  let captures = qt! { #( #captures ),* };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::convert::From< #item_name< #generics_ty > > for ( #( #field_types ),* )
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn from( other : #item_name< #generics_ty > ) -> Self
        {
          match other
          {
            #( #item_name::#variant_idents( #captures ) )|* => ( ( #captures ) )
          }
        }
      }
    }
  )
}

/// Generates `From` implementation for the inner types regarding enums with named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub enum E
/// {
///   A { a : i32, b : Vec< String > },
///   B { a : i32, b : Vec< String > },
///   C { a : i32, b : Vec< String > },
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub enum E
/// {
///   A { a : i32, b : Vec< String > },
///   B { a : i32, b : Vec< String > },
///   C { a : i32, b : Vec< String > },
/// }
/// #[ automatically_derived ]
/// impl ::core::convert::From< E > for ( i32, Vec< String > )
/// {
///   #[ inline( always ) ]
///   fn from( other : E ) -> Self
///   {
///     match other
///     {
///       E::A { a, b } | E::B { a, b } | E::C { a, b } => ( a, b ),
///     }
///   }
/// }
/// ```
///
fn generate_enum_named_variants
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant_idents : &[ syn::Ident ],
  fields : &syn::FieldsNamed,
)
-> Result< proc_macro2::TokenStream >
{
  let field_names = fields.named.iter().map( | field | field.ident.as_ref().unwrap() );
  let field_types = fields.named.iter().map( | field | &field.ty );
  let captures = qt! { #( #field_names ),* };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::convert::From< #item_name< #generics_ty > > for ( #( #field_types ),* )
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn from( other : #item_name< #generics_ty > ) -> Self
        {
          match other
          {
            #( #item_name::#variant_idents{ #captures } )|* => ( #captures )
          }
        }
      }
    }
  )
}
