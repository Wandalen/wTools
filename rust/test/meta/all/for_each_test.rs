use test_tools::*;

#[cfg( feature = "in_wtools" )]
use wtools::meta as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use for_each as TheModule;

impls!
{

  //

  #[ test ]
  fn braces_unwrap_test()
  {
    static mut GOT : String = String::new();
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
      TheModule::braces_unwrap!( dbg, { a, b, c } );
      // generates :
      // dbg!( a, b, c );
      TheModule::braces_unwrap!( dbg, a, b, c );
      // generates :
      // dbg!( a, b, c );
    }

    /* test.case( "sample2" ) */
    {
      let ( prefix, a, b, c, postfix ) = ( "prefix", 1, 2, 3, "postfix" );
      TheModule::braces_unwrap!
      (
        dbg where
        @PREFIX{ prefix, }
        @POSTFIX{ postfix }
        @SRC{ { a, b, c, } }
      );
      // generates :
      // dbg!( prefix, a, b, c, psotfix );
      TheModule::braces_unwrap!
      (
        dbg where
        @PREFIX{ prefix, }
        @POSTFIX{ postfix }
        @SRC{ a, b, c, }
      );
      // generates :
      // dbg!( prefix, a, b, c, psotfix );
    }

    /* test.case( "function-style" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::braces_unwrap!( test_with, a, b, c );
      let exp = "a, b, c;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!( test_with, { a, b, c } );
      let exp = "a, b, c;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!( test_with, { { a, b, c } } );
      let exp = "{ a, b, c };";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!( test_with, ( a, b, c ) );
      let exp = "(a, b, c);";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!( test_with, ( ( a, b, c ) ) );
      let exp = "((a, b, c));";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!( test_with, [ a, b, c ] );
      let exp = "[a, b, c];";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!( test_with, [ [ a, b, c ] ] );
      let exp = "[[a, b, c]];";
      assert_eq!( GOT, exp );

    }

    /* test.case( "map-style" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @SRC{ a, b, c }
      );
      let exp = "a, b, c;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @SRC{ { { a, b, c } } }
      );
      let exp = "{ a, b, c };";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @SRC{ ( a, b, c ) }
      );
      let exp = "(a, b, c);";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "((a, b, c));";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @SRC{ [ a, b, c ] }
      );
      let exp = "[a, b, c];";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "[[a, b, c]];";
      assert_eq!( GOT, exp );
    }

    /* test.case( "prefix and postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ { { a, b, c } } }
      );
      let exp = "prefix { a, b, c } postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ ( a, b, c ) }
      );
      let exp = "prefix(a, b, c) postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "prefix((a, b, c)) postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ [ a, b, c ] }
      );
      let exp = "prefix [a, b, c] postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "prefix [[a, b, c]] postfix;";
      assert_eq!( GOT, exp );

    }

    /* test.case( "prefix and postfix unwrapping" ) */
    unsafe
    {
      /* 0 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ { prefix } }
        @POSTFIX{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 1 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ { prefix } }
        @POSTFIX{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 2 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ { prefix } }
        @POSTFIX{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 3 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ { prefix } }
        @POSTFIX{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 4 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 5 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ { postfix } }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 6 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 7 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @POSTFIX{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c postfix;";
      assert_eq!( GOT, exp );
    }

    /* test.case( "prefix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ { { a, b, c } } }
      );
      let exp = "prefix { a, b, c };";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ ( a, b, c ) }
      );
      let exp = "prefix(a, b, c);";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "prefix((a, b, c));";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ [ a, b, c ] }
      );
      let exp = "prefix [a, b, c];";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "prefix [[a, b, c]];";
      assert_eq!( GOT, exp );

    }

    /* test.case( "prefix unwrapping" ) */
    unsafe
    {
      /* 0 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ { prefix } }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c;";
      assert_eq!( GOT, exp );
      /* 1 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ { prefix } }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c;";
      assert_eq!( GOT, exp );
      /* 2 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ { a, b, c } }
      );
      let exp = "prefix a, b, c;";
      assert_eq!( GOT, exp );
      /* 3 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @PREFIX{ prefix }
        @SRC{ a, b, c }
      );
      let exp = "prefix a, b, c;";
      assert_eq!( GOT, exp );
    }

    /* test.case( "postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "a, b, c postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ { { a, b, c } } }
      );
      let exp = "{ a, b, c } postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ ( a, b, c ) }
      );
      let exp = "(a, b, c) postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ ( ( a, b, c ) ) }
      );
      let exp = "((a, b, c)) postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ [ a, b, c ] }
      );
      let exp = "[a, b, c] postfix;";
      assert_eq!( GOT, exp );

      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ [ [ a, b, c ] ] }
      );
      let exp = "[[a, b, c]] postfix;";
      assert_eq!( GOT, exp );

    }

    /* test.case( "postfix unwrapping" ) */
    unsafe
    {
      /* 0 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ { postfix } }
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 1 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ { postfix } }
        @SRC{ a, b, c }
      );
      let exp = "a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 2 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ { a, b, c } }
      );
      let exp = "a, b, c postfix;";
      assert_eq!( GOT, exp );
      /* 3 */
      GOT = "".to_string();
      TheModule::braces_unwrap!
      (
        test_with where
        @POSTFIX{ postfix }
        @SRC{ a, b, c }
      );
      let exp = "a, b, c postfix;";
      assert_eq!( GOT, exp );
    }

  }

  ///
  /// Tests macro crate::for_each!().
  ///

  #[ test ]
  fn for_each_test()
  {

    macro_rules! test_with
    {
      (
        $( $Arg : tt )*
      ) =>
      {{
        GOT += stringify!( $( $Arg )* );
        GOT += "+";
      }};
    }

    static mut GOT : String = String::new();

    /* test.case( "sample : function-style" ) */
    {
      TheModule::for_each!( dbg, "a", "b", "c" );
      // generates
      dbg!( "a" );
      dbg!( "b" );
      dbg!( "c" );
    }

    /* test.case( "sample : map-style" ) */
    {
      TheModule::for_each!
      {
        dbg where
        @PREFIX { "prefix".to_string() + }
        @POSTFIX { + "postfix" }
        @EACH "a" "b" "c"
      };
      // generates
      dbg!( "prefix".to_string() + "a" + "postfix" );
      dbg!( "prefix".to_string() + "b" + "postfix" );
      dbg!( "prefix".to_string() + "c" + "postfix" );
    }

    /* test.case( "sample : more than single token" ) */
    {
      TheModule::for_each!
      {
        dbg where
        @PREFIX { "prefix".to_string() + }
        @POSTFIX { + "postfix" }
        @EACH { "a" + "1" } { "b" + "2" } { "c" + "3" }
      };
      // generates
      dbg!( "prefix".to_string() + "a" + "1" + "postfix" );
      dbg!( "prefix".to_string() + "b" + "2" + "postfix" );
      dbg!( "prefix".to_string() + "c" + "3" + "postfix" );
    }

    /* test.case( "sample : callbackless" ) */
    {
      TheModule::for_each!
      {
        @PREFIX { dbg! }
        @EACH ( "a" ) ( "b" ) ( "c" )
      };
      // generates
      dbg!( "a" );
      dbg!( "b" );
      dbg!( "c" );
    }

    // function-style

    /* test.case( "function-style" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with, a, b, c );
      let exp = "a+b+c+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "function-style, paths, unwrapping" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with, { std::collections::HashMap }, { std::collections::BTreeMap } );
      let exp = "std :: collections :: HashMap+std :: collections :: BTreeMap+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "function-style, complex, unwrapping" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with, { a _ a }, { b _ b } );
      let exp = "a _ a+b _ b+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "function-style, complex, unwrapping, trailing comma" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with, { a _ a }, { b _ b }, );
      let exp = "a _ a+b _ b+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "function-style, paths, parentheses" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with, ( std::collections::HashMap ), ( std::collections::BTreeMap ) );
      let exp = "(std :: collections :: HashMap)+(std :: collections :: BTreeMap)+";
      assert_eq!( GOT, exp );
    }

    // callbackless

    /* test.case( "callbackless, prefix, postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        @PREFIX { test_with! }
        @POSTFIX { ; test_with!( postfix ); }
        @EACH ( a ) ( b ) ( c )
      };
      let exp = "a+postfix+b+postfix+c+postfix+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "callbackless, prefix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        @PREFIX { test_with! }
        @EACH ( a ) ( b ) ( c )
      };
      let exp = "a+b+c+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "callbackless, postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        @POSTFIX { ; test_with!( postfix ); }
        @EACH { test_with!( a ) } { test_with!( b ) } { test_with!( c ) }
      };
      let exp = "a+postfix+b+postfix+c+postfix+";
      assert_eq!( GOT, exp );
    }

    // map-style

    /* test.case( "map-style" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with where @EACH a b c );
      let exp = "a+b+c+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "map-style, prefix + postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with where @PREFIX prefix @POSTFIX postfix @EACH a b c );
      let exp = "prefix a postfix+prefix b postfix+prefix c postfix+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "map-style, prefix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with where @PREFIX prefix @EACH a b c );
      let exp = "prefix a+prefix b+prefix c+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "map-style, postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!( test_with where @POSTFIX postfix @EACH a b c );
      let exp = "a postfix+b postfix+c postfix+";
      assert_eq!( GOT, exp );
    }

    // map-style, complex

    /* test.case( "map-style" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        test_with where
        @EACH { a _ a } { b _ b } { c _ c }
      };
      let exp = "a _ a+b _ b+c _ c+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "map-style, prefix + postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        test_with where
        @PREFIX { pre fix }
        @POSTFIX { post fix }
        @EACH { a _ a } { b _ b } { c _ c }
      };
      let exp = "pre fix a _ a post fix+pre fix b _ b post fix+pre fix c _ c post fix+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "map-style, prefix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        test_with where
        @PREFIX { pre fix }
        @EACH { a _ a } { b _ b } { c _ c }
      };
      let exp = "pre fix a _ a+pre fix b _ b+pre fix c _ c+";
      assert_eq!( GOT, exp );
    }

    /* test.case( "map-style, postfix" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        test_with where
        @POSTFIX { post fix }
        @EACH { a _ a } { b _ b } { c _ c }
      };
      let exp = "a _ a post fix+b _ b post fix+c _ c post fix+";
      assert_eq!( GOT, exp );
    }

  }

  ///
  /// Higher order cases
  ///

  #[ test ]
  fn for_each_higher_order_test()
  {
    static mut GOT : String = String::new();
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
        TheModule::for_each!
        (
          $Callback where
          $( $( $Args )* )?
          @EACH f32 f64
        );
      };

    }

    /* test.case( "manual" ) */
    unsafe
    {
      GOT = "".to_string();
      for_each_float!( test_with where @PREFIX { pre fix 1 } @POSTFIX { post fix } );
      for_each_float!( test_with where @PREFIX { pre fix 2 } @POSTFIX { post fix } );
      let exp = "pre fix 1 f32 post fix;pre fix 1 f64 post fix;pre fix 2 f32 post fix;pre fix 2 f64 post fix;";
      assert_eq!( GOT, exp );
    }

    /* test.case( "without fixes" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        for_each_float where
        @EACH
          { test_with where @PREFIX { pre fix 1 } @POSTFIX { post fix } }
          { test_with where @PREFIX { pre fix 2 } @POSTFIX { post fix } }
      }
      let exp = "pre fix 1 f32 post fix;pre fix 1 f64 post fix;pre fix 2 f32 post fix;pre fix 2 f64 post fix;";
      assert_eq!( GOT, exp );
    }

    /* test.case( "without fixes" ) */
    unsafe
    {
      GOT = "".to_string();
      TheModule::for_each!
      {
        for_each_float where
        @PREFIX { test_with where @PREFIX }
        @POSTFIX { @POSTFIX { post fix } }
        @EACH
          { { pre fix 1 } }
          { { pre fix 2 } }
      }
      let exp = "pre fix 1 f32 post fix;pre fix 1 f64 post fix;pre fix 2 f32 post fix;pre fix 2 f64 post fix;";
      assert_eq!( GOT, exp );
    }

  }

}

//

index!
{
  braces_unwrap_test,
  for_each_test,
  for_each_higher_order_test,
}
