( function _l0_l1_Map_test_s()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( '../Include1.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

//--
// dichotomy
//--

function dichotomy( test ) /* qqq : extend with construbile* checks */
{

  test.case = 'pure empty map';
  var src = Object.create( null );
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), true );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), true );
  // test.identical( _.aux.isEmpty( src ), true );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), true );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), true );
  // test.identical( _.aux.isPopulated( src ), true );

  test.case = 'pure map with constructor';
  var src = Object.create( null );
  src.constructor = function(){};
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), true );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), true );
  // test.identical( _.aux.isPopulated( src ), true );

  test.case = 'from pure with iterator';
  var src = Object.create( null );
  src[ Symbol.iterator ] = function(){};
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), true );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'empty polluted map';
  var src = {};
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), true );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), true );
  // test.identical( _.aux.isEmpty( src ), true );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'polluted map';
  var src = { a : 7, b : 13 };
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), true );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), true );
  // test.identical( _.aux.isPopulated( src ), true );

  test.case = 'polluted map with constructor';
  var src = {};
  src.constructor = function(){};
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), true );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), true );
  // test.identical( _.aux.isPopulated( src ), true );

  test.case = 'from polluted with iterator';
  var src = {};
  src[ Symbol.iterator ] = function(){};
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), true );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'new object';
  var src = new Object();
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), true );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), true );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), true );
  // test.identical( _.aux.isEmpty( src ), true );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'empty array';
  var src = [];
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'empty arguments array';
  var src = _.argumentsArray.make();
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'undefined';
  var src = undefined;
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'null';
  var src = null;
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'string';
  var src = 'str';
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'number';
  var src = 13;
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'symbol';
  var src = Symbol.for( 'a' );
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'boolean false';
  var src = true;
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'boolean true';
  var src = true;
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'routine';
  var src = function() {};
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'prototyped from pure map';
  var prototype = Object.create( null );
  var src = Object.create( prototype );
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), true );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), true );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'prototyped from pure map deep';
  var prototype1 = Object.create( null );
  var prototype2 = Object.create( prototype1 );
  var src = Object.create( prototype1 );
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), true );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), true );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'prototyped from pure map deep with props';
  var prototype1 = Object.create( null );
  prototype1.a = 1;
  var prototype2 = Object.create( prototype1 );
  prototype2.b = 1;
  var src = Object.create( prototype1 );
  src.c = 1;
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), true );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), true );

  test.case = 'prototyped from polluted map';
  var prototype = {};
  var src = Object.create( prototype );
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), true );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), true );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'prototyped from polluted map with constructor';
  var prototype = {};
  prototype.constructor = function(){};
  var src = Object.create( prototype );
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'prototyped from polluted map deep';
  var prototype1 = {};
  var prototype2 = Object.create( prototype1 );
  var src = Object.create( prototype1 );
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), true );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), true );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), false );

  test.case = 'prototyped from polluted map deep with props';
  var prototype1 = {};
  prototype1.a = 1;
  var prototype2 = Object.create( prototype1 );
  prototype2.b = 1;
  var src = Object.create( prototype1 );
  src.c = 1;
  test.identical( _.object.isBasic( src ), true );
  test.identical( _.object.like( src ), true );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), true );
  test.identical( _.aux.isPrototyped( src ), true );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), true );
  // test.identical( _.map.isEmpty( src ), false );
  // test.identical( _.aux.isEmpty( src ), false );
  // test.identical( _.map.isPopulated( src ), false );
  // test.identical( _.aux.isPopulated( src ), true );
}

dichotomy.timeOut = 10000;

//

//

/* qqq : extend and implement tests for other routines */
function make( test )
{

  test.case = 'without arguments';
  var got = _.map.make();
  var expected = {};
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );

  test.case = 'src - null';
  var got = _.map.make( null );
  var expected = {};
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );

  // test.case = 'src - undefined';
  // var got = _.map.make( undefined );
  // var expected = {};
  // test.identical( got, expected );
  // test.true( _.mapIsPure( got ) );

  /* */

  test.case = 'src - empty map';
  var src = {};
  var got = _.map.make( src );
  var expected = {};
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'src - not pure map';
  var src = { a : 7, b : 13 };
  var got = _.map.make( src );
  var expected = { a : 7, b : 13 };
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'src - empty pure map';
  var src = Object.create( null );
  var got = _.map.make( src );
  var expected = {};
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'src - pure map';
  var src = Object.create( null );
  src.a = 7;
  src.b = 13;
  var got = _.map.make( src );
  var expected = { a : 7, b : 13 };
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'src - empty Map';
  var src = new HashMap([]);
  var got = _.map.make( src );
  var expected = {};
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'src - pure map';
  var src = new HashMap( [ [ 'a', 1 ], [ 2, 2 ] ] );
  var got = _.map.make( src );
  var expected = { '2' : 2, a : 1 };
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  /* */

  test.case = 'src - empty array';
  var src = [];
  var got = _.map.make( src );
  var expected = {};
  test.identical( got, expected );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  // test.case = 'src - array with primitives';
  // var src = [ 0, 'str', null, undefined ];
  // var got = _.map.make( src );
  // var expected = { 0 : 0, 1 : 'str', 2 : null, 3 : undefined };
  // test.identical( got, expected );
  // test.true( _.mapIsPure( got ) );
  // test.true( got !== src );
  //
  // test.case = 'src - array with maps';
  // var src = [ { a : 7 }, { b : 13 } ];
  // var got = _.map.make( src );
  // var expected = { 0 : { a : 7 }, 1 : { b : 13 } };
  // test.identical( got, expected );
  // test.true( _.mapIsPure( got ) );
  // test.true( got !== src );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.map.make( { a : 1 }, { a : 'extra' } ) );

  test.case = 'wrong argument';
  test.shouldThrowErrorSync( () => _.map.make( '' ) );
  test.shouldThrowErrorSync( () => _.map.make( 1 ) );
  test.shouldThrowErrorSync( () => _.map.make( false ) );
  test.shouldThrowErrorSync( () => _.map.make( undefined ) );
  test.shouldThrowErrorSync( () => _.map.make( [ 0, 'str', null, undefined ] ) );
  test.shouldThrowErrorSync( () => _.map.make( [ { a : 7 }, { b : 13 } ] ) );

}

//

function makeBugWithArray( test )
{
  test.case = 'failed';
  var src = [ { a : 1 }, { b : 2 } ];
  var got = _.map.make.apply( undefined, src );
  var exp = { 0 : { a : 1 }, 1 : { b : 2 } };
  test.identical( got, exp );
  test.true( got !== src );

  test.case = 'all ok';
  var src = [ { a : 1 }, { b : 2 } ];
  var got = _.map.make( src );
  var exp = { 0 : { a : 1 }, 1 : { b : 2 } };
  test.identical( got, exp );
  test.true( got !== src );
}
makeBugWithArray.experimental = 1;
makeBugWithArray.description =
`
 routines mapBut and _mapOnly uncorrect use method apply()
 Previus call was :
   _.map.make.apply( this, src ); // src = [ {...}, {...} ]
   its equivalent to
   _.map.make( {...}, {...} );

   After changing behavior of make call should be
   _.map.make.apply( this, [ src ] );
`

// --
// container
// --

/* qqq : for junior : rewrite tests, please */
function keys( test )
{

  test.case = 'trivial';

  var got = _.map.keys( {} );
  var expected = [];
  test.identical( got, expected );

  var got = _.map.keys( { a : 1, b : undefined } );
  var expected = [ 'a', 'b' ];
  test.identical( got, expected );

  var got = _.map.keys( { a : 7, b : 13 } );
  var expected = [ 'a', 'b' ];
  test.identical( got, expected );

  var got = _.map.keys( { 7 : 'a', 3 : 'b', 13 : 'c' } );
  var expected = [ '3', '7', '13' ];
  test.identical( got, expected );

  // var f = () => {};
  // Object.setPrototypeOf( f, String );
  // f.a = 1;
  // var got = _.map.keys( f );
  // var expected = [ 'a' ];
  // test.identical( got, expected );
  //
  // var got = _.map.keys( new Date );
  // var expected = [ ];
  // test.identical( got, expected );

  /* */

  // test.case = 'options';
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  //
  // /* onlyOwn off */
  //
  // var got = _.map.keys( a );
  // var expected = [ 'a', 'b' ];
  // test.identical( got, expected );
  //
  // /* onlyOwn on */
  //
  // var o = { onlyOwn : 1 };
  // var got = _.map.keys( a, o );
  // var expected = [ 'a' ];
  // test.identical( got, expected );
  //
  // /* enumerable/onlyOwn off */
  //
  // var o = { onlyEnumerable : 0, onlyOwn : 0 };
  // Object.defineProperty( b, 'k', { enumerable : 0 } );
  // var got = _.map.keys( a, o );
  // var expected = _.map.allKeys( a );
  // test.identical( got, expected );
  //
  // /* enumerable off, onlyOwn on */
  //
  // var o = { onlyEnumerable : 0, onlyOwn : 1 };
  // Object.defineProperty( a, 'k', { enumerable : 0 } );
  // var got = _.map.keys( a, o );
  // var expected = [ 'a', 'k' ]
  // test.identical( got, expected );
  //
  // /* */
  //
  // test.case = 'onlyEnumerable : 0, onlyOwn : 0'
  // var a = Object.create( null );
  // a.a = 1;
  // Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  // var b = Object.create( a );
  // b.b = 2;
  // var exp = [ 'b', 'a', 'k' ];
  // var got = _.map.keys( b, { onlyEnumerable : 0, onlyOwn : 0 } );
  // test.identical( got, exp );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.keys();
  });

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.keys( 'wrong arguments' );
  });

  test.case = 'unknonlyOwn option';
  test.shouldThrowErrorSync( function()
  {
    _.map.keys( { x : 1 }, { 'wrong' : null } );
  });

}

//

function onlyOwnKeys( test )
{
  test.case = 'empty'
  var got = _.map.onlyOwnKeys( {} );
  var expected = [];
  test.identical( got, expected )

  /* */

  test.case = 'simplest'

  var got = _.map.onlyOwnKeys( { a : '1', b : '2' } );
  var expected = [ 'a', 'b' ];
  test.identical( got, expected )

  // var got = _.map.onlyOwnKeys( new Date );
  // var expected = [ ];
  // test.identical( got, expected )

  /* */

  test.case = ''

  var a = { a : 1 };
  var b = { b : 2 };
  var c = { c : 3 };
  Object.setPrototypeOf( a, b );
  Object.setPrototypeOf( b, c );

  var got = _.map.onlyOwnKeys( a );
  var expected = [ 'a' ];
  test.identical( got, expected )

  var got = _.map.onlyOwnKeys( b );
  var expected = [ 'b' ];
  test.identical( got, expected )

  var got = _.map.onlyOwnKeys( c );
  var expected = [ 'c' ];
  test.identical( got, expected );

  /* */

  test.case = 'enumerable on/off';
  var a = { a : '1' };

  var got = _.map.onlyOwnKeys( a );
  var expected = [ 'a' ]
  test.identical( got, expected );

  Object.defineProperty( a, 'k', { enumerable : false } );
  var o = { onlyEnumerable : 0 };
  var got = _.map.onlyOwnKeys( a, o );
  var expected = [ 'a', 'k' ]
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnKeys();
  })

  test.case = 'invalid type';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnKeys( 1 );
  })

  test.case = 'unknonlyOwn option';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnKeys( { onlyOwn : 0 }, { 'wrong' : null } );
  })

}

//

function allKeys( test )
{
  var _expected =
  [
    '__defineGetter__',
    '__defineSetter__',
    'hasOwnProperty',
    '__lookupGetter__',
    '__lookupSetter__',
    'propertyIsEnumerable',
    '__proto__',
    'constructor',
    'toString',
    'toLocaleString',
    'valueOf',
    'isPrototypeOf'
  ]

  /* */

  test.case = 'empty'
  var got = _.map.allKeys( {} );
  test.identical( got.sort(), _expected.sort() )

  /* */

  test.case = 'one onlyOwn property'
  var got = _.map.allKeys( { a : 1 } );
  var expected = _expected.slice();
  expected.push( 'a' );
  test.identical( got.sort(), expected.sort() )

  /* */

  // test.case = 'date'
  // var got = _.map.allKeys( new Date );
  // test.identical( got.length, 55 );

  /* */

  test.case = 'not enumerable'
  var a = { };
  Object.defineProperty( a, 'k', { enumerable : 0 } )
  var got = _.map.allKeys( a );
  var expected = _expected.slice();
  expected.push( 'k' );
  test.identical( got.sort(), expected.sort() );

  /* */

  // test.case = 'from prototype'
  // var a = { a : 1 };
  // var b = { b : 1 };
  // Object.setPrototypeOf( a, b );
  // Object.defineProperty( a, 'k', { enumerable : 0 } );
  // Object.defineProperty( b, 'y', { enumerable : 0 } );
  // var got = _.map.allKeys( a );
  // var expected = _expected.slice();
  // expected = expected.concat( [ 'a', 'b', 'k', 'y' ] );
  // test.identical( got.sort(), expected.sort() );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args';
  test.shouldThrowErrorSync( function()
  {
    _.map.allKeys();
  })

  test.case = 'invalid argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.allKeys();
  })

  test.case = 'unknonlyOwn option';
  test.shouldThrowErrorSync( function()
  {
    _.map.allKeys( { onlyOwn : 0 }, { 'wrong' : null } );
  })

}

//

function vals( test )
{

  test.case = 'trivial';

  var got = _.map.vals( {} );
  var expected = [];
  test.identical( got, expected );

  var got = _.map.vals( { a : 1, b : undefined } );
  var expected = [ 1, undefined ];
  test.identical( got, expected );

  var got = _.map.vals( { a : 7, b : 13 } );
  var expected = [ 7, 13 ];
  test.identical( got, expected );

  var got = _.map.vals( { 7 : 'a', 3 : 'b', 13 : 'c' } );
  var expected = [ 'b', 'a', 'c' ];
  test.identical( got, expected );

  // var got = _.map.vals( new Date );
  // var expected = [ ];
  // test.identical( got, expected );

  /* */

  // test.case = 'onlyOwn'
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  //
  // var o = { onlyOwn : 0, onlyEnumerable : 1 };
  // var got = _.map.vals( a, o );
  // var expected = [ 1, 2 ]
  // test.identical( got, expected );
  //
  // var o = { onlyOwn : 1, onlyEnumerable : 1 };
  // var got = _.map.vals( a, o );
  // var expected = [ 1 ];
  // test.identical( got, expected );

  /* */

  test.case = 'enumerable'
  var a = { a : 1 };
  Object.defineProperty( a, 'k', { enumerable : 0, value : 2 } );

  /* */

  var got = _.map.vals( a, { onlyEnumerable : 1, onlyOwn : 0 } );
  var expected = [ 1 ];
  test.identical( got, expected );

  /* */

  // test.case = 'onlyEnumerable : 0, onlyOwn : 0'
  // var a = Object.create( null );
  // a.a = 1;
  // Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  // var b = Object.create( a );
  // b.b = 2;
  // var exp = [ 2, 1, 3 ];
  // var got = _.map.vals( b, { onlyEnumerable : 0, onlyOwn : 0 } );
  // test.identical( got, exp );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.vals();
  });

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.vals( 'wrong argument' );
  });

  test.case = 'wrong option';
  test.shouldThrowErrorSync( function()
  {
    _.map.vals( { a : 1 }, { 'wrong' : null } );
  });

}

//

function onlyOwnVals( test )
{

  test.case = 'trivial';

  var got = _.map.onlyOwnVals( {} );
  var expected = [];
  test.identical( got, expected );

  var got = _.map.onlyOwnVals( { a : 7, b : 13 } );
  var expected = [ 7, 13 ];
  test.identical( got, expected );

  var got = _.map.onlyOwnVals( { 7 : 'a', 3 : 'b', 13 : 'c' } );
  var expected = [ 'b', 'a', 'c' ];
  test.identical( got, expected );

  // var got = _.map.onlyOwnVals( new Date );
  // var expected = [ ];
  // test.identical( got, expected );

  /* */

  test.case = ' only onlyOwn values'
  var a = { a : 1 };
  var b = { b : 2 };
  Object.setPrototypeOf( a, b );

  /* */

  var got = _.map.onlyOwnVals( a );
  var expected = [ 1 ];
  test.identical( got, expected );

  /* enumerable off */

  test.case = 'enumerable - 0';
  Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  Object.defineProperty( b, 'y', { enumerable : 0, value : 4 } );
  var got = _.map.onlyOwnVals( a, { onlyEnumerable : 0 } );
  var expected = [ 1, 3 ];
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnVals();
  });

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnVals( 'wrong argument' );
  });

  test.case = 'wrong option';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnVals( { a : 1 }, { 'wrong' : null } );
  });

}

//

function allVals( test )
{
  test.case = 'trivial';

  var got = _.map.allVals( {} );
  test.true( got.length !== 0 );

  /* */

  var got = _.map.allVals( { a : 7, b : 13 } );
  test.true( got.length !== 0 );
  test.true( got.indexOf( 7 ) !== -1 );
  test.true( got.indexOf( 13 ) !== -1 );

  /* */

  // var got = _.map.allVals( new Date );
  // test.true( got.length > _.map.allVals( {} ).length );

  /* */

  // test.case = 'from prototype'
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  // var got = _.map.allVals( a );
  // var expected = [ 1 ];
  // test.true( got.length > _.map.allVals( {} ).length );
  // test.true( got.indexOf( 1 ) !== -1 );
  // test.true( got.indexOf( 2 ) !== -1 );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.allVals();
  });

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.allVals( 'wrong argument' );
  });

  test.case = 'wrong option';
  test.shouldThrowErrorSync( function()
  {
    _.map.allVals( { a : 1 }, { 'wrong' : null } );
  });

}

//

function pairs( test )
{

  test.case = 'empty';

  var got = _.map.pairs( {} );
  var expected = [];
  test.identical( got, expected );

  // var got = _.map.pairs( [] );
  // var expected = [];
  // test.identical( got, expected );

  /* */

  test.case = 'list of [ key, value ] pairs';

  var got = _.map.pairs( { a : 7, b : 13 } );
  var expected = [ [ 'a', 7 ], [ 'b', 13 ] ];
  test.identical( got, expected );

  test.case = 'list of [ key, value ] pairs'
  var got = _.map.pairs( { a : 3, b : 13, c : 7 } );
  var expected = [ [ 'a', 3 ], [ 'b', 13 ], [ 'c', 7 ] ];
  test.identical( got, expected );

  /* */

  // var arrObj = [];
  // arrObj[ 'k' ] = 1;
  // var got = _.map.pairs( arrObj );
  // var expected = [ [ 'k', 1 ] ];
  // test.identical( got, expected );
  //
  // /* */
  //
  // var got = _.map.pairs( new Date );
  // var expected = [];
  // test.identical( got, expected );
  //
  // /* */
  //
  // test.case = 'from prototype';
  //
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  // var got = _.map.pairs( a );
  // var expected = [ [ 'a', 1 ], [ 'b', 2 ] ];
  // test.identical( got, expected );
  //
  // /* using onlyOwn */
  //
  // var got = _.map.pairs( a, { onlyOwn : 1 } );
  // var expected = [ [ 'a', 1 ] ];
  // test.identical( got, expected );
  //
  // /* using enumerable off, onlyOwn on */
  //
  // Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  // var o = { onlyEnumerable : 0, onlyOwn : 1 };
  // var got = _.map.pairs( a, o );
  // var expected = [ [ 'a', 1 ], [ 'k', 3 ] ];
  // test.identical( got, expected );
  //
  // /* using enumerable off, onlyOwn off */
  //
  // Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  // var o = { onlyEnumerable : 0, onlyOwn : 0 };
  // var got = _.map.pairs( a, o );
  // test.true( got.length > 2 );
  // test.identical( got[ 0 ], [ 'a', 1 ] );
  // test.identical( got[ 1 ], [ 'k', 3 ] );

  test.case = 'src - empty map';
  var got = _.map.pairs( {} );
  var expected = [];
  test.identical( got, expected );

  // test.case = 'src - empty array';
  // var got = _.map.pairs( [] );
  // var expected = [];
  // test.identical( got, expected );

  test.case = 'src - filled map';
  var got = _.map.pairs( { a : 7, b : 13 } );
  var expected = [ [ 'a', 7 ], [ 'b', 13 ] ];
  test.identical( got, expected );

  test.case = 'src - filled map'
  var got = _.map.pairs( { a : 3, b : 13, 1 : 7 } );
  var expected = [ [ '1', 7 ], [ 'a', 3 ], [ 'b', 13 ] ];
  test.identical( got, expected );

  // test.case = 'src - array with literal key';
  // var arrObj = [];
  // arrObj[ 'k' ] = 1;
  // var got = _.map.pairs( arrObj );
  // var expected = [ [ 'k', 1 ] ];
  // test.identical( got, expected );
  //
  // test.case = 'src - Date object';
  // var got = _.map.pairs( new Date );
  // var expected = [];
  // test.identical( got, expected );
  //
  // test.case = 'src - map prototyped by another map';
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  // var got = _.map.pairs( a );
  // var expected = [ [ 'a', 1 ], [ 'b', 2 ] ];
  // test.identical( got, expected );
  //
  // test.case = 'src - map prototyped by another map, onlyOwn pairs';
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  // var got = _.map.pairs( a, { onlyOwn : 1 } );
  // var expected = [ [ 'a', 1 ] ];
  // test.identical( got, expected );
  //
  // test.case = 'src - map prototyped by another map, onlyOwn pairs, not enumerable property';
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  // Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  // var got = _.map.pairs( a, { onlyEnumerable : 0, onlyOwn : 1 } );
  // var expected = [ [ 'a', 1 ], [ 'k', 3 ] ];
  // test.identical( got, expected );
  //
  // test.case = 'src - map prototyped by another map, onlyOwn pairs disable, not enumerable property';
  // var a = Object.create( null );
  // a.a = 1;
  // var b = Object.create( null );
  // b.b = 2;
  // Object.setPrototypeOf( a, b );
  // Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  // var got = _.map.pairs( a, { onlyEnumerable : 0, onlyOwn : 0 } );
  // var expected = [ [ 'a', 1 ], [ 'k', 3 ], [ 'b', 2 ] ];
  // test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without argument';
  test.shouldThrowErrorSync( () => _.map.pairs() );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.map.pairs( 1 ) );
  test.shouldThrowErrorSync( () => _.map.pairs( 'wrong' ) );
  test.shouldThrowErrorSync( () => _.map.pairs( Object.create( Object.create( null ) ) ) );

  /* */

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.pairs();
  });

  /* */

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.pairs();
  });

  test.case = 'primitive';
  test.shouldThrowErrorSync( function()
  {
    _.map.pairs( 1 );
  });

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.pairs( 'wrong argument' );
  });

  test.case = 'redundant argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.pairs( {}, 'wrong arguments' );
  });

  test.case = 'wrong type of array';
  test.shouldThrowErrorSync( function()
  {
    _.map.pairs( null );
  });

}

//

function onlyOwnPairs( test )
{
  test.case = 'empty';
  var got = _.map.onlyOwnPairs( {} );
  var expected = [];
  test.identical( got, expected );

  /* */

  test.case = 'list of [ key, value ] pairs';

  var got = _.map.onlyOwnPairs( { a : 7, b : 13 } );
  var expected = [ [ 'a', 7 ], [ 'b', 13 ] ];
  test.identical( got, expected );

  /* */

  var arrObj = [];
  arrObj[ 'k' ] = 1;
  var got = _.map.onlyOwnPairs( arrObj );
  var expected = [ [ 'k', 1 ] ];
  test.identical( got, expected );

  /* */

  // var got = _.map.onlyOwnPairs( new Date );
  // var expected = [];
  // test.identical( got, expected );

  /* */

  test.case = 'from prototype';

  var a = { a : 1 };
  var b = { b : 2 };
  Object.setPrototypeOf( a, b );
  var got = _.map.onlyOwnPairs( a );
  var expected = [ [ 'a', 1 ] ];
  test.identical( got, expected );

  /* using enumerable off */

  Object.defineProperty( a, 'k', { enumerable : 0, value : 3 } );
  var got = _.map.onlyOwnPairs( a, { onlyEnumerable : 0 } );
  var expected = [ [ 'a', 1 ], [ 'k', 3 ] ];
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnPairs();
  });

  test.case = 'primitive';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnPairs( 1 );
  });

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.onlyOwnPairs( 'wrong argument' );
  });

}

//

function allPairs( test )
{
  test.case = 'empty';
  var got = _.map.allPairs( {} );
  test.true( got.length !== 0 );

  /* */

  test.case = 'list of [ key, value ] pairs';

  var got = _.map.allPairs( { a : 7, b : 13 } );
  test.true( got.length > 2 );
  test.identical( got[ 0 ], [ 'a', 7 ] );
  test.identical( got[ 1 ], [ 'b', 13 ] );

  /* */

  // var arrObj = [];
  // arrObj[ 'k' ] = 1;
  // var got = _.map.allPairs( arrObj );
  // test.true( got.length > 1 );
  // got = _.arrayFlatten( [], got );
  // test.true( got.indexOf( 'k' ) !== -1 );
  // test.identical( got[ got.indexOf( 'k' ) + 1 ], 1 );

  /* */

  // var got = _.map.allPairs( new Date );
  // test.true( got.length > 1 );
  // got = _.arrayFlatten( [], got );
  // test.true( got.indexOf( 'constructor' ) !== -1 );
  // test.identical( got[ got.indexOf( 'constructor' ) + 1 ].name, 'Date' );

  /* */

  // test.case = 'from prototype';
  //
  // var a = { a : 1 };
  // var b = { b : 2 };
  // Object.setPrototypeOf( a, b );
  // var got = _.map.allPairs( a );
  // test.true( got.length > 2 );
  // test.identical( got[ 0 ], [ 'a', 1 ] );
  // test.identical( got[ 1 ], [ 'b', 2 ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.allPairs();
  });

  test.case = 'primitive';
  test.shouldThrowErrorSync( function()
  {
    _.map.allPairs( 1 );
  });

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.allPairs( 'wrong argument' );
  });

}

//

/* qqq : extend */
function extend( test )
{
  test.open( 'first argument is null' );

  test.case = 'trivial';
  var src1 = { a : 1, b : 2 };
  var src1Copy = { a : 1, b : 2 };
  var src2 = { c : 3, d : 4 };
  var src2Copy = { c : 3, d : 4 };
  var got = _.map.extend( null, src1, src2 );
  var expected = { a : 1, b : 2, c : 3, d : 4 };
  test.will = 'return';
  test.identical( got, expected );
  test.will = 'preserve src1';
  test.identical( src1, src1Copy );
  test.will = 'preserve src2';
  test.identical( src2, src2Copy );
  test.will = 'return not src1';
  test.true( got !== src1 );
  test.will = 'return not src2';
  test.true( got !== src2 );

  /* */

  test.case = 'rewriting';
  var src1 = { a : 1, b : 2 };
  var src1Copy = { a : 1, b : 2 };
  var src2 = { b : 22, c : 3, d : 4 };
  var src2Copy = { b : 22, c : 3, d : 4 };
  var got = _.map.extend( null, src1, src2 );
  var expected = { a : 1, b : 22, c : 3, d : 4 };
  test.will = 'return';
  test.identical( got, expected );
  test.will = 'preserve src1';
  test.identical( src1, src1Copy );
  test.will = 'preserve src2';
  test.identical( src2, src2Copy );
  test.will = 'return not src1';
  test.true( got !== src1 );
  test.will = 'return not src2';
  test.true( got !== src2 );

  test.close( 'first argument is null' );

  /* - */

  test.open( 'first argument is dst' );

  test.case = 'trivial';
  var dst = { a : 1, b : 2 };
  var src2 = { c : 3, d : 4 };
  var src2Copy = { c : 3, d : 4 };
  var got = _.map.extend( dst, src2 );
  var expected = { a : 1, b : 2, c : 3, d : 4 };
  test.will = 'return';
  test.identical( got, expected );
  test.will = 'preserve src2';
  test.identical( src2, src2Copy );
  test.will = 'return dst';
  test.true( got === dst );
  test.will = 'return not src2';
  test.true( got !== src2 );

  /* */

  test.case = 'rewriting';
  var dst = { a : 1, b : 2 };
  var src2 = { b : 22, c : 3, d : 4 };
  var src2Copy = { b : 22, c : 3, d : 4 };
  var got = _.map.extend( dst, src2 );
  var expected = { a : 1, b : 22, c : 3, d : 4 };
  test.will = 'return';
  test.identical( got, expected );
  test.will = 'preserve src2';
  test.identical( src2, src2Copy );
  test.will = 'return not dst';
  test.true( got === dst );
  test.will = 'return not src2';
  test.true( got !== src2 );

  test.close( 'first argument is dst' );

  /* - */

  test.case = 'trivial, first argument';
  var src1 = { a : 7, b : 13 };
  var src1Copy = { a : 7, b : 13 };
  var src2 = { c : 3, d : 33 };
  var src2Copy = { c : 3, d : 33 };
  var got = _.map.extend( src1, src2 );
  var expected = { a : 7, b : 13, c : 3, d : 33 };
  test.identical( got, expected );
  test.identical( src2, src2Copy );
  test.true( got === src1 );
  test.true( got !== src2 );

  test.case = 'complex, first argument is null';
  var src1 = { a : 1, b : 1, c : 1, z : 1 };
  var src1Copy = { a : 1, b : 1, c : 1, z : 1 };
  var src2 = { a : 2, c : 2, d : 2 };
  var src2Copy = { a : 2, c : 2, d : 2 };
  var src3 = { a : 3, b : 3, e : 3 };
  var src3Copy = { a : 3, b : 3, e : 3 };
  var got = _.map.extend( null, src1, src2, src3 );
  var expected = { a : 3, b : 3, c : 2, d : 2, e : 3, z : 1 };
  test.identical( got, expected );
  test.identical( src1, src1Copy );
  test.identical( src2, src2Copy );
  test.identical( src3, src3Copy );
  test.true( got !== src1 );
  test.true( got !== src2 );
  test.true( got !== src3 );

  test.case = 'complex, first argument is not null';
  var src1 = { a : 1, b : 1, c : 1, z : 1 };
  var src1Copy = { a : 1, b : 1, c : 1, z : 1 };
  var src2 = { a : 2, c : 2, d : 2 };
  var src2Copy = { a : 2, c : 2, d : 2 };
  var src3 = { a : 3, b : 3, e : 3 };
  var src3Copy = { a : 3, b : 3, e : 3 };
  var got = _.map.extend( src1, src2, src3 );
  var expected = { a : 3, b : 3, c : 2, d : 2, e : 3, z : 1 };
  test.identical( got, expected );
  test.identical( src2, src2Copy );
  test.identical( src3, src3Copy );
  test.true( got === src1 );
  test.true( got !== src2 );
  test.true( got !== src3 );

  test.case = 'extend pure map by empty strings, first argument is null';
  var src1 = Object.create( null );
  src1.a = '1';
  src1.b = '1';
  src1.c = '1';
  src1.z = '1';
  var src1Copy = Object.create( null );
  src1Copy.a = '1';
  src1Copy.b = '1';
  src1Copy.c = '1';
  src1Copy.z = '1';
  var src2 = Object.create( null );
  src2.a = '';
  src2.c = '';
  src2.d = '';
  src2.e = '2';
  var src2Copy = Object.create( null );
  src2Copy.a = '';
  src2Copy.c = '';
  src2Copy.d = '';
  src2Copy.e = '2';
  var got = _.map.extend( null, src1, src2 );
  var expected = { a : '', b : '1', c : '', d : '', e : '2', z : '1' };
  test.identical( got, expected );
  test.identical( src1, src1Copy );
  test.identical( src2, src2Copy );
  test.true( got !== src1 );
  test.true( got !== src2 );

  test.case = 'extend pure map by empty strings, first argument is not null';
  var src1 = Object.create( null );
  src1.a = '1';
  src1.b = '1';
  src1.c = '1';
  src1.z = '1';
  var src1Copy = Object.create( null );
  src1Copy.a = '1';
  src1Copy.b = '1';
  src1Copy.c = '1';
  src1Copy.z = '1';
  var src2 = Object.create( null );
  src2.a = '';
  src2.c = '';
  src2.d = '';
  src2.e = '2';
  var src2Copy = Object.create( null );
  src2Copy.a = '';
  src2Copy.c = '';
  src2Copy.d = '';
  src2Copy.e = '2';
  var got = _.map.extend( src1, src2 );
  var expected = { a : '', b : '1', c : '', d : '', e : '2', z : '1' };
  test.identical( got, expected );
  test.identical( src2, src2Copy );
  test.true( got === src1 );
  test.true( got !== src2 );

  test.case = 'object like array';
  var got = _.map.extend( null, [ 3, 7, 13, 73 ] );
  var expected = { 0 : 3, 1 : 7, 2 : 13, 3 : 73 };
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( () => _.map.extend() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.map.extend( {} ) );

  test.case = 'wrong type of src';
  test.shouldThrowErrorSync( () => _.map.extend( [], 'wrong' ) );

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.map.extend( undefined, {} ) );
  // test.shouldThrowErrorSync( () => _.map.extend( Object.create( Object.create( null ) ), {} ) );

}

//

/* qqq : extend */
function supplement( test )
{

  test.case = 'an object';
  var got = _.map.supplement( { a : 1, b : 2 }, { a : 1, c : 3 } );
  var expected = { a : 1, b : 2, c : 3 };
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no argument';
  test.shouldThrowErrorSync( function()
  {
    _.map.supplement();
  });

  test.case = 'wrong type of arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.supplement( 'wrong arguments' );
  });

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.map.supplement( Object.create( Object.create( null ) ), {} ) );

}

// --
// define test suite
// --

const Proto =
{

  name : 'Tools.Map.l0.l1',
  silencing : 1,

  tests :
  {

    // dichotomy

    dichotomy,

    // maker

    make,
    makeBugWithArray,

    // container

    keys, /* qqq : for junior : rewrite tests, please */
    // onlyOwnKeys, /* qqq : for junior : rewrite tests, please */
    allKeys, /* qqq : for junior : rewrite tests, please */

    vals, /* qqq : for junior : rewrite tests, please */
    // onlyOwnVals, /* qqq : for junior : rewrite tests, please */
    allVals, /* qqq : for junior : rewrite tests, please */

    pairs, /* qqq : for junior : rewrite tests, please */
    // onlyOwnPairs, /* qqq : for junior : rewrite tests, please */
    allPairs, /* qqq : for junior : rewrite tests, please */

    extend,
    supplement,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
