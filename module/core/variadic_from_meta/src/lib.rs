#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/variadic_from_meta/latest/variadic_from_meta/" ) ]
#![ allow( clippy::doc_markdown ) ] // Added to bypass doc_markdown lint for now
//! This crate provides a procedural macro for deriving `VariadicFrom` traits.

use macro_tools::
{
  quote,
  syn,
  proc_macro2,
};
use quote::ToTokens;
use syn::{ parse_macro_input, DeriveInput, Type, Data, Fields }; // Added Fields import

/// Context for generating `VariadicFrom` implementations.
struct VariadicFromContext<'a>
{
  name : &'a syn::Ident,
  field_types : Vec< &'a syn::Type >,
  field_names_or_indices : Vec<proc_macro2::TokenStream>,
  is_tuple_struct : bool,
  num_fields : usize,
  generics : &'a syn::Generics,
}

impl<'a> VariadicFromContext<'a>
{
  fn new( ast : &'a DeriveInput ) -> syn::Result<Self>
  {
    let name = &ast.ident;

    let ( field_types, field_names_or_indices, is_tuple_struct ) : ( Vec< &Type >, Vec< proc_macro2::TokenStream >, bool ) = match &ast.data
    {
      Data::Struct( data ) =>
      {
        match &data.fields
        {
          Fields::Named( fields ) =>
          {
            let types = fields.named.iter().map( |f| &f.ty ).collect();
            let names = fields.named.iter().map( |f| f.ident.as_ref().unwrap().to_token_stream() ).collect();
            ( types, names, false )
          },
          Fields::Unnamed( fields ) =>
          {
            let types = fields.unnamed.iter().map( |f| &f.ty ).collect();
            let indices = ( 0..fields.unnamed.len() ).map( |i| syn::Index::from( i ).to_token_stream() ).collect();
            ( types, indices, true )
          },
          Fields::Unit => return Err( syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs with named or unnamed fields." ) ),
        }
      },
      _ => return Err( syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs." ) ),
    };

    let num_fields = field_types.len();

    Ok( Self
    {
      name,
      field_types,
      field_names_or_indices,
      is_tuple_struct,
      num_fields,
      generics : &ast.generics,
    })
  }

  /// Generates the constructor for the struct based on its type (tuple or named).
  fn constructor( &self, args : &[ proc_macro2::Ident ] ) -> proc_macro2::TokenStream
  {
    if self.is_tuple_struct
    {
      quote! { ( #( #args ),* ) }
    }
    else
    {
      let named_field_inits = self.field_names_or_indices.iter().zip( args.iter() ).map( |( name, arg )|
      {
        quote! { #name : #arg }
      }).collect::< Vec<_> >();
      quote! { { #( #named_field_inits ),* } }
    }
  }

  /// Generates the constructor for the struct when all fields are the same type.
  fn constructor_uniform( &self, arg : &proc_macro2::Ident ) -> proc_macro2::TokenStream
  {
    if self.is_tuple_struct
    {
      let repeated_args = (0..self.num_fields).map(|_| arg).collect::<Vec<_>>();
      quote! { ( #( #repeated_args ),* ) }
    }
    else
    {
      let named_field_inits = self.field_names_or_indices.iter().map( |name|
      {
        quote! { #name : #arg }
      }).collect::< Vec<_> >();
      quote! { { #( #named_field_inits ),* } }
    }
  }

  /// Checks if all field types are identical.
  fn are_all_field_types_identical( &self ) -> bool
  {
    if self.num_fields == 0 { return true; }
    let first_type = &self.field_types[ 0 ];
    self.field_types.iter().all( |ty| ty.to_token_stream().to_string() == first_type.to_token_stream().to_string() )
  }

  /// Checks if a subset of field types are identical.
  fn are_field_types_identical_from( &self, start_idx : usize ) -> bool
  {
    if start_idx >= self.num_fields { return true; }
    let first_type = &self.field_types[ start_idx ];
    self.field_types[ start_idx.. ].iter().all( |ty| ty.to_token_stream().to_string() == first_type.to_token_stream().to_string() )
  }
}

/// Helper function to check if a type is `String`.
fn is_type_string(ty: &syn::Type) -> bool {
    ty.to_token_stream().to_string() == quote! { String }.to_string()
}

/// Generates `FromN` trait implementations.
#[ allow( clippy::similar_names ) ]
fn generate_from_n_impls( context : &VariadicFromContext<'_>, from_fn_args : &[ proc_macro2::Ident ] ) -> proc_macro2::TokenStream
{
  let mut impls = quote! {};
  let name = context.name;
  let num_fields = context.num_fields;
  let ( impl_generics, ty_generics, where_clause ) = context.generics.split_for_impl();

  if num_fields == 1
  {
    let from_fn_arg1 = &from_fn_args[ 0 ];
    let field_type = &context.field_types[ 0 ];
    let constructor = context.constructor( core::slice::from_ref( from_fn_arg1 ) );
    impls.extend( quote!
    {
      impl #impl_generics ::variadic_from::exposed::From1< #field_type > for #name #ty_generics #where_clause
      {
        fn from1( #from_fn_arg1 : #field_type ) -> Self
        {
          Self #constructor
        }
      }
    });
  }
  else if num_fields == 2
  {
    let from_fn_arg1 = &from_fn_args[ 0 ];
    let from_fn_arg2 = &from_fn_args[ 1 ];
    let field_type1 = &context.field_types[ 0 ];
    let field_type2 = &context.field_types[ 1 ];
    let constructor = context.constructor( &[ from_fn_arg1.clone(), from_fn_arg2.clone() ] );
    impls.extend( quote!
    {
      impl #impl_generics ::variadic_from::exposed::From2< #field_type1, #field_type2 > for #name #ty_generics #where_clause
      {
        fn from2( #from_fn_arg1 : #field_type1, #from_fn_arg2 : #field_type2 ) -> Self
        {
          Self #constructor
        }
      }
    });
  }
  else if num_fields == 3
  {
    let from_fn_arg1 = &from_fn_args[ 0 ];
    let from_fn_arg2 = &from_fn_args[ 1 ];
    let from_fn_arg3 = &from_fn_args[ 2 ];
    let field_type1 = &context.field_types[ 0 ];
    let field_type2 = &context.field_types[ 1 ];
    let field_type3 = &context.field_types[ 2 ];
    let constructor = context.constructor( &[ from_fn_arg1.clone(), from_fn_arg2.clone(), from_fn_arg3.clone() ] );
    impls.extend( quote!
    {
      impl #impl_generics ::variadic_from::exposed::From3< #field_type1, #field_type2, #field_type3 > for #name #ty_generics #where_clause
      {
        fn from3( #from_fn_arg1 : #field_type1, #from_fn_arg2 : #field_type2, #from_fn_arg3 : #field_type3 ) -> Self
        {
          Self #constructor
        }
      }
    });
  }
  impls
}

/// Generates `From<T>` or `From<(T1, ..., TN)>` trait implementations.
#[ allow( clippy::similar_names ) ]
fn generate_from_tuple_impl( context : &VariadicFromContext<'_>, from_fn_args : &[ proc_macro2::Ident ] ) -> proc_macro2::TokenStream
{
  let mut impls = quote! {};
  let name = context.name;
  let num_fields = context.num_fields;
  let ( impl_generics, ty_generics, where_clause ) = context.generics.split_for_impl();

  if num_fields == 1
  {
    let from_fn_arg1 = &from_fn_args[ 0 ];
    let field_type = &context.field_types[ 0 ];
    impls.extend( quote!
    {
      impl #impl_generics From< #field_type > for #name #ty_generics #where_clause
      {
        #[ inline( always ) ]
        fn from( #from_fn_arg1 : #field_type ) -> Self
        {
          // Delegate to From1 trait method
          Self::from1( #from_fn_arg1.clone() ) // Fixed: Added #
        }
      }
    });
  }
  else if num_fields == 2
  {
    let from_fn_arg1 = &from_fn_args[ 0 ];
    let from_fn_arg2 = &from_fn_args[ 1 ];
    let field_type1 = &context.field_types[ 0 ];
    let field_type2 = &context.field_types[ 1 ];
    let tuple_types = quote! { #field_type1, #field_type2 };
    let from_fn_args_pattern = quote! { #from_fn_arg1, #from_fn_arg2 };
    impls.extend( quote!
    {
      impl #impl_generics From< ( #tuple_types ) > for #name #ty_generics #where_clause
      {
        #[ inline( always ) ]
        fn from( ( #from_fn_args_pattern ) : ( #tuple_types ) ) -> Self
        {
          // Delegate to From2 trait method
          Self::from2( #from_fn_arg1.clone(), #from_fn_arg2.clone() ) // Fixed: Added #
        }
      }
    });
  }
  else if num_fields == 3
  {
    let from_fn_arg1 = &from_fn_args[ 0 ];
    let from_fn_arg2 = &from_fn_args[ 1 ];
    let from_fn_arg3 = &from_fn_args[ 2 ];
    let field_type1 = &context.field_types[ 0 ];
    let field_type2 = &context.field_types[ 1 ];
    let field_type3 = &context.field_types[ 2 ];
    let tuple_types = quote! { #field_type1, #field_type2, #field_type3 };
    let from_fn_args_pattern = quote! { #from_fn_arg1, #from_fn_arg2, #from_fn_arg3 };
    impls.extend( quote!
    {
      impl #impl_generics From< ( #tuple_types ) > for #name #ty_generics #where_clause
      {
        #[ inline( always ) ]
        fn from( ( #from_fn_args_pattern ) : ( #tuple_types ) ) -> Self
        {
          // Delegate to From3 trait method
          Self::from3( #from_fn_arg1.clone(), #from_fn_arg2.clone(), #from_fn_arg3.clone() ) // Fixed: Added #
        }
      }
    });
  }
  impls
}

/// Generates convenience `FromN` implementations.
#[ allow( clippy::similar_names ) ]
fn generate_convenience_impls( context : &VariadicFromContext<'_>, from_fn_args : &[ proc_macro2::Ident ] ) -> proc_macro2::TokenStream
{
  let mut impls = quote! {};
  let name = context.name;
  let num_fields = context.num_fields;
  let ( impl_generics, ty_generics, where_clause ) = context.generics.split_for_impl();

  if num_fields == 2
  {
    if context.are_all_field_types_identical()
    {
      let from_fn_arg1 = &from_fn_args[ 0 ];
      let field_type = &context.field_types[ 0 ];
      let constructor = context.constructor_uniform( from_fn_arg1 );
      impls.extend( quote!
      {
        impl #impl_generics ::variadic_from::exposed::From1< #field_type > for #name #ty_generics #where_clause
        {
          fn from1( #from_fn_arg1 : #field_type ) -> Self
          {
            Self #constructor
          }
        }
      });
    }
  }
  else if num_fields == 3
  {
    let from_fn_arg1 = &from_fn_args[ 0 ];
    let from_fn_arg2 = &from_fn_args[ 1 ];
    let field_type1 = &context.field_types[ 0 ];
    let constructor_uniform_all = context.constructor_uniform( from_fn_arg1 );

    if context.are_all_field_types_identical()
    {
      impls.extend( quote!
      {
        impl #impl_generics ::variadic_from::exposed::From1< #field_type1 > for #name #ty_generics #where_clause
        {
          fn from1( #from_fn_arg1 : #field_type1 ) -> Self
          {
            Self #constructor_uniform_all
          }
        }
      });
    }

    let field_type1 = &context.field_types[ 0 ];
    let field_type2 = &context.field_types[ 1 ];
    let constructor_uniform_last_two = if context.is_tuple_struct {
        let arg1 = from_fn_arg1;
        let arg2_for_first_use = if is_type_string(context.field_types[1]) {
            quote! { #from_fn_arg2.clone() }
        } else {
            quote! { #from_fn_arg2 }
        };
        let arg2_for_second_use = if is_type_string(context.field_types[2]) {
            quote! { #from_fn_arg2.clone() }
        } else {
            quote! { #from_fn_arg2 }
        };
        quote! { ( #arg1, #arg2_for_first_use, #arg2_for_second_use ) }
    } else {
        let field_name_or_index1 = &context.field_names_or_indices[0];
        let field_name_or_index2 = &context.field_names_or_indices[1];
        let field_name_or_index3 = &context.field_names_or_indices[2];
        let arg1 = from_fn_arg1;
        let arg2_for_first_use = if is_type_string(context.field_types[1]) {
            quote! { #from_fn_arg2.clone() }
        } else {
            quote! { #from_fn_arg2 }
        };
        let arg2_for_second_use = if is_type_string(context.field_types[2]) {
            quote! { #from_fn_arg2.clone() }
        } else {
            quote! { #from_fn_arg2 }
        };
        quote! { { #field_name_or_index1 : #arg1, #field_name_or_index2 : #arg2_for_first_use, #field_name_or_index3 : #arg2_for_second_use } }
    };

    if context.are_field_types_identical_from( 1 )
    {
      impls.extend( quote!
      {
        impl #impl_generics ::variadic_from::exposed::From2< #field_type1, #field_type2 > for #name #ty_generics #where_clause
        {
          fn from2( #from_fn_arg1 : #field_type1, #from_fn_arg2 : #field_type2 ) -> Self
          {
            Self #constructor_uniform_last_two
          }
        }
      });
    }
  }
  impls
}

/// Derive macro for `VariadicFrom`.
#[ proc_macro_derive( VariadicFrom ) ]
pub fn variadic_from_derive( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let ast = parse_macro_input!( input as DeriveInput );
  let context = match VariadicFromContext::new( &ast )
  {
    Ok( c ) => c,
    Err( e ) => return e.to_compile_error().into(),
  };

  let mut impls = quote! {};

  if context.num_fields == 0 || context.num_fields > 3
  {
    return proc_macro::TokenStream::new();
  }

  // Generate argument names once
  let from_fn_args : Vec<proc_macro2::Ident> = (0..context.num_fields).map(|i| proc_macro2::Ident::new(&format!("__a{}", i + 1), proc_macro2::Span::call_site())).collect();

  impls.extend( generate_from_n_impls( &context, &from_fn_args ) );
  impls.extend( generate_from_tuple_impl( &context, &from_fn_args ) );
  impls.extend( generate_convenience_impls( &context, &from_fn_args ) );

  let result = quote!
  {
    #impls
  };
  result.into()
}