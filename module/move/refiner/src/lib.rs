#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wcensor/latest/wcensor/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

::meta_tools::mod_interface!
{
  /// Result of parsing.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer instruction;
  /// Properties parsing.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer props;
}
