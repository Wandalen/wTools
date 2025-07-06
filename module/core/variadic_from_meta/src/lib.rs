#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/variadic_from_meta/latest/variadic_from_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]
#![ allow( clippy::doc_markdown ) ] // Added to bypass doc_markdown lint for now

use macro_tools::
{
  quote,
  syn,
  proc_macro2,
};
use quote::ToTokens;
use syn::{ parse_macro_input, DeriveInput, Data, Fields, Type };

/// Context for generating `VariadicFrom` implementations.
struct VariadicFromContext<'a>
{
  name : &'a syn::Ident,
  field_types : Vec< &'a syn::Type >,
  field_names_or_indices : Vec<proc_macro2::TokenStream>,
  is_tuple_struct : bool,
  num_fields : usize,
}

impl<'a> VariadicFromContext<'a>
{
  fn new( ast : &'a DeriveInput ) -> syn::Result<Self>
  {
    let name = &ast.ident;

    let Data::Struct( data ) = &ast.data else
    {
      return Err( syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs." ) );
    };

    let ( field_types, field_names_or_indices, is_tuple_struct ) : ( Vec< &Type >, Vec< proc_macro2::TokenStream >, bool ) = match &data.fields
    {
      Fields::Unnamed( fields ) =>
      {
        let types = fields.unnamed.iter().map( |f| &f.ty ).collect();
        let indices = ( 0..fields.unnamed.len() ).map( |i| syn::Index::from( i ).to_token_stream() ).collect();
        ( types, indices, true )
      },
      Fields::Named( fields ) =>
      {
        let types = fields.named.iter().map( |f| &f.ty ).collect();
        let names = fields.named.iter().map( |f| f.ident.as_ref().unwrap().to_token_stream() ).collect();
        ( types, names, false )
      },
      Fields::Unit => return Err( syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs with named or unnamed fields." ) ), // Fixed: match_wildcard_for_single_variants
    };

    let num_fields = field_types.len();

    Ok( Self
    {
      name,
      field_types,
      field_names_or_indices,
      is_tuple_struct,
      num_fields,
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
      quote! { ( #arg ) } // Fixed: removed repetition for single arg
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

/// Generates `FromN` trait implementations.
fn generate_from_n_impls( context : &VariadicFromContext<'_> ) -> proc_macro2::TokenStream
{
  let mut impls = quote! {};
  let name = context.name;
  let num_fields = context.num_fields;

  // Generate new argument names for the `from` function
  let from_fn_args : Vec<proc_macro2::Ident> = (0..num_fields).map(|i| proc_macro2::Ident::new(&format!("__a{}", i + 1), proc_macro2::Span::call_site())).collect();
  let from_fn_args_ts = from_fn_args.iter().map(|arg| quote! { #arg }).collect::<proc_macro2::TokenStream>();

  if num_fields == 1
  {
    let field_type = &context.field_types[ 0 ]; // Use context.field_types directly
    let constructor = context.constructor( &from_fn_args );
    impls.extend( quote!
    {
      impl ::variadic_from::exposed::From1< #field_type > for #name
      {
        fn from1( #from_fn_args_ts : #field_type ) // Use from_fn_args_ts here
        {
          Self #constructor
        }
      }
    });
  }
  else if num_fields == 2
  {
    let field_type1 = &context.field_types[ 0 ]; // Use context.field_types directly
    let field_type2 = &context.field_types[ 1 ]; // Use context.field_types directly
    let constructor = context.constructor( &from_fn_args );
    impls.extend( quote!
    {
      impl ::variadic_from::exposed::From2< #field_type1, #field_type2 > for #name
      {
        fn from2( #from_fn_args_ts : #field_type1, a2 : #field_type2 ) // Use from_fn_args_ts here
        {
          Self #constructor
        }
      }
    });
  }
  else if num_fields == 3
  {
    let field_type1 = &context.field_types[ 0 ]; // Use context.field_types directly
    let field_type2 = &context.field_types[ 1 ]; // Use context.field_types directly
    let field_type3 = &context.field_types[ 2 ]; // Use context.field_types directly
    let constructor = context.constructor( &from_fn_args );
    impls.extend( quote!
    {
      impl ::variadic_from::exposed::From3< #field_type1, #field_type2, #field_type3 > for #name
      {
        fn from3( #from_fn_args_ts : #field_type1, a2 : #field_type2, a3 : #field_type3 ) // Use from_fn_args_ts here
        {
          Self #constructor
        }
      }
    });
  }
  impls
}

/// Generates `From<T>` or `From<(T1, ..., TN)>` trait implementations.
fn generate_from_trait_impl( context : &VariadicFromContext<'_> ) -> proc_macro2::TokenStream
{
  let mut impls = quote! {};
  let name = context.name;
  let num_fields = context.num_fields;

  // Generate new argument names for the `from` function
  let from_fn_args : Vec<proc_macro2::Ident> = (0..num_fields).map(|i| proc_macro2::Ident::new(&format!("__a{}", i + 1), proc_macro2::Span::call_site())).collect();
  let from_fn_args_ts = from_fn_args.iter().map(|arg| quote! { #arg }).collect::<proc_macro2::TokenStream>();

  if num_fields == 1
  {
    let field_type = &context.field_types[ 0 ]; // Use context.field_types directly
    let from_fn_arg = &from_fn_args[ 0 ];
    impls.extend( quote!
    {
      impl From< #field_type > for #name
      {
        #[ inline( always ) ]
        fn from( #from_fn_arg : #field_type ) -> Self
        {
          // Delegate to From1 trait method
          Self::from1( #from_fn_arg )
        }
      }
    });
  }
  else if num_fields == 2
  {
    let field_types_iter = context.field_types.iter(); // Fixed: local variable for iterator
    let tuple_types = quote! { #( #field_types_iter ),* }; // Use field_types_iter here
    let from_fn_args_pattern = from_fn_args_ts; // Use from_fn_args_ts here
    impls.extend( quote!
    {
      impl From< ( #tuple_types ) > for #name
      {
        #[ inline( always ) ]
        fn from( ( #from_fn_args_pattern ) : ( #tuple_types ) ) -> Self // Use from_fn_args_pattern here
        {
          // Delegate to From2 trait method
          Self::from2( #from_fn_args_pattern )
        }
      }
    });
  }
  else if num_fields == 3
  {
    let field_types_iter = context.field_types.iter(); // Fixed: local variable for iterator
    let tuple_types = quote! { #( #field_types_iter ),* }; // Use field_types_iter here
    let from_fn_args_pattern = from_fn_args_ts; // Use from_fn_args_ts here
    impls.extend( quote!
    {
      impl From< ( #tuple_types ) > for #name
      {
        #[ inline( always ) ]
        fn from( ( #from_fn_args_pattern ) : ( #tuple_types ) ) -> Self // Use from_fn_args_pattern here
        {
          // Delegate to From3 trait method
          Self::from3( #from_fn_args_pattern )
        }
      }
    });
  }
  impls
}

/// Generates convenience `FromN` implementations.
fn generate_convenience_impls( context : &VariadicFromContext<'_> ) -> proc_macro2::TokenStream
{
  let mut impls = quote! {};
  let name = context.name;
  let num_fields = context.num_fields;

  if num_fields == 2
  {
    if context.are_all_field_types_identical()
    {
      let field_type = &context.field_types[ 0 ]; // Use context.field_types directly
      let from_fn_arg = proc_macro2::Ident::new( "__a1", proc_macro2::Span::call_site() );
      let constructor = context.constructor_uniform( &from_fn_arg );
      impls.extend( quote!
      {
        impl ::variadic_from::exposed::From1< #field_type > for #name
        {
          fn from1( #from_fn_arg : #field_type ) -> Self
          {
            Self #constructor
          }
        }
      });
    }
  }
  else if num_fields == 3
  {
    let field_type1 = &context.field_types[ 0 ]; // Use context.field_types directly
    let from_fn_arg1 = proc_macro2::Ident::new( "__a1", proc_macro2::Span::call_site() );
    let constructor_uniform_all = context.constructor_uniform( &from_fn_arg1 );

    if context.are_all_field_types_identical()
    {
      impls.extend( quote!
      {
        impl ::variadic_from::exposed::From1< #field_type1 > for #name
        {
          fn from1( #from_fn_arg1 : #field_type1 ) -> Self
          {
            Self #constructor_uniform_all
          }
        }
      });
    }

    let field_type2 = &context.field_types[ 1 ]; // Use context.field_types directly
    let from_fn_arg1 = proc_macro2::Ident::new( "__a1", proc_macro2::Span::call_site() );
    let from_fn_arg2 = proc_macro2::Ident::new( "__a2", proc_macro2::Span::call_site() );
    let constructor_uniform_last_two = if context.is_tuple_struct {
        quote! { ( #from_fn_arg1, #from_fn_arg2, #from_fn_arg2 ) }
    } else {
        let field_name_or_index1 = &context.field_names_or_indices[0];
        let field_name_or_index2 = &context.field_names_or_indices[1];
        let field_name_or_index3 = &context.field_names_or_indices[2];
        quote! { { #field_name_or_index1 : #from_fn_arg1, #field_name_or_index2 : #from_fn_arg2, #field_name_or_index3 : #from_fn_arg2 } }
    };

    if context.are_field_types_identical_from( 1 )
    {
      impls.extend( quote!
      {
        impl ::variadic_from::exposed::From2< #field_type1, #field_type2 > for #name
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

  impls.extend( generate_from_n_impls( &context ) );
  impls.extend( generate_from_trait_impl( &context ) );
  impls.extend( generate_convenience_impls( &context ) );

  let result = quote!
  {
    #impls
  };
  result.into()
}