use macro_tools ::
{
  diag, generic_params, item_struct, struct_like ::StructLike, Result, qt, attr, syn, proc_macro2, return_syn_err, Spanned,
};

use super ::item_attributes :: { ItemAttributes };

///
/// Derive macro to implement Not when-ever it's possible to do automatically.
///
pub fn not(input: proc_macro ::TokenStream) -> Result< proc_macro2 ::TokenStream > 
{
  let original_input = input.clone();
  let parsed = syn ::parse :: < StructLike >(input)?;
  let has_debug = attr ::has_debug(parsed.attrs().iter())?;
  let _item_attrs = ItemAttributes ::from_attrs(parsed.attrs().iter())?;
  let item_name = &parsed.ident();

  let (_generics_with_defaults, generics_impl, generics_ty, generics_where) = generic_params ::decompose(parsed.generics());

  let result =  match parsed 
  {
  StructLike ::Unit(ref _item) => generate_unit(item_name, &generics_impl, &generics_ty, &generics_where),
  StructLike ::Struct(ref item) =>
  {
   let field_type = item_struct ::first_field_type(item)?;
   let field_name_option = item_struct ::first_field_name(item)?;
   let field_name = field_name_option.as_ref();
   generate_struct(
  item_name,
  &generics_impl,
  &generics_ty,
  &generics_where,
  &field_type,
  field_name,
 )
 }
  StructLike ::Enum(ref item) =>
  {
   return_syn_err!(item.span(), "Not can be applied only to a structure");
 }
 };

  if has_debug 
  {
  let about = format!("derive: Not\nstructure: {item_name}");
  diag ::report_print(about, &original_input, &result);
 }

  Ok(result)
}

/// Generates `Not` implementation for unit structs.
///
/// Example of generated code :
/// ```text
/// impl Not for MyUnit
/// {
///   type Output = Self;
///   fn not( self ) -> Self
///   {
///     self
/// }
/// }
/// ```
fn generate_unit(
  item_name: &syn ::Ident,
  generics_impl: &syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  generics_ty: &syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  generics_where: &syn ::punctuated ::Punctuated< syn ::WherePredicate, syn ::token ::Comma >,
) -> proc_macro2 ::TokenStream {
  qt! {
  #[ automatically_derived ]
  impl< #generics_impl > core ::ops ::Not for #item_name< #generics_ty >
  where
   #generics_where
  {
   type Output = Self;
   #[ inline( always ) ]
   fn not( self ) -> Self ::Output
   {
  self
 }
 }
 }
}

/// Generates `Not` implementation for structs with fields.
///
/// Example of generated code :
/// ```text
/// impl Not for MyStruct
/// {
///   type Output = bool;
///   fn not( self ) -> bool
///   {
///     !self.0
/// }
/// }
/// ```
fn generate_struct(
  item_name: &syn ::Ident,
  generics_impl: &syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  generics_ty: &syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  generics_where: &syn ::punctuated ::Punctuated< syn ::WherePredicate, syn ::token ::Comma >,
  _field_type: &syn ::Type,
  field_name: Option< &syn ::Ident >,
) -> proc_macro2 ::TokenStream {
  let body =  if let Some(field_name) = field_name 
  {
  qt! { Self { #field_name: !self.#field_name } }
 } else {
  qt! { Self( !self.0 ) }
 };

  qt! {
  #[ automatically_derived ]
  impl< #generics_impl > core ::ops ::Not for #item_name< #generics_ty >
  where
   #generics_where
  {
   type Output = Self;
   #[ inline( always ) ]
   fn not( self ) -> Self ::Output
   {
  #body
 }
 }
 }
}
