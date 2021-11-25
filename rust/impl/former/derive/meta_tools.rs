
#[allow(unused_macros)]
macro_rules! tree_print
{
  ( $src : expr ) =>
  {{
    println!( "{}", tree_export_str!( $src ) );
  }};
  ( $( $src : expr ),+ $(,)? ) =>
  {{
    $( tree_print!( $src ) );+;
  }};
}

#[allow(unused_macros)]
macro_rules! tree_export_str
{
  ( $src : expr ) =>
  {{
    let src2 = &$src;
    format!( "{} : {} :\n{:#?}", stringify!( $src ), quote!{ #src2 }, $src )
  }};
}
