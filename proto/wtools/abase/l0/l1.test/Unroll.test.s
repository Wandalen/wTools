( function _l0_l1_Unroll_test_s_( )
{

'use strict';

if( typeof module !== 'undefined' )
{
  require( '../Include1.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
// unroll
// --

/* qqq : merge test routine with other similar into test routine dichotomy */
function is( test )
{
  test.case = 'unroll from empty array';

  var src = [];
  var got = _.unroll.make( src );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  test.case = 'unroll from not empty array';

  var src = [ 1 ];
  var got = _.unroll.make( src );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  var src = [ 'str' ];
  var got = _.unroll.from( src );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  test.case = 'not unroll';

  var got = new F32x( [ 3, 1, 2 ] );
  test.identical( _.unrollIs( got ), false );

  test.identical( _.unrollIs( [] ), false );

  test.identical( _.unrollIs( 1 ), false );

  test.identical( _.unrollIs( 'str' ), false );

  test.case = 'second argument is unroll';

  var got = _.unroll.make( [ 2, 4 ] );
  test.identical( _.unrollIs( [ 1, 'str' ], got ), false );
  test.true( _.arrayIs( got ) );

  var got = _.unroll.from( [ 2, 4 ] );
  test.identical( _.unrollIs( 1, got ), false );
  test.true( _.arrayIs( got ) );

  var got = _.unroll.make( [ 2, 4 ] );
  test.identical( _.unrollIs( 'str', got ), false );
  test.true( _.arrayIs( got ) );

}

//

function isPopulated( test )
{
  test.case = 'unroll from not empty array';

  var src = [ 1 ];
  var got = _.unroll.make( src );
  test.true( _.unrollIsPopulated( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  var src = [ 'str' ];
  var got = _.unroll.from( src );
  test.true( _.unrollIsPopulated( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  var src = [ [] ];
  var got = _.unroll.from( src );
  test.true( _.unrollIsPopulated( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  var src = [ null ];
  var got = _.unroll.make( src );
  test.true( _.unrollIsPopulated( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  test.case = 'unroll from empty array';
  var src = [];
  var got = _.unroll.from( src );
  test.identical( _.unrollIsPopulated( got ), false );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  test.case = 'not unroll';

  var got = new F32x( [ 3, 1, 2 ] );
  test.identical( _.unrollIs( got ), false );

  test.identical( _.unrollIsPopulated( [] ), false );

  test.identical( _.unrollIsPopulated( 1 ), false );

  test.identical( _.unrollIsPopulated( 'str' ), false );

  test.case = 'second argument is unroll';

  var got = _.unroll.make( [ 2, 4 ] );
  test.identical( _.unrollIsPopulated( [ 1, 'str' ], got ), false );
  test.true( _.arrayIs( got ) );

  var got = _.unroll.from( [ 2, 4 ] );
  test.identical( _.unrollIsPopulated( 1, got ), false );
  test.true( _.arrayIs( got ) );

  var got = _.unroll.make( [ 2, 4 ] );
  test.identical( _.unrollIsPopulated( 'str', got ), false );
  test.true( _.arrayIs( got ) );
}

//

function make( test )
{
  test.case = 'without arguments';
  var got = _.unroll.make();
  test.equivalent( got, [] );
  test.true( _.unrollIs( got ) );

  test.case = 'src - null';
  var got = _.unroll.make( null );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  // test.case = 'src - undefined';
  // var got = _.unroll.make( undefined );
  // test.equivalent( got, [] );
  // test.true( _.arrayIs( got ) );
  // test.true( _.unrollIs( got ) );

  test.case = 'src - zero';
  var got = _.unroll.make( 0 );
  var expected = new Array( 0 );
  test.equivalent( got, expected );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( expected !== got );

  test.case = 'src - number > 0';
  var got = _.unroll.make( 3 );
  var expected = new Array( 3 );
  test.equivalent( got, expected );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( expected !== got );

  test.case = 'empty array';
  var src = [];
  var got = _.unroll.make( src );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - array, array.length - 1';
  var src = [ 0 ];
  var got = _.unroll.make( src );
  test.equivalent( got, [ 0 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - array, array.length > 1';
  var src = [ 1, 2, 3, 'str' ];
  var got = _.unroll.make( src );
  test.equivalent( got, [ 1, 2, 3, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'from empty BufferTyped - F32x';
  var src = new F32x();
  var got = _.unroll.make( src );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'from Float32 - U8x';
  var src = new U8x( [ 1, 2, 3 ] );
  var got = _.unroll.make( src );
  test.equivalent( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'from empty arguments array';
  var src = _.argumentsArray.make( [] );
  var got = _.unroll.make( src );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'from arguments array';
  var src = _.argumentsArray.make( 3 );
  var got = _.unroll.make( src );
  test.equivalent( got, [ undefined, undefined, undefined ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'from empty unroll';
  var src = _.unroll.make( [] );
  var got = _.unroll.make( src );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'from unroll';
  var src = _.unroll.make( [ 1, 2, 3, 'str' ] );
  var got = _.unroll.make( src );
  test.equivalent( got, [ 1, 2, 3, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* qqq : for junior : add similar test case for all *.make* and *.from* routines of all namespaces */
  test.case = 'from countable';
  var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
  var got = _.unroll.make( src );
  test.equivalent( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.unroll.make( 1, 'extra' ) );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.unroll.make( {} ) );
  test.shouldThrowErrorSync( () => _.unroll.make( 'wrong' ) );

  /* qqq : for junior : add similar test case for all *.make* and *.from* routines */
  test.shouldThrowErrorSync( () => _.unroll.make( undefined ) );

}

//

function makeLongDescriptor( test )
{
  let times = 4;

  debugger;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let name = namespace.TypeName;
    let long = _.withLong[ name ];

    test.open( `long - ${ name }` );
    testRun( long );
    test.close( `long - ${ name }` );

    if( times < 1 )
    break;
    times--;
  }

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;
  //   let long = _.withLong[ name ];
  //
  //   test.open( `long - ${ name }` );
  //   testRun( long );
  //   test.close( `long - ${ name }` );
  //
  //   if( times < 1 )
  //   break;
  //   times--;
  // }

  /* - */

  function testRun( long )
  {

    test.case = 'without arguments';
    var got = long.unroll.make();
    test.equivalent( got, [] );
    test.true( _.unrollIs( got ) );

    test.case = 'src - null';
    var got = long.unroll.make( null );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );

    test.case = 'src - zero';
    var got = long.unroll.make( 0 );
    var expected = new Array( 0 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( expected !== got );

    test.case = 'src - number > 0';
    var got = long.unroll.make( 3 );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( expected !== got );

    test.case = 'empty array';
    var src = [];
    var got = long.unroll.make( src );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - array, array.length - 1';
    var src = [ 0 ];
    var got = long.unroll.make( src );
    test.equivalent( got, [ 0 ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - array, array.length > 1';
    var src = [ 1, 2, 3, 'str' ];
    var got = long.unroll.make( src );
    test.equivalent( got, [ 1, 2, 3, 'str' ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'from empty BufferTyped - F32x';
    var src = new F32x();
    var got = long.unroll.make( src );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'from Float32 - U8x';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = long.unroll.make( src );
    test.equivalent( got, [ 1, 2, 3 ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'from empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = long.unroll.make( src );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'from arguments array';
    var src = _.argumentsArray.make( 3 );
    var got = long.unroll.make( src );
    test.equivalent( got, [ undefined, undefined, undefined ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'from empty unroll';
    var src = long.unroll.make( [] );
    var got = long.unroll.make( src );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'from unroll';
    var src = long.unroll.make( [ 1, 2, 3, 'str' ] );
    var got = long.unroll.make( src );
    test.equivalent( got, [ 1, 2, 3, 'str' ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* - */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.unroll.make( 1, 'extra' ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.unroll.make( {} ) );
      test.shouldThrowErrorSync( () => long.unroll.make( 'wrong' ) );
      test.shouldThrowErrorSync( () => long.unroll.make( undefined ) );
    }

  }
}

//

function makeUndefined( test )
{
  test.case = 'without arguments';
  var got = _.unroll.makeUndefined();
  var expected = _.unroll.make();
  test.identical( got, expected );

  test.case = 'src - null';
  var src = null;
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );
  //
  // test.case = 'src - null, length - null';
  // var src = null;
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( 0 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );
  //
  // test.case = 'src - null, length - undefined';
  // var src = null;
  // var got = _.unroll.makeUndefined( src, undefined );
  // var expected = _.unroll.make( 0 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );

  test.case = 'src - null, length - number';
  var src = null;
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - null, length - long';
  var src = null;
  var got = _.unroll.makeUndefined( src, new U8x( 4 ) );
  var expected = _.unroll.make( 4 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* */

  test.case = 'src - number';
  var src = 5;
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 5 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  // test.case = 'src - number, length - null';
  // var src = 5;
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( 5 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );
  //
  // test.case = 'src - null, length - undefined';
  // var src = 5;
  // var got = _.unroll.makeUndefined( src, undefined );
  // var expected = _.unroll.make( 5 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );

  /* */

  test.case = 'src - empty array';
  var src = [];
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  // test.case = 'src - empty array, length - null';
  // var src = [];
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( [] );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );

  test.case = 'src - empty array, length - 2';
  var src = [];
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - array, src.length - 1';
  var src = [ 0 ];
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - array, src.length - 1, length - 2';
  var src = [ 0 ];
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - array, src.length > 1';
  var src = [ 1, 2, 3 ];
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - array, src.length > 1, length < src.length';
  var src = [ 1, 2, 3 ];
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* */

  test.case = 'src - empty U8x';
  var src = new U8x();
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  // test.case = 'src - empty U8x, length - null';
  // var src = new U8x();
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( [] );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );

  test.case = 'src - empty U8x, length - 2';
  var src = new U8x();
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - U8x, src.length - 1';
  var src = new U8x( 1 );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - U8x, src.length - 1, length > src.length';
  var src = new U8x( 1 );
  var got = _.unroll.makeUndefined( src, 3 );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - U8x, src.length > 1';
  var src = new U8x( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - U8x, src.length > 1, length < src.length';
  var src = new U8x( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src, 0 );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* */

  test.case = 'src - empty I16x';
  var src = new I16x();
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  // test.case = 'src - empty I16x, length - null';
  // var src = new I16x();
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( 0 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );

  test.case = 'src - empty I16x, length - 2';
  var src = new I16x();
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - I16x, src.length - 1';
  var src = new I16x( 1 );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - I16x, src.length - 1, length > src.length';
  var src = new I16x( 1 );
  var got = _.unroll.makeUndefined( src, 3 );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - I16x, src.length > 1';
  var src = new I16x( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - I16x, src.length > 1, length < src.length';
  var src = new I16x( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src, 0 );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* */

  test.case = 'src - empty F32x';
  var src = new F32x();
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  // test.case = 'src - empty F32x, length - null';
  // var src = new F32x();
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( 0 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );

  test.case = 'src - empty F32x, length - 2';
  var src = new F32x();
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - F32x, src.length - 1';
  var src = new F32x( 1 );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - F32x, src.length - 1, length > src.length';
  var src = new F32x( 1 );
  var got = _.unroll.makeUndefined( src, 3 );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - F32x, src.length > 1';
  var src = new F32x( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - F32x, src.length > 1, length < src.length';
  var src = new F32x( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src, 0 );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* */

  test.case = 'src - empty arguments array';
  var src = _.argumentsArray.make( [] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( !_.argumentsArray.is( got ) );
  test.true( src !== got );

  // test.case = 'src - empty arguments array, length - null';
  // var src = _.argumentsArray.make( [] );
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( 0 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( !_.argumentsArray.is( got ) );
  // test.true( src !== got );

  test.case = 'src - empty arguments array, length - number > 0';
  var src = _.argumentsArray.make( [] );
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( !_.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'src - arguments array, src.length - 1';
  var src = _.argumentsArray.make( [ {} ] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( !_.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'src - arguments array, src.length - 1, length > src.length';
  var src = _.argumentsArray.make( [ {} ] );
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( !_.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'src - arguments array, src.length > 1';
  var src = _.argumentsArray.make( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( !_.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'src - arguments array, src.length > 1, length < src.length';
  var src = _.argumentsArray.make( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src, 1 );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( !_.argumentsArray.is( got ) );
  test.true( src !== got );

  /* */

  test.case = 'src - empty unroll';
  var src = _.unroll.make( [] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( [] );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  // test.case = 'src - empty unroll, length - null';
  // var src = _.unroll.make( [] );
  // var got = _.unroll.makeUndefined( src, null );
  // var expected = _.unroll.make( 0 );
  // test.equivalent( got, expected );
  // test.true( _.unrollIs( got ) );
  // test.true( src !== got );

  test.case = 'src - empty unroll, length - 2';
  var src = _.unroll.make( [] );
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - unroll, src.length - 1';
  var src = _.unroll.make( [ 'str' ] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - unroll, src.length - 1, length > src.length';
  var src = _.unroll.make( [ 'str' ] );
  var got = _.unroll.makeUndefined( src, 2 );
  var expected = _.unroll.make( 2 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - unroll, src.length > 1';
  var src = _.unroll.make( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src );
  var expected = _.unroll.make( 3 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - unroll, src.length > 1, length < src.length';
  var src = _.unroll.make( [ 1, 2, 3 ] );
  var got = _.unroll.makeUndefined( src, 1 );
  var expected = _.unroll.make( 1 );
  test.equivalent( got, expected );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( 1, 3, 'extra' ) );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( undefined ) );
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( {} ) );
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( 'wrong' ) );

  test.case = 'wrong length';
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( [], null ) );
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( [], undefined ) );
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( [], Infinity ) );

  test.case = 'wrong type of length';
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( [], 'wrong' ) );
  test.shouldThrowErrorSync( () => _.unroll.makeUndefined( [], {} ) );
}

//

function makeUndefinedLongDescriptor( test )
{
  let times = 4;

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;

  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let name = namespace.TypeName;
    let long = _.withLong[ name ];

    test.open( `long - ${ name }` );
    testRun( long );
    test.close( `long - ${ name }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function testRun( long )
  {
    test.case = 'without arguments';
    var got = _.unroll.makeUndefined();
    var expected = _.unroll.make();
    test.identical( got, expected );

    test.case = 'src - null';
    var src = null;
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    // test.case = 'src - null, length - null';
    // var src = null;
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( 0 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );
    //
    // test.case = 'src - null, length - undefined';
    // var src = null;
    // var got = long.unroll.makeUndefined( src, undefined );
    // var expected = _.unroll.make( 0 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );

    test.case = 'src - null, length - number';
    var src = null;
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - null, length - long';
    var src = null;
    var got = long.unroll.makeUndefined( src, new U8x( 4 ) );
    var expected = _.unroll.make( 4 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src - number';
    var src = 5;
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 5 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    // test.case = 'src - number, length - null';
    // var src = 5;
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( 5 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );
    //
    // test.case = 'src - null, length - undefined';
    // var src = 5;
    // var got = long.unroll.makeUndefined( src, undefined );
    // var expected = _.unroll.make( 5 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );

    /* */

    test.case = 'src - empty array';
    var src = [];
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    // test.case = 'src - empty array, length - null';
    // var src = [];
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( [] );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );

    test.case = 'src - empty array, length - 2';
    var src = [];
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - array, src.length - 1';
    var src = [ 0 ];
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - array, src.length - 1, length - 2';
    var src = [ 0 ];
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - array, src.length > 1';
    var src = [ 1, 2, 3 ];
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - array, src.length > 1, length < src.length';
    var src = [ 1, 2, 3 ];
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src - empty U8x';
    var src = new U8x();
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    // test.case = 'src - empty U8x, length - null';
    // var src = new U8x();
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( [] );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );

    test.case = 'src - empty U8x, length - 2';
    var src = new U8x();
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - U8x, src.length - 1';
    var src = new U8x( 1 );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - U8x, src.length - 1, length > src.length';
    var src = new U8x( 1 );
    var got = long.unroll.makeUndefined( src, 3 );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - U8x, src.length > 1';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - U8x, src.length > 1, length < src.length';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src, 0 );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src - empty I16x';
    var src = new I16x();
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    // test.case = 'src - empty I16x, length - null';
    // var src = new I16x();
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( 0 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );

    test.case = 'src - empty I16x, length - 2';
    var src = new I16x();
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - I16x, src.length - 1';
    var src = new I16x( 1 );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - I16x, src.length - 1, length > src.length';
    var src = new I16x( 1 );
    var got = long.unroll.makeUndefined( src, 3 );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - I16x, src.length > 1';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - I16x, src.length > 1, length < src.length';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src, 0 );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src - empty F32x';
    var src = new F32x();
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    // test.case = 'src - empty F32x, length - null';
    // var src = new F32x();
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( 0 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );

    test.case = 'src - empty F32x, length - 2';
    var src = new F32x();
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - F32x, src.length - 1';
    var src = new F32x( 1 );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - F32x, src.length - 1, length > src.length';
    var src = new F32x( 1 );
    var got = long.unroll.makeUndefined( src, 3 );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - F32x, src.length > 1';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - F32x, src.length > 1, length < src.length';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src, 0 );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src - empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    // test.case = 'src - empty arguments array, length - null';
    // var src = _.argumentsArray.make( [] );
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( 0 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( !_.argumentsArray.is( got ) );
    // test.true( src !== got );

    test.case = 'src - empty arguments array, length - number > 0';
    var src = _.argumentsArray.make( [] );
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src - arguments array, src.length - 1';
    var src = _.argumentsArray.make( [ {} ] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src - arguments array, src.length - 1, length > src.length';
    var src = _.argumentsArray.make( [ {} ] );
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src - arguments array, src.length > 1';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src - arguments array, src.length > 1, length < src.length';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src, 1 );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src - empty unroll';
    var src = _.unroll.make( [] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( [] );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    // test.case = 'src - empty unroll, length - null';
    // var src = _.unroll.make( [] );
    // var got = long.unroll.makeUndefined( src, null );
    // var expected = _.unroll.make( 0 );
    // test.equivalent( got, expected );
    // test.true( _.unrollIs( got ) );
    // test.true( src !== got );

    test.case = 'src - empty unroll, length - 2';
    var src = _.unroll.make( [] );
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - unroll, src.length - 1';
    var src = _.unroll.make( [ 'str' ] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - unroll, src.length - 1, length > src.length';
    var src = _.unroll.make( [ 'str' ] );
    var got = long.unroll.makeUndefined( src, 2 );
    var expected = _.unroll.make( 2 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - unroll, src.length > 1';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src );
    var expected = _.unroll.make( 3 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - unroll, src.length > 1, length < src.length';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = long.unroll.makeUndefined( src, 1 );
    var expected = _.unroll.make( 1 );
    test.equivalent( got, expected );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* - */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( 1, 3, 'extra' ) );

      test.case = 'wrong length';
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( [], null ) );
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( [], undefined ) );
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( [], Infinity ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( {} ) );
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( 'wrong' ) );

      test.case = 'wrong type of length';
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( [], 'wrong' ) );
      test.shouldThrowErrorSync( () => long.unroll.makeUndefined( [], {} ) );
    }
  }
}

//

function from( test )
{

  test.case = 'src - null';
  var got = _.unroll.from( null );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  // test.case = 'src - undefined';
  // var got = _.unroll.from( undefined );
  // test.equivalent( got, [] );
  // test.true( _.arrayIs( got ) );
  // test.true( _.unrollIs( got ) );

  test.case = 'src - zero';
  var got = _.unroll.from( 0 );
  var expected = new Array( 0 );
  test.equivalent( got, expected );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( expected !== got );

  test.case = 'src - number > 0';
  var got = _.unroll.from( 3 );
  var expected = new Array( 3 );
  test.equivalent( got, expected );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( expected !== got );

  test.case = 'src - empty unroll';
  var src = _.unroll.make( 0 );
  var got = _.unroll.from( src );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( got === src );

  test.case = 'src - unroll, src.length > 0';
  var src = _.unroll.make( 2 );
  var got = _.unroll.from( src );
  test.identical( got, [ undefined, undefined ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( got === src );

  test.case = 'src - filled unroll';
  var src = _.unroll.make( [ 1, 'str', 3 ] );
  var got = _.unroll.from( src );
  test.identical( got, [ 1, 'str', 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( got === src );

  test.case = 'src - empty array';
  var src = [];
  var got = _.unroll.from( src );
  test.equivalent( got, src );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - array with single element';
  var src = [ 0 ];
  var got = _.unroll.from( src );
  test.equivalent( got, src );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - arrray with several elements';
  var src = [ 1, 2, 'str' ];
  var got = _.unroll.from( src );
  test.equivalent( got, src );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - empty BufferTyped - F32x';
  test.case = 'from F32x';
  var src = new F32x();
  var got = _.unroll.from( src );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - filled BufferTyped - I16x';
  var src = new I16x( [ 1, 2, 3 ] );
  var got = _.unroll.from( src );
  test.equivalent( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - empty argumentsArray';
  var src = _.argumentsArray.make( [] );
  var got = _.unroll.from( src );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'src - filled argumentsArray';
  var src = _.argumentsArray.make( [ 1, 2, 3 ] );
  var got = _.unroll.from( src );
  test.equivalent( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'from countable';
  var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
  var got = _.unroll.from( src );
  test.equivalent( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );
  test.true( src !== got );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.unroll.from() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.unroll.from( 1, 3 ) );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.unroll.from( {} ) );
  test.shouldThrowErrorSync( () => _.unroll.from( 'wrong' ) );
  test.shouldThrowErrorSync( () => _.unroll.from( -1 ) );
  test.shouldThrowErrorSync( () => _.unroll.from( undefined ) );

}

//

function fromLongDescriptor( test )
{
  let times = 4;


  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;

  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let name = namespace.TypeName;
    let long = _.withLong[ name ];

    test.open( `long - ${ name }` );
    testRun( long );
    test.close( `long - ${ name }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function testRun( long )
  {
    test.case = 'src - null';
    var got = long.unroll.from( null );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );

    // test.case = 'src - undefined';
    // var got = long.unroll.from( undefined );
    // test.equivalent( got, [] );
    // test.true( _.arrayIs( got ) );
    // test.true( _.unrollIs( got ) );

    test.case = 'src - zero';
    var got = long.unroll.from( 0 );
    var expected = new Array( 0 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( expected !== got );

    test.case = 'src - number > 0';
    var got = long.unroll.from( 3 );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( expected !== got );

    test.case = 'src - empty unroll';
    var src = _.unroll.make( 0 );
    var got = long.unroll.from( src );
    test.identical( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( got === src );

    test.case = 'src - unroll, src.length > 0';
    var src = _.unroll.make( 2 );
    var got = long.unroll.from( src );
    test.identical( got, [ undefined, undefined ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( got === src );

    test.case = 'src - filled unroll';
    var src = _.unroll.make( [ 1, 'str', 3 ] );
    var got = long.unroll.from( src );
    test.identical( got, [ 1, 'str', 3 ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( got === src );

    test.case = 'src - empty array';
    var src = [];
    var got = long.unroll.from( src );
    test.equivalent( got, src );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - array with single element';
    var src = [ 0 ];
    var got = long.unroll.from( src );
    test.equivalent( got, src );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - arrray with several elements';
    var src = [ 1, 2, 'str' ];
    var got = long.unroll.from( src );
    test.equivalent( got, src );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - empty BufferTyped - F32x';
    test.case = 'from F32x';
    var src = new F32x();
    var got = long.unroll.from( src );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - filled BufferTyped - I16x';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = long.unroll.from( src );
    test.equivalent( got, [ 1, 2, 3 ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - empty argumentsArray';
    var src = _.argumentsArray.make( [] );
    var got = long.unroll.from( src );
    test.equivalent( got, [] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src - filled argumentsArray';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.unroll.from( src );
    test.equivalent( got, [ 1, 2, 3 ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src !== got );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.unroll.from() );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.unroll.from( 1, 3 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.unroll.from( {} ) );
      test.shouldThrowErrorSync( () => long.unroll.from( 'wrong' ) );
      test.shouldThrowErrorSync( () => long.unroll.from( undefined ) );
      test.shouldThrowErrorSync( () => long.unroll.from( null, null ) );
      test.shouldThrowErrorSync( () => long.unroll.from( [], null ) );
      test.shouldThrowErrorSync( () => long.unroll.from( -1 ) );
    }
  }
}

// //
//
// function unrollsFrom( test )
// {
//   test.case = 'srcs - null';
//   var got = _.unrollsFrom( null );
//   test.equivalent( got, [ [] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//
//   // test.case = 'srcs - undefined';
//   // var got = _.unrollsFrom( undefined );
//   // test.equivalent( got, [ [] ] );
//   // test.true( _.arrayIs( got ) );
//   // test.true( _.unrollIs( got ) );
//   // test.true( _.unrollIs( got[ 0 ] ) );
//
//   test.case = 'srcs - several arguments';
//   var got = _.unrollsFrom( 1, [], null, [ 1, { a : 2 } ] );
//   var expected = [ [ undefined ], [], [], [ 1, { a : 2 } ] ];
//   test.equivalent( got, expected );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( _.unrollIs( got[ 3 ] ) );
//   test.true( got !== expected );
//
//   test.case = 'srcs - empty unroll';
//   var src = _.unroll.make( 0 );
//   var got = _.unrollsFrom( src );
//   test.identical( got, [ [] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( got !== [ [] ] );
//
//   test.case = 'srcs - filled unroll';
//   var src = _.unroll.make( [ 1, 'str', 3 ] );
//   var got = _.unrollsFrom( src );
//   test.identical( got, [ [ 1, 'str', 3 ] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( got !== [ [ 1, 'str', 3 ] ] );
//
//   test.case = 'srcs - several arguments with unroll';
//   var src = _.unroll.make( [ 1, 'str', 3 ] );
//   var got = _.unrollsFrom( 1, [], src );
//   var expected = [ [ undefined ], [], [ 1, 'str', 3 ] ];
//   test.identical( got, expected );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( _.unrollIs( got[ 2 ] ) );
//   test.true( got !== expected );
//
//   test.case = 'srcs - empty F32x buffer';
//   var src = new F32x();
//   var got = _.unrollsFrom( src );
//   test.equivalent( got, [ [] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( [ src ] !== got );
//
//   test.case = 'srcs - filled U8x buffer';
//   var src = new U8x( [ 1, 2, 3 ] );
//   var got = _.unrollsFrom( src );
//   test.equivalent( got, [ [ 1, 2, 3 ] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( [ src ] !== got );
//
//   test.case = 'srcs - several arguments with I16x buffer';
//   var src = new I16x( [ 1, 2, 3 ] );
//   var got = _.unrollsFrom( [], 1, src );
//   test.equivalent( got, [ [], [ undefined ], [ 1, 2, 3 ] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( _.unrollIs( got[ 1 ] ) );
//   test.true( _.unrollIs( got[ 2 ] ) );
//   test.true( [ src ] !== got );
//
//   test.case = 'srcs - empty arguments array';
//   var src = _.argumentsArray.make( [] );
//   var got = _.unrollsFrom( src );
//   test.equivalent( got, [ [] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( [ src ] !== got );
//
//   test.case = 'srcs - filled arguments array';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.unrollsFrom( src );
//   test.equivalent( got, [ [ 1, 2, 3 ] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( [ src ] !== got );
//
//   test.case = 'srcs - several arguments with arguments array';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.unrollsFrom( [], 1, src );
//   test.equivalent( got, [ [], [ undefined ], [ 1, 2, 3 ] ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( _.unrollIs( got[ 0 ] ) );
//   test.true( _.unrollIs( got[ 1 ] ) );
//   test.true( _.unrollIs( got[ 2 ] ) );
//   test.true( [ src ] !== got );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.unrollsFrom() );
//
//   test.case = 'argument is not array, not null';
//   test.shouldThrowErrorSync( () => _.unrollsFrom( {} ) );
//   test.shouldThrowErrorSync( () => _.unrollsFrom( 'wrong' ) );
//   test.shouldThrowErrorSync( () => _.unrollsFrom( 2, {} ) );
//   test.shouldThrowErrorSync( () => _.unrollsFrom( [ '1' ], [ 1, 'str' ], 'abc' ) );
//   test.shouldThrowErrorSync( () => _.unrollsFrom( undefined ) );
//
// }

//

function unrollsFromLongDescriptor( test )
{
  let times = 4;

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let name = namespace.TypeName;
    let long = _.withLong[ name ];

    test.open( `long - ${ name }` );
    testRun( long );
    test.close( `long - ${ name }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function testRun( long )
  {
    test.case = 'srcs - null';
    var got = long.unrollsFrom( null );
    test.equivalent( got, [ [] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );

    // test.case = 'srcs - undefined';
    // var got = long.unrollsFrom( undefined );
    // test.equivalent( got, [ [] ] );
    // test.true( _.arrayIs( got ) );
    // test.true( _.unrollIs( got ) );
    // test.true( _.unrollIs( got[ 0 ] ) );

    test.case = 'srcs - several arguments';
    var got = long.unrollsFrom( 1, [], null, [ 1, { a : 2 } ] );
    var expected = [ [ undefined ], [], [], [ 1, { a : 2 } ] ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( _.unrollIs( got[ 3 ] ) );
    test.true( got !== expected );

    test.case = 'srcs - empty unroll';
    var src = _.unroll.make( 0 );
    var got = long.unrollsFrom( src );
    test.identical( got, [ [] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( got !== [ [] ] );

    test.case = 'srcs - filled unroll';
    var src = _.unroll.make( [ 1, 'str', 3 ] );
    var got = long.unrollsFrom( src );
    test.identical( got, [ [ 1, 'str', 3 ] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( got !== [ [ 1, 'str', 3 ] ] );

    test.case = 'srcs - several arguments with unroll';
    var src = _.unroll.make( [ 1, 'str', 3 ] );
    var got = long.unrollsFrom( 1, [], src );
    var expected = [ [ undefined ], [], [ 1, 'str', 3 ] ];
    test.identical( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( _.unrollIs( got[ 2 ] ) );
    test.true( got !== expected );

    test.case = 'srcs - empty F32x buffer';
    var src = new F32x();
    var got = long.unrollsFrom( src );
    test.equivalent( got, [ [] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( [ src ] !== got );

    test.case = 'srcs - filled U8x buffer';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = long.unrollsFrom( src );
    test.equivalent( got, [ [ 1, 2, 3 ] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( [ src ] !== got );

    test.case = 'srcs - several arguments with I16x buffer';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = long.unrollsFrom( [], 1, src );
    test.equivalent( got, [ [], [ undefined ], [ 1, 2, 3 ] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( _.unrollIs( got[ 1 ] ) );
    test.true( _.unrollIs( got[ 2 ] ) );
    test.true( [ src ] !== got );

    test.case = 'srcs - empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = long.unrollsFrom( src );
    test.equivalent( got, [ [] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( [ src ] !== got );

    test.case = 'srcs - filled arguments array';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.unrollsFrom( src );
    test.equivalent( got, [ [ 1, 2, 3 ] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( [ src ] !== got );

    test.case = 'srcs - several arguments with arguments array';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.unrollsFrom( [], 1, src );
    test.equivalent( got, [ [], [ undefined ], [ 1, 2, 3 ] ] );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( _.unrollIs( got[ 0 ] ) );
    test.true( _.unrollIs( got[ 1 ] ) );
    test.true( _.unrollIs( got[ 2 ] ) );
    test.true( [ src ] !== got );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.unrollsFrom() );

      test.case = 'argument is not array, not null';
      test.shouldThrowErrorSync( () => long.unrollsFrom( {} ) );
      test.shouldThrowErrorSync( () => long.unrollsFrom( 'wrong' ) );
      test.shouldThrowErrorSync( () => long.unrollsFrom( undefined ) );
      test.shouldThrowErrorSync( () => long.unrollsFrom( 2, {} ) );
      test.shouldThrowErrorSync( () => long.unrollsFrom( [ '1' ], [ 1, 'str' ], 'abc' ) );
    }
  }
}

//

// function fromMaybe( test )
// {
//   test.case = 'src - undefined';
//   var got = _.unroll.fromMaybe( undefined );
//   test.identical( got, undefined );
//
//   test.case = 'src - empty string';
//   var got = _.unroll.fromMaybe( '' );
//   test.identical( got, '' );
//   test.true( _.primitive.is( got ) );
//
//   test.case = 'src - string';
//   var got = _.unroll.fromMaybe( 'str' );
//   test.identical( got, 'str' );
//   test.true( _.primitive.is( got ) );
//
//   test.case = 'src - booleant - true';
//   var got = _.unroll.fromMaybe( true );
//   test.identical( got, true );
//   test.true( _.primitive.is( got ) );
//
//   test.case = 'src - booleant - false';
//   var got = _.unroll.fromMaybe( false );
//   test.identical( got, false );
//   test.true( _.primitive.is( got ) );
//
//   test.case = 'src - empty map';
//   var got = _.unroll.fromMaybe( {} );
//   test.identical( got, {} );
//   test.true( _.mapIs( got ) );
//
//   test.case = 'src - filled map';
//   var got = _.unroll.fromMaybe( { a : 0, b : 'str' } );
//   test.identical( got, { a : 0, b : 'str' } );
//   test.true( _.mapIs( got ) );
//
//   test.case = 'src - empty pure map';
//   var got = _.unroll.fromMaybe( Object.create( null ) );
//   test.identical( got, {} );
//   test.true( _.mapIs( got ) );
//
//   test.case = 'src - filled map';
//   var src = Object.create( null );
//   src.a = 0;
//   src.b = 'str'
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, { a : 0, b : 'str' } );
//   test.true( _.mapIs( got ) );
//
//   test.case = 'src - empty HashMap';
//   var src = new HashMap();
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, new HashMap() );
//   test.true( _.hashMap.is( got ) );
//
//   test.case = 'src - filled HashMap';
//   var src = new HashMap( [ [ 1, 2 ], [ 'a', 'b' ] ] );
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, new HashMap( [ [ 1, 2 ], [ 'a', 'b' ] ] ) );
//   test.true( _.hashMap.is( got ) );
//
//   test.case = 'src - empty Set';
//   var src = new Set( [] );
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, new Set() );
//   test.true( _.set.is( got ) );
//
//   test.case = 'src - filled Set';
//   var src = new Set( [ 1, 'abc' ] );
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, new Set( [ 1, 'abc' ] ) );
//   test.true( _.set.is( got ) );
//
//   test.case = 'src - instance of constructor';
//   function Constr(){ this.x = 1; return this };
//   var src = new Constr();
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, src );
//   test.true( _.object.isBasic( got ) );
//
//   test.case = 'src - null';
//   var got = _.unroll.fromMaybe( null );
//   test.equivalent( got, [] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( [] !== got );
//
//   test.case = 'src - unroll';
//   var src = _.unroll.make( 0 );
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, [] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( got !== [] );
//
//   test.case = 'src - filled unroll';
//   var src = _.unroll.make( [ 1, 'str', 3 ] );
//   var got = _.unroll.fromMaybe( src );
//   test.identical( got, [ 1, 'str', 3 ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( got !== [ 1, 'str', 3 ] );
//
//   test.case = 'src - empty array';
//   var src = [];
//   var got = _.unroll.fromMaybe( src );
//   test.equivalent( got, src );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( src !== got );
//
//   test.case = 'src - filled array';
//   var src = [ 1, 2, 'str' ];
//   var got = _.unroll.fromMaybe( src );
//   test.equivalent( got, src );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( src !== got );
//
//   test.case = 'src - instance of Array constructor';
//   var got = _.unroll.fromMaybe( 3 );
//   var expected = new Array( 3 );
//   test.equivalent( got, expected );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( expected !== got );
//
//   test.case = 'src - empty F32x buffer';
//   var src = new F32x();
//   var got = _.unroll.fromMaybe( src );
//   test.equivalent( got, [] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( src !== got );
//
//   test.case = 'src - filled U8x buffer';
//   var src = new U8x( [ 1, 2, 3 ] );
//   var got = _.unroll.fromMaybe( src );
//   test.equivalent( got, [ 1, 2, 3 ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( src !== got );
//
//   test.case = 'src - empty arguments array';
//   var src = _.argumentsArray.make( [] );
//   var got = _.unroll.fromMaybe( src );
//   test.equivalent( got, [] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( src !== got );
//
//   test.case = 'src - filled arguments array';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.unroll.fromMaybe( src );
//   test.equivalent( got, [ 1, 2, 3 ] );
//   test.true( _.arrayIs( got ) );
//   test.true( _.unrollIs( got ) );
//   test.true( src !== got );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.unroll.fromMaybe() );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.unroll.fromMaybe( 1, 3 ) );
//   test.shouldThrowErrorSync( () => _.unroll.fromMaybe( [], 3 ) );
//   test.shouldThrowErrorSync( () => _.unroll.fromMaybe( [], [] ) );
// }
//
// //
//
// function fromMaybeLongDescriptor( test )
// {
//   let times = 4;
//
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* - */
//
//   function testRun( long )
//   {
//     test.case = 'src - undefined';
//     var got = long.unroll.fromMaybe( undefined );
//     test.identical( got, undefined );
//
//     test.case = 'src - empty string';
//     var got = long.unroll.fromMaybe( '' );
//     test.identical( got, '' );
//     test.true( _.primitive.is( got ) );
//
//     test.case = 'src - string';
//     var got = long.unroll.fromMaybe( 'str' );
//     test.identical( got, 'str' );
//     test.true( _.primitive.is( got ) );
//
//     test.case = 'src - booleant - true';
//     var got = long.unroll.fromMaybe( true );
//     test.identical( got, true );
//     test.true( _.primitive.is( got ) );
//
//     test.case = 'src - booleant - false';
//     var got = long.unroll.fromMaybe( false );
//     test.identical( got, false );
//     test.true( _.primitive.is( got ) );
//
//     test.case = 'src - empty map';
//     var got = long.unroll.fromMaybe( {} );
//     test.identical( got, {} );
//     test.true( _.mapIs( got ) );
//
//     test.case = 'src - filled map';
//     var got = long.unroll.fromMaybe( { a : 0, b : 'str' } );
//     test.identical( got, { a : 0, b : 'str' } );
//     test.true( _.mapIs( got ) );
//
//     test.case = 'src - empty pure map';
//     var got = long.unroll.fromMaybe( Object.create( null ) );
//     test.identical( got, {} );
//     test.true( _.mapIs( got ) );
//
//     test.case = 'src - filled map';
//     var src = Object.create( null );
//     src.a = 0;
//     src.b = 'str'
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, { a : 0, b : 'str' } );
//     test.true( _.mapIs( got ) );
//
//     test.case = 'src - empty HashMap';
//     var src = new HashMap();
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, new HashMap() );
//     test.true( _.hashMap.is( got ) );
//
//     test.case = 'src - filled HashMap';
//     var src = new HashMap( [ [ 1, 2 ], [ 'a', 'b' ] ] );
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, new HashMap( [ [ 1, 2 ], [ 'a', 'b' ] ] ) );
//     test.true( _.hashMap.is( got ) );
//
//     test.case = 'src - empty Set';
//     var src = new Set( [] );
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, new Set() );
//     test.true( _.set.is( got ) );
//
//     test.case = 'src - filled Set';
//     var src = new Set( [ 1, 'abc' ] );
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, new Set( [ 1, 'abc' ] ) );
//     test.true( _.set.is( got ) );
//
//     test.case = 'src - instance of constructor';
//     function Constr1(){ this.x = 1; return this };
//     var src = new Constr1();
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, src );
//     test.true( _.object.isBasic( got ) );
//
//     test.case = 'src - null';
//     var got = long.unroll.fromMaybe( null );
//     test.equivalent( got, [] );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( [] !== got );
//
//     test.case = 'src - unroll';
//     var src = _.unroll.make( 0 );
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, [] );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== [] );
//
//     test.case = 'src - filled unroll';
//     var src = _.unroll.make( [ 1, 'str', 3 ] );
//     var got = long.unroll.fromMaybe( src );
//     test.identical( got, [ 1, 'str', 3 ] );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== [ 1, 'str', 3 ] );
//
//     test.case = 'src - empty array';
//     var src = [];
//     var got = long.unroll.fromMaybe( src );
//     test.equivalent( got, src );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( src !== got );
//
//     test.case = 'src - filled array';
//     var src = [ 1, 2, 'str' ];
//     var got = long.unroll.fromMaybe( src );
//     test.equivalent( got, src );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( src !== got );
//
//     test.case = 'src - instance of Array constructor';
//     var got = long.unroll.fromMaybe( 3 );
//     var expected = new Array( 3 );
//     test.equivalent( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( expected !== got );
//
//     test.case = 'src - empty F32x buffer';
//     var src = new F32x();
//     var got = long.unroll.fromMaybe( src );
//     test.equivalent( got, [] );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( src !== got );
//
//     test.case = 'src - filled U8x buffer';
//     var src = new U8x( [ 1, 2, 3 ] );
//     var got = long.unroll.fromMaybe( src );
//     test.equivalent( got, [ 1, 2, 3 ] );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( src !== got );
//
//     test.case = 'src - empty arguments array';
//     var src = _.argumentsArray.make( [] );
//     var got = long.unroll.fromMaybe( src );
//     test.equivalent( got, [] );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( src !== got );
//
//     test.case = 'src - filled arguments array';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.unroll.fromMaybe( src );
//     test.equivalent( got, [ 1, 2, 3 ] );
//     test.true( _.arrayIs( got ) );
//     test.true( _.unrollIs( got ) );
//     test.true( src !== got );
//
//     /* - */
//
//     if( !Config.debug )
//     {
//       test.case = 'without arguments';
//       test.shouldThrowErrorSync( () => long.unroll.fromMaybe() );
//
//       test.case = 'extra arguments';
//       test.shouldThrowErrorSync( () => long.unroll.fromMaybe( 1, 3 ) );
//       test.shouldThrowErrorSync( () => long.unroll.fromMaybe( [], 3 ) );
//       test.shouldThrowErrorSync( () => long.unroll.fromMaybe( [], [] ) );
//     }
//   }
// }

//

function unrollNormalize( test )
{
  test.case = 'dst is array';
  var got = _.unroll.normalize( [] );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  var got = _.unroll.normalize( [ 1, 'str' ] );
  test.identical( got, [ 1, 'str' ] );
  test.true( _.arrayIs( got ) );

  test.case = 'dst is unroll';
  var got = _.unroll.normalize( _.unroll.make( [] ) );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  var got = _.unroll.normalize( _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 'str' ] );
  test.true( _.arrayIs( got ) );

  test.case = 'dst is unroll from array';
  var src = new Array( 0 );
  var got = _.unroll.normalize( _.unroll.from( src ) );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  var src = new Array( [] );
  var got = _.unroll.normalize( _.unroll.from( src ) );
  test.identical( got, [ [] ] );
  test.true( _.arrayIs( got ) );

  var src = new Array( [ 1, 2, 'str' ] );
  var got = _.unroll.normalize( _.unroll.from( src ) );
  test.identical( got, [ [ 1, 2, 'str' ] ] );
  test.true( _.arrayIs( got ) );

  test.case = 'dst is unroll from array';
  var src = new F32x( [] );
  var got = _.unroll.normalize( _.unroll.from( src ) );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  var src = new F32x( [ 1, 2, 3 ] );
  var got = _.unroll.normalize( _.unroll.from( src ) );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );

  test.case = 'dst is complex unroll';
  var got = _.unroll.normalize( _.unroll.from( [ 1, _.unroll.from ( [ 2, _.unroll.from( [ 'str' ] ) ] ) ] ) );
  test.identical( got, [ 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );

  test.case = 'mixed types';
  var a = _.unroll.make( [ 'a', 'b' ] );
  var b = _.unroll.from( [ 1, 2 ] );
  var got = _.unroll.normalize( [ 0, null, a, b, undefined ] );
  test.identical( got, [ 0, null, 'a', 'b', 1, 2, undefined ] );
  test.true( _.arrayIs( got ) );

  var a = _.unroll.make( [ 'a', 'b' ] );
  var b = _.unroll.from( [ 1, 2 ] );
  var got = _.unroll.normalize( [ 0, [ null, a ], _.unroll.from( [ b, undefined ] ) ] );
  test.identical( got, [ 0, [ null, 'a', 'b' ], 1, 2, undefined ] );
  test.true( _.arrayIs( got ) );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'dst is empty';
  test.shouldThrowErrorSync( function()
  {
    _.unroll.normalize();
  });

  test.case = 'two arguments';
  test.shouldThrowErrorSync( function()
  {
    _.unroll.normalize( [], [] );
  });

  test.case = 'dst is not array';
  test.shouldThrowErrorSync( function()
  {
    _.unroll.normalize( null );
  });

  test.shouldThrowErrorSync( function()
  {
    _.unroll.normalize( 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.unroll.normalize( 'str' );
  });
}

//

// function unrollSelect( test )
// {
//   /* constructors */
//
//   let array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var argumentsArray = ( src ) => _.argumentsArray.make( src );
//   function bufferTyped( buf )
//   {
//     let name = buf.name;
//     return ({ [ name ] : function( src ){ return new buf( src ) } })[ name ];
//   };
//
//   /* lists */
//
//   var listTyped =
//   [
//     I8x,
//     // U8x,
//     // U8xClamped,
//     // I16x,
//     U16x,
//     // I32x,
//     // U32x,
//     F32x,
//     F64x,
//   ];
//   var list =
//   [
//     array,
//     unroll,
//     argumentsArray,
//   ];
//   for( let i = 0; i < listTyped.length; i++ )
//   list.push( bufferTyped( listTyped[ i ] ) );
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     run( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function run( make )
//   {
//     test.case = 'without arguments';
//     var got = _.unrollSelect();
//     var expected = _.unroll.make( [] );
//     test.identical( got, expected );
//
//     test.case = 'only dst';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst );
//     var expected = _.unroll.make( [ 1, 2, 3, 4, 5 ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'range > dst.length, not a val';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst, [ 0, dst.length + 2 ] );
//     var expected = _.unroll.make( [ 1, 2, 3, 4, 5, undefined, undefined ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'range > dst.length, val = number';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst, [ 0, dst.length + 2 ], 0 );
//     var expected = _.unroll.make( [ 1, 2, 3, 4, 5, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'range > dst.length, val = number';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst, [ dst.length - 1, dst.length * 2 ], 0 );
//     var expected = _.unroll.make( [ 5, 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'range < dst.length';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst, [ 0, 3 ] );
//     var expected = _.unroll.make( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'range < dst.length, val = number';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst, [ 0, 3 ], 0 );
//     var expected = _.unroll.make( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'f < 0, not a val';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     got = _.unrollSelect( dst, [ -1, 3 ] );
//     expected = _.unroll.make( [ 1, 2, 3, 4 ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'l < 0, not a val';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst, [ 0, -1 ] );
//     var expected = _.unroll.make( [] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//
//     test.case = 'f < 0, val = number';
//     var dst = make( [ 1, 2, 3, 4, 5 ] );
//     var got = _.unrollSelect( dst, [ -1, 3 ], 0 );
//     var expected = _.unroll.make( [ 1, 2, 3, 4 ] );
//     test.identical( got, expected );
//     test.true( _.unrollIs( got ) );
//     test.true( got !== dst );
//   }
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.unrollSelect( [ 1 ], [ 1, 4 ], '5', 1 ) );
//
//   test.case = 'array is not long';
//   test.shouldThrowErrorSync( () => _.unrollSelect( 1, [ 0, 1 ] ) );
//   test.shouldThrowErrorSync( () => _.unrollSelect( new ArrayBuffer( 4 ), [ 0, 5 ] ) );
//
//   test.case = 'not a range';
//   test.shouldThrowErrorSync( () => _.unrollSelect( [ 1 ], [ 1 ] ) );
//   test.shouldThrowErrorSync( () => _.unrollSelect( [ 1 ], 'str' ) );
// }

//

function unrollPrepend( test )
{
  test.open( 'one argument' );

  test.case = 'dst is null';
  var got = _.unrollPrepend( null );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is unroll';
  var got = _.unrollPrepend( _.unroll.make( [ 1, 2, 'str' ] ) );
  test.identical( got, [ 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll';
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is array';
  var got = _.unrollPrepend( [ 1, 2, 'str' ] );
  test.identical( got, [ 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.close( 'one argument' );

  /* - */

  test.open( 'two arguments' );

  test.case = 'dst is null, second arg is null';
  var got = _.unrollPrepend( null, null );
  test.identical( got, [ null ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is unroll';
  var got = _.unrollPrepend( null, _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is complex unroll';
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( null, src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is complex unroll';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollPrepend( null, _.unroll.from( [ 1, 2, a1, a2, 10 ] ) );
  var expected = [ 1, 2, 3, 4, 5, 6, [ 7, 8, 9 ], 10 ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is array';
  var got = _.unrollPrepend( null, [ 1, 2, 'str' ] );
  test.identical( got, [ [ 1, 2, 'str' ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg makes from F32x';
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var got = _.unrollPrepend( null, src );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg makes from argumentsArray';
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var got = _.unrollPrepend( null, src );
  test.identical( got, [ 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );
  test.false( _.unrollIs( got ) );

  /* */

  test.case = 'dst is array, second arg is null';
  var got = _.unrollPrepend( [ 1 ], null );
  test.identical( got, [ null, 1 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg is unroll';
  var got = _.unrollPrepend( [ 1 ], _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 'str', 1 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg is complex unroll';
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( [ 'str', 3 ], src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', 'str', 3 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg is array';
  var got = _.unrollPrepend( [ 'str', 2 ], [ 1, 2, 'str' ] );
  test.identical( got, [ [ 1, 2, 'str' ], 'str', 2 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg makes from F32x';
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var got = _.unrollPrepend( [ 'str', 0 ], src );
  test.identical( got, [ 1, 2, 3, 'str', 0 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg makes from argumentsArray';
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var got = _.unrollPrepend( [ 'str', 0 ], src );
  test.identical( got, [ 1, 2, 'str', 'str', 0 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  /* */

  test.case = 'dst is unroll, second arg is null';
  var dst = _.unroll.make( [ 1 ] );
  var got = _.unrollPrepend( dst, null );
  test.identical( got, [ null, 1 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg is unroll';
  var dst = _.unroll.make( [ 1 ] );
  var got = _.unrollPrepend( dst, _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 'str', 1 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg is complex unroll';
  var dst = _.unroll.make( [ 1 ] );
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( dst, src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', 1 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg is array';
  var dst = _.unroll.make( [ 1 ] );
  var got = _.unrollPrepend( dst, [ 1, 2, 'str' ] );
  test.identical( got, [ [ 1, 2, 'str' ], 1 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg makes from F32x';
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var dst = _.unroll.make( [ 'str', 0 ] );
  var got = _.unrollPrepend( dst, src );
  test.identical( got, [ 1, 2, 3, 'str', 0 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg makes from argumentsArray';
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var dst = _.unroll.make( [ 'str', 0 ] );
  var got = _.unrollPrepend( dst, src );
  test.identical( got, [ 1, 2, 'str', 'str', 0 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  /* */

  test.case = 'dst is complex unroll, second arg is null';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( dst, null );
  test.identical( got, [ null, 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg is unroll';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( dst, _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 'str', 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg is complex unroll';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( dst, src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg is array';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollPrepend( dst, [ 1, 2, 'str' ] );
  test.identical( got, [ [ 1, 2, 'str' ], 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg makes from F32x';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var got = _.unrollPrepend( dst, src );
  test.identical( got, [ 1, 2, 3, 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg makes from argumentsArray';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var got = _.unrollPrepend( dst, src );
  test.identical( got, [ 1, 2, 'str', 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.close( 'two arguments' );

  /* - */

  test.open( 'three arguments or more' );

  test.case = 'dst is null, complex unrolls';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollPrepend( null, [ 1, 2, a1 ], [ a2, 10 ] );
  var expected = [ [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, manually unrolled src';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollPrepend( null, 1, 2, a1, a2, 10  );
  var expected = [ 1, 2, 3, 4, 5, 6, [ 7, 8, 9 ], 10 ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, next args have unroll from argumentsArray';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( _.argumentsArray.make( [ 3, 4, _.unroll.make( [ 5, 6 ] ) ] ) );
  var got = _.unrollPrepend( null, [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, next args have unroll from F32x';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( new F32x( [ 3, 4 ] ) );
  var got = _.unrollPrepend( null, [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is unroll, complex unrolls';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollPrepend( _.unroll.from( [] ), [ 1, 2 ], a1, [ a2, 10 ] );
  var expected = [ [ 1, 2 ], 3, 4, 5, 6, [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, next args have unroll from argumentsArray';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( _.argumentsArray.make( [ 3, 4, _.unroll.make( [ 5, 6 ] ) ] ) );
  var got = _.unrollPrepend( _.unroll.make( [ 0 ] ), [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ], 0 ];
  test.identical( got, expected );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, next args have unroll from F32x';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( new F32x( [ 3, 4 ] ) );
  var got = _.unrollPrepend( _.unroll.make( [ 0 ] ), [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4 ], [ [ 7, 8, 9 ], 10 ], 0 ];
  test.identical( got, expected );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is array, complex unrolls';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollPrepend( [], [ 1, 2 ], a1, [ a2, 10 ] );
  var expected = [ [ 1, 2 ], 3, 4, 5, 6, [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, next args have unroll from argumentsArray';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( _.argumentsArray.make( [ 3, 4, _.unroll.make( [ 5, 6 ] ) ] ) );
  var got = _.unrollPrepend( [ 0 ], [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ], 0 ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, next args have unroll from F32x';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( new F32x( [ 3, 4 ] ) );
  var got = _.unrollPrepend( [ 0 ], [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4 ], [ [ 7, 8, 9 ], 10 ], 0 ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.close( 'three arguments or more' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.unrollPrepend();
  });

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.unrollPrepend( 1, 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.unrollPrepend( 'str', 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.unrollPrepend( undefined, 1 );
  });
}

//

function unrollAppend( test )
{
  test.open( 'one argument' );

  test.case = 'dst is null';
  var got = _.unrollAppend( null );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is unroll';
  var got = _.unrollAppend( _.unroll.make( [ 1, 2, 'str' ] ) );
  test.identical( got, [ 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll';
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is array';
  var got = _.unrollAppend( [ 1, 2, 'str' ] );
  test.identical( got, [ 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.close( 'one argument' );

  /* - */

  test.open( 'two arguments' );

  test.case = 'dst is null, second arg is null';
  var got = _.unrollAppend( null, null );
  test.identical( got, [ null ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is unroll';
  var got = _.unrollAppend( null, _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is complex unroll';
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( null, src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is complex unroll';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollAppend( null, _.unroll.from( [ 1, 2, a1, a2, 10 ] ) );
  var expected = [ 1, 2, 3, 4, 5, 6, [ 7, 8, 9 ], 10 ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg is array';
  var got = _.unrollAppend( null, [ 1, 2, 'str' ] );
  test.identical( got, [ [ 1, 2, 'str' ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, second arg makes from F32x';
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var got = _.unrollAppend( null, src );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );
  test.true( src !== got );

  test.case = 'dst is null, second arg makes from argumentsArray';
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var got = _.unrollAppend( null, src );
  test.identical( got, [ 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );
  test.true( src !== got );

  /* */

  test.case = 'dst is array, second arg is null';
  var got = _.unrollAppend( [ 1 ], null );
  test.identical( got, [ 1, null ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg is unroll';
  var got = _.unrollAppend( [ 1 ], _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 1, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg is complex unroll';
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( [ 'str', 3 ], src );
  test.identical( got, [ 'str', 3, 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg is array';
  var got = _.unrollAppend( [ 'str', 2 ], [ 1, 2, 'str' ] );
  test.identical( got, [ 'str', 2, [ 1, 2, 'str' ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg makes from F32x';
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var got = _.unrollAppend( [ 'str', 0 ], src );
  test.identical( got, [ 'str', 0, 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, second arg makes from argumentsArray';
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var got = _.unrollAppend( [ 'str', 0 ], src );
  test.identical( got, [ 'str', 0, 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  /* */

  test.case = 'dst is unroll, second arg is null';
  var dst = _.unroll.make( [ 1 ] );
  var got = _.unrollAppend( dst, null );
  test.identical( got, [ 1, null ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg is unroll';
  var dst = _.unroll.make( [ 1 ] );
  var got = _.unrollAppend( dst, _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 1, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg is complex unroll';
  var dst = _.unroll.make( [ 1 ] );
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( dst, src );
  test.identical( got, [ 1, 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg is array';
  var dst = _.unroll.make( [ 1 ] );
  var got = _.unrollAppend( dst, [ 1, 2, 'str' ] );
  test.identical( got, [ 1, [ 1, 2, 'str' ] ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg makes from F32x';
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var dst = _.unroll.make( [ 'str', 0 ] );
  var got = _.unrollAppend( dst, src );
  test.identical( got, [ 'str', 0, 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, second arg makes from argumentsArray';
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var dst = _.unroll.make( [ 'str', 0 ] );
  var got = _.unrollAppend( dst, src );
  test.identical( got, [ 'str', 0, 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  /* */

  test.case = 'dst is complex unroll, second arg is null';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( dst, null );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', null ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg is unroll';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( dst, _.unroll.make( [ 1, 'str' ] ) );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', 1, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg is complex unroll';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var src = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( dst, src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', 1, 2, 1, [], 'str', 'str2' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg is array';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var got = _.unrollAppend( dst, [ 1, 2, 'str' ] );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', [ 1, 2, 'str' ] ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg makes from F32x';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var src = _.unroll.make( new F32x( [ 1, 2, 3 ] ) );
  var got = _.unrollAppend( dst, src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', 1, 2, 3 ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is complex unroll, second arg makes from argumentsArray';
  var dst = _.unroll.from( [ 1, 2, _.unroll.make( [ 1, [] ] ), _.unroll.from( [ 'str', _.unroll.make( [ 'str2' ] ) ] ) ] );
  var src = _.unroll.make( _.argumentsArray.make( [ 1, 2, 'str' ] ) );
  var got = _.unrollAppend( dst, src );
  test.identical( got, [ 1, 2, 1, [], 'str', 'str2', 1, 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.close( 'two arguments' );

  /* - */

  test.open( 'three arguments or more' );

  test.case = 'dst is null, complex unrolls';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollAppend( null, [ 1, 2, a1 ], [ a2, 10 ] );
  var expected = [ [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, manually unrolled src';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollAppend( null, 1, 2, a1, a2, 10  );
  var expected = [ 1, 2, 3, 4, 5, 6, [ 7, 8, 9 ], 10 ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, next args have unroll from argumentsArray';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( _.argumentsArray.make( [ 3, 4, _.unroll.make( [ 5, 6 ] ) ] ) );
  var got = _.unrollAppend( null, [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is null, next args have unroll from F32x';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( new F32x( [ 3, 4 ] ) );
  var got = _.unrollAppend( null, [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ [ 1, 2, 3, 4 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is unroll, complex unrolls';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollAppend( _.unroll.from( [] ), [ 1, 2 ], a1, [ a2, 10 ] );
  var expected = [ [ 1, 2 ], 3, 4, 5, 6, [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, next args have unroll from argumentsArray';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( _.argumentsArray.make( [ 3, 4, _.unroll.make( [ 5, 6 ] ) ] ) );
  var got = _.unrollAppend( _.unroll.make( [ 0 ] ), [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ 0, [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is unroll, next args have unroll from F32x';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( new F32x( [ 3, 4 ] ) );
  var got = _.unrollAppend( _.unroll.make( [ 0 ] ), [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ 0, [ 1, 2, 3, 4 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.true( _.unrollIs( got ) );

  test.case = 'dst is array, complex unrolls';
  var a1 = _.unroll.from( [ 3, 4, _.unroll.from( [ 5, 6 ] ) ] );
  var a2 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var got = _.unrollAppend( [], [ 1, 2 ], a1, [ a2, 10 ] );
  var expected = [ [ 1, 2 ], 3, 4, 5, 6, [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, next args have unroll from argumentsArray';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( _.argumentsArray.make( [ 3, 4, _.unroll.make( [ 5, 6 ] ) ] ) );
  var got = _.unrollAppend( [ 0 ], [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ 0, [ 1, 2, 3, 4, 5, 6 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is array, next args have unroll from F32x';
  var a1 = [ 7, _.unroll.from( [ 8, 9 ] ) ];
  var a2 = _.unroll.from( new F32x( [ 3, 4 ] ) );
  var got = _.unrollAppend( [ 0 ], [ 1, 2, a2 ], [ a1, 10 ] );
  var expected = [ 0, [ 1, 2, 3, 4 ], [ [ 7, 8, 9 ], 10 ] ];
  test.identical( got, expected );
  test.false( _.unrollIs( got ) );

  test.close( 'three arguments or more' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.unrollAppend();
  });

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.unrollAppend( 1, 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.unrollAppend( 'str', 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.unrollAppend( undefined, 1 );
  });
}

//

function unrollRemove( test )
{
  test.case = 'dst is null'
  var got = _.unrollRemove( null, 0 );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( null, 'str' );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( null, null );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( null, [ 1, 2, 'str' ] );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( null, _.unroll.make( [ 1 ] ) );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'dst is unroll from null'
  var got = _.unrollRemove( _.unroll.make( null ), 'str' );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var got = _.unrollRemove( _.unroll.make( null ), _.unroll.make( [ 1 ] ) );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var got = _.unrollRemove( _.unroll.make( null ), _.unroll.make( null ) );
  test.equivalent( got, [] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  /* - */

  test.open( 'dstArray is array' );

  test.case = 'array remove element';
  var got = _.unrollRemove( [ 1, 1, 2, 'str' ], 1 );
  test.equivalent( got, [ 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 1, 2, 'str' ], 'str' );
  test.equivalent( got, [ 1, 1, 2 ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 1, 2, 'str', {} ], 0 );
  test.equivalent( got, [ 1, 1, 2, 'str', {} ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'array remove array or object';
  var got = _.unrollRemove( [ 1, 1, 2, 'str', [ 0 ] ], [ 0 ] );
  test.equivalent( got, [ 1, 1, 2, 'str', [ 0 ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 1, 2, 'str', { a : 1, b : 'str' } ], { a : 1, b : 'str' } );
  test.equivalent( got, [ 1, 1, 2, 'str', { a : 1, b : 'str' } ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'array remove elements';
  var got = _.unrollRemove( [ 1, 1, 2, 'str', [ 1 ] ], 1, [ 1 ] );
  test.equivalent( got, [ 2, 'str', [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 1, 2, 'str', { a : 2 }, 'str' ], 0, { a : 2 }, 4, 'str' );
  test.equivalent( got, [ 1, 1, 2, { a : 2 } ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 1, 2, 'str', { a : 2 } ], null, undefined, 4, [] );
  test.equivalent( got, [ 1, 1, 2, 'str', { a : 2 } ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'array remove elements included array or object';
  var got = _.unrollRemove( [ 1, 1, 2, 'str', [ 0 ] ], 1, [ 0 ] );
  test.equivalent( got, [ 2, 'str', [ 0 ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 1, 2, 'str', { a : 1, b : 'str' } ], 2, 'str', { a : 1, b : 'str' } );
  test.equivalent( got, [ 1, 1, { a : 1, b : 'str' } ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.case = 'array remove unroll';
  var got = _.unrollRemove( [ 1, 1, 2, 3, 'str', 3 ], _.unroll.from( [ 1, 3 ] ) );
  test.equivalent( got, [ 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 2, 1, 3, 'str', [ 1 ] ], _.unroll.from( [ 1, 3, 'str', [ 1 ] ] ) );
  test.equivalent( got, [ 2, [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var got = _.unrollRemove( [ 1, 2, 3, 'str', [ 1 ] ], _.unroll.from( [ 0, 'a', [ 2 ] ] ) );
  test.equivalent( got, [ 1, 2, 3, 'str', [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  var ins =  _.unroll.from( [ 1, _.unroll.make( [ 2, 3, _.unroll.make( [ 'str', [ 1 ] ] ) ] ) ] );
  var got = _.unrollRemove( [ 1, 2, 3, 'str', [ 1 ] ], ins );
  test.equivalent( got, [ [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.false( _.unrollIs( got ) );

  test.close( 'dstArray is array' );

  /* - */

  test.open( 'dstArray is unroll' );

  test.case = 'unroll remove element';
  var dst = _.unroll.make( [ 1, 1, 2, 'str', [ 1 ] ] );
  var got = _.unrollRemove( dst, 1);
  test.equivalent( got, [ 2, 'str', [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var dst = _.unroll.make( [ 1, 1, 2, 'str', { a : 2 }, 'str' ] );
  var got = _.unrollRemove( dst, 'str' );
  test.equivalent( got, [ 1, 1, 2, { a : 2 } ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var dst = _.unroll.make( [ 1, 1, 2, 'str', { a : 2 } ] );
  var got = _.unrollRemove( dst, 4 );
  test.equivalent( got, [ 1, 1, 2, 'str', { a : 2 } ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'unroll remove elements';
  var dst = _.unroll.make( [ 1, 1, 2, 'str', [ 1 ] ] );
  var got = _.unrollRemove( dst, 1, [ 1 ] );
  test.equivalent( got, [ 2, 'str', [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var dst = _.unroll.make( [ 1, 1, 2, 'str', { a : 2 }, 'str' ] );
  var got = _.unrollRemove( dst, 0, { a : 2 }, 4, 'str' );
  test.equivalent( got, [ 1, 1, 2, { a : 2 } ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var dst = _.unroll.make( [ 1, 1, 2, 'str', { a : 2 } ] );
  var got = _.unrollRemove( dst, null, undefined, 4, [] );
  test.equivalent( got, [ 1, 1, 2, 'str', { a : 2 } ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.case = 'unroll remove unroll';
  var dst = _.unroll.make( [ 1, 1, 2, 3, 'str', 3 ] );
  var got = _.unrollRemove( dst, _.unroll.from( [ 1, 3 ] ) );
  test.equivalent( got, [ 2, 'str' ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var dst = _.unroll.make( [ 1, 2, 1, 3, 'str', [ 1 ] ] );
  var got = _.unrollRemove( dst, _.unroll.from( [ 1, 3, 'str', [ 1 ] ] ) );
  test.equivalent( got, [ 2, [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var dst = _.unroll.make( [ 1, 2, 3, 'str', [ 1 ] ] );
  var got = _.unrollRemove( dst, _.unroll.from( [ 0, 'a', [ 2 ] ] ) );
  test.equivalent( got, [ 1, 2, 3, 'str', [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  var ins =  _.unroll.from( [ 1, _.unroll.make( [ 2, 3, _.unroll.make( [ 'str', [ 1 ] ] ) ] ) ] );
  var got = _.unrollRemove( _.unroll.from( [ 1, 2, 3, 'str', [ 1 ] ] ), ins );
  test.equivalent( got, [ [ 1 ] ] );
  test.true( _.arrayIs( got ) );
  test.true( _.unrollIs( got ) );

  test.close( 'dstArray is unroll' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.unrollRemove() );

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.unrollRemove( 1, 1 ) );
  test.shouldThrowErrorSync( () => _.unrollRemove( 'wrong', 1 ) );
  test.shouldThrowErrorSync( () => _.unrollRemove( undefined, 1 ) );
}

// --
//
// --

const Proto =
{

  name : 'Tools.Unroll.l0.l1',
  silencing : 1,
  enabled : 1,
  routineTimeOut : 10000,

  tests :
  {

    is,
    isPopulated,

    make,
    makeLongDescriptor,
    makeUndefined,
    makeUndefinedLongDescriptor,
    from,
    fromLongDescriptor,
    // unrollsFrom,
    // unrollsFromLongDescriptor,
    // fromMaybe,
    // fromMaybeLongDescriptor,
    unrollNormalize,

    // unrollSelect,

    unrollPrepend,
    unrollAppend,
    unrollRemove,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
