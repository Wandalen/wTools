( function _l0_l9_ArraySet_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _ = _global_.wTools;

// --
// array set
// --

// function arraySetDiff( test )
// {
//
//   test.case = 'first argument has single extra element, second argument has single extra element either';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var got = _.arraySetDiff( a, b );
//   var expected = [ 15, 5 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3, 4 ];
//   var got = _.arraySetDiff( a, b );
//   var expected = [ 1, 2, 3, 4 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'second argument is an empty array';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [];
//   var got = _.arraySetDiff( a, b );
//   var expected = [ 1, 2, 3, 4 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'both arguments are empty arrays';
//   var a = [];
//   var b = [];
//   var got = _.arraySetDiff( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'all of the elements is present in both arrays';
//   var a = [ 3, 3, 3 ];
//   var b = [ 3, 3, 3, 3 ];
//   var got = _.arraySetDiff( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   /* */
//
//   test.case = 'extra';
//
//   var cases =
//   [
//     { src1 : [], src2 : [], expected : [] },
//     { src1 : [ 1, 2, 3 ], src2 : [], expected : [ 1, 2, 3 ] },
//     { src1 : [], src2 : [ 1, 2, 3 ], expected : [ 1, 2, 3 ] },
//     { src1 : [ 1, 2, 3 ], src2 : [ 4, 5, 6 ], expected : [ 1, 2, 3, 4, 5, 6 ] },
//     { src1 : [ 1, 2, 3 ], src2 : [ 3, 4, 5 ], expected : [ 1, 2, 4, 5 ] },
//     { src1 : [ 1, 1, 2, 2, 3, 3 ], src2 : [ 1, 2, 3 ], expected : [] },
//     { src1 : [ 1, 1, 2, 3, 3 ], src2 : [ 3, 3, 4, 5, 5 ], expected : [ 1, 1, 2, 4, 5, 5 ] },
//     { src1 : 1, src2 : 1, error : true },
//   ]
//
//   for( var i = 0; i < cases.length; i++ )
//   {
//     var c = cases[ i ];
//
//     if( c.error )
//     test.shouldThrowErrorSync( () => _.arraySetDiff( c.src1, c.src2 ) );
//     else
//     test.identical( _.arraySetDiff( c.src1, c.src2 ), c.expected );
//   }
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff();
//   });
//
//   test.case = 'too few arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( [ 1, 2, 3, 4 ] );
//   });
//
//   test.case = 'too many arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( [ 1, 2, 3, 4 ], [ 5, 7, 8, 9 ], [ 13, 15, 17 ] );
//   });
//
//
//   test.case = 'one or both arguments are not longIs entities, numeric arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( 10, 15 );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, string like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( 'a', 'c' );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, map arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( { a : 1 }, { b : 3, c : 8 } );
//   });
//
//   test.case = 'wrong argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( [ 1, 2, 3 ], 'wrong argument' );
//   });
//
//   test.case = 'both arguments are null';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( null, null );
//   });
//
//   test.case = 'both arguments are undefined';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetDiff( undefined, undefined );
//   });
//
// }

//

function diff_( test )
{
  test.open( 'array' );

  test.case = 'empty arrays';
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'src2 - empty array';
  var src1 = [ 1, 2, 3 ];
  var src2 = [];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'src1 - empty array';
  var src1 = [];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'src1 and src2 is not empty array, without duplicates';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'src1 and src2 is not empty array, duplicates';
  var src1 = [ 1, 1, 2, 3, 3 ];
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 1, 2, 4, 5, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'all of the elements is present in both arrays';
  var src1 = [ 3, 3, 3 ];
  var src2 = [ 3, 3, 3, 3 ];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'one evaluator';
  var src1 = [ [ 1 ], 1, 2, [ 3 ], 3 ];
  var src2 = [ [ 3 ], 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( src1, src2, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ] ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'two evaluators';
  var src1 = [ 1, 1, 2, [ 3 ], 3 ];
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( src1, src2, ( e ) => e[ 0 ], ( ins ) => ins );
  var exp = [ 1, 1, 2, [ 3 ], 3, 4, 5, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'equalizer';
  var src1 = [ 1, 1, 2, [ 3 ], 3 ];
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( src1, src2, ( e, ins ) => e[ 0 ] === ins );
  var exp = [ 1, 1, 2, [ 3 ], 3, 4, 5, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'dst - not empty, src1 and src2 - empty arrays';
  var dst = [ 1, 2 ];
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src2 - empty array';
  var dst = [ 1 ];
  var src1 = [ 1, 2, 3 ];
  var src2 = [];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 1, 2, 3 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1 - empty array';
  var dst = [ 2 ];
  var src1 = [];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 2, 1, 2, 3 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1 and src2 is not empty array, without duplicates';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1 and src2 is not empty array, duplicates';
  var dst = new Set( [ 0 ] );
  var src1 = [ 1, 1, 2, 3, 3 ];
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 0, 1, 2, 4, 5 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, all of the elements is present in both arrays';
  var dst = [ 1 ];
  var src1 = [ 3, 3, 3 ];
  var src2 = [ 3, 3, 3, 3 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'one evaluator';
  var dst = [ 1 ];
  var src1 = [ [ 1 ], 1, 2, [ 3 ], 3 ];
  var src2 = [ [ 3 ], 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ] );
  var exp = [ 1, [ 1 ] ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'two evaluators';
  var dst = [];
  var src1 = [ 1, 1, 2, [ 3 ], 3 ];
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ], ( ins ) => ins );
  var exp = [ 1, 1, 2, [ 3 ], 3, 4, 5, 5 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'equalizer';
  var dst = [];
  var src1 = [ 1, 1, 2, [ 3 ], 3 ];
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e, ins ) => e[ 0 ] === ins );
  var exp = [ 1, 1, 2, [ 3 ], 3, 4, 5, 5 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'dst - src1, src1 and src2 - empty arrays';
  var dst = [];
  var src1 = dst;
  var src2 = [];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst - src2, src2 - empty array';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = dst;
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got !== src1 );
  test.true( got === src2 );
  test.identical( got, exp );

  test.case = 'dst - src1, src1 - empty array';
  var dst = [];
  var src1 = dst;
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst - src2, src1 and src2 is not empty array, without duplicates';
  var dst = [ 4, 5, 6 ];
  var src1 = [ 1, 2, 3 ];
  var src2 = dst;
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 4, 5, 6, 1, 2, 3 ];
  test.true( got !== src1 );
  test.true( got === src2 );
  test.identical( got, exp );

  test.case = 'dst - src1, src1 and src2 is not empty array, duplicates';
  var dst = [ 1, 1, 2, 3, 3 ];
  var src1 = dst;
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 1, 2, 4, 5, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst - src2, all of the elements is present in both arrays';
  var dst = [ 3, 3, 3, 3 ];
  var src1 = [ 3, 3, 3 ];
  var src2 = dst;
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [];
  test.true( got !== src1 );
  test.true( got === src2 );
  test.identical( got, exp );

  test.case = 'one evaluator';
  var dst = [ [ 1 ], 1, 2, [ 3 ], 3 ];
  var src1 = dst;
  var src2 = [ [ 3 ], 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ] ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'two evaluators';
  var dst = [ 1, 1, 2, [ 3 ], 3 ];
  var src1 = dst;
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ], ( ins ) => ins );
  var exp = [ 1, 1, 2, [ 3 ], 3, 4, 5, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'equalizer';
  var dst = [ 1, 1, 2, [ 3 ], 3 ];
  var src1 = dst;
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e, ins ) => e[ 0 ] === ins );
  var exp = [ 1, 1, 2, [ 3 ], 3, 4, 5, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'empty Sets';
  var src1 = new Set();
  var src2 = new Set();
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'src2 - empty Set';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set();
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'src1 - empty Set';
  var src1 = new Set();
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( _.set.is( got ) );
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'src1 and src2 is not empty containers, without duplicates';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'src1 and src2 is not empty Sets, duplicates';
  var src1 = new Set( [ 1, 1, 2, 3, 3 ] );
  var src2 = new Set( [ 3, 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [ 1, 2, 4, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'all of the elements is present in both arrays';
  var src1 = new Set( [ 3, 3, 3 ] );
  var src2 = new Set( [ 3, 3, 3, 3 ] );
  var got = _.arraySet.diff_( src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'one evaluator';
  var src1 = new Set( [ [ 1 ], 1, 2, [ 3 ], 3 ] );
  var src2 = new Set( [ [ 3 ], 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( src1, src2, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ] ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'two evaluators';
  var src1 = new Set( [ 1, 1, 2, [ 3 ], 3 ] );
  var src2 = new Set( [ 3, 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( src1, src2, ( e ) => e[ 0 ], ( ins ) => ins );
  var exp = [ 1, 2, [ 3 ], 3, 4, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'equalizer';
  var src1 = new Set( [ 1, 1, 2, [ 3 ], 3 ] );
  var src2 = new Set( [ 3, 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( src1, src2, ( e, ins ) => e[ 0 ] === ins );
  var exp = [ 1, 2, [ 3 ], 3, 4, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst - not empty, src1 and src2 - empty arrays';
  var dst = new Set( [ 1, 2 ] );
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src2 - empty array';
  var dst = new Set( [ 1 ] );
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1 - empty array';
  var dst = new Set( [ 2 ] );
  var src1 = [];
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 2, 1, 3 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1 and src2 is not empty Sets, without duplicates';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 4, 5, 6 ] );
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1 and src2 is not empty Sets, duplicates';
  var dst = [ 0 ];
  var src1 = new Set( [ 1, 1, 2, 3, 3 ] );
  var src2 = new Set( [ 3, 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 0, 1, 2, 4, 5 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, all of the elements is present in both containers';
  var dst = new Set( [ 1 ] );
  var src1 = new Set( [ 3, 3, 3 ] );
  var src2 = [ 3, 3, 3, 3 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'one evaluator';
  var dst = new Set( [ 1 ] );
  var src1 = new Set( [ [ 1 ], 1, 2, [ 3 ], 3 ] );
  var src2 = new Set( [ [ 3 ], 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ] );
  var exp = [ 1, [ 1 ] ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'two evaluators';
  var dst = new Set();
  var src1 = new Set( [ 1, 1, 2, [ 3 ], 3 ] );
  var src2 = new Set( [ 3, 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ], ( ins ) => ins );
  var exp = [ 1, 2, [ 3 ], 3, 4, 5 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'equalizer';
  var dst = new Set();
  var src1 = new Set( [ 1, 1, 2, [ 3 ], 3 ] );
  var src2 = new Set( [ 3, 3, 4, 5, 5 ] );
  var got = _.arraySet.diff_( dst, src1, src2, ( e, ins ) => e[ 0 ] === ins );
  var exp = [ 1, 2, [ 3 ], 3, 4, 5 ];
  test.true( got === dst );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst - src1, src1 and src2 - empty arrays';
  var dst = new Set();
  var src1 = dst;
  var src2 = [];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - src2, src2 - empty array';
  var dst = new Set();
  var src1 = [ 1, 2, 3 ];
  var src2 = dst;
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got !== src1 );
  test.true( got === src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - src1, src1 - empty array';
  var dst = new Set();
  var src1 = dst;
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - src2, src1 and src2 is not empty array, without duplicates';
  var dst = new Set( [ 4, 5, 6 ] );
  var src1 = [ 1, 2, 3 ];
  var src2 = dst;
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 4, 5, 6, 1, 2, 3 ];
  test.true( got !== src1 );
  test.true( got === src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - src1, src1 and src2 is not empty array, duplicates';
  var dst = new Set( [ 1, 1, 2, 3, 3 ] );
  var src1 = dst;
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [ 1, 2, 4, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - src2, all of the elements is present in both arrays';
  var dst = new Set( [ 3, 3, 3, 3 ] );
  var src1 = [ 3, 3, 3 ];
  var src2 = dst;
  var got = _.arraySet.diff_( dst, src1, src2 );
  var exp = [];
  test.true( got !== src1 );
  test.true( got === src2 );
  test.identical( [ ... got ], exp );

  test.case = 'one evaluator';
  var dst = new Set( [ [ 1 ], 1, 2, [ 3 ], 3 ] );
  var src1 = dst;
  var src2 = [ [ 3 ], 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ] ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'two evaluators';
  var dst = new Set( [ 1, 1, 2, [ 3 ], 3 ] );
  var src1 = dst;
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e ) => e[ 0 ], ( ins ) => ins );
  var exp = [ 1, 2, [ 3 ], 3, 4, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'equalizer';
  var dst = new Set( [ 1, 1, 2, [ 3 ], 3 ] );
  var src1 = dst;
  var src2 = [ 3, 3, 4, 5, 5 ];
  var got = _.arraySet.diff_( dst, src1, src2, ( e, ins ) => e[ 0 ] === ins );
  var exp = [ 1, 2, [ 3 ], 3, 4, 5 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.close( 'Set' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.diff_() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.arraySet.diff_( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.diff_( [ 1, 2 ], [ 5, 7 ], [ 13 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong type of arguments';
  test.shouldThrowErrorSync( () => _.arraySet.diff_( 10, [ 15, 10 ] ) );
  test.shouldThrowErrorSync( () => _.arraySet.diff_( [ 10, 12 ], 'wrong' ) );
  test.shouldThrowErrorSync( () => _.arraySet.diff_( { a : 1 }, { b : 3, c : 8 } ) );
  test.shouldThrowErrorSync( () => _.arraySet.diff_( [], [ 1 ], [ 2 ], [] ) );
}

diff_.timeOut = 10000;

//

// function arraySetBut( test )
// {
//
//   test.case = 'first argument has single extra element, second argument has single extra element either';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var got = _.arraySetBut( a, b );
//   var expected = [ 15 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first argument has single extra element, second argument has single extra element either';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var got = _.arraySetBut( null, a, b );
//   var expected = [ 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first argument has several elements that are not present in second argument';
//   var a = [ 1, 4, 9 ];
//   var b = [ 2, 5 ];
//   var got = _.arraySetBut( a, b );
//   var expected = [ 1, 4, 9 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first argument has several elements that are not present in second argument';
//   var a = [ 1, 4, 9 ];
//   var b = [ 2, 5 ];
//   var got = _.arraySetBut( null, a, b );
//   var expected = [ 1, 4, 9 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first argument is the same as second';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1, 2, 3, 4 ];
//   var got = _.arraySetBut( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first argument is the same as second';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1, 2, 3, 4 ];
//   var got = _.arraySetBut( null, a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3, 4 ];
//   var got = _.arraySetBut( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3, 4 ];
//   var got = _.arraySetBut( null, a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'second argument is an empty array';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [];
//   var got = _.arraySetBut( a, b );
//   var expected = [ 1, 2, 3, 4 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'second argument is an empty array';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [];
//   var got = _.arraySetBut( null, a, b );
//   var expected = [ 1, 2, 3, 4 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first array has the same element as the second ';
//   var a = [ 1, 1, 1 ];
//   var b = [ 1 ];
//   var got = _.arraySetBut( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first array has the same element as the second ';
//   var a = [ 1, 1, 1 ];
//   var b = [ 1 ];
//   var got = _.arraySetBut( null, a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'both arguments are empty arrays';
//   var a = [];
//   var b = [];
//   var got = _.arraySetBut( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'both arguments are empty arrays';
//   var a = [];
//   var b = [];
//   var got = _.arraySetBut( null, a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'single empty argument';
//   var a = [];
//   var got = _.arraySetBut( a );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//
//   test.case = 'single empty argument';
//   var a = [];
//   var got = _.arraySetBut( null, a );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//
//   test.case = 'single not empty argument';
//   var a = [ 3, 4, 5 ];
//   var got = _.arraySetBut( a );
//   var expected = [ 3, 4, 5 ];
//   test.identical( got, expected );
//   test.true( got === a );
//
//   test.case = 'single not empty argument';
//   var a = [ 3, 4, 5 ];
//   var got = _.arraySetBut( null, a );
//   var expected = [ 3, 4, 5 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//
//   test.case = 'three arguments, same elements';
//   var a = [ 3, 4, 5 ];
//   var b = [ 3, 4, 5 ];
//   var c = [ 3, 4, 5 ];
//   var got = _.arraySetBut( a, b, c );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'three arguments, same elements';
//   var a = [ 3, 4, 5 ];
//   var b = [ 3, 4, 5 ];
//   var c = [ 3, 4, 5 ];
//   var got = _.arraySetBut( null, a, b, c );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'three arguments, differet elements';
//   var a = [ 3, 4, 5 ];
//   var b = [ 5 ];
//   var c = [ 3 ];
//   var got = _.arraySetBut( a, b, c );
//   var expected = [ 4 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'three arguments, differet elements';
//   var a = [ 3, 4, 5 ];
//   var b = [ 5 ];
//   var c = [ 3 ];
//   var got = _.arraySetBut( null, a, b, c );
//   var expected = [ 4 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'three arguments, no elements in the second and third';
//   var a = [ 3, 4, 5 ];
//   var b = [];
//   var c = [];
//   var got = _.arraySetBut( a, b, c );
//   var expected = [ 3, 4, 5 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'three arguments, no elements in the second and third';
//   var a = [ 3, 4, 5 ];
//   var b = [];
//   var c = [];
//   var got = _.arraySetBut( null, a, b, c );
//   var expected = [ 3, 4, 5 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'three arguments, no elements in the first';
//   var a = [];
//   var b = [ 3, 4, 5 ];
//   var c = [ 3, 4, 5 ];
//   var got = _.arraySetBut( a, b, c );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'three arguments, no elements in the first';
//   var a = [];
//   var b = [ 3, 4, 5 ];
//   var c = [ 3, 4, 5 ];
//   var got = _.arraySetBut( null, a, b, c );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'complex case';
//   var got = _.arraySetBut( [ 1, 10, 0, 5 ], [ 5, 8, 2 ], [ 3, 1, 6, 4 ], [ 0 ] );
//   var expected = [ 10 ];
//   test.identical( got, expected );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetBut( null, a );
//   var expected = [ 1, 1, 1, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetBut( a );
//   var expected = [ 1, 1, 1, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 null';
//   var got = _.arraySetBut( null );
//   var expected = [];
//   test.identical( got, expected );
//
//   /* */
//
//   var cases =
//   [
//     { src : [], but : [], expected : [] },
//     { src : [ 1, 2, 3 ], but : [], expected : [ 1, 2, 3 ] },
//     { src : [], but : [ 1, 2, 3 ], expected : [] },
//     { src : [ 1, 1, 1 ], but : [ 1 ], expected : [] },
//     { src : [ 1, 2, 3 ], but : [ 3, 2, 1 ], expected : [] },
//     { src : [ 1, 2, 3 ], but : [ 3 ], expected : [ 1, 2 ] },
//     { src : [ 1, 2, 3 ], but : [ 4, 5, 6 ], expected : [ 1, 2, 3 ] },
//     { src : 1, but : 1, error : true },
//   ]
//
//   for( var i = 0; i < cases.length; i++ )
//   {
//     var c1 = cases[ i ];
//
//     if( c1.error )
//     test.shouldThrowErrorSync( () => _.arraySetBut( c1.src, c1.but ) );
//     else
//     test.identical( _.arraySetBut( c1.src, c1.but ), c1.expected );
//   }
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   /* bad arguments */
//
//   test.case = 'not array';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut( '3' );
//   });
//
//   test.case = 'no arguments, the count of arguments doesn\'t match 2';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut();
//   });
//
//   test.case = 'one or both arguments are not longIs entities, numerical arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut( 5, 8 );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, string like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut( 'a', 'c' );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, map like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut( { a : 1 }, { b : 3, c : 8 } );
//   });
//
//   test.case = 'wrong argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut( [ 1, 2, 3 ], 'wrong argument' );
//   });
//
//   test.case = 'both arguments are null';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut( null, null );
//   });
//
//   test.case = 'both arguments are undefined';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetBut( undefined, undefined );
//   });
//
// }

//

function but_( test )
{
  test.case = 'one argument, null';
  var got = _.arraySet.but_( null );
  var exp = [];
  test.identical( got, exp );

  /* - */

  test.open( 'array' );

  test.case = 'one argument, empty array';
  var src1 = [];
  var got = _.arraySet.but_( src1 );
  var exp = [];
  test.true( got === src1 );
  test.identical( got, exp );

  test.case = 'one argument, not empty array';
  var src1 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( src1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.identical( got, exp );

  /* */

  test.case = 'dst - null, src1 - array, src2 - routine';
  var dst = null;
  var src1 = [ 1, 2, 'str' ];
  var got = _.arraySet.but_( dst, src1, ( e ) => e );
  var exp = [ 1, 2, 'str' ];
  test.true( got !== src1 );
  test.identical( got, exp );

  test.case = 'dst - null, src1 - array, src2 - undefined';
  var dst = null;
  var src1 = [ 1, 2, 'str' ];
  var got = _.arraySet.but_( dst, src1 );
  var exp = [ 1, 2, 'str' ];
  test.true( got !== src1 );
  test.identical( got, exp );

  /* */

  test.case = 'src1, src2, without entries';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.but_( src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst === src2, src1, entries';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.but_( src1, src2 );
  var exp = [ 1 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'src1, src2, entries, one evaluator';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( src1, src2, ( e ) => e );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'dst === src1, src2, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = dst;
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst === src1, src2, entries';
  var dst = [ 1, 2, 3 ];
  var src1 = dst;
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst === src2, src1, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 4, 5, 6 ];
  var src2 = dst;
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst === src2, src1, entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 2, 3, 6 ];
  var src2 = dst;
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst, src1, src2, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 1, 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst, src1, src2, entries, one evaluator';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.but_( dst, src1, src2, ( e ) => e );
  var exp = [];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var dst = [];
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( dst, src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( dst, src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'one argument, empty array';
  var src1 = new Set();
  var got = _.arraySet.but_( src1 );
  var exp = [];
  test.true( got === src1 );
  test.identical( [ ... got ], exp );

  test.case = 'one argument, not empty array';
  var src1 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.but_( src1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst - null, src1 - array, src2 - routine';
  var dst = null;
  var src1 = new Set( [ 1, 2, 'str' ] );
  var got = _.arraySet.but_( dst, src1, ( e ) => e );
  var exp = [ 1, 2, 'str' ];
  test.true( _.set.is( got ) );
  test.true( got !== src1 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - null, src1 - array, src2 - undefined';
  var dst = null;
  var src1 = new Set( [ 1, 2, 'str' ] );
  var got = _.arraySet.but_( dst, src1 );
  var exp = [ 1, 2, 'str' ];
  test.true( _.set.is( got ) );
  test.true( got !== src1 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'src1, src2, without entries';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 4, 5, 6 ] );
  var got = _.arraySet.but_( src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src2, src1, entries';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 2, 3, 6 ] );
  var got = _.arraySet.but_( src1, src2 );
  var exp = [ 1 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'src1, src2, entries, one evaluator';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( src1, src2, ( e ) => e );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.but_( src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.but_( src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst === src1, src2, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = dst;
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src1, src2, entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = dst;
  var src2 = new Set( [ 2, 3, 6 ] );
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst === src2, src1, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = new Set( [ 4, 5, 6 ] );
  var src2 = dst;
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src2, src1, entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = new Set( [ 2, 3, 6 ] );
  var src2 = dst;
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst, src1, src2, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = [ 1, 2, 3, 7 ];
  var src2 = new Set( [ 4, 5, 6 ] );
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 7 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.but_( dst, src1, src2 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst, src1, src2, entries, one evaluator';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( dst, src1, src2, ( e ) => e );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.but_( dst, src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var dst = new Set();
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.but_( dst, src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.close( 'Set' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.but_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.but_( [], [ 1, 2 ], [ 1 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'one argument, wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.but_( '3' ) );

  test.case = 'two arguments, wrong type of src1 or src2';
  test.shouldThrowErrorSync( () => _.arraySet.but_( 'wrong', [ 1, 2 ] ) );
  test.shouldThrowErrorSync( () => _.arraySet.but_( [ 1, 2 ], 'wrong' ) );
}

//

// function arraySetIntersection( test )
// {
//
//   test.case = 'second argument has extra element, third argument has two extra elements';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var got = _.arraySetIntersection( a, b );
//   var expected = [ 1, 2, 3, 4 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'second argument has extra element, third argument has two extra elements';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var got = _.arraySetIntersection( null, a, b );
//   var expected = [ 1, 2, 3, 4 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'single array argument';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetIntersection( a );
//   var expected = [ 1, 2, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got === a );
//
//   test.case = 'single array argument';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetIntersection( null, a );
//   var expected = [ 1, 2, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetIntersection( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetIntersection( null, a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first and second argument are empty arrays';
//   var a = [];
//   var b = [];
//   var got = _.arraySetIntersection( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first and second argument are empty arrays';
//   var a = [];
//   var b = [];
//   var got = _.arraySetIntersection( null, a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = '3 arguments, nothing in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var c = [ 15, 16, 17 ];
//   var got = _.arraySetIntersection( a, b, c );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, nothing in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var c = [ 15, 16, 17 ];
//   var got = _.arraySetIntersection( null, a, b, c );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, something in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var c = [ 3, 15, 16, 17, 1 ];
//   var got = _.arraySetIntersection( null, a, b, c );
//   var expected = [ 1, 3 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, something in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 3 ];
//   var c = [ 1, 3 ];
//   var got = _.arraySetIntersection( null, a, b, c );
//   var expected = [ 3 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var b = [ 3, 1 ];
//   var c = [ 1, 3 ];
//   var got = _.arraySetIntersection( null, a, b, c );
//   var expected = [ 1, 1, 1, 3 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetIntersection( null, a );
//   var expected = [ 1, 1, 1, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetIntersection( a );
//   var expected = [ 1, 1, 1, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 null';
//   var got = _.arraySetIntersection( null );
//   var expected = [];
//   test.identical( got, expected );
//
//   /* extra */
//
//   var cases =
//   [
//     { args : [ [] ], expected : [] },
//     { args : [ [ 1 ], [] ], expected : [] },
//     { args : [ [], [ 1 ] ], expected : [] },
//     { args : [ [ 1 ], [ 2 ] ], expected : [] },
//     { args : [ [ 1, 2, 3 ], [ 2 ] ], expected : [ 2 ] },
//     { args : [ [ 1, 2, 3 ], [ 2 ], [ 1 ], [ 3 ] ], expected : [] },
//     { args : [ [ 1, 1, 1 ], [ 1 ] ], expected : [ 1, 1, 1 ] },
//     { args : [ [ 1, 2, 3 ], [ 0 ], [ 4 ], [ 0, 0, 3 ] ], expected : [] },
//     { args : [ [ 1, 2, 3 ], [ 0 ], 1, [ 3 ] ], error : true },
//     { args : [ 1 ], error : true },
//   ]
//
//   for( var i = 0; i < cases.length; i++ )
//   {
//     var c1 = cases[ i ];
//     if( c1.error )
//     test.shouldThrowErrorSync( () => _.arraySetIntersection.apply( _, c1.args ) );
//     else
//     test.identical( _.arraySetIntersection.apply( _, c1.args ), c1.expected );
//   }
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetIntersection();
//   });
//
//   test.case = 'one or several arguments are not longIs entities, numerical arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetIntersection( 10, 15, 25 );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, string like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetIntersection( 'a', 'c' );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, map arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetIntersection( { a : 1 }, { b : 3, c : 8 } );
//   });
//
//   test.case = 'wrong argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetIntersection( [ 1, 2, 3 ], 'wrong argument' );
//   });
//
//   test.case = 'one or more arguments are null';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetIntersection( null, null );
//   });
//
//   test.case = 'one or more arguments are undefined';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetIntersection( undefined, undefined );
//   });
//
// }

//

function intersection_( test )
{
  test.case = 'one argument, null';
  var got = _.arraySet.intersection_( null );
  var exp = [];
  test.identical( got, exp );

  /* - */

  test.open( 'array' );

  test.case = 'one argument, empty array';
  var src1 = [];
  var got = _.arraySet.intersection_( src1 );
  var exp = [];
  test.true( got === src1 );
  test.identical( got, exp );

  test.case = 'one argument, not empty array';
  var src1 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( src1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.identical( got, exp );

  /* */

  test.case = 'dst - null, src1 - array, src2 - routine';
  var dst = null;
  var src1 = [ 1, 2, 'str' ];
  var got = _.arraySet.intersection_( dst, src1, ( e ) => e );
  var exp = [ 1, 2, 'str' ];
  test.true( got !== src1 );
  test.identical( got, exp );

  test.case = 'dst - null, src1 - array, src2 - undefined';
  var dst = null;
  var src1 = [ 1, 2, 'str' ];
  var got = _.arraySet.intersection_( dst, src1 );
  var exp = [ 1, 2, 'str' ];
  test.true( got !== src1 );
  test.identical( got, exp );

  /* */

  test.case = 'src1, src2, without entries';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.intersection_( src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst === src2, src1, entries';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.intersection_( src1, src2 );
  var exp = [ 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'src1, src2, entries, one evaluator';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'dst === src1, src2, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = dst;
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst === src1, src2, entries';
  var dst = [ 1, 2, 3 ];
  var src1 = dst;
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst === src2, src1, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 4, 5, 6 ];
  var src2 = dst;
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst === src2, src1, entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 2, 3, 6 ];
  var src2 = dst;
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst, src1, src2, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst, src1, src2, entries, one evaluator';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.intersection_( dst, src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var dst = [];
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( dst, src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( dst, src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'one argument, empty array';
  var src1 = new Set();
  var got = _.arraySet.intersection_( src1 );
  var exp = [];
  test.true( got === src1 );
  test.identical( [ ... got ], exp );

  test.case = 'one argument, not empty array';
  var src1 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.intersection_( src1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst - null, src1 - array, src2 - routine';
  var dst = null;
  var src1 = new Set( [ 1, 2, 'str' ] );
  var got = _.arraySet.intersection_( dst, src1, ( e ) => e );
  var exp = [ 1, 2, 'str' ];
  test.true( _.set.is( got ) );
  test.true( got !== src1 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - null, src1 - array, src2 - undefined';
  var dst = null;
  var src1 = new Set( [ 1, 2, 'str' ] );
  var got = _.arraySet.intersection_( dst, src1 );
  var exp = [ 1, 2, 'str' ];
  test.true( _.set.is( got ) );
  test.true( got !== src1 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'src1, src2, without entries';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 4, 5, 6 ] );
  var got = _.arraySet.intersection_( src1, src2 );
  var exp = [];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src2, src1, entries';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 2, 3, 6 ] );
  var got = _.arraySet.intersection_( src1, src2 );
  var exp = [ 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'src1, src2, entries, one evaluator';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.intersection_( src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.intersection_( src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst === src1, src2, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = dst;
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src1, src2, entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = dst;
  var src2 = new Set( [ 2, 3, 6 ] );
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst === src2, src1, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = new Set( [ 4, 5, 6 ] );
  var src2 = dst;
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src2, src1, entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = new Set( [ 2, 3, 6 ] );
  var src2 = dst;
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst, src1, src2, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = [ 1, 2, 3, 7 ];
  var src2 = new Set( [ 4, 5, 6 ] );
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.intersection_( dst, src1, src2 );
  var exp = [ 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst, src1, src2, entries, one evaluator';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( dst, src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.intersection_( dst, src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var dst = new Set();
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.intersection_( dst, src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.close( 'Set' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.intersection_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.intersection_( [], [ 1, 2 ], [ 1 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'one argument, wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.intersection_( '3' ) );

  test.case = 'two arguments, wrong type of src1 or src2';
  test.shouldThrowErrorSync( () => _.arraySet.intersection_( 'wrong', [ 1, 2 ] ) );
  test.shouldThrowErrorSync( () => _.arraySet.intersection_( [ 1, 2 ], 'wrong' ) );
}

//

// function arraySetUnion( test )
// {
//
//   test.case = 'second argument has extra element, third argument has two extra elements';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var got = _.arraySetUnion( a, b );
//   var expected = [ 1, 2, 3, 4, 15, 5 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'second argument has extra element, third argument has two extra elements';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var got = _.arraySetUnion( null, a, b );
//   var expected = [ 1, 2, 3, 4, 15, 5 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'single array argument';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetUnion( a );
//   var expected = [ 1, 2, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got === a );
//
//   test.case = 'single array argument';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetUnion( null, a );
//   var expected = [ 1, 2, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetUnion( a, b );
//   var expected = [ 1, 2, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3, 4, 15 ];
//   var got = _.arraySetUnion( null, a, b );
//   var expected = [ 1, 2, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first and second argument are empty arrays';
//   var a = [];
//   var b = [];
//   var got = _.arraySetUnion( a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//
//   test.case = 'first and second argument are empty arrays';
//   var a = [];
//   var b = [];
//   var got = _.arraySetUnion( null, a, b );
//   var expected = [];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = '3 arguments, nothing in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var c = [ 15, 16, 17 ];
//   var got = _.arraySetUnion( a, b, c );
//   var expected = [ 1, 2, 3, 4, 15, 5, 16, 17 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, nothing in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var c = [ 15, 16, 17 ];
//   var got = _.arraySetUnion( null, a, b, c );
//   var expected = [ 1, 2, 3, 4, 15, 5, 16, 17 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, something in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 1, 2, 3, 4, 5 ];
//   var c = [ 3, 15, 16, 17, 1 ];
//   var got = _.arraySetUnion( null, a, b, c );
//   var expected = [ 1, 2, 3, 4, 15, 5, 16, 17 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, something in common';
//   var a = [ 1, 2, 3, 4, 15 ];
//   var b = [ 3 ];
//   var c = [ 1, 3 ];
//   var got = _.arraySetUnion( null, a, b, c );
//   var expected = [ 1, 2, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '3 arguments, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var b = [ 3, 1 ];
//   var c = [ 1, 3 ];
//   var got = _.arraySetUnion( null, a, b, c );
//   var expected = [ 1, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetUnion( null, a );
//   var expected = [ 1, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetUnion( a );
//   var expected = [ 1, 1, 1, 3, 4, 15 ];
//   test.identical( got, expected );
//   test.true( got === a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = '1 null';
//   var got = _.arraySetUnion( null );
//   var expected = [];
//   test.identical( got, expected );
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetUnion();
//   });
//
//   test.case = 'one or several arguments are not longIs entities, numerical arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetUnion( 10, 15, 25 );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, string like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetUnion( 'a', 'c' );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, map arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetUnion( { a : 1 }, { b : 3, c : 8 } );
//   });
//
//   test.case = 'wrong argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetUnion( [ 1, 2, 3 ], 'wrong argument' );
//   });
//
//   test.case = 'one or more arguments are null';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetUnion( null, null );
//   });
//
//   test.case = 'one or more arguments are undefined';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetUnion( undefined, undefined );
//   });
//
// }

//

function union_( test )
{
  test.case = 'one argument, null';
  var got = _.arraySet.union_( null );
  var exp = [];
  test.identical( got, exp );

  /* - */

  test.open( 'array' );

  test.case = 'one argument, empty array';
  var src1 = [];
  var got = _.arraySet.union_( src1 );
  var exp = [];
  test.true( got === src1 );
  test.identical( got, exp );

  test.case = 'one argument, not empty array';
  var src1 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( src1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.identical( got, exp );

  /* */

  test.case = 'dst - null, src1 - array, src2 - routine';
  var dst = null;
  var src1 = [ 1, 2, 'str' ];
  var got = _.arraySet.union_( dst, src1, ( e ) => e );
  var exp = [ 1, 2, 'str' ];
  test.true( got !== src1 );
  test.identical( got, exp );

  test.case = 'dst - null, src1 - array, src2 - undefined';
  var dst = null;
  var src1 = [ 1, 2, 'str' ];
  var got = _.arraySet.union_( dst, src1 );
  var exp = [ 1, 2, 'str' ];
  test.true( got !== src1 );
  test.identical( got, exp );

  /* */

  test.case = 'src1, src2, without entries';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.union_( src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst === src2, src1, entries';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.union_( src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'src1, src2, entries, one evaluator';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2, 3, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2, 3, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( got, exp );

  /* */

  test.case = 'dst === src1, src2, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = dst;
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst === src1, src2, entries';
  var dst = [ 1, 2, 3 ];
  var src1 = dst;
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst === src2, src1, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 4, 5, 6 ];
  var src2 = dst;
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst === src2, src1, entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 2, 3, 6 ];
  var src2 = dst;
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst, src1, src2, without entries';
  var dst = [ 1, 2, 3 ];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( got, exp );

  /* */

  test.case = 'dst, src1, src2, entries, one evaluator';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.union_( dst, src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var dst = [];
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( dst, src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2, 3, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var dst = [];
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( dst, src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2, 3, 3 ];
  test.true( got === dst );
  test.identical( got, exp );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'one argument, empty array';
  var src1 = new Set();
  var got = _.arraySet.union_( src1 );
  var exp = [];
  test.true( got === src1 );
  test.identical( [ ... got ], exp );

  test.case = 'one argument, not empty array';
  var src1 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.union_( src1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst - null, src1 - array, src2 - routine';
  var dst = null;
  var src1 = new Set( [ 1, 2, 'str' ] );
  var got = _.arraySet.union_( dst, src1, ( e ) => e );
  var exp = [ 1, 2, 'str' ];
  test.true( _.set.is( got ) );
  test.true( got !== src1 );
  test.identical( [ ... got ], exp );

  test.case = 'dst - null, src1 - array, src2 - undefined';
  var dst = null;
  var src1 = new Set( [ 1, 2, 'str' ] );
  var got = _.arraySet.union_( dst, src1 );
  var exp = [ 1, 2, 'str' ];
  test.true( _.set.is( got ) );
  test.true( got !== src1 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'src1, src2, without entries';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 4, 5, 6 ] );
  var got = _.arraySet.union_( src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src2, src1, entries';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 2, 3, 6 ] );
  var got = _.arraySet.union_( src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'src1, src2, entries, one evaluator';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.union_( src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.union_( src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === src1 );
  test.true( got !== src2 );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst === src1, src2, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = dst;
  var src2 = [ 4, 5, 6 ];
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src1, src2, entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = dst;
  var src2 = new Set( [ 2, 3, 6 ] );
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst === src2, src1, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = new Set( [ 4, 5, 6 ] );
  var src2 = dst;
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst === src2, src1, entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = new Set( [ 2, 3, 6 ] );
  var src2 = dst;
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst, src1, src2, without entries';
  var dst = new Set( [ 1, 2, 3 ] );
  var src1 = [ 1, 2, 3, 7 ];
  var src2 = new Set( [ 4, 5, 6 ] );
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 7, 4, 5, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 2, 3, 6 ];
  var got = _.arraySet.union_( dst, src1, src2 );
  var exp = [ 1, 2, 3, 6 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  /* */

  test.case = 'dst, src1, src2, entries, one evaluator';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( dst, src1, src2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, two evaluators';
  var dst = new Set();
  var src1 = new Set( [ 1, 2, 3 ] );
  var src2 = new Set( [ 1, 2, 3 ] );
  var got = _.arraySet.union_( dst, src1, src2, ( e ) => e, ( ins ) => ins + 1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.case = 'dst, src1, src2, entries, equalizer';
  var dst = new Set();
  var src1 = [ 1, 2, 3 ];
  var src2 = [ 1, 2, 3 ];
  var got = _.arraySet.union_( dst, src1, src2, ( e, ins ) => e === ins + 1 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got ], exp );

  test.close( 'Set' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.union_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.union_( [], [ 1, 2 ], [ 1 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'one argument, wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.union_( '3' ) );

  test.case = 'two arguments, wrong type of src1 or src2';
  test.shouldThrowErrorSync( () => _.arraySet.union_( 'wrong', [ 1, 2 ] ) );
  test.shouldThrowErrorSync( () => _.arraySet.union_( [ 1, 2 ], 'wrong' ) );
}

//

// function arraySetContainAll( test )
// {
//
//   test.case = '1 argument, empty';
//   var a = [];
//   var got = _.arraySetContainAll( a );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetContainAll( a );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '2 arguments, empty';
//   var a = [];
//   var b = [];
//   var got = _.arraySetContainAll( a, b );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '2 arguments, src empty';
//   var a = [];
//   var b = [ 1 ];
//   var got = _.arraySetContainAll( a, b );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = '2 arguments, ins empty';
//   var a = [ 1 ];
//   var b = [];
//   var got = _.arraySetContainAll( a, b );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'bigger second argument';
//   var a = [ 1, 3 ];
//   var b = [ 1, 1, 1, 1 ];
//   var got = _.arraySetContainAll( a, b );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'bigger third argument';
//   var a = [ 1, 3 ];
//   var b = [ 1, 1 ];
//   var c = [ 1, 1, 1, 1 ];
//   var got = _.arraySetContainAll( a, b, c );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '4 arguments';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 3, 1 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAll( a, b, c, d );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 0 ];
//   var c = [ 3, 1 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAll( a, b, c, d );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 0, 1 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAll( a, b, c, d );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 3, 0 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAll( a, b, c, d );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 3, 1 ];
//   var d = [ 4, 1, 0, 2 ];
//   var got = _.arraySetContainAll( a, b, c, d );
//   var expected = false;
//   test.identical( got, expected );
//
//
//   test.case = 'Second argument contains all the same values as in the (src), and new ones';
//   var a = [ 1, 'b', 'c', 4 ];
//   var b = [ 1, 2, 3, 4, 5, 'b', 'c' ];
//   var got = _.arraySetContainAll( a, b );
//   var expected = false;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'length of the first argument is more than second argument';
//   var a = [ 1, 2, 3, 4, 5 ];
//   var b = [ 1, 2, 3, 4 ];
//   var got = _.arraySetContainAll( a, b );
//   var expected = true;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'length of the first argument is more than second argument';
//   var a = [ 'abc', 'def', true, 26 ];
//   var b = [ 1, 2, 3, 4 ];
//   var c = [ 26, 'abc', 'def', true ];
//   var got = _.arraySetContainAll( a, b, c );
//   var expected = false;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'second argument is an empty array';
//   var a = [ 1, 2, 3 ];
//   var b = [];
//   var got = _.arraySetContainAll( a, b );
//   var expected = true;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 2, 3 ];
//   var got = _.arraySetContainAll( a, b );
//   var expected = false;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   test.case = 'both arguments are empty';
//   var a = [];
//   var b = [];
//   var got = _.arraySetContainAll( a, b );
//   var expected = true;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//
//   /* */
//
//   var cases =
//   [
//     { args : [ [], [] ], expected : true },
//     { args : [ [ 1 ], [] ], expected : true },
//     { args : [ [ 1 ], [ 1 ] ], expected : true },
//     { args : [ [ 1, 1 ], [ 1, 1, 1 ] ], expected : true },
//     { args : [ [ 1, 1, 1 ], [ 1, 1 ] ], expected : true },
//     { args : [ [ 1 ], [ 1 ], [ 1 ], [] ], expected : true },
//     { args : [ [ 1 ], [ 1 ], [ 1 ], [ 1, 1, 1 ] ], expected : true },
//     { args : [ [ 1 ], [ 0, 1 ], [ 3, 2, 1 ], [ 1 ] ], expected : false },
//     { args : [ [ 1, 2, 3 ], [ 1, 2, 3 ], [ 3, 2, 1 ], [ 1, 2 ] ], expected : true },
//     { args : [ [], [ 1, 2, 3 ], [ 3, 2, 1 ], [ 1, 2 ] ], expected : false },
//     { args : [ [], 1, [ 3, 2, 1 ], [ 1, 2 ] ], error : true },
//     { args : [ 1 ], error : true },
//   ]
//
//   for( var i = 0; i < cases.length; i++ )
//   {
//     var _case = cases[ i ];
//     console.log( _.entity.exportString( _case, { levels : 3 } ) );
//     if( _case.error )
//     test.shouldThrowErrorSync( () => _.arraySetContainAll.apply( _, _case.args ) );
//     else
//     test.identical( _.arraySetContainAll.apply( _, _case.args ), _case.expected );
//   }
//
//   /* special cases */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAll();
//   });
//
//   test.case = 'one or both arguments are not longIs entities, numerical arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAll( 5, 8 );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, string like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAll( 'a', 'c' );
//   });
//
//   test.case = 'one or both arguments are not longIs entities, map like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAll( { a : 1, b : 2 }, { c : 3 } );
//   });
//
//   test.case = 'wrong arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAll( [ 1, 2, 3, 4 ], 'wrong arguments' );
//   });
//
//   test.case = 'both arguments are null';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAll( null, null );
//   });
//
//   test.case = 'both arguments are undefined';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAll( undefined, undefined );
//   });
// }

//

function containAll_( test )
{
  test.open( 'array' );

  test.case = 'src1 - empty array, src2 - empty array';
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - filled array';
  var src1 = [];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - empty array';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content';
  var src1 = [ -1, 1, 'str', '', undefined, null, false ];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : -1 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : -1 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : -1 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'src' } ];
  var got = _.arraySet.containAll_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : -1 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'src1 - empty Set, src2 - empty Set';
  var src1 = new Set( [] );
  var src2 = new Set( [] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled Set';
  var src1 = new Set( [] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty Set';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, 6, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'src' } ] );
  var got = _.arraySet.containAll_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'Set' );

  /* - */

  test.open( 'mixed' );

  test.case = 'src1 - empty array, src2 - empty Set';
  var src1 = [];
  var src2 = new Set( [] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled array';
  var src1 = new Set( [] );
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty array';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = [];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : -1 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'src' } ];
  var got = _.arraySet.containAll_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : -1 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a :  -1 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAll_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'mixed' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containAll_() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.arraySet.containAll_( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containAll_( [ 1, 2 ], new Set( [] ), ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.containAll_( 'wrong', new Set( [] ) ) );

  test.case = 'wrong type of src2';
  test.shouldThrowErrorSync( () => _.arraySet.containAll_( new Set( [] ), 'wrong' ) );
}

//

// function arraySetContainAny( test )
// {
//
//   test.case = '1 argument, empty';
//   var a = [];
//   var got = _.arraySetContainAny( a );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '1 argument, repeats';
//   var a = [ 1, 1, 1, 3, 4, 15 ];
//   var got = _.arraySetContainAny( a );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '2 arguments, empty';
//   var a = [];
//   var b = [];
//   var got = _.arraySetContainAny( a, b );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '2 arguments, src empty';
//   var a = [];
//   var b = [ 1 ];
//   var got = _.arraySetContainAny( a, b );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '2 arguments, ins empty';
//   var a = [ 1 ];
//   var b = [];
//   var got = _.arraySetContainAny( a, b );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'bigger second argument';
//   var a = [ 1, 3 ];
//   var b = [ 1, 1, 1, 1 ];
//   var got = _.arraySetContainAny( a, b );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'bigger third argument';
//   var a = [ 1, 3 ];
//   var b = [ 1, 1 ];
//   var c = [ 1, 1, 1, 1 ];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '3 arguments, the first is empty';
//   var a = [];
//   var b = [ 1 ];
//   var c = [ 2, 3 ];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = '4 arguments';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 3, 1 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAny( a, b, c, d );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 0 ];
//   var c = [ 3, 1 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAny( a, b, c, d );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 0, 1 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAny( a, b, c, d );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 3, 0 ];
//   var d = [ 4, 1, 3, 2 ];
//   var got = _.arraySetContainAny( a, b, c, d );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'one argument have redundant element';
//   var a = [ 1, 2, 3, 4 ];
//   var b = [ 1 ];
//   var c = [ 3, 1 ];
//   var d = [ 4, 1, 0, 2 ];
//   var got = _.arraySetContainAny( a, b, c, d );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'second and third arrays contains several values from (src) array';
//   var a = [ 33, 4, 5, 'b', 'c' ];
//   var b = [ 1, 'b', 'c', 4 ];
//   var c = [ 33, 13, 3 ];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = true;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'second array is empty, third array contains elements from (src) array';
//   var a = [ 33, 4, 5, 'b', 'c' ];
//   var b = [];
//   var c = [ 33 ];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = false;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'second and third arrays doesn\'t contains matching elemets from (src) array';
//   var a = [ 33, 4, 5, 'b', 'c' ];
//   var b = [ 1, 'bcda', 'ce', 8 ];
//   var c = [ 45, 13, 3 ];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = false;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'first argument is an empty array';
//   var a = [];
//   var b = [ 1, 'bcda', 'ce', 8 ];
//   var c = [ 45, 13, 3 ];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = true;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'following array are empty, (src) array is not empty';
//   var a = [ 33, 4, 5, 'b', 'c' ];
//   var b = [];
//   var c = [];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = false;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'all the array are empty';
//   var a = [];
//   var b = [];
//   var c = [];
//   var got = _.arraySetContainAny( a, b, c );
//   var expected = true;
//   test.identical( got, expected );
//   test.true( got !== a );
//   test.true( got !== b );
//   test.true( got !== c );
//
//   test.case = 'single argument';
//   var got = _.arraySetContainAny( [ 33, 4, 5, 'b', 'c' ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   /**/
//
//   var cases =
//   [
//     { args : [ [], [] ], expected : true },
//     { args : [ [ 1, 2, 3 ], [ 1 ] ], expected : true },
//     { args : [ [ 1, 2, 3 ], [], [ 3 ] ], expected : false },
//     { args : [ [ 1, 2, 3 ], [ 0, 1 ], [ 9, 3 ] ], expected : true },
//     { args : [ [ 1, 2, 3 ], [ 4 ], [ 3 ] ], expected : false },
//     { args : [ [], [ 0 ], [ 4 ], [ 3 ] ], expected : true },
//     { args : [ [ 1, 2, 3 ], [ 4 ], [ 5 ] ], expected : false },
//     { args : [ [ 0, 0, 0, 1 ], [ 5 ], [ 6 ], [ 2, 1 ] ], expected : false },
//     { args : [ [ 1, 2, 3 ], [ 4 ], 1 ], error : true },
//     { args : [ 1, [ 4 ], 1 ], error : true },
//   ]
//
//   for( var i = 0; i < cases.length; i++ )
//   {
//     var c1 = cases[ i ];
//
//     if( c1.error )
//     test.shouldThrowErrorSync( () => _.arraySetContainAny.apply( _, c1.args ) );
//     else
//     test.identical( _.arraySetContainAny.apply( _, c1.args ), c1.expected );
//   }
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAny();
//   });
//
//   test.case = 'one or several arguments are not longIs entities, numeric arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAny( [ 33, 4, 5, 'b', 'c' ], 15, 25 );
//   });
//
//   test.case = 'one or several arguments are not longIs entities, string like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAny( [ 33, 4, 5, 'b', 'c' ], 'dfdf', 'ab' );
//   });
//
//   test.case = 'one or several arguments are not longIs entities, map like arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAny( [ 33, 4, 5, 'b', 'c' ], { a : 33 }, { b : 44 } );
//   });
//
//   test.case = 'wrong argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arraySetContainAny( [ 1, 2, 3 ], 'wrong argument' );
//   });
//
// }

//

function containAny_( test )
{
  test.open( 'array' );

  test.case = 'src1 - empty array, src2 - empty array';
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty array, src2 - filled array';
  var src1 = [];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - empty array';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content';
  var src1 = [ -1, 1, 'str', '', undefined, null, false ];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'src' } ];
  var got = _.arraySet.containAny_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'src1 - empty Set, src2 - empty Set';
  var src1 = new Set( [] );
  var src2 = new Set( [] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty Set, src2 - filled Set';
  var src1 = new Set( [] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty Set';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'src' } ] );
  var got = _.arraySet.containAny_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'Set' );

  /* - */

  test.open( 'mixed' );

  test.case = 'src1 - empty array, src2 - empty Set';
  var src1 = [];
  var src2 = new Set( [] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty Set, src2 - filled array';
  var src1 = new Set( [] );
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty array';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = [];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'src' } ];
  var got = _.arraySet.containAny_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containAny_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'mixed' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containAny_() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.arraySet.containAny_( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containAny_( [ 1, 2 ], new Set( [] ), ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.containAny_( 'wrong', new Set( [] ) ) );

  test.case = 'wrong type of src2';
  test.shouldThrowErrorSync( () => _.arraySet.containAny_( new Set( [] ), 'wrong' ) );
}

containAny_.timeOut = 10000;

//

function containNone_( test )
{
  test.open( 'array' );

  test.case = 'src1 - empty array, src2 - empty array';
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - filled array';
  var src1 = [];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - empty array';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content';
  var src1 = [ -1, 1, 'str', '', undefined, null, false ];
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'src' } ];
  var got = _.arraySet.containNone_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'src1 - empty Set, src2 - empty Set';
  var src1 = new Set( [] );
  var src2 = new Set( [] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled Set';
  var src1 = new Set( [] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - empty Set';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'src' } ] );
  var got = _.arraySet.containNone_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.close( 'Set' );

  /* - */

  test.open( 'mixed' );

  test.case = 'src1 - empty array, src2 - empty Set';
  var src1 = [];
  var src2 = new Set( [] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled array';
  var src1 = new Set( [] );
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - empty array';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = [];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = [ 0, 1, 'str', '', undefined, null, false ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'src' } ];
  var got = _.arraySet.containNone_( src1, src2, 2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var got = _.arraySet.containNone_( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.close( 'mixed' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containNone_() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.arraySet.containNone_( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containNone_( [ 1, 2 ], new Set( [] ), ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.containNone_( 'wrong', new Set( [] ) ) );

  test.case = 'wrong type of src2';
  test.shouldThrowErrorSync( () => _.arraySet.containNone_( new Set( [] ), 'wrong' ) );
}

//

function containSetsAll( test )
{
  test.open( 'array' );

  test.case = 'src1 - empty array, src2 - empty array';
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - array with empty arrays';
  var src1 = [];
  var src2 = [ [], [], [] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - filled array';
  var src1 = [];
  var src2 = [ [ 0, 1 ], [ 'str', '' ], [ undefined, null, false ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - empty array';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - array with empty arrays';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ [], [], [] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ [ 0, 1 ], [ 'str', '' ], [ undefined, null, false ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content';
  var src1 = [ -1, 1, 'str', '', undefined, null, false ];
  var src2 = [ [ 0, 1 ], [ 'str', '' ], [ undefined, null, false ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 } ], [ { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : -1 } ], [ { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 } ], [ { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : -1 } ], [ { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : -1 }, { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ] ], [ [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], [ { a : 'src' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : -1 }, { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 } ], [ { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'src1 - empty Set, src2 - empty Set';
  var src1 = new Set( [] );
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - Set with empty arrays and Sets';
  var src1 = new Set( [] );
  var src2 = new Set( [ [], [], new Set( [] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled Set';
  var src1 = new Set( [] );
  var src2 = new Set( [ [ 0, 1 ], [ 'str', '' ], new Set( [ undefined, null, false ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty Set';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - Set with empty arrays and Sets';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ [], [], new Set( [] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ [ 0, 1 ], [ 'str', '' ], new Set( undefined, null, false ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ [ 0, 1 ], [ 'str', '', undefined ], new Set( null, false ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], new Set( [ undefined ], [ null ], [ false ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], new Set( [ undefined ], [ null ], [ false ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : -1 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : -1 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], new Set( [ [ undefined ], [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : -1 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, 6, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'src' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : -1 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : -1 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'Set' );

  /* - */

  test.open( 'mixed' );

  test.case = 'src1 - empty array, src2 - empty Set';
  var src1 = [];
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - Set with empty arrays and Sets';
  var src1 = [];
  var src2 = new Set( [ [], [], new Set() ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled array';
  var src1 = new Set( [] );
  var src2 = [ [ 0, 1 ], new Set( [ 'str', '' ] ), [ undefined, null, false ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty array';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = [];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - array with empty array and Sets';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = [ [], new Set(), [] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = new Set( [ [ 0, 1, 'str' ], new Set( [ '', undefined, null ] ), [ false ] ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = [ [ 0, 1 ], new Set( [ 'str', '', undefined, null ] ), [ false ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ] ], new Set( [ [ 'str' ], [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ] ], new Set( [ [ 'str' ], [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : -1 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], [ [ '' ], [ undefined ], [ null ] ], new Set( [ [ false ] ] ) ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], [ [ '' ], [ undefined ], [ null ] ], new Set( [ [ false ] ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : -1 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ] ] ), new Set( [ [ 'str' ], [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), new Set( [ [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ new Set( [ { a : -1 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsAll( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ]  ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'src' } ] ) ];
  var got = _.arraySet.containSetsAll( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : -1 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAll( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a :  -1 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAll( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'mixed' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAll() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAll( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAll( [ 1, 2 ], new Set( [] ), ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAll( 'wrong', new Set( [] ) ) );

  test.case = 'wrong type of src2';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAll( new Set( [] ), 'wrong' ) );
}

//

function containSetsAny( test )
{
  test.open( 'array' );

  test.case = 'src1 - empty array, src2 - empty array';
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty array, src2 - array with empty arrays and Sets';
  var src1 = [];
  var src2 = [ [], new Set(), [] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty array, src2 - filled array';
  var src1 = [];
  var src2 = [ [ 0, 1 ], new Set( [ 'str', '', undefined ] ), [ null, false ] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - empty array';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - array with empty arrays and Sets';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ [], new Set(), [] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ [ 0, 1 ], new Set( [ 'str', '', undefined ] ), [ null, false ] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content';
  var src1 = [ -1, 1, 'str', '', undefined, null, false ];
  var src2 = [ [ 0, 1, 'str' ], new Set( [ '', undefined ] ), [ null, false ] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], new Set( [ [ 'str' ], [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), new Set( [ [ false ] ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'src' } ] ];
  var got = _.arraySet.containSetsAny( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], new Set( [ [ 'str' ], [ '' ] ] ), [ [ undefined ], [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'src1 - empty Set, src2 - empty Set';
  var src1 = new Set( [] );
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty Set, src2 - Set with empty arrays and Sets';
  var src1 = new Set( [] );
  var src2 = new Set( [ [], new Set(), [] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty Set, src2 - filled Set';
  var src1 = new Set( [] );
  var src2 = new Set( [ [ 0, 1, 'str' ], new Set( [ '', undefined, null ] ), new Set( [ false ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty Set';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - Set with empty arrays and Sets';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ [], new Set(), [] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ [ 0, 1, 'str' ], new Set( [ '', undefined, null ] ), [ false ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ new Set( [ 0, 1, 'str' ] ), [ '', undefined, null ], [ false ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'src' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' }  ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'Set' );

  /* - */

  test.open( 'mixed' );

  test.case = 'src1 - empty array, src2 - empty Set';
  var src1 = [];
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty array, src2 - Set with empty arrays and Sets';
  var src1 = [];
  var src2 = new Set( [ [], new Set(), [] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - empty Set, src2 - filled array';
  var src1 = new Set( [] );
  var src2 = [ [ 0, 1, 'str' ], new Set( [ '', undefined, null ] ), [ false ] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - empty array';
  var src1 = new Set( [ new Set( [ 0, 1, 'str' ] ), [ '', undefined, null ], [ false ] ] );
  var src2 = [];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = new Set( [ new Set( [ 0, 1, 'str' ] ), [ '', undefined, null ], [ false ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = [ [ 0, 1, 'str', '' ], new Set( [ undefined, null, false ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], new Set( [ [ false ] ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), new Set( [ [ false ] ] ) ] );
  var got = _.arraySet.containSetsAny( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'src' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, 2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, 3, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), new Set( [ [ false ] ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsAny( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, false );

  test.close( 'mixed' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAny() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAny( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAny( [ 1, 2 ], new Set( [] ), ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAny( 'wrong', new Set( [] ) ) );

  test.case = 'wrong type of src2';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsAny( new Set( [] ), 'wrong' ) );
}

//

function containSetsNone( test )
{
  test.open( 'array' );

  test.case = 'src1 - empty array, src2 - empty array';
  var src1 = [];
  var src2 = [];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - array with empty arrays and Sets';
  var src1 = [];
  var src2 = [ [], new Set(), [] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - filled array';
  var src1 = [];
  var src2 = [ [ 0, 1 ], new Set( [ 'str', '', undefined ] ), [ null, false ] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - empty array';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - array with empty arrays and Sets';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ new Set(), [], new Set() ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = [ [ 0, 1, 'str' ], new Set( [ '', undefined, null ] ), [ false ] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content';
  var src1 = [ -1, 1, 'str', '', undefined, null, false ];
  var src2 = [ [ 0, 1, 'str' ], new Set( [ '', undefined, null ] ), [ false ] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), new Set( [ [ false ] ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], [ [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], new Set( [ [ false ] ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), new Set( [ [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), new Set( [ [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), new Set( [ [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'src' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, 2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsNone( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = [ [ [ 0 ], [ 1 ] ], [ [ 'str' ], [ '' ] ], new Set( [ [ undefined ], [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.close( 'array' );

  /* - */

  test.open( 'Set' );

  test.case = 'src1 - empty Set, src2 - empty Set';
  var src1 = new Set( [] );
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - Set without arrays and Sets';
  var src1 = new Set( [] );
  var src2 = new Set( [ [], new Set(), [], new Set() ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled Set';
  var src1 = new Set( [] );
  var src2 = new Set( [ new Set( [ 0, 1, 'str' ] ), [ '', undefined, null ], new Set( [ false ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - empty Set';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - Set with empty arrays and Sets';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ new Set(), [], new Set() ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ [ 0, 1, 'str' ], new Set( [ '', undefined, null ] ), [ false ] ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = new Set( [ new Set( [ 0, 1, 'str' ] ), [ '', undefined, null ], new Set( [ false ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), [ [ false ] ] ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ], [ null ] ], new Set( [ [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ], [ null ] ] ), new Set( [ [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'src' } ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, 2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled Set, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.close( 'Set' );

  /* - */

  test.open( 'mixed' );

  test.case = 'src1 - empty array, src2 - empty Set';
  var src1 = [];
  var src2 = new Set( [] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty array, src2 - Set with empty arrays and Sets';
  var src1 = [];
  var src2 = new Set( [ new Set(), [], new Set() ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - empty Set, src2 - filled array';
  var src1 = new Set( [] );
  var src2 = [ [ 0, 1, 'str' ], new Set( [ '', undefined ] ), new Set( [ null, false ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - empty array';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = [];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - array with empty arrays and Sets';
  var src1 = new Set( [ 0, 1, 'str', '', undefined, null, false ] );
  var src2 = [ [], new Set(), [] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content';
  var src1 = [ 0, 1, 'str', '', undefined, null, false ];
  var src2 = new Set( [ new Set( [ 0, 1, 'str' ] ), [ '', undefined, null ], new Set( [ false ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content';
  var src1 = new Set( [ -1, 1, 'str', '', undefined, null, false ] );
  var src2 = [ new Set( [ 0, 1, 'str' ] ), [ '', undefined ], [ null, false ] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = new Set( [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2 );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), new Set( [ [ null ], [ false ] ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, one evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, one evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a );
  test.identical( got, false );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e[ 0 ], ( ins ) => 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, two evaluators';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, two evaluators';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2, 3, ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, 7, ( e ) => e[ 0 ] );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, identical content, complex data, fromIndex and evaluator';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'src' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, 2, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, not identical content, complex data, fromIndex and evaluator';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, 3, ( e ) => e.a );
  test.identical( got, true );

  /* */

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ [ 0 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ];
  var src2 = new Set( [ [ [ 0 ], [ 1 ], [ 'str' ] ], new Set( [ [ '' ], [ undefined ] ] ), new Set( [ [ null ], [ false ] ] ) ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ [ -1 ], [ 1 ], [ 'str' ], [ '' ], [ undefined ], [ null ], [ false ] ] );
  var src2 = [ new Set( [ [ 0 ], [ 1 ], [ 'str' ] ] ), [ [ '' ], [ undefined ] ], [ [ null ], [ false ] ] ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e[ 0 ] > 5 );
  test.identical( got, true );

  test.case = 'src1 - filled array, src2 - filled Set, identical content, complex data, equalizer';
  var src1 = [ { a : 0 }, { a : 1 }, { a : 'str' } ];
  var src2 = new Set( [ new Set( [ { a : 0 }, { a : 1 } ] ), [ { a : 'str' } ] ] );
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.case = 'src1 - filled Set, src2 - filled array, not identical content, complex data, equalizer';
  var src1 = new Set( [ { a : 0 }, { a : 1 }, { a : 'str' } ] );
  var src2 = [ [ { a : 0 }, { a : 1 } ], new Set( [ { a : 'str' } ] ) ];
  var got = _.arraySet.containSetsNone( src1, src2, ( e, ins ) => e.a === e.b );
  test.identical( got, true );

  test.close( 'mixed' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsNone() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsNone( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsNone( [ 1, 2 ], new Set( [] ), ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong type of src1';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsNone( 'wrong', new Set( [] ) ) );

  test.case = 'wrong type of src2';
  test.shouldThrowErrorSync( () => _.arraySet.containSetsNone( new Set( [] ), 'wrong' ) );
}

//

function identical( test )
{

  console.log( 'xxx' );

  test.case = '2 arguments, empty';
  var a = [];
  var b = [];
  var got = _.arraySet.identical( a, b );
  var expected = true;
  test.identical( got, expected );

  test.case = '2 arguments, src empty';
  var a = [];
  var b = [ 1 ];
  var got = _.arraySet.identical( a, b );
  var expected = false;
  test.identical( got, expected );

  test.case = '2 arguments, ins empty';
  var a = [ 1 ];
  var b = [];
  var got = _.arraySet.identical( a, b );
  var expected = false;
  test.identical( got, expected );

  test.case = 'repeats, bigger second argument';
  var a = [ 1 ];
  var b = [ 1, 1, 1, 1 ];
  var got = _.arraySet.identical( a, b );
  var expected = false;
  test.identical( got, expected );

  test.case = 'repeats, bigger first argument';
  var a = [ 1, 1, 1, 1 ];
  var b = [ 1 ];
  var got = _.arraySet.identical( a, b );
  var expected = false;
  test.identical( got, expected );

  test.case = 'repeats';
  var a = [ 1, 3 ];
  var b = [ 1, 1, 1, 1 ];
  var got = _.arraySet.identical( a, b );
  var expected = false;
  test.identical( got, expected );

  test.case = 'arguments have the same elements but the order is differ';
  var a = [ 1, 2, 4, 7, 5 ];
  var b = [ 4, 2, 1, 5, 7 ];
  var got = _.arraySet.identical( a, b );
  var expected = true;
  test.identical( got, expected );
  test.true( got !== a );
  test.true( got !== b );

  test.case = 'argument length mismatch';
  var a = [ 1, 2, 4, 7, 5 ];
  var b = [ 1, 5, 7 ];
  var got = _.arraySet.identical( a, b );
  var expected = false;
  test.identical( got, expected );
  test.true( got !== a );
  test.true( got !== b );

  test.case = 'arguments have the same elements have inner arrays';
  var a = [ 1, 2, [ 1, 3 ], 7, 5 ];
  var b = [ [ 1, 3 ], 2, 1, 5, 7 ];
  var got = _.arraySet.identical( a, b );
  var expected = false;
  test.identical( got, expected );
  test.true( got !== a );
  test.true( got !== b );

  test.case = 'both arrays are empty';
  var a = [];
  var b = [];
  var got = _.arraySet.identical( a, b );
  var expected = true;
  test.identical( got, expected );
  test.true( got !== a );
  test.true( got !== b );

  /* */

  var cases =
  [
    { args : [ [], [] ], expected : true },
    { args : [ [ 1 ], [] ], expected : false },
    { args : [ [ 1 ], [ 1 ] ], expected : true },
    { args : [ [ 1, 1 ], [ 1 ] ], expected : false },
    { args : [ [ 1 ], [ 1, 1 ] ], expected : false },
    { args : [ [ 1, 2, 3 ], [ 1, 2, 4 ] ], expected : false },
    { args : [ [ 1, 2, 3 ], [ 3, 2, 1 ] ], expected : true },
    { args : [ [ 1, 2, 3 ], [ 3, 2, 1 ] ], expected : true },
    { args : [ [ [ 1 ], 2, 3 ], [ 3, 2, [ 1 ] ] ], expected : false },
    { args : [ 1, [ 1 ] ], error : true },
    { args : [ [ 1 ], 1 ], error : true },
  ]

  for( var i = 0; i < cases.length; i++ )
  {
    var c = cases[ i ];

    if( c.error )
    test.shouldThrowErrorSync( () => _.arraySet.identical.apply( _, c.args ) );
    else
    test.identical( _.arraySet.identical.apply( _, c.args ), c.expected );
  }

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arraySet.identical();
  });

  test.case = 'one or 2 arguments are not longIs entities, numeric argument';
  test.shouldThrowErrorSync( function()
  {
    _.arraySet.identical( [ 1, 2, 4, 7, 5 ], 15 );
  });

  test.case = 'one or 2 arguments are not longIs entities, string like arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arraySet.identical( 'a', 'a' );
  });

  test.case = 'one or 2 arguments are not longIs entities, map like arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arraySet.identical( { a : 1 }, { b : 3, c : 8 } );
  });

  test.case = 'wrong argument';
  test.shouldThrowErrorSync( function()
  {
    _.arraySet.identical( [ 1, 2, 4, 7, 5 ], 'wrong argument' );
  });

  test.case = 'both arguments are null';
  test.shouldThrowErrorSync( function()
  {
    _.arraySet.identical( null, null );
  });

  test.case = 'both arguments are undefined';
  test.shouldThrowErrorSync( function()
  {
    _.arraySet.identical( undefined, undefined );
  });

}

//

function left( test )
{
  test.open( 'Set' );

  test.case = 'empty container';
  var src = new Set( [] );
  var got = _.arraySet.left( src, 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ] );
  var got = _.arraySet.left( src, 3 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, 4 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.left( src, 'str', 4 );
  test.identical( got, { index : -1 } );

  test.case = 'searches complex data, fromIndex';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, ( e ) => typeof e );
  test.identical( got, { index : 0, element : 1 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.left( src, 'str', ( e ) => e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, 2, ( e ) => typeof e );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.left( src, 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : -1 } );

  test.case = 'searches complex data, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.left( src, 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.left( src, 2, 5, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.left( src, 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.left( src, 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.close( 'Set' );

  /* */

  test.open( 'array' );

  test.case = 'empty container';
  var src = [];
  var got = _.arraySet.left( src, 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ];
  var got = _.arraySet.left( src, 3 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, 4 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.left( src, 'str', 4 );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, ( e ) => typeof e );
  test.identical( got, { index : 0, element : 1 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.left( src, 'str', ( e ) => e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, 2, ( e ) => typeof e );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.left( src, 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.left( src, 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src =[ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.left( src, 2, 7, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.left( src, 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.left( src, 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.close( 'array' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.left() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.left( [ 1, 2 ], 1, 0, ( e ) => e, ( ins ) => ins, 'extra' ) );
  test.shouldThrowErrorSync( () => _.arraySet.left( new Set(), 1, 0, ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () => _.arraySet.left( [ 1, 2 ], 1, 0, 'wrong' ) );
  test.shouldThrowErrorSync( () => _.arraySet.left( new Set( [ 1, 2 ] ), 1, 0, 'wrong' ) );

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () => _.arraySet.left( [ 1, 2 ], 1, 0, () => 1 ) );
  test.shouldThrowErrorSync( () => _.arraySet.left( new Set( [ 1, 2 ] ), 1, 0, () => 1 ) );

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () => _.arraySet.left( [ 1, 2 ], 1, 0, ( e ) => e, () => 1 ) );
  test.shouldThrowErrorSync( () => _.arraySet.left( new Set( [ 1, 2 ] ), 1, 0, ( e ) => e, () => 1 ) );
}

//

function right( test )
{
  test.open( 'Set' );

  test.case = 'empty container';
  var src = new Set( [] );
  var got = _.arraySet.right( src, 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ] );
  var got = _.arraySet.right( src, 3 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, 4 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.right( src, 'str', 4 );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.right( src, 'str', ( e ) => e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 }, [ 3 ] ] );
  var got = _.arraySet.right( src, [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 5, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, 2, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.right( src, 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 }, [ 3 ] ] );
  var got = _.arraySet.right( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 5, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.right( src, 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = new Set( [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.right( src, 2, 5, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : -1 } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = new Set( [ 1, 2, { a : 2 }, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = _.arraySet.right( src, 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 6, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = new Set( [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ] );
  var got = _.arraySet.right( src, 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  test.close( 'Set' );

  /* */

  test.open( 'array' );

  test.case = 'empty container';
  var src = [];
  var got = _.arraySet.right( src, 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ];
  var got = _.arraySet.right( src, 3 );
  test.identical( got, { index : 5, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, 4 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.right( src, 'str', 4 );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.right( src, 'str', ( e ) => e );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 4, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, 2, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.right( src, 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, { a : 2 }, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.right( src, 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 8, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src =[ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 2, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, { a : 2 }, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.right( src, 2, 7, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 3, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.arraySet.right( src, 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.arraySet.right( src, 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  test.close( 'array' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arraySet.right() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arraySet.right( [ 1, 2 ], 1, 0, ( e ) => e, ( ins ) => ins, 'extra' ) );
  test.shouldThrowErrorSync( () => _.arraySet.right( new Set(), 1, 0, ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () => _.arraySet.right( [ 1, 2 ], 1, 0, 'wrong' ) );
  test.shouldThrowErrorSync( () => _.arraySet.right( new Set( [ 1, 2 ] ), 1, 1, 'wrong' ) );

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () => _.arraySet.right( [ 1, 2 ], 1, 0, () => 1 ) );
  test.shouldThrowErrorSync( () => _.arraySet.right( new Set( [ 1, 2 ] ), 1, 2, () => 1 ) );

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () => _.arraySet.right( [ 1, 2 ], 1, 0, ( e ) => e, () => 1 ) );
  test.shouldThrowErrorSync( () => _.arraySet.right( new Set( [ 1, 2 ] ), 1, 1, ( e ) => e, () => 1 ) );
}

// --
//
// --

const Proto =
{

  name : 'Tools.ArraySet.l0.l9',
  silencing : 1,
  enabled : 1,

  tests :
  {

    // array set

    /* xxx : move out */
    // arraySetDiff,
    diff_,
    // arraySetBut,
    but_,
    // arraySetIntersection,
    intersection_,
    // arraySetUnion,
    union_,

    // arraySetContainAll,
    containAll_,
    // arraySetContainAny,
    containAny_,
    containNone_,
    identical,

    containSetsAll,
    containSetsAny,
    containSetsNone,

    left,
    right,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
