#![ allow( missing_docs ) ]

/* xxx : seems declarative macro is not enough to solve the problem */

#[ macro_export ]
macro_rules! select_matching
{
  // ( $src : expr, $else : stmt, { $( $left : pat => $right : expr ),+ $(,)? } ) =>
  (
    $src : expr,
    $else : stmt,
    $( $left : pat => $right : expr ),+ $(,)?
  ) =>
  {
    $crate::select_matching!
    (
      @SRC => $src,
      @ELSE => $else,
      @SELECT_LEFT => $( $left ),+,
      @SELECT_RIGHT => $( $right ),+,
      @BEGIN => [],
      @END => [],
      // @BEGIN => [ match $src { ]
      // @END => [ } ]
    );
  };
  (
    @SRC => $src : expr,
    @ELSE => $else : stmt,
    @SELECT_LEFT => $left1 : pat $( , $left : pat )+,
    @SELECT_RIGHT => $right1 : expr $( , $right : expr )+,
    @BEGIN => [ $( $begin : tt )* ],
    @END => [ $( $end : tt )* ],
  ) =>
  {{

    // $crate::select_matching!
    // (
    //   @SRC => $src,
    //   @ELSE => $else,
    //   @SELECT_LEFT => $( $left ),+,
    //   @SELECT_RIGHT => $( $right ),+,
    //   @BEGIN => [ $( $begin : tt )* $left1 => match $right1 { ],
    //   @END => [ } $( $end : tt )* ],
    // );

    // println!( "yyy" );
    // $left1 => match $right1
    // {
    //   $crate::select_matching!
    //   (
    //     @ELSE => $else,
    //     @SELECT_LEFT => $( $left ),+,
    //     @SELECT_RIGHT => $( $right ),+,
    //   );
    // },
    // _ => $else,
  }};
  (
    @SRC => $src : expr,
    @ELSE => $else : stmt,
    @SELECT_LEFT => $left1 : pat,
    @SELECT_RIGHT => $right1 : expr,
    @BEGIN => [ $( $begin : tt )* ],
    @END => [ $( $end : tt )* ],
  ) =>
  {{
    // println!( "xxx" );
    match $src
    {
      $( $begin )*
      $left1 => $right1,
      _ => { $else },
      $( $end )*
    }
  }};
}
