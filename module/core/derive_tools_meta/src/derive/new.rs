use macro_tools :: { diag, generic_params, struct_like ::StructLike, Result, qt, attr, syn, proc_macro2, return_syn_err, Spanned };

use super ::field_attributes :: { FieldAttributes };
use super ::item_attributes :: { ItemAttributes };

///
/// Derive macro to implement New when-ever it's possible to do automatically.
///
pub fn new(input: proc_macro ::TokenStream) -> Result< proc_macro2 ::TokenStream > 
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
   let fields_result: Result< Vec< (syn ::Ident, syn ::Type) >> = item
  .fields
  .iter()
  .map(|field| {
   let _attrs = FieldAttributes ::from_attrs(field.attrs.iter())?;
   let field_name = field.ident.clone().expect("Expected named field");
   let field_type = field.ty.clone();
   Ok((field_name, field_type))
 })
  .collect();

   let fields = fields_result?;

   generate_struct(item_name, &generics_impl, &generics_ty, &generics_where, &fields)
 }
  StructLike ::Enum(ref item) =>
  {
   return_syn_err!(item.span(), "New can be applied only to a structure");
 }
 };

  if has_debug 
  {
  let about = format!("derive: New\nstructure: {item_name}");
  diag ::report_print(about, &original_input, &result);
 }

  Ok(result)
}

/// Generates `New` implementation for unit structs.
///
/// Example of generated code :
/// ```text
/// impl New for MyUnit
/// {
///   fn new() -> Self
///   {
///     Self
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
  impl< #generics_impl > crate ::New for #item_name< #generics_ty >
  where
   #generics_where
  {
   #[ inline( always ) ]
   fn new() -> Self
   {
  Self {}
 }
 }
 }
}

/// Generates `New` implementation for structs with fields.
///
/// Example of generated code :
/// ```text
/// impl New for MyStruct
/// {
///   fn new( field1: i32, field2: i32 ) -> Self
///   {
///     Self { field1, field2 }
/// }
/// }
/// ```
fn generate_struct(
  item_name: &syn ::Ident,
  generics_impl: &syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  generics_ty: &syn ::punctuated ::Punctuated< syn ::GenericParam, syn ::token ::Comma >,
  generics_where: &syn ::punctuated ::Punctuated< syn ::WherePredicate, syn ::token ::Comma >,
  fields: &[ (syn ::Ident, syn ::Type)],
) -> proc_macro2 ::TokenStream {
  let fields_init = fields
  .iter()
  .map(|(field_name, _field_type)| {
   qt! { #field_name }
 })
  .collect :: < Vec< _ >>();

  let fields_params = fields
  .iter()
  .map(|(field_name, field_type)| {
   qt! { #field_name: #field_type }
 })
  .collect :: < Vec< _ >>();

  let body =  if fields.is_empty() 
  {
  qt! { Self {} }
 } else {
  qt! { Self { #( #fields_init ),* } }
 };

  qt! {
  #[ automatically_derived ]
  impl< #generics_impl > crate ::New for #item_name< #generics_ty >
  where
   #generics_where
  {
   #[ inline( always ) ]
   fn new( #( #fields_params ),* ) -> Self
   {
  #body
 }
 }
 }
}
