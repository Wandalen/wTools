
use super::*;

mod as_path_test;
mod try_into_path_test;
mod try_into_cow_path_test;

mod absolute_path_test;
mod path_join_fn_test;
mod path_join_trait_test;

mod current_path;
mod path_canonicalize;
mod path_change_ext;
mod path_common;
mod path_ext;
mod path_exts;
mod path_is_glob;
mod path_normalize;
mod path_relative;
mod rebase_path;
mod transitive;
mod without_ext;

#[ cfg( feature = "path_unique_folder_name" ) ]
mod path_unique_folder_name;
