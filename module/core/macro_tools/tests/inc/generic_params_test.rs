
use super::*;

//

#[ test ]
fn generics_with_where()
{

  let got : the_module::GenericsWithWhere = parse_quote!
  {
    < 'a, T : Clone, U : Default, V : core::fmt::Debug >
    where
      Definition : former::FormerDefinition,
  };
  let got = got.unwrap();

  let mut exp : syn::Generics = parse_quote!
  {
    < 'a, T : Clone, U : Default, V : core::fmt::Debug >
  };
  exp.where_clause = parse_quote!
  {
    where
      Definition : former::FormerDefinition,
  };

  // a_id!( tree_print!( got ), tree_print!( exp ) );
  // code_print!( got );
  // code_print!( exp );
  // code_print!( got.where_clause );
  // code_print!( exp.where_clause );

  assert_eq!( got.params, exp.params );
  assert_eq!( got.where_clause, exp.where_clause );
  assert_eq!( got, exp );

}

//

#[ test ]
fn merge_assumptions()
{
  use the_module::generic_params;

  let mut generics_a : syn::Generics = parse_quote!{ < T : Clone, U : Default > };
  generics_a.where_clause = parse_quote!{ where T : Default };
  let mut generics_b : syn::Generics = parse_quote!{ < V : core::fmt::Debug > };
  generics_b.where_clause = parse_quote!{ where V : Sized };
  let got = generic_params::merge( &generics_a, &generics_b );

  let mut exp : syn::Generics = parse_quote!
  {
    < T : Clone, U : Default, V : core::fmt::Debug >
  };
  exp.where_clause = parse_quote!
  {
    where
      T : Default,
      V : Sized
  };

  // a_id!( tree_print!( got ), tree_print!( exp ) );
  // code_print!( got );
  // code_print!( exp );
  // code_print!( got.where_clause );
  // code_print!( exp.where_clause );

  assert_eq!( got.params, exp.params );
  assert_eq!( got.where_clause, exp.where_clause );
  assert_eq!( got, exp );

}

//

#[ test ]
fn merge_defaults()
{
  use the_module::generic_params;

  let mut generics_a : syn::Generics = parse_quote!{ < T : Clone, U : Default = Default1 > };
  generics_a.where_clause = parse_quote!{ where T : Default };
  let mut generics_b : syn::Generics = parse_quote!{ < V : core::fmt::Debug = Debug1 > };
  generics_b.where_clause = parse_quote!{ where V : Sized };
  let got = generic_params::merge( &generics_a, &generics_b );

  let mut exp : syn::Generics = parse_quote!
  {
    < T : Clone, U : Default = Default1, V : core::fmt::Debug = Debug1 >
  };
  exp.where_clause = parse_quote!
  {
    where
      T : Default,
      V : Sized
  };

  // a_id!( tree_print!( got ), tree_print!( exp ) );
  // code_print!( got );
  // code_print!( exp );
  // code_print!( got.where_clause );
  // code_print!( exp.where_clause );

  assert_eq!( got.params, exp.params );
  assert_eq!( got.where_clause, exp.where_clause );
  assert_eq!( got, exp );

}

//

#[ test ]
fn only_names()
{

  use macro_tools::syn::parse_quote;

  let generics : the_module::GenericsWithWhere = parse_quote!{ < T : Clone + Default, U, 'a, const N : usize > where T: core::fmt::Debug };
  let simplified_generics = macro_tools::generic_params::only_names( &generics.unwrap() );

  assert_eq!( simplified_generics.params.len(), 4 ); // Contains T, U, 'a, and N
  assert!( simplified_generics.where_clause.is_none() ); // Where clause is removed

}

//

#[ test ]
fn decompose_empty_generics()
{
  let generics : syn::Generics = syn::parse_quote! {};
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  assert!( impl_gen.is_empty(), "Impl generics should be empty" );
  assert!( ty_gen.is_empty(), "Type generics should be empty" );
  assert!( where_gen.is_empty(), "Where generics should be empty" );
}

#[ test ]
fn decompose_generics_without_where_clause()
{
  let generics : syn::Generics = syn::parse_quote! { < T, U > };
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  assert_eq!( impl_gen.len(), 2, "Impl generics should have two parameters" );
  assert_eq!( ty_gen.len(), 2, "Type generics should have two parameters" );
  assert!( where_gen.is_empty(), "Where generics should be empty" );

  let exp : syn::Generics = syn::parse_quote! { < T, U, > };
  a_id!( impl_gen, exp.params );
  let exp : syn::Generics = syn::parse_quote! { < T, U, > };
  a_id!( ty_gen, exp.params );

}

#[ test ]
fn decompose_generics_with_where_clause()
{
  use macro_tools::quote::ToTokens;

  let generics : the_module::GenericsWithWhere = syn::parse_quote! { < T, U > where T : Clone, U : Default };
  let generics = generics.unwrap();
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  let impl_exp : syn::Generics = syn::parse_quote! { < T, U, > };
  let ty_exp : syn::Generics = syn::parse_quote! { < T, U, > };
  a_id!( impl_gen, impl_exp.params );
  a_id!( ty_gen, ty_exp.params );

  assert_eq!( impl_gen.len(), 2, "Impl generics should have two parameters" );
  assert_eq!( ty_gen.len(), 2, "Type generics should have two parameters" );
  assert_eq!( where_gen.len(), 2, "Where generics should have two predicates" );

  let where_clauses : Vec< _ > = where_gen.iter().collect();

  // Properly match against the `syn::WherePredicate::Type` variant to extract `bounded_ty`
  if let syn::WherePredicate::Type( pt ) = &where_clauses[0]
  {
    assert_eq!( pt.bounded_ty.to_token_stream().to_string(), "T", "The first where clause should be for T" );
  }
  else
  {
    panic!( "First where clause is not a Type predicate as expected." );
  }

  if let syn::WherePredicate::Type( pt ) = &where_clauses[1]
  {
    assert_eq!( pt.bounded_ty.to_token_stream().to_string(), "U", "The second where clause should be for U" );
  }
  else
  {
    panic!( "Second where clause is not a Type predicate as expected." );
  }
}

#[ test ]
fn decompose_generics_with_only_where_clause()
{
  let generics : the_module::GenericsWithWhere = syn::parse_quote! { where T : Clone, U : Default };
  let generics = generics.unwrap();
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  assert!( impl_gen.is_empty(), "Impl generics should be empty" );
  assert!( ty_gen.is_empty(), "Type generics should be empty" );
  assert_eq!( where_gen.len(), 2, "Where generics should have two predicates" );

}

#[ test ]
fn decompose_generics_with_complex_constraints()
{
  use macro_tools::quote::ToTokens;
  let generics : the_module::GenericsWithWhere = syn::parse_quote! { < T : Clone + Send, U : Default > where T: Send, U: Default };
  let generics = generics.unwrap();
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  let impl_exp : syn::Generics = syn::parse_quote! { < T : Clone + Send, U : Default, > };
  let ty_exp : syn::Generics = syn::parse_quote! { < T, U, > };
  a_id!( impl_gen, impl_exp.params );
  a_id!( ty_gen, ty_exp.params );

  assert_eq!( impl_gen.len(), 2, "Impl generics should reflect complex constraints" );
  assert_eq!( ty_gen.len(), 2, "Type generics should reflect complex constraints" );
  assert_eq!( where_gen.len(), 2, "Where generics should reflect complex constraints" );

  let where_clauses : Vec<_> = where_gen.iter().collect();

  // Properly matching against the WherePredicate::Type variant
  if let syn::WherePredicate::Type( pt ) = &where_clauses[0]
  {
    assert_eq!( pt.bounded_ty.to_token_stream().to_string(), "T", "The first where clause should be for T" );
  }
  else
  {
    panic!( "First where clause is not a Type predicate as expected." );
  }

  if let syn::WherePredicate::Type( pt ) = &where_clauses[1]
  {
    assert_eq!( pt.bounded_ty.to_token_stream().to_string(), "U", "The second where clause should be for U" );
  }
  else
  {
    panic!( "Second where clause is not a Type predicate as expected." );
  }
}

#[ test ]
fn decompose_generics_with_nested_generic_types()
{
  let generics : syn::Generics = syn::parse_quote! { < T : Iterator< Item = U >, U > };
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  let impl_exp : syn::Generics = syn::parse_quote! { < T : Iterator< Item = U >, U, > };
  let ty_exp : syn::Generics = syn::parse_quote! { < T, U, > };
  a_id!( impl_gen, impl_exp.params );
  a_id!( ty_gen, ty_exp.params );

  assert_eq!( impl_gen.len(), 2, "Impl generics should handle nested generics" );
  assert_eq!( ty_gen.len(), 2, "Type generics should handle nested generics" );
  assert!( where_gen.is_empty(), "Where generics should be empty for non-conditional types" );
}

#[ test ]
fn decompose_generics_with_lifetime_parameters_only()
{
  let generics : syn::Generics = syn::parse_quote! { < 'a, 'b > };
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  let impl_exp : syn::Generics = syn::parse_quote! { < 'a, 'b, > };
  let ty_exp : syn::Generics = syn::parse_quote! { < 'a, 'b, > };
  a_id!( impl_gen, impl_exp.params );
  a_id!( ty_gen, ty_exp.params );

  assert_eq!( impl_gen.len(), 2, "Impl generics should contain only lifetimes" );
  assert_eq!( ty_gen.len(), 2, "Type generics should contain only lifetimes" );
  assert!( where_gen.is_empty(), "Where generics should be empty" );
}

#[ test ]
fn decompose_generics_with_constants_only()
{
  let generics : syn::Generics = syn::parse_quote! { < const N : usize, const M : usize > };
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  let impl_exp : syn::Generics = syn::parse_quote! { < const N : usize, const M : usize, > };
  let ty_exp : syn::Generics = syn::parse_quote! { < N, M, > };
  a_id!( impl_gen, impl_exp.params );
  a_id!( ty_gen, ty_exp.params );

  assert_eq!( impl_gen.len(), 2, "Impl generics should contain constants" );
  assert_eq!( ty_gen.len(), 2, "Type generics should contain constants" );
  assert!( where_gen.is_empty(), "Where generics should be empty" );
}

#[ test ]
fn decompose_generics_with_default_values()
{
  let generics : syn::Generics = syn::parse_quote! { < T = usize, U = i32 > };
  let ( impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  let impl_with_exp : syn::Generics = syn::parse_quote! { < T = usize, U = i32, > };
  let impl_exp : syn::Generics = syn::parse_quote! { < T, U, > };
  let ty_exp : syn::Generics = syn::parse_quote! { < T, U, > };
  a_id!( impl_with_def, impl_with_exp.params );
  a_id!( impl_gen, impl_exp.params );
  a_id!( ty_gen, ty_exp.params );

  assert_eq!( impl_gen.len(), 2, "Impl generics should retain default types" );
  assert_eq!( ty_gen.len(), 2, "Type generics should retain default types" );
  assert!( where_gen.is_empty(), "Where generics should be empty" );
}

#[ test ]
fn decompose_mixed_generics_types()
{
  use macro_tools::quote::ToTokens;
  let generics : the_module::GenericsWithWhere = syn::parse_quote! { < 'a, T, const N : usize, U : Trait1 > where T : Clone, U : Default };
  let generics = generics.unwrap();
  let ( _impl_with_def, impl_gen, ty_gen, where_gen ) = the_module::generic_params::decompose( &generics );

  let impl_exp : syn::Generics = syn::parse_quote! { < 'a, T, const N : usize, U : Trait1, > };
  let ty_exp : syn::Generics = syn::parse_quote! { < 'a, T, N, U, > };
  a_id!( impl_gen, impl_exp.params );
  a_id!( ty_gen, ty_exp.params );

  assert_eq!( impl_gen.len(), 4, "Impl generics should correctly interleave types" );
  assert_eq!( ty_gen.len(), 4, "Type generics should correctly interleave types" );
  assert_eq!( where_gen.len(), 2, "Where generics should include conditions for T and U" );

  // Correctly handling the pattern matching for WherePredicate::Type
  let where_clauses : Vec<_> = where_gen.iter().collect();
  if let syn::WherePredicate::Type( pt ) = &where_clauses[0]
  {
    assert_eq!( pt.bounded_ty.to_token_stream().to_string(), "T", "The first where clause should be for T" );
  }
  else
  {
    panic!( "First where clause is not a Type predicate as expected." );
  }

  if let syn::WherePredicate::Type( pt ) = &where_clauses[1]
  {
    assert_eq!( pt.bounded_ty.to_token_stream().to_string(), "U", "The second where clause should be for U" );
  }
  else
  {
    panic!( "Second where clause is not a Type predicate as expected." );
  }

}
