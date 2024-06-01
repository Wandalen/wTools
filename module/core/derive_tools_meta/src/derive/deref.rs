use super::*;
use macro_tools::{ attr, diag, generic_params, Result, struct_like::StructLike };

//

pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

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
    let about = format!( "derive : Deref\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `Deref` implementation for unit structs and enums
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Deref;
/// #[ derive( Deref ) ]
/// pub struct Struct;
/// ```
/// 
/// ## Output
/// ```rust
/// pub struct Struct;
/// #[ automatically_derived ]
/// impl ::core::ops::Deref for Struct
/// {
///   type Target = ();
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &()
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
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Deref for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Target = ();
        #[ inline( always ) ]
        fn deref( &self ) -> &Self::Target
        {
          &()
        }
      }
    }
  )
}

/// An aggregator function to generate `Deref` implementation for unit, tuple structs and the ones with named fields
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

/// Generates `Deref` implementation for structs with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Deref;
/// #[ derive( Deref ) ]
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
  let fields = &fields.unnamed;
  let field_type = match fields.first()
  {
    Some( field ) => &field.ty,
    None => return generate_unit
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
    ),
  };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Deref for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Target = #field_type;
        #[ inline( always ) ]
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }
    }
  )
}

/// Generates `Deref` implementation for structs with named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Deref;
/// #[ derive( Deref ) ]
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
  let ( field_name, field_type ) = match fields.first()
  {
    Some( field ) => ( field.ident.as_ref().unwrap(), &field.ty ),
    None => return generate_unit
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
    ),
  };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Deref for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Target = #field_type;
        #[ inline( always ) ]
        fn deref( &self ) -> &Self::Target
        {
          &self.#field_name
        }
      }
    }
  )
}

/// An aggregator function to generate `Deref` implementation for unit, tuple enums and the ones with named fields
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

/// Generates `Deref` implementation for enums with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Deref;
/// #[ derive( Deref ) ]
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
  let fields = &fields.unnamed;
  let field_ty = match fields.first()
  {
    Some( field ) => &field.ty,
    None => return generate_unit
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
    ),
  };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Deref for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Target = #field_ty;
        #[ inline( always ) ]
        fn deref( &self ) -> &Self::Target
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

/// Generates `Deref` implementation for enums with named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Deref;
/// #[ derive( Deref ) ]
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
  let ( field_name, field_ty ) = match fields.first()
  {
    Some( field ) => ( field.ident.as_ref().unwrap(), &field.ty ),
    None => return generate_unit
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
    ),
  };

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Deref for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Target = #field_ty;
        #[ inline( always ) ]
        fn deref( &self ) -> &Self::Target
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
