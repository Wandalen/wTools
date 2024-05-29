use super::*;

tests_impls!
{

  //

  fn braces_unwrap_test()
  {
    // let mut GOT : String = String::new();
    let mut GOT : String = String::new();
    macro_rules! test_with
    {
      (
        $( $Arg : tt )*
      ) =>
      {{
        GOT += stringify!( $( $Arg )* );
        GOT += ";";
      }};
    }

    /* test.case( "sample1" ) */
    {
      let ( a, b, c ) = ( 1, 2, 3 );
      the_module::braces_unwrap!( dbg, { a, b, c } );
      // generates :
      // dbg!( a, b, c );
      the_module::braces_unwrap!( dbg, a, b, c );
      // generates :
      // dbg!( a, b, c );
    }

    /* test.case( "sample2" ) */
    {
      let ( prefix, a, b, c, postfix ) = ( "prefix", 1, 2, 3, "postfix" );
      the_module::braces_unwrap!
      (
        dbg where
        @Prefix{ prefix, }
        @Postfix{ postfix }
        @SRC{ { a, b, c, } }
      );
      // generates :
      // dbg!( prefix, a, b, c, psotfix );
      the_module::braces_unwrap!
      (
        dbg where
        @Prefix{ prefix, }
        @Postfix{ postfix }
        @SRC{ a, b, c, }
      );
      // generates :
      // dbg!( prefix, a, b, c, psotfix );
    }

    /* test.case( "function-style" ) */

    {
      GOT = "".to_string();
      the_module::braces_unwrap!( test_with, a, b, c );
      let exp = "a, b, c;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!( test_with, { a, b, c } );
      let exp = "a, b, c;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!( test_with, { { a, b, c } } );
      let exp = "{ a, b, c };";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!( test_with, ( a, b, c ) );
      let exp = "(a, b, c);";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!( test_with, ( ( a, b, c ) ) );
      let exp = "((a, b, c));";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!( test_with, [ a, b, c ] );
      let exp = "[a, b, c];";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!( test_with, [ [ a, b, c ] ] );
      let exp = "[[a, b, c]];";
      a_id!( GOT, exp );

    }

    /* test.case( "map-style" ) */

    {
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @SRC{ a, b, c }
      );
      let exp = "a, b, c;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @SRC{ { { a, b, c } } }
      );
      let exp = "{ a, b, c };";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @SRC{ ( a, b, c ) }
      );
      let exp = "(a, b, c);";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "((a, b, c));";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @SRC{ [ a, b, c ] }
      );
      let exp = "[a, b, c];";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "[[a, b, c]];";
      a_id!( GOT, exp );
    }

    /* test.case( "prefix and postfix" ) */

    {
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ { { a, b, c } } }
      );
      let exp = "prefix { a, b, c } postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ ( a, b, c ) }
      );
      let exp = "prefix(a, b, c) postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "prefix((a, b, c)) postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ [ a, b, c ] }
      );
      let exp = "prefix [a, b, c] postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "prefix [[a, b, c]] postfix;";
      a_id!( GOT, exp );

    }

    /* test.case( "prefix and postfix unwrapping" ) */

    {
      /* 0 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ { prefix } }
        @Postfix{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
      /* 1 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ { prefix } }
        @Postfix{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
      /* 2 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ { prefix } }
        @Postfix{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
      /* 3 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ { prefix } }
        @Postfix{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
      /* 4 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
      /* 5 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ { postfix } }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
      /* 6 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
      /* 7 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @Postfix{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c postfix;";
      a_id!( GOT, exp );
    }

    /* test.case( "prefix" ) */

    {
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ { { a, b, c } } }
      );
      let exp = "prefix { a, b, c };";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ ( a, b, c ) }
      );
      let exp = "prefix(a, b, c);";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "prefix((a, b, c));";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ [ a, b, c ] }
      );
      let exp = "prefix [a, b, c];";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "prefix [[a, b, c]];";
      a_id!( GOT, exp );

    }

    /* test.case( "prefix unwrapping" ) */

    {
      /* 0 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ { prefix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c;";
      a_id!( GOT, exp );
      /* 1 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ { prefix } }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c;";
      a_id!( GOT, exp );
      /* 2 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c;";
      a_id!( GOT, exp );
      /* 3 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Prefix{ prefix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c;";
      a_id!( GOT, exp );
    }

    /* test.case( "postfix" ) */

    {
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "a, b, c postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ { { a, b, c } } }
      );
      let exp = "{ a, b, c } postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ ( a, b, c ) }
      );
      let exp = "(a, b, c) postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "((a, b, c)) postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ [ a, b, c ] }
      );
      let exp = "[a, b, c] postfix;";
      a_id!( GOT, exp );

      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "[[a, b, c]] postfix;";
      a_id!( GOT, exp );

    }

    /* test.case( "postfix unwrapping" ) */

    {
      /* 0 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c postfix;";
      a_id!( GOT, exp );
      /* 1 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ { postfix } }
        @SRC{ a, b, c }
      );
      let exp = "a, b, c postfix;";
      a_id!( GOT, exp );
      /* 2 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c postfix;";
      a_id!( GOT, exp );
      /* 3 */
      GOT = "".to_string();
      the_module::braces_unwrap!
      (
        test_with where
        @Postfix{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "a, b, c postfix;";
      a_id!( GOT, exp );
    }

  }

  ///
  /// Tests macro crate::for_each!().
  ///


  fn for_each_test()
  {
    let mut GOT : String = String::new();

    macro_rules! test_with
    {
      (
        $( $Arg:tt )*
      ) =>
      {{
        GOT += stringify!( $( $Arg )* );
        GOT += "+";
      }};
    }

    /* test.case( "sample : function-style" ) */
    {
      the_module::for_each!( dbg, "a", "b", "c" );
      // generates
      dbg!( "a" );
      dbg!( "b" );
      dbg!( "c" );
    }

    /* test.case( "sample : map-style" ) */
    {
      the_module::for_each!
      {
        dbg where
        @Prefix { "prefix".to_string() + }
        @Postfix { + "postfix" }
        @Each "a" "b" "c"
      };
      // generates
      dbg!( "prefix".to_string() + "a" + "postfix" );
      dbg!( "prefix".to_string() + "b" + "postfix" );
      dbg!( "prefix".to_string() + "c" + "postfix" );
    }

    /* test.case( "sample : more than single token" ) */
    {
      the_module::for_each!
      {
        dbg where
        @Prefix { "prefix".to_string() + }
        @Postfix { + "postfix" }
        @Each { "a" + "1" } { "b" + "2" } { "c" + "3" }
      };
      // generates
      dbg!( "prefix".to_string() + "a" + "1" + "postfix" );
      dbg!( "prefix".to_string() + "b" + "2" + "postfix" );
      dbg!( "prefix".to_string() + "c" + "3" + "postfix" );
    }

    /* test.case( "sample : callbackless" ) */
    {
      the_module::for_each!
      {
        @Prefix { dbg! }
        @Each ( "a" ) ( "b" ) ( "c" )
      };
      // generates
      dbg!( "a" );
      dbg!( "b" );
      dbg!( "c" );
    }

    // function-style

    /* test.case( "function-style" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with, a, b, c );
      let exp = "a+b+c+";
      a_id!( GOT, exp );
    }

    /* test.case( "function-style, paths, unwrapping" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with, { std :: collections :: HashMap }, { std :: collections :: BTreeMap } );
      let exp = "std :: collections :: HashMap+std :: collections :: BTreeMap+";
      a_id!( GOT, exp );
    }

    /* test.case( "function-style, complex, unwrapping" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with, { a _ a }, { b _ b } );
      let exp = "a _ a+b _ b+";
      a_id!( GOT, exp );
    }

    /* test.case( "function-style, complex, unwrapping, trailing comma" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with, { a _ a }, { b _ b }, );
      let exp = "a _ a+b _ b+";
      a_id!( GOT, exp );
    }

    /* test.case( "function-style, paths, parentheses" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with, ( std :: collections :: HashMap ), ( std :: collections :: BTreeMap ) );
      let exp = "(std :: collections :: HashMap)+(std :: collections :: BTreeMap)+";
      a_id!( GOT, exp );
    }

    // callbackless

    /* test.case( "callbackless, prefix, postfix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        @Prefix { test_with! }
        @Postfix { ; test_with!( postfix ); }
        @Each ( a ) ( b ) ( c )
      };
      let exp = "a+postfix+b+postfix+c+postfix+";
      a_id!( GOT, exp );
    }

    /* test.case( "callbackless, prefix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        @Prefix { test_with! }
        @Each ( a ) ( b ) ( c )
      };
      let exp = "a+b+c+";
      a_id!( GOT, exp );
    }

    /* test.case( "callbackless, postfix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        @Postfix { ; test_with!( postfix ); }
        @Each { test_with!( a ) } { test_with!( b ) } { test_with!( c ) }
      };
      let exp = "a+postfix+b+postfix+c+postfix+";
      a_id!( GOT, exp );
    }

    // map-style

    /* test.case( "map-style" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with where @Each a b c );
      let exp = "a+b+c+";
      a_id!( GOT, exp );
    }

    /* test.case( "map-style, prefix + postfix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with where @Prefix prefix @Postfix postfix @Each a b c );
      let exp = "prefix a postfix+prefix b postfix+prefix c postfix+";
      a_id!( GOT, exp );
    }

    /* test.case( "map-style, prefix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with where @Prefix prefix @Each a b c );
      let exp = "prefix a+prefix b+prefix c+";
      a_id!( GOT, exp );
    }

    /* test.case( "map-style, postfix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!( test_with where @Postfix postfix @Each a b c );
      let exp = "a postfix+b postfix+c postfix+";
      a_id!( GOT, exp );
    }

    // map-style, complex

    /* test.case( "map-style" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        test_with where
        @Each { a _ a } { b _ b } { c _ c }
      };
      let exp = "a _ a+b _ b+c _ c+";
      a_id!( GOT, exp );
    }

    /* test.case( "map-style, prefix + postfix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        test_with where
        @Prefix { pre fix }
        @Postfix { post fix }
        @Each { a _ a } { b _ b } { c _ c }
      };
      let exp = "pre fix a _ a post fix+pre fix b _ b post fix+pre fix c _ c post fix+";
      a_id!( GOT, exp );
    }

    /* test.case( "map-style, prefix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        test_with where
        @Prefix { pre fix }
        @Each { a _ a } { b _ b } { c _ c }
      };
      let exp = "pre fix a _ a+pre fix b _ b+pre fix c _ c+";
      a_id!( GOT, exp );
    }

    /* test.case( "map-style, postfix" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        test_with where
        @Postfix { post fix }
        @Each { a _ a } { b _ b } { c _ c }
      };
      let exp = "a _ a post fix+b _ b post fix+c _ c post fix+";
      a_id!( GOT, exp );
    }

  }

  ///
  /// Higher order cases
  ///


  fn for_each_higher_order_test()
  {
    let mut GOT : String = String::new();
    macro_rules! test_with
    {
      (
        $( $Arg : tt )*
      ) =>
      {{
        GOT += stringify!( $( $Arg )* );
        GOT += ";";
      }};
    }

    macro_rules! for_each_float
    {

      (
        $Callback : path
        $( where $( $Args : tt )* )?
      ) =>
      {
        the_module::for_each!
        (
          $Callback where
          $( $( $Args )* )?
          @Each f32 f64
        );
      };

    }

    /* test.case( "manual" ) */

    {
      GOT = "".to_string();
      for_each_float!( test_with where @Prefix { pre fix 1 } @Postfix { post fix } );
      for_each_float!( test_with where @Prefix { pre fix 2 } @Postfix { post fix } );
      let exp = "pre fix 1 f32 post fix;pre fix 1 f64 post fix;pre fix 2 f32 post fix;pre fix 2 f64 post fix;";
      a_id!( GOT, exp );
    }

    /* test.case( "without fixes" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        for_each_float where
        @Each
          { test_with where @Prefix { pre fix 1 } @Postfix { post fix } }
          { test_with where @Prefix { pre fix 2 } @Postfix { post fix } }
      }
      let exp = "pre fix 1 f32 post fix;pre fix 1 f64 post fix;pre fix 2 f32 post fix;pre fix 2 f64 post fix;";
      a_id!( GOT, exp );
    }

    /* test.case( "without fixes" ) */

    {
      GOT = "".to_string();
      the_module::for_each!
      {
        for_each_float where
        @Prefix { test_with where @Prefix }
        @Postfix { @Postfix { post fix } }
        @Each
          { { pre fix 1 } }
          { { pre fix 2 } }
      }
      let exp = "pre fix 1 f32 post fix;pre fix 1 f64 post fix;pre fix 2 f32 post fix;pre fix 2 f64 post fix;";
      a_id!( GOT, exp );
    }

  }

}

//

tests_index!
{
  braces_unwrap_test,
  for_each_test,
  for_each_higher_order_test,
}
