( function _l0_l9_Array_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

//--
//
//--

function constructorLikeArray( test )
{

  test.case = 'an array';
  var got = _.constructorLikeArray( [].constructor );
  var expected = true;
  test.identical( got, expected );

  test.case = 'arguments, not possible to say yes by constructor';
  var got = _.constructorLikeArray( arguments.constructor );
  var expected = false;
  test.identical( got, expected );

  test.case = 'raw array buffer';
  var got = _.constructorLikeArray( new BufferRaw( 10 ).constructor );
  var expected = false;
  test.identical( got, expected );

  test.case = 'typed array buffer';
  var got = _.constructorLikeArray( new F32x( 10 ).constructor );
  var expected = true;
  test.identical( got, expected );

  test.case = 'no argument';
  var got = _.constructorLikeArray();
  var expected  = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.constructorLikeArray( null );
  var expected  = false;
  test.identical( got, expected );

  test.case = 'number';
  var got = _.constructorLikeArray( 1 );
  var expected  = false;
  test.identical( got, expected );

  test.case = 'function';
  var got = _.constructorLikeArray( (function() {}).constructor );
  var expected  = false;
  test.identical( got, expected );

  test.case = 'string';
  var got = _.constructorLikeArray( 'x'.constructor );
  var expected  = false;
  test.identical( got, expected );

  test.case = 'string';
  var got = _.constructorLikeArray( 'x' );
  var expected  = false;
  test.identical( got, expected );

  test.case = 'object';
  var got = _.constructorLikeArray( {}.constructor );
  var expected  = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

}

// --
//
// --

function arrayMakeNotDefaultDescriptor( test )
{

  let times = 2; // quantity of elements in LongDescriptors map

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;
  //   let long = _.withLong[ name ];

  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let name = namespace.TypeName;
    let long = _.withLong[ name ];

    test.open( `long - ${ name }` );
    run( long );
    test.close( `long - ${ name }` );

    if( times < 1 )
    break;
    times--;
  }

  /* test subroutine */

  function run( long )
  {
    test.case = 'without arguments';
    var got = long.array.make();
    var expected = [];
    test.identical( got, expected );
    test.true( _.arrayIs( got ) );

    // test.case = 'src = undefined';
    // var src = undefined;
    // var got = long.array.make( src );
    // var expected = [];
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( src !== got );

    test.case = 'src = null';
    var src = null;
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty array';
    var src = [];
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length = 1';
    var src = [ 0 ];
    var got = long.array.make( src );
    var expected = [ 0 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length > 1';
    var src = [ 1, 2, 3 ];
    var got = long.array.make( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = number, src = 0';
    var got = long.array.make( 0 );
    var expected = new Array( 0 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = number, src > 0';
    var got = long.array.make( 3 );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty U8x';
    var src = new U8x();
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length = 1';
    var src = new U8x( 1 );
    var got = long.array.make( src );
    var expected = [ 0 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length > 1';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = long.array.make( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty I16x';
    var src = new I16x();
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length = 1';
    var src = new I16x( 1 );
    var got = long.array.make( src );
    var expected = [ 0 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length > 1';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = long.array.make( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty F32x';
    var src = new F32x();
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1';
    var src = new F32x( 1 );
    var got = long.array.make( src );
    var expected = [ 0 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length > 1';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = long.array.make( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length = 1';
    var src = _.argumentsArray.make( [ {} ] );
    var got = long.array.make( src );
    var expected = [ {} ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length > 1';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.array.make( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty unroll';
    var src = _.unroll.make( [] );
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 'str' ] );
    var got = long.array.make( src );
    var expected = [ 'str' ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length > 1';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = long.array.make( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty Set';
    var src = new Set( [] );
    var got = long.array.make( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = Set, src.length = 1';
    var src = new Set( [ 'str' ] );
    var got = long.array.make( src );
    var expected = [ 'str' ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = Set, src.length > 1';
    var src = new Set( [ 1, 2, 3 ] );
    var got = long.array.make( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.shouldThrowErrorSync( () => long.array.make( undefined ) );

  }

}

arrayMakeNotDefaultDescriptor.timeOut = 15000;

//

function arrayMakeUndefinedNotDefaultDescriptor( test )
{
  let times = 2; // quantity of elements in LongDescriptors map

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;
  //   let long = _.withLong[ name ];

  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let name = namespace.TypeName;
    let long = _.withLong[ name ];

    test.open( `long - ${ name }` );
    run( long );
    test.close( `long - ${ name }` );

    if( times < 1 )
    break;
    times--;
  }

  /* test subroutine */

  function run( long )
  {
    test.case = 'without arguments';
    var got = long.array.makeUndefined();
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );

    test.case = 'src = null';
    var src = null;
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = null, length = 2';
    var src = null;
    var got = long.array.makeUndefined( src, 2 );
    var expected = new Array( 2 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty array';
    var src = [];
    var got = long.array.makeUndefined( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty array, length = 2';
    var src = [];
    var got = long.array.makeUndefined( src, 2 );
    var expected = new Array( 2 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length = 1';
    var src = [ 0 ];
    var got = long.array.makeUndefined( src );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length = 1, length = 2';
    var src = [ 0 ];
    var got = long.array.makeUndefined( src, 2 );
    var expected = new Array( 2 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length > 1';
    var src = [ 1, 2, 3 ];
    var got = long.array.makeUndefined( src );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length > 1, length < src.length';
    var src = [ 1, 2, 3 ];
    var got = long.array.makeUndefined( src, 2 );
    var expected = new Array( 2 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = number, src = 0';
    var got = long.array.makeUndefined( 0 );
    var expected = new Array( 0 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    // test.case = 'src = number, src = 0, length > src';
    // var got = long.array.makeUndefined( 0, 2 );
    // var expected = new Array( 2 );
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( src !== got );

    test.case = 'src = number, src > 0';
    var got = long.array.makeUndefined( 3 );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    // test.case = 'src = number, src > 0, length < src';
    // var got = long.array.makeUndefined( 3, 1 );
    // var expected = new Array( 1 );
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( src !== got );

    /* */

    test.case = 'src = empty U8x';
    var src = new U8x();
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty U8x, length = 2';
    var src = new U8x();
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length = 1';
    var src = new U8x( 1 );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length = 1, length > src.length';
    var src = new U8x( 1 );
    var got = long.array.makeUndefined( src, 3 );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length > 1';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length > 1, length < src.length';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src, 0 );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty I16x';
    var src = new I16x();
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty I16x, length = 2';
    var src = new I16x();
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length = 1';
    var src = new I16x( 1 );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length = 1, length > src.length';
    var src = new I16x( 1 );
    var got = long.array.makeUndefined( src, 3 );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length > 1';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length > 1, length < src.length';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src, 0 );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty F32x';
    var src = new F32x();
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty F32x, length = 2';
    var src = new F32x();
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1';
    var src = new F32x( 1 );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1, length > src.length';
    var src = new F32x( 1 );
    var got = long.array.makeUndefined( src, 3 );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length > 1';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length > 1, length < src.length';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src, 0 );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = empty arguments array, length > 0';
    var src = _.argumentsArray.make( [] );
    var got = long.array.makeUndefined( src, 2 );
    var expected = new Array( 2 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length = 1';
    var src = _.argumentsArray.make( [ {} ] );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length = 1, length > src.length';
    var src = _.argumentsArray.make( [ {} ] );
    var got = long.array.makeUndefined( src, 2 );
    var expected = new Array( 2 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length > 1';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length > 1, length < src.length';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src, 1 );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty unroll';
    var src = _.unroll.make( [] );
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty unroll, length = 2';
    var src = _.unroll.make( [] );
    var got = long.array.makeUndefined( src );
    var expected = new Array();
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 'str' ] );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length = 1, length > src.length';
    var src = _.unroll.make( [ 'str' ] );
    var got = long.array.makeUndefined( src, 2 );
    var expected = new Array( 2 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length > 1';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src );
    var expected = new Array( 3 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length > 1, length < src.length';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = long.array.makeUndefined( src, 1 );
    var expected = new Array( 1 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.unrollIs( got ) );
    test.true( src !== got );

    /* */

    // test.case = 'src = empty Set';
    // var src = new Set( [] );
    // var got = long.array.makeUndefined( src );
    // var expected = new Array();
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( !_.set.is( got ) );
    // test.true( src !== got );
    //
    // test.case = 'src = empty Set, length = 2';
    // var src = new Set( [] );
    // var got = long.array.makeUndefined( src, 2 );
    // var expected = new Array( 2 );
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( !_.set.is( got ) );
    // test.true( src !== got );
    //
    // test.case = 'src = Set, src.size = 1';
    // var src = new Set( [ 'str' ] );
    // var got = long.array.makeUndefined( src );
    // var expected = new Array( 1 );
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( !_.set.is( got ) );
    // test.true( src !== got );
    //
    // test.case = 'src = Set, src.length = 1, length > src.length';
    // var src = new Set( [ 'str' ] );
    // var got = long.array.makeUndefined( src, 2 );
    // var expected = new Array( 2 );
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( !_.set.is( got ) );
    // test.true( src !== got );
    //
    // test.case = 'src = unroll, src.length > 1';
    // var src = new Set( [ 1, 2, 3 ] );
    // var got = long.array.makeUndefined( src );
    // var expected = new Array( 3 );
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( !_.set.is( got ) );
    // test.true( src !== got );
    //
    // test.case = 'src = unroll, src.length > 1, length < src.length';
    // var src = new Set( [ 1, 2, 3 ] );
    // var got = long.array.makeUndefined( src, 1 );
    // var expected = new Array( 1 );
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( !_.set.is( got ) );
    // test.true( src !== got );

    test.shouldThrowErrorSync( () => long.array.makeUndefined( undefined ) );
    test.shouldThrowErrorSync( () => long.array.makeUndefined( 1, 1 ) );
    test.shouldThrowErrorSync( () => long.array.makeUndefined( 1, [] ) );
    test.shouldThrowErrorSync( () => long.array.makeUndefined( new Set ) );

  }
}
arrayMakeUndefinedNotDefaultDescriptor.timeOut = 15000;

//

function arrayFromLongDescriptor( test )
{
  let times = 4;

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;
  //   let long = _.withLong[ name ];

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
    test.case = 'src = null';
    var src = null;
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    // test.case = 'src = undefined';
    // var src = undefined;
    // var got = long.array.from( src );
    // var expected = [];
    // test.equivalent( got, expected );
    // test.true( _.arrayIs( got ) );
    // test.true( src !== got );

    test.case = 'src = number';
    var src = 5;
    var got = long.array.from( src );
    var expected = new Array( 5 );
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty array';
    var src = [];
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src === got );

    test.case = 'src = empty unroll';
    var src = _.unroll.make( [] );
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src === got );

    test.case = 'src = empty argumentsArray';
    var src = _.argumentsArray.make( [] );
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = empty I8x';
    var src = new I8x();
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty U16x';
    var src = new U16x();
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = empty F32x';
    var src = new F32x();
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = array, src.length = 1';
    var src = [ 1 ];
    var got = long.array.from( src );
    var expected = [ 1 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src === got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 1 ] );
    var got = long.array.from( src );
    var expected = [ 1 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src === got );

    test.case = 'src = argumentsArray, src.length = 1';
    var src = _.argumentsArray.make( [ 'str' ] );
    var got = long.array.from( src );
    var expected = [ 'str' ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = I8x, src.length = 1';
    var src = new I8x( [ 1 ] );
    var got = long.array.from( src );
    var expected = [ 1 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U16x, src.length = 1';
    var src = new U16x( [ 1 ] );
    var got = long.array.from( src );
    var expected = [ 1 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1';
    var src = new F32x( [ 2 ] );
    var got = long.array.from( src );
    var expected = [ 2 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = array, src.length > 1';
    var src = [ 1, 2, 'str' ];
    var got = long.array.from( src );
    var expected = [ 1, 2, 'str' ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src === got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 1, 2, 'str' ] );
    var got = long.array.from( src );
    var expected = [ 1, 2, 'str' ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( _.unrollIs( got ) );
    test.true( src === got );

    test.case = 'src = argumentsArray, src.length = 1';
    var src = _.argumentsArray.make( [ 1, 2, 'str' ] );
    var got = long.array.from( src );
    var expected = [ 1, 2, 'str' ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( !_.argumentsArray.is( got ) );
    test.true( src !== got );

    test.case = 'src = I8x, src.length = 1';
    var src = new I8x( [ 1, 2, 3 ] );
    var got = long.array.from( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = U16x, src.length = 1';
    var src = new U16x( [ 1, 2, 3 ] );
    var got = long.array.from( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = long.array.from( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = array from _.array.make, src = empty array';
    var src = _.array.make( [] );
    var got = long.array.from( src );
    var expected = [];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src === got );

    test.case = 'src = array from _.array.make, src.length = 1';
    var src = _.array.make( [ 'a' ] );
    var got = long.array.from( src );
    var expected = [ 'a' ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src === got );

    test.case = 'src = array from _.array.make, src.length > 1';
    var src = _.array.make( [ 1, 2, 3 ] );
    var got = long.array.from( src );
    var expected = [ 1, 2, 3 ];
    test.equivalent( got, expected );
    test.true( _.arrayIs( got ) );
    test.true( src === got );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.array.from() );
      test.shouldThrowErrorSync( () => long.array.from( undefined ) );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.array.from( 1, 3 ) );
      test.shouldThrowErrorSync( () => long.array.from( [], 3 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.array.from( {} ) );
      test.shouldThrowErrorSync( () => long.array.from( 'wrong' ) );
    }
  }
}

arrayFromLongDescriptor.timeOut = 15000;

//

function hasLength( test )
{

  test.case = 'an empty array';
  var got = _.vector.hasLength( [] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'an array';
  var got = _.vector.hasLength( [ 1, 2, 3 ] );
  var expected  = true;
  test.identical( got, expected );

  test.case = 'a pseudo array';
  var got = ( function()
  {
    return _.vector.hasLength( arguments );
  } )('Hello there!');
  var expected = true;
  test.identical( got, expected );

  test.case = 'an array-like';
  var got = _.vector.hasLength( { '0' : 1, '1' : 2, '2' : 3, 'length' : 3 } );
  var expected = true;
  test.identical( got, expected );

  test.case = 'a Function.length';
  function fn( a, b, c ) { };
  var got = _.vector.hasLength( fn );
  var expected = true;
  test.identical( got, expected );

  test.case = 'a "string".length';
  var got = _.vector.hasLength( 'Hello there!' );
  var expected = true;
  test.identical( got, expected );

  test.case = 'no arguments';
  var got = _.vector.hasLength();
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.vector.hasLength();
  var expected = false;
  test.identical( got, expected );

  /**/

  if( !Config.debug )
  return;

};

//

function arrayFromCoercing( test )
{
  test.case = 'src - empty array';
  var src = [];
  var got = _.arrayFromCoercing( src );
  var exp = [];
  test.identical( got, exp );
  test.true( got === src );

  test.case = 'src - filled array';
  var src = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
  var got = _.arrayFromCoercing( src );
  var exp = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
  test.identical( got, exp );
  test.true( got === src );

  test.case = 'src - empty argumentsArray';
  var src = _.argumentsArray.make( [] );
  var got = _.arrayFromCoercing( src );
  var exp = [];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - filled argumentsArray';
  var src = _.argumentsArray.make( [ 3, 7, 13, 'abc', false, undefined, null, {} ] );
  var got = _.arrayFromCoercing( src );
  var exp = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - empty unroll';
  var src = _.unroll.make( [] );
  var got = _.arrayFromCoercing( src );
  var exp = [];
  test.identical( got, exp );
  test.true( !_.unrollIs( got ) );
  test.true( got !== src );

  test.case = 'src - filled unroll';
  var src = _.unroll.make( [ 3, 7, 13, 'abc', false, undefined, null, {} ] );
  var got = _.arrayFromCoercing( src );
  var exp = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
  test.identical( got, exp );
  test.true( !_.unrollIs( got ) );
  test.true( got !== src );

  test.case = 'src - empty BufferTyped - U8x';
  var src = new U8x( [] );
  var got = _.arrayFromCoercing( src );
  var exp = [];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - filled BufferTyped - F64x';
  var src = new F64x( [ 3, 7, 13, 0, 2 ] );
  var got = _.arrayFromCoercing( src );
  var exp = [ 3, 7, 13, 0, 2 ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - empty string';
  var src = '';
  var got = _.arrayFromCoercing( src );
  var exp = [ undefined ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - string with not number literals';
  var src = 'a bc def';
  var got = _.arrayFromCoercing( src );
  var exp = [ NaN, NaN, NaN ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - string with number literals';
  var src = '0 1.2 345';
  var got = _.arrayFromCoercing( src );
  var exp = [ 0, 1.2, 345 ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - string with mixed literals';
  var src = '0 1.2 34,5 a6';
  var got = _.arrayFromCoercing( src );
  var exp = [ 0, 1.2, 34, 5, NaN ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - empty map';
  var src = {};
  var got = _.arrayFromCoercing( src );
  var exp = [];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - filled map';
  var src = { a : 3, b : 7, c : '13' };
  var got = _.arrayFromCoercing( src );
  var exp = [ [ 'a', 3 ], [ 'b', 7 ], [ 'c', '13' ] ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - empty pure map';
  var src = Object.create( null );
  var got = _.arrayFromCoercing( src );
  var exp = [];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - filled pure map';
  var src = Object.create( null );
  src.a = 3;
  src.b = '13';
  var got = _.arrayFromCoercing( src );
  var exp = [ [ 'a', 3 ], [ 'b', '13' ] ];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - empty constructor instance';
  function Constr1(){ return this };
  var src = new Constr1();
  var got = _.arrayFromCoercing( src );
  var exp = [];
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'src - filled pure map';
  function Constr2(){ this.x = 1; this.y = 'a'; return this };
  var src = new Constr2();
  var got = _.arrayFromCoercing( src );
  var exp = [ [ 'x', 1 ], [ 'y', 'a' ] ];
  test.identical( got, exp );
  test.true( got !== src );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFromCoercing() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFromCoercing( [ 1, 2 ], 'extra' ) );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.arrayFromCoercing( 6 ) );
  test.shouldThrowErrorSync( () => _.arrayFromCoercing( true ) );
  test.shouldThrowErrorSync( () => _.arrayFromCoercing( new HashMap() ) );
}

//

function arrayFromCoercingLongDescriptor( test )
{
  let times = 4;

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;
  //   let long = _.withLong[ name ];

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
    test.case = 'src - empty array';
    var src = [];
    var got = long.arrayFromCoercing( src );
    var exp = [];
    test.identical( got, exp );
    test.true( got === src );

    test.case = 'src - filled array';
    var src = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
    var got = long.arrayFromCoercing( src );
    var exp = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
    test.identical( got, exp );
    test.true( got === src );

    test.case = 'src - empty argumentsArray';
    var src = _.argumentsArray.make( [] );
    var got = long.arrayFromCoercing( src );
    var exp = [];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - filled argumentsArray';
    var src = _.argumentsArray.make( [ 3, 7, 13, 'abc', false, undefined, null, {} ] );
    var got = long.arrayFromCoercing( src );
    var exp = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - empty unroll';
    var src = _.unroll.make( [] );
    var got = long.arrayFromCoercing( src );
    var exp = [];
    test.identical( got, exp );
    test.true( !_.unrollIs( got ) );
    test.true( got !== src );

    test.case = 'src - filled unroll';
    var src = _.unroll.make( [ 3, 7, 13, 'abc', false, undefined, null, {} ] );
    var got = long.arrayFromCoercing( src );
    var exp = [ 3, 7, 13, 'abc', false, undefined, null, {} ];
    test.identical( got, exp );
    test.true( !_.unrollIs( got ) );
    test.true( got !== src );

    test.case = 'src - empty BufferTyped - U8x';
    var src = new U8x( [] );
    var got = long.arrayFromCoercing( src );
    var exp = [];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - filled BufferTyped - F64x';
    var src = new F64x( [ 3, 7, 13, 0, 2 ] );
    var got = long.arrayFromCoercing( src );
    var exp = [ 3, 7, 13, 0, 2 ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - empty string';
    var src = '';
    var got = long.arrayFromCoercing( src );
    var exp = [ undefined ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - string with not number literals';
    var src = 'a bc def';
    var got = long.arrayFromCoercing( src );
    var exp = [ NaN, NaN, NaN ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - string with number literals';
    var src = '0 1.2 345';
    var got = long.arrayFromCoercing( src );
    var exp = [ 0, 1.2, 345 ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - string with mixed literals';
    var src = '0 1.2 34,5 a6';
    var got = long.arrayFromCoercing( src );
    var exp = [ 0, 1.2, 34, 5, NaN ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - empty map';
    var src = {};
    var got = long.arrayFromCoercing( src );
    var exp = [];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - filled map';
    var src = { a : 3, b : 7, c : '13' };
    var got = long.arrayFromCoercing( src );
    var exp = [ [ 'a', 3 ], [ 'b', 7 ], [ 'c', '13' ] ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - empty pure map';
    var src = Object.create( null );
    var got = long.arrayFromCoercing( src );
    var exp = [];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - filled pure map';
    var src = Object.create( null );
    src.a = 3;
    src.b = '13';
    var got = long.arrayFromCoercing( src );
    var exp = [ [ 'a', 3 ], [ 'b', '13' ] ];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - empty constructor instance';
    function Constr1(){ return this };
    var src = new Constr1();
    var got = long.arrayFromCoercing( src );
    var exp = [];
    test.identical( got, exp );
    test.true( got !== src );

    test.case = 'src - filled pure map';
    function Constr2(){ this.x = 1; this.y = 'a'; return this };
    var src = new Constr2();
    var got = long.arrayFromCoercing( src );
    var exp = [ [ 'x', 1 ], [ 'y', 'a' ] ];
    test.identical( got, exp );
    test.true( got !== src );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.arrayFromCoercing() );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.arrayFromCoercing( [ 1, 2 ], 'extra' ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.arrayFromCoercing( 6 ) );
      test.shouldThrowErrorSync( () => long.arrayFromCoercing( true ) );
      test.shouldThrowErrorSync( () => long.arrayFromCoercing( new HashMap() ) );
    }
  }
}

arrayFromCoercingLongDescriptor.timeOut = 15000;

//

function arraySlice( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    test.case = 'f = undefined, l = undefined';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = undefined, l = src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, undefined, src.length );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = undefined, l < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, undefined, 3 );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = undefined, l < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, undefined, -2 );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = undefined, l > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, undefined, src.length + 5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'f = 0, l = undefined';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 0, src.length );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    var expected2 = _.array.slice( src );
    test.identical( got, expected );
    test.identical( got, expected2 );
    test.true( got !== src );

    test.case = 'f = 0, l = src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 0, src.length );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    var expected2 = _.array.slice( src );
    test.identical( got, expected );
    test.identical( got, expected2 );
    test.true( got !== src );

    test.case = 'f = 0, l < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 0, src.length - 2 );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = 0, l < 0';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 0, -2 );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = 0, l > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 0, src.length + 5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    var expected2 = _.array.slice( src );
    test.identical( got, expected );
    test.identical( got, expected2 );
    test.true( got !== src );

    /* */

    test.case = 'f > 0, l = undefined';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 2 );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > 0, l = src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 2, src.length );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > 0, l < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 2, src.length - 2 );
    var expected = [ 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > 0, l < 0';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 2, -2 );
    var expected = [ 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > 0, l > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, 2, src.length + 2 );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'f < 0, l = undefined';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, -2 );
    var expected = [ 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, l = src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, -2, src.length );
    var expected = [ 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, l < src.length, l > f';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, -2, src.length - 1 );
    var expected = [ 'str' ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, l < src.length, l < 0';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, -2, -1 );
    var expected = [ 'str' ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, l < src.length, l < f';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, -2, src.length - 3 );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'f > src.length, l = undefined';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, src.length + 1, src.length );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > src.length, l = src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, src.length + 1, src.length );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = undefined, l < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, src.length + 1, 3 );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = undefined, l < 0';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, src.length + 1, -2 );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f = undefined, l > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.array.slice( src, src.length + 1, src.length + 5 );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.array.slice() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.array.slice( [ 1, 2, 3 ], 1, 2, 'extra' ) );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.array.slice( 'wrong' ) );
  test.shouldThrowErrorSync( () => _.array.slice( _.argumentsArray.make( [ 1, 2 ] ) ) );
  test.shouldThrowErrorSync( () => _.array.slice( new F32x( 2 ) ) );
}

//

function arrayEmpty( test )
{
  test.case = 'empty array';
  var src = [];
  var got = _.array.empty( src );
  test.identical( got, [] );
  test.true( got === src );

  test.case = 'filled array';
  var src = [ 1, undefined, null, false, [], {}, new Set(), '', 'str' ];
  var got = _.array.empty( src );
  test.identical( got, [] );
  test.true( got === src );

  test.case = 'empty unroll';
  var src = _.unroll.make( [] );
  var got = _.array.empty( src );
  test.identical( got, [] );
  test.true( got === src );

  test.case = 'filled unroll';
  var src = _.unroll.make( [ 1, undefined, null, false, [], {}, new Set(), '', 'str' ] );
  var got = _.array.empty( src );
  test.identical( got, [] );
  test.true( got === src );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.array.empty() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.array.empty( [], [] ) );

  test.case = 'wrong type of dstArray';
  test.shouldThrowErrorSync( () => _.array.empty( _.argumentsArray.make( [] ) ) );
  test.shouldThrowErrorSync( () => _.array.empty( new U8x( 10 ) ) );
  test.shouldThrowErrorSync( () => _.array.empty( 'str' ) );
}

//

function arrayBut( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    test.case = 'range = undefined, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, 1 );
    var expected = [ 1, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, 0, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ { a : 1 }, 2, [ 10 ], 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, -5, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ { a : 1 }, 2, [ 10 ], 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = undefined, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, 1 );
    var expected = [ 1, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, 0, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ { a : 1 }, 2, [ 10 ], 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, -5, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ { a : 1 }, 2, [ 10 ], 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, [ 0, 2 ] );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, [ -5, 2 ] );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, [ 0, -5 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, [ 0, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ { a : 1 }, 2, [ 10 ], 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, [ -5, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ { a : 1 }, 2, [ 10 ], 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, [ 0, -5 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ { a : 1 }, 2, [ 10 ], 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > range[ 1 ]';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut( src, [ 3, 0 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayBut() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayBut( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );

  test.case = 'ins is not long';
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], 0, 4 ) );
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], 0, { a : 1 } ) );
  test.shouldThrowErrorSync( () => _.arrayBut( [ 1, 2 ], 0, new BufferRaw( 2 ) ) );

}

//

function arrayButInplace( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    test.case = 'range = undefined, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, 1 );
    var expected = make( [ 1, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, -5 );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, 0, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ { a : 1 }, 2, [ 10 ], 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, -5, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ { a : 1 }, 2, [ 10 ], 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, [ 0, 2 ] );
    var expected = ( [ 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, [ -5, 2 ] );
    var expected = make( [ 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, [ 0, -5 ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, [ 0, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ { a : 1 }, 2, [ 10 ], 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, [ -5, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ { a : 1 }, 2, [ 10 ], 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, [ 0, -5 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ { a : 1 }, 2, [ 10 ], 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ]';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayButInplace( src, [ 3, 0 ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range = number, not src';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayButInplace( dst, 2 );
    var expected = make( [ 1, 2, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayButInplace( dst, 0, [ 0 ] );
    var expected = make( [ 0, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range, src.length > range[ 1 ] - range[ 0 ]';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = [ 1, 2, 3 ];
    test.shouldThrowErrorSync( () => _.arrayButInplace( dst, [ 1, 3 ], src ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayButInplace() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayButInplace( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayButInplace( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayButInplace( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );

  test.case = 'ins is not long';
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], 0, 4 ) );
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], 0, { a : 1 } ) );
  test.shouldThrowErrorSync( () => _.arrayButInplace( [ 1, 2 ], 0, new BufferRaw( 2 ) ) );

}

//

function arrayBut_( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    test.open( 'not inplace' );

    test.case = 'range = undefined, not ins';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayBut_( null, src );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, 1 );
    var expected = [ 1, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, 0, [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, -5, [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, [ 0, 1 ] );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, [ -5, 1 ] );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, [ 0, -5 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, [ 0, 1 ], [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, [ -5, 1 ], [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, [ 0, -5 ], [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > range[ 1 ]';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( null, src, [ 3, 0 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.close( 'not inplace' );

    /* - */

    test.open( 'dst === src' );

    test.case = 'range = undefined, not ins';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayBut_( src, src );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, src, 1 );
    var expected = [ 1, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, src, 0, [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, src, -5, [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, [ 0, 1 ] );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, [ -5, 1 ] );
    var expected = [ 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, [ 0, -5 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, [ 0, 1 ], [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, [ -5, 1 ], [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, [ 0, -5 ], [ 0, 0, 0 ] );
    var expected = [ 0, 0, 0, 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ]';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( src, [ 3, 0 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range = number, not src';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayBut_( dst, 2 );
    var expected = make( [ 1, 2, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayBut_( dst, 0, [ 0 ] );
    var expected = make( [ 0, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range, src.length > range[ 1 ] - range[ 0 ]';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = [ 1, 2, 3 ];
    test.shouldThrowErrorSync( () => _.arrayBut_( dst, [ 1, 2 ], src ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );

    test.close( 'dst === src' );

    /* - */

    test.open( 'dst !== src' );

    test.case = 'range = number, ins';
    var dst = [ 1 ];
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( dst, src, 0, [ 0, 0, 0 ] );
    var expected = make( [ 0, 0, 0, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got !== src );
    test.true( got === dst );

    test.case = 'range = negative number, ins';
    var dst = [ 1, 2, 3, 'str' ];
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( dst, src, -5, [ 0, 0, 0 ] );
    var expected = make( [ 0, 0, 0, 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got !== src );
    test.true( got === dst );

    /* range = array range */

    test.case = 'range = array range, ins';
    var dst = [];
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( dst, src, [ 0, 1 ], [ 0, 0, 0 ] );
    var expected = make( [ 0, 0, 0, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got !== src );
    test.true( got === dst );

    test.case = 'range = array range, range[ 0 ] < 0, ins';
    var dst = [ { a : 2 }, 1, 2 ];
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( dst, src, [ -5, 1 ], [ 0, 0, 0 ] );
    var expected = make( [ 0, 0, 0, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got !== src );
    test.true( got === dst );

    test.case = 'range = array range, range[ 1 ] < 0, ins';
    var dst = [ null, undefined, 1, 0 ];
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayBut_( dst, src, [ 0, -5 ], [ 0, 0, 0 ] );
    var expected = make( [ 0, 0, 0, 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got !== src );
    test.true( got === dst );

    /* */

    test.case = 'container is not extensible, range = number, not src';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [] );
    var got = _.arrayBut_( dst, src, 2, [] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range, src.length > range[ 1 ] - range[ 0 ]';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [ 1, 2, 3, 4 ] );
    test.shouldThrowErrorSync( () => _.arrayBut_( dst, src, [ 1, 2 ], src ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );
    test.identical( src, expected );

    test.close( 'dst !== src' );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayBut_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayBut_( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut_( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut_( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );

  test.case = 'ins is not long';
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], 0, 4 ) );
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], 0, { a : 1 } ) );
  test.shouldThrowErrorSync( () => _.arrayBut_( [ 1, 2 ], 0, new BufferRaw( 2 ) ) );
}

arrayBut_.timeOut = 10000;

//

function arrayBut_CheckReturnedContainer( test )
{
  test.case = 'dst - undefined, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayBut_( src, 1, [ 3, 4 ] );
  var expected = [ 1, 3, 4, 3 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - null, new container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayBut_( null, src, 1, [ 3, 4 ] );
  var expected = [ 1, 3, 4, 3 ];
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'dst - src, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayBut_( src, src, 1, [ 3, 4 ] );
  var expected = [ 1, 3, 4, 3 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - another container, dst container';
  var src = [ 1, 2, 3 ];
  var dst = [];
  var got = _.arrayBut_( dst, src, 1, [ 3, 4 ] );
  var expected = [ 1, 3, 4, 3 ];
  test.identical( got, expected );
  test.true( got === dst );
}

//

function arrayShrink( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, 1 );
    var expected = [ 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, 0, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, -5, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, [ 0, 2 ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, [ -5, 2 ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, [ 0, -5 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, [ 0, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, [ -5, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, [ 0, -5 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > range[ 1 ]';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrink( src, [ 3, 0 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayShrink() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayShrink( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayShrink( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrink( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrink( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayShrink( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrink( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrink( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrink( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );

}

//

function arrayShrinkInplace( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, 1 );
    var expected = make( [ 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, -5 );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, 0, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, -5, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, [ 0, 2 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, [ -5, 2 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, [ 0, -5 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, [ 0, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, [ -5, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, [ 0, -5 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ]';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayShrinkInplace( src, [ 3, 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrinkInplace( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );

}

//

function arrayShrink_( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    test.open( 'dst === null' );

    test.case = 'range =  number < 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, -5 );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number === 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, 0 );
    var expected = make( [ 1 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number > 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, 1 );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, 6 );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'range[ 0 ] < 0, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ -1, -3 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] === range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ -1, -1 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] === 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ -1, 0 ] );
    var expected = make( [ 1 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] > 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ -1, 1 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ -1, 6 ] );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'range[ 0 ] === 0, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 0, -1 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] === 0, range[ 1 ] === 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 0, 0 ] );
    var expected = make( [ 1 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] === 0, range[ 1 ] > 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 0, 1 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] === 0, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 0, 6 ] );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'range[ 0 ] > 0, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 1, -3 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > 0, range[ 1 ] === range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 1, 1 ] );
    var expected = make( [ 2 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > 0, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 1, 6 ] );
    var expected = make( [ 2, 3 ] );
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'range[ 0 ] > src.length, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 4, -3 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > src.length, range[ 1 ] === range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 4, 4 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > src.length, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( null, src, [ 4, 6 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.close( 'dst === null' );

    /* - */

    test.open( 'dst === src' );

    test.case = 'range =  number < 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, -5 );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number === 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, 0 );
    var expected = make( [ 1 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number > 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, 1 );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, 6 );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'range[ 0 ] < 0, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ -1, -3 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] === range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ -1, -1 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] === 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ -1, 0 ] );
    var expected = make( [ 1 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] > 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ -1, 1 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ -1, 6 ] );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'range[ 0 ] === 0, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 0, -1 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] === 0, range[ 1 ] === 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 0, 0 ] );
    var expected = make( [ 1 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] === 0, range[ 1 ] > 0';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 0, 1 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] === 0, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 0, 6 ] );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'range[ 0 ] > 0, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 1, -3 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, range[ 1 ] === range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 1, 1 ] );
    var expected = make( [ 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 1, 6 ] );
    var expected = make( [ 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'range[ 0 ] > src.length, range[ 1 ] < range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 4, -3 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > src.length, range[ 1 ] === range[ 0 ]';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 4, 4 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > src.length, range[ 1 ] > src.length';
    var src = make( [ 1, 2, 3 ] );
    var got = _.arrayShrink_( src, src, [ 4, 6 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.close( 'dst === src' );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayShrink_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayShrink_( [ 1, 2 ], [ 2, 3, 4 ], [ 1, 4 ], 4 ) );

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.arrayShrink_( { a : 1 }, [ 1, 2, 3 ], [ 0, 1 ] ) );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.arrayShrink_( [ 1, 2, 3 ], null, [ 1, 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayShrink_( [ 1, 2, 3 ], 'str', [ 1, 1 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayShrink_( [ 1, 2 ], 'str' ) );
  test.shouldThrowErrorSync( () => _.arrayShrink_( [ 1, 2 ], [] ) );
  test.shouldThrowErrorSync( () => _.arrayShrink_( [ 1, 2 ], [ 'a', 'b' ] ) );
}
//

function arrayShrink_CheckReturnedContainer( test )
{
  test.case = 'dst - undefined, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayShrink_( src, 1 );
  var expected = [ 1, 2 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - null, new container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayShrink_( null, src, 1 );
  var expected = [ 1, 2 ];
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'dst - src, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayShrink_( src, src, 1 );
  var expected = [ 1, 2 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - another container, dst container';
  var src = [ 1, 2, 3 ];
  var dst = [];
  var got = _.arrayShrink_( dst, src, 1 );
  var expected = [ 1, 2 ];
  test.identical( got, expected );
  test.true( got === dst );
}

//

function arrayGrow( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    /* range = number */

    test.case = 'range = number, number < src length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow( src, 1 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow( src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow( src, 6, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ], [ { a : 1 }, 2, [ 10 ] ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow( src, -5, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'only src';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range > src.length, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ 0, src.length + 2 ] );
    var expected = src.length + 2;
    test.identical( got, [ 1, 2, 3, 4, 5, undefined, undefined ] );
    test.identical( got.length, expected );
    test.true( got !== src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ 0, src.length + 2 ], 0 );
    var expected = [ 1, 2, 3, 4, 5, 0, 0 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ src.length - 1, src.length * 2 ], 0 );
    var expected = [ 1, 2, 3, 4, 5, 0, 0, 0, 0, 0 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range < src.length';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ 0, 3 ] );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range < src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ 0, 3 ], 0 );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    got = _.arrayGrow( src, [ -1, 3 ] );
    expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'l < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ 0, -1 ] );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ -1, 3 ], 0 );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, l < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ -1, -1 ], 0 );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > l, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow( src, [ 6, 3 ], 0 );
    var expected = [ 1, 2, 3, 4, 5, 0 ];
    test.identical( got, expected );
    test.true( got !== src );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayGrow() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayGrow( [ 1 ], [ 1, 4 ], '5', 1 ) );

  test.case = 'src is not long';
  test.shouldThrowErrorSync( () => _.arrayGrow( 1, [ 0, 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayGrow( new ArrayBuffer( 4 ), [ 0, 5 ] ) );

  test.case = 'not a range';
  test.shouldThrowErrorSync( () => _.arrayGrow( [ 1 ], 'str' ) );

}

//

function arrayGrowInplace( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    /* range = number */

    test.case = 'range = number, number < src length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrowInplace( src, 1 );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrowInplace( src, -5 );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrowInplace( src, 6, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ], [ { a : 1 }, 2, [ 10 ] ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrowInplace( src, -5, [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'only src';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range > src.length, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ 0, src.length + 2 ] );
    var expected = make( [ 1, 2, 3, 4, 5, undefined, undefined ] );
    test.identical( got, expected );
    test.identical( got.length, 7 );
    test.true( got === src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ 0, src.length + 2 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5, 0, 0 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ src.length - 1, src.length * 2 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5, 0, 0, 0, 0, 0 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range < src.length';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ 0, 3 ] );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range < src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ 0, 3 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    got = _.arrayGrowInplace( src, [ -1, 3 ] );
    expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'l < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ 0, -1 ] );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ -1, 3 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, l < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ -1, -1 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f > l, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrowInplace( src, [ 6, 3 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5, 0 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range = number < dst.length';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayGrowInplace( dst, 3 );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number < dst.length, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayGrowInplace( dst, 3, [ 0 ] );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range[ 1 ] > dst.length, src.length ';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = [ 1, 2, 3 ];
    test.shouldThrowErrorSync( () => _.arrayGrowInplace( dst, [ 1, 6 ], src ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayGrowInplace() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayGrowInplace( [ 1 ], [ 1, 4 ], '5', 1 ) );

  test.case = 'src is not long';
  test.shouldThrowErrorSync( () => _.arrayGrowInplace( 1, [ 0, 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayGrowInplace( new ArrayBuffer( 4 ), [ 0, 5 ] ) );

  test.case = 'not a range';
  test.shouldThrowErrorSync( () => _.arrayGrowInplace( [ 1 ], 'str' ) );

}

//

function arrayGrow_( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    test.open( 'not inplace' );

    /* range = number */

    test.case = 'range = number, number < src length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( null, src, 1 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( null, src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( null, src, 6, [ 2 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ], [ 2 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( null, src, -5, [ 2 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'only src';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range > src.length, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ 0, 6 ] );
    var expected = src.length + 2;
    test.identical( got, [ 1, 2, 3, 4, 5, undefined, undefined ] );
    test.identical( got.length, expected );
    test.true( got !== src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ 0, 6 ], 0 );
    var expected = [ 1, 2, 3, 4, 5, 0, 0 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ 4, 9 ], 0 );
    var expected = [ 1, 2, 3, 4, 5, 0, 0, 0, 0, 0 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range < src.length';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ 0, 2 ] );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range < src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ 0, 2 ], 0 );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    got = _.arrayGrow_( null, src, [ -1, 3 ] );
    expected = [ undefined, 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'l < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ 0, -1 ] );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ -1, 3 ], 0 );
    var expected = [ 0, 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, l < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ -1, -1 ], 0 );
    var expected = [ 0, 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > l, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( null, src, [ 6, 3 ], 0 );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.close( 'not inplace' );

    /* - */

    test.open( 'inplace' );

    test.open( 'without dst' );

    test.case = 'range = number, number < src length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, 1 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, 6, [ 2 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ], [ 2 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, -5, [ 2 ] );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'only src';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range > src.length, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ 0, 6 ] );
    var expected = [ 1, 2, 3, 4, 5, undefined, undefined ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ 0, 6 ], 0 );
    var expected = [ 1, 2, 3, 4, 5, 0, 0 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ 3, 9 ], 0 );
    var expected = [ 1, 2, 3, 4, 5, 0, 0, 0, 0, 0 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range < src.length';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ 0, 3 ] );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range < src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ 0, 3 ], 0 );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    got = _.arrayGrow_( src, [ -1, 3 ] );
    expected = [ undefined, 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'l < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ 0, -1 ] );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ -1, 3 ], 0 );
    var expected = [ 0, 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, l < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ -1, -1 ], 0 );
    var expected = [ 0, 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f > l, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, [ 6, 3 ], 0 );
    var expected = [ 1, 2, 3, 4, 5 ];
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range = number < dst.length';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayGrow_( dst, 2 );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number < dst.length, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayGrow_( dst, 2, [ 0 ] );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range[ 1 ] > dst.length, src.length ';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = [ 1, 2, 3 ];
    test.shouldThrowErrorSync( () => _.arrayGrow_( dst, [ 1, 5 ], src ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );

    test.close( 'without dst' );

    /* - */

    test.open( 'dst !== src' );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( [ 1 ], src, 6, [ 2 ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ], [ 2 ] ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( [ 1 ], src, -5, [ 2 ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( [ 1 ], src, [ 0, 6 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5, 0, 0 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( [ 1 ], src, [ 3, 9 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5, 0, 0, 0, 0, 0 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range < src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( [ 1 ], src, [ 0, 3 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( [ 1 ], src, [ -1, 3 ], 0 );
    var expected = make( [ 0, 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f < 0, l < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( [ 1 ], src, [ -1, -1 ], 0 );
    var expected = make( [ 0, 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'f > l, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( [ 1 ], src, [ 6, 3 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'container is not extensible, dst.length === src.length';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [ 1, 2, 3, 4 ] );
    var got = _.arrayGrow_( dst, src, 2, undefined );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number < dst.length, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [ 1, 2, 3, 4 ] );
    var got = _.arrayGrow_( dst, src, 2, [ 0 ] );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range[ 1 ] > dst.length, src.length ';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [ 1, 2, 3, 4 ] );
    test.shouldThrowErrorSync( () => _.arrayGrow_( dst, src, [ 1, 6 ], [ 1, 2, 3 ] ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );


    test.close( 'dst !== src' );

    /* range = number */

    test.open( 'dst === src' );

    test.case = 'range = number, number < src length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, src, 1 );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, src, -5 );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, src, 6, [ 2 ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ], [ 2 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayGrow_( src, src, -5, [ 2 ] );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'only src';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range > src.length, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ 0, 6 ] );
    var expected = make( [ 1, 2, 3, 4, 5, undefined, undefined ] );
    test.identical( got, expected );
    test.identical( got.length, 7 );
    test.true( got === src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ 0, 6 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5, 0, 0 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range > src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ 3, 9 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5, 0, 0, 0, 0, 0 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range < src.length';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ 0, 3 ] );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range < src.length, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ 0, 3 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    got = _.arrayGrow_( src, src, [ -1, 3 ] );
    expected = make( [ undefined, 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'l < 0, not a ins';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ 0, -1 ] );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ -1, 3 ], 0 );
    var expected = make( [ 0, 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f < 0, l < 0, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ -1, -1 ], 0 );
    var expected = make( [ 0, 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'f > l, ins = number';
    var src = make( [ 1, 2, 3, 4, 5 ] );
    var got = _.arrayGrow_( src, src, [ 6, 3 ], 0 );
    var expected = make( [ 1, 2, 3, 4, 5 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range - number < src.length';
    var src = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( src );
    var got = _.arrayGrow_( src, src, 3 );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'container is not extensible, range = number < src.length, ins.length = 1';
    var src = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( src );
    var got = _.arrayGrow_( src, src, 3, [ 0 ] );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'container is not extensible, range[ 1 ] > src.length, ins.length ';
    var src = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( src );
    test.shouldThrowErrorSync( () => _.arrayGrow_( src, src, [ 1, 6 ], [ 1, 2, 3 ] ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( src, expected );

    test.close( 'dst === src' );

    test.close( 'inplace' );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayGrow_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayGrow_( [ 1 ], [ 1, 4 ], '5', 1 ) );

  test.case = 'src is not long';
  test.shouldThrowErrorSync( () => _.arrayGrow_( 1, [ 0, 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayGrow_( new ArrayBuffer( 4 ), [ 0, 5 ] ) );

  test.case = 'not a range';
  test.shouldThrowErrorSync( () => _.arrayGrow_( [ 1 ], 'str' ) );

}

arrayGrow_.timeOut = 20000;

//

function arrayGrow_CheckReturnedContainer( test )
{
  test.case = 'dst - undefined, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayGrow_( src, [ -1, 3 ], 0 );
  var expected = [ 0, 1, 2, 3, 0 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - null, new container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayGrow_( null, src, [ -1, 3 ], 0 );
  var expected = [ 0, 1, 2, 3, 0 ];
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'dst - src, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayGrow_( src, src, [ -1, 3 ], 0 );
  var expected = [ 0, 1, 2, 3, 0 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - another container, dst container';
  var src = [ 1, 2, 3 ];
  var dst = [];
  var got = _.arrayGrow_( dst, src, [ -1, 3 ], 0 );
  var expected = [ 0, 1, 2, 3, 0 ];
  test.identical( got, expected );
  test.true( got === dst );
}

//

function arrayRelength( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, 1 );
    var expected = [ 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, -5 );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, 6, 'abc' );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, -5, 'abc' );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 0, 2 ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ -5, 2 ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 0, -5 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 0, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ -5, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [ 1, 2 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] = 0, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 0, -5 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 8, 0 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 3, 1 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 3, 6 ] );
    var expected = [ 'str', [ 1 ], undefined ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength( src, [ 3, 7 ], 7 );
    var expected = [ 'str', [ 1 ], 7, 7 ];
    test.identical( got, expected );
    test.true( got !== src );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayRelength() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayRelength( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayRelength( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayRelength( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );

}

//

function arrayRelengthInplace( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, 1 );
    var expected = make( [ 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, -5 );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, 6, 'abc' );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, -5, 'abc' );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 0, 2 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ -5, 2 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 0, -5 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 0, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ -5, 2 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] = 0, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 0, -5 ], [ { a : 1 }, 2, [ 10 ] ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 8, 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 3, 1 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 3, 6 ] );
    var expected = make( [ 'str', [ 1 ], undefined ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelengthInplace( src, [ 3, 7 ], 7 );
    var expected = make( [ 'str', [ 1 ], 7, 7 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range = number, not src';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayRelengthInplace( dst, 2 );
    var expected = make( [ 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayRelengthInplace( dst, 0, [ 0 ] );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range, src.length > range[ 1 ] - range[ 0 ]';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = [ 1, 2, 3 ];
    test.shouldThrowErrorSync( () => _.arrayRelengthInplace( dst, [ 1, 8 ], src ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelengthInplace( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );

}

//

function arrayRelength_( test )
{
  var array = ( src ) => _.array.make( src );
  var unroll = ( src ) => _.unroll.make( src );

  /* - */

  test.open( 'array' );
  run( array );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( unroll );
  test.close( 'unroll' );

  /* - */

  function run( make )
  {
    test.open( 'not inplace' );

    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, 1 );
    var expected = [ 1 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, -5 );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, 5, 'abc' );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, -5, 'abc' );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 0, 2 ] );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ -2, 2 ] );
    var expected = [ undefined, undefined, 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 0, -5 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 0, 2 ], [ 0 ] );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ -2, 2 ], [ 0 ] );
    var expected = [ [ 0 ], [ 0 ], 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] = 0, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 0, -5 ], [ 0 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 8, 0 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 3, 1 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 3, 5 ] );
    var expected = [ 'str', [ 1 ], undefined ];
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( null, src, [ 3, 6 ], 7 );
    var expected = [ 'str', [ 1 ], 7, 7 ];
    test.identical( got, expected );
    test.true( got !== src );

    test.close( 'not inplace' );

    /* - */

    test.open( 'inplace' );

    test.open( 'without dst' );

    /* range = number */

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, 1 );
    var expected = [ 1 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, -5 );
    var expected = [];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, 5, 'abc' );
    var expected = [ 1, 2, 3, 'str', [ 1 ] ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, -5, 'abc' );
    var expected = [];
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 0, 2 ] );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ -2, 2 ] );
    var expected = [ undefined, undefined, 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 0, -5 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 0, 2 ], [ 0 ] );
    var expected = [ 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ -2, 2 ], [ 0 ] );
    var expected = [ [ 0 ], [ 0 ], 1, 2, 3 ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] = 0, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 0, -5 ], [ 0 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 8, 0 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 3, 1 ] );
    var expected = [];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 3, 5 ] );
    var expected = [ 'str', [ 1 ], undefined ];
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, [ 3, 6 ], 7 );
    var expected = [ 'str', [ 1 ], 7, 7 ];
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range = number, not src';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayRelength_( dst, 2 );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var got = _.arrayRelength_( dst, 0, [ 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range, src.length > range[ 1 ] - range[ 0 ]';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    test.shouldThrowErrorSync( () => _.arrayRelength_( dst, [ 1, 8 ], [ 0 ] ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );

    test.close( 'without dst' );

    /* - */

    test.open( 'dst !== src' );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( [ 1 ], src, 5, 'abc' );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( [ 1 ], src, -5, 'abc' );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    /* range = array range */

    test.case = 'range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( [ 1 ], src, [ 0, 2 ], [ 0 ] );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( [ 1 ], src, [ -2, 2 ], [ 0 ] );
    var expected = make( [ [ 0 ], [ 0 ], 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] = 0, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( [ 1 ], src, [ 0, -5 ], [ 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got !== src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( [ 1 ], src, [ 3, 6 ], 7 );
    var expected = make( [ 'str', [ 1 ], 7, 7 ] );
    test.identical( got, expected );
    test.true( got !== src );

    /* */

    test.case = 'container is not extensible, range = number, not src';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [ 1, 2, 3, 4 ] );
    var got = _.arrayRelength_( dst, src, 2, [ 0 ] );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range = number, src.length = 1';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [ 1, 2, 3, 4 ] );
    var got = _.arrayRelength_( dst, src, 0, [ 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === dst );

    test.case = 'container is not extensible, range, src.length > range[ 1 ] - range[ 0 ]';
    var dst = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( dst );
    var src = make( [ 1, 2, 3, 4 ] );
    test.shouldThrowErrorSync( () => _.arrayRelength_( dst, src, [ 1, 8 ], [ 1, 2, 3 ] ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( dst, expected );

    test.close( 'dst !== src' );


    /* range = number */

    test.open( 'dst === src' );

    test.case = 'range = number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, 1 );
    var expected = make( [ 1 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, -5 );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = number, range > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, 5, 'abc' );
    var expected = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = negative number, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, -5, 'abc' );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    /* range = array range */

    test.case = 'range = array range, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 0, 2 ] );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 0 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ -2, 2 ] );
    var expected = make( [ undefined, undefined, 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range = array range, range[ 1 ] < 0, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 0, -5 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 0, 2 ], [ 0 ] );
    var expected = make( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] < 0, range[ 1 ] < src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ -2, 2 ], [ 0 ] );
    var expected = make( [ [ 0 ], [ 0 ], 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] = 0, range[ 1 ] < 0, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 0, -5 ], [ 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] > src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 8, 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > range[ 1 ], range[ 0 ] < src.length';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 3, 1 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, not ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 3, 5 ] );
    var expected = make( [ 'str', [ 1 ], undefined ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'range[ 0 ] > 0, [ 1 ] > src.length, ins';
    var src = make( [ 1, 2, 3, 'str', [ 1 ] ] );
    var got = _.arrayRelength_( src, src, [ 3, 6 ], 7 );
    var expected = make( [ 'str', [ 1 ], 7, 7 ] );
    test.identical( got, expected );
    test.true( got === src );

    /* */

    test.case = 'container is not extensible, range = number, not src';
    var src = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( src );
    var got = _.arrayRelength_( src, src, 2 );
    var expected = make( [ 1, 2 ] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'container is not extensible, range = number, src.length = 1';
    var src = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( src );
    var got = _.arrayRelength_( src, src, 0, [ 0 ] );
    var expected = make( [] );
    test.identical( got, expected );
    test.true( got === src );

    test.case = 'container is not extensible, range, src.length > range[ 1 ] - range[ 0 ]';
    var src = make( [ 1, 2, 3, 4 ] );
    Object.preventExtensions( src );
    test.shouldThrowErrorSync( () => _.arrayRelength_( src, src, [ 1, 8 ], [ 1, 2, 3 ] ) );
    var expected = make( [ 1, 2, 3, 4 ] );
    test.identical( src, expected );

    test.close( 'dst === src' );

    test.close( 'inplace' );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayRelength_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayRelength_( [ 1, 2 ], 0, [ 4 ], 4 ) );

  test.case = 'src is not array';
  test.shouldThrowErrorSync( () => _.arrayRelength_( 'str', 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength_( { a : 1 }, 0, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength_( new Fx(), 0, [ 4 ] ) );

  test.case = 'not range';
  test.shouldThrowErrorSync( () => _.arrayRelength_( [ 1, 2 ], 'str', [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength_( [ 1, 2 ], [], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength_( [ 1, 2 ], [ 'str' ], [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.arrayRelength_( [ 1, 2 ], [ 1, 2, 3 ], [ 4 ] ) );
}

arrayRelength_.timeOut = 10000;

//

function arrayRelength_CheckReturnedContainer( test )
{
  test.case = 'dst - undefined, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayRelength_( src, [ -1, 1 ], 0 );
  var expected = [ 0, 1, 2 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - null, new container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayRelength_( null, src, [ -1, 1 ], 0 );
  var expected = [ 0, 1, 2 ];
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'dst - src, same container';
  var src = [ 1, 2, 3 ];
  var got = _.arrayRelength_( src, src, [ -1, 1 ], 0 );
  var expected = [ 0, 1, 2 ];
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'dst - another container, dst container';
  var src = [ 1, 2, 3 ];
  var dst = [];
  var got = _.arrayRelength_( dst, src, [ -1, 1 ], 0 );
  var expected = [ 0, 1, 2 ];
  test.identical( got, expected );
  test.true( got === dst );
}

// ---
// array transformation
// ---

function arrayPrepend( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrepend( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'dstArray is empty';

  var dst = [];
  var got = _.arrayPrepend( dst, null );
  test.identical( got, [ null ] );
  test.true( got === dst );

  var dst = [];
  var got = _.arrayPrepend( dst, undefined );
  test.identical( got, [ undefined ] );
  test.true( got === dst );

  var dst = [];
  var got = _.arrayPrepend( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [];
  var got = _.arrayPrepend( dst, '1' );
  test.identical( got, [ '1' ] );
  test.true( got === dst );

  var dst = [];
  var got = _.arrayPrepend( dst, [ 1, 2 ] );
  test.identical( got, [ [ 1, 2 ] ] );
  test.true( got === dst );

  test.case = 'simple';

  var dst = [ 1 ];
  var got = _.arrayPrepend( dst, 1 );
  test.identical( got, [ 1, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrepend( dst, 2 );
  test.identical( got, [ 2, 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrepend( dst, 3 );
  test.identical( got, [ 3, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrepend( dst, '1' );
  test.identical( got, [ '1', 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrepend( dst, undefined );
  test.identical( got, [ undefined, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrepend( dst, -1 );
  test.identical( got, [ -1, 1 ] );
  test.true( got === dst );

  test.case = 'Array prepended as an element';

  var dst = [ 1 ];
  var got = _.arrayPrepend( dst, [ 1 ] );
  test.identical( got, [ [ 1 ], 1 ] );
  test.true( got === dst );

  var dst = [ 'Choose an option' ];
  var got = _.arrayPrepend( dst, [ 1, 0, - 1 ] );
  test.identical( got, [ [ 1, 0, -1 ], 'Choose an option' ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrepend();
  })

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrepend( [], 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrepend( 1, 1 );
  })
}

//

function arrayPrependOnce( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependOnce( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependOnce( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnce( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnce( dst, 2 );
  test.identical( got, [ 2, 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependOnce( dst, 3 );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnce( dst, '1' );
  test.identical( got, [ '1', 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnce( dst, -1 );
  test.identical( got, [ -1, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnce( dst, [ 1 ] );
  test.identical( got, [ [ 1 ], 1 ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependOnce( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependOnce( dst, { num : 1 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayPrependOnce( dst, 4, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayPrependOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependOnce();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependOnce( 1, 1, 1 );
  })

}

//

function arrayPrependOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependOnceStrictly( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependOnceStrictly( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnceStrictly( dst, 2 );
  test.identical( got, [ 2, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnceStrictly( dst, '1' );
  test.identical( got, [ '1', 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnceStrictly( dst, -1 );
  test.identical( got, [ -1, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependOnceStrictly( dst, [ 1 ] );
  test.identical( got, [ [ 1 ], 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependOnceStrictly( dst, 0 );
  test.identical( got, [ 0, 1, 2, 2, 3, 3 ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayPrependOnceStrictly( dst, 4, onEqualize );
  test.identical( got, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependOnceStrictly( 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  // test.case = 'onEqualize is not a routine';

  // test.shouldThrowErrorSync( function()
  // {
  //    _.arrayPrependOnceStrictly( [ 1, 2, 3 ], 3, 3 );
  // });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayPrependOnceStrictly( dst, { num : 1 }, onEqualize );
  });

}

//

function arrayPrepended( test )
{

  test.case = 'dstArray is empty';

  var dst = [];
  var got = _.arrayPrepended( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [];
  var got = _.arrayPrepended( dst, null );
  test.identical( dst, [ null ] );
  test.identical( got, 0 );

  var dst = [];
  var got = _.arrayPrepended( dst, undefined );
  test.identical( dst, [ undefined ] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [ 1 ];
  var got = _.arrayPrepended( dst, 1 );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrepended( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrepended( dst, 3 );
  test.identical( dst, [ 3, 1, 2, 3 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrepended( dst, '1' );
  test.identical( dst, [ '1', 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrepended( dst, -1 );
  test.identical( dst, [ -1, 1 ] );
  test.identical( got, 0 );

  test.case = 'Array prepended as an element';

  var dst = [ 1 ];
  var got = _.arrayPrepended( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1 ] );
  test.identical( got, 0 );

  var dst = [ 'Choose an option' ];
  var got = _.arrayPrepended( dst, [ 1, 0, - 1 ] );
  test.identical( dst, [ [ 1, 0, -1 ], 'Choose an option' ] );
  test.identical( got, 0 );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrepended();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrepended( [], 1, 1 );
  });

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrepended( 1, 1 );
  });
}

//

function arrayPrependedOnce( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnce( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedOnce( dst, 3 );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnce( dst, '1' );
  test.identical( dst, [ '1', 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnce( dst, -1 );
  test.identical( dst, [ -1, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnce( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1 ] );
  test.identical( got, 0 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependedOnce( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependedOnce( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayPrependedOnce( dst, 4, onEqualize );
  test.identical( dst, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayPrependedOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedOnce();
  })

  test.case = 'fourth is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedOnce( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedOnce( 1, 1 );
  })
}

//

function arrayPrependedOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedOnceStrictly( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnceStrictly( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnceStrictly( dst, '1' );
  test.identical( dst, [ '1', 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnceStrictly( dst, -1 );
  test.identical( dst, [ -1, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayPrependedOnceStrictly( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1 ] );
  test.identical( got, 0 );

  var dst = [ 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedOnceStrictly( dst, 0 );
  test.identical( dst, [ 0, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependedOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayPrependedOnceStrictly( dst, 4, onEqualize );
  test.identical( dst, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedOnceStrictly( 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayPrependedOnceStrictly( dst, { num : 1 }, onEqualize );
  });

}

//

function arrayPrependElement( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependElement( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependElement( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElement( dst, 1 );
  test.identical( got, [ 1, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElement( dst, 2 );
  test.identical( got, [ 2, 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependElement( dst, 3 );
  test.identical( got, [ 3, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElement( dst, '1' );
  test.identical( got, [ '1', 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElement( dst, -1 );
  test.identical( got, [ -1, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElement( dst, [ 1 ] );
  test.identical( got, [ [ 1 ], 1 ] );
  test.true( got === dst );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElement();
  })

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElement( [], 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElement( 1, 1 );
  })
}

//

function arrayPrependElementOnce( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependElementOnce( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependElementOnce( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnce( dst, 2 );
  test.identical( got, [ 2, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnce( dst, '1' );
  test.identical( got, [ '1', 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnce( dst, -1 );
  test.identical( got, [ -1, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnce( dst, [ 1 ] );
  test.identical( got, [ [ 1 ], 1 ] );
  test.true( got === dst );

  test.case = 'ins already in srcArray';

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnce( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependElementOnce( dst, 3 );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ true, false, true ];
  var got = _.arrayPrependElementOnce( dst, false );
  test.identical( got, [ true, false, true ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependElementOnce( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependElementOnce( dst, { num : 1 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayPrependElementOnce( dst, 4, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayPrependElementOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElementOnce();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElementOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElementOnce( 1, 1, 1 );
  })

}

//

function arrayPrependElementOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependElementOnceStrictly( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'dstArray is null';
  var got = _.arrayPrependElementOnceStrictly( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependElementOnceStrictly( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnceStrictly( dst, 2 );
  test.identical( got, [ 2, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnceStrictly( dst, '1' );
  test.identical( got, [ '1', 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnceStrictly( dst, -1 );
  test.identical( got, [ -1, 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayPrependElementOnceStrictly( dst, [ 1 ] );
  test.identical( got, [ [ 1 ], 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependElementOnceStrictly( dst, 0 );
  test.identical( got, [ 0, 1, 2, 2, 3, 3 ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependElementOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayPrependElementOnceStrictly( dst, 4, onEqualize );
  test.identical( got, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElementOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElementOnceStrictly( 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElementOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependElementOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayPrependElementOnceStrictly( dst, { num : 1 }, onEqualize );
  });

}

//

function arrayPrependedElement( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedElement( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElement( dst, 1 );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElement( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedElement( dst, 3 );
  test.identical( dst, [ 3, 1, 2, 3 ] );
  test.identical( got, 3 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElement( dst, '1' );
  test.identical( dst, [ '1', 1 ] );
  test.identical( got, '1' );

  var dst = [ 1 ];
  var got = _.arrayPrependedElement( dst, -1 );
  test.identical( dst, [ -1, 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElement( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1 ] );
  test.identical( got, [ 1 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElement();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElement( [], 1, 1 );
  });

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElement( 1, 1 );
  });
}

//

function arrayPrependedElementOnce( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedElementOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnce( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 2 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnce( dst, '1' );
  test.identical( dst, [ '1', 1 ] );
  test.identical( got, '1' );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnce( dst, -1 );
  test.identical( dst, [ -1, 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnce( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'ins already in dstArray';

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, undefined );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedElementOnce( dst, 3 );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, undefined );

  var dst = [ false, true, false, true ];
  var got = _.arrayPrependedElementOnce( dst, true );
  test.identical( dst, [ false, true, false, true ] );
  test.identical( got, undefined );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependedElementOnce( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, { num : 4 } );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependedElementOnce( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, undefined );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayPrependedElementOnce( dst, 4, onEqualize );
  test.identical( dst, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 4 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayPrependedElementOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, undefined );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElementOnce();
  })

  test.case = 'fourth is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElementOnce( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElementOnce( 1, 1 );
  })
}

//

function arrayPrependedElementOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedElementOnceStrictly( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnceStrictly( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 2 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnceStrictly( dst, '1' );
  test.identical( dst, [ '1', 1 ] );
  test.identical( got, '1' );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnceStrictly( dst, -1 );
  test.identical( dst, [ -1, 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayPrependedElementOnceStrictly( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1 ] );
  test.identical( got, [ 1 ] );

  var dst = [ 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedElementOnceStrictly( dst, 0 );
  test.identical( dst, [ 0, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayPrependedElementOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 4 }, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, { num : 4 } );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayPrependedElementOnceStrictly( dst, 4, onEqualize );
  test.identical( dst, [ 4, { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 4 );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElementOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElementOnceStrictly( 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElementOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedElementOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayPrependedElementOnceStrictly( dst, { num : 1 }, onEqualize );
  });

}

//

function arrayPrependArray( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependArray( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';
  var got = _.arrayPrependArray( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayPrependArray( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArray( dst, [ 4, 5 ] );
  test.identical( dst, [ 4, 5, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependArray( dst, [ 1, 1 ] );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependArray( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 }, 1 ] );
  test.true( got === dst );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArray( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayPrependArray( dst, [ undefined, 2 ] );
  test.identical( dst, [ undefined, 2, 1 ] );
  test.true( got === dst );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArray( dst, dst );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArray();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArray( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArray( [ 1, 2 ], 2 );
  });
};

//

function arrayPrependArrayOnce( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependArrayOnce( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';

  var got = _.arrayPrependArrayOnce( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependArrayOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrayOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 4, 5, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependArrayOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependArrayOnce( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 'a', [ { a : 1 } ], { b : 2 }, 1 ] );
  test.true( got === dst );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArrayOnce( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ undefined, 2, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependArrayOnce( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrayOnce( dst, dst );
  test.identical( got, [ 1, 2, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArrayOnce( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArrayOnce( dst, dst, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArrayOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArrayOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArrayOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrayOnce();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrayOnce( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrayOnce( [ 1, 2 ], 2 );
  });

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayPrependArrayOnce( [ 1, 2 ], [ 2 ], 3 );
  // });
}

//

function arrayPrependArrayOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependArrayOnceStrictly( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';

  var got = _.arrayPrependArrayOnceStrictly( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependArrayOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrayOnceStrictly( dst, [ 4, 5 ] );
  test.identical( dst, [ 4, 5, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayPrependArrayOnceStrictly( dst, [ 4, 5 ] );
  test.identical( dst, [ 4, 5, 1, 2, 3, 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArrayOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ undefined, 2, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependArrayOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArrayOnceStrictly( dst, dst ) );
  else
  _.arrayPrependArrayOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArrayOnceStrictly( dst, dst ) );
  else
  _.arrayPrependArrayOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArrayOnceStrictly( dst, dst, ( e ) => e ) );
  else
  _.arrayPrependArrayOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  _.arrayPrependArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrayOnceStrictly();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrayOnceStrictly( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrayOnceStrictly( [ 1, 2 ], 2 );
  });

  test.case = 'one of elements is not unique';

  var dst = [ 1, 2, 3 ];
  var onErrorCallback = ( err, arg ) => test.identical( dst, [ 4, 5, 1, 2, 3 ] );
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArrayOnceStrictly( dst, [ 4, 5, 2 ] );
  }, onErrorCallback );

  var dst = [ 1, 1, 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArrayOnceStrictly( dst, [ 1 ] );
  })
  test.identical( dst, [ 1, 1, 1 ] );

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayPrependArrayOnceStrictly( [ 1, 2 ], [ 2 ], 3 );
  // });
}

//

function arrayPrependedArray( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayPrependedArray( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedArray( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArray( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 2, 4, 5, 1, 2, 3 ] );
  test.identical( got, 3 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependedArray( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependedArray( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 }, 1 ] );
  test.identical( got, 4 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependedArray( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ undefined, 2, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependedArray( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArray( dst, dst );
  test.identical( got, 3 );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArray();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArray( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArray( [ 1, 2 ], 2 );
  });

}

//

function arrayPrependedArrayOnce( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayPrependedArrayOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedArrayOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArrayOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 4, 5, 1, 2, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependedArrayOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 0 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependedArrayOnce( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 'a', [ { a : 1 } ], { b : 2 }, 1 ] );
  test.identical( got, 3 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependedArrayOnce( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ undefined, 2, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependedArrayOnce( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrayOnce( dst, dst );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrayOnce( dst, dst, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrayOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrayOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrayOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 6 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnce();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnce( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnce( [ 1, 2 ], 2 );
  });

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayPrependedArrayOnce( [ 1, 2 ], [ 2 ], 3 );
  // });
}

//

function arrayPrependedArrayOnceStrictly( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayPrependedArrayOnceStrictly( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedArrayOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'prepends only if all elements are unique';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArrayOnceStrictly( dst, [ 3.5, 4, 5 ] );
  test.identical( dst, [ 3.5, 4, 5, 1, 2, 3 ] );
  test.identical( got, 3 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependedArrayOnceStrictly( dst, [ 0 ] );
  test.identical( dst, [ 0, 1, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependedArrayOnceStrictly( dst, [ 'a', 0, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 'a', 0, [ { a : 1 } ], { b : 2 }, 1 ] );
  test.identical( got, 4 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependedArrayOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ undefined, 2, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependedArrayOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArrayOnceStrictly( dst, dst ) );
  else
  {
    var got1 = _.arrayPrependedArrayOnceStrictly( dst, dst );
    test.identical( got1, 0 );
  }
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArrayOnceStrictly( dst, dst ) );
  else
  {
    var got2 = _.arrayPrependArrayOnceStrictly( dst, dst );
    test.identical( got2, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArrayOnceStrictly( dst, dst, ( e ) => e ) );
  else
  {
    var got3 = _.arrayPrependedArrayOnceStrictly( dst, dst, ( e ) => e )
    test.identical( got3, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  {
    var got4 = _.arrayPrependArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
    test.identical( got4, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnceStrictly();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnceStrictly( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnceStrictly( [ 1, 2 ], 2 );
  });

  test.case = 'One of args is not unique';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnceStrictly( [ 1, 1, 1 ], [ 1 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrayOnceStrictly( [ 1, 2, 3 ], [ 2, 4, 5 ] );
  });

}

// --
//arrayPrependElement*Arrays*
// --

function arrayPrependArrays( test )
{

  test.case = 'dstArray is null';
  var got = _.arrayPrependArrays( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';
  var got = _.arrayPrependArrays( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayPrependArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrays( dst, [ 4, 5 ] );
  test.identical( dst, [ 5, 4, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependArrays( dst, [ 1, 1 ] );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.true( got === dst );

  var dst = [];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4 ] ] ];
  var got = _.arrayPrependArrays( dst, insArray );
  test.identical( dst, [ 3, [ 4 ], 2, 1 ] );
  test.true( got === dst );

  var dst = [];
  var insArray = [ 1, 2, 3 ]
  var got = _.arrayPrependArrays( dst, insArray );
  test.identical( dst, [ 3, 2, 1 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ 'a', 1, [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayPrependArrays( dst, insArray );
  test.identical( dst, [ { b : 2 }, { a : 1 }, 1, 'a', 1 ] );
  test.true( got === dst );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got;
  var got = _.arrayPrependArrays( dst, [ undefined, 2 ] );
  test.identical( dst, [ 2, undefined, 1 ] );
  test.true( got === dst );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrays( dst, dst );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrays( dst, [ dst ] );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrays( dst, [ [ 4, 5, 6 ], [ 7, 8, 9 ] ] );
  test.identical( dst, [ 7, 8, 9, 4, 5, 6, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArrays( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ 6, 5, 4, 1, 2, 3 ] );
  test.true( got === dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArrays( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrays();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrays( 1, [ 2 ] );
  });

  test.case = 'second arg is no a ArrayLike';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrays( [], 2 );
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArrays( [], [ 1 ], [ 2 ] );
  });

}

//

function arrayPrependArraysOnce( test )
{

  test.case = 'dstArray is null';
  var got = _.arrayPrependArraysOnce( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';

  var got = _.arrayPrependArraysOnce( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.true( got === dst );

  test.case = 'should keep sequence';

  var dst = [ 6 ];
  var src = [ [ 1, 2 ], 3, [ 6, 4, 5, 1, 2, 3 ] ];
  var srcCopy = [ [ 1, 2 ], 3, [ 6, 4, 5, 1, 2, 3 ] ];
  var got = _.arrayPrependArraysOnce( dst, src );
  test.identical( dst, [ 4, 5, 3, 1, 2, 6 ] );
  test.identical( src, srcCopy );
  test.true( got === dst );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 5, 4, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependArraysOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependArraysOnce( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ { 'b' : 2 }, { 'a' : 1 }, 'a', 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4 ], 5 ] ];
  var got = _.arrayPrependArraysOnce( dst, insArray );
  test.identical( dst, [ [ 4 ], 5, 1, 2, 3, 4 ] );
  test.true( got === dst );

  var dst = [ 1, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 2, 1, 3 ] );
  test.identical( dst, got );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ [ 4, 5, 6 ], [ 7, 8, 9 ] ] );
  test.identical( dst, [ 7, 8, 9, 4, 5, 6, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ [ 4, 5, 6 ], [ 1, 2, 9 ] ] );
  test.identical( dst, [ 9, 4, 5, 6, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ 6, 5, 4, 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }

  var dst = [ 1, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ 1, 2, 3 ], onEqualize );
  test.identical( got, [ 2, 1, 3 ] );
  test.identical( dst, got );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArraysOnce( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 2, undefined, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependArraysOnce( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArraysOnce( dst, dst );
  test.identical( got, [ 1, 2, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ dst ] );
  test.identical( got, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, dst, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnce( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnce();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnce( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnce( [], [ 1, 2, 3 ], {} );
  });

  test.case = 'second arg is no a ArrayLike';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnce( [], 2 );
  });

}

//

function arrayPrependArraysOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayPrependArraysOnceStrictly( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';

  var got = _.arrayPrependArraysOnceStrictly( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.true( got === dst );

  test.case = 'should keep sequence';

  var dst = [ 6 ];
  var src = [ [ 1, 2 ], 3, [ 4, 5 ] ];
  var srcCopy = [ [ 1, 2 ], 3, [ 4, 5 ] ];
  var got = _.arrayPrependArraysOnceStrictly( dst, src );
  test.identical( dst, [ 4, 5, 3, 1, 2, 6 ] );
  test.identical( src, srcCopy );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ [ 'a' ], [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayPrependArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ { b : 2 }, { a : 1 }, 'a', 1 ] );
  test.true( got === dst );

  var dst = [ 0 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, [ 5 ] ] ] ];
  var got = _.arrayPrependArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ 3, [ 4, [ 5 ] ], 2, 1, 0 ] );
  test.true( got === dst );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArraysOnceStrictly( dst, [ [ 4, 5, 6 ], [ 7, 8, 9 ] ] );
  test.identical( dst, [ 7, 8, 9, 4, 5, 6, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependArraysOnceStrictly( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ 6, 5, 4, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 4, 5 ];
  var got = _.arrayPrependArraysOnceStrictly( dst, [ 1, 2, 3 ], onEqualize )
  test.identical( got, [ 3, 2, 1, 4, 5 ] );
  test.identical( dst, got );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 2, undefined, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependArraysOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, dst ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, dst ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, dst, ( e ) => e ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, [ dst ], ( e ) => e ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e ) );
  else
  _.arrayPrependArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependArraysOnceStrictly( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnceStrictly();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnceStrictly( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnceStrictly( [], [ 1, 2, 3 ], {} );
  });

  test.case = 'Same element in insArray and in dstArray';
  var dst = [ 1, 2, 3 ];
  var onErrorCallback = ( err, arg ) => test.identical( dst, [ 5, 4, 1, 2, 3 ] );
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArraysOnceStrictly( dst, [ 4, 2, 5 ] );
  }, onErrorCallback );

  var dst = [ 1, 1, 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArraysOnceStrictly( dst, [ 1 ] );
  })
  test.identical( dst, [ 1, 1, 1 ] );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependArraysOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'second arg is no a ArrayLike';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependArraysOnceStrictly( [], 2 );
  });
}

//

function arrayPrependedArrays( test )
{
  test.case = 'nothing';
  var dst = [];
  var got = _.arrayPrependedArrays( dst, [] );
  var expected = [];
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayPrependedArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArrays( dst, [ 4, 5 ] );
  test.identical( dst, [ 5, 4, 1, 2, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependedArrays( dst, [ 1, 1 ] );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  var dst = [];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4 ], 5 ] ];
  var got = _.arrayPrependedArrays( dst, insArray );
  test.identical( dst, [ 3, [ 4 ], 5, 2, 1 ] );
  test.identical( got, 5 );

  var dst = [];
  var got = _.arrayPrependedArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.identical( got, 3 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependedArrays( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ { b : 2 }, { a : 1 }, 1, 'a', 1 ] );
  test.identical( got, 4 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependedArrays( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got;
  var got = _.arrayPrependedArrays( dst, [ undefined, 2 ] );
  test.identical( dst, [ 2, undefined, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst === ins';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrays( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === ins';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArrays( dst, [ dst ] );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === ins';
  var dst = [ 1, 1, 2, 2 ];
  var got = _.arrayPrependedArrays( dst, [ dst, dst ] );
  test.identical( dst, [ 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2 ] );
  test.identical( got, 12);

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArrays( dst, [ [ 4, 5, 6 ], [ 7, 8, 9 ] ] );
  test.identical( dst, [ 7, 8, 9, 4, 5, 6, 1, 2, 3 ] );
  test.identical( got, 6 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArrays( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ 6, 5, 4, 1, 2, 3 ] );
  test.identical( got, 3 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrays();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrays( 1, [ 2 ] );
  });

  test.case = 'second arg is no a ArrayLike';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrays( [], 2 );
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArrays( [], [ 1 ], [ 2 ] );
  });

}

//

function arrayPrependedArraysOnce( test )
{

  test.case = 'nothing';

  var dst = [];
  var got = _.arrayPrependedArraysOnce( dst, [] );
  var expected = [];
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.identical( got, 3 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.identical( got, 3 );

  test.case = 'should keep sequence';

  var dst = [ 6 ];
  var src = [ [ 1, 2 ], 3, [ 6, 4, 5, 1, 2, 3 ] ];
  var srcCopy = [ [ 1, 2 ], 3, [ 6, 4, 5, 1, 2, 3 ] ];
  var got = _.arrayPrependedArraysOnce( dst, src );
  test.identical( dst, [ 4, 5, 3, 1, 2, 6 ] );
  test.identical( src, srcCopy );
  test.identical( got, 5 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 5, 4, 1, 2, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayPrependedArraysOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 0 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependedArraysOnce( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ { 'b' : 2 }, { 'a' : 1 }, 'a', 1 ] );
  test.identical( got, 3 );

  var dst = [];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, [ 5 ] ] ] ];
  var got = _.arrayPrependedArraysOnce( dst, insArray );
  test.identical( dst, [ 3, [ 4, [ 5 ] ], 2, 1 ] );
  test.identical( got, 4 );

  var dst = [ 1, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 2, 1, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, [ [ 4, 5, 6 ], [ 7, 8, 9 ] ] );
  test.identical( dst, [ 7, 8, 9, 4, 5, 6, 1, 2, 3 ] );
  test.identical( got, 6 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ 6, 5, 4, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }
  var dst = [ 1, 3 ];
  var insArray = [ 1, 2, 3 ]
  var got = _.arrayPrependedArraysOnce( dst, insArray, onEqualize );
  test.identical( dst, [ 2, 1, 3 ] );
  test.identical( got, 1 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependedArraysOnce( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 2, undefined, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependedArraysOnce( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, [ dst ] );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, dst, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnce( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 18 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnce();
  });

  // test.case = 'dst is not a array';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayPrependedArraysOnce( 1, [ 2 ] );
  // });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnce( [], [ 1, 2, 3 ], [] )
  });

  test.case = 'second arg is not a ArrayLike entity';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnce( [ 1 ], 2 );
  });

}

//

function arrayPrependedArraysOnceStrictly( test )
{

  test.case = 'nothing';

  var dst = [];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [] );
  var expected = [];
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.identical( got, 3 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1 ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ 6, 5, 4, 1, 2, 3 ] );
  test.identical( got, 3 );

  var dst = [ 0, 0, 0 ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ 1 ] );
  test.identical( dst, [ 1, 0, 0, 0 ] );
  test.identical( got, 1 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ 'a', 0, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ { 'b' : 2 }, { 'a' : 1 }, 0, 'a', 1 ] );
  test.identical( got, 4 );

  var dst = [];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, [ 5 ] ] ] ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ 3, [ 4, [ 5 ] ], 2, 1 ] );
  test.identical( got, 4 );

  var dst = [ '1', '3' ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 3, 2, 1, '1', '3' ] );
  test.identical( got, 3 );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }
  var dst = [ 1, 3 ];
  var insArray = [ 0, 2, 4 ]
  var got = _.arrayPrependedArraysOnceStrictly( dst, insArray, onEqualize );
  test.identical( dst, [ 4, 2, 0, 1, 3 ] );
  test.identical( got, 3 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayPrependedArraysOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 2, undefined, 1 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayPrependedArraysOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, dst ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, dst ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, dst, ( e ) => e ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, [ dst ], ( e ) => e ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayPrependedArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e ) );
  else
  _.arrayPrependedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayPrependedArraysOnceStrictly( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 18 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnceStrictly();
  });

  // test.case = 'dst is not a array';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayPrependedArraysOnceStrictly( 1, [ 2 ] );
  // }); sfkldb fiubds lkfbds gbkdsfb gkldsfg fdsbfkldsfbdsl gbjs, fn kgn d

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnceStrictly( [], [ 1, 2, 3 ], [] )
  });

  test.case = 'second arg is not a ArrayLike entity';
  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnceStrictly( [ 1 ], 2 );
  });

  test.case = 'Elements must be unique';

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnceStrictly( [ 1, 1, 1 ], [ [ 1 ] ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnceStrictly( [ 1, 2, 3 ], [ [ 4, 5 ], 2 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayPrependedArraysOnceStrictly( [ 6 ], [ [ 1, 2 ], 3, [ 6, 4, 5, 1, 2, 3 ] ] );
  });

}

//

function arrayAppend( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppend( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var got = _.arrayAppend( [], 1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayAppend( [ 1 ], 1 );
  test.identical( got, [ 1, 1 ] );

  var got = _.arrayAppend( [ 1 ], 2 );
  test.identical( got, [ 1, 2 ] );

  var got = _.arrayAppend( [ 1, 2, 3 ], 3 );
  test.identical( got, [ 1, 2, 3, 3 ] );

  var got = _.arrayAppend( [ 1 ], '1' );
  test.identical( got, [ 1, '1' ] );

  var got = _.arrayAppend( [ 1 ], -1 );
  test.identical( got, [ 1, -1 ] );

  var got = _.arrayAppend( [ 1 ], [ 1 ] );
  test.identical( got, [ 1, [ 1 ] ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppend();
  })

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppend( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppend( 1, 1 );
  })
}

//

function arrayAppendOnce( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendOnce( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var got = _.arrayAppendOnce( [], 1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayAppendOnce( [ 1 ], 1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayAppendOnce( [ 1 ], 2 );
  test.identical( got, [ 1, 2 ] );

  var got = _.arrayAppendOnce( [ 1, 2, 3 ], 3 );
  test.identical( got, [ 1, 2, 3 ] );

  var got = _.arrayAppendOnce( [ 1 ], '1' );
  test.identical( got, [ 1, '1' ] );

  var got = _.arrayAppendOnce( [ 1 ], -1 );
  test.identical( got, [ 1, -1 ] );

  var got = _.arrayAppendOnce( [ 1 ], [ 1 ] );
  test.identical( got, [ 1, [ 1 ] ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendOnce( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendOnce( dst, { num : 1 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendOnce( dst, 4, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayAppendOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendOnce();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendOnce( 1, 1, 1 );
  })
}

//

function arrayAppendOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendOnceStrictly( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendOnceStrictly( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendOnceStrictly( dst, 2 );
  test.identical( got, [ 1, 2 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendOnceStrictly( dst, '1' );
  test.identical( got, [ 1, '1' ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendOnceStrictly( dst, -1 );
  test.identical( got, [ 1, -1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendOnceStrictly( dst, [ 1 ] );
  test.identical( got, [ 1, [ 1 ] ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendOnceStrictly( dst, 4, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendOnceStrictly( 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  // test.case = 'onEqualize is not a routine';

  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayAppendOnceStrictly( [ 1, 2, 3 ], 3, 3 );
  // });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayAppendOnceStrictly( dst, { num : 1 }, onEqualize );
  });

}

//

function arrayAppended( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppended( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayAppended( dst, 1 );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppended( dst, 2 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppended( dst, 3 );
  test.identical( dst, [ 1, 2, 3, 3 ] );
  test.identical( got, 3 );

  var dst = [ 1 ];
  var got = _.arrayAppended( dst, '1' );
  test.identical( dst, [ 1, '1' ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppended( dst, -1 );
  test.identical( dst, [ 1, -1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppended( dst, [ 1 ] );
  test.identical( dst, [ 1, [ 1 ] ] );
  test.identical( got, 1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppended();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppended( [], 1, 1, 1 );
  });

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppended( 1, 1 );
  });
}

//

function arrayAppendedOnce( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnce( dst, 2 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedOnce( dst, 3 );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnce( dst, '1' );
  test.identical( dst, [ 1, '1' ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnce( dst, -1 );
  test.identical( dst, [ 1, -1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, [ 1 ] ] );
  test.identical( got, 1 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendedOnce( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );
  test.identical( got, 3 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendedOnce( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendedOnce( dst, 4, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );
  test.identical( got, 3 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayAppendedOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnce();
  })

  test.case = 'third is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnce( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnce( 1, 1, 1 );
  })
}

//

function arrayAppendedOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedOnceStrictly( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnceStrictly( dst, 2 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnceStrictly( dst, '1' );
  test.identical( dst, [ 1, '1' ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnceStrictly( dst, -1 );
  test.identical( dst, [ 1, -1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedOnceStrictly( dst, [ 1 ] );
  test.identical( dst, [ 1, [ 1 ] ] );
  test.identical( got, 1 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendedOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );
  test.identical( got, 3 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendedOnceStrictly( dst, 4, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );
  test.identical( got, 3 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnceStrictly();
  })

  test.case = 'third is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnceStrictly( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnceStrictly( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnceStrictly( 1, 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayAppendedOnceStrictly( dst, { num : 1 }, onEqualize );
  });
}

//

function arrayAppendElement( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendElement( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var got = _.arrayAppendElement( [], 1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayAppendElement( [ 1 ], 1 );
  test.identical( got, [ 1, 1 ] );

  var got = _.arrayAppendElement( [ 1 ], 2 );
  test.identical( got, [ 1, 2 ] );

  var got = _.arrayAppendElement( [ 1, 2, 3 ], 3 );
  test.identical( got, [ 1, 2, 3, 3 ] );

  var got = _.arrayAppendElement( [ 1 ], '1' );
  test.identical( got, [ 1, '1' ] );

  var got = _.arrayAppendElement( [ 1 ], -1 );
  test.identical( got, [ 1, -1 ] );

  var got = _.arrayAppendElement( [ 1 ], [ 1 ] );
  test.identical( got, [ 1, [ 1 ] ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElement();
  })

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElement( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElement( 1, 1 );
  })
}

//

function arrayAppendElementOnce( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendElementOnce( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var got = _.arrayAppendElementOnce( [], 1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayAppendElementOnce( [ 1 ], 1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayAppendElementOnce( [ 1 ], 2 );
  test.identical( got, [ 1, 2 ] );

  var got = _.arrayAppendElementOnce( [ 1, 2, 3 ], 3 );
  test.identical( got, [ 1, 2, 3 ] );

  var got = _.arrayAppendElementOnce( [ 1 ], '1' );
  test.identical( got, [ 1, '1' ] );

  var got = _.arrayAppendElementOnce( [ 1 ], -1 );
  test.identical( got, [ 1, -1 ] );

  var got = _.arrayAppendElementOnce( [ 1 ], [ 1 ] );
  test.identical( got, [ 1, [ 1 ] ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendElementOnce( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendElementOnce( dst, { num : 1 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendElementOnce( dst, 4, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayAppendElementOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElementOnce();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElementOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElementOnce( 1, 1, 1 );
  })
}

//

function arrayAppendElementOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendElementOnceStrictly( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendElementOnceStrictly( dst, 1 );
  test.identical( got, [ 1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendElementOnceStrictly( dst, 2 );
  test.identical( got, [ 1, 2 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendElementOnceStrictly( dst, '1' );
  test.identical( got, [ 1, '1' ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendElementOnceStrictly( dst, -1 );
  test.identical( got, [ 1, -1 ] );
  test.true( got === dst );

  var dst = [ 1 ];
  var got = _.arrayAppendElementOnceStrictly( dst, [ 1 ] );
  test.identical( got, [ 1, [ 1 ] ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendElementOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendElementOnceStrictly( dst, 4, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElementOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElementOnceStrictly( 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElementOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendElementOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  // test.case = 'onEqualize is not a routine';

  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayAppendOnceStrictly( [ 1, 2, 3 ], 3, 3 );
  // });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayAppendElementOnceStrictly( dst, { num : 1 }, onEqualize );
  });

}

//

function arrayAppendedElement( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedElement( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElement( dst, 1 );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElement( dst, 2 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedElement( dst, 3 );
  test.identical( dst, [ 1, 2, 3, 3 ] );
  test.identical( got, 3 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElement( dst, '1' );
  test.identical( dst, [ 1, '1' ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElement( dst, -1 );
  test.identical( dst, [ 1, -1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElement( dst, [ 1 ] );
  test.identical( dst, [ 1, [ 1 ] ] );
  test.identical( got, 1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElement();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElement( [], 1, 1 );
  });

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElement( 1, 1 );
  });
}

//

function arrayAppendedElementOnce( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedElementOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnce( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, false );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnce( dst, 2 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedElementOnce( dst, 3 );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, false );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnce( dst, '1' );
  test.identical( dst, [ 1, '1' ] );
  test.identical( got, '1' );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnce( dst, -1 );
  test.identical( dst, [ 1, -1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, [ 1 ] ] );
  test.identical( got, [ 1 ] );

  var dst = [ 0, 1, 2 ];
  var got = _.arrayAppendedElementOnce( dst, NaN );
  test.identical( dst, [ 0, 1, 2, NaN ] );
  test.identical( got, NaN );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendedElementOnce( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );
  test.identical( got, { num : 4 } );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendedElementOnce( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, false );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendedElementOnce( dst, 4, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );
  test.identical( got, 4 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayAppendedElementOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, false );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnce();
  })

  test.case = 'third is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnce( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnce( 1, 1, 1 );
  })
}

//

function arrayAppendedElementOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedElementOnceStrictly( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnceStrictly( dst, 2 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnceStrictly( dst, '1' );
  test.identical( dst, [ 1, '1' ] );
  test.identical( got, '1' );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnceStrictly( dst, -1 );
  test.identical( dst, [ 1, -1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayAppendedElementOnceStrictly( dst, [ 1 ] );
  test.identical( dst, [ 1, [ 1 ] ] );
  test.identical( got, [ 1 ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayAppendedElementOnceStrictly( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, { num : 4 } ] );
  test.identical( got, { num : 4 } );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayAppendedElementOnceStrictly( dst, 4, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 }, 4 ] );
  test.identical( got, 4 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnceStrictly();
  })

  test.case = 'fourth is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedOnceStrictly( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnceStrictly( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnceStrictly( 1, 1, 1 );
  })

  test.case = 'ins already exists in dst';

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnceStrictly( [ 1 ], 1 );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedElementOnceStrictly( [ 1, 2, 3 ], 3 );
  });

  test.shouldThrowErrorSync( function()
  {
    var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayAppendedElementOnceStrictly( dst, { num : 1 }, onEqualize );
  });
}

// //
//
// function arrayAppendArray( test )
// {
//
//   test.case = 'nothing';
//   var got = _.arrayAppendArray( [] );
//   var expected = [];
//   test.identical( got, expected );
//
//   test.case = 'an argument';
//   var got = _.arrayAppendArray( [ 1, 2, undefined ] );
//   var expected = [ 1, 2, undefined ];
//   test.identical( got, expected );
//
//   test.case = 'an array';
//   var got = _.arrayAppendArray( [ 1, 2 ], 'str', false, { a : 1 }, 42, [ 3, 7, 13 ] );
//   var expected = [ 1, 2, 'str', false, { a : 1 }, 42, 3, 7, 13 ];
//   test.identical( got, expected );
//
//   /**/
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayAppendArray();
//   });
//
//   test.case = 'arguments[ 0 ] is wrong, has to be an array';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayAppendArray( 'wrong argument', 'str', false, { a : 1 }, 42, [ 3, 7, 13 ] );
//   });
//
//   test.case = 'arguments[ 1 ] is undefined';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayAppendArray( [ 1, 2 ], undefined, false, { a : 1 }, 42, [ 3, 7, 13 ] );
//   });
//
// };

//

function arrayAppendArray( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendArray( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';
  var got = _.arrayAppendArray( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayAppendArray( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArray( dst, [ 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendArray( dst, [ 1, 1 ] );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayAppendArray( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 1, 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.true( got === dst );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArray( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayAppendArray( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.true( got === dst );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArray( dst, dst );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArray();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArray( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArray( [ 1, 2 ], 2 );
  });
};

//

function arrayAppendArrayOnce( test )
{
  test.case = 'dst - empty array, ins - number, evaluator returns first el of array'

  test.case = 'dstArray is null';
  var got = _.arrayAppendArrayOnce( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';
  var got = _.arrayAppendArrayOnce( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayAppendArrayOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'src array has duplicates';
  var dst = [ 1, 2, 3, 'str', 5 ];
  var got = _.arrayAppendArrayOnce( dst, [ 1, 1, 2, 2, 3, 3, 'str', 'str', 5 ] );
  test.identical( got, [ 1, 2, 3, 'str', 5 ] );
  test.true( got === dst );

  test.case = 'dst and src array has duplicates';
  var dst = [ 1, 1, 2, 2, 3, 3, 'str', 'str', 5, 5 ];
  var got = _.arrayAppendArrayOnce( dst, [ 1, 1, 2, 2, 3, 3, 'str', 'str', 5 ] );
  test.identical( got, [ 1, 1, 2, 2, 3, 3, 'str', 'str', 5, 5 ] );
  test.true( got === dst );

  test.case = 'appends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArrayOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendArrayOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayAppendArrayOnce( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 1, 'a', [ { a : 1 } ], { b : 2 } ] );
  test.true( got === dst );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayAppendArrayOnce( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.true( got === dst );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArrayOnce( dst, dst );
  test.identical( got, [ 1, 2, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArrayOnce( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArrayOnce( dst, dst, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArrayOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArrayOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArrayOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst - empty array, ins - number, evaluator returns first el of array';
  var dst = [];
  var got = _.arrayAppendArrayOnce( dst, [ 0, [ 2 ], 5, { a : 0 } ], ( e ) => e[ 0 ] );
  test.identical( dst, [ 0, [ 2 ] ] );
  test.identical( got, dst );

  test.case = 'dst - empty array, ins - number, evaluator returns first el of array';
  var dst = [];
  var got = _.arrayAppendArrayOnce( dst, [ 0, [ 2 ], 5, { a : 0 } ], ( e ) => e[ 0 ] !== undefined );
  test.identical( dst, [ 0, [ 2 ] ] );
  test.identical( got, dst );

  test.case = 'dst - empty array, ins - number, evaluator returns first el of array';
  var dst = [ [ 1 ], [ 3 ] ];
  var got = _.arrayAppendArrayOnce( dst, [ 0, [ 2 ], 5, { a : 0 } ], ( e ) => e[ 0 ] );
  test.identical( dst, [ [ 1 ], [ 3 ], 0, [ 2 ] ] );
  test.identical( got, dst );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrayOnce();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrayOnce( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrayOnce( [ 1, 2 ], 2 );
  });

  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArrayOnce( [ 1 ], undefined );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrayOnce( [ 1, 2 ], [ 2 ], 2, 'wrong' );
  });

}

//

function arrayAppendArrayOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendArrayOnceStrictly( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';

  var got = _.arrayAppendArrayOnceStrictly( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendArrayOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArrayOnceStrictly( dst, [ 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.true( got === dst );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArrayOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendArrayOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArrayOnceStrictly( dst, dst ) );
  else
  _.arrayAppendArrayOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArrayOnceStrictly( dst, dst ) );
  else
  _.arrayAppendArrayOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArrayOnceStrictly( dst, dst, ( e ) => e ) );
  else
  _.arrayAppendArrayOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  _.arrayAppendArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrayOnceStrictly();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrayOnceStrictly( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrayOnceStrictly( [ 1, 2 ], 2 );
  });

  test.case = 'one of elements is not unique';

  var dst = [ 1, 2, 3 ];
  var onErrorCallback = ( err, arg ) => test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArrayOnceStrictly( dst, [ 4, 5, 2 ] );
  }, onErrorCallback );

  var dst = [ 1, 1, 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArrayOnceStrictly( dst, [ 1 ] );
  })
  test.identical( dst, [ 1, 1, 1 ] );

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayAppendArrayOnceStrictly( [ 1, 2 ], [ 2 ], 3 );
  // });
}

//

function arrayAppendedArray( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayAppendedArray( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedArray( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArray( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 2, 4, 5 ] );
  test.identical( got, 3 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendedArray( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var got = _.arrayAppendedArray( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 1, 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( got, 4 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArray( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendedArray( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArray( dst, dst );
  test.identical( got, 3 );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArray();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArray( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArray( [ 1, 2 ], 2 );
  });
}

//

function arrayAppendedArrayOnce( test )
{

  test.case = 'nothing';

  var dst = [];
  var got = _.arrayAppendedArrayOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedArrayOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArrayOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendedArrayOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 0 );

  test.case = 'mixed arguments types';

  var dst = [ 1 ];
  var got = _.arrayAppendedArrayOnce( dst, [ 'a', 1, [ { a : 1 } ], { b : 2 } ] );
  test.identical( dst, [ 1, 'a', [ { a : 1 } ], { b : 2 } ] );
  test.identical( got, 3 );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendedArrayOnce( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArrayOnce( dst, dst );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArrayOnce( dst, dst, ( e ) => e );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArrayOnce( dst, dst, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArrayOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArrayOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 6 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArrayOnce( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnce();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnce( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnce( [ 1, 2 ], 2 );
  });

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayAppendedArrayOnce( [ 1, 2 ], [ 2 ], 3 );
  // });

}

//

function arrayAppendedArrayOnceWithSelector( test )
{

  test.case = 'nothing, single equalizer';

  var dst = [];
  var got = _.arrayAppendedArrayOnce( dst, [], ( e ) => e.a );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple, single equalizer';

  var dst = [];
  var got = _.arrayAppendedArrayOnce( dst, [ { a : 1 }, { a : 2 }, { a : 3 } ], ( e ) => e.a );
  test.identical( dst, [ { a : 1 }, { a : 2 }, { a : 3 } ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements, single equalizer';

  var dst = [ { a : 1 }, { a : 2 }, { a : 3 } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { a : 2 }, { a : 3 }, { a : 4 } ], ( e ) => e.a );
  test.identical( dst, [ { a : 1 }, { a : 2 }, { a : 3 }, { a : 4 } ] );
  test.identical( got, 1 );

  var dst = [ { a : 1 }, { a : 1 }, { a : 1 } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { a : 1 } ], ( e ) => e.a );
  test.identical( dst, [ { a : 1 }, { a : 1 }, { a : 1 } ] );
  test.identical( got, 0 );

  test.case = 'mixed arguments types, single equalizer';

  var dst = [ { a : 1 } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { a : 'a' }, { a : 1 }, { a : [ { y : 2 } ] } ], ( e ) => e.a );
  test.identical( dst, [ { a : 1 }, { a : 'a' }, { a : [ { y : 2 } ] } ] );
  test.identical( got, 2 );

  test.case = 'array has undefined, single equalizer';

  var dst = [ { a : 1 } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { a : undefined }, { a : 2 } ], ( e ) => e.a );
  test.identical( dst, [ { a : 1 }, { a : undefined }, { a : 2 } ] );
  test.identical( got, 2 );

  var dst = [ { a : 1 }, { a : undefined } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { a : undefined }, { a : 2 } ], ( e ) => e.a );
  test.identical( dst, [ { a : 1 }, { a : undefined }, { a : 2 } ] );
  test.identical( got, 1 );

  /* */

  test.case = 'nothing, two equalizers';

  var dst = [];
  var got = _.arrayAppendedArrayOnce( dst, [], ( e ) => e.a, ( e ) => e.b );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple, two equalizers';

  var dst = [];
  var got = _.arrayAppendedArrayOnce( dst, [ { b : 1 }, { b : 2 }, { b : 3 } ], ( e ) => e.a, ( e ) => e.b );
  test.identical( dst, [ { b : 1 }, { b : 2 }, { b : 3 } ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements, two equalizers';

  var dst = [ { a : 1 }, { a : 2 }, { a : 3 } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { b : 2 }, { b : 3 }, { b : 4 } ], ( e ) => e.a, ( e ) => e.b );
  test.identical( dst, [ { a : 1 }, { a : 2 }, { a : 3 }, { b : 4 } ] );
  test.identical( got, 1 );

  var dst = [ { a : 1 }, { a : 1 }, { a : 1 } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { b : 1 } ], ( e ) => e.a, ( e ) => e.b );
  test.identical( dst, [ { a : 1 }, { a : 1 }, { a : 1 } ] );
  test.identical( got, 0 );

  test.case = 'mixed arguments types, two equalizers';

  var dst = [ { a : 1 } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { b : 'a' }, { b : 1 }, { b : [ { y : 2 } ] } ], ( e ) => e.a, ( e ) => e.b );
  test.identical( dst, [ { a : 1 }, { b : 'a' }, { b : [ { y : 2 } ] } ] );
  test.identical( got, 2 );

  test.case = 'array has undefined, two equalizers';

  var dst = [ { a : 1 } ];
  var got;
  var onResultCallback = ( err, arg ) => test.identical( dst, [ { a : 1 }, { b : undefined }, { b : 2 } ] );
  test.mustNotThrowError( function ()
  {
    var got = _.arrayAppendedArrayOnce( dst, [ { b : undefined }, { b : 2 } ], ( e ) => e.a, ( e ) => e.b );
  }, onResultCallback );
  test.identical( got, 2 );

  var dst = [ { a : 1 }, { a : undefined } ];
  var got = _.arrayAppendedArrayOnce( dst, [ { b : undefined }, { b : 2 } ], ( e ) => e.a, ( e ) => e.b );
  test.identical( dst, [ { a : 1 }, { b : undefined }, { b : 2 } ] );
  test.identical( got, 1 );

  /* */

  if( !Config.debug )
  return;

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArrayOnce( dst, undefined, ( e ) => e.a );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnce();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnce( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnce( [ 1, 2 ], 2, ( e ) => e.a );
  });

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayAppendedArrayOnce( [ 1, 2 ], [ 2 ], 3 );
  // });

}

//

function arrayAppendedArrayOnceStrictly( test )
{
  test.case = 'nothing';

  var got = _.arrayAppendedArrayOnceStrictly( [], [] );
  var expected = 0;
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedArrayOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArrayOnceStrictly( dst, [ 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 2 );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArrayOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendedArrayOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArrayOnceStrictly( dst, dst ) );
  else
  {
    var got1 = _.arrayAppendedArrayOnceStrictly( dst, dst );
    test.identical( got1, 0 );
  }
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArrayOnceStrictly( dst, dst ) );
  else
  {
    var got2 = _.arrayPrependArrayOnceStrictly( dst, dst );
    test.identical( got2, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArrayOnceStrictly( dst, dst, ( e ) => e ) );
  else
  {
    var got3 = _.arrayAppendedArrayOnceStrictly( dst, dst, ( e ) => e )
    test.identical( got3, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  {
    var got4 = _.arrayPrependArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
    test.identical( got4, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnceStrictly();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnceStrictly( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrayOnceStrictly( [ 1, 2 ], 2 );
  });

  test.case = 'one of elements is not unique';

  var dst = [ 1, 2, 3 ];
  var onErrorCallback = ( err, arg ) => test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArrayOnceStrictly( dst, [ 4, 5, 2 ] );
  }, onErrorCallback );

  var dst = [ 1, 1, 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArrayOnceStrictly( dst, [ 1 ] );
  })
  test.identical( dst, [ 1, 1, 1 ] );

}

//

function arrayAppendArrays( test )
{

  test.case = 'dstArray is null, src is scalar';
  var got = _.arrayAppendArrays( null, 1 );
  test.identical( got, [ 1 ] );

  test.case = 'dstArray is null, src = array';
  var got = _.arrayAppendArrays( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  // test.case = 'dstArray is undefined, src is scalar';
  // var got = _.arrayAppendArrays( undefined, 1 );
  // test.identical( got, 1 );
  /* */
  // test.case = 'dstArray is undefined, src = array';
  // let src = [ 1 ];
  // var got = _.arrayAppendArrays( undefined, src );
  // test.identical( got, [ 1 ] );
  // test.true( src === got );

  test.case = 'nothing';
  var got = _.arrayAppendArrays( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayAppendArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArrays( dst, [ 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendArrays( dst, [ 1, 1 ] );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 5 ] ] ]
  var got = _.arrayAppendArrays( dst, insArray );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3, [ 5 ] ] );
  test.true( got === dst );

  test.case = 'arguments are not arrays';
  var dst = [];
  var got = _.arrayAppendArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ 'a', 1, [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayAppendArrays( dst, insArray );
  test.identical( dst, [ 1, 'a', 1, { a : 1 }, { b : 2 } ] );
  test.true( got === dst );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArrays( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayAppendArrays( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.true( got === dst );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArrays( dst, dst );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArrays( dst, [ dst ] );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArrays( dst, [ dst, dst ] );
  test.identical( got, [ 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrays();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrays( 1, [ 2 ] );
  });

  test.case = 'second arg is not a ArrayLike entity';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrays( [], undefined );
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArrays( [], [ 1 ], [ 2 ] );
  });
};

//

function arrayAppendArraysOnce( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendArraysOnce( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';

  var got = _.arrayAppendArraysOnce( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArraysOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendArraysOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ 'a', 1, [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayAppendArraysOnce( dst, insArray );
  test.identical( dst, [ 1, 'a', { a : 1 }, { b : 2 } ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 5 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, [ 5 ] ], 6 ] ];
  var got = _.arrayAppendArraysOnce( dst, insArray );
  test.identical( dst, [ 1, 2, 3, 5, [ 4, [ 5 ] ], 6 ] );
  test.true( got === dst );

  var dst = [ 1, 3 ];
  var got = _.arrayAppendArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 3, 2 ] );
  test.identical( dst, got );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }

  var dst = [ 1, 3 ];
  var got = _.arrayAppendArraysOnce( dst, [ 1, 2, 3 ], onEqualize )
  test.identical( got, [ 1, 3, 2 ] );
  test.identical( dst, got );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendArraysOnce( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendArraysOnce( dst, dst );
  test.identical( got, [ 1, 2, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, [ dst ] );
  test.identical( got, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, dst, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnce( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArraysOnce();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArraysOnce( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArraysOnce( [], [ 1, 2, 3 ], [] );
  });

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArraysOnce( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

}

//

function arrayAppendArraysOnceStrictly( test )
{
  test.case = 'dstArray is null';
  var got = _.arrayAppendArraysOnceStrictly( null, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'nothing';

  var got = _.arrayAppendArraysOnceStrictly( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ [ 'a' ], [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayAppendArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ 1, 'a', { a : 1 }, { b : 2 } ] );
  test.true( got === dst );

  var dst = [ 0 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, [ 5 ] ], 6 ] ];
  var got = _.arrayAppendArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ 0, 1, 2, 3, [ 4, [ 5 ] ], 6 ] );
  test.true( got === dst );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }

  var dst = [ 4, 5 ];
  var got = _.arrayAppendArraysOnceStrictly( dst, [ 1, 2, 3 ], onEqualize );
  test.identical( got, [ 4, 5, 1, 2, 3 ] );
  test.identical( dst, got );

  test.case = 'ins has existing element';

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendArraysOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => { _.arrayAppendArraysOnceStrictly( dst, dst ) });
  else
  _.arrayAppendArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayAppendArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArraysOnceStrictly( dst, dst ) );
  else
  _.arrayAppendArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayAppendArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArraysOnceStrictly( dst, dst, ( e ) => e ) );
  else
  _.arrayAppendArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArraysOnceStrictly( dst, [ dst ], ( e ) => e ) );
  else
  _.arrayAppendArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  _.arrayAppendArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e ) );
  else
  _.arrayAppendArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendArraysOnceStrictly( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArraysOnceStrictly();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArraysOnceStrictly( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArraysOnceStrictly( [], [ 1, 2, 3 ], [] )
  });

  test.case = 'One of ins elements is not unique';
  var dst = [ 1, 2, 3 ];
  var onErrorCallback = ( err, arg ) => test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArraysOnceStrictly( dst, [ 4, 2, 5 ] );
  });

  var dst = [ 1, 1, 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArraysOnceStrictly( dst, [ 1 ] );
  })
  test.identical( dst, [ 1, 1, 1 ] );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendArraysOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'second arg is not a ArrayLike entity';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendArraysOnceStrictly( [], 1 );
  });

}

//

function arrayAppendedArrays( test )
{
  test.case = 'nothing';
  var dst = [];
  var got = _.arrayAppendedArrays( dst, [] );
  var expected = [];
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayAppendedArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArrays( dst, [ 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendedArrays( dst, [ 1, 1 ] );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, 5 ], 6 ] ];
  var got = _.arrayAppendedArrays( dst, insArray );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3, [ 4, 5 ], 6 ] );
  test.identical( got, 5 );

  test.case = 'arguments are not arrays';
  var dst = [];
  var got = _.arrayAppendedArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ 'a', 1, [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayAppendedArrays( dst, insArray );
  test.identical( dst, [ 1, 'a', 1, { a : 1 }, { b : 2 } ] );
  test.identical( got, 4 );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayAppendedArrays( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArrays( dst, dst );
  test.identical( got, 3 );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArrays( dst, [ dst ] );
  test.identical( got, 6 );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArrays( dst, [ dst, dst ] );
  test.identical( got, 18 );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrays();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrays( 1, [ 2 ] );
  });

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArrays( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'second arg is not a ArrayLike entity';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrays( [], undefined );
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArrays( [], [ 1 ], [ 2 ] );
  });

}

//

function arrayAppendedArraysOnce( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayAppendedArraysOnce( dst, [] );
  var expected = [];
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayAppendedArraysOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 0 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ 'a', 1, [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayAppendedArraysOnce( dst, insArray );
  test.identical( dst, [ 1, 'a', { a : 1 }, { b : 2 } ] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 3, 5 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, [ 5 ] ], 6 ] ];
  var got = _.arrayAppendedArraysOnce( dst, insArray );
  test.identical( dst, [ 1, 2, 3, 5, [ 4, [ 5 ] ], 6 ] );
  test.identical( got, 2 );

  var dst = [ 1, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, 1 );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }

  var dst = [ 1, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, [ 1, 2, 3 ], onEqualize );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, 1 );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendedArraysOnce( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, [ dst ] );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, dst, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnce( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 18 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnce();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnce( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnce( [], [ 1, 2, 3 ], [] )
  });

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArraysOnce( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'second arg is not a ArrayLike entity';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnce( [], undefined );
  });

}

//

function arrayAppendedArraysOnceStrictly( test )
{
  test.case = 'nothing';

  var got = _.arrayAppendedArraysOnceStrictly( [], [] );
  var expected = 0;
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayAppendedArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'mixed arguments types';
  var dst = [ 1 ];
  var insArray = [ [ 'a' ], [ { a : 1 } ], { b : 2 } ];
  var got = _.arrayAppendedArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ 1, 'a', { a : 1 }, { b : 2 } ] );
  test.identical( got, 3 );

  var dst = [ 0 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3, [ 4, [ 5 ] ], 6 ] ];
  var got = _.arrayAppendedArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ 0, 1, 2, 3, [ 4, [ 5 ] ], 6 ] );
  test.identical( got, 5 );

  test.case = 'onEqualize';

  var onEqualize = ( a, b ) =>
  {
    return a === b;
  }

  var dst = [ 4, 5 ];
  var got = _.arrayAppendedArraysOnceStrictly( dst, [ 1, 2, 3 ], onEqualize );
  test.identical( dst, [ 4, 5, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'ins has existing element';

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var onResultCallback = ( err, arg ) => test.identical( dst, [ 1, undefined, 2 ] );
  test.mustNotThrowError( function ()
  {
    _.arrayAppendedArraysOnceStrictly( dst, [ undefined, 2 ] );
  }, onResultCallback );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, dst ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, dst ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, [ dst ] ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, dst, ( e ) => e ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, [ dst ], ( e ) => e ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst, ( e ) => e )
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayAppendedArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e ) );
  else
  _.arrayAppendedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e)
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayAppendedArraysOnceStrictly( dst, [ dst, dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ] );
  test.identical( got, 18 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnceStrictly();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnceStrictly( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnceStrictly( [], [ 1, 2, 3 ], [] )
  });

  test.case = 'One of ins elements is not unique';
  var dst = [ 1, 2, 3 ];
  var onErrorCallback = ( err, arg ) => test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.shouldThrowErrorSync( () =>
  {
    _.arrayAppendedArraysOnceStrictly( dst, [ 4, 2, 5 ] );
  }, onErrorCallback );

  var dst = [ 1, 1, 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArraysOnceStrictly( dst, [ 1 ] );
  })
  test.identical( dst, [ 1, 1, 1 ] );

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayAppendedArraysOnceStrictly( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'second arg is not a ArrayLike entity';
  test.shouldThrowErrorSync( function()
  {
    _.arrayAppendedArraysOnceStrictly( [], 1 );
  });

}

// --
// arrayRemove
// --

function arrayRemove( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemove( dst, 1 );
  test.identical( dst, [] );

  var dst = [ 1 ];
  var got = _.arrayRemove( dst, 1 );
  test.identical( dst, [] );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemove( dst, 2 );
  test.identical( dst, [ 1 ] );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemove( dst, 1 );
  test.identical( dst, [ 2, 2 ] );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemove( dst, 1 );
  test.identical( dst, [] );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemove( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );

  var dst = [ 1 ];
  var got = _.arrayRemove( dst, '1' );
  test.identical( dst, [ 1 ] );

  var dst = [ 1 ];
  var got = _.arrayRemove( dst, -1 );
  test.identical( dst, [ 1 ] );

  var dst = [ 1 ];
  var got = _.arrayRemove( dst, [ 1 ] );
  test.identical( dst, [ 1 ] );

  var dst = [ { x : 1 } ];
  var got = _.arrayRemove( dst, { x : 1 } );
  test.identical( dst, [ { x : 1 } ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemove( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemove( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayRemove( dst, 4, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemove( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );

  test.case = 'equalizer 1 arg';

  var dst = [ [ 1 ], [ 1 ], [ 1 ] ];
  var onEqualize = ( a ) =>
  {
    return a[ 0 ];
  }
  var got = _.arrayRemove( dst, [ 1 ], onEqualize );
  test.identical( dst, [] );

  test.case = 'equalizer 2 args';

  var dst = [ [ 1 ], [ 1 ], [ 1 ] ];
  var onEqualize = ( a ) =>
  {
    return a[ 0 ];
  }
  var onEqualize2 = ( a ) =>
  {
    return a;
  }
  var got = _.arrayRemove( dst, 1, onEqualize, onEqualize2 );
  test.identical( dst, [] );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemove();
  })

  test.case = 'fourth is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemove( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemove( 1, 1 );
  })
}

//

function arrayRemoveOnce( test )
{
  test.case = 'simple';

  var got = _.arrayRemoveOnce( [], 1 );
  test.identical( got, [] );

  var got = _.arrayRemoveOnce( [ 1 ], 1 );
  test.identical( got, [] );

  var got = _.arrayRemoveOnce( [ 1, 2, 2 ], 2 );
  test.identical( got, [ 1, 2 ] );

  var got = _.arrayRemoveOnce( [ 1, 3, 2, 3 ], 3 );
  test.identical( got, [ 1, 2, 3 ] );

  var got = _.arrayRemoveOnce( [ 1 ], '1' );
  test.identical( got, [ 1 ] );

  var got = _.arrayRemoveOnce( [ 1 ], -1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayRemoveOnce( [ 1 ], [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoveOnce( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoveOnce( dst, { num : 1 }, onEqualize );
  test.identical( got, [ { num : 2 }, { num : 3 } ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var onEqualize2 = ( a ) =>
  {
    return a;
  }
  var got = _.arrayRemoveOnce( dst, 4, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemoveOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 1 }, { num : 3 } ];
  var got = _.arrayRemoveOnce( dst, 1, onEqualize, onEqualize2 );
  test.identical( got, [ { num : 2 }, { num : 1 }, { num : 3 } ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveOnce();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveOnce( 1, 1, 1 );
  })
}

//

function arrayRemoveOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveOnceStrictly( dst, 2 );
  test.identical( got, [ 1, 3 ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoveOnceStrictly( dst, { num : 3 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemoveOnceStrictly( dst, 3, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 1 }, { num : 2 } ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveOnceStrictly( 1, 1 );
  })

  test.case = 'ins doesn´t exist';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveOnceStrictly( [ 1 ], 2 );
  });

  test.case = 'ins is not unique in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveOnceStrictly( [ 1, 2, 2 ], 2 );
  });

  test.case = 'onEqualize is not a routine';

  test.shouldThrowErrorSync( () =>
  {
    _.arrayRemoveOnceStrictly( [ 1, 2, 3 ], 3, 3 );
  });

  test.case = 'onEqualize';
  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];

  test.shouldThrowErrorSync( () =>
  {
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayRemoveOnceStrictly( dst, { num : 4 }, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )


  test.shouldThrowErrorSync( () =>
  {
    var onEqualize = ( a ) =>
    {
      return a.num;
    }
    _.arrayRemoveOnceStrictly( dst, 4, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )
}

//

function arrayRemoved( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemoved( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemoved( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 1 );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemoved( dst, 2 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 2 );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemoved( dst, 1 );
  test.identical( dst, [ 2, 2 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayRemoved( dst, '1' );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemoved( dst, -1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemoved( dst, [ 1 ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoved( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 3 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoved( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 0 );

  var dst = [ { x : 1 } ];
  var got = _.arrayRemoved( dst, { x : 1 } );
  test.identical( dst, [ { x : 1 } ] );
  test.identical( got, 0 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoved( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoved( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 1 );


  test.case = 'evaluator 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var evaluator1 = ( a ) =>
  {
    return a.num;
  }
  var evaluator2 = ( a ) =>
  {
    return a;
  }
  var got = _.arrayRemoved( dst, 4, evaluator1 );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemoved( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 1 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 1 }, { num : 3 } ];
  var got = _.arrayRemoved( dst, 1, evaluator1, evaluator2 );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 2 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoved();
  })

  test.case = 'fourth is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoved( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoved( 1, 1 );
  })
}

//

function arrayRemovedOnce( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemovedOnce( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayRemovedOnce( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedOnce( dst, 3 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemovedOnce( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedOnce( dst, '1' );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayRemovedOnce( dst, -1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayRemovedOnce( dst, [ 1 ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedOnce( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedOnce( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayRemovedOnce( dst, 4, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemovedOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnce();
  })

  test.case = 'third is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnce( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnce( 1, 1 );
  })
}

//

function arrayRemovedOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1 ];
  var got = _.arrayRemovedOnceStrictly( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedOnceStrictly( dst, 2 );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedOnceStrictly( dst, { num : 3 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 } ] );
  test.identical( got, 2 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemovedOnceStrictly( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly();
  })

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [], 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( 1, 1 );
  })

  test.case = 'Simple no match element';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [], 1 );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [ 1 ], '1' );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [ 1 ], - 1 );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [ 1 ], [ 1 ] );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [ 1 ], 2 );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [ 1 ], - 1 );
  })

  test.case = 'Ins several times in srcArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( [ 2, 2, 1 ], 2 );
  })

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedOnceStrictly( dst, { num : 4 }, onEqualize );
  })

}

//

function arrayRemoveElement( test )
{
  test.open( 'array' );
  run( ( src ) => _.array.make( src ) );
  test.close( 'array' );

  /* - */

  test.open( 'unroll' );
  run( ( src ) => _.unroll.make( src ) );
  test.close( 'unroll' );

  /* - */

  function run( makeLong )
  {
    test.case = 'dst = empty array, ins = number';
    var dst = makeLong( [] );
    var got = _.arrayRemoveElement( dst, 1 );
    test.identical( dst, [] );
    test.true( got === dst );

    test.case = 'dst = array, ins = number, full removing';
    var dst = makeLong( [ 1 ] );
    var got = _.arrayRemoveElement( dst, 1 );
    test.identical( dst, [] );
    test.true( got === dst );

    var dst = makeLong( [ 2, 2, 2, 2, 2 ] );
    var got = _.arrayRemoveElement( dst, 2 );
    test.identical( dst, [] );
    test.true( got === dst );

    test.case = 'dst = array, ins = number, not a full removing';
    var dst = makeLong( [ 2, 2, 1 ] );
    var got = _.arrayRemoveElement( dst, 2 );
    test.identical( dst, [ 1 ] );
    test.true( got === dst );

    test.case = 'dst = array, ins = array, not a evaluator, no removing';
    var dst = makeLong( [ 1, 1, 1 ] );
    var got = _.arrayRemoveElement( dst, [ 1 ] );
    test.identical( dst, [ 1, 1, 1 ] );
    test.true( got === dst );

    test.case = 'dst = array, ins = string, no removing';
    var dst = makeLong( [ 1, 1, 1 ] );
    var got = _.arrayRemoveElement( dst, '1' );
    test.identical( dst, [ 1, 1, 1 ] );
    test.true( got === dst );

    test.case = 'dst = array, ins = negative number, no removing';
    var dst = makeLong( [ 1, 1, 1 ] );
    var got = _.arrayRemoveElement( dst, -1 );
    test.identical( dst, [ 1, 1, 1 ] );
    test.true( got === dst );

    test.case = 'dst = array with map, ins = same map, not a evaluator, not removing';
    var dst = makeLong( [ { x : 1 }, { x : 1 } ] );
    var got = _.arrayRemoveElement( dst, { x : 1 } );
    test.identical( dst, [ { x : 1 }, { x : 1 } ] );
    test.true( got === dst );

    test.case = 'dst = array with map, ins = number, equalizer, removing';
    var dst = makeLong( [ { value : 1 }, { value : 1 }, { value : 2 } ] );
    var onEqualize = ( a, b ) => a.value === b;
    var got = _.arrayRemoveElement( dst, 1, onEqualize );
    test.identical( got, [ { value : 2 } ] );
    test.true( got === dst );

    test.case = 'dst = array, ins = number, offset for removing';
    var dst = makeLong( [ 1, 2, 3, 1, 2, 3 ] );
    var got = _.arrayRemoveElement( dst, 1, 2 );
    test.identical( got, [ 1, 2, 3, 2, 3 ] );
    test.true( dst === got )

    test.case = 'dst = array with maps, ins = map, equalizer, no removing';
    var dst = makeLong( [ { num : 1 }, { num : 2 }, { num : 3 } ] );
    var onEqualize = ( a, b ) => a.num === b.num;
    var got = _.arrayRemoveElement( dst, { num : 4 }, onEqualize );
    test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
    test.true( got === dst );

    test.case = 'dst = array with maps, ins = map, equalizer, removing';
    var dst = makeLong( [ { num : 1 }, { num : 2 }, { num : 3 } ] );
    var onEqualize = ( a, b ) => a.num === b.num;
    var got = _.arrayRemoveElement( dst, { num : 1 }, onEqualize );
    test.identical( dst, [ { num : 2 }, { num : 3 } ] );
    test.true( got === dst );

    test.case = 'dst = array with maps, ins = map, one evaluator, no removing';
    var dst = makeLong( [ { num : 1 }, { num : 2 }, { num : 3 } ] );
    var onEvaluate = ( a ) => a.num;
    var got = _.arrayRemoveElement( dst, { num : 4 }, onEvaluate );
    test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
    test.true( got === dst );

    test.case = 'dst = array with maps, ins = map, two evaluators, removing';
    var dst = makeLong( [ { num : 1 }, { num : 2 }, { num : 3 } ] );
    var got = _.arrayRemoveElement( dst, 1, ( e ) => e.num, ( e ) => e );
    test.identical( dst, [ { num : 2 }, { num : 3 } ] );
    test.true( got === dst );

    test.case = 'dst = complex array, ins = array, one evaluator, full removing';
    var dst = makeLong( [ [ 1 ], [ 1 ], [ 1 ] ] );
    var onEvaluate = ( a ) => a[ 0 ];
    var got = _.arrayRemoveElement( dst, [ 1 ], onEvaluate );
    test.identical( dst, [] );
    test.true( got === dst );

    test.case = 'dst = complex array, ins = number, two evaluators, full removing';
    var dst = makeLong( [ [ 1 ], [ 1 ], [ 1 ] ] );
    var onEvaluate1 = ( a ) => a[ 0 ];
    var onEvaluate2 = ( a ) => a;
    var got = _.arrayRemoveElement( dst, 1, onEvaluate1, onEvaluate2 );
    test.identical( dst, [] );
    test.true( got === dst );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayRemoveElement() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( [ 1 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( [ 1, 1 ], 0, 1, ( e ) => e, ( e ) => e, 'extra' ) );

  test.case = 'wrong type of dstArray';
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( 1, 1 ) );
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( _.argumentsArray.make( [ 1 ] ), 1 ) );
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( new U8x( [ 1 ] ), 1 ) );

  test.case = 'evaluator is not a routine';
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( [ 1 ], 1, 1, 1 ) );

  test.case = 'evaluator ( equalizer ) has wrong length';
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( [ 1 ], 1, () => 1 ) );
  /* eslint-disable */
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( [ 1 ], 1, ( a, b, c, d ) => a - b + c === d ) );
  /* eslint-enable */
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( [ 1 ], 1, ( a ) => a, () => 1 ) );
  test.shouldThrowErrorSync( () => _.arrayRemoveElement( [ 1 ], 1, ( a ) => a, ( a, b ) => a === b ) );
}

//

function arrayRemoveElementOnce( test )
{
  test.case = 'simple';

  var got = _.arrayRemoveElementOnce( [], 1 );
  test.identical( got, [] );

  var got = _.arrayRemoveElementOnce( [ 1 ], 1 );
  test.identical( got, [] );

  var got = _.arrayRemoveElementOnce( [ 1, 2, 2 ], 2 );
  test.identical( got, [ 1, 2 ] );

  var got = _.arrayRemoveElementOnce( [ 1, 3, 2, 3 ], 3 );
  test.identical( got, [ 1, 2, 3 ] );

  var got = _.arrayRemoveElementOnce( [ 1 ], '1' );
  test.identical( got, [ 1 ] );

  var got = _.arrayRemoveElementOnce( [ 1 ], -1 );
  test.identical( got, [ 1 ] );

  var got = _.arrayRemoveElementOnce( [ 1 ], [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoveElementOnce( dst, { num : 4 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoveElementOnce( dst, { num : 1 }, onEqualize );
  test.identical( got, [ { num : 2 }, { num : 3 } ] );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var onEqualize2 = ( a ) =>
  {
    return a;
  }
  var got = _.arrayRemoveElementOnce( dst, 4, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemoveElementOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 2 }, { num : 3 } ] );

  var dst = [ { num : 1 }, { num : 2 }, { num : 1 }, { num : 3 } ];
  var got = _.arrayRemoveElementOnce( dst, 1, onEqualize, onEqualize2 );
  test.identical( got, [ { num : 2 }, { num : 1 }, { num : 3 } ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnce();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnce( 1, 1 );
  })

  test.case = 'onEqualize is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnce( 1, 1, 1 );
  })
}

//

// function arrayRemoveElementOnce( test ) {
//
//   test.case = 'nothing';
//   var got = _.arrayRemoveElementOnce( [], 2 );
//   var expected = [];
//   test.identical( got, expected );
//
//   test.case = 'one element left';
//   var got = _.arrayRemoveElementOnce( [ 2, 4 ], 4 );
//   var expected = [ 2 ];
//   test.identical( got, expected );
//
//   test.case = 'two elements left';
//   var got = _.arrayRemoveElementOnce( [ true, false, 6 ], true );
//   var expected = [ false, 6 ];
//   test.identical( got, expected );
//
//   /**/
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayRemoveElementOnce();
//   });
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayRemoveElementOnce( [ 2, 4, 6 ] );
//   });
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayRemoveElementOnce( [ 2, 4, 6 ], 2, function( el, ins ) { return el > ins }, 'redundant argument' );
//   });
//
//   test.case = 'arguments[ 0 ] is wrong';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayRemoveElementOnce( 'wrong argument', 2 );
//   });
//
//   test.case = 'arguments[ 2 ] is wrong';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayRemoveElementOnce( [ 2, 4, 6 ], 2, 'wrong argument' );
//   });
//
// };

//

function arrayRemoveElementOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveElementOnceStrictly( dst, 2 );
  test.identical( got, [ 1, 3 ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoveElementOnceStrictly( dst, { num : 3 }, onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemoveElementOnceStrictly( dst, 3, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 1 }, { num : 2 } ] );
  test.true( got === dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnceStrictly( 1, 1 );
  })

  test.case = 'ins doesn´t exist';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnceStrictly( [ 1 ], 2 );
  });

  test.case = 'ins is not unique in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnceStrictly( [ 1, 2, 2 ], 2 );
  });

  test.case = 'onEqualize is not a routine';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveElementOnceStrictly( [ 1, 2, 3 ], 3, 3 );
  });

  test.case = 'onEqualize';
  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];

  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayRemoveElementOnceStrictly( dst, { num : 4 }, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )


  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a ) =>
    {
      return a.num;
    }
    _.arrayRemoveElementOnceStrictly( dst, 4, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )
}

//

function arrayRemovedElement( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemovedElement( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElement( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 1 );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemovedElement( dst, 2 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 2 );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemovedElement( dst, 1 );
  test.identical( dst, [ 2, 2 ] );
  test.identical( got, 1 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElement( dst, '1' );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElement( dst, -1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElement( dst, [ 1 ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemovedElement( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 3 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemovedElement( dst, [ 1 ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 0 );

  var dst = [ { x : 1 } ];
  var got = _.arrayRemovedElement( dst, { x : 1 } );
  test.identical( dst, [ { x : 1 } ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElement( dst, '1' );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElement( dst, -1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElement( dst, [ 1 ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  function onEqualize( a, b )
  {
    return a.value === b;
  }
  var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
  var got = _.arrayRemovedElement( dst, 1, onEqualize );
  test.identical( dst, [ { value : 2 } ] );
  test.identical( got, 2 );

  var src = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayRemovedElement( src, 1, 1 );
  test.identical( got, 1 );
  test.identical( src, [ 1, 2, 3, 2, 3 ] );;

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedElement( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedElement( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 1 );


  test.case = 'evaluator 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var evaluator1 = ( a ) =>
  {
    return a.num;
  }
  var evaluator2 = ( a ) =>
  {
    return a;
  }
  var got = _.arrayRemovedElement( dst, 4, evaluator1 );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemovedElement( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 1 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 1 }, { num : 3 } ];
  var got = _.arrayRemovedElement( dst, 1, evaluator1, evaluator2 );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 2 );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElement();
  })

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElement( [ 1 ], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElement( 1, 1 );
  })

}

//

function arrayRemovedElement_( test )
{
  test.open( 'without evaluators' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElement_( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, full deletion, entry - first element';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 2 );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElement_( dst, 1 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string, without entry';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, '1' );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - array, identical, without entry';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElement_( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ] );
  test.identical( got, undefined );

  test.case = 'dst - array with map, ins - map, identical, without entry';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElement_( dst, { x : 1 } );
  test.identical( dst, [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ] );
  test.identical( got, undefined );

  test.close( 'without evaluators' );

  /* - */

  test.open( 'evaluator1 - fromIndex' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElement_( dst, 1, 1 );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, full deletion, entry - first element';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, 3 );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 2, 3 );
  test.identical( dst, [ 1, 2, 1, 1, 1 ] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, 3 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string, without entry';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, '1', 3 );
  test.identical( dst, [ 1, 1, 1, 1, 1 ] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - array, identical, without entry';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElement_( dst, [ 1 ], 3 );
  test.identical( dst, [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ] );
  test.identical( got, undefined );

  test.case = 'dst - array with map, ins - map, identical, without entry';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElement_( dst, { x : 1 }, 3 );
  test.identical( dst, [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ] );
  test.identical( got, undefined );

  test.close( 'evaluator1 - fromIndex' );

  /* - */

  test.open( 'equalizer' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElement_( dst, 1, ( e, ins ) => e === ins );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, full deletion, entry - first element';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, ( e, ins ) => e === ins );
  test.identical( dst, [] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 2, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, ( e, ins ) => e === ins );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string, without entry';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, '1', ( e, ins ) => e === parseFloat( ins ) );
  test.identical( dst, [] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - array, identical, without entry';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElement_( dst, [ 1 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array with map, ins - map, identical, without entry';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElement_( dst, { x : 1 }, ( e, ins ) => e.x === ins.x );
  test.identical( dst, [ { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'equalizer' );

  /* - */

  test.open( 'single evaluator' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElement_( dst, 1, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, full deletion, entry - first element';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 2, ( e ) => e );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, ( e ) => e );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string, without entry';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, '1', ( e ) => parseFloat( e ) );
  test.identical( dst, [] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - array, identical, without entry';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElement_( dst, [ 1 ], ( e ) => e[ 0 ] );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array with map, ins - map, identical, without entry';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElement_( dst, { x : 1 }, ( e ) => e.x );
  test.identical( dst, [ { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'single evaluator' );

  /* - */

  test.open( 'two evaluators' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElement_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, full deletion, entry - first element';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, 2, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElement_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string, without entry';
  var dst = [ 1, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElement_( dst, '1', ( e ) => e, ( ins ) => parseFloat( ins ) );
  test.identical( dst, [] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - array, identical, without entry';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElement_( dst, [ 1 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array with map, ins - map, identical, without entry';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElement_( dst, { x : 1 }, ( e ) => e.x, ( ins ) => ins.x );
  test.identical( dst, [ { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'two evaluators' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( [], 0, 2, ( e ) => e, ( el ) => el, 'extra' ) );

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( 1, 1 ) );

  test.case = 'wrong type of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( [ 1 ], 1, 1, 'wrong' ) );

  test.case = 'wrong length of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( [ 1 ], 1, 1, () => 'str' ) );
  /* eslint-disable */
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( [ 1 ], 1, 1, ( a, b, c, d ) => a - b + c === d ) );
  /* eslint-enable */

  test.case = 'wrong type of evaluator2';
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( [ 1 ], 1, 1, ( e ) => e, 'wrong' ) );

  test.case = 'wrong length of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( [ 1 ], 1, 1, ( e ) => e, () => 'str' ) );
  test.shouldThrowErrorSync( () => _.arrayRemovedElement_( [ 1 ], 1, 1, ( e ) => e, ( a, b ) => a === b ) );
}

//

function arrayRemovedElementOnce( test )
{
  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemovedElementOnce( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElementOnce( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedElementOnce( dst, 3 );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  var dst = [ 2, 2, 1 ];
  var got = _.arrayRemovedElementOnce( dst, 2 );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 0 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElementOnce( dst, '1' );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElementOnce( dst, -1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  var dst = [ 1 ];
  var got = _.arrayRemovedElementOnce( dst, [ 1 ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, -1 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedElementOnce( dst, { num : 4 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedElementOnce( dst, { num : 1 }, onEqualize );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a ) =>
  {
    return a.num;
  }
  var got = _.arrayRemovedElementOnce( dst, 4, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] );
  test.identical( got, -1 );

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemovedElementOnce( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, 0 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnce();
  })

  test.case = 'fourth is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnce( [], 1, 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnce( 1, 1 );
  })
}

//

function arrayRemovedElementOnce_( test )
{
  test.open( 'without evaluators' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElementOnce_( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 1, 2, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1 );
  test.identical( dst, [ 1, 2, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 2 );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string, without entry';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, '1' );
  test.identical( dst, [ 1, 2, 1, 1, 1 ] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - array, identical, without entry';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElementOnce_( dst, [ 1 ] );
  test.identical( dst, [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ] );
  test.identical( got, undefined );

  test.case = 'dst - array with map, ins - map, identical, without entry';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnce_( dst, { x : 1 } );
  test.identical( dst, [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ] );
  test.identical( got, undefined );

  test.close( 'without evaluators' );

  /* - */

  test.open( 'evaluator1 - fromIndex' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElementOnce_( dst, 1, 1 );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 1, 2, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, 3 );
  test.identical( dst, [ 1, 1, 2, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 2, 3 );
  test.identical( dst, [ 1, 2, 1, 1, 1 ] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, 3 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string, without entry';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, '1', 3 );
  test.identical( dst, [ 1, 2, 1, 1, 1 ] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - array, identical, without entry';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElementOnce_( dst, [ 1 ], 3 );
  test.identical( dst, [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ] );
  test.identical( got, undefined );

  test.case = 'dst - array with map, ins - map, identical, without entry';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnce_( dst, { x : 1 }, 3 );
  test.identical( dst, [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ] );
  test.identical( got, undefined );

  test.close( 'evaluator1 - fromIndex' );

  /* - */

  test.open( 'equalizer' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e, ins ) => e === ins );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 1, 2, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, 2, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 2, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e, ins ) => e === ins );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, '1', ( e, ins ) => e === parseFloat( ins ) );
  test.identical( dst, [ 2, 1, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - array, identical';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElementOnce_( dst, [ 1 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  test.identical( dst, [ 1, [ 1, 0 ], 1, [ 1, 0 ] ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array with map, ins - map, identical';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnce_( dst, { x : 1 }, ( e, ins ) => e.x === ins.x );
  test.identical( dst, [ { x : 1, y : 1 }, { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'equalizer' );

  /* - */

  test.open( 'single evaluator' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 1, 2, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e ) => e );
  test.identical( dst, [ 1, 2, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 2, ( e ) => e );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e ) => e );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, '1', ( e ) => parseFloat( e ) );
  test.identical( dst, [ 2, 1, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - array, identical';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElementOnce_( dst, [ 1 ], ( e ) => e[ 0 ] );
  test.identical( dst, [ 1, [ 1, 0 ], 1, [ 1, 0 ] ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array with map, ins - map, identical';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnce_( dst, { x : 1 }, ( e ) => e.x );
  test.identical( dst, [ { x : 1, y : 1 }, { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'single evaluator' );

  /* - */

  test.open( 'two evaluators' );

  test.case = 'dst - empty array, ins - number, without entry';
  var dst = [];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 1, 2, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 2, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnce_( dst, '1', ( e ) => e, ( ins ) => parseFloat( ins ) );
  test.identical( dst, [ 2, 1, 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - array, identical';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 1, 0 ] ];
  var got = _.arrayRemovedElementOnce_( dst, [ 1 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  test.identical( dst, [ 1, [ 1, 0 ], 1, [ 1, 0 ] ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array with map, ins - map, identical';
  var dst = [ { x : 1 }, { x : 1, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnce_( dst, { x : 1 }, ( e ) => e.x, ( ins ) => ins.x );
  test.identical( dst, [ { x : 1, y : 1 }, { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'two evaluators' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( [], 0, 2, ( e ) => e, ( el ) => el, 'extra' ) );

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( 1, 1 ) );

  test.case = 'wrong type of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( [ 1 ], 1, 1, 'wrong' ) );

  test.case = 'wrong length of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( [ 1 ], 1, 1, () => 'str' ) );
  /* eslint-disable */
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( [ 1 ], 1, 1, ( a, b, c, d ) => a - b + c === d ) );
  /* eslint-enable */

  test.case = 'wrong type of evaluator2';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( [ 1 ], 1, 1, ( e ) => e, 'wrong' ) );

  test.case = 'wrong length of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( [ 1 ], 1, 1, ( e ) => e, () => 'str' ) );
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnce_( [ 1 ], 1, 1, ( e ) => e, ( a, b ) => a === b ) );

}

//

function arrayRemovedElementOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1 ];
  var got = _.arrayRemovedElementOnceStrictly( dst, 1 );
  test.identical( dst, [] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedElementOnceStrictly( dst, 2 );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedElementOnceStrictly( dst, { num : 3 }, onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 } ] );
  test.identical( got, { num : 3 } );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemovedElementOnceStrictly( dst, 1, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 }, { num : 3 } ] );
  test.identical( got, { num : 1 } );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly();
  })

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [], 1, 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( 1, 1 );
  })

  test.case = 'Simple no match element';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [], 1 );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [ 1 ], '1' );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [ 1 ], - 1 );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [ 1 ], [ 1 ] );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [ 1 ], 2 );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [ 1 ], - 1 );
  })

  test.case = 'Ins several times in srcArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( [ 2, 2, 1 ], 2 );
  })

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedElementOnceStrictly( dst, { num : 4 }, onEqualize );
  })

}

//

function arrayRemovedElementOnceStrictly_( test )
{
  test.open( 'without evaluators' );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 2, 2, 2, 2 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 2, 2, 1, 2, 2 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.close( 'without evaluators' );

  /* - */

  test.open( 'evaluator1 - fromIndex' );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 2, 2, 2, 2 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, 0 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 2, 1, 2 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, 3 );
  test.identical( dst, [ 1, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, 3 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.close( 'evaluator1 - fromIndex' );

  /* - */

  test.open( 'equalizer' );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 2, 1, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 2, ( e, ins ) => e === ins - 1 );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, '1', ( e, ins ) => e === parseFloat( ins ) + 1 );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - array, identical';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 2, 0 ] ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, [ 1 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] + 1 );
  test.identical( dst, [ [ 1 ], 1, [ 1, 0 ], 1 ] );
  test.identical( got, [ 2, 0 ] );

  test.case = 'dst - array with map, ins - map, identical';
  var dst = [ { x : 1 }, { x : 2, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, { x : 1 }, ( e, ins ) => e.x === ins.x );
  test.identical( dst, [ { x : 2, y : 1 }, { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'equalizer' );

  /* - */

  test.open( 'single evaluator' );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 2, 2, 2, 2 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, ( e ) => e );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 2, ( e ) => e );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, ( e ) => e );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, '2', ( e ) => parseFloat( e ) );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - array, identical';
  var dst = [ [ 1 ], 1, [ 2, 0 ], 1, [ 2, 0 ] ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, [ 1 ], ( e ) => e[ 0 ] );
  test.identical( dst, [ 1, [ 2, 0 ], 1, [ 2, 0 ] ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array with map, ins - map, identical';
  var dst = [ { x : 1 }, { x : 2, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, { x : 1 }, ( e ) => e.x );
  test.identical( dst, [ { x : 2, y : 1 }, { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'single evaluator' );

  /* - */

  test.open( 'two evaluators' );

  test.case = 'dst - filled array, ins - number, entry - first element';
  var dst = [ 1, 2, 2, 2, 2 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - number, entry - at the middle';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - number, entry - last element';
  var dst = [ 2, 2, 2, 2, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, 1, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 2, 2, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - filled array, ins - string';
  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, '2', ( e ) => e, ( ins ) => parseFloat( ins ) );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - filled array, ins - array, identical';
  var dst = [ [ 1 ], 1, [ 1, 0 ], 1, [ 2, 0 ] ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, [ 1 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] + 1 );
  test.identical( dst, [ [ 1 ], 1, [ 1, 0 ], 1 ] );
  test.identical( got, [ 2, 0 ] );

  test.case = 'dst - array with map, ins - map, identical';
  var dst = [ { x : 1 }, { x : 2, y : 1 }, { y : 2 } ];
  var got = _.arrayRemovedElementOnceStrictly_( dst, { x : 1 }, ( e ) => e.x, ( ins ) => ins.x );
  test.identical( dst, [ { x : 2, y : 1 }, { y : 2 } ] );
  test.identical( got, { x : 1 } );

  test.close( 'two evaluators' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 2, 1, 0 ], 0, 2, ( e ) => e, ( el ) => el, 'extra' ) );

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( 1, 1 ) );

  test.case = 'empty dst, without entry';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [], 1 ) );
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [], 1, ( e, ins ) => e === ins ) );

  test.case = 'dst has a few entries';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 1, 1 ], 1 ) );

  test.case = 'wrong type of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 1 ], 1, 0, 'wrong' ) );

  test.case = 'wrong length of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 1 ], 1, 0, () => 'str' ) );
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 1 ], 1, 0, ( a, b, c ) => a === b - c ) );

  test.case = 'wrong type of evaluator2';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 1 ], 1, 0, ( e ) => e, 'wrong' ) );

  test.case = 'wrong length of evaluator1';
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 1 ], 1, 0, ( e ) => e, () => 'str' ) );
  test.shouldThrowErrorSync( () => _.arrayRemovedElementOnceStrictly_( [ 1 ], 1, 0, ( e ) => e, ( a, b ) => a === b ) );
}

//

function arrayRemoveArray( test )
{

  test.case = 'nothing';
  var got = _.arrayRemoveArray( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayRemoveArray( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArray( dst, [ 4, 5 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArray( dst, [ 1, 3 ] );
  test.identical( dst, [ 2 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArray( dst, [ 1, 1 ] );
  test.identical( dst, [] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArray( dst, [ 1 ] );
  test.identical( dst, [] );
  test.true( got === dst );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayRemoveArray( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1 ] );
  test.true( got === dst );

  test.case = 'dstc === src';
  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArray( dst, dst );
  test.identical( dst, [] );
  test.true( got === dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'argument is undefined';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArray( [ 1 ], undefined );
  });

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArray();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArray( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArray( [ 1, 2 ], 2 );
  });
};

//

function arrayRemoveArrayOnce( test )
{
  test.case = 'nothing';

  var got = _.arrayRemoveArrayOnce( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemoveArrayOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.true( got === dst );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArrayOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArrayOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1 ] );
  test.true( got === dst );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  test.mustNotThrowError( function ()
  {
    var got = _.arrayRemoveArrayOnce( dst, [ undefined, 2 ] );
    test.identical( dst, [ 1 ] );
    test.true( got === dst );
  });

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnce( dst, dst );
  test.identical( got, [] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnce( dst, dst, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnce( dst, dst, ( e ) => e + 10 );
  test.identical( got, [] );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src with evaluators, nothing to delete';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( got, [ 1, 1, 2, 2, 3, 3 ] );

  /* */

  if( !Config.debug )
  return;


  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnce();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnce( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnce( [ 1, 2 ], 2 );
  });

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayRemoveArrayOnce( [ 1, 2 ], [ 2 ], 3 );
  // });
}

//

function arrayRemoveArrayOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, [ 2 ] );
  test.identical( got, [ 1, 3 ] );
  test.true( got === dst );

  test.case = 'ins has several values';

  var dst = [ 1, 2, 3, 4, 5, 6, 6 ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, [ 1, 3, 5 ] );
  test.identical( got, [ 2, 4, 6, 6 ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemoveArrayOnceStrictly( dst, [ { num : 3 } ], onEqualize );
  test.identical( got, [ { num : 1 }, { num : 2 } ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, [ 3 ], ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 1 }, { num : 2 } ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args - ins several values';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, [ 3, 1 ], ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 2 } ] );
  test.true( got === dst );

  test.case = 'dst === src'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, dst );
  test.identical( got, [] );

  test.case = 'dst === src with single evaluator'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, dst, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src with single evaluator'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, dst, ( e ) => e + 10 );
  test.identical( got, [] );

  test.case = 'dst === src with two evaluators'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayRemoveArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got1 = _.arrayRemoveArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
    test.identical( got1, [ 1, 1, 2, 2, 3, 3 ] );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnceStrictly( 1, 1 );
  })

  test.case = 'ins not exists';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnceStrictly( [ 1 ], [ 2 ] );
  });

  test.case = 'ins repeated in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnceStrictly( [ 1, 2, 2 ], [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnceStrictly( [ 1, 2, 3 ], 3, 3 );
  });

  test.case = 'onEqualize';
  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];

  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayRemoveArrayOnceStrictly( dst, [ { num : 4 } ], onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )


  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a ) =>
    {
      return a.num;
    }
    _.arrayRemoveArrayOnceStrictly( dst, [ 4 ], onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  });
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
}

//

function arrayRemovedArray( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayRemovedArray( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemovedArray( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedArray( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemovedArray( dst, [ 1 ] );
  test.identical( dst, [] );
  test.identical( got, 3 );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got;
  var got = _.arrayRemovedArray( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ]
  var got = _.arrayRemovedArray( dst, dst );
  test.identical( dst, [] );
  test.identical( got, 3 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'argument is undefined';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArray( [ 1 ], undefined );
  });

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArray();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArray( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArray( [ 1, 2 ], 2 );
  });
}

//

function arrayRemovedArrayOnce( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayRemovedArrayOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemovedArrayOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedArrayOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemovedArrayOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, 1 );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  test.mustNotThrowError( function ()
  {
    var got = _.arrayRemovedArrayOnce( dst, [ undefined, 2 ] );
    test.identical( dst, [ 1 ] );
    test.identical( got, 0 );
  });

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnce( dst, dst );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnce( dst, dst, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, nothing to delete';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnce();
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnce( [ 1, 2 ], [ 1 ], [ 2 ] );
  });

  test.case = 'second args is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnce( [ 1, 2 ], 2 );
  });

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayRemovedArrayOnce( [ 1, 2 ], [ 2 ], 3 );
  // });

}

//

function arrayRemovedArrayOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, [ 2 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  test.case = 'ins has several values';

  var dst = [ 1, 2, 3, 4, 5, 6, 6 ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, [ 1, 3, 5 ] );
  test.identical( dst, [ 2, 4, 6, 6 ] );
  test.identical( got, 3 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var got = _.arrayRemovedArrayOnceStrictly( dst, [ { num : 3 } ], onEqualize );
  test.identical( dst, [ { num : 1 }, { num : 2 } ] );
  test.identical( got, 1 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, [ 3 ], ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 1 }, { num : 2 } ] );
  test.identical( got, 1 );

  test.case = 'equalizer 2 args - ins several values';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, [ 3, 1 ], ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 } ] );
  test.identical( got, 2 );

  test.case = 'dst === src'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, dst );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with single evaluator'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, dst, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with single evaluator'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with two evaluators'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayRemovedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got1 = _.arrayRemovedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
    test.identical( got1, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnceStrictly( 1, 1 );
  })

  test.case = 'ins not exists';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnceStrictly( [ 1 ], [ 2 ] );
  });

  test.case = 'ins repeated in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnceStrictly( [ 1, 2, 2 ], [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnceStrictly( [ 1, 2, 3 ], 3, 3 );
  });

  test.case = 'onEqualize';
  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];

  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    _.arrayRemovedArrayOnceStrictly( dst, [ { num : 4 } ], onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )


  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a ) =>
    {
      return a.num;
    }
    _.arrayRemovedArrayOnceStrictly( dst, [ 4 ], onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrayOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  });
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );
}

//

function arrayRemoveArrays( test )
{
  test.case = 'nothing';
  var got = _.arrayRemoveArrays( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';
  var dst = [];
  var got = _.arrayRemoveArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArrays( dst, [ 4, 5 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArrays( dst, [ 1, 3 ] );
  test.identical( dst, [ 2 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1, 2, 2, 2 ];
  var got = _.arrayRemoveArrays( dst, [ [ 1 ], [ 2 ] ] );
  test.identical( dst, [] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], [ 2, [ 3 ] ], [ [ [ 4 ] ], 5 ] ];
  var got = _.arrayRemoveArrays( dst, insArray );
  test.identical( dst, [ 3, 4 ] );
  test.true( got === dst );

  var dst = [ 5 ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemoveArrays( dst, insArray );
  test.identical( dst, [] );
  test.true( got === dst );

  var dst = [ [ 5 ] ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemoveArrays( dst, insArray );
  test.identical( dst, [ [ 5 ] ] );
  test.true( got === dst );

  var dst = [ [ 5 ] ];
  var insArray = [ [ [ 5 ] ] ];
  var got = _.arrayRemoveArrays( dst, insArray );
  test.identical( dst, [ [ 5 ] ] );
  test.true( got === dst );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayRemoveArrays( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1 ] );
  test.true( got === dst );

  test.case = 'dst === ins';
  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArrays( dst, dst );
  test.identical( dst, [] );
  test.true( got === dst );

  test.case = 'dst === ins';
  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArrays( dst, [ dst ] );
  test.identical( dst, [] );
  test.true( got === dst );

  test.case = 'dst === ins';
  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArrays( dst, [ dst, dst ] );
  test.identical( dst, [] );
  test.true( got === dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArrays();
  });

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemoveArrays( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'second arg is not longIs entity';
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemoveArrays( [], 1 );
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemoveArrays( [], [ 1 ], [ 1 ] );
  });

};

//

function arrayRemoveArraysOnce( test )
{
  test.case = 'nothing';

  var got = _.arrayRemoveArraysOnce( [], [] );
  var expected = [];
  test.identical( got, expected );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemoveArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemoveArraysOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 2, 3, 3, 4, 5, 5 ];
  var insArray = [ [ 1 ], [ 2, [ 3 ] ], [ [ [ 4 ] ], 5 ] ];
  var got = _.arrayRemoveArraysOnce( dst, insArray );
  test.identical( got, [ 1, 3, 3, 4, 5 ] );
  test.true( got === dst );

  var dst = [ 1, 1, 2, 2, 3, 4, 4, 5 ];
  var insArray = [ [ 1, 1 ], 2, [ 3 ], 4, 4, [ 5 ] ];
  var got = _.arrayRemoveArraysOnce( dst, insArray );
  test.identical( dst, [ 2 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], [ 2, [ 3 ] ], [ [ [ 4 ] ], 5 ] ];
  var got = _.arrayRemoveArraysOnce( dst, insArray );
  test.identical( got, [ 3, 4 ] );
  test.true( got === dst );

  var dst = [ 5, 5 ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemoveArraysOnce( dst, insArray );
  test.identical( got, [ 5 ] );
  test.true( got === dst );

  var dst = [ [ 5 ], [ 5 ] ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemoveArraysOnce( dst, insArray );
  test.identical( dst, [ [ 5 ], [ 5 ] ] );
  test.true( got === dst );

  var dst = [ [ 5 ], [ 5 ] ];
  var insArray = [ [ [ 5 ] ] ];
  var got = _.arrayRemoveArraysOnce( dst, insArray );
  test.identical( dst, [ [ 5 ], [ 5 ] ] );
  test.true( got === dst );

  function onEqualize( a, b ){ return a === b }
  var dst = [ 1, 2, [ 3 ] ];
  var insArray = [ 1, 2, [ 3 ] ];
  var got = _.arrayRemoveArraysOnce( dst, insArray, onEqualize );
  test.identical( got, [ [ 3 ] ] );
  test.true( got === dst );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  test.mustNotThrowError( function ()
  {
    var got = _.arrayRemoveArraysOnce( dst, [ undefined, 2 ] );
    test.identical( dst, [ 1 ] );
    test.true( got === dst );
  });

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var insArray = [ [ { num : 3 } ], { num : 1 } ];
  var got = _.arrayRemoveArraysOnce( dst, insArray, onEqualize )
  test.identical( got, [ { num : 2 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var insArray = [ [ 3 ], 1 ];
  var got = _.arrayRemoveArraysOnce( dst, insArray, ( e ) => e.num, ( e ) => e )
  test.identical( got, [ { num : 2 } ] );
  test.true( got === dst );

  /*  */

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, dst );
  test.identical( got, [] );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, [ dst ] );
  test.identical( got, [] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, dst, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, [ dst ], ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, dst, ( e ) => e + 10 );
  test.identical( got, [] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, [ dst ], ( e ) => e + 10 );
  test.identical( got, [] );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src with evaluators, nothing to delete';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( got, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, nothing to delete';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, [ 1, 1, 2, 2, 3, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnce();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnce( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnce( [], [ 1, 2, 3 ], [] )
  });

  test.case = 'second arg is not longIs entity';
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemoveArraysOnce( [], 1 );
  });
}

//

function arrayRemoveArraysOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, [ 2 ] );
  test.identical( got, [ 1, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( got, [] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3 ], [ 4 ] ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, insArray );
  test.identical( got, [] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], [ 2, 3 ], [ 4 ], 5 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, insArray );
  test.identical( got, [] );
  test.true( got === dst );

  var dst = [ 5, 6, 7, 8 ];
  var insArray = [ [ 5, 6 ], 7 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, insArray );
  test.identical( got, [ 8 ] );
  test.true( got === dst );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var insArray = [ [ { num : 3 } ], { num : 1 } ]
  var got = _.arrayRemoveArraysOnceStrictly( dst, insArray, onEqualize )
  test.identical( got, [ { num : 2 } ] );
  test.true( got === dst );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var insArray = [ [ 3 ], 1 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, insArray, ( e ) => e.num, ( e ) => e );
  test.identical( got, [ { num : 2 } ] );
  test.true( got === dst );

  /*  */

  test.case = 'dst === src'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, dst );
  test.identical( got, [] );

  test.case = 'dst === src with single evaluator'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, dst, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src with single evaluator'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, dst, ( e ) => e + 10 );
  test.identical( got, [] );

  test.case = 'dst === src with two evaluators'

  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemoveArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( got, [] );

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayRemoveArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 ) );
  else
  _.arrayRemoveArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnceStrictly( 1, 1 );
  })

  test.case = 'ins not exists';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnceStrictly( [ 1 ], [ 2 ] );
  });

  test.case = 'ins repeated in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnceStrictly( [ 1, 2, 2 ], [ [ 2 ] ] );
  });

  test.case = 'ins element repeated';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnceStrictly( [ 1, 2, 3, 4, 5 ], [ [ 2, 3 ], 2 ] );
  });

  test.case = 'onEqualize is not a routine';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveArraysOnceStrictly( [], [ 1, 2, 3 ], [] );
  });

  test.case = 'onEqualize';
  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];

  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    var insArray = [ [ { num : 4 } ] ];
    _.arrayRemoveArraysOnceStrictly( dst, insArray, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )

  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a ) =>
    {
      return a.num;
    }
    var insArray = [ [ 4 ] ];
    _.arrayRemoveArraysOnceStrictly( dst, insArray, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )
}

//

function arrayRemovedArrays( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayRemovedArrays( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemovedArrays( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedArrays( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemovedArrays( dst, [ 1 ] );
  test.identical( dst, [] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], [ 2, [ 3 ] ], [ [ [ 4 ] ], 5 ] ];
  var got = _.arrayRemovedArrays( dst, insArray );
  test.identical( dst, [ 3, 4 ] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], 2, [ 3 ], 4, [ 5 ] ]
  var got = _.arrayRemovedArrays( dst, insArray );
  test.identical( dst, [] );
  test.identical( got, 5 );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], [ 2, [ 3 ] ], [ [ [ 4 ] ], 5 ] ];
  var got = _.arrayRemovedArrays( dst, insArray );
  test.identical( dst, [ 3, 4 ] );
  test.identical( got, 3 );

  var dst = [ 5 ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemovedArrays( dst, insArray );
  test.identical( dst, [] );
  test.identical( got, 1 );

  var dst = [ [ 5 ] ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemovedArrays( dst, insArray );
  test.identical( dst, [ [ 5 ] ] );
  test.identical( got, 0 );

  var dst = [ [ 5 ] ];
  var insArray = [ [ [ 5 ] ] ];
  var got = _.arrayRemovedArrays( dst, insArray );
  test.identical( dst, [ [ 5 ] ] );
  test.identical( got, 0 );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  var got = _.arrayRemovedArrays( dst, [ undefined, 2 ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ]
  var got = _.arrayRemovedArrays( dst, dst );
  test.identical( dst, [] );
  test.identical( got, 3 );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ]
  var got = _.arrayRemovedArrays( dst, [ dst ] );
  test.identical( dst, [] );
  test.identical( got, 3 );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ]
  var got = _.arrayRemovedArrays( dst, [ dst, dst ] );
  test.identical( dst, [] );
  test.identical( got, 3 );


  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArrays();
  });

  test.case = 'argument is undefined';
  var dst = [ 1 ];
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemovedArrays( dst, undefined );
  });
  test.identical( dst, [ 1 ] );

  test.case = 'second arg is not longIs entity';
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemovedArrays( [], 1 );
  });

  test.case = 'too many args';
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemovedArrays( [], [ 1 ], [ 1 ] );
  });
}

//

function arrayRemovedArraysOnce( test )
{
  test.case = 'nothing';

  var dst = [];
  var got = _.arrayRemovedArraysOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'simple';

  var dst = [];
  var got = _.arrayRemovedArraysOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'prepends only unique elements';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, [ 2, 4, 5 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayRemovedArraysOnce( dst, [ 1 ] );
  test.identical( dst, [ 1, 1 ] );
  test.identical( got, 1 );

  var dst = [ 1, 1, 2, 3, 3, 4, 5, 5 ];
  var insArray = [ [ 1 ], [ 2, [ 3 ] ], [ [ [ 4 ] ], 5 ] ]
  var got = _.arrayRemovedArraysOnce( dst, insArray );
  test.identical( dst, [ 1, 3, 3, 4, 5 ] );
  test.identical( got, 3 );

  var dst = [ 1, 1, 2, 2, 3, 4, 4, 5 ];
  var insArray = [ [ 1, 1 ], 2, [ 3 ], 4, 4, [ 5 ] ];
  var got = _.arrayRemovedArraysOnce( dst, insArray );
  test.identical( dst, [ 2 ] );
  test.identical( got, 7 );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], [ 2, [ 3 ] ], [ [ [ 4 ] ], 5 ] ];
  var got = _.arrayRemovedArraysOnce( dst, insArray );
  test.identical( dst, [ 3, 4 ] );
  test.identical( got, 3 );

  var dst = [ 5, 5 ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemovedArraysOnce( dst, insArray );
  test.identical( dst, [ 5 ] );
  test.identical( got, 1 );

  var dst = [ [ 5 ], [ 5 ] ];
  var insArray = [ [ 5 ] ];
  var got = _.arrayRemovedArraysOnce( dst, insArray );
  test.identical( dst, [ [ 5 ], [ 5 ] ] );
  test.identical( got, 0 );

  var dst = [ [ 5 ], [ 5 ] ];
  var insArray = [ [ [ 5 ] ] ];
  var got = _.arrayRemovedArraysOnce( dst, insArray );
  test.identical( dst, [ [ 5 ], [ 5 ] ] );
  test.identical( got, 0 );

  function onEqualize( a, b ){ return a === b }
  var dst = [ 1, 2, [ 3 ] ];
  var insArray = [ 1, 2, [ 3 ] ];
  var got = _.arrayRemovedArraysOnce( dst, insArray, onEqualize );
  test.identical( dst, [ [ 3 ] ] );
  test.identical( got, 2 );

  test.case = 'array has undefined';
  var dst = [ 1 ];
  test.mustNotThrowError( function ()
  {
    var got = _.arrayRemovedArraysOnce( dst, [ undefined, 2 ] );
    test.identical( dst, [ 1 ] );
    test.identical( got, 0 );
  });

  /*  */

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, dst );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, [ dst ] );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, dst, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, [ dst ], ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, [ dst ], ( e ) => e + 10 );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, nothing to delete';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, dst, ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with evaluators, nothing to delete';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnce( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnce();
  });

  test.case = 'dst is not a array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnce( 1, [ 2 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnce( [], [ 1, 2, 3 ], [] )
  });

  test.case = 'second arg is not longIs entity';
  test.shouldThrowErrorSync( function ()
  {
    _.arrayRemovedArraysOnce( [], 1 );
  });
}

//

function arrayRemovedArraysOnceStrictly( test )
{
  test.case = 'simple';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, [ 2 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 3, 4 ];
  var insArray = [ [ 1 ], [ 2 ], [ 3 ], [ 4 ] ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, insArray );
  test.identical( dst, [] );
  test.identical( got, 4 );

  var dst = [ 1, 2, 3, 4, 5 ];
  var insArray = [ [ 1 ], [ 2, 3 ], [ 4 ], 5 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, insArray );
  test.identical( dst, [] );
  test.identical( got, 5 );

  var dst = [ 5, 6, 7, 8 ];
  var insArray = [ [ 5, 6 ], 7 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, insArray );
  test.identical( dst, [ 8 ] );
  test.identical( got, 3 );

  test.case = 'equalizer 2 args';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var onEqualize = ( a, b ) =>
  {
    return a.num === b.num;
  }
  var insArray = [ [ { num : 3 } ], { num : 1 } ]
  var got = _.arrayRemovedArraysOnceStrictly( dst, insArray, onEqualize )
  test.identical( dst, [ { num : 2 } ] );
  test.identical( got, 2 );

  test.case = 'equalizer 1 arg';

  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];
  var insArray = [ [ 3 ], 1 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, insArray, ( e ) => e.num, ( e ) => e );
  test.identical( dst, [ { num : 2 } ] );
  test.identical( got, 2 );

  /*  */

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, dst );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, [ dst ] );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with single evaluator'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, dst, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with single evaluator'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, [ dst ], ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with single evaluator'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, dst, ( e ) => e + 10 );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with single evaluator'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, [ dst ], ( e ) => e + 10 );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayRemovedArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e );
  test.identical( dst, [] );
  test.identical( got, 6 );

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayRemovedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got1 = _.arrayRemovedArraysOnceStrictly( dst, dst, ( e ) => e, ( e ) => e + 10 );
    test.identical( got1, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src with two evaluators'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayRemovedArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got2 = _.arrayRemovedArraysOnceStrictly( dst, [ dst ], ( e ) => e, ( e ) => e + 10 );
    test.identical( got2, 0 );
  }
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnceStrictly();
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnceStrictly( 1, 1 );
  })

  test.case = 'ins not exists';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnceStrictly( [ 1 ], [ 2 ] );
  });

  test.case = 'ins repeated in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnceStrictly( [ 1, 2, 2 ], [ [ 2 ] ] );
  });

  test.case = 'ins element repeated';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnceStrictly( [ 1, 2, 3, 4, 5 ], [ [ 2, 3 ], 2 ] );
  });

  test.case = 'onEqualize is not a routine';

  test.shouldThrowErrorSync( function()
  {
    _.arrayRemovedArraysOnceStrictly( [], [ 1, 2, 3 ], [] );
  });

  test.case = 'onEqualize';
  var dst = [ { num : 1 }, { num : 2 }, { num : 3 } ];

  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a, b ) =>
    {
      return a.num === b.num;
    }
    var insArray = [ [ { num : 4 } ] ];
    _.arrayRemovedArraysOnceStrictly( dst, insArray, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )

  test.shouldThrowErrorSync( function()
  {
    var onEqualize = ( a ) =>
    {
      return a.num;
    }
    var insArray = [ [ 4 ] ];
    _.arrayRemovedArraysOnceStrictly( dst, insArray, onEqualize );
  });
  test.identical( dst, [ { num : 1 }, { num : 2 }, { num : 3 } ] )
}

//

function arrayRemoveDuplicates( test )
{
  test.case = 'empty';

  var dst = [];
  var got = _.arrayRemoveDuplicates( dst );
  var expected = [];
  test.identical( dst, expected );
  test.identical( got, expected );

  test.case = 'No duplicates - One element';

  var dst = [ 1 ];
  var got = _.arrayRemoveDuplicates( dst );
  var expected = [ 1 ];
  test.identical( dst, expected );
  test.identical( got, expected );

  test.case = 'No duplicates - Several elements';

  var dst = [ 1, 2, 3, '4', '5' ];
  var got = _.arrayRemoveDuplicates( dst );
  var expected = [ 1, 2, 3, '4', '5' ];
  test.identical( dst, expected );
  test.identical( got, expected );

  test.case = 'One duplicated element';

  var dst = [ 1, 2, 2 ];
  var got = _.arrayRemoveDuplicates( dst );
  var expected = [ 1, 2 ];
  test.identical( dst, expected );
  test.identical( got, expected );

  test.case = 'One duplicated element - Several elements';

  var dst = [ 1, 2, 1, 1, 1 ];
  var got = _.arrayRemoveDuplicates( dst );
  var expected = [ 1, 2 ];
  test.identical( dst, expected );
  test.identical( got, expected );

  test.case = 'Several duplicates several times';

  var dst = [ 1, 2, 3, '4', '4', 1, 2, 1, 5 ];
  var got = _.arrayRemoveDuplicates( dst );
  var expected = [ 1, 2, 3, '4', 5 ];
  test.identical( dst, expected );
  test.identical( got, expected );

  test.case = 'onEqualize';
  var dst = [ 1, 2, 3, '4', '4', 1, 2, 1, 5 ];

  var got  = _.arrayRemoveDuplicates( dst, function( a, b )
  {
    return a === b;
  });
  var expected = [ 1, 2, 3, '4', 5 ];
  test.identical( dst, expected );
  test.identical( got, expected );

  test.case = 'Evaluator';
  var dst = [ { 'num' : 0 }, { 'num' : 1 }, { 'num' : 2 }, { 'num' : 0 } ];

  var got  = _.arrayRemoveDuplicates( dst, function( a )
  {
    return a.num;
  });
  var expected = [ { 'num' : 0 }, { 'num' : 1 }, { 'num' : 2 } ];
  test.identical( dst, expected );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveDuplicates();
  })

  // test.case = 'more than two args';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayRemoveDuplicates( [ 1 ], 1, 1 );
  // })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveDuplicates( 1 );
  })

  test.case = 'dst is not an array';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveDuplicates( new U8x( [ 1, 2, 3, 4, 5 ] ) );
  })

  test.case = 'second arg is not a function';
  test.shouldThrowErrorSync( function()
  {
    _.arrayRemoveDuplicates( 1, 1 );
  })
}

//

function arrayFlatten( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlatten( [ 0, 1, 0, 1 ] )
  var expected = [ 0, 1, 0, 1 ];
  test.identical( got, expected );

  test.case = 'level 2';
  var got = _.arrayFlatten( [ [ 0 ], 0, 1, [ 0, 1 ] ] );
  var expected = [ 0, 0, 1, 0, 1 ];
  test.identical( got, expected );

  test.case = 'level 3';
  var got = _.arrayFlatten( [ [ [ 0 ] ] ] );
  var expected = [ 0 ];
  test.identical( got, expected );

  test.case = 'from arrays level 5';
  var src = [ [ [ [ [ 1 ] ] ] ] ];
  var dst = [ src, src, src, src ];
  var got = _.arrayFlatten( dst );
  var expected = [ 1, 1, 1, 1 ];
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - null, src - empty array';
  var got  = _.arrayFlatten( null, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - empty array';
  var got  = _.arrayFlatten( [], [] );
  test.identical( got, [] );

  test.case = 'dst - null, src - flat array';
  var got  = _.arrayFlatten( null, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - flat array';
  var got  = _.arrayFlatten( [], [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  var got = _.arrayFlatten( [], [ 'str', {}, [ 1, 2 ], 5, true ] );
  test.identical( got, [ 'str', {}, 1, 2, 5, true ] );

  var got = _.arrayFlatten( [ 1, 2, 3 ], [ 13, 'abc', null, undefined ] );
  test.identical( got, [ 1, 2, 3, 13, 'abc', null, undefined ] );

  test.case = 'dst - empty array, src - array, level 2';
  var got  = _.arrayFlatten( [], [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 3';
  var got  = _.arrayFlatten( [], [ [ 1, [ 2, [ 3 ] ] ] ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 5';
  var got  = _.arrayFlatten( [], [ [ [ [ [ 1 ] ] ] ] ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - flat array, src - flat array';
  var got  = _.arrayFlatten( [ 1, 2, 3 ], [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst - flat array, src - array, level 2';
  var got  = _.arrayFlatten( [ 1, 2, 3 ], [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst - flat array, src - array, level 5';
  var got  = _.arrayFlatten( [ 1 ], [ [ [ [ [ 1 ] ] ] ] ] );
  test.identical( got, [ 1, 1 ] );

  test.case = 'dst - empty array, src - number';
  var got = _.arrayFlatten( [], 1 );
  test.identical( got, [ 1 ] );

  test.case = 'dst - empty array, src - map';
  var got = _.arrayFlatten( [ 1, 2, 3 ], { k : 'e' } );
  test.identical( got, [ 1, 2, 3, { k : 'e' } ] );

  test.case = 'dst - empty array, src - array contains array with diff levels';
  var got  = _.arrayFlatten( [], [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( got, [ 1, 2, 3, 4 ] );

  test.case = 'dst - null, src - array, level 2';
  var got = _.arrayFlatten( null, [ 1, 1, 3, 3, [ 5, 5 ] ] );
  var expected = [ 1, 1, 3, 3, 5, 5 ];
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlatten() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlatten( [], [ 1, 2, 3 ], [ 'extra' ] ) );

  test.case = 'wrong type of dstArray';
  test.shouldThrowErrorSync( () => _.arrayFlatten( undefined, [ 1, 2, 3 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlatten( new U32x([ 1, 2, 3 ]), [ 1, 2, 3 ] ) );
}

//

function arrayFlattenSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - flat array, src - flat array';
  var dst = [ 1, 2, 3 ];
  var src = dst;
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3, 1, 2, 3 ] );

  test.case = 'dst - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var src = dst;
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ 3 ] ] ] ];
  var src = [ dst, 4, 5 ];
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3, 4, 5 ] );
  test.identical( got, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3, 4, 5 ] );

  test.case = 'dst - array, level 5, src contains a few dst';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ]
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'src push self';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'src push self twice';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2 ] );
  src.add( src );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2 ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst push self';
  var dst = [ 1 ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst push self twice';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst push self, src - Set';
  var dst = [ 1 ];
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 'str', { a : 2 } ] );
  test.identical( got, [ 1, 'str', { a : 2 } ] );

  test.case = 'dst push self twice, src - Set';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 'str', { a : 2 } ] );
  test.identical( got, [ 1, 'str', { a : 2 } ] );

  test.case = 'dst push self, dst === src';
  var dst = [ 1 ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, [ 1, 1, 1 ] );

  test.case = 'dst push self twice, dst === src';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, [ 1, 1, 1, 1 ] );

  test.case = 'dst push self';
  var dst = [ 1 ];
  dst.push( dst );
  var got  = _.arrayFlatten( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst push self twice';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlatten( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  /* */

  test.case = 'src inserts self';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 1, 0, dst );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'src inserts self twice';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 0, 0, dst );
  src.splice( 2, 0, dst );
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst inserts self';
  var dst = [ 1, 2 ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2, 2 ] );
  test.identical( got, [ 1, 2, 2 ] );

  test.case = 'dst inserts self twice';
  var dst = [ 1, 2 ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var src = [ 3 ];
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst inserts self, dst === src';
  var dst = [ 1 ];
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, [ 1, 1, 1 ] );

  test.case = 'dst inserts self twice, dst === src';
  var dst = [ 1, 2 ];
  dst.splice( 3, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2, 1, 2, 1, 2 ] );
  test.identical( got, [ 1, 2, 1, 2, 1, 2 ] );

  test.case = 'dst inserts self';
  var dst = [ 1 ];
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlatten( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst inserts self twice';
  var dst = [ 1 ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var got  = _.arrayFlatten( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.close( 'dst or src contains self' );
}

//

function arrayFlattenSets( test )
{
  test.open( 'dst - null' );

  test.case = 'src - empty Set';
  var dst = null;
  var src = new Set();
  var got = _.arrayFlatten( dst, src );
  test.identical( got, [] );

  test.case = 'src - flat Set';
  var dst = null;
  var src = new Set( [ 1, 2, 3 ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'src - Set, level 2';
  var dst = null;
  var src = new Set( [ 1, [ 'str' ], 3 ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = null;
  var src = new Set( [ 1, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 5';
  var dst = null;
  var src = new Set( [ [ [ 1, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = null;
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( got, [ 1, 'str', { a : 3 }, undefined, 1, 'str', 3 ] );

  test.close( 'dst - null' );

  /* - */

  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, 2, 3 ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ 'str' ], 3 ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ 1, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, undefined, 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', { a : 3 }, undefined, 1, 'str', 3 ] );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 } ] );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, 2, 3 ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2, 3 ] );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ 'str' ], 3 ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 1, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', { a : 3 }, undefined, 1, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', { a : 3 }, undefined, 1, 'str', 3 ] );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], 1, undefined, 2, { a : 0 } ] );
  test.identical( got, [ 1, [ undefined, [ 2 ], { a : 0 } ], 1, undefined, 2, { a : 0 } ] );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ] ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlatten( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], 1, undefined, 2, 1, undefined, 2, 1, undefined, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], 1, undefined, 2, 1, undefined, 2, 1, undefined, 2 ] );

  test.close( 'dst - array' );
}

//

function arrayFlattenOnce( test )
{
  test.open( 'single argument' );

  test.case = 'flat array, without duplicates';
  var dst = [ 0, 1, 2, 3 ];
  var got = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 0, 1, 2, 3 ] );
  test.identical( got, [ 0, 1, 2, 3 ] );

  test.case = 'flat array, duplicates';
  var dst = [ 0, 1, 0, 1 ];
  var got = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, [ 0, 1 ] );

  test.case = 'array, level 2, duplicates';
  var dst = [ [ 0, 0 ], [ 1, 1 ] ];
  var got = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, [ 0, 1 ] );

  test.case = 'array with diff levels, duplicates';
  var dst = [ 1, [ [ 0 ], 1 ], 1, 0 ];
  var got = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 1, 0 ] );
  test.identical( got, [ 1, 0 ] );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - number';
  var dst = [];
  var got = _.arrayFlattenOnce( dst, 2 );
  test.identical( dst, [ 2 ] );
  test.identical( got, [ 2 ] );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - flat array, duplicates';
  var dst = [];
  var got = _.arrayFlattenOnce( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 2, duplicates';
  var dst = [];
  var got = _.arrayFlattenOnce( dst, [ [ 1 ], [ 1 ], [ 2 ], [ 2 ], [ 3 ], [ 3 ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 3, duplicates';
  var dst = [];
  var got = _.arrayFlattenOnce( dst, [ [ 1, 1, [ 2, 2, [ 3, 3 ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 5, duplicates';
  var dst = [];
  var got = _.arrayFlattenOnce( dst, [ [ [ [ [ 1, 1, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  /* */

  test.case = 'dst - flat array, src - string';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenOnce( dst, 'str' );
  test.identical( dst, [ 1, 2, 3, 'str' ] );
  test.identical( got, [ 1, 2, 3, 'str' ] );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnce( dst, [ 4 ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, [ 1, 2, 3, 4 ] );

  test.case = 'dst - flat array, src - array, level 2, duplicates';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenOnce( dst, [ [ 1 ], [ 2 ], [ 3 ], [ 4 ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, [ 1, 2, 3, 4 ] );

  test.case = 'dst - flat array, src - array, level 3, duplicates';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenOnce( dst, [ 1, 2, 3, [ [ 4 ] ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, [ 1, 2, 3, 4 ] );

  test.case = 'dst - flat array, src - array, level 5, duplicates';
  var dst = [ 1 ];
  var got = _.arrayFlattenOnce( dst, [ [ [ [ [ 1, 1, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var got = _.arrayFlattenOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );

  test.case = 'dst - flat array, src - array with diff level arrays, duplicates';
  var dst = [ 1, 4 ];
  var got  = _.arrayFlattenOnce( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( dst, [ 1, 4, 2, 3 ] );
  test.identical( got, [ 1, 4, 2, 3 ] );

  /* */

  test.case = 'evaluator1';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnce( dst, [ 1, 4, 2, 5 ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 5 ] );

  test.case = 'evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnce( dst, [ 1, 4, 2, 5 ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 5 ] );

  test.case = 'evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnce( dst, [ 1, 4, 2, 5 ], 1, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 1, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 1, 5 ] );

  test.case = 'equilizer';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnce( dst, [ 1, 4, 2, 5 ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 5 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnce() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnce( [], [ 1 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'dstArray is not array';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnce( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnce( [], [ 1 ], [] ) );

  test.case = 'evaluator2 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnce( [], [ 1 ], ( e ) => e, [] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenOnce( [], [ 1 ], 0, {} ) );
}

//

function arrayFlattenOnceSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, 2, 3, 3, 3, 1 ];
  var src = dst;
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2, 3, 3, 3, 1 ] );
  test.identical( got, [ 1, 2, 3, 3, 3, 1 ] );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var src = dst;
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );

  test.case = 'dst - array, level 3, src contains dst, duplicates';
  var dst = [ [ 1, [ 2, [ 3 ] ] ] ];
  var src = [ dst, 1, 2 ];
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3 ] );
  test.identical( got, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3 ] );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, duplicates, evaluator';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenOnce( dst, src, ( e ) => e );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, [ [ [ [ [ 1 ] ] ] ], 1 ] );

  test.case = 'src contains a few dst, duplicates, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, [ [ [ [ [ 1 ] ] ] ], 1 ] );

  test.case = 'src contains a few dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ 1, [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, [ 1, [ [ [ [ 1 ] ] ] ], 1 ] );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenOnce( dst, src, 1, ( e, ins ) => e === ins );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, [ [ [ [ [ 1 ] ] ] ], 1 ] );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2 ] );
  src.add( src );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2 ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - array level 2, push self twice, src - flat array, duplicates';
  var dst = [ 1, [ 2, 3 ] ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 2, 3 ] ] );
  test.identical( got, [ 1, [ 2, 3 ] ] );

  test.case = 'dst - flat array, push self, src - Set, duplicates';
  var dst = [ 1, 2, 3 ];
  dst.push( dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - array, level 2, push self twice, src - Set, duplicates';
  var dst = [ 1, [ 1 ] ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 }, 1 ] );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 'str', { a : 2 } ] );
  test.identical( got, [ 1, [ 1 ], 'str', { a : 2 } ] );

  test.case = 'dst - array, level 2, push self, dst === src';
  var dst = [ 1, [ 3 ], 2 ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 3 ], 2, 3 ] );
  test.identical( got, [ 1, [ 3 ], 2, 3 ] );

  test.case = 'dst - array, level 2, push self twice, dst === src, duplicates';
  var dst = [ 1, [ 1 ], 2 ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 2 ] );
  test.identical( got, [ 1, [ 1 ], 2 ] );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, [ 1, 3, 2 ] );

  test.case = 'dst - array, level 6, push self twice, duplicates';
  var dst = [ [ [ [ [ [ 1, 1, 2 ] ] ] ] ] ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - array level 2, inserts self twice, src - flat array, duplicates';
  var dst = [ 1, [ 2, 3 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 2, 3 ] ] );
  test.identical( got, [ 1, [ 2, 3 ] ] );

  test.case = 'dst - flat array, inserts self, src - Set, duplicates';
  var dst = [ 1, 2, 3 ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - array, level 2, inserts self twice, src - Set, duplicates';
  var dst = [ 1, [ 1 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = new Set( [ 'str', { a : 2 }, 1 ] );
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 'str', { a : 2 } ] );
  test.identical( got, [ 1, [ 1 ], 'str', { a : 2 } ] );

  test.case = 'dst - array, level 2, inserts self, dst === src';
  var dst = [ 1, [ 3 ], 2 ];
  dst.splice( 1, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 3 ], 2, 3 ] );
  test.identical( got, [ 1, [ 3 ], 2, 3 ] );

  test.case = 'dst - array, level 2, inserts self twice, dst === src, duplicates';
  var dst = [ 1, [ 1 ], 2 ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 2 ] );
  test.identical( got, [ 1, [ 1 ], 2 ] );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, [ 1, 3, 2 ] );

  test.case = 'dst - array, level 6, inserts self twice, duplicates';
  var dst = [ [ [ [ [ [ 1, 1, 2 ] ] ] ] ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlattenOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  /* */

  test.case = 'dst inserts self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst inserts self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.close( 'dst or src contains self, evaluators' );
}

//

function arrayFlattenOnceSets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'src - flat Set, duplicates';
  var dst = [];
  var src = new Set( [ 1, 1, 2, 3, 3, 1 ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str' ] ], [ [ 'str' ] ], 1, 3 ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str' ] ], 3 ] ] ], 1, 'str', 3, 3 ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, undefined, 3 ] );
  test.identical( got, [ 1, 'str', { a : 3 }, undefined, 3 ] );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 } ] );

  test.case = 'src - flat Set, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, 2, 3 ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );

  test.case = 'src - Set, level 3, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ [ 'str' ] ], 3 ] ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 1, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', { a : 3 }, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', { a : 3 }, 3 ] );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], undefined, 2, { a : 0 } ] );
  test.identical( got, [ 1, [ undefined, [ 2 ], { a : 0 } ], undefined, 2, { a : 0 } ] );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set from two dst in container, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.case = 'src - Set from two dst in container, evaluator1 and evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.case = 'src - Set from two dst in container, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );

  test.case = 'src - Set from two dst in container, equalizer';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.close( 'dst - array, evaluators' );
}

//

function arrayFlattenOnceStrictly( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlattenOnceStrictly( [ 0, 1, 2, 3 ] );
  var expected = [ 0, 1, 2, 3 ];
  test.identical( got, expected );

  test.case = 'level 2';
  var got = _.arrayFlattenOnceStrictly( [ [ 0 ], [ 1 ] ] );
  var expected = [ 0, 1 ];
  test.identical( got, expected );

  test.case = 'diff levels';
  var got = _.arrayFlattenOnceStrictly( [ [ 0 ], 1, [ 2, [ 3, 4 ] ] ] );
  var expected = [ 0, 1, 2, 3, 4 ];
  test.identical( got, expected );

  var got = _.arrayFlattenOnceStrictly( [ 0, [ [ 1 ], 2 ], 3, 4 ] );
  var expected = [ 0, 1, 2, 3, 4 ];
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - string';
  var dst = [];
  var got = _.arrayFlattenOnceStrictly( dst, 'str' );
  test.identical( dst, [ 'str' ] );
  test.identical( got, [ 'str' ] );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenOnceStrictly( dst, [] );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - flat array';
  var dst = [];
  var got = _.arrayFlattenOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 2';
  var dst = [];
  var got = _.arrayFlattenOnceStrictly( dst, [ [ 1 ], [ 2 ], 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenOnceStrictly( dst, [ [ 1, [ 2, [ 3, 4 ] ] ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, [ 1, 2, 3, 4 ] );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenOnceStrictly( dst, [ [ [ [ [ 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  /* */

  test.case = 'dst - flat array, src - empty array';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnceStrictly( dst, [] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, [ 1, 2, 3, 4 ] );

  test.case = 'dst - flat array, src - array, level 3';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenOnceStrictly( dst, [ [ [ 4 ] ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, [ 1, 2, 3, 4 ] );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenOnceStrictly( dst, [ [ 4 ], [ 5 ], [ 6 ], [ 7 ] ] );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7 ] );
  test.identical( got, [ 1, 2, 3, 4, 5, 6, 7 ] );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ -1, 0 ];
  var got  = _.arrayFlattenOnceStrictly( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( dst, [ -1, 0, 1, 2, 3, 4 ] );
  test.identical( got, [ -1, 0, 1, 2, 3, 4 ] );

  test.case = 'dst - flat array, src - array, level 5';
  var dst = [ 1 ];
  var got = _.arrayFlattenOnceStrictly( dst, [ [ [ [ [ 2 ] ] ] ] ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var got = _.arrayFlattenOnceStrictly( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 4, 5, 6 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ 3 ], 4, 5, 6 ] );

  test.case = 'dst - array, level 2, src - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var got = _.arrayFlattenOnceStrictly( dst, [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );

  test.case = 'dst - array, level 2, src - number';
  var dst = [ 1, 3, [ 5 ] ];
  var got = _.arrayFlattenOnceStrictly( dst, 5 );
  var expected = [ 1, 3, [ 5 ], 5 ];
  test.identical( dst, expected );
  test.identical( got, [ 1, 3, [ 5 ], 5 ] );

  /* */

  test.case = 'dst - flat array, src - flat array, evaluator';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnceStrictly( dst, [ 5, 6, 7, 8 ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );
  test.identical( got, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );

  test.case = 'dst - flat array, src - flat array, evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnceStrictly( dst, [ 5, 6, 7, 8 ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );
  test.identical( got, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );

  test.case = 'dst - flat array, src - flat array, evaluator1 - fromIndex, evaluator2, has duplicates';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnceStrictly( dst, [ 1, 2, 7, 8 ], 2, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 1, 2, 7, 8 ] );
  test.identical( got, [ 1, 2, 3, 4, 1, 2, 7, 8 ] );

  test.case = 'dst - flat array, src - flat array, equalizer';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenOnceStrictly( dst, [ 5, 6, 7, 8 ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );
  test.identical( got, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly() );

  test.case = 'dstArray is not an array';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( [], [ 1 ], [] ) );

  test.case = 'duplicates in dstArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( [ 0, 0 ], [ 3, 4 ] ) );

  test.case = 'duplicates in insArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( [], [ 1, 1, 2, 2, 3, 3 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( [], [ [ 1 ], [ 1 ], [ 2 ], [ 2 ], [ 3 ], [ 3 ] ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( [], [ [ 1, 1, [ 2, 2, [ 3, 3 ] ] ] ] ) );

  test.case = 'insArray is undefined';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( [], undefined ) );

  test.case = 'insArray has undefined';
  test.shouldThrowErrorSync( () => _.arrayFlattenOnceStrictly( [], [ 1, [ undefined ] ] ) );

}

//

function arrayFlattenOnceStrictlySame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var src = dst;
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ 3 ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3 ] );
  test.identical( got, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3 ] );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );

  test.case = 'src contains dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenOnceStrictly( dst, src, 1, ( e ) => e );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, [ [ [ [ [ 1 ] ] ] ], 1 ] );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenOnceStrictly( dst, src, 1, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2 ] );
  src.add( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2 ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - flat array, push self, src - Set';
  var dst = [ 1, 2, 3 ];
  dst.push( dst );
  var src = new Set( [ 4, [ 5 ], 6 ] );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6 ] );
  test.identical( got, [ 1, 2, 3, 4, 5, 6 ] );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenOnceStrictly( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, [ 1, 3, 2 ] );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - flat array, inserts self, src - Set';
  var dst = [ 1, 2, 3 ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 4, [ [ 'str' ], 5 ] ] );
  var got  = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, 4, 'str', 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 'str', 5 ] );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenOnceStrictly( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, [ 1, 3, 2 ] );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 2, [ 3 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2, 3 ] );
  test.identical( got, [ 1, [ 2 ], 2, 3 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ null, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenOnceStrictly( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], null, 2 ] );
  test.identical( got, [ 1, [ 2 ], null, 2 ] );

  /* */

  test.case = 'dst inserts self, evaluator';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 3, [ 2 ] ];
  var got  = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 3, 2 ] );
  test.identical( got, [ 1, [ 2 ], 3, 2 ] );

  test.case = 'dst inserts self, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self, equalizer';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.close( 'dst or src contains self, evaluators' );

  if( !Config.debug )
  return;

  test.case = 'dst === src';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, 2, 3 ];
    var src = dst;
    var got  = _.arrayFlattenOnceStrictly( dst, src );
  });

  test.case = 'src contains a few dst, simple evaluator';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1 ] ] ] ] ];
    var src = [ dst, dst, dst, dst ];
    var got  = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e );
  });

  test.case = 'dst push self twice';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1 ] ] ] ] ];
    dst.push( dst );
    dst.push( dst );
    var got  = _.arrayFlattenOnceStrictly( dst );
  });
}

//

function arrayFlattenOnceStrictlySets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, [ 'str' ], 2, 3, { a : 0 }, { a : 0 } ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 2, 3, { a : 0 }, { a : 0 } ] );
  test.identical( got, [ 1, 'str', 2, 3, { a : 0 }, { a : 0 } ] );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str' ] ], 3 ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str' ] ], 3 ] ] ], 2, 4, 6 ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3, 2, 4, 6 ] );
  test.identical( got, [ 1, 'str', 3, 2, 4, 6 ] );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 } ] ] ], [ [ [ 2, [ 'src' ], 4 ] ] ] ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, 2, 'src', 4 ] );
  test.identical( got, [ 1, 'str', { a : 3 }, 2, 'src', 4 ] );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 } ] );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3, 4 ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3, 4 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3, 4 ] );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ [ 'str' ] ], 3 ] ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 2, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 2 ] ] ], [ [ [ [ 'str' ] ] ] ] ] );
  var got = _.arrayFlattenOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str' ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str' ] );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3 ] );
  var got = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );

  test.case = 'src - Set from dst, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );

  test.case = 'src - Set from dst, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenOnceStrictly( dst, src, 3, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );

  test.case = 'src - Set from dst, equalizer';
  var dst = [ 1, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );

  test.close( 'dst - array, evaluators' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'src - Set from dst';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, [ [ 2 ], { a : 0 } ] ];
    var src = new Set( dst );
    var got = _.arrayFlattenOnceStrictly( dst, src );
  });
}

//

function arrayFlattened( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlattened( [ 0, 1, 2, 3 ] );
  var expected = 4;
  test.identical( got, expected );

  test.case = 'array, level 2';
  var got = _.arrayFlattened( [ [ 0, 0 ], [ 1, 1 ] ] );
  var expected = 4;
  test.identical( got, expected );

  test.case = 'array, level 3';
  var got = _.arrayFlattened( [ [ 0 ], 0, 1, [ [ 0 ], 1 ] ] );
  var expected = 5;
  test.identical( got, expected );

  test.case = 'array, level 5';
  var got = _.arrayFlattened( [ [ [ [ [ 0 ] ] ] ] ] );
  var expected = 1;
  test.identical( got, expected );

  test.case = 'array from other arrays, level 5';
  var src = [ [ [ [ [ 0 ] ] ] ] ];
  var got = _.arrayFlattened( [ src, src, src, src ] );
  var expected = 4;
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got  = _.arrayFlattened( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - flat array';
  var dst = [];
  var got  = _.arrayFlattened( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 2';
  var dst = [];
  var got  = _.arrayFlattened( dst, [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 3';
  var dst = [];
  var got  = _.arrayFlattened( dst, [ [ 1, [ 2, [ 3 ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 5';
  var dst = [];
  var got  = _.arrayFlattened( dst, [ [ [ [ [ 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - empty array, src - array from arrays level 5';
  var src = [ [ [ [ [ 0 ] ] ] ] ];
  var dst = [];
  var got = _.arrayFlattened( dst, [ src, src, src, src ] );
  var expected = [ 0, 0, 0, 0 ];
  test.identical( dst, expected );
  test.identical( got, 4 );

  test.case = 'dst - flat array, src - flat array';
  var dst = [ 1, 2, 3 ];
  var got  = _.arrayFlattened( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattened( dst, [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - array, level 3';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattened( dst, [ [ 1, [ 2, [ 3 ] ] ] ] );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, src - array, level 5';
  var dst = [ 1 ];
  var got = _.arrayFlattened( dst, [ [ [ [ [ 1 ] ] ] ], [ [ [ [ 1 ] ] ] ], [ [ [ [ 1 ] ] ] ] ] );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ 1, [ 2, 3 ] ];
  var got  = _.arrayFlattened( dst, [ 4 ] );
  test.identical( dst, [ 1, [ 2, 3 ], 4 ] );
  test.identical( got, 1 );

  test.case = 'dst - empty array, src - multiple arrays with diff levels';
  var dst = [];
  var got = _.arrayFlattened( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, 4 );

  test.case = 'dst - empty array, src - number';
  var dst = [];
  var got  = _.arrayFlattened( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, src - number';
  var dst = [ 1, 1, 3, 3, [ 5, 5 ] ];
  var got = _.arrayFlattened( dst, 5 );
  var expected = [ 1, 1, 3, 3, [ 5, 5 ], 5 ];
  test.identical( dst, expected );
  test.identical( got, 1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattened() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattened( [], [ 1 ], [ 'extra' ] ) );

  test.case = 'dstArray is not arrayLike';
  test.shouldThrowErrorSync( () => _.arrayFlattened( 1, [ 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattened( new U8x( 2 ), [ 1 ] ) );
}

//

function arrayFlattenedSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - flat array';
  var dst = [ 1, 2, 3 ];
  var src = dst;
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2, 3, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var src = dst;
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ 3 ] ] ] ];
  var src = [ dst, 4, 5 ];
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3, 4, 5 ] );
  test.identical( got, 5 );

  test.case = 'dst - array, level 5, src contains a few dst';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, 4 );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'src push self';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'src push self twice';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2 ] );
  src.add( src );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2 ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst push self';
  var dst = [ 1 ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst push self twice';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst push self, src - Set';
  var dst = [ 1 ];
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 'str', { a : 2 } ] );
  test.identical( got, 2 );

  test.case = 'dst push self twice, src - Set';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 'str', { a : 2 } ] );
  test.identical( got, 2 );

  test.case = 'dst push self, dst === src';
  var dst = [ 1 ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst push self twice, dst === src';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 1, 1, 1 ] );
  test.identical( got, 3 );

  test.case = 'dst push self';
  var dst = [ 1 ];
  dst.push( dst );
  var got  = _.arrayFlattened( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst push self twice';
  var dst = [ 1 ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlattened( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'src inserts self';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 1, 0, dst );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'src inserts self twice';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 0, 0, dst );
  src.splice( 2, 0, dst );
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst inserts self';
  var dst = [ 1, 2 ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self twice';
  var dst = [ 1, 2 ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var src = [ 3 ];
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self, dst === src';
  var dst = [ 1 ];
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst inserts self twice, dst === src';
  var dst = [ 1, 2 ];
  dst.splice( 3, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2, 1, 2, 1, 2 ] );
  test.identical( got, 4 );

  test.case = 'dst inserts self';
  var dst = [ 1 ];
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlattened( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self twice';
  var dst = [ 1 ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var got  = _.arrayFlattened( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.close( 'dst or src contains self' );
}

//

function arrayFlattenedSets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, 2, 3 ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ 'str' ], 3 ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ 1, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two array level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, undefined, 1, 'str', 3 ] );
  test.identical( got, 7 );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, 2, 3 ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ 'str' ], 3 ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 1, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 'str', { a : 3 }, undefined, 1, 'str', 3 ] );
  test.identical( got, 7 );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], 1, undefined, 2, { a : 0 } ] );
  test.identical( got, 4 );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ] ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattened( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], 1, undefined, 2, 1, undefined, 2, 1, undefined, 2 ] );
  test.identical( got, 9 );

  test.close( 'dst - array' );
}

//

function arrayFlattenedOnce( test )
{
  test.open( 'single argument' );

  test.case = 'flat array, without duplicates';
  var dst = [ 0, 1, 2, 3 ];
  var got = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 0, 1, 2, 3 ] );
  test.identical( got, 4 );

  test.case = 'flat array, duplicates';
  var dst = [ 0, 1, 0, 1 ];
  var got = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, 2 );

  test.case = 'array, level 2, duplicates';
  var dst = [ [ 0, 0 ], [ 1, 1 ] ];
  var got = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, 2 );

  test.case = 'array with diff levels, duplicates';
  var dst = [ 1, [ [ 0 ], 1 ], 1, 0 ];
  var got = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 1, 0 ] );
  test.identical( got, 2 );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - number';
  var dst = [];
  var got = _.arrayFlattenedOnce( dst, 2 );
  test.identical( dst, [ 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenedOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - flat array, duplicates';
  var dst = [];
  var got = _.arrayFlattenedOnce( dst, [ 1, 1, 2, 2, 3, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 2, duplicates';
  var dst = [];
  var got = _.arrayFlattenedOnce( dst, [ [ 1 ], [ 1 ], [ 2 ], [ 2 ], [ 3 ], [ 3 ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 3, duplicates';
  var dst = [];
  var got = _.arrayFlattenedOnce( dst, [ [ 1, 1, [ 2, 2, [ 3, 3 ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 5, duplicates';
  var dst = [];
  var got = _.arrayFlattenedOnce( dst, [ [ [ [ [ 1, 1, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst - flat array, src - string';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenedOnce( dst, 'str' );
  test.identical( dst, [ 1, 2, 3, 'str' ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnce( dst, [ 4 ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - array, level 2, duplicates';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenedOnce( dst, [ [ 1 ], [ 2 ], [ 3 ], [ 4 ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, src - array, level 3, duplicates';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenedOnce( dst, [ 1, 2, 3, [ [ 4 ] ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, src - array, level 5, duplicates';
  var dst = [ 1 ];
  var got = _.arrayFlattenedOnce( dst, [ [ [ [ [ 1, 1, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var got = _.arrayFlattenedOnce( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - array with diff level arrays, duplicates';
  var dst = [ 1, 4 ];
  var got  = _.arrayFlattenedOnce( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( dst, [ 1, 4, 2, 3 ] );
  test.identical( got, 2 );

  /* */

  test.case = 'evaluator1';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnce( dst, [ 1, 4, 2, 5 ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 1 );

  test.case = 'evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnce( dst, [ 1, 4, 2, 5 ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 1 );

  test.case = 'evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnce( dst, [ 1, 4, 2, 5 ], 1, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 1, 5 ] );
  test.identical( got, 2 );

  test.case = 'equilizer';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnce( dst, [ 1, 4, 2, 5 ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnce() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnce( [], [ 1 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'dstArray is not array';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnce( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnce( [], [ 1 ], [] ) );

  test.case = 'evaluator2 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnce( [], [ 1 ], ( e ) => e, [] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnce( [], [ 1 ], 0, {} ) );
}

//

function arrayFlattenedOnceSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, 2, 3, 3, 3, 1 ];
  var src = dst;
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3, 3, 3, 1 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var src = dst;
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 3, src contains dst, duplicates';
  var dst = [ [ 1, [ 2, [ 3 ] ] ] ];
  var src = [ dst, 1, 2 ];
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3 ] );
  test.identical( got, 3 );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, duplicates, evaluator';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.case = 'src contains a few dst, duplicates, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.case = 'src contains a few dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ 1, [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedOnce( dst, src, 1, ( e, ins ) => e === ins );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2 ] );
  src.add( src );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2 ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array level 2, push self twice, src - flat array, duplicates';
  var dst = [ 1, [ 2, 3 ] ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, 3 ] ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, push self, src - Set, duplicates';
  var dst = [ 1, 2, 3 ];
  dst.push( dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, push self twice, src - Set, duplicates';
  var dst = [ 1, [ 1 ] ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 }, 1 ] );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 'str', { a : 2 } ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 2, push self, dst === src';
  var dst = [ 1, [ 3 ], 2 ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 3 ], 2, 3 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, push self twice, dst === src, duplicates';
  var dst = [ 1, [ 1 ], 2 ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 2 ] );
  test.identical( got, 0 );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 6, push self twice, duplicates';
  var dst = [ [ [ [ [ [ 1, 1, 2 ] ] ] ] ] ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array level 2, inserts self twice, src - flat array, duplicates';
  var dst = [ 1, [ 2, 3 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, 3 ] ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, inserts self, src - Set, duplicates';
  var dst = [ 1, 2, 3 ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, inserts self twice, src - Set, duplicates';
  var dst = [ 1, [ 1 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = new Set( [ 'str', { a : 2 }, 1 ] );
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 'str', { a : 2 } ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 2, inserts self, dst === src';
  var dst = [ 1, [ 3 ], 2 ];
  dst.splice( 1, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 3 ], 2, 3 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, inserts self twice, dst === src, duplicates';
  var dst = [ 1, [ 1 ], 2 ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], 2 ] );
  test.identical( got, 0 );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 6, inserts self twice, duplicates';
  var dst = [ [ [ [ [ [ 1, 1, 2 ] ] ] ] ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlattenedOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst inserts self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.close( 'dst or src contains self, evaluators' );
}

//

function arrayFlattenedOnceSets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'src - flat Set, duplicates';
  var dst = [];
  var src = new Set( [ 1, 1, 2, 3, 3, 1 ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str' ] ], [ [ 'str' ] ], 1, 3 ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str' ] ], 3 ] ] ], 1, 'str', 3, 3 ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, undefined, 3 ] );
  test.identical( got, 5 );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, 0 );

  test.case = 'src - flat Set, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, 2, 3 ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 3, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ [ 'str' ] ], 3 ] ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 1, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', { a : 3 }, 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], undefined, 2, { a : 0 } ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set from two dst in container, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.case = 'src - Set from two dst in container, evaluator1 and evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.case = 'src - Set from two dst in container, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from two dst in container, equalizer';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.close( 'dst - array, evaluators' );
}

//

function arrayFlattenedOnceStrictly( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlattenedOnceStrictly( [ 0, 1, 2, 3 ] );
  var expected = 4;
  test.identical( got, expected );

  test.case = 'level 2';
  var got = _.arrayFlattenedOnceStrictly( [ [ 0 ], [ 1 ] ] );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'diff levels';
  var got = _.arrayFlattenedOnceStrictly( [ [ 0 ], 1, [ 2, [ 3, 4 ] ] ] );
  var expected = 5;
  test.identical( got, expected );

  var got = _.arrayFlattenedOnceStrictly( [ 0, [ [ 1 ], 2 ], 3, 4 ] );
  var expected = 5;
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - string';
  var dst = [];
  var got = _.arrayFlattenedOnceStrictly( dst, 'str' );
  test.identical( dst, [ 'str' ] );
  test.identical( got, 1 );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenedOnceStrictly( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - flat array';
  var dst = [];
  var got = _.arrayFlattenedOnceStrictly( dst, [ 1, 2, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 2';
  var dst = [];
  var got = _.arrayFlattenedOnceStrictly( dst, [ [ 1 ], [ 2 ], 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenedOnceStrictly( dst, [ [ 1, [ 2, [ 3, 4 ] ] ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, 4 );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenedOnceStrictly( dst, [ [ [ [ [ 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst - flat array, src - empty array';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - array, level 3';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ [ [ 4 ] ] ] );
  test.identical( dst, [ 1, 2, 3, 4 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ [ 4 ], [ 5 ], [ 6 ], [ 7 ] ] );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7 ] );
  test.identical( got, 4 );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ -1, 0 ];
  var got  = _.arrayFlattenedOnceStrictly( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( dst, [ -1, 0, 1, 2, 3, 4 ] );
  test.identical( got, 4 );

  test.case = 'dst - flat array, src - array, level 5';
  var dst = [ 1 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ [ [ [ [ 2 ] ] ] ] ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ 4, 5, 6 ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 4, 5, 6 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 2, src - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 2, src - number';
  var dst = [ 1, 3, [ 5 ] ];
  var got = _.arrayFlattenedOnceStrictly( dst, 5 );
  var expected = [ 1, 3, [ 5 ], 5 ];
  test.identical( dst, expected );
  test.identical( got, 1 );

  /* */

  test.case = 'dst - flat array, src - flat array, evaluator';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ 5, 6, 7, 8 ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );
  test.identical( got, 4 );

  test.case = 'dst - flat array, src - flat array, evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ 5, 6, 7, 8 ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );
  test.identical( got, 4 );

  test.case = 'dst - flat array, src - flat array, evaluator1 - fromIndex, evaluator2, has duplicates';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ 1, 2, 7, 8 ], 2, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 1, 2, 7, 8 ] );
  test.identical( got, 4 );

  test.case = 'dst - flat array, src - flat array, equalizer';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedOnceStrictly( dst, [ 5, 6, 7, 8 ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6, 7, 8 ] );
  test.identical( got, 4 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly() );

  test.case = 'dstArray is not an array';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( [], [ 1 ], [] ) );

  test.case = 'duplicates in dstArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( [ 0, 0 ], [ 3, 4 ] ) );

  test.case = 'duplicates in insArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( [], [ 1, 1, 2, 2, 3, 3 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( [], [ [ 1 ], [ 1 ], [ 2 ], [ 2 ], [ 3 ], [ 3 ] ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( [], [ [ 1, 1, [ 2, 2, [ 3, 3 ] ] ] ] ) );

  test.case = 'insArray is undefined';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( [], undefined ) );

  test.case = 'insArray has undefined';
  test.shouldThrowErrorSync( () => _.arrayFlattenedOnceStrictly( [], [ 1, [ undefined ] ] ) );
}

//

function arrayFlattenedOnceStrictlySame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  var src = dst;
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ 3 ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ 3 ] ] ], 1, 2, 3 ] );
  test.identical( got, 3 );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, 4 );

  test.case = 'src contains dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src, 1, ( e ) => e );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1 ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src, 1, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ [ [ [ [ 1 ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, 4 );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2 ];
  src.push( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2 ] );
  src.add( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2 ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, push self, src - Set';
  var dst = [ 1, 2, 3 ];
  dst.push( dst );
  var src = new Set( [ 4, [ 5 ], 6 ] );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, 4, 5, 6 ] );
  test.identical( got, 3 );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenedOnceStrictly( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, 3 );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2 ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2 ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, inserts self, src - Set';
  var dst = [ 1, 2, 3 ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 4, [ [ 'str' ], 5 ] ] );
  var got  = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, 4, 'str', 5 ] );
  test.identical( got, 3 );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ 2 ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenedOnceStrictly( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, 3 );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 2, [ 3 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2, 3 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ null, [ 2 ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], null, 2 ] );
  test.identical( got, 2 );

  /* */

  test.case = 'dst inserts self, evaluator';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 3, [ 2 ] ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 3, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst inserts self, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self, equalizer';
  var dst = [ 1, [ 2 ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.close( 'dst or src contains self, evaluators' );

  if( !Config.debug )
  return;

  test.case = 'dst === src';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, 2, 3 ];
    var src = dst;
    var got  = _.arrayFlattenedOnceStrictly( dst, src );
  });

  test.case = 'src contains a few dst, simple evaluator';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1 ] ] ] ] ];
    var src = [ dst, dst, dst, dst ];
    var got  = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e );
  });

  test.case = 'dst push self twice';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1 ] ] ] ] ];
    dst.push( dst );
    dst.push( dst );
    var got  = _.arrayFlattenedOnceStrictly( dst );
  });
}

arrayFlattenedOnceStrictlySame.timeOut = 10000;

//

function arrayFlattenedOnceStrictlySets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, [ 'str' ], 2, 3, { a : 0 }, { a : 0 } ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 2, 3, { a : 0 }, { a : 0 } ] );
  test.identical( got, 6 );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str' ] ], 3 ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str' ] ], 3 ] ] ], 2, 4, 6 ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3, 2, 4, 6 ] );
  test.identical( got, 6 );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str' ], { a : 3 } ] ] ], [ [ [ 2, [ 'src' ], 4 ] ] ] ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, 2, 'src', 4 ] );
  test.identical( got, 6 );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3, 4 ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3, 4 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ 'str' ], 3 ] ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ [ 'str' ] ], 3 ] ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 2, [ 'str' ], 3 ] ] ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 2 ] ] ], [ [ [ [ 'str' ] ] ] ] ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str' ] );
  test.identical( got, 2 );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3 ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from dst, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, 3 );

  test.case = 'src - Set from dst, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src, 3, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, 3 );

  test.case = 'src - Set from dst, equalizer';
  var dst = [ 1, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenedOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ 2 ], { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, 3 );

  test.close( 'dst - array, evaluators' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'src - Set from dst';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, [ [ 2 ], { a : 0 } ] ];
    var src = new Set( dst );
    var got = _.arrayFlattenedOnceStrictly( dst, src );
  });
}

//

function arrayFlattenDefined( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlattenDefined( [ 0, 1, 2, 3 ] );
  var expected = [ 0, 1, 2, 3 ];
  test.identical( got, expected );

  test.case = 'array, level 2';
  var got = _.arrayFlattenDefined( [ [ 0, 0 ], [ 1, 1 ] ] );
  var expected = [ 0, 0, 1, 1 ];
  test.identical( got, expected );

  test.case = 'array, level 3';
  var got = _.arrayFlattenDefined( [ [ 0 ], 0, 1, [ [ 0 ], 1 ] ] );
  var expected = [ 0, 0, 1, 0, 1 ];
  test.identical( got, expected );

  test.case = 'array, level 5';
  var got = _.arrayFlattenDefined( [ [ [ [ [ 0 ] ] ] ] ] );
  var expected = [ 0 ];
  test.identical( got, expected );

  test.case = 'array from other arrays, level 5';
  var src = [ [ [ [ [ 0 ] ] ] ] ];
  var got = _.arrayFlattenDefined( [ src, src, src, src ] );
  var expected = [ 0, 0, 0, 0 ];
  test.identical( got, expected );

  test.case = 'array, level 5, has undefined';
  var dst = [ undefined, [ undefined, [ undefined, [ undefined, [ 0 ] ] ] ] ];
  var got = _.arrayFlattenDefined( dst );
  var expected = [ 0 ];
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got  = _.arrayFlattenDefined( dst, [] );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - flat array';
  var dst = [];
  var got  = _.arrayFlattenDefined( dst, [ 1, 2, undefined ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - array, level 2';
  var dst = [];
  var got  = _.arrayFlattenDefined( dst, [ [ 1 ], [ 2 ], [ undefined ] ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - array, level 3';
  var dst = [];
  var got  = _.arrayFlattenDefined( dst, [ [ 1, [ 2, [ undefined ] ] ] ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - array, level 5';
  var dst = [];
  var got  = _.arrayFlattenDefined( dst, [ [ [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - array from arrays level 5';
  var src = [ [ [ [ [ undefined ] ] ] ] ];
  var dst = [];
  var got = _.arrayFlattenDefined( dst, [ src, src, src, src ] );
  test.identical( dst, [] );
  test.identical( got, [] );

  /* */

  test.case = 'dst - null, src - empty array';
  var dst = null;
  var got  = _.arrayFlattenDefined( dst, [] );
  test.identical( dst, null );
  test.identical( got, [] );

  test.case = 'dst - null, src - flat array';
  var dst = null;
  var got  = _.arrayFlattenDefined( dst, [ 1, 2, undefined ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - null, src - array, level 2';
  var dst = null;
  var got  = _.arrayFlattenDefined( dst, [ [ 1 ], [ 2 ], [ undefined ] ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - null, src - array, level 3';
  var dst = null;
  var got  = _.arrayFlattenDefined( dst, [ [ 1, [ 2, [ undefined ] ] ] ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - null, src - array, level 5';
  var dst = null;
  var got  = _.arrayFlattenDefined( dst, [ [ [ [ [ undefined ] ] ] ] ] );
  test.identical( got, [] );

  test.case = 'dst - null, src - array from arrays level 5';
  var src = [ [ [ [ [ undefined ] ] ] ] ];
  var dst = null;
  var got = _.arrayFlattenDefined( dst, [ src, src, src, src ] );
  test.identical( got, [] );

  /* */

  test.case = 'dst - flat array, src - flat array';
  var dst = [ 1, undefined, 3 ];
  var got  = _.arrayFlattenDefined( dst, [ 1, 2, undefined ] );
  test.identical( dst, [ 1, undefined, 3, 1, 2 ] );
  test.identical( got, [ 1, undefined, 3, 1, 2 ] );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenDefined( dst, [ [ 1 ], [ 2 ], [ undefined ] ] );
  test.identical( dst, [ 1, undefined, 3, 1, 2 ] );
  test.identical( got, [ 1, undefined, 3, 1, 2 ] );

  test.case = 'dst - flat array, src - array, level 3';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenDefined( dst, [ [ 1, [ 2, [ undefined ] ] ] ] );
  test.identical( dst, [ 1, undefined, 3, 1, 2 ] );
  test.identical( got, [ 1, undefined, 3, 1, 2 ] );

  test.case = 'dst - array, src - array, level 5';
  var dst = [ 1 ];
  var got = _.arrayFlattenDefined( dst, [ [ [ [ [ 1 ] ] ] ], [ [ [ [ 1 ] ] ] ], [ [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, [ 1, 1, 1 ] );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ 1, [ 2, 3 ] ];
  var got  = _.arrayFlattenDefined( dst, [ undefined ] );
  test.identical( dst, [ 1, [ 2, 3 ] ] );
  test.identical( got, [ 1, [ 2, 3 ] ] );

  test.case = 'dst - empty array, src - multiple arrays with diff levels';
  var dst = [];
  var got = _.arrayFlattenDefined( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - number';
  var dst = [];
  var got  = _.arrayFlattenDefined( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array, level 2, src - number';
  var dst = [ 1, 1, 3, 3, [ 5, 5 ] ];
  var got = _.arrayFlattenDefined( dst, 5 );
  var expected = [ 1, 1, 3, 3, [ 5, 5 ], 5 ];
  test.identical( dst, expected );
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefined() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefined( [], [ 1 ], [ 'extra' ] ) );

  test.case = 'dstArray is not arrayLike';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefined( 1, [ 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenDefined( new U8x( 2 ), [ 1 ] ) );
}

//

function arrayFlattenDefinedSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - flat array, src - flat array';
  var dst = [ 1, 2, undefined ];
  var src = dst;
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, 2, undefined, 1, 2 ] );
  test.identical( got, [ 1, 2, undefined, 1, 2 ] );

  test.case = 'dst - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var src = dst;
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ undefined ] ] ] ];
  var src = [ dst, 4, 5 ];
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ undefined ] ] ], 1, 2, 4, 5 ] );
  test.identical( got, [ [ 1, [ 2, [ undefined ] ] ], 1, 2, 4, 5 ] );

  test.case = 'dst - array, level 5, src contains a few dst';
  var dst = [ [ [ [ [ undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ [ [ [ [ undefined ] ] ] ] ] );
  test.identical( got, [ [ [ [ [ undefined ] ] ] ] ] );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - null, src push self';
  var dst = null;
  var src = [ 1, undefined ];
  src.push( src );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( got, [ 1 ] );

  test.case = 'src push self';
  var dst = [];
  var src = [ 1, undefined ];
  src.push( src );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'src push self twice';
  var dst = [];
  var src = [ 1, [ undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, [ undefined ] ] );
  src.add( src );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ [ undefined ] ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst push self';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.identical( got, [ 1, undefined, 2 ] );

  test.case = 'dst push self twice';
  var dst = [ 1, undefined ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.identical( got, [ 1, undefined, 2 ] );

  test.case = 'dst push self, src - Set';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 'str', { a : 2 } ] );
  test.identical( got, [ 1, undefined, 'str', { a : 2 } ] );

  test.case = 'dst push self twice, src - Set';
  var dst = [ 1, undefined ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 'str', { a : 2 } ] );
  test.identical( got, [ 1, undefined, 'str', { a : 2 } ] );

  test.case = 'dst push self, dst === src';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1 ] );
  test.identical( got, [ 1, undefined, 1, 1 ] );

  test.case = 'dst push self twice, dst === src';
  var dst = [ 1, undefined ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1, 1 ] );
  test.identical( got, [ 1, undefined, 1, 1, 1 ] );

  test.case = 'dst push self';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var got  = _.arrayFlattenDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst push self twice';
  var dst = [ 1, undefined, undefined ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlattenDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  /* */

  test.case = 'src inserts self';
  var dst = [];
  var src = [ 1, undefined ];
  src.splice( 1, 0, dst );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'src inserts self twice';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.splice( 0, 0, dst );
  src.splice( 2, 0, dst );
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst inserts self';
  var dst = [ 1, 2, undefined ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, 2, undefined, 2 ] );
  test.identical( got, [ 1, 2, undefined, 2 ] );

  test.case = 'dst inserts self twice';
  var dst = [ 1, undefined ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var src = [ 3 ];
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 3 ] );
  test.identical( got, [ 1, undefined, 3 ] );

  test.case = 'dst inserts self, dst === src';
  var dst = [ 1, undefined ];
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1 ] );
  test.identical( got, [ 1, undefined, 1, 1 ] );

  test.case = 'dst inserts self twice, dst === src';
  var dst = [ 1, undefined ];
  dst.splice( 3, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1, 1 ] );
  test.identical( got, [ 1, undefined, 1, 1, 1 ] );

  test.case = 'dst inserts self';
  var dst = [ 1, undefined ];
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlattenDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst inserts self twice';
  var dst = [ 1, undefined, [ undefined ] ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var got  = _.arrayFlattenDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.close( 'dst or src contains self' );
}

//

function arrayFlattenDefinedSets( test )
{
  test.open( 'dst - null' );

  test.case = 'src - empty Set';
  var dst = null;
  var src = new Set();
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( got, [] );

  test.case = 'src - flat Set';
  var dst = null;
  var src = new Set( [ 1, 2, undefined ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( got, [ 1, 2 ] );

  test.case = 'src - Set, level 2';
  var dst = null;
  var src = new Set( [ 1, [ undefined ], 3 ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( got, [ 1, 3 ] );

  test.case = 'src - Set, level 3';
  var dst = null;
  var src = new Set( [ 1, [ [ undefined ], 3 ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( got, [ 1, 3 ] );

  test.case = 'src - Set, level 5';
  var dst = null;
  var src = new Set( [ [ [ 1, [ undefined ], 3 ] ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( got, [ 1, 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = null;
  var src = new Set( [ [ [ [ 1, [ undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ undefined ], 3 ] ] ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( got, [ 1, { a : 3 }, 1, 3 ] );

  test.close( 'dst - null' );

  /* - */

  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, 2, undefined ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ undefined ], 3 ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, [ 1, 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ undefined ], 3 ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, [ 1, 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ 1, [ undefined ], 3 ] ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, [ 1, 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ undefined ], 3 ] ] ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, { a : 3 }, 1, 3 ] );
  test.identical( got, [ 1, { a : 3 }, 1, 3 ] );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 } ] );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, 2, undefined ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ undefined ], 3 ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ undefined, undefined ], 3 ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 1, [ undefined ], 3 ] ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ undefined ], 3 ] ] ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, { a : 3 }, 1, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, { a : 3 }, 1, 3 ] );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], 1, 2, { a : 0 } ] );
  test.identical( got, [ 1, [ undefined, [ 2 ], { a : 0 } ], 1, 2, { a : 0 } ] );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2, { a : 0 }, 1, 2, { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2, { a : 0 }, 1, 2, { a : 0 }, 1, 2, { a : 0 } ] );

  test.close( 'dst - array' );
}

//

function arrayFlattenDefinedOnce( test )
{
  test.open( 'single argument' );

  test.case = 'flat array, without duplicates';
  var dst = [ 0, 1, undefined, 3 ];
  var got = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 0, 1, 3 ] );
  test.identical( got, [ 0, 1, 3 ] );

  test.case = 'flat array, duplicates';
  var dst = [ 0, 1, 0, undefined, 1 ];
  var got = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, [ 0, 1 ] );

  test.case = 'array, level 2, duplicates';
  var dst = [ [ 0, 0, undefined ], [ 1, 1, undefined ] ];
  var got = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, [ 0, 1 ] );

  test.case = 'array with diff levels, duplicates';
  var dst = [ 1, undefined, [ [ 0, undefined ], 1, undefined ], 1, 0 ];
  var got = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 1, 0 ] );
  test.identical( got, [ 1, 0 ] );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - null, src - undefined';
  var dst = null;
  var got = _.arrayFlattenDefinedOnce( dst, undefined );
  test.identical( got, [] );

  test.case = 'dst - null, src - empty array';
  var dst = null;
  var got = _.arrayFlattenDefinedOnce( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - null, src - flat array, duplicates';
  var dst = null;
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 1, 2, 2, undefined, 3, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - null, src - array, level 2, duplicates';
  var dst = null;
  var got = _.arrayFlattenDefinedOnce( dst, [ [ 1 ], [ 1 ], [ undefined ], [ 2 ], [ 3 ], [ 3 ], [ undefined ] ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - null, src - array, level 3, duplicates';
  var dst = null;
  var got = _.arrayFlattenDefinedOnce( dst, [ [ 1, 1, undefined, [ 2, 2, undefined, [ 3, 3, undefined ] ] ] ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - null, src - array, level 5, duplicates';
  var dst = null;
  var got = _.arrayFlattenDefinedOnce( dst, [ undefined, [ [ [ [ 1, 1, undefined, 1 ] ] ] ] ] );
  test.identical( got, [ 1 ] );

  /* */

  test.case = 'dst - empty array, src - undefined';
  var dst = [];
  var got = _.arrayFlattenDefinedOnce( dst, undefined );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenDefinedOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - flat array, duplicates';
  var dst = [];
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 1, 2, 2, undefined, 3, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 2, duplicates';
  var dst = [];
  var got = _.arrayFlattenDefinedOnce( dst, [ [ 1 ], [ 1 ], [ undefined ], [ 2 ], [ 3 ], [ 3 ], [ undefined ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 3, duplicates';
  var dst = [];
  var got = _.arrayFlattenDefinedOnce( dst, [ [ 1, 1, undefined, [ 2, 2, undefined, [ 3, 3, undefined ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 5, duplicates';
  var dst = [];
  var got = _.arrayFlattenDefinedOnce( dst, [ undefined, [ [ [ [ 1, 1, undefined, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  /* */

  test.case = 'dst - flat array, src - string';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenDefinedOnce( dst, 'str' );
  test.identical( dst, [ 1, undefined, 3, 'str' ] );
  test.identical( got, [ 1, undefined, 3, 'str' ] );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, undefined, 3, 4 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ 4 ] );
  test.identical( dst, [ 1, undefined, 3, 4 ] );
  test.identical( got, [ 1, undefined, 3, 4 ] );

  test.case = 'dst - flat array, src - array, level 2, duplicates';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ [ 1 ], [ 2 ], [ 3 ], [ 4 ], [ undefined ] ] );
  test.identical( dst, [ 1, undefined, 3, 2, 4 ] );
  test.identical( got, [ 1, undefined, 3, 2, 4 ] );

  test.case = 'dst - flat array, src - array, level 3, duplicates';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 2, 3, [ [ undefined ] ] ] );
  test.identical( dst, [ 1, undefined, 3, 2 ] );
  test.identical( got, [ 1, undefined, 3, 2 ] );

  test.case = 'dst - flat array, src - array, level 5, duplicates';
  var dst = [ 1 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ [ [ [ [ 1, 1, undefined, undefined, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2, undefined ], [ 3 ] ];
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 2, undefined, 3 ] );
  test.identical( dst, [ [ 1 ], [ 2, undefined ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, [ [ 1 ], [ 2, undefined ], [ 3 ], 1, 2, 3 ] );

  test.case = 'dst - flat array, src - array with diff level arrays, duplicates';
  var dst = [ 1, 4 ];
  var got  = _.arrayFlattenDefinedOnce( dst, [ [ 1 ], [ [ 2, undefined ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( dst, [ 1, 4, 2, 3 ] );
  test.identical( got, [ 1, 4, 2, 3 ] );

  /* */

  test.case = 'evaluator1';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 4, 2, 5, undefined ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 5 ] );

  test.case = 'evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 4, 2, 5, [ [ undefined ] ] ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 5 ] );

  test.case = 'evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 4, 2, [ undefined ], 5 ], 1, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 1, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 1, 5 ] );

  test.case = 'equilizer';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenDefinedOnce( dst, [ 1, 4, 2, [ undefined ], 5 ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, [ 1, 2, 3, 4, 5 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnce() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnce( [], [ 1 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'dstArray is not array';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnce( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnce( [], [ 1 ], [] ) );

  test.case = 'evaluator2 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnce( [], [ 1 ], ( e ) => e, [] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnce( [], [ 1 ], 0, {} ) );
}

//

function arrayFlattenDefinedOnceSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, 2, undefined, 3, undefined, 1 ];
  var src = dst;
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, undefined, 3, undefined, 1 ] );
  test.identical( got, [ 1, 2, undefined, 3, undefined, 1 ] );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var src = dst;
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );

  test.case = 'dst - array, level 3, src contains dst, duplicates';
  var dst = [ [ 1, [ 2, [ undefined ] ] ] ];
  var src = [ dst, 1, 2 ];
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ undefined ] ] ], 1, 2 ] );
  test.identical( got, [ [ 1, [ 2, [ undefined ] ] ], 1, 2 ] );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, duplicates, evaluator';
  var dst = [ [ [ [ [ undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ [ [ [ [ undefined ] ] ] ] ] );
  test.identical( got, [ [ [ [ [ undefined ] ] ] ] ] );

  test.case = 'src contains a few dst, duplicates, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );

  test.case = 'src contains a few dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ 1, [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, [ 1, [ [ [ [ 1, undefined ] ] ] ], 1 ] );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, 1, ( e, ins ) => e === ins );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.push( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2, undefined ] );
  src.add( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2, undefined ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 2 ] );

  test.case = 'dst - array level 2, push self twice, src - flat array, duplicates';
  var dst = [ 1, undefined, [ 2, 3 ] ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2, 3 ] ] );
  test.identical( got, [ 1, undefined, [ 2, 3 ] ] );

  test.case = 'dst - flat array, push self, src - Set, duplicates';
  var dst = [ 1, 2, 3, undefined ];
  dst.push( dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3, undefined ] );
  test.identical( got, [ 1, 2, 3, undefined ] );

  test.case = 'dst - array, level 2, push self twice, src - Set, duplicates';
  var dst = [ 1, [ 1, undefined ] ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ undefined, { a : 2 }, 1 ] );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 1, undefined ], { a : 2 } ] );
  test.identical( got, [ 1, [ 1, undefined ], { a : 2 } ] );

  test.case = 'dst - array, level 2, push self, dst === src';
  var dst = [ 1, [ undefined ], 2 ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, [ 1, [ undefined ], 2 ] );

  test.case = 'dst - array, level 2, push self twice, dst === src, duplicates';
  var dst = [ 1, [ undefined ], 2 ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, [ 1, [ undefined ], 2 ] );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ undefined ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, [ 1, 3 ] );

  test.case = 'dst - array, level 6, push self twice, duplicates';
  var dst = [ undefined, [ [ [ [ [ 1, 1, 2, undefined, undefined ] ] ] ] ] ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 2 ] );

  test.case = 'dst - array level 2, inserts self twice, src - flat array, duplicates';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ] ] );
  test.identical( got, [ 1, [ 2, undefined ] ] );

  test.case = 'dst - flat array, inserts self, src - Set, duplicates';
  var dst = [ 1, 2, undefined ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, undefined, 3 ] );
  test.identical( got, [ 1, 2, undefined, 3 ] );

  test.case = 'dst - array, level 2, inserts self twice, src - Set, duplicates';
  var dst = [ 1, [ 1 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = new Set( [ undefined, { a : 2 }, 'undefined' ] );
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], { a : 2 }, 'undefined' ] );
  test.identical( got, [ 1, [ 1 ], { a : 2 }, 'undefined' ] );

  test.case = 'dst - array, level 2, inserts self, dst === src';
  var dst = [ 1, [ undefined ], 2 ];
  dst.splice( 1, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, [ 1, [ undefined ], 2 ] );

  test.case = 'dst - array, level 2, inserts self twice, dst === src, duplicates';
  var dst = [ 1, [ undefined ], 2 ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, [ 1, [ undefined ], 2 ] );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ 2, undefined ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, [ 1, 3, 2 ] );

  test.case = 'dst - array, level 6, inserts self twice, duplicates';
  var dst = [ undefined, [ [ [ [ [ 1, 1, 2, undefined ] ] ] ] ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlattenDefinedOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, [ undefined ] ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, [ 1, [ 2 ], 2 ] );

  /* */

  test.case = 'dst inserts self twice, duplicates, evaluator';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 2 ] );

  test.case = 'dst inserts self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2, undefined ] ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2, undefined, [ undefined ] ] ];
  var got  = _.arrayFlattenDefinedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 2 ] );

  test.close( 'dst or src contains self, evaluators' );
}

//

function arrayFlattenDefinedOnceSets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'src - flat Set, duplicates';
  var dst = [];
  var src = new Set( [ 1, undefined, 2, 3, 3, 1 ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str', undefined ] ], [ [ undefined ] ], 1, 3 ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str', undefined ] ], 3 ] ] ], 1, 'str', 3, undefined, 3 ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str', undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, 3 ] );
  test.identical( got, [ 1, 'str', { a : 3 }, 3 ] );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 } ] );

  test.case = 'src - flat Set, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, undefined, 3 ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 3 ] );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ 'str', undefined ], 3, undefined ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );

  test.case = 'src - Set, level 3, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ [ 'str', undefined ] ], 3, undefined ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ undefined, [ [ 1, [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ 'str', undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', { a : 3 }, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 'str', { a : 3 }, 3 ] );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], 2, { a : 0 } ] );
  test.identical( got, [ 1, [ undefined, [ 2 ], { a : 0 } ], 2, { a : 0 } ] );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set from two dst in container, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.case = 'src - Set from two dst in container, evaluator1 and evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.case = 'src - Set from two dst in container, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );

  test.case = 'src - Set from two dst in container, equalizer';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenDefinedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );

  test.close( 'dst - array, evaluators' );
}

//

function arrayFlattenDefinedOnceStrictly( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlattenDefinedOnceStrictly( [ 0, 1, 2, undefined ] );
  var expected = [ 0, 1, 2 ];
  test.identical( got, expected );

  test.case = 'level 2';
  var got = _.arrayFlattenDefinedOnceStrictly( [ [ 0 ], [ undefined ] ] );
  var expected = [ 0 ];
  test.identical( got, expected );

  test.case = 'diff levels';
  var got = _.arrayFlattenDefinedOnceStrictly( [ [ 0 ], 1, [ 2, [ 3, undefined ] ] ] );
  var expected = [ 0, 1, 2, 3 ];
  test.identical( got, expected );

  var got = _.arrayFlattenDefinedOnceStrictly( [ 0, [ [ undefined ], 2 ], 3, 4 ] );
  var expected = [ 0, 2, 3, 4 ];
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - string';
  var dst = [];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, 'str' );
  test.identical( dst, [ 'str' ] );
  test.identical( got, [ 'str' ] );

  test.case = 'dst - empty array, src - undefined';
  var dst = [];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, undefined );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [] );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - empty array, src - flat array';
  var dst = [];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ 1, 2, undefined ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - array, level 2';
  var dst = [];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ [ 1 ], [ undefined ], 3 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, [ 1, 3 ] );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ [ 1, [ 2, [ 3, undefined ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ [ [ [ [ 1, undefined ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, [ 1 ] );

  /* */

  test.case = 'dst - flat array, src - empty array';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [] );
  test.identical( dst, [ 1, 2, 3, undefined ] );
  test.identical( got, [ 1, 2, 3, undefined ] );

  test.case = 'dst - flat array, src - array, level 3';
  var dst = [ 1, 2, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ [ [ undefined ] ] ] );
  test.identical( dst, [ 1, 2, undefined ] );
  test.identical( got, [ 1, 2, undefined ] );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ 1, 2, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ [ 4 ], [ 5 ], [ undefined ], [ 7 ] ] );
  test.identical( dst, [ 1, 2, undefined, 4, 5, 7 ] );
  test.identical( got, [ 1, 2, undefined, 4, 5, 7 ] );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ -1, 0, undefined ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [ -1, 0, undefined, 1, 2, 3 ] );
  test.identical( got, [ -1, 0, undefined, 1, 2, 3 ] );

  test.case = 'dst - flat array, src - array, level 5';
  var dst = [ 1, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ [ [ [ [ 2, undefined ] ] ] ] ] );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.identical( got, [ 1, undefined, 2 ] );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ 4, 5, undefined ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 4, 5 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ undefined ], 4, 5 ] );

  test.case = 'dst - array, level 2, src - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ [ 1 ], [ 2 ], [ undefined ] ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );

  test.case = 'dst - array, level 2, src - number';
  var dst = [ 1, 3, [ 5, undefined ] ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, 5 );
  test.identical( dst, [ 1, 3, [ 5, undefined ], 5 ] );
  test.identical( got, [ 1, 3, [ 5, undefined ], 5 ] );

  /* */

  test.case = 'dst - flat array, src - flat array, evaluator';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ 5, 6, 7, undefined ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, undefined, 5, 6, 7 ] );
  test.identical( got, [ 1, 2, 3, undefined, 5, 6, 7 ] );

  test.case = 'dst - flat array, src - flat array, evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ 5, 6, 7, undefined ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, undefined, 5, 6, 7 ] );
  test.identical( got, [ 1, 2, 3, undefined, 5, 6, 7 ] );

  test.case = 'dst - flat array, src - flat array, evaluator1 - fromIndex, evaluator2, has duplicates';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ 1, 2, 7, undefined ], 2, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, undefined, 1, 2, 7 ] );
  test.identical( got, [ 1, 2, 3, undefined, 1, 2, 7 ] );

  test.case = 'dst - flat array, src - flat array, equalizer';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenDefinedOnceStrictly( dst, [ 5, 6, 7, undefined ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, undefined, 5, 6, 7 ] );
  test.identical( got, [ 1, 2, 3, undefined, 5, 6, 7 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnceStrictly() );

  test.case = 'dstArray is not an array';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnceStrictly( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnceStrictly( [], [ 1 ], [] ) );

  test.case = 'duplicates in dstArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnceStrictly( [ 0, 0 ], [ 3, 4 ] ) );

  test.case = 'duplicates in insArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnceStrictly( [], [ 1, 1, 2, 2, 3, 3 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnceStrictly( [], [ [ 1 ], [ 1 ], [ 2 ], [ 2 ], [ 3 ], [ 3 ] ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenDefinedOnceStrictly( [], [ [ 1, 1, [ 2, 2, [ 3, 3 ] ] ] ] ) );
}

//

function arrayFlattenDefinedOnceStrictlySame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var src = dst;
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ undefined ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ undefined ] ] ], 1, 2 ] );
  test.identical( got, [ [ 1, [ 2, [ undefined ] ] ], 1, 2 ] );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, [ [ [ [ [ 1, undefined ] ] ] ], 1, 1, 1, 1 ] );

  test.case = 'src contains dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, 1, ( e ) => e );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, 1, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, [ [ [ [ [ 1, undefined ] ] ] ], 1, 1, 1, 1 ] );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.push( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2, undefined ] );
  src.add( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2, undefined ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 2 ] );

  test.case = 'dst - flat array, push self, src - Set';
  var dst = [ 1, 2, 3, undefined ];
  dst.push( dst );
  var src = new Set( [ 4, [ undefined ], 6 ] );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, undefined, 4, 6 ] );
  test.identical( got, [ 1, 2, 3, undefined, 4, 6 ] );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ undefined ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, [ 1, 3 ] );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 2 ] );

  test.case = 'dst - flat array, inserts self, src - Set';
  var dst = [ 1, 2, 3, undefined ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 4, [ [ undefined ], 5 ] ] );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, undefined, 4, 5 ] );
  test.identical( got, [ 1, 2, 3, undefined, 4, 5 ] );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ undefined ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, [ 1, 3 ] );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 2, [ 3, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2, 3 ] );
  test.identical( got, [ 1, [ 2 ], 2, 3 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, [ undefined ] ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, [ 1, [ 2 ], 1, 2 ] );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ null, [ 2, [ undefined ] ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], null, 2 ] );
  test.identical( got, [ 1, [ 2 ], null, 2 ] );

  /* */

  test.case = 'dst inserts self, evaluator';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  var src = [ 3, [ 2, undefined ] ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 3, 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 3, 2 ] );

  test.case = 'dst inserts self, evaluator1 and evaluator2';
  var dst = [ 1, [ 2, [ undefined ] ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2, [ undefined ] ] ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2, [ undefined ] ], 1, 2 ] );
  test.identical( got, [ 1, [ 2, [ undefined ] ], 1, 2 ] );

  test.case = 'dst - array, src push self, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2, [ undefined ] ] ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 1, 2 ] );

  test.case = 'dst - array, src push self, equalizer';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2, undefined ] ];
  var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, [ 1, [ 2, undefined ], 1, 2 ] );

  test.close( 'dst or src contains self, evaluators' );

  if( !Config.debug )
  return;

  test.case = 'dst === src';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, 2, 3, undefined ];
    var src = dst;
    var got  = _.arrayFlattenDefinedOnceStrictly( dst, src );
  });

  test.case = 'src contains a few dst, simple evaluator';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
    var src = [ dst, dst, dst, dst ];
    var got  = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e );
  });

  test.case = 'dst push self twice';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
    dst.push( dst );
    dst.push( dst );
    var got  = _.arrayFlattenDefinedOnceStrictly( dst );
  });
}

//

function arrayFlattenDefinedOnceStrictlySets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, [] );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, [ 'str' ], 2, undefined, { a : 0 }, { a : 0 } ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 2, { a : 0 }, { a : 0 } ] );
  test.identical( got, [ 1, 'str', 2, { a : 0 }, { a : 0 } ] );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str', undefined ] ], 3 ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, [ 1, 'str', 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str', undefined ] ], 3 ] ] ], 2, 4, 6 ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3, 2, 4, 6 ] );
  test.identical( got, [ 1, 'str', 3, 2, 4, 6 ] );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str', undefined ], { a : 3 } ] ] ], [ [ [ 2, [ undefined ], 4 ] ] ] ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, 2, 4 ] );
  test.identical( got, [ 1, 'str', { a : 3 }, 2, 4 ] );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 } ] );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3, undefined ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ 'str', undefined ], 3 ] ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ [ 'str', undefined ] ], 3 ] ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 2, [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 2, undefined ] ] ], [ [ [ [ 'str', undefined ] ] ] ] ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str' ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str' ] );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3, undefined ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );

  test.case = 'src - Set from dst, evaluator1 and evaluator2';
  var dst = [ 1, [ undefined ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );
  test.identical( got, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );

  test.case = 'src - Set from dst, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ undefined ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src, 3, ( ins ) => ins );
  test.identical( dst, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );
  test.identical( got, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );

  test.case = 'src - Set from dst, equalizer';
  var dst = [ 1, [ undefined ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenDefinedOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );
  test.identical( got, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );

  test.close( 'dst - array, evaluators' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'src - Set from dst';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, [ [ undefined ], { a : 0 } ] ];
    var src = new Set( dst );
    var got = _.arrayFlattenDefinedOnceStrictly( dst, src );
  });
}

//

function arrayFlattenedDefined( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlattenedDefined( [ 0, 1, 2, 3 ] );
  var expected = 4;
  test.identical( got, expected );

  test.case = 'array, level 2';
  var got = _.arrayFlattenedDefined( [ [ 0, 0 ], [ 1, 1 ] ] );
  var expected = 4;
  test.identical( got, expected );

  test.case = 'array, level 3';
  var got = _.arrayFlattenedDefined( [ [ 0 ], 0, 1, [ [ 0 ], 1 ] ] );
  var expected = 5;
  test.identical( got, expected );

  test.case = 'array, level 5';
  var got = _.arrayFlattenedDefined( [ [ [ [ [ 0 ] ] ] ] ] );
  var expected = 1;
  test.identical( got, expected );

  test.case = 'array from other arrays, level 5';
  var src = [ [ [ [ [ 0 ] ] ] ] ];
  var got = _.arrayFlattenedDefined( [ src, src, src, src ] );
  var expected = 4;
  test.identical( got, expected );

  test.case = 'array, level 5, has undefined';
  var dst = [ undefined, [ undefined, [ undefined, [ undefined, [ 0 ] ] ] ] ];
  var got = _.arrayFlattenedDefined( dst );
  var expected = 1;
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got  = _.arrayFlattenedDefined( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - flat array';
  var dst = [];
  var got  = _.arrayFlattenedDefined( dst, [ 1, 2, undefined ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - array, level 2';
  var dst = [];
  var got  = _.arrayFlattenedDefined( dst, [ [ 1 ], [ 2 ], [ undefined ] ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - array, level 3';
  var dst = [];
  var got  = _.arrayFlattenedDefined( dst, [ [ 1, [ 2, [ undefined ] ] ] ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - array, level 5';
  var dst = [];
  var got  = _.arrayFlattenedDefined( dst, [ [ [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - array from arrays level 5';
  var src = [ [ [ [ [ undefined ] ] ] ] ];
  var dst = [];
  var got = _.arrayFlattenedDefined( dst, [ src, src, src, src ] );
  var expected = [];
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - flat array';
  var dst = [ 1, undefined, 3 ];
  var got  = _.arrayFlattenedDefined( dst, [ 1, 2, undefined ] );
  test.identical( dst, [ 1, undefined, 3, 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenedDefined( dst, [ [ 1 ], [ 2 ], [ undefined ] ] );
  test.identical( dst, [ 1, undefined, 3, 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - flat array, src - array, level 3';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenedDefined( dst, [ [ 1, [ 2, [ undefined ] ] ] ] );
  test.identical( dst, [ 1, undefined, 3, 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src - array, level 5';
  var dst = [ 1 ];
  var got = _.arrayFlattenedDefined( dst, [ [ [ [ [ 1 ] ] ] ], [ [ [ [ 1 ] ] ] ], [ [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [ 1, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ 1, [ 2, 3 ] ];
  var got  = _.arrayFlattenedDefined( dst, [ undefined ] );
  test.identical( dst, [ 1, [ 2, 3 ] ] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - multiple arrays with diff levels';
  var dst = [];
  var got = _.arrayFlattenedDefined( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - number';
  var dst = [];
  var got  = _.arrayFlattenedDefined( dst, 1 );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, src - number';
  var dst = [ 1, 1, 3, 3, [ 5, 5 ] ];
  var got = _.arrayFlattenedDefined( dst, 5 );
  var expected = [ 1, 1, 3, 3, [ 5, 5 ], 5 ];
  test.identical( dst, expected );
  test.identical( got, 1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefined() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefined( [], [ 1 ], [ 'extra' ] ) );

  test.case = 'dstArray is not arrayLike';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefined( 1, [ 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefined( null, [ 1 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefined( new U8x( 2 ), [ 1 ] ) );
}

//

function arrayFlattenedDefinedSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - flat array';
  var dst = [ 1, 2, undefined ];
  var src = dst;
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, 2, undefined, 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var src = dst;
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ undefined ] ] ] ];
  var src = [ dst, 4, 5 ];
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ undefined ] ] ], 1, 2, 4, 5 ] );
  test.identical( got, 4 );

  test.case = 'dst - array, level 5, src contains a few dst';
  var dst = [ [ [ [ [ undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ [ [ [ [ undefined ] ] ] ] ] );
  test.identical( got, 0 );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'src push self';
  var dst = [];
  var src = [ 1, undefined ];
  src.push( src );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'src push self twice';
  var dst = [];
  var src = [ 1, [ undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, [ undefined ] ] );
  src.add( src );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ [ undefined ] ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst push self';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst push self twice';
  var dst = [ 1, undefined ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst push self, src - Set';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 'str', { a : 2 } ] );
  test.identical( got, 2 );

  test.case = 'dst push self twice, src - Set';
  var dst = [ 1, undefined ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ 'str', { a : 2 } ] );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 'str', { a : 2 } ] );
  test.identical( got, 2 );

  test.case = 'dst push self, dst === src';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst push self twice, dst === src';
  var dst = [ 1, undefined ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1, 1 ] );
  test.identical( got, 3 );

  test.case = 'dst push self';
  var dst = [ 1, undefined ];
  dst.push( dst );
  var got  = _.arrayFlattenedDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst push self twice';
  var dst = [ 1, undefined, undefined ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlattenedDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'src inserts self';
  var dst = [];
  var src = [ 1, undefined ];
  src.splice( 1, 0, dst );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'src inserts self twice';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.splice( 0, 0, dst );
  src.splice( 2, 0, dst );
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst inserts self';
  var dst = [ 1, 2, undefined ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, 2, undefined, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self twice';
  var dst = [ 1, undefined ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var src = [ 3 ];
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 3 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self, dst === src';
  var dst = [ 1, undefined ];
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1 ] );
  test.identical( got, 2 );

  test.case = 'dst inserts self twice, dst === src';
  var dst = [ 1, undefined ];
  dst.splice( 3, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, 1, 1, 1 ] );
  test.identical( got, 3 );

  test.case = 'dst inserts self';
  var dst = [ 1, undefined ];
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlattenedDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self twice';
  var dst = [ 1, undefined, [ undefined ] ];
  dst.splice( 0, 0, dst );
  dst.splice( 2, 0, dst );
  var got  = _.arrayFlattenedDefined( dst );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  test.close( 'dst or src contains self' );
}

//

function arrayFlattenedDefinedSets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, 2, undefined ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ undefined ], 3 ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ undefined ], 3 ] ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ 1, [ undefined ], 3 ] ] ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from two array level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ undefined ], 3 ] ] ] ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, { a : 3 }, 1, 3 ] );
  test.identical( got, 4 );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, 2, undefined ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ undefined ], 3 ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ undefined, undefined ], 3 ] ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 1, [ undefined ], 3 ] ] ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ undefined ], 3 ] ] ] ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, { a : 3 }, 1, 3 ] );
  test.identical( got, 4 );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], 1, 2, { a : 0 } ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedDefined( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2, { a : 0 }, 1, 2, { a : 0 }, 1, 2, { a : 0 } ] );
  test.identical( got, 9 );

  test.close( 'dst - array' );
}

//

function arrayFlattenedDefinedOnce( test )
{
  test.open( 'single argument' );

  test.case = 'flat array, without duplicates';
  var dst = [ 0, 1, undefined, 3 ];
  var got = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 0, 1, 3 ] );
  test.identical( got, 3 );

  test.case = 'flat array, duplicates';
  var dst = [ 0, 1, 0, undefined, 1 ];
  var got = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, 2 );

  test.case = 'array, level 2, duplicates';
  var dst = [ [ 0, 0, undefined ], [ 1, 1, undefined ] ];
  var got = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, 2 );

  test.case = 'array with diff levels, duplicates';
  var dst = [ 1, undefined, [ [ 0, undefined ], 1, undefined ], 1, 0 ];
  var got = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 1, 0 ] );
  test.identical( got, 2 );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - undefined';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnce( dst, undefined );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnce( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - flat array, duplicates';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 1, 1, 2, 2, undefined, 3, 3 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 2, duplicates';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnce( dst, [ [ 1 ], [ 1 ], [ undefined ], [ 2 ], [ 3 ], [ 3 ], [ undefined ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 3, duplicates';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnce( dst, [ [ 1, 1, undefined, [ 2, 2, undefined, [ 3, 3, undefined ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 5, duplicates';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnce( dst, [ undefined, [ [ [ [ 1, 1, undefined, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst - flat array, src - string';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenedDefinedOnce( dst, 'str' );
  test.identical( dst, [ 1, undefined, 3, 'str' ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, undefined, 3, 4 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 4 ] );
  test.identical( dst, [ 1, undefined, 3, 4 ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - array, level 2, duplicates';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ [ 1 ], [ 2 ], [ 3 ], [ 4 ], [ undefined ] ] );
  test.identical( dst, [ 1, undefined, 3, 2, 4 ] );
  test.identical( got, 2 );

  test.case = 'dst - flat array, src - array, level 3, duplicates';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 1, 2, 3, [ [ undefined ] ] ] );
  test.identical( dst, [ 1, undefined, 3, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, src - array, level 5, duplicates';
  var dst = [ 1 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ [ [ [ [ 1, 1, undefined, undefined, 1 ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2, undefined ], [ 3 ] ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 1, 2, undefined, 3 ] );
  test.identical( dst, [ [ 1 ], [ 2, undefined ], [ 3 ], 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - array with diff level arrays, duplicates';
  var dst = [ 1, 4 ];
  var got  = _.arrayFlattenedDefinedOnce( dst, [ [ 1 ], [ [ 2, undefined ] ], [ 3, [ [ [ 4 ] ] ] ] ] );
  test.identical( dst, [ 1, 4, 2, 3 ] );
  test.identical( got, 2 );

  /* */

  test.case = 'evaluator1';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 1, 4, 2, 5, undefined ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 1 );

  test.case = 'evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 1, 4, 2, 5, [ [ undefined ] ] ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 1 );

  test.case = 'evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 1, 4, 2, [ undefined ], 5 ], 1, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, 4, 1, 5 ] );
  test.identical( got, 2 );

  test.case = 'equilizer';
  var dst = [ 1, 2, 3, 4 ];
  var got = _.arrayFlattenedDefinedOnce( dst, [ 1, 4, 2, [ undefined ], 5 ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, 4, 5 ] );
  test.identical( got, 1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnce() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnce( [], [ 1 ], ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'dstArray is not array';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnce( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnce( [], [ 1 ], [] ) );

  test.case = 'evaluator2 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnce( [], [ 1 ], ( e ) => e, [] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnce( [], [ 1 ], 0, {} ) );
}

//

function arrayFlattenedDefinedOnceSame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - flat array, duplicates';
  var dst = [ 1, 2, undefined, 3, undefined, 1 ];
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, undefined, 3, undefined, 1 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 3, src contains dst, duplicates';
  var dst = [ [ 1, [ 2, [ undefined ] ] ] ];
  var src = [ dst, 1, 2 ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ undefined ] ] ], 1, 2 ] );
  test.identical( got, 2 );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, duplicates, evaluator';
  var dst = [ [ [ [ [ undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ [ [ [ [ undefined ] ] ] ] ] );
  test.identical( got, 0 );

  test.case = 'src contains a few dst, duplicates, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.case = 'src contains a few dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ 1, [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, 1, ( e, ins ) => e === ins );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2, undefined ] );
  src.add( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2, undefined ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array level 2, push self twice, src - flat array, duplicates';
  var dst = [ 1, undefined, [ 2, 3 ] ];
  dst.push( dst );
  dst.push( dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2, 3 ] ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, push self, src - Set, duplicates';
  var dst = [ 1, 2, 3, undefined ];
  dst.push( dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3, undefined ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, push self twice, src - Set, duplicates';
  var dst = [ 1, [ 1, undefined ] ];
  dst.push( dst );
  dst.push( dst );
  var src = new Set( [ undefined, { a : 2 }, 1 ] );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 1, undefined ], { a : 2 } ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, push self, dst === src';
  var dst = [ 1, [ undefined ], 2 ];
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, push self twice, dst === src, duplicates';
  var dst = [ 1, [ undefined ], 2 ];
  dst.push( dst );
  dst.push( dst );
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, 0 );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ undefined ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 6, push self twice, duplicates';
  var dst = [ undefined, [ [ [ [ [ 1, 1, 2, undefined, undefined ] ] ] ] ] ];
  dst.push( dst );
  dst.push( dst );
  var got  = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array level 2, inserts self twice, src - flat array, duplicates';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1 ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ] ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, inserts self, src - Set, duplicates';
  var dst = [ 1, 2, undefined ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 1, 2, 3 ] );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, undefined, 3 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, inserts self twice, src - Set, duplicates';
  var dst = [ 1, [ 1 ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = new Set( [ undefined, { a : 2 }, 'undefined' ] );
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ 1 ], { a : 2 }, 'undefined' ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 2, inserts self, dst === src';
  var dst = [ 1, [ undefined ], 2 ];
  dst.splice( 1, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, inserts self twice, dst === src, duplicates';
  var dst = [ 1, [ undefined ], 2 ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined ], 2 ] );
  test.identical( got, 0 );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ 2, undefined ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 1, 3, 2 ] );
  test.identical( got, 3 );

  test.case = 'dst - array, level 6, inserts self twice, duplicates';
  var dst = [ undefined, [ [ [ [ [ 1, 1, 2, undefined ] ] ] ] ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var got  = _.arrayFlattenedDefinedOnce( dst );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, [ undefined ] ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], 2 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst inserts self twice, duplicates, evaluator';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst inserts self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2 ] ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2, undefined ] ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  dst.splice( 0, 0, dst );
  var src = [ 1, [ 2, undefined, [ undefined ] ] ];
  var got  = _.arrayFlattenedDefinedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, 1 );

  test.close( 'dst or src contains self, evaluators' );
}

//

function arrayFlattenedDefinedOnceSets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'src - flat Set, duplicates';
  var dst = [];
  var src = new Set( [ 1, undefined, 2, 3, 3, 1 ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str', undefined ] ], [ [ undefined ] ], 1, 3 ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str', undefined ] ], 3 ] ] ], 1, 'str', 3, undefined, 3 ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str', undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, 3 ] );
  test.identical( got, 4 );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, 0 );

  test.case = 'src - flat Set, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, undefined, 3 ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 3 ] );
  test.identical( got, 1 );

  test.case = 'src - Set, level 2, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ 'str', undefined ], 3, undefined ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 3, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 1, [ [ [ 'str', undefined ] ], 3, undefined ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ undefined, [ [ 1, [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 1, [ 'str', undefined ], { a : 3 }, undefined ] ] ], [ [ [ 1, [ 'str' ], 3 ] ] ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 'str', { a : 3 }, 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from dst';
  var dst = [ 1, [ undefined, [ 2 ], { a : 0 } ] ];
  var src = new Set( dst );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, [ undefined, [ 2 ], { a : 0 } ], 2, { a : 0 } ] );
  test.identical( got, 2 );

  test.case = 'src - Set from two dst in container';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set from two dst in container, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.case = 'src - Set from two dst in container, evaluator1 and evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src, ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.case = 'src - Set from two dst in container, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src, 1, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from two dst in container, equalizer';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ dst, [ dst, dst ] ] );
  var got = _.arrayFlattenedDefinedOnce( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2 ] );
  test.identical( got, 1 );

  test.close( 'dst - array, evaluators' );
}

//

function arrayFlattenedDefinedOnceStrictly( test )
{
  test.open( 'single argument' );

  test.case = 'flat array';
  var got = _.arrayFlattenedDefinedOnceStrictly( [ 0, 1, 2, undefined ] );
  var expected = 3;
  test.identical( got, expected );

  test.case = 'level 2';
  var got = _.arrayFlattenedDefinedOnceStrictly( [ [ 0 ], [ undefined ] ] );
  var expected = 1;
  test.identical( got, expected );

  test.case = 'diff levels';
  var got = _.arrayFlattenedDefinedOnceStrictly( [ [ 0 ], 1, [ 2, [ 3, undefined ] ] ] );
  var expected = 4;
  test.identical( got, expected );

  var got = _.arrayFlattenedDefinedOnceStrictly( [ 0, [ [ undefined ], 2 ], 3, 4 ] );
  var expected = 4;
  test.identical( got, expected );

  test.close( 'single argument' );

  /* - */

  test.case = 'dst - empty array, src - string';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, 'str' );
  test.identical( dst, [ 'str' ] );
  test.identical( got, 1 );

  test.case = 'dst - empty array, src - undefined';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, undefined );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - empty array';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - empty array, src - flat array';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ 1, 2, undefined ] );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - array, level 2';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ 1 ], [ undefined ], 3 ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ 1, [ 2, [ 3, undefined ] ] ] ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - empty array, src - array, level 4';
  var dst = [];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ [ [ [ 1, undefined ] ] ] ] ] );
  test.identical( dst, [ 1 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst - flat array, src - empty array';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [] );
  test.identical( dst, [ 1, 2, 3, undefined ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - array, level 3';
  var dst = [ 1, 2, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ [ undefined ] ] ] );
  test.identical( dst, [ 1, 2, undefined ] );
  test.identical( got, 0 );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ 1, 2, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ 4 ], [ 5 ], [ undefined ], [ 7 ] ] );
  test.identical( dst, [ 1, 2, undefined, 4, 5, 7 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - array, level 2';
  var dst = [ -1, 0, undefined ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ 1 ], [ [ 2 ] ], [ 3, [ [ [ undefined ] ] ] ] ] );
  test.identical( dst, [ -1, 0, undefined, 1, 2, 3 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - array, level 5';
  var dst = [ 1, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ [ [ [ 2, undefined ] ] ] ] ] );
  test.identical( dst, [ 1, undefined, 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - array, level 2, src - flat array';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ 4, 5, undefined ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 4, 5 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 2, src - array, level 2';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ [ 1 ], [ 2 ], [ undefined ] ] );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 2, src - number';
  var dst = [ 1, 3, [ 5, undefined ] ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, 5 );
  var expected = [ 1, 3, [ 5, undefined ], 5 ];
  test.identical( dst, expected );
  test.identical( got, 1 );

  /* */

  test.case = 'dst - flat array, src - flat array, evaluator';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ 5, 6, 7, undefined ], ( e ) => e );
  test.identical( dst, [ 1, 2, 3, undefined, 5, 6, 7 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - flat array, evaluator1 and evaluator2';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ 5, 6, 7, undefined ], ( e ) => e, ( ins ) => ins );
  test.identical( dst, [ 1, 2, 3, undefined, 5, 6, 7 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - flat array, evaluator1 - fromIndex, evaluator2, has duplicates';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ 1, 2, 7, undefined ], 2, ( e ) => e );
  test.identical( dst, [ 1, 2, 3, undefined, 1, 2, 7 ] );
  test.identical( got, 3 );

  test.case = 'dst - flat array, src - flat array, equalizer';
  var dst = [ 1, 2, 3, undefined ];
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, [ 5, 6, 7, undefined ], ( a, b ) => a === b );
  test.identical( dst, [ 1, 2, 3, undefined, 5, 6, 7 ] );
  test.identical( got, 3 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnceStrictly() );

  test.case = 'dstArray is not an array';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnceStrictly( 1, [ 1 ] ) );

  test.case = 'evaluator1 is not a routine or a number';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnceStrictly( [], [ 1 ], [] ) );

  test.case = 'duplicates in dstArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnceStrictly( [ 0, 0 ], [ 3, 4 ] ) );

  test.case = 'duplicates in insArray';
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnceStrictly( [], [ 1, 1, 2, 2, 3, 3 ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnceStrictly( [], [ [ 1 ], [ 1 ], [ 2 ], [ 2 ], [ 3 ], [ 3 ] ] ) );
  test.shouldThrowErrorSync( () => _.arrayFlattenedDefinedOnceStrictly( [], [ [ 1, 1, [ 2, 2, [ 3, 3 ] ] ] ] ) );
}

//

function arrayFlattenedDefinedOnceStrictlySame( test )
{
  test.case = 'dst - empty array';
  var dst = [];
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'dst - array, level 2, no duplicates';
  var dst = [ [ 1 ], [ 2 ], [ undefined ] ];
  var src = dst;
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ [ 1 ], [ 2 ], [ undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, level 3, src contains dst';
  var dst = [ [ 1, [ 2, [ undefined ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ [ 1, [ 2, [ undefined ] ] ], 1, 2 ] );
  test.identical( got, 2 );

  /* - */

  test.open( 'evaluators' );

  test.case = 'src contains a few dst, evaluator1 and evaluator2';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, 4 );

  test.case = 'src contains dst, duplicates, evaluator1 - fromIndex, evaluator2, duplicates';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, 1, ( e ) => e );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1 ] );
  test.identical( got, 1 );

  test.case = 'src contains a few dst, duplicates, equalizer';
  var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
  var src = [ dst, dst, dst, dst ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, 1, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ [ [ [ [ 1, undefined ] ] ] ], 1, 1, 1, 1 ] );
  test.identical( got, 4 );

  test.close( 'evaluators' );

  /* - */

  test.open( 'dst or src contains self' );

  test.case = 'dst - empty array, src push self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src push self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self';
  var dst = [];
  var src = new Set( [ 1, 2, undefined ] );
  src.add( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src - Set, add self twice';
  var dst = [];
  var src = new Set( [ 1, [ 2, undefined ] ] );
  src.add( src );
  src.add( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, push self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.push( dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, push self, src - Set';
  var dst = [ 1, 2, 3, undefined ];
  dst.push( dst );
  var src = new Set( [ 4, [ undefined ], 6 ] );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, undefined, 4, 6 ] );
  test.identical( got, 2 );

  test.case = 'dst = array, level 3, push self';
  var dst = [ 1, [ 3, [ undefined ] ] ];
  dst.push( dst );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  /* */

  test.case = 'dst - empty array, src inserts self';
  var dst = [];
  var src = [ 1, 2, undefined ];
  src.splice( 1, 0, src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - empty array, src inserts self twice';
  var dst = [];
  var src = [ 1, [ 2, undefined ] ];
  src.splice( 0, 0, src );
  src.splice( 2, 0, src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array level 2, inserts self, src - flat array';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 0, 0, dst );
  var src = [ 2 ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, [ 2, undefined ], 2 ] );
  test.identical( got, 1 );

  test.case = 'dst - flat array, inserts self, src - Set';
  var dst = [ 1, 2, 3, undefined ];
  dst.splice( 1, 0, dst );
  var src = new Set( [ 4, [ [ undefined ], 5 ] ] );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 2, 3, undefined, 4, 5 ] );
  test.identical( got, 2 );

  test.case = 'dst = array, level 3, inserts self';
  var dst = [ 1, [ 3, [ undefined ] ] ];
  dst.splice( 1, 0, dst );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 2 );

  test.close( 'dst or src contains self' );

  /* - */

  test.open( 'dst or src contains self, evaluators' );

  test.case = 'dst - array, src push self twice, duplicates, evaluator';
  var dst = [ 1, [ 2 ] ];
  var src = [ 2, [ 3, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 2, 3 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 and evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, [ undefined ] ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2 ] ];
  var src = [ 1, [ 2, undefined ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2 ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self twice, duplicates, equalizer';
  var dst = [ 1, [ 2 ] ];
  var src = [ null, [ 2, [ undefined ] ] ];
  src.push( src );
  src.push( src );
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e, ins ) => e === ins );
  test.identical( dst, [ 1, [ 2 ], null, 2 ] );
  test.identical( got, 2 );

  /* */

  test.case = 'dst inserts self, evaluator';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  var src = [ 3, [ 2, undefined ] ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 3, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst inserts self, evaluator1 and evaluator2';
  var dst = [ 1, [ 2, [ undefined ] ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2, [ undefined ] ] ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ 2, [ undefined ] ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2, [ undefined ] ] ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, 2, ( e ) => e );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.case = 'dst - array, src push self, equalizer';
  var dst = [ 1, [ 2, undefined ] ];
  dst.splice( 1, 0, dst );
  var src = [ 1, [ 2, undefined ] ];
  var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ 2, undefined ], 1, 2 ] );
  test.identical( got, 2 );

  test.close( 'dst or src contains self, evaluators' );

  if( !Config.debug )
  return;

  test.case = 'dst === src';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, 2, 3, undefined ];
    var src = dst;
    var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  });

  test.case = 'src contains a few dst, simple evaluator';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
    var src = [ dst, dst, dst, dst ];
    var got  = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e );
  });

  test.case = 'dst push self twice';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ [ [ [ [ 1, undefined ] ] ] ] ];
    dst.push( dst );
    dst.push( dst );
    var got  = _.arrayFlattenedDefinedOnceStrictly( dst );
  });
}

//

function arrayFlattenedDefinedOnceStrictlySets( test )
{
  test.open( 'dst - empty array' );

  test.case = 'src - empty Set';
  var dst = [];
  var src = new Set();
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [];
  var src = new Set( [ 1, [ 'str' ], 2, undefined, { a : 0 }, { a : 0 } ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 2, { a : 0 }, { a : 0 } ] );
  test.identical( got, 5 );

  test.case = 'src - Set, level 2';
  var dst = [];
  var src = new Set( [ 1, [ [ 'str', undefined ] ], 3 ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [];
  var src = new Set( [ 1, [ [ [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ [ 'str', undefined ] ], 3 ] ] ], 2, 4, 6 ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', 3, 2, 4, 6 ] );
  test.identical( got, 6 );

  test.case = 'src - Set from two array level 5, duplicates';
  var dst = [];
  var src = new Set( [ [ [ [ 1, [ 'str', undefined ], { a : 3 } ] ] ], [ [ [ 2, [ undefined ], 4 ] ] ] ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, 'str', { a : 3 }, 2, 4 ] );
  test.identical( got, 5 );

  test.close( 'dst - empty array' );

  /* - */

  test.open( 'dst - array' );

  test.case = 'src - empty Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set();
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 } ] );
  test.identical( got, 0 );

  test.case = 'src - flat Set';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3, undefined ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set, level 2';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ 'str', undefined ], 3 ] ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 3';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, [ [ [ 'str', undefined ] ], 3 ] ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set, level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ 2, [ 'str', undefined ], 3 ] ] ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str', 3 ] );
  test.identical( got, 3 );

  test.case = 'src - Set from two array level 5';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ [ [ [ 2, undefined ] ] ], [ [ [ [ 'str', undefined ] ] ] ] ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 'str' ] );
  test.identical( got, 2 );

  test.close( 'dst - array' );

  /* - */

  test.open( 'dst - array, evaluators' );

  test.case = 'src - Set, evaluator';
  var dst = [ 1, undefined, [ 2 ], { a : 0 } ];
  var src = new Set( [ 2, 3, undefined ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e );
  test.identical( dst, [ 1, undefined, [ 2 ], { a : 0 }, 2, 3 ] );
  test.identical( got, 2 );

  test.case = 'src - Set from dst, evaluator1 and evaluator2';
  var dst = [ 1, [ undefined ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e ) => e, ( ins ) => ins + 1 );
  test.identical( dst, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );
  test.identical( got, 2 );

  test.case = 'src - Set from dst, evaluator1 - fromIndex, evaluator2';
  var dst = [ 1, [ undefined ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src, 3, ( ins ) => ins );
  test.identical( dst, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );
  test.identical( got, 2 );

  test.case = 'src - Set from dst, equalizer';
  var dst = [ 1, [ undefined ], { a : 0 } ];
  var src = new Set( [ dst ] );
  var got = _.arrayFlattenedDefinedOnceStrictly( dst, src, ( e, ins ) => e === ins + 1 );
  test.identical( dst, [ 1, [ undefined ], { a : 0 }, 1, { a : 0 } ] );
  test.identical( got, 2 );

  test.close( 'dst - array, evaluators' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'src - Set from dst';
  test.shouldThrowErrorSync( () =>
  {
    var dst = [ 1, [ [ undefined ], { a : 0 } ] ];
    var src = new Set( dst );
    var got = _.arrayFlattenedDefinedOnceStrictly( dst, src );
  });
}

//
// array replace
//

function arrayReplace( test )
{
  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplace( dst, undefined, 0 );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplace( dst, 0, 0 );
  var expected = [];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplace( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplace( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'Several elements';
  var dst = [ true, true, true, true, false, false ];
  var got = _.arrayReplace( dst, false, true );
  var expected = [ true, true, true, true, true, true ];
  test.identical( got, expected );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayReplace( dst, 1, 0 );
  test.identical( got, [ 0, 0, 0 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 1 ];
  var got = _.arrayReplace( dst, 1, 0 );
  test.identical( got, [ 0, 2, 0 ] );
  test.true( got === dst );

  test.case = 'No match';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplace( dst, 4, 0 );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( got === dst );

  function onEqualize( a, b )
  {
    return a.value === b;
  };

  var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
  var got = _.arrayReplace( dst, 1, { value : 0 }, onEqualize );
  test.identical( got, [ { value : 0 }, { value : 0 }, { value : 2 } ] );
  test.true( got === dst );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplace( );
  })

  test.case = 'first arg is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplace( 1, 1, 1 );
  })

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplace( 1, 1, 1, 1);
  })
}

//

function arrayReplaceOnce( test )
{

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplaceOnce( dst, 0, 0 );
  var expected = [];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplaceOnce( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplaceOnce( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false, false ];
  var got = _.arrayReplaceOnce( dst, false, true );
  var expected = [ true, true, true, true, true, false ];
  test.true( got === dst );

  test.case = 'element not exists';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceOnce( dst, [ 1 ], [ 4 ] );
  var expected = [ 1, 2, 3 ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'replace just first match';
  var dst = [ 0, 0, 0, 0, 0, 0 ];
  var got = _.arrayReplaceOnce( dst, 0, 1 );
  var expected = [ 1, 0, 0, 0, 0, 0 ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'equalize';
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  };
  var got = _.arrayReplaceOnce( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( got, expected );
  test.true( got === dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnce();
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnce( [ 1, 2, undefined, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnce( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnce( 'wrong argument', undefined, 3 );
  });
}

//

function arrayReplaceOnceStrictly( test )
{

  test.case = 'repeated element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplaceOnceStrictly( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplaceOnceStrictly( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplaceOnceStrictly( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplaceOnceStrictly( dst, false, true );
  var expected = [ true, true, true, true, true ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'equalize';
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  };
  var got = _.arrayReplaceOnceStrictly( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( got, expected );
  test.true( got === dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly();
  });

  test.case = 'nothing';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly( [], 0, 0 );
  });

  test.case = 'element doesn´t exist';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly( [ 1, 2, 3 ], [ 1 ], [ 4 ] );
  });

  test.case = 'element two times in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly( [ 1, 2, 3, 1 ], [ 1 ], [ 4 ] );
  });

  test.case = 'element several times in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly( [ 0, 0, 0, 0, 0, 0 ], 0, 1 );
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly( [ 1, 2, undefined, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceOnceStrictly( 'wrong argument', undefined, 3 );
  });
}

//

function arrayReplaced( test )
{
  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplaced( dst, undefined, 0 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplaced( dst, 0, 0 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplaced( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, 1 );

  test.case = 'fourth element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplaced( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( dst, expected );
  test.identical( got, 1 );

  test.case = 'Several elements';
  var dst = [ true, true, true, true, false, false ];
  var got = _.arrayReplaced( dst, false, true );
  var expected = [ true, true, true, true, true, true ];
  test.identical( dst, expected );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayReplaced( dst, 1, 0 );
  test.identical( dst, [ 0, 0, 0 ] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 1 ];
  var got = _.arrayReplaced( dst, 1, 0 );
  test.identical( dst, [ 0, 2, 0 ] );
  test.identical( got, 2 );

  test.case = 'No match';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaced( dst, 4, 0 );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  function onEqualize( a, b )
  {
    return a.value === b;
  }

  var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
  var got = _.arrayReplaced( dst, 1, { value : 0 }, onEqualize );
  test.identical( dst, [ { value : 0 }, { value : 0 }, { value : 2 } ] );
  test.identical( got, 2 );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaced( );
  })

  test.case = 'first arg is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaced( 1, 1, 1 );
  })

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaced( 1, 1, 1, 1);
  })
}

//

function arrayReplacedOnce( test )
{

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplacedOnce( dst, 0, 0 );
  test.identical( dst, [] );
  test.identical( got, -1 );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplacedOnce( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, 1 );

  test.case = 'fourth element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplacedOnce( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( dst, expected );
  test.identical( got, 3 );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplacedOnce( dst, false, true );
  var expected = [ true, true, true, true, true ];
  test.identical( dst, expected );
  test.identical( got, 4 );

  test.case = 'first of several elements';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplacedOnce( dst, true, false );
  var expected = [ false, true, true, true, false ];
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'element not exists';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedOnce( dst, [ 1 ], [ 4 ] );
  var expected = [ 1, 2, 3 ];
  test.identical( dst, expected );
  test.identical( got, -1 );

  test.case = 'equalize';
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedOnce( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( dst, expected );
  test.identical( got, 0 );


  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnce();
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnce( [ 1, 2, undefined, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnce( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnce( 'wrong argument', undefined, 3 );
  });
}

//

function arrayReplacedOnceStrictly( test )
{

  test.case = 'first element';
  var dst = [ 1, 2, 3, 4, 5 ];
  var got = _.arrayReplacedOnceStrictly( dst, 1, 2 );
  var expected = [ 2, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplacedOnceStrictly( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, 1 );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplacedOnceStrictly( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( dst, expected );
  test.identical( got, 3 );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplacedOnceStrictly( dst, false, true );
  var expected = [ true, true, true, true, true ];
  test.identical( dst, expected );
  test.identical( got, 4 );

  test.case = 'equalize';
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedOnceStrictly( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( dst, expected );
  test.identical( got, 0 );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnceStrictly();
  });

  test.case = 'nothing';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnceStrictly( [], 0, 0 );
  });

  test.case = 'element several times in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnceStrictly( [ 1, 2, 3, 1, 2, 3 ], 1, 4 );
  });

  test.case = 'element doesn´t exist';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnceStrictly( [ 1, 2, 3 ], [ 1 ], [ 4 ] );
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnceStrictly( [ 1, 2, undefined, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnceStrictly( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedOnceStrictly( 'wrong argument', undefined, 3 );
  });
}

//

function arrayReplaceElement( test )
{
  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplaceElement( dst, undefined, 0 );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplaceElement( dst, 0, 0 );
  var expected = [];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplaceElement( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplaceElement( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'Several elements';
  var dst = [ true, true, true, true, false, false ];
  var got = _.arrayReplaceElement( dst, false, true );
  var expected = [ true, true, true, true, true, true ];
  test.identical( got, expected );
  test.true( got === dst );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayReplaceElement( dst, 1, 0 );
  test.identical( got, [ 0, 0, 0 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 1 ];
  var got = _.arrayReplaceElement( dst, 1, 0 );
  test.identical( got, [ 0, 2, 0 ] );
  test.true( got === dst );

  test.case = 'No match';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceElement( dst, 4, 0 );
  test.identical( got, [ 1, 2, 3 ] );

  function onEqualize( a, b )
  {
    return a.value === b;
  };

  var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
  var got = _.arrayReplaceElement( dst, 1, { value : 0 }, onEqualize );
  test.identical( got, [ { value : 0 }, { value : 0 }, { value : 2 } ] );
  test.true( got === dst );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElement( );
  })

  test.case = 'first arg is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElement( 1, 1, 1 );
  })

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElement( 1, 1, 1, 1);
  })
}

//

function arrayReplaceElement2( test )
{
  test.case = 'replace all ins with sub';

  var dst = [];
  var got = _.arrayReplaceElement( dst, undefined, 0 );
  test.identical( got, [] );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayReplaceElement( dst, 1, 0 );
  test.identical( got, [ 0, 0, 0 ] );

  var dst = [ 1, 2, 1 ];
  var got = _.arrayReplaceElement( dst, 1, 0 );
  test.identical( got, [ 0, 2, 0 ] );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceElement( dst, 4, 0 );
  test.identical( got, [ 1, 2, 3 ] );

  function onEqualize( a, b )
  {
    return a.value === b;
  }

  var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
  var got = _.arrayReplaceElement( dst, 1, { value : 0 }, onEqualize );
  test.identical( got, [ { value : 0 }, { value : 0 }, { value : 2 } ] );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElement( );
  });

  test.case = 'first arg is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElement( 1, 1, 1 );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElement( 1, 1, 1, 1);
  });
}

//

function arrayReplaceElementOnce( test )
{

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplaceElementOnce( dst, 0, 0 );
  var expected = [];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplaceElementOnce( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplaceElementOnce( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false, false ];
  var got = _.arrayReplaceElementOnce( dst, false, true );
  var expected = [ true, true, true, true, true, false ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'element not exists';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceElementOnce( dst, [ 1 ], [ 4 ] );
  var expected = [ 1, 2, 3 ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'replace just first match';
  var dst = [ 0, 0, 0, 0, 0, 0 ];
  var got = _.arrayReplaceElementOnce( dst, 0, 1 );
  var expected = [ 1, 0, 0, 0, 0, 0 ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'equalize';
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  };
  var got = _.arrayReplaceElementOnce( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( got, expected );
  test.true( got === dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnce();
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnce( [ 1, 2, undefined, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnce( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnce( 'wrong argument', undefined, 3 );
  });
}

//

function arrayReplaceElementOnceStrictly( test )
{

  test.case = 'second element';
  var dst = [ 1, 0, 3, 4, 5 ];
  var got = _.arrayReplaceElementOnceStrictly( dst, 0, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplaceElementOnceStrictly( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplaceElementOnceStrictly( dst, false, true );
  var expected = [ true, true, true, true, true ];
  test.identical( got, expected );
  test.true( got === dst );

  test.case = 'equalize';
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  };
  var got = _.arrayReplaceElementOnceStrictly( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( got, expected );
  test.true( got === dst );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly();
  });

  test.case = 'nothing';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly( [], 0, 0 );
  });

  test.case = 'element doesn´t exist';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly( [ 1, 2, 3 ], [ 1 ], [ 4 ] );
  });

  test.case = 'element two times in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly( [ 1, 2, 3, 1 ], [ 1 ], [ 4 ] );
  });

  test.case = 'element several times in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly( [ 0, 0, 0, 0, 0, 0 ], 0, 1 );
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly( [ 1, 2, undefined, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceElementOnceStrictly( 'wrong argument', undefined, 3 );
  });
}

//

function arrayReplacedElement( test )
{
  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplacedElement( dst, undefined, 0 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplacedElement( dst, 0, 0 );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplacedElement( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, 1 );

  test.case = 'fourth element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplacedElement( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( dst, expected );
  test.identical( got, 1 );

  test.case = 'Several elements';
  var dst = [ true, true, true, true, false, false ];
  var got = _.arrayReplacedElement( dst, false, true );
  var expected = [ true, true, true, true, true, true ];
  test.identical( dst, expected );
  test.identical( got, 2 );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayReplacedElement( dst, 1, 0 );
  test.identical( dst, [ 0, 0, 0 ] );
  test.identical( got, 3 );

  var dst = [ 1, 2, 1 ];
  var got = _.arrayReplacedElement( dst, 1, 0 );
  test.identical( dst, [ 0, 2, 0 ] );
  test.identical( got, 2 );

  test.case = 'No match';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedElement( dst, 4, 0 );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  function onEqualize( a, b )
  {
    return a.value === b;
  }

  var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
  var got = _.arrayReplacedElement( dst, 1, { value : 0 }, onEqualize );
  test.identical( dst, [ { value : 0 }, { value : 0 }, { value : 2 } ] );
  test.identical( got, 2 );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElement( );
  })

  test.case = 'first arg is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElement( 1, 1, 1 );
  })

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElement( 1, 1, 1, 1);
  })
}

//

function arrayReplacedElement2( test )
{
  test.case = 'replace all ins with sub';

  var dst = [];
  var got = _.arrayReplacedElement( dst, undefined, 0 );
  test.identical( got, 0 );
  test.identical( dst, [] );

  var dst = [ 1, 1, 1 ];
  var got = _.arrayReplacedElement( dst, 1, 0 );
  test.identical( got, 3 );
  test.identical( dst, [ 0, 0, 0 ] );

  var dst = [ 1, 2, 1 ];
  var got = _.arrayReplacedElement( dst, 1, 0 );
  test.identical( got, 2 );
  test.identical( dst, [ 0, 2, 0 ] );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedElement( dst, 4, 0 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 2, 3 ] );

  function onEqualize( a, b )
  {
    return a.value === b;
  }

  var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
  var got = _.arrayReplacedElement( dst, 1, { value : 0 }, onEqualize );
  test.identical( got, 2 );
  test.identical( dst, [ { value : 0 }, { value : 0 }, { value : 2 } ] );

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElement( );
  });

  test.case = 'first arg is not longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElement( 1, 1, 1 );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElement( 1, 1, 1, 1 );
  });

}

//

function arrayReplacedElementOnce( test )
{

  test.case = 'nothing';
  var dst = [];
  var got = _.arrayReplacedElementOnce( dst, 0, 0 );
  test.identical( dst, [] );
  test.identical( got, undefined );

  test.case = 'second element';
  var dst = [ 1, undefined, 3, 4, 5 ];
  var got = _.arrayReplacedElementOnce( dst, undefined, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, undefined );

  test.case = 'fourth element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplacedElementOnce( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( dst, expected );
  test.identical( got, 'Dmitry' );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplacedElementOnce( dst, false, true );
  var expected = [ true, true, true, true, true ];
  test.identical( dst, expected );
  test.identical( got, false );

  test.case = 'first of several elements';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplacedElementOnce( dst, true, false );
  var expected = [ false, true, true, true, false ];
  test.identical( dst, expected );
  test.identical( got, true );

  test.case = 'element not exists';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedElementOnce( dst, [ 1 ], [ 4 ] );
  var expected = [ 1, 2, 3 ];
  test.identical( dst, expected );
  test.identical( got, undefined );

  test.case = 'equalize';
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedElementOnce( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( dst, expected );
  test.identical( got, [ 1 ] );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnce();
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnce( [ 1, 2, undefined, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnce( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnce( 'wrong argument', undefined, 3 );
  });
}

//

function arrayReplacedElementOnceStrictly( test )
{

  test.case = 'first element';
  var dst = [ 1, 2, 3, 4, 5 ];
  var got = _.arrayReplacedElementOnceStrictly( dst, 1, 2 );
  var expected = [ 2, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, 1 );

  test.case = 'second element';
  var dst = [ 1, 0, 3, 4, 5 ];
  var got = _.arrayReplacedElementOnceStrictly( dst, 0, 2 );
  var expected = [ 1, 2, 3, 4, 5 ] ;
  test.identical( dst, expected );
  test.identical( got, 0 );

  test.case = 'third element';
  var dst = [ 'Petre', 'Mikle', 'Oleg', 'Dmitry' ];
  var got = _.arrayReplacedElementOnceStrictly( dst, 'Dmitry', 'Bob' );
  var expected = [ 'Petre', 'Mikle', 'Oleg', 'Bob' ];
  test.identical( dst, expected );
  test.identical( got, 'Dmitry' );

  test.case = 'fourth element';
  var dst = [ true, true, true, true, false ];
  var got = _.arrayReplacedElementOnceStrictly( dst, false, true );
  var expected = [ true, true, true, true, true ];
  test.identical( dst, expected );
  test.identical( got, false );

  test.case = 'equalize';
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedElementOnceStrictly( dst, [ 1 ], [ 4 ], onEqualize );
  var expected = [ [ 4 ], 2, 3 ];
  test.identical( dst, expected );
  test.identical( got, [ 1 ] );
  /*
    test.case = 'element several times in dstArray';
    test.shouldThrowErrorSync( function()
    {
      _.arrayReplacedElementOnceStrictly( [ 1, 2, 3, 4 ], 4, 1 );
    });
  */
  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly();
  });

  test.case = 'nothing';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly( [], 0, 0 );
  });

  test.case = 'element several times in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly( [ 1, 2, 3, 1, 2, 3 ], 1, 4 );
  });

  test.case = 'element doesn´t exist';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly( [ 1, 2, 3 ], [ 1 ], [ 4 ] );
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly( [ 1, 2, 3, 4, 5 ] );
  });

  test.case = 'fourth argument is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly( [ 1, 2, 0, 4, 5 ], 0, 3, 'argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly( 'wrong argument', 0, 3 );
  });

  test.case = 'second argument is undefined';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedElementOnceStrictly( [ 1, 2, undefined, 4, 5 ], undefined, 3, 'argument' );
  });
}

//

function arrayReplaceArray( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplaceArray( dst, [], [] );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArray( dst, [], [] );
  test.identical( got, [ 'a', 'b', 'c', 'd' ] );
  test.true( got === dst );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArray( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArray( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 3, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 1 ];
  var got = _.arrayReplaceArray( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 3, 2, 3, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4, 5, 4, 3, 2, 1 ];
  var got = _.arrayReplaceArray( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 3, 2, 3, 6, 5, 6, 3, 2, 3 ] );
  test.true( got === dst );

  test.case = 'ins has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArray( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'ins and dst have undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplaceArray( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 0, 3 ] );
  test.true( got === dst );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArray( dst, [ 1 ], [ undefined ] );
  test.identical( got, [ 2, 3 ] );
  test.true( got === dst );

  test.case = 'ins and sub have mirror elements';
  var dst = [ 0, 0, 0, 1, 1, 1, 0, 1 ];
  var got = _.arrayReplaceArray( dst, [ 1, 0 ], [ 0, 1 ] );
  test.identical( got, [ 1, 1, 1, 1, 1, 1, 1, 1 ] );
  test.true( got === dst );

  var dst = [ 'a', 'b', 'c', false, 'c', 'b', 'a', true, 2 ];
  var got = _.arrayReplaceArray( dst, [ 'a', 'b', 'c', false, true ], [ 'c', 'a', 'b', true, false ] );
  test.identical( got, [ 'b', 'a', 'b', false, 'b', 'a', 'b', false, 2 ] );
  test.true( got === dst );

  test.case = 'onEqualize'

  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize( a, b )
  {
    return a[ 0 ] === b[ 0 ];
  }
  var got = _.arrayReplaceArray( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize );
  test.identical( got, [ [ 0 ], [ 2 ], [ 3 ] ] );
  test.true( got === dst );

  /* */

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArray( dst, dst, [ 4, 5, 6 ] );
  test.identical( got, dst );
  test.identical( got, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArray( dst, dst, [ 4, 5, 6 ], ( e ) => e );
  test.identical( got, dst );
  test.identical( got, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArray( dst, dst, [ 4, 5, 6 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, dst );
  test.identical( got, [ 1, 2, 3 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 1 ];
  var got = _.arrayReplaceArray( dst, dst, [ 2, 1 ] );
  test.identical( got, [ 2, 2 ] );
  test.identical( got, dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArray();
  })

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArray( [ 1 ], [ 1 ], 1 );
  })

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArray( 1, [ 1 ], [ 1 ] );
  })

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArray( [ 1, 2 ], 1, [ 1 ] );
  })

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayReplaceArray( [ 1, 2 ], [ 1 ], [ 1 ], 1 );
  // });

  test.case = 'not equal length of ins and sub';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArray( [ 1, 2, 3 ], [ 1, 2 ], [ 3 ] );
  });

}

//

function arrayReplaceArrayOnce( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplaceArrayOnce( dst, [], [] );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArrayOnce( dst, [], [] );
  test.identical( got, [ 'a', 'b', 'c', 'd' ] );
  test.true( got === dst );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 3, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 1 ];
  var got = _.arrayReplaceArrayOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 3, 2, 3, 1 ] );
  test.true( got === dst );

  test.case = 'ins has undefined';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'ins and dst has undefined';

  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 0, 3 ] );
  test.true( got === dst );

  test.case = 'sub has undefined';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, [ 1 ], [ undefined ] );
  test.identical( got, [ 2, 3 ] );
  test.true( got === dst );

  test.case = 'onEqualize'

  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize( a, b )
  {
    return a[ 0 ] === b[ 0 ];
  }
  var got = _.arrayReplaceArrayOnce( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize );
  test.identical( got, [ [ 0 ], [ 2 ], [ 3 ] ] );
  test.true( got === dst );

  test.case = 'ins and sub have mirror elements';
  var dst = [ 0, 0, 0, 1, 1, 1, 0, 1 ];
  var got = _.arrayReplaceArrayOnce( dst, [ 1, 0 ], [ 0, 1 ] );
  test.identical( got, [ 1, 0, 0, 0, 1, 1, 0, 1 ] );
  test.true( got === dst );

  /* */

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === ins';
  var dst = [ 1, 1 ];
  var got = _.arrayReplaceArrayOnce( dst, dst, [ 2, 1 ] );
  test.identical( got, dst );
  test.identical( got, [ 2, 1 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnce();
  })

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnce( [ 1 ], [ 1 ], 1 );
  })

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnce( 1, [ 1 ], [ 1 ] );
  })

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnce( [ 1, 2 ], 1, [ 1 ] );
  })

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayReplaceArrayOnce( [ 1, 2 ], [ 1 ], [ 1 ], 1 );
  // });

  test.case = 'not equal length of ins and sub';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnce( [ 1, 2, 3 ], [ 1, 2 ], [ 3 ] );
  });

}

//

function arrayReplaceArrayOnceStrictly( test )
{

  test.case = 'trivial';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, [ 'a', 'b', 'c' ], [ 'x', 'y', undefined ] );
  test.identical( got, [ 'x', 'y', 'd' ] );
  test.true( got === dst );

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplaceArrayOnceStrictly( dst, [], [] );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, [], [] );
  test.identical( got, [ 'a', 'b', 'c', 'd' ] );
  test.true( got === dst );

  test.case = 'only sub is empty';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, [ 'a', 'b', 'c' ], [ undefined, undefined, undefined ] );
  test.identical( got, [ 'd' ] );
  test.true( got === dst );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, [ 1, 2 ], [ 3, undefined ] );
  test.identical( got, [ 3, 3 ] );
  test.true( got === dst );

  test.case = 'ins and dst has undefined';

  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 0, 3 ] );
  test.true( got === dst );

  test.case = 'onEqualize'

  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize( a, b )
  {
    return a[ 0 ] === b[ 0 ];
  }
  var got = _.arrayReplaceArrayOnceStrictly( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize );
  test.identical( got, [ [ 0 ], [ 2 ], [ 3 ] ] );
  test.true( got === dst );

  /* */

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, got );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, got );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, got );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, got );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayReplaceArrayOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got1 = _.arrayReplaceArrayOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 )
    test.identical( got1, 0 );
  }
  test.identical( dst, [ 1, 1, 2 ] );

  test.case = 'dst === ins';
  var dst = [ 1, 1 ];
  var got = _.arrayReplaceArrayOnceStrictly( dst, dst, [ 2, 1 ] );
  test.identical( got, dst );
  test.identical( got, [ 2, 1 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnceStrictly();
  })

  test.shouldThrowErrorSync( function()
  {
    var dst = [ 1, 2, 3 ];
    _.arrayReplaceArrayOnceStrictly( dst, [ undefined ], 0 );
  })

  test.case = 'only one replaced';
  test.shouldThrowErrorSync( function()
  {
    var dst = [ 1, 2, 3 ];
    _.arrayReplaceArrayOnceStrictly( dst, [ 1, 0, 4 ], 3 );
  })

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnceStrictly( [ 1 ], [ 1 ], 1 );
  })

  test.case = 'ins element several times in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnceStrictly( [ 1, 1 ], 1, 2 );
  })
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnceStrictly( [ 1, 2, 3, 4, 1, 2, 3 ], [ 1, 2, 3 ], [ 6, 7, 8 ] );
  })

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnceStrictly( 1, [ 1 ], [ 1 ] );
  })

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnceStrictly( [ 1, 2 ], 1, [ 1 ] );
  })

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrayOnceStrictly( [ 1, 2 ], [ 1 ], [ 1 ], 1 );
  })

  test.case = 'dst, ins are empty, sub is not, dst does not has ins';

  test.shouldThrowErrorSync( function()
  {
    var dst = [];
    _.arrayReplaceArrayOnceStrictly( dst, [ undefined ], [ 'x' ] );
  });

  test.case = 'dst does not has ins';

  test.shouldThrowErrorSync( function()
  {
    var dst = [ 'b', 'c' ];
    var got = _.arrayReplaceArrayOnceStrictly( dst, [ 'a' ], [ 'x' ] );
  });

  test.case = 'dst, sub are empty, ins is not';

  test.shouldThrowErrorSync( function()
  {
    var dst = [];
    var got = _.arrayReplaceArrayOnceStrictly( dst, [ 'a', 'b' ], [] );
  });

  test.case = 'only ins is empty';

  test.shouldThrowErrorSync( function()
  {
    var dst = [ 'a', 'b', 'c', 'd' ];
    var got = _.arrayReplaceArrayOnceStrictly( dst, [], [ 'x', 'y' ] );
  });

  test.case = 'not equal length of ins and sub';

  test.shouldThrowErrorSync( function()
  {
    var dst = [ 1, 2, 3 ];
    var got = _.arrayReplaceArrayOnceStrictly( dst, [ 1, 2 ], [ 3 ] );
  });

}

//

function arrayReplacedArray( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplacedArray( dst, [], [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplacedArray( dst, [], [] );
  test.identical( dst, [ 'a', 'b', 'c', 'd' ] );
  test.identical( got, 0 );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 3, 2, 3 ] );
  test.identical( got, 1 );

  test.case = 'Repeated elements in dstArray';

  var dst = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3, 2, 2, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3, 4, 3, 2, 1 ];
  var got = _.arrayReplacedArray( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3, 4, 3, 4, 3 ] );
  test.identical( got, 4 );

  var dst = [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ];
  var got = _.arrayReplacedArray( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 5, 5, 3, 3, 2, 2, 3, 3, 6, 6 ] );
  test.identical( got, 6 );


  test.case = 'ins has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'ins and dst has undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplacedArray( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 0, 3 ] );
  test.identical( got, 1 );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, [ 2 ], [ undefined ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  test.case = 'Element repeated in ins';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, [ 2, 2, 2 ], [ 'a', 'b', 'c' ] );
  test.identical( dst, [ 1, 'a', 3 ] );
  test.identical( got, 1 );

  test.case = 'ins and sub have mirror elements';
  var dst = [ 0, 0, 0, 1, 1, 1 ];
  var got = _.arrayReplacedArray( dst, [ 0, 1 ], [ 1, 0 ] );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 9 );

  var dst = [ 0, 0, 0, 1, 1, 1, 0, 1 ];
  var got = _.arrayReplacedArray( dst, [ 1, 0 ], [ 0, 1 ] );
  test.identical( dst, [ 1, 1, 1, 1, 1, 1, 1, 1 ] );
  test.identical( got, 12 );

  var dst = [ 'a', 'b', 'c', false, 'c', 'b', 'a', true, 2 ];
  var got = _.arrayReplacedArray( dst, [ 'a', 'b', 'c', false, true ], [ 'c', 'a', 'b', true, false ] );
  test.identical( dst, [ 'b', 'a', 'b', false, 'b', 'a', 'b', false, 2 ] );
  test.identical( got, 11 );

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, dst, [ 4, 5, 6 ] );
  test.identical( got, 3 );
  test.identical( dst, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, dst, [ 4, 5, 6 ], ( e ) => e );
  test.identical( got, 3 );
  test.identical( dst, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArray( dst, dst, [ 4, 5, 6 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArray( dst, dst, [ 2, 1 ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 2 ] );

  test.case = 'dst === ins === sub, equalize';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArray( dst, dst, dst );
  test.identical( got, 2 );
  test.identical( dst, [ 1, 1 ] );

  test.case = 'onEqualize'
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize( a, b )
  {
    return a[ 0 ] === b[ 0 ];
  }
  var got = _.arrayReplacedArray( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize );
  test.identical( dst, [ [ 0 ], [ 2 ], [ 3 ] ] );
  test.identical( got, 1 );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArray();
  })

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArray( [ 1 ], [ 1 ], 1 );
  })

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArray( 1, [ 1 ], [ 1 ] );
  })

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArray( [ 1, 2 ], 1, [ 1 ] );
  })

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayReplacedArray( [ 1, 2 ], [ 1 ], [ 1 ], 1 );
  // });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArray( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

}

//

function arrayReplacedArrayOnce( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplacedArrayOnce( dst, [], [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplacedArrayOnce( dst, [], [] );
  test.identical( dst, [ 'a', 'b', 'c', 'd' ] );
  test.identical( got, 0 );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 3, 2, 3 ] );
  test.identical( got, 1 );

  test.case = 'Repeated elements in dstArray';

  var dst = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3, 1, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3, 4, 3, 2, 1 ];
  var got = _.arrayReplacedArrayOnce( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3, 4, 3, 2, 1 ] );
  test.identical( got, 2 );

  var dst = [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ];
  var got = _.arrayReplacedArrayOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 5, 0, 3, 1, 2, 2, 3, 3, 6, 4 ] );
  test.identical( got, 3 );

  test.case = 'ins has undefined';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'ins and dst has undefined';

  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 0, 3 ] );
  test.identical( got, 1 );

  test.case = 'sub has undefined';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, [ 2 ], [ undefined ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  test.case = 'onEqualize'

  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize( a, b )
  {
    return a[ 0 ] === b[ 0 ];
  }
  var got = _.arrayReplacedArrayOnce( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize );
  test.identical( dst, [ [ 0 ], [ 2 ], [ 3 ] ] );
  test.identical( got, 1 );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === ins';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArrayOnce( dst, dst, [ 2, 1 ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 1 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnce();
  })

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnce( [ 1 ], [ 1 ], 1 );
  })

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnce( 1, [ 1 ], [ 1 ] );
  })

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnce( [ 1, 2 ], 1, [ 1 ] );
  })

  // test.case = 'onEqualize is not a routine';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.arrayReplacedArrayOnce( [ 1, 2 ], [ 1 ], [ 1 ], 1 );
  // });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnce( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

}

//

function arrayReplacedArrayOnceStrictly( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplacedArrayOnceStrictly( dst, [], [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, [], [] );
  test.identical( dst, [ 'a', 'b', 'c', 'd' ] );
  test.identical( got, 0 );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3 ] );
  test.identical( got, 2 );

  test.case = 'ins and dst have undefined';

  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 0, 3 ] );
  test.identical( got, 1 );

  test.case = 'sub has undefined';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, [ 2 ], [ undefined ] );
  test.identical( dst, [ 1, 3 ] );
  test.identical( got, 1 );

  test.case = 'onEqualize'

  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize( a, b )
  {
    return a[ 0 ] === b[ 0 ];
  }
  var got = _.arrayReplacedArrayOnceStrictly( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize );
  test.identical( dst, [ [ 0 ], [ 2 ], [ 3 ] ] );
  test.identical( got, 1 );

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayReplacedArrayOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got1 = _.arrayReplacedArrayOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 )
    test.identical( got1, 0 );
  }
  test.identical( dst, [ 1, 1, 2 ] );

  test.case = 'dst === inst';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArrayOnceStrictly( dst, dst, [ 2, 1 ] );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, 2 );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly();
  });

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 1 ], [ 1 ], 1 );
  });

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( 1, [ 1 ], [ 1 ] );
  });

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 1, 2 ], 1, [ 1 ] );
  });

  test.case = 'onEqualize is not a routine';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 1, 2 ], [ 1 ], [ 1 ], 1 );
  });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

  test.case = 'Repeated elements in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 1, 2, 3, 1, 2, 3 ], [ 1 ], [ 2 ] );
  });
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 1, 2, 3, 4, 3, 2, 1 ], [ 1, 2 ], [ 3, 4 ] );
  });
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ], [ 1, 0, 4 ], [ 3, 5, 6 ] );
  });

  test.case = 'Element not found in dstArray';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 1, 2, 3 ], [ 1, 0, 4 ], [ 3, 5, 6 ] );
  });
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrayOnceStrictly( [ 1, 2, 3, 4, 3, 2, 1 ], [ undefined ], [ 10 ] );
  });

}

//

function arrayReplaceArrays( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplaceArrays( dst, [], [] );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArrays( dst, [], [] );
  test.identical( got, [ 'a', 'b', 'c', 'd' ] );
  test.true( got === dst );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( got, [ 3, 4, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 3, 2, 3 ] );
  test.true( got === dst );

  test.case = 'Repeated elements in dstArray';

  var dst = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3, 2, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4, 3, 2, 1 ];
  var got = _.arrayReplaceArrays( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( got, [ 3, 4, 3, 4, 3, 4, 3 ] );
  test.true( got === dst );

  var dst = [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ];
  var got = _.arrayReplaceArrays( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 5, 5, 3, 3, 2, 2, 3, 3, 6, 6 ] );
  test.true( got === dst );

  test.case = 'ins has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'ins and dst has undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplaceArrays( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 0, 3 ] );
  test.true( got === dst );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, [ 2 ], [ undefined ] );
  test.identical( got, [ 1, undefined, 3 ] );
  test.true( got === dst );

  test.case = 'ins and sub Array of arrays';
  var dst = [ 0, 1, 2, 3, 4, 5, 0 ];
  var got = _.arrayReplaceArrays( dst, [ [ 0, 1 ], 2, [ 5 ] ], [ [ '0', '1' ], '2', [ '5' ] ] );
  test.identical( got, [ '0', '1', '2', 3, 4, '5', '0' ] );
  test.true( got === dst );

  var dst = [ 0, 'a', 'b', false, true, 'c', 0 ];
  var got = _.arrayReplaceArrays( dst, [ [ 0, false ] ], [ [ 1, true ] ] );
  test.identical( got, [ 1, 'a', 'b', true, true, 'c', 1 ] );
  test.true( got === dst );

  var dst = [ 0, 0, 0, 2, 1, 0, 0 ];
  var got = _.arrayReplaceArrays( dst, [ [ 0, 1 ], 0 ], [ [ 1, 0 ], '0' ] );
  test.identical( got, [ '0', '0', '0', 2, '0', '0', '0' ] );
  test.true( got === dst );

  test.case = 'ins and sub Array of arrays with mirror elements';
  var dst = [ 1, 1, 0, 0 ];
  var got = _.arrayReplaceArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ 0, 0, 0, 0 ] );
  test.true( got === dst );

  var dst = [ 'a', 'b', 'c' ];
  var got = _.arrayReplaceArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ 'a', 'a', 'c' ] );
  test.true( got === dst );

  var dst = [ true, false, true, false ];
  var got = _.arrayReplaceArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ true, true, true, true ] );
  test.true( got === dst );

  var dst = [ 0, 'a', true, 2, 'c', false, 'b', 1 ];
  var got = _.arrayReplaceArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ 0, 'a', true, 2, 'c', true, 'a', 0 ] );
  test.true( got === dst );

  test.case = 'onEqualize'

  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize1( a, b )
  {
    return a[ 0 ] === b;
  }
  var got = _.arrayReplaceArrays( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize1 );
  test.identical( got, [ 0, [ 2 ], [ 3 ] ] );
  test.true( got === dst );

  test.case = 'onEqualize'

  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var got = _.arrayReplaceArrays( dst, [ [ [ 1 ] ] ], [ [ [ 0 ] ] ], onEqualize );
  test.identical( got, [ [ 0 ], 2, 3 ] );
  test.true( got === dst );

  /* */

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, dst, [ 4, 5, 6 ] );
  test.identical( got, dst );
  test.identical( dst, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, dst, [ 4, 5, 6 ], ( e ) => e );
  test.identical( got, dst );
  test.identical( dst, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArrays( dst, dst, [ 4, 5, 6 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, dst );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 1 ];
  var got = _.arrayReplaceArrays( dst, dst, [ 2, 1 ] );
  test.identical( got, [ 2, 2 ] );
  test.identical( got, dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrays();
  });

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrays( [ 1 ], [ 1 ], 1 );
  });

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrays( 1, [ 1 ], [ 1 ] );
  });

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrays( [ 1, 2 ], 1, [ 1 ] );
  });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArrays( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

}

//

function arrayReplaceArraysOnce( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplaceArraysOnce( dst, [], [] );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArraysOnce( dst, [], [] );
  test.identical( got, [ 'a', 'b', 'c', 'd' ] );
  test.true( got === dst );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( got, [ 3, 4, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 3, 2, 3 ] );
  test.true( got === dst );

  test.case = 'Repeated elements in dstArray';

  var dst = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3, 1, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3, 4, 3, 2, 1 ];
  var got = _.arrayReplaceArraysOnce( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( got, [ 3, 4, 3, 4, 3, 2, 1 ] );
  test.true( got === dst );

  var dst = [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ];
  var got = _.arrayReplaceArraysOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( got, [ 5, 0, 3, 1, 2, 2, 3, 3, 6, 4 ] );
  test.true( got === dst );

  test.case = 'ins has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( got === dst );

  test.case = 'ins and dst has undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 0, 3 ] );
  test.true( got === dst );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, [ 2 ], [ undefined ] );
  test.identical( got, [ 1, undefined, 3 ] );
  test.true( got === dst );

  test.case = 'ins and sub Array of arrays';
  var dst = [ 0, 1, 2, 3, 4, 5, 0 ];
  var got = _.arrayReplaceArraysOnce( dst, [ [ 0, 1 ], 2, [ 5 ] ], [ [ '0', '1' ], '2', [ '5' ] ] );
  test.identical( got, [ '0', '1', '2', 3, 4, '5', 0 ] );
  test.true( got === dst );

  var dst = [ 0, 'a', 'b', false, true, 'c', 0 ];
  var got = _.arrayReplaceArraysOnce( dst, [ [ 0, false ] ], [ [ 1, true ] ] );
  test.identical( got, [ 1, 'a', 'b', true, true, 'c', 0 ] );
  test.true( got === dst );

  var dst = [ 0, 0, 0, 2, 1, 0, 0 ];
  var got = _.arrayReplaceArraysOnce( dst, [ [ 0, 1 ], 0 ], [ [ 1, 0 ], '0' ] );
  test.identical( got, [ '0', 0, 0, 2, 1, 0, 0 ] );
  test.true( got === dst );

  test.case = 'ins and sub Array of arrays with mirror elements';
  var dst = [ 1, 1, 0, 0 ];
  var got = _.arrayReplaceArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ 0, 1, 1, 0 ] );
  test.true( got === dst );

  var dst = [ 'a', 'b', 'c' ];
  var got = _.arrayReplaceArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ 'a', 'b', 'c' ] );
  test.true( got === dst );

  var dst = [ true, false, true, false ];
  var got = _.arrayReplaceArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ true, false, true, false ] );
  test.true( got === dst );

  var dst = [ 0, 'a', true, 2, 'c', false, 'b', 1 ];
  var got = _.arrayReplaceArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ 0, 'a', true, 2, 'c', false, 'b', 1 ] );
  test.true( got === dst );

  test.case = 'onEqualize'
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize1( a, b )
  {
    return a[ 0 ] === b;
  }
  var got = _.arrayReplaceArraysOnce( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize1 );
  test.identical( got, [ 0, [ 2 ], [ 3 ] ] );
  test.true( got === dst );

  test.case = 'onEqualize'
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var got = _.arrayReplaceArraysOnce( dst, [ [ [ 1 ] ] ], [ [ [ 0 ] ] ], onEqualize );
  test.identical( got, [ [ 0 ], 2, 3 ] );
  test.true( got === dst );

  /* */

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, dst );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src';
  var dst = [ 1, 1 ];
  var got = _.arrayReplaceArraysOnce( dst, dst, [ 2, 1 ] );
  test.identical( got, dst );
  test.identical( dst, [ 2, 1 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnce();
  });

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnce( [ 1 ], [ 1 ], 1 );
  });

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnce( 1, [ 1 ], [ 1 ] );
  });

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnce( [ 1, 2 ], 1, [ 1 ] );
  });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnce( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

}

arrayReplaceArraysOnce.timeOut = 10000;

//

function arrayReplaceArraysOnceStrictly( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [], [] );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [], [] );
  test.identical( got, [ 'a', 'b', 'c', 'd' ] );
  test.true( got === dst );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ 1 ], [ 2 ] );
  test.identical( got, [ 2, 2, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( got, [ 3, 4, 3 ] );
  test.true( got === dst );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ 1, 2, 3 ], [ 4, 5, 6 ] );
  test.identical( got, [ 4, 5, 6 ] );
  test.true( got === dst );

  test.case = 'ins has undefined';

  test.case = 'ins and dst has undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ undefined ], [ 0 ] );
  test.identical( got, [ 1, 0, 3 ] );
  test.true( got === dst );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ 2 ], [ undefined ] );
  test.identical( got, [ 1, undefined, 3 ] );
  test.true( got === dst );

  test.case = 'ins and sub Array of arrays';
  var dst = [ 0, 1, 2, 3, 4, 5, 0 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 0, 1 ], 2, [ 5 ] ], [ [ '0', '1' ], '2', [ '5' ] ] );
  test.identical( got, [ '0', '1', '2', 3, 4, '5', 0 ] );
  test.true( got === dst );

  var dst = [ 0, 'a', 'b', false, true, 'c', 0 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 0, false ] ], [ [ 1, true ] ] );
  test.identical( got, [ 1, 'a', 'b', true, true, 'c', 0 ] );
  test.true( got === dst );

  test.case = 'ins and sub Array of arrays with mirror elements';

  var dst = [ 1, 0 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 0, 1 ] ], [ [ 1, 0 ] ] );
  test.identical( got, [ 0, 1 ] );
  test.true( got === dst );

  var dst = [ 'a', 'b', 'c' ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 'a', 'b' ] ], [ [ 'b', 'a' ] ] );
  test.identical( got, [ 'a', 'b', 'c' ] );
  test.true( got === dst );

  var dst = [ true, false, true, false ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ true, false ] ], [ [ false, true ] ] );
  test.identical( got, [ true, false, true, false ] );
  test.true( got === dst );

  var dst = [ 0, 'a', true, 2, 'c', false, 'b', 1 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( got, [ 0, 'a', true, 2, 'c', false, 'b', 1 ] );
  test.true( got === dst );

  test.case = 'onEqualize'
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize1( a, b )
  {
    return a[ 0 ] === b;
  };
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize1 );
  test.identical( got, [ 0, [ 2 ], [ 3 ] ] );
  test.true( got === dst );

  test.case = 'onEqualize'
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  };
  var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ [ 1 ] ] ], [ [ [ 0 ] ] ], onEqualize );
  test.identical( got, [ [ 0 ], 2, 3 ] );
  test.true( got === dst );

  /* */

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, dst );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayReplaceArraysOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got1 = _.arrayReplaceArraysOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 )
    test.identical( got1, dst );
  }
  test.identical( dst, [ 1, 1, 2 ] );

  test.case = 'dst === ins';
  var dst = [ 1, 1 ];
  var got = _.arrayReplaceArraysOnceStrictly( dst, dst, [ 2, 1 ] );
  test.identical( dst, [ 2, 1 ] );
  test.identical( got, dst );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly();
  });

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 1 ], [ 1 ], 1 );
  });

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( 1, [ 1 ], [ 1 ] );
  });

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 1, 2 ], 1, [ 1 ] );
  });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

  test.case = 'ins element is not in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 1, 2, 3 ], [ 1, 0, 4 ], [ 3, 5, 6 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 1, 2, 3 ], [ undefined ], [ 0 ] );
  });

  test.case = 'Repeated elements in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 1, 2, 3, 1, 2, 3 ], [ 1 ], [ 2 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 1, 2, 3, 4, 3, 2, 1 ], [ 1, 2 ], [ 3, 4 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ], [ 1, 0, 4 ], [ 3, 5, 6 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplaceArraysOnceStrictly( [ 0, 0, 0, 2, 1, 0, 0 ], [ [ 0, 1 ], 0 ], [ [ 1, 0 ], '0' ] );
  });

}

//

function arrayReplacedArrays( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplacedArrays( dst, [], [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplacedArrays( dst, [], [] );
  test.identical( dst, [ 'a', 'b', 'c', 'd' ] );
  test.identical( got, 0 );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 3, 2, 3 ] );
  test.identical( got, 1 );

  test.case = 'Repeated elements in dstArray';

  var dst = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3, 2, 2, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3, 4, 3, 2, 1 ];
  var got = _.arrayReplacedArrays( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3, 4, 3, 4, 3 ] );
  test.identical( got, 4 );

  var dst = [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ];
  var got = _.arrayReplacedArrays( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 5, 5, 3, 3, 2, 2, 3, 3, 6, 6 ] );
  test.identical( got, 6 );


  test.case = 'ins has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'ins and dst has undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplacedArrays( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 0, 3 ] );
  test.identical( got, 1 );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, [ 2 ], [ undefined ] );
  test.identical( dst, [ 1, undefined, 3 ] );
  test.identical( got, 1 );

  test.case = 'ins and sub Array of arrays';
  var dst = [ 0, 1, 2, 3, 4, 5, 0 ];
  var got = _.arrayReplacedArrays( dst, [ [ 0, 1 ], 2, [ 5 ] ], [ [ '0', '1' ], '2', [ '5' ] ] );
  test.identical( dst, [ '0', '1', '2', 3, 4, '5', '0' ] );
  test.identical( got, 5 );

  var dst = [ 0, 'a', 'b', false, true, 'c', 0 ];
  var got = _.arrayReplacedArrays( dst, [ [ 0, false ] ], [ [ 1, true ] ] );
  test.identical( dst, [ 1, 'a', 'b', true, true, 'c', 1 ] );
  test.identical( got, 3 );

  var dst = [ 0, 0, 0, 2, 1, 0, 0 ];
  var got = _.arrayReplacedArrays( dst, [ [ 0, 1 ], 0 ], [ [ 1, 0 ], '0' ] );
  test.identical( dst, [ '0', '0', '0', 2, '0', '0', '0' ] );
  test.identical( got, 17 );

  test.case = 'ins and sub Array of arrays with mirror elements';
  var dst = [ 1, 1, 0, 0 ];
  var got = _.arrayReplacedArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  var dst = [ 'a', 'b', 'c' ];
  var got = _.arrayReplacedArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ 'a', 'a', 'c' ] );
  test.identical( got, 3 );

  var dst = [ true, false, true, false ];
  var got = _.arrayReplacedArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ true, true, true, true ] );
  test.identical( got, 6 );

  var dst = [ 0, 'a', true, 2, 'c', false, 'b', 1 ];
  var got = _.arrayReplacedArrays( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ 0, 'a', true, 2, 'c', true, 'a', 0 ] );
  test.identical( got, 9 );

  test.case = 'onEqualize'
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize1( a, b )
  {
    return a[ 0 ] === b;
  }
  var got = _.arrayReplacedArrays( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize1 );
  test.identical( dst, [ 0, [ 2 ], [ 3 ] ] );
  test.identical( got, 1 );

  test.case = 'onEqualize'
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var got = _.arrayReplacedArrays( dst, [ [ [ 1 ] ] ], [ [ [ 0 ] ] ], onEqualize );
  test.identical( dst, [ [ 0 ], 2, 3 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst === src';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, dst, [ 4, 5, 6 ] );
  test.identical( got, 3 );
  test.identical( dst, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, dst, [ 4, 5, 6 ], ( e ) => e );
  test.identical( got, 3 );
  test.identical( dst, [ 4, 5, 6 ] );

  test.case = 'dst === src, equalize';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArrays( dst, dst, [ 4, 5, 6 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 2, 3 ] );

  test.case = 'dst === src, loop check';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArrays( dst, dst, [ 2, 1 ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 2 ] );

  test.case = 'dst === src ';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArrays( dst, [ dst ], [ [ 2, 1 ] ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 2 ] );

  test.case = 'dst === src ';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArrays( dst, [ dst ], [ dst ] );
  test.identical( got, 2 );
  test.identical( dst, [ 1, 1 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrays();
  });

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrays( [ 1 ], [ 1 ], 1 );
  });

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrays( 1, [ 1 ], [ 1 ] );
  });

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrays( [ 1, 2 ], 1, [ 1 ] );
  });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArrays( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

}

//

function arrayReplacedArraysOnce( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplacedArraysOnce( dst, [], [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplacedArraysOnce( dst, [], [] );
  test.identical( dst, [ 'a', 'b', 'c', 'd' ] );
  test.identical( got, 0 );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 3, 2, 3 ] );
  test.identical( got, 1 );

  test.case = 'Repeated elements in dstArray';

  var dst = [ 1, 2, 3, 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3, 1, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3, 4, 3, 2, 1 ];
  var got = _.arrayReplacedArraysOnce( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3, 4, 3, 2, 1 ] );
  test.identical( got, 2 );

  var dst = [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ];
  var got = _.arrayReplacedArraysOnce( dst, [ 1, 0, 4 ], [ 3, 5, 6 ] );
  test.identical( dst, [ 5, 0, 3, 1, 2, 2, 3, 3, 6, 4 ] );
  test.identical( got, 3 );


  test.case = 'ins has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 2, 3 ] );
  test.identical( got, 0 );

  test.case = 'ins and dst has undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 0, 3 ] );
  test.identical( got, 1 );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, [ 2 ], [ undefined ] );
  test.identical( dst, [ 1, undefined, 3 ] );
  test.identical( got, 1 );

  test.case = 'ins and sub Array of arrays';
  var dst = [ 0, 1, 2, 3, 4, 5, 0 ];
  var got = _.arrayReplacedArraysOnce( dst, [ [ 0, 1 ], 2, [ 5 ] ], [ [ '0', '1' ], '2', [ '5' ] ] );
  test.identical( dst, [ '0', '1', '2', 3, 4, '5', 0 ] );
  test.identical( got, 4 );

  var dst = [ 0, 'a', 'b', false, true, 'c', 0 ];
  var got = _.arrayReplacedArraysOnce( dst, [ [ 0, false ] ], [ [ 1, true ] ] );
  test.identical( dst, [ 1, 'a', 'b', true, true, 'c', 0 ] );
  test.identical( got, 2 );

  var dst = [ 0, 0, 0, 2, 1, 0, 0 ];
  var got = _.arrayReplacedArraysOnce( dst, [ [ 0, 1 ], 0 ], [ [ 1, 0 ], '0' ] );
  test.identical( dst, [ '0', 0, 0, 2, 1, 0, 0 ] );
  test.identical( got, 3 );

  test.case = 'ins and sub Array of arrays with mirror elements';
  var dst = [ 1, 1, 0, 0 ];
  var got = _.arrayReplacedArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ 0, 1, 1, 0 ] );
  test.identical( got, 2 );

  var dst = [ 'a', 'b', 'c' ];
  var got = _.arrayReplacedArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ 'a', 'b', 'c' ] );
  test.identical( got, 2 );

  var dst = [ true, false, true, false ];
  var got = _.arrayReplacedArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ true, false, true, false ] );
  test.identical( got, 2 );

  var dst = [ 0, 'a', true, 2, 'c', false, 'b', 1 ];
  var got = _.arrayReplacedArraysOnce( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ 0, 'a', true, 2, 'c', false, 'b', 1 ] );
  test.identical( got, 6 );

  test.case = 'onEqualize'
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize1( a, b )
  {
    return a[ 0 ] === b;
  }
  var got = _.arrayReplacedArraysOnce( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize1 );
  test.identical( dst, [ 0, [ 2 ], [ 3 ] ] );
  test.identical( got, 1 );

  test.case = 'onEqualize'
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  }
  var got = _.arrayReplacedArraysOnce( dst, [ [ [ 1 ] ] ], [ [ [ 0 ] ] ], onEqualize );
  test.identical( dst, [ [ 0 ], 2, 3 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for forever lopp';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnce( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 );
  test.identical( got, 0 );
  test.identical( dst, [ 1, 1, 2, 2, 3, 3 ] );

  test.case = 'dst === src, loop check';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArraysOnce( dst, dst, [ 2, 1 ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 1 ] );

  test.case = 'dst === src ';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArraysOnce( dst, [ dst ], [ [ 2, 1 ] ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 1 ] );

  test.case = 'dst === src ';
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArraysOnce( dst, [ dst ], [ dst ] );
  test.identical( got, 2 );
  test.identical( dst, [ 1, 1 ] );


  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnce();
  });

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnce( [ 1 ], [ 1 ], 1 );
  });

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnce( 1, [ 1 ], [ 1 ] );
  });

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnce( [ 1, 2 ], 1, [ 1 ] );
  });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnce( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

}

//

function arrayReplacedArraysOnceStrictly( test )
{

  test.case = 'dst, ins, sub are empty';

  var dst = [];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [], [] );
  test.identical( dst, [] );
  test.identical( got, 0 );

  test.case = 'ins, sub are empty, dst is not';

  var dst = [ 'a', 'b', 'c', 'd' ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [], [] );
  test.identical( dst, [ 'a', 'b', 'c', 'd' ] );
  test.identical( got, 0 );

  test.case = 'other';

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ 1 ], [ 2 ] );
  test.identical( dst, [ 2, 2, 3 ] );
  test.identical( got, 1 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ 1, 2 ], [ 3, 4 ] );
  test.identical( dst, [ 3, 4, 3 ] );
  test.identical( got, 2 );

  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ 1, 2, 3 ], [ 4, 5, 6 ] );
  test.identical( dst, [ 4, 5, 6 ] );
  test.identical( got, 3 );

  test.case = 'ins has undefined';

  test.case = 'ins and dst has undefined';
  var dst = [ 1, undefined, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ undefined ], [ 0 ] );
  test.identical( dst, [ 1, 0, 3 ] );
  test.identical( got, 1 );

  test.case = 'sub has undefined';
  var dst = [ 1, 2, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ 2 ], [ undefined ] );
  test.identical( dst, [ 1, undefined, 3 ] );
  test.identical( got, 1 );

  test.case = 'ins and sub Array of arrays';
  var dst = [ 0, 1, 2, 3, 4, 5, 0 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ 0, 1 ], 2, [ 5 ] ], [ [ '0', '1' ], '2', [ '5' ] ] );
  test.identical( dst, [ '0', '1', '2', 3, 4, '5', 0 ] );
  test.identical( got, 4 );

  var dst = [ 0, 'a', 'b', false, true, 'c', 0 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ 0, false ] ], [ [ 1, true ] ] );
  test.identical( dst, [ 1, 'a', 'b', true, true, 'c', 0 ] );
  test.identical( got, 2 );

  test.case = 'ins and sub Array of arrays with mirror elements';

  var dst = [ 1, 0 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ 0, 1 ] ], [ [ 1, 0 ] ] );
  test.identical( dst, [ 0, 1 ] );
  test.identical( got, 2 );

  var dst = [ 'a', 'b', 'c' ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ 'a', 'b' ] ], [ [ 'b', 'a' ] ] );
  test.identical( dst, [ 'a', 'b', 'c' ] );
  test.identical( got, 2 );

  var dst = [ true, false, true, false ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ true, false ] ], [ [ false, true ] ] );
  test.identical( dst, [ true, false, true, false ] );
  test.identical( got, 2 );

  var dst = [ 0, 'a', true, 2, 'c', false, 'b', 1 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ 0, 1 ], [ 'a', 'b' ], [ true, false ] ], [ [ 1, 0 ], [ 'b', 'a' ], [ false, true ] ] );
  test.identical( dst, [ 0, 'a', true, 2, 'c', false, 'b', 1 ] );
  test.identical( got, 6 );

  test.case = 'onEqualize'
  var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
  function onEqualize1( a, b )
  {
    return a[ 0 ] === b;
  };
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ 1 ] ], [ [ 0 ] ], onEqualize1 );
  test.identical( dst, [ 0, [ 2 ], [ 3 ] ] );
  test.identical( got, 1 );

  test.case = 'onEqualize'
  var dst = [ 1, 2, 3 ];
  function onEqualize( a, b )
  {
    return a === b[ 0 ];
  };
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ [ [ 1 ] ] ], [ [ [ 0 ] ] ], onEqualize );
  test.identical( dst, [ [ 0 ], 2, 3 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'dst === src'
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src single evaluator';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e + 10 );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators';
  var dst = [ 1, 1, 2, 2, 3, 3 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, dst, [ 0, 0, 0, 0, 0, 0 ], ( e ) => e, ( e ) => e );
  test.identical( dst, [ 0, 0, 0, 0, 0, 0 ] );
  test.identical( got, 6 );

  test.case = 'dst === src with evaluators, check for infinity loop';
  var dst = [ 1, 1, 2 ];
  if( Config.debug )
  test.shouldThrowErrorSync( () => _.arrayReplacedArraysOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 ) );
  else
  {
    var got1 = _.arrayReplacedArraysOnceStrictly( dst, dst, [ 0, 0, 0 ], ( e ) => e, ( e ) => e + 10 )
    test.identical( got1, 0 );
  }
  test.identical( dst, [ 1, 1, 2 ] );

  test.case = 'dst === ins'
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, dst, [ 2, 1 ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 1 ] );

  test.case = 'dst === ins'
  var dst = [ 1, 1 ];
  var got = _.arrayReplacedArraysOnceStrictly( dst, [ dst ], [ [ 2, 1 ] ] );
  test.identical( got, 2 );
  test.identical( dst, [ 2, 1 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly();
  });

  test.case = 'sub is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 1 ], [ 1 ], 1 );
  });

  test.case = 'dstArray is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( 1, [ 1 ], [ 1 ] );
  });

  test.case = 'ins is not a longIs';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 1, 2 ], 1, [ 1 ] );
  });

  test.case = 'ins and sub don´t have the same length ';
  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 'a', 'b', 'c', 'd' ], [ 'a', 'b', 'c' ], [ 'x', 'y' ] );
  });

  test.case = 'ins element is not in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 1, 2, 3 ], [ 1, 0, 4 ], [ 3, 5, 6 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 1, 2, 3 ], [ undefined ], [ 0 ] );
  });

  test.case = 'Repeated elements in dstArray';

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 1, 2, 3, 1, 2, 3 ], [ 1 ], [ 2 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 1, 2, 3, 4, 3, 2, 1 ], [ 1, 2 ], [ 3, 4 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 0, 0, 1, 1, 2, 2, 3, 3, 4, 4 ], [ 1, 0, 4 ], [ 3, 5, 6 ] );
  });

  test.shouldThrowErrorSync( function()
  {
    _.arrayReplacedArraysOnceStrictly( [ 0, 0, 0, 2, 1, 0, 0 ], [ [ 0, 1 ], 0 ], [ [ 1, 0 ], '0' ] );
  });

}

// //
//
// function arrayReplaceArraysOnce( test )
// {
//   test.case = 'replace elements from arrays from ins with relevant values from sub';
//
//   var dst = [];
//   var got = _.arrayReplaceArraysOnce( dst, [ [] ], [ [] ] );
//   test.identical( got, [] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1 ] ], [ [ 3 ] ] );
//   test.identical( got, [ 3, 2, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], [ 3 ] ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], 3 ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ 3, 3 ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1, 2, 3 ] ], [ 3 ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1, 2, 3 ] ], [ [ 3, 3, 3 ] ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1, 2, 3 ] ], [ [ 3 ] ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], [ 4 ] ] );
//   test.identical( got, [ 3, 4, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ [ 1 ] ] ], [ 0 ] );
//   test.identical( got, [ 1, 2, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnce( dst, [ [ 1, [ 2 ], 3 ] ], [ 0 ] );
//   test.identical( got, [ 0, 2, 0 ] );
//
//   var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
//   function onEqualize( a, b )
//   {
//     return a[ 0 ] === b[ 0 ];
//   }
//   var got = _.arrayReplaceArraysOnce( dst, [ [ [ 1 ], [ 2 ], [ 3 ] ] ], [ [ [ 0 ] ] ], onEqualize );
//   test.identical( got, [ [ 0 ], [ 0 ], [ 0 ] ] );
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce();
//   })
//
//   test.case = 'dstArray is not a longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce( 1, [ [ 1 ] ], [ 1 ] );
//   })
//
//   test.case = 'ins is not a longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce( [ 1, 2 ], 1, [ 1 ] );
//   })
//
//   test.case = 'ins must be array of arrays';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce( [ 1, 2 ], [ 1 ], [ 1 ] );
//   })
//
//   test.case = 'onEqualize is not a routine';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce( [ 1, 2 ], [ [ 1 ] ], [ 1 ], 1 );
//   })
//
//   test.case = 'ins and sub length are different';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce( [ 1 ], [ [ 1 ] ], [ 10, 20 ] );
//   })
//
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce( [ 1 ], [ [ 1, 2 ] ], [ 10, 20 ] );
//   })
//
//   test.case = 'ins[ 0 ] and sub[ 0 ] length are different';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnce( [ 1 ], [ [ 1 ] ], [ [ 10, 20 ] ] );
//   })
// }
//
// //
//
// function arrayReplaceArraysOnceStrictly( test )
// {
//   test.case = 'replace elements from arrays from ins with relevant values from sub';
//
//   var dst = [];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [] ], [ [] ] );
//   test.identical( got, [] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1 ] ], [ [ 3 ] ] );
//   test.identical( got, [ 3, 2, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], [ 3 ] ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], 3 ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1 ], [ 2 ] ], [ 3, 3 ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1, 2, 3 ] ], [ 3 ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1, 2, 3 ] ], [ [ 3, 3, 3 ] ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1, 2, 3 ] ], [ [ 3 ] ] );
//   test.identical( got, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], [ 4 ] ] );
//   test.identical( got, [ 3, 4, 3 ] );
//
//   var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
//   function onEqualize( a, b )
//   {
//     return a[ 0 ] === b[ 0 ];
//   }
//   var got = _.arrayReplaceArraysOnceStrictly( dst, [ [ [ 1 ], [ 2 ], [ 3 ] ] ], [ [ [ 0 ] ] ], onEqualize );
//   test.identical( got, [ [ 0 ], [ 0 ], [ 0 ] ] );
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly();
//   })
//
//   test.shouldThrowErrorSync( function()
//   {
//     var dst = [ 1, 2, 3 ];
//     _.arrayReplaceArraysOnceStrictly( dst, [ [ [ 1 ] ] ], [ 0 ] );
//   })
//
//   test.case = 'one element is not replaced';
//   test.shouldThrowErrorSync( function()
//   {
//     var dst = [ 1, 2, 3 ];
//     _.arrayReplaceArraysOnceStrictly( dst, [ [ 1, [ 2 ], 3 ] ], [ 0 ] );
//   })
//
//   test.case = 'dstArray is not a longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly( 1, [ [ 1 ] ], [ 1 ] );
//   })
//
//   test.case = 'ins is not a longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly( [ 1, 2 ], 1, [ 1 ] );
//   })
//
//   test.case = 'ins must be array of arrays';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly( [ 1, 2 ], [ 1 ], [ 1 ] );
//   })
//
//   test.case = 'onEqualize is not a routine';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly( [ 1, 2 ], [ [ 1 ] ], [ 1 ], 1 );
//   })
//
//   test.case = 'ins and sub length are different';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly( [ 1 ], [ [ 1 ] ], [ 10, 20 ] );
//   })
//
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly( [ 1 ], [ [ 1, 2 ] ], [ 10, 20 ] );
//   })
//
//   test.case = 'ins[ 0 ] and sub[ 0 ] length are different';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceArraysOnceStrictly( [ 1 ], [ [ 1 ] ], [ [ 10, 20 ] ] );
//   })
// }
//
// //
//
// function arrayReplacedArraysOnce( test )
// {
//   test.case = 'replace elements from arrays from ins with relevant values from sub';
//
//   var dst = [];
//   var got = _.arrayReplacedArraysOnce( dst, [ [] ], [ [] ] );
//   test.identical( got, 0 );
//   test.identical( dst, [] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1 ] ], [ [ 3 ] ] );
//   test.identical( got, 1 );
//   test.identical( dst, [ 3, 2, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], [ 3 ] ] );
//   test.identical( got, 2 );
//   test.identical( dst, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], 3 ] );
//   test.identical( got, 2 );
//   test.identical( dst, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ 3, 3 ] );
//   test.identical( got, 2 );
//   test.identical( dst, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1, 2, 3 ] ], [ 3 ] );
//   test.identical( got, 3 );
//   test.identical( dst, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1, 2, 3 ] ], [ [ 3, 3, 3 ] ] );
//   test.identical( got, 3 );
//   test.identical( dst, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1, 2, 3 ] ], [ [ 3 ] ] );
//   test.identical( got, 3 );
//   test.identical( dst, [ 3, 3, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1 ], [ 2 ] ], [ [ 3 ], [ 4 ] ] );
//   test.identical( got, 2 );
//   test.identical( dst, [ 3, 4, 3 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ [ 1 ] ] ], [ 0 ] );
//   test.identical( dst, [ 1, 2, 3 ] );
//   test.identical( got, 0 );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedArraysOnce( dst, [ [ 1, [ 2 ], 3 ] ], [ 0 ] );
//   test.identical( dst, [ 0, 2, 0 ] );
//   test.identical( got, 2 );
//
//   var dst = [ [ 1 ], [ 2 ], [ 3 ] ];
//   function onEqualize( a, b )
//   {
//     return a[ 0 ] === b[ 0 ];
//   }
//   var got = _.arrayReplacedArraysOnce( dst, [ [ [ 1 ], [ 2 ], [ 3 ] ] ], [ [ [ 0 ] ] ], onEqualize );
//   test.identical( dst, [ [ 0 ], [ 0 ], [ 0 ] ] );
//   test.identical( got, 3 );
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce();
//   })
//
//   test.case = 'dstArray is not a longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce( 1, [ [ 1 ] ], [ 1 ] );
//   })
//
//   test.case = 'ins is not a longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce( [ 1, 2 ], 1, [ 1 ] );
//   })
//
//   test.case = 'ins must be array of arrays';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce( [ 1, 2 ], [ 1 ], [ 1 ] );
//   })
//
//   test.case = 'onEqualize is not a routine';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce( [ 1, 2 ], [ [ 1 ] ], [ 1 ], 1 );
//   })
//
//   test.case = 'ins and sub length are different';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce( [ 1 ], [ [ 1 ] ], [ 10, 20 ] );
//   })
//
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce( [ 1 ], [ [ 1, 2 ] ], [ 10, 20 ] );
//   })
//
//   test.case = 'ins[ 0 ] and sub[ 0 ] length are different';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedArraysOnce( [ 1 ], [ [ 1 ] ], [ [ 10, 20 ] ] );
//   })
// }
//
//
//
// function arrayReplaceAll( test )
// {
//   test.case = 'replace all ins with sub';
//
//   var dst = [];
//   var got = _.arrayReplaceAll( dst, undefined, 0 );
//   test.identical( got, [] );
//
//   var dst = [ 1, 1, 1 ];
//   var got = _.arrayReplaceAll( dst, 1, 0 );
//   test.identical( got, [ 0, 0, 0 ] );
//
//   var dst = [ 1, 2, 1 ];
//   var got = _.arrayReplaceAll( dst, 1, 0 );
//   test.identical( got, [ 0, 2, 0 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplaceAll( dst, 4, 0 );
//   test.identical( got, [ 1, 2, 3 ] );
//
//   function onEqualize( a, b )
//   {
//     return a.value === b;
//   }
//
//   var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
//   var got = _.arrayReplaceAll( dst, 1, { value : 0 }, onEqualize );
//   test.identical( got, [ { value : 0 }, { value : 0 }, { value : 2 } ] );
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no args';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceAll( );
//   });
//
//   test.case = 'first arg is not longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceAll( 1, 1, 1 );
//   });
//
//   test.case = 'fourth argument is not a routine';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplaceAll( 1, 1, 1, 1);
//   });
// }
//
// //
//
// function arrayReplacedAll( test )
// {
//   test.case = 'replace all ins with sub';
//
//   var dst = [];
//   var got = _.arrayReplacedAll( dst, undefined, 0 );
//   test.identical( got, 0 );
//   test.identical( dst, [] );
//
//   var dst = [ 1, 1, 1 ];
//   var got = _.arrayReplacedAll( dst, 1, 0 );
//   test.identical( got, 3 );
//   test.identical( dst, [ 0, 0, 0 ] );
//
//   var dst = [ 1, 2, 1 ];
//   var got = _.arrayReplacedAll( dst, 1, 0 );
//   test.identical( got, 2 );
//   test.identical( dst, [ 0, 2, 0 ] );
//
//   var dst = [ 1, 2, 3 ];
//   var got = _.arrayReplacedAll( dst, 4, 0 );
//   test.identical( got, 0 );
//   test.identical( dst, [ 1, 2, 3 ] );
//
//   function onEqualize( a, b )
//   {
//     return a.value === b;
//   }
//
//   var dst = [ { value : 1 }, { value : 1 }, { value : 2 } ];
//   var got = _.arrayReplacedAll( dst, 1, { value : 0 }, onEqualize );
//   test.identical( got, 2 );
//   test.identical( dst, [ { value : 0 }, { value : 0 }, { value : 2 } ] );
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no args';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedAll( );
//   });
//
//   test.case = 'first arg is not longIs';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedAll( 1, 1, 1 );
//   });
//
//   test.case = 'fourth argument is not a routine';
//   test.shouldThrowErrorSync( function()
//   {
//     _.arrayReplacedAll( 1, 1, 1, 1 );
//   });
//
// }

//

function arrayUpdate( test )
{

  test.case = 'add a new element';
  var got = _.arrayUpdate( [], 1, 1 );
  var expected = 0;
  test.identical( got, expected );

  test.case = 'add a new element';
  var got = _.arrayUpdate( [ 1, 2, 3, 4, 5 ], 6, 6 );
  var expected = 5;
  test.identical( got, expected );

  test.case = 'add a new element';
  var got = _.arrayUpdate( [ 'Petre', 'Mikle', 'Oleg' ], 'Dmitry', 'Dmitry' );
  var expected = 3;
  test.identical( got, expected );

  test.case = 'change the fourth element';
  var got = _.arrayUpdate( [ true, true, true, true, false ], false, true );
  var expected = 4;
  test.identical( got, expected );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayUpdate();
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.arrayUpdate( [ 1, 2, 3, 4, 5 ] );
  });

  test.case = 'extra argument';
  test.shouldThrowErrorSync( function()
  {
    _.arrayUpdate( [ 1, 2, 3, 4, 5 ], 6, 6, 'redundant argument' );
  });

  test.case = 'arguments[ 0 ] is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.arrayUpdate( 'wrong argument', 6, 6 );
  });

}

// --
//
// --

const Proto =
{

  name : 'Tools.Array.l0.l9',
  silencing : 1,
  enabled : 1,

  tests :
  {

    // type test

    constructorLikeArray,

    // producer

    // arrayMake,
    // arrayMakeNotDefaultDescriptor, /* Dmytro : long long does not exists */
    // arrayMakeUndefined,
    // arrayMakeUndefinedNotDefaultDescriptor, /* Dmytro : long long does not exists */
    // arrayFrom,
    // arrayFromLongDescriptor, /* Dmytro : long long does not exists */
    arrayFromCoercing,
    // arrayFromCoercingLongDescriptor, /* Dmytro : long long does not exists */

    // arrayAs,

    // array transformer

    arraySlice,
    arrayEmpty,

    arrayBut,
    arrayButInplace,
    arrayBut_,
    arrayBut_CheckReturnedContainer,
    arrayShrink,
    arrayShrinkInplace,
    arrayShrink_,
    arrayShrink_CheckReturnedContainer,
    arrayGrow,
    arrayGrowInplace,
    arrayGrow_,
    arrayGrow_CheckReturnedContainer,
    arrayRelength,
    arrayRelengthInplace,
    arrayRelength_,
    arrayRelength_CheckReturnedContainer,

    // array prepend

    arrayPrepend,
    arrayPrependOnce,
    arrayPrependOnceStrictly,
    arrayPrepended,
    arrayPrependedOnce,
    arrayPrependedOnceStrictly,

    arrayPrependElement,
    arrayPrependElementOnce,
    arrayPrependElementOnceStrictly,
    arrayPrependedElement,
    arrayPrependedElementOnce,
    arrayPrependedElementOnceStrictly,

    arrayPrependArray,
    arrayPrependArrayOnce,
    arrayPrependArrayOnceStrictly,
    arrayPrependedArray,
    arrayPrependedArrayOnce,
    arrayPrependedArrayOnceStrictly,

    arrayPrependArrays,
    arrayPrependArraysOnce,
    arrayPrependArraysOnceStrictly,
    arrayPrependedArrays,
    arrayPrependedArraysOnce,
    arrayPrependedArraysOnceStrictly,

    // array append

    arrayAppend,
    arrayAppendOnce,
    arrayAppendOnceStrictly,
    arrayAppended,
    arrayAppendedOnce,
    arrayAppendedOnceStrictly,

    arrayAppendElement,
    arrayAppendElementOnce,
    arrayAppendElementOnceStrictly,
    arrayAppendedElement,
    arrayAppendedElementOnce,
    arrayAppendedElementOnceStrictly,

    arrayAppendArray,
    arrayAppendArrayOnce,
    arrayAppendArrayOnceStrictly,
    arrayAppendedArray,
    arrayAppendedArrayOnce,
    arrayAppendedArrayOnceWithSelector,
    arrayAppendedArrayOnceStrictly,

    arrayAppendArrays,
    arrayAppendArraysOnce,
    arrayAppendArraysOnceStrictly,
    arrayAppendedArrays,
    arrayAppendedArraysOnce,
    arrayAppendedArraysOnceStrictly,

    // array remove

    arrayRemove,
    arrayRemoveOnce,
    arrayRemoveOnceStrictly,
    arrayRemoved,
    arrayRemovedOnce,
    arrayRemovedOnceStrictly,

    arrayRemoveElement,
    arrayRemoveElementOnce,
    arrayRemoveElementOnceStrictly,
    arrayRemovedElement,
    arrayRemovedElement_,
    arrayRemovedElementOnce,
    arrayRemovedElementOnce_,
    arrayRemovedElementOnceStrictly,
    arrayRemovedElementOnceStrictly_,

    // arrayRemovedOnceStrictly,
    // arrayRemovedElementOnce2,
    // arrayRemovedOnceElementStrictly,

    arrayRemoveArray,
    arrayRemoveArrayOnce,
    arrayRemoveArrayOnceStrictly,
    arrayRemovedArray,
    arrayRemovedArrayOnce,
    arrayRemovedArrayOnceStrictly,

    arrayRemoveArrays,
    arrayRemoveArraysOnce,
    arrayRemoveArraysOnceStrictly,
    arrayRemovedArrays,
    arrayRemovedArraysOnce,
    arrayRemovedArraysOnceStrictly,

    arrayRemoveDuplicates,

    // array flatten

    /* aaa : extend test coverage */
    /* Dmytro : extended */
    /* aaa : extend implementation for sets */
    /* Dmytro : extended */

    arrayFlatten,
    arrayFlattenSame,
    arrayFlattenSets,
    arrayFlattenOnce,
    arrayFlattenOnceSame,
    arrayFlattenOnceSets,
    arrayFlattenOnceStrictly,
    arrayFlattenOnceStrictlySame,
    arrayFlattenOnceStrictlySets,
    arrayFlattened,
    arrayFlattenedSame,
    arrayFlattenedSets,
    arrayFlattenedOnce,
    arrayFlattenedOnceSame,
    arrayFlattenedOnceSets,
    arrayFlattenedOnceStrictly,
    arrayFlattenedOnceStrictlySame,
    arrayFlattenedOnceStrictlySets,

    arrayFlattenDefined,
    arrayFlattenDefinedSame,
    arrayFlattenDefinedSets,
    arrayFlattenDefinedOnce,
    arrayFlattenDefinedOnceSame,
    arrayFlattenDefinedOnceSets,
    arrayFlattenDefinedOnceStrictly,
    arrayFlattenDefinedOnceStrictlySame,
    arrayFlattenDefinedOnceStrictlySets,
    arrayFlattenedDefined,
    arrayFlattenedDefinedSame,
    arrayFlattenedDefinedSets,
    arrayFlattenedDefinedOnce,
    arrayFlattenedDefinedOnceSame,
    arrayFlattenedDefinedOnceSets,
    arrayFlattenedDefinedOnceStrictly,
    arrayFlattenedDefinedOnceStrictlySame,
    arrayFlattenedDefinedOnceStrictlySets,

    // array replace

    arrayReplace,
    arrayReplaceOnce,
    arrayReplaceOnceStrictly,
    arrayReplaced,
    arrayReplacedOnce,
    arrayReplacedOnceStrictly,

    arrayReplaceElement,
    arrayReplaceElement2,
    arrayReplaceElementOnce,
    arrayReplaceElementOnceStrictly,
    arrayReplacedElement,
    arrayReplacedElement2,
    arrayReplacedElementOnce,
    arrayReplacedElementOnceStrictly,

    arrayReplaceArray,
    arrayReplaceArrayOnce,
    arrayReplaceArrayOnceStrictly,
    arrayReplacedArray,
    arrayReplacedArrayOnce,
    arrayReplacedArrayOnceStrictly,

    arrayReplaceArrays,
    arrayReplaceArraysOnce,
    arrayReplaceArraysOnceStrictly,
    arrayReplacedArrays,
    arrayReplacedArraysOnce,
    arrayReplacedArraysOnceStrictly,

    // arrayReplaceAll,
    // arrayReplacedAll,

    arrayUpdate,

  }

}

/* qqq for Dmytro : this test suite fails with shoulding:0. check also other test suites */ /* aaa : checked and fixed */

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
