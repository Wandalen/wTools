#[ cfg( test ) ]
mod test_decompose 
{
  use crate ::generic_params;
  use syn ::parse_quote;
  
  #[ test ]
  fn test_trailing_comma_issue() 
  {
  // Test case from the issue  
  let generics: syn ::Generics = parse_quote! { < 'a > };
  let (_, impl_gen, ty_gen, _) = generic_params ::decompose(&generics);
  
  println!("Input generics: {}", quote ::quote!(#generics));
  println!("impl_gen: {}", quote ::quote!(#impl_gen));
  println!("ty_gen: {}", quote ::quote!(#ty_gen));
  
  // Check if there's a trailing comma 
  assert!(!impl_gen.trailing_punct(), "impl_gen should not have trailing comma");
  assert!(!ty_gen.trailing_punct(), "ty_gen should not have trailing comma");
  
  // Test with multiple parameters
  let generics2: syn ::Generics = parse_quote! { < 'a, T > };
  let (_, impl_gen2, ty_gen2, _) = generic_params ::decompose(&generics2);
  
  println!("Input generics2: {}", quote ::quote!(#generics2));
  println!("impl_gen2: {}", quote ::quote!(#impl_gen2));
  println!("ty_gen2: {}", quote ::quote!(#ty_gen2));
  
  // Check trailing commas for multi-param case
  assert!(!impl_gen2.trailing_punct(), "impl_gen2 should not have trailing comma");
  assert!(!ty_gen2.trailing_punct(), "ty_gen2 should not have trailing comma");
 }
}