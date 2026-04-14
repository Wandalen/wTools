use macro_tools ::
{
  diag, struct_like ::StructLike, Result, qt, attr, syn, proc_macro2, return_syn_err, syn_err, Spanned,
};

///
/// Derive macro to implement `DerefMut` when-ever it's possible to do automatically.
///
/// Fix(issue-5): Changed from `generic_params::decompose` to `split_for_impl`.
/// Root cause: `decompose()` returns `Punctuated` types incompatible with `quote!` macro, causing "expected one of..." errors with generic types.
/// Pitfall: Always use `split_for_impl()` for trait implementations, not `decompose()`. `split_for_impl()` returns properly formatted `ImplGenerics` and `TypeGenerics`.
///
pub fn deref_mut(input: proc_macro ::TokenStream) -> Result< proc_macro2 ::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn ::parse :: < StructLike >(input)?;
  let has_debug = attr ::has_debug(parsed.attrs().iter())?;
  let item_name = &parsed.ident();

  let (generics_impl, generics_ty, generics_where_option) = parsed.generics().split_for_impl();

  let result =  match parsed 
  {
  StructLike ::Unit(ref _item) =>
  {
   return_syn_err!(parsed.span(), "Expects a structure with one field");
 }
  StructLike ::Struct(ref item) =>
  {
   let fields_count = item.fields.len();
   let mut target_field_type = None;
   let mut target_field_name = None;
   let mut deref_mut_attr_count = 0;

   if fields_count == 0 
   {
  return_syn_err!(item.span(), "DerefMut cannot be derived for structs with no fields.");
 } else  if fields_count == 1 
  {
  // Single field struct: automatically deref_mut to that field
  let field = item.fields.iter().next().expect("Expects a single field to derive DerefMut");
  target_field_type = Some(field.ty.clone());
  target_field_name.clone_from(&field.ident);
 } else {
  // Multi-field struct: require #[ deref_mut ] attribute on one field
  for field in &item.fields 
  {
   if attr ::has_deref_mut(field.attrs.iter())? 
   {
  deref_mut_attr_count += 1;
  target_field_type = Some(field.ty.clone());
  target_field_name.clone_from(&field.ident);
 }
 }

  if deref_mut_attr_count == 0 
  {
   return_syn_err!(
  item.span(),
  "DerefMut cannot be derived for multi-field structs without a `#[ deref_mut ]` attribute on one field."
 );
 } else  if deref_mut_attr_count > 1 
  {
   return_syn_err!(item.span(), "Only one field can have the `#[ deref_mut ]` attribute.");
 }
 }

   let field_type =
  target_field_type.ok_or_else(|| syn_err!(item.span(), "Could not determine target field type for DerefMut."))?;
   let field_name = target_field_name;

   generate(
  item_name,
  &generics_impl,
  &generics_ty,
  generics_where_option,
  &field_type,
  field_name.as_ref(),
 )
 }
  StructLike ::Enum(ref item) =>
  {
   return_syn_err!(
  item.span(),
  "DerefMut cannot be derived for enums. It is only applicable to structs with a single field."
 );
 }
 };

  if has_debug 
  {
  let about = format!("derive: DerefMut\nstructure: {item_name}");
  diag ::report_print(about, &original_input, &result);
 }

  Ok(result)
}

/// Generates `DerefMut` implementation for structs.
///
/// Fix(issue-5): Updated signature to use `ImplGenerics`, `TypeGenerics`, and `WhereClause`.
/// Root cause: `Punctuated` types don't format correctly in `quote!` macro for generic impl blocks.
/// Pitfall: Always use `split_for_impl()` types (`ImplGenerics`, `TypeGenerics`, `WhereClause`) for trait implementations.
///
/// Example of generated code :
/// ```text
/// impl< T > ::core ::ops ::DerefMut for IsTransparent< T >
/// {
///   fn deref_mut( &mut self ) -> &mut T
/// ///   {
/// ///     &mut self.0
/// /// }
/// /// }
/// ```
fn generate(
  item_name: &syn ::Ident,
  generics_impl: &syn ::ImplGenerics< '_ >,
  generics_ty: &syn ::TypeGenerics< '_ >,
  generics_where_option: Option< &syn ::WhereClause >,
  field_type: &syn ::Type,
  field_name: Option< &syn ::Ident >,
) -> proc_macro2 ::TokenStream {
  let body =  if let Some(field_name) = field_name
  {
  qt! { &mut self.#field_name }
 } else {
  qt! { &mut self.0 }
 };

  qt! {
  #[ automatically_derived ]
  impl #generics_impl ::core ::ops ::DerefMut for #item_name #generics_ty
  #generics_where_option
  {
   fn deref_mut( &mut self ) -> &mut #field_type
   {
  #body
 }
 }
 }
}
