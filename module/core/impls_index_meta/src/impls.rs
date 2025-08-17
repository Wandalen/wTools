extern crate alloc;
use macro_tools::
{
  proc_macro2::TokenStream,
  quote,
  quote::ToTokens,
  syn,
  syn::
  {
    parse::{ Parse, ParseStream },
    Result, // Use syn's Result directly
    Token,
    Item,
    spanned::Spanned, // Import Spanned trait for error reporting
  },
};
use core::fmt; // Import fmt for manual Debug impl if needed
use alloc::vec::IntoIter; // Use alloc instead of std

// --- Local replacements for macro_tools types/traits ---

/// Marker trait used to indicate how to parse multiple elements.
trait AsMuchAsPossibleNoDelimiter {}

/// Wrapper for parsing multiple elements.
// No derive(Debug) here as T might not implement Debug
pub struct Many<T: ToTokens>(pub Vec< T >);

// Manual Debug implementation for Many<T> if T implements Debug
impl<T> fmt::Debug for Many<T>
where
  T: ToTokens + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("Many").field(&self.0).finish()
  }
}

impl<T> Many<T>
where
  T: ToTokens,
{
  /// Iterator over the contained elements.
  pub fn iter(&self) -> core::slice::Iter<'_, T> {
    self.0.iter()
  }
}

impl<T> IntoIterator for Many<T>
where
  T: ToTokens,
{
  type Item = T;
  type IntoIter = IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a Many<T>
where
  T: ToTokens,
{
  type Item = &'a T;
  type IntoIter = core::slice::Iter<'a, T>;
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<T> quote::ToTokens for Many<T>
where
  T: ToTokens,
{
  fn to_tokens(&self, tokens: &mut TokenStream) {
    for item in &self.0 {
      item.to_tokens(tokens);
    }
  }
}

// --- Original code adapted ---

///
/// Module-specific item.
/// Represents an optional `?` followed by a `syn::Item`.
///
// Removed #[ derive( Debug ) ]
pub struct Item2 {
  pub optional: Option< Token![ ? ] >,
  pub func: syn::Item,
}

// Manual Debug implementation for Item2
impl fmt::Debug for Item2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct( "Item2" )
         .field( "optional", &self.optional.is_some() ) // Debug only if present
         .field( "func", &self.func.to_token_stream().to_string() ) // Debug func as string
         .finish()
  }
}

// Implement the marker trait for Item2 to use in Many's parse impl.
impl AsMuchAsPossibleNoDelimiter for Item2 {}

impl Parse for Item2 {
  fn parse(input: ParseStream<'_>) -> Result< Self > {
    // Look for an optional '?' token first
    let optional: Option< Token![ ? ] > = input.parse()?;

    // Parse the item (expected to be a function, but we parse Item for flexibility)
    let func: Item = input.parse()?;

    // Ensure the parsed item is a function
    if !matches!(func, Item::Fn(_)) {
      // Use spanned for better error location
      return Err(syn::Error::new(func.span(), "Expected a function item"));
    }

    Ok(Self { optional, func })
  }
}

impl ToTokens for Item2 {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    self.optional.to_tokens(tokens);
    self.func.to_tokens(tokens);
  }
}

// No derive(Debug) here as Item2 does not derive Debug anymore
pub struct Items2(pub Many<Item2>);

// Manual Debug implementation for Items2
impl fmt::Debug for Items2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("Items2").field(&self.0).finish()
  }
}

// Implement Parse for Many<Item2> specifically
// because Item2 implements AsMuchAsPossibleNoDelimiter
impl<T> Parse for Many<T>
where
  T: Parse + ToTokens + AsMuchAsPossibleNoDelimiter,
{
  fn parse(input: ParseStream<'_>) -> Result< Self > {
    let mut items = Vec::new();
    // Continue parsing as long as the input stream is not empty
    while !input.is_empty() {
      // Parse one element of type T
      let item: T = input.parse()?;
      items.push(item);
    }
    Ok(Self(items))
  }
}

impl Parse for Items2 {
  fn parse(input: ParseStream<'_>) -> Result< Self > {
    let many: Many<Item2> = input.parse()?;
    Ok(Self(many))
  }
}

impl ToTokens for Items2 {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    self.0.iter().for_each(|e| {
      // Extract the function item specifically
      let Item::Fn(func) = &e.func else {
        panic!(
          "Internal error: Item2 should always contain a function item at {:?}",
          e.func.span()
        )
      };

      // Get the function name identifier
      let name_ident = &func.sig.ident;

      // Construct the macro definition
      let declare_aliased = quote! {
        ( as $Name2 : ident ) =>
        {
          // Note: impls_index::fn_rename! is external, assuming it exists
          impls_index::fn_rename!
          {
            @Name { $Name2 }
            @Fn
            {
              #func // Use the full function item here
            }
          }
        };
      };

      let mut mandatory = quote! {
        #[ allow( unused_macros ) ]
      };

      if e.optional.is_none() {
        mandatory = quote! {
          #[ deny( unused_macros ) ]
        }
      }

      let result = quote! {
        #mandatory
        macro_rules! #name_ident // Use the original function identifier
        {
          #declare_aliased
          () =>
          {
            #func // Use the full function item here
          };
        }
      };
      result.to_tokens(tokens);
    });
  }
}

pub fn impls(input: proc_macro::TokenStream) -> Result< TokenStream > {
  let items2: Items2 = syn::parse(input)?;

  let result = quote! {
    #items2
  };

  Ok(result)
}
