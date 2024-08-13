//! To avoid messing up with long logical expressions in the codebase.

use cfg_aliases::cfg_aliases;

fn main()
{
  // Setup cfg aliases
  cfg_aliases!
  {
    // Platforms
    // wasm : { target_arch = "wasm32" },
    // android : { target_os = "android" },
    // macos : { target_os = "macos" },
    // linux : { target_os = "linux" },
    all_derives:
    {
      all
      (
        feature = "derive_as_mut",
        feature = "derive_as_ref",
        feature = "derive_deref",
        feature = "derive_deref_mut",
        feature = "derive_from",
        feature = "derive_index",
        feature = "derive_index_mut",
        feature = "derive_inner_from",
        feature = "derive_variadic_from",
        feature = "derive_not",
        feature = "derive_reflect",
        feature = "derive_phantom"
      )
    },
    any_derive :
    {
      any
      (
        feature = "derive_as_mut",
        feature = "derive_as_ref",
        feature = "derive_deref",
        feature = "derive_deref_mut",
        feature = "derive_from",
        feature = "derive_index",
        feature = "derive_index_mut",
        feature = "derive_inner_from",
        feature = "derive_variadic_from",
        feature = "derive_not",
        feature = "derive_reflect",
        feature = "derive_phantom"
      )
    },
  }
}
