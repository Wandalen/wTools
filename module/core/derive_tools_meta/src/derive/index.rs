
use super::*;
use macro_tools::
{
  attr, diag, generic_params, proc_macro2::TokenStream, struct_like::StructLike, Result
};

#[ path = "index/field_attributes.rs" ]
mod field_attributes;
use field_attributes::*;

#[ path = "index/item_attributes.rs" ]
mod item_attributes;
use item_attributes::*;




/// Generates [Index](core::ops::Index) trait implementation.
pub fn index( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
   
  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where ) 
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed 
  {
    StructLike::Enum( ref item ) => 
    {
      let variants_result : Result< Vec< proc_macro2::TokenStream > > = item.variants.iter().map( | variant |
      {
 generate_enum
      ( 
        item_name,
        &item_attrs, 
        &generics_impl, 
        &generics_ty, 
        &generics_where,
        variant, 
        &original_input,
        &item.variants,
      )?;

Ok( qt!{} )
        }
      
      ).collect();

      let variants = variants_result?;

      Ok(qt!
      {
        #( #variants )*
      })
    },
       StructLike::Struct( ref item ) =>
      generate_struct
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &item.fields,
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
        #index => &self.0[index]
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
  let generated = fields.iter().enumerate().map(|( _index, field )| 
    {
      let field_name = &field.ident;

      qt! 
      {
        &self.#field_name[index]
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
           
            #(#generated)*
        }
      }
    }
  )
}



/// An aggregator function to generate `Index` implementation for Enum
fn generate_enum
(
  item_name: &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant : &syn::Variant,
  _original_input : &proc_macro::TokenStream,
  variants : &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) 
-> Result< proc_macro2::TokenStream > 
{

  let fields = &variant.fields;

  let idents = variants.iter().map( | v | v.ident.clone() ).collect::< Vec< _ > >();

  let attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;


 if !attrs.index.value( item_attrs.index.value( true ) )
  {
    return Ok( qt!{} )
  }

    if fields.len() <= 0
  {
    return Ok( qt!{} )
  }

  let ( args, _use_src ) = if fields.len() == 1
  {
    let field = fields.iter().next().unwrap();
    (
      qt!{ #field },
      qt!{ src },
    )
  }
  else
  {
    let src_i = ( 0..fields.len() ).map( | e |
    {
      let i = syn::Index::from( e );
      qt!{ src.#i, }
    });
    (
      qt!{ #fields },
      qt!{ #( #src_i )* },
      // qt!{ src.0, src.1 },
    )
  };

    
  match fields 
  {
    syn::Fields::Named( ref item ) => 
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
      item,
      &idents,
      &args,
    ),  
    syn::Fields::Unit => Err( 
      syn::Error::new(
        variant.fields.span(),
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
  fields: &syn::FieldsUnnamed,
  variant_idents : &[ syn::Ident ],
  _args: &TokenStream
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
           #( #item_name::#variant_idents( v ) )|* => &v[index]
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

/*
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
  )*/
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

      dbg!(&field_name);
      qt! 
      {
        #index => match self 
        {
          #( #item_name::#variant_idents { #field_name: v, .. } )|* => v[index],   
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


