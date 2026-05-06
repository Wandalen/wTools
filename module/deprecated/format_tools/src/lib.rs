#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/reflect_tools/latest/reflect_tools/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Formatting utilities" ) ]
#![ allow( clippy::similar_names ) ]
#![ allow( clippy::double_parens ) ]
#![ allow( clippy::empty_line_after_doc_comments ) ]
#![ allow( clippy::redundant_else ) ]
#![ allow( clippy::single_match_else ) ]
#![ allow( clippy::needless_late_init ) ]
#![ allow( clippy::match_same_arms ) ]
#![ allow( clippy::implicit_clone ) ]
#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::explicit_iter_loop ) ]
#![ allow( clippy::elidable_lifetime_names ) ]
#![ allow( clippy::needless_borrow ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::doc_lazy_continuation ) ]
#![ allow( clippy::cast_possible_truncation ) ]
#![ allow( clippy::cast_sign_loss ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::unreadable_literal ) ]
#![ allow( clippy::type_complexity ) ]
#![ allow( clippy::default_trait_access ) ]
#![ allow( clippy::missing_errors_doc ) ]
#![ allow( clippy::manual_string_new ) ]
#![ allow( clippy::explicit_counter_loop ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::manual_map ) ]
#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::extra_unused_lifetimes ) ]
#![ allow( clippy::unnecessary_cast ) ]
#![ allow( clippy::redundant_closure ) ]
#![ allow( clippy::needless_borrows_for_generic_args ) ]
#![ allow( clippy::derivable_impls ) ]
#![ allow( clippy::write_with_newline ) ]
#![ allow( clippy::bool_to_int_with_if ) ]
#![ allow( clippy::redundant_static_lifetimes ) ]
#![ allow( clippy::inconsistent_struct_constructor ) ]
#![ allow( clippy::len_zero ) ]
#![ allow( clippy::needless_as_bytes ) ]
#![ allow( clippy::struct_field_names ) ]
#![ allow( clippy::unnecessary_semicolon ) ]
#![ allow( clippy::match_bool ) ]
#![ allow( clippy::implicit_hasher ) ]
#![ allow( clippy::map_identity ) ]
#![ allow( clippy::manual_repeat_n ) ]
#![ allow( clippy::too_many_lines ) ]
#![ allow( clippy::needless_pass_by_value ) ]
#![ allow( clippy::collapsible_else_if ) ]
#![ allow( clippy::needless_return ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::ref_option ) ]
#![ allow( clippy::owned_cow ) ]

#[ cfg( feature = "enabled" ) ]
pub mod format;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::reflect_tools;
  pub use ::former;
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super :: *;

  #[ doc( inline ) ]
  pub use orphan :: *;

  #[ doc( inline ) ]
  pub use super ::format ::own :: *;

  // #[ doc( inline ) ]
  // pub use super ::format ::orphan :: *;

}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super :: *;

  #[ doc( inline ) ]
  pub use super ::format ::orphan :: *;

  #[ doc( inline ) ]
  pub use exposed :: *;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super :: *;

  #[ doc( inline ) ]
  pub use prelude :: *;

  #[ doc( inline ) ]
  pub use super ::format ::exposed :: *;

  #[ doc( inline ) ]
  pub use super ::dependency ::reflect_tools ::
  {
  Fields,
  IteratorTrait,
  _IteratorTrait,
 };

}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super :: *;

  #[ doc( inline ) ]
  pub use super ::format ::prelude :: *;

  // #[ doc( inline ) ]
  // pub use super ::format ::prelude :: *;

}
