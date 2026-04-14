//! Popular type support for ComponentModel derive macro
//!
//! This module contains logic to generate additional Assign implementations for popular Rust types.

use macro_tools ::prelude :: *;

/// Generate additional Assign implementations for popular types
/// This is called by the `ComponentModel` derive macro for each field
#[ allow(clippy ::too_many_lines, clippy ::similar_names) ]
pub fn generate_popular_type_assigns( 
  struct_name: &syn ::Ident, 
  field_name: &syn ::Ident, 
  field_type: &syn ::Type,
  generics: &syn ::Generics,
  impl_generics: &syn ::ImplGenerics< '_ >,
  ty_generics: &syn ::TypeGenerics< '_ >,
  where_clause: Option< &syn ::WhereClause >
) -> Vec< proc_macro2 ::TokenStream >
{
  let mut impls = Vec ::new();
  
  // Convert field type to string for matching
  let type_str = quote ::quote!( #field_type ).to_string();
  
  match type_str.as_str()
  {
  "Duration" =>
  {
   // Generate Assign implementations for Duration from u64, f64, (u64, u32)
   let impl1 =  if generics.params.is_empty() 
  {
  quote ::quote!
  {
   impl component_model_types ::Assign< std ::time ::Duration, u64 > for #struct_name
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: u64 )
  {
   self.#field_name = std ::time ::Duration ::from_secs( component );
 }
 }
 }
 } else {
  quote ::quote!
  {
   impl #impl_generics component_model_types ::Assign< std ::time ::Duration, u64 > for #struct_name #ty_generics
   #where_clause
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: u64 )
  {
   self.#field_name = std ::time ::Duration ::from_secs( component );
 }
 }
 }
 };
   
   let impl2 =  if generics.params.is_empty() 
  {
  quote ::quote!
  {
   impl component_model_types ::Assign< std ::time ::Duration, f64 > for #struct_name
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: f64 )
  {
   self.#field_name = std ::time ::Duration ::from_secs_f64( component );
 }
 }
 }
 } else {
  quote ::quote!
  {
   impl #impl_generics component_model_types ::Assign< std ::time ::Duration, f64 > for #struct_name #ty_generics
   #where_clause
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: f64 )
  {
   self.#field_name = std ::time ::Duration ::from_secs_f64( component );
 }
 }
 }
 };
   
   let impl3 =  if generics.params.is_empty() 
  {
  quote ::quote!
  {
   impl component_model_types ::Assign< std ::time ::Duration, ( u64, u32 ) > for #struct_name
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: ( u64, u32 ) )
  {
   self.#field_name = std ::time ::Duration ::new( component.0, component.1 );
 }
 }
 }
 } else {
  quote ::quote!
  {
   impl #impl_generics component_model_types ::Assign< std ::time ::Duration, ( u64, u32 ) > for #struct_name #ty_generics
   #where_clause
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: ( u64, u32 ) )
  {
   self.#field_name = std ::time ::Duration ::new( component.0, component.1 );
 }
 }
 }
 };
   
   impls.push( impl1 );
   impls.push( impl2 );
   impls.push( impl3 );
 }
  
  "PathBuf" =>
  {
   // Generate Assign implementations for PathBuf from &str, String
   let impl1 =  if generics.params.is_empty() 
  {
  quote ::quote!
  {
   impl component_model_types ::Assign< std ::path ::PathBuf, &str > for #struct_name
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: &str )
  {
   self.#field_name = std ::path ::PathBuf ::from( component );
 }
 }
 }
 } else {
  quote ::quote!
  {
   impl #impl_generics component_model_types ::Assign< std ::path ::PathBuf, &str > for #struct_name #ty_generics
   #where_clause
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: &str )
  {
   self.#field_name = std ::path ::PathBuf ::from( component );
 }
 }
 }
 };
   
   let impl2 =  if generics.params.is_empty() 
  {
  quote ::quote!
  {
   impl component_model_types ::Assign< std ::path ::PathBuf, String > for #struct_name
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: String )
  {
   self.#field_name = std ::path ::PathBuf ::from( component );
 }
 }
 }
 } else {
  quote ::quote!
  {
   impl #impl_generics component_model_types ::Assign< std ::path ::PathBuf, String > for #struct_name #ty_generics
   #where_clause
   {
  #[ inline( always ) ]
  fn assign( &mut self, component: String )
  {
   self.#field_name = std ::path ::PathBuf ::from( component );
 }
 }
 }
 };
   
   impls.push( impl1 );
   impls.push( impl2 );
 }
  
  _ => {} // No special implementations for this type
 }
  
  impls
}

// Note: is_popular_type function was removed as it's currently unused.
// Type detection is handled directly in generate_popular_type_assigns() through pattern matching.