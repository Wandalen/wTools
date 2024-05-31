use super::*;
use macro_tools::{ attr, diag, generic_params, Result, struct_like::StructLike };

//

pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {

    StructLike::Unit( _ ) => generate_unit(),

    StructLike::Struct( ref item ) =>
    generate_struct
    (
      item_name,
      &generics_impl,
      &generics_ty,
      &generics_where,
      &item.fields,
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

  }?;

  if has_debug
  {
    let about = format!( "derive : DerefMut\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Placeholder for unit structs and enums. Does not generate any `DerefMut` implementation
fn generate_unit() -> Result< proc_macro2::TokenStream >
{
  Ok( qt!{} )
}

/// An aggregator function to generate `DerefMut` implementation for unit, tuple structs and the ones with named fields
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

    syn::Fields::Unit => generate_unit(),

    syn::Fields::Unnamed( _ ) =>
    generate_struct_tuple_fields
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
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

/// Generates `DerefMut` implementation for structs with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::{ Deref, DerefMut };
/// #[ derive( Deref, DerefMut ) ]
/// pub struct Struct( i32, Vec< String > );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct( i32, Vec< String > );
/// #[ automatically_derived ]
/// impl ::core::ops::Deref for Struct
/// {
///   type Target = i32;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &self.0
///   }
/// }
/// #[ automatically_derived ]
/// impl ::core::ops::DerefMut for Struct
/// {
///   #[ inline( always ) ]
///   fn deref_mut( &mut self ) -> &mut Self::Target
///   {
///     &mut self.0
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
)
-> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::DerefMut for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          &mut self.0
        }
      }
    }
  )
}

/// Generates `DerefMut` implementation for structs with named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::{ Deref, DerefMut };
/// #[ derive( Deref, DerefMut ) ]
/// pub struct Struct
/// {
///   a : i32,
///   b : Vec< String >,
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct
/// {
///   a : i32,
///   b : Vec< String >,
/// }
/// #[ automatically_derived ]
/// impl ::core::ops::Deref for Struct
/// {
///   type Target = i32;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &self.a
///   }
/// }
/// #[ automatically_derived ]
/// impl ::core::ops::DerefMut for Struct
/// {
///   #[ inline( always ) ]
///   fn deref_mut( &mut self ) -> &mut Self::Target
///   {
///     &mut self.a
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
  let fields = &fields.named;
  let field_name = match fields.first()
  {
    Some( field ) => field.ident.as_ref().unwrap(),
    None => return generate_unit(),
  };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::DerefMut for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          &mut self.#field_name
        }
      }
    }
  )
}

/// An aggregator function to generate `DerefMut` implementation for unit, tuple enums and the ones with named fields
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
    None => return generate_unit(),
  };

  let idents = variants.iter().map( | v | v.ident.clone() ).collect::< Vec< _ > >();

  match fields
  {

    syn::Fields::Unit => generate_unit(),

    syn::Fields::Unnamed( _ ) =>
    generate_enum_tuple_variants
    (
      item_name,
      &generics_impl,
      &generics_ty,
      &generics_where,
      &idents,
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

/// Generates `DerefMut` implementation for enums with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::{ Deref, DerefMut };
/// #[ derive( Deref, DerefMut ) ]
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
/// impl ::core::ops::Deref for E
/// {
///   type Target = i32;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     match self
///     {
///       E::A( v, .. ) | E::B( v, .. ) | E::C( v, .. ) => v,
///     }
///   }
/// }
/// #[ automatically_derived ]
/// impl ::core::ops::DerefMut for E
/// {
///   #[ inline( always ) ]
///   fn deref_mut( &mut self ) -> &mut Self::Target
///   {
///     match self
///     {
///       E::A( v, .. ) | E::B( v, .. ) | E::C( v, .. ) => v,
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
)
-> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::DerefMut for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          match self
          {
            #( #item_name::#variant_idents( v, .. ) )|* => v
          }
        }
      }
    }
  )
}

/// Generates `DerefMut` implementation for enums with named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::{ Deref, DerefMut };
/// #[ derive( Deref, DerefMut ) ]
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
/// impl ::core::ops::Deref for E
/// {
///   type Target = i32;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     match self
///     {
///       E::A { a : v, .. } | E::B { a : v, .. } | E::C { a : v, .. } => v,
///     }
///   }
/// }
/// #[ automatically_derived ]
/// impl ::core::ops::DerefMut for E
/// {
///   #[ inline( always ) ]
///   fn deref_mut( &mut self ) -> &mut Self::Target
///   {
///     match self
///     {
///       E::A { a : v, .. } | E::B { a : v, .. } | E::C { a : v, .. } => v,
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
  let fields = &fields.named;
  let field_name = match fields.first()
  {
    Some( field ) => field.ident.as_ref().unwrap(),
    None => return generate_unit(),
  };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::DerefMut for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline( always ) ]
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          match self
          {
            #( #item_name::#variant_idents{ #field_name : v, ..} )|* => v
          }
        }
      }
    }
  )
}
