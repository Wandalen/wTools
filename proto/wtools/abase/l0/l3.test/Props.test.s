( function _l0_l3_Props_test_s()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( '../Include3.s' );
  // const _ = require( 'Tools' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

//--
// map checker
//--

/* qqq : for junior : extend with construbile* cases */
/* qqq : for junior : extend with props checks */
function dichotomy( test )
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
  test.identical( _.map.isEmpty( src ), true );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), true );
  test.identical( _.aux.isPopulated( src ), true );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), true );
  test.identical( _.aux.isPopulated( src ), true );

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
  test.identical( _.map.isEmpty( src ), true );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), true );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), true );
  test.identical( _.aux.isPopulated( src ), true );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), true );
  test.identical( _.aux.isPopulated( src ), true );

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
  test.identical( _.map.isEmpty( src ), true );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), true );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

  test.case = 'null';
  var src = null;
  test.identical( _.object.isBasic( src ), false );
  test.identical( _.object.like( src ), false );
  test.identical( _.mapIs( src ), false );
  test.identical( _.aux.is( src ), false );
  test.identical( _.aux.isPrototyped( src ), false );
  test.identical( _.mapIsPure( src ), false );
  test.identical( _.mapIsPolluted( src ), false );
  test.identical( _.aux.isPolluted( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), true );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), true );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), false );

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
  test.identical( _.map.isEmpty( src ), false );
  test.identical( _.aux.isEmpty( src ), false );
  test.identical( _.map.isPopulated( src ), false );
  test.identical( _.aux.isPopulated( src ), true );
}

dichotomy.timeOut = 10000;

//

function exportStringDiagnosticShallow( test )
{

  test.case = 'pure empty map';
  var src = Object.create( null );
  var exp = '{- Map.pure with 0 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'pure map';
  var src = Object.create( null );
  var exp = '{- Map.pure with 1 elements -}';
  src.x = 1;
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'empty polluted map';
  var src = {};
  var exp = '{- Map.polluted with 0 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'polluted map';
  var src = { a : 7, b : 13 };
  var exp = '{- Map.polluted with 2 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'prototyped from pure map';
  var prototype = Object.create( null );
  var src = Object.create( prototype );
  var exp = '{- Aux.pure.prototyped with 0 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'prototyped from pure map deep';
  var prototype1 = Object.create( null );
  var prototype2 = Object.create( prototype1 );
  var src = Object.create( prototype1 );
  var exp = '{- Aux.pure.prototyped with 0 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'prototyped from pure map deep with props';
  var prototype1 = Object.create( null );
  prototype1.a = 1;
  var prototype2 = Object.create( prototype1 );
  prototype2.b = 1;
  var src = Object.create( prototype1 );
  src.c = 1;
  var exp = '{- Aux.pure.prototyped with 2 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'prototyped from polluted map';
  var prototype = {};
  var src = Object.create( prototype );
  var exp = '{- Aux.polluted.prototyped with 0 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'prototyped from polluted map deep';
  var prototype1 = {};
  var prototype2 = Object.create( prototype1 );
  var src = Object.create( prototype1 );
  var exp = '{- Aux.polluted.prototyped with 0 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'prototyped from polluted map deep with props';
  var prototype1 = {};
  prototype1.a = 1;
  var prototype2 = Object.create( prototype1 );
  prototype2.b = 1;
  var src = Object.create( prototype1 );
  src.c = 1;
  var exp = '{- Aux.polluted.prototyped with 2 elements -}';
  var got = _.props.exportStringDiagnosticShallow( src );
  test.identical( got, exp );

  test.case = 'number with options map';
  var exp = '{- Number with 0 elements -}';
  var got = _.props.exportStringDiagnosticShallow( 13, {} );
  test.identical( got, exp );

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.props.exportStringDiagnosticShallow() );

  test.case = 'bad second argument'
  test.shouldThrowErrorSync( () => _.props.exportStringDiagnosticShallow( {}, 'a' ) );

  test.case = 'extra argument'
  test.shouldThrowErrorSync( () => _.props.exportStringDiagnosticShallow( [], {}, 'a' ) );

}

//

function identicalShallow( test )
{

  test.case = 'same map';
  var src = { a : 1 }
  var got = _.props.identicalShallow( src, src );
  test.identical( got, true );

  test.case = 'same values';
  var got = _.props.identicalShallow( { a : 7, b : 13 }, { a : 7, b : 13 } );
  var expected = true;
  test.identical( got, expected );

  test.case = 'same values with nested objects';
  var got = _.props.identicalShallow( { a : 7, b : { c : 13 } }, { a : 7, b : { c : 13 } } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'not the same values in'
  var got = _.props.identicalShallow( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b' : 14 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'different number of keys, more in the first argument'
  var got = _.props.identicalShallow( { 'a' : 7, 'b' : 13, 'с' : 15 }, { 'a' : 7, 'b' : 13 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'different number of keys, more in the second argument'
  var got = _.props.identicalShallow( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b' : 13, 'с' : 15 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty maps, standrard'
  var got = _.props.identicalShallow( {}, {} );
  var expected = true;
  test.identical( got, expected );

  test.case = 'empty maps, pure'
  var got = _.props.identicalShallow( Object.create( null ), Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'empty maps, one standard another pure'
  var got = _.props.identicalShallow( {}, Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'not object-like 1 arg';
  var got = _.props.identicalShallow( [ 'a', 7, 'b', 14 ], { a : 7, b : 14 } );
  var expected = false;
  test.identical( got, expected );
  var got = _.props.identicalShallow( { a : 7, b : 14 }, [ 'a', 7, 'b', 14 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'not object-like arguments';
  var got = _.props.identicalShallow( [ 'a', 7, 'b', 14 ], [ 'a', 7, 'b', 14 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'not object-like arguments';
  var got = _.props.identicalShallow( [ 'a', 7, 'b', 13 ], [ 'a', 7, 'b', 14 ] );
  var expected = false;
  test.identical( got, expected );

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.props.identicalShallow();
  });

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.props.identicalShallow( {} );
  });

  test.case = 'redundant arguments';
  test.shouldThrowErrorSync( function()
  {
    _.props.identicalShallow( {}, {}, {}, 'redundant argument' );
  });

}

//

function onlyImplicit( test )
{

  /* */

  test.case = 'pure map';
  var src = Object.create( null );
  src.a = '1';
  src[ Symbol.for( 'a' ) ] = '1';
  var exp = new HashMap();
  var got = _.props.onlyImplicit( src );
  test.identical( got, exp );

  /* */

  test.case = 'polluted map';
  var src = {};
  src.a = '1';
  src[ Symbol.for( 'a' ) ] = '1';
  var exp = new HashMap();
  exp.set( _.props.implicit.prototype, Object.prototype );
  var got = _.props.onlyImplicit( src );
  test.identical( got, exp );

  /* */

  test.case = 'prototyped from pure map';
  var prototype = Object.create( null );
  var src = Object.create( prototype );
  src.a = '1';
  src[ Symbol.for( 'a' ) ] = '1';
  var exp = new HashMap();
  exp.set( _.props.implicit.prototype, prototype );
  var got = _.props.onlyImplicit( src );
  test.identical( got, exp );

  /* */

  test.case = 'prototyped from pure map';
  var prototype = {};
  var src = Object.create( prototype );
  src.a = '1';
  src[ Symbol.for( 'a' ) ] = '1';
  var exp = new HashMap();
  exp.set( _.props.implicit.prototype, prototype );
  var got = _.props.onlyImplicit( src );
  test.identical( got, exp );

  /* */

  test.case = 'function';
  var src = function(){};
  src.a = '1';
  src[ Symbol.for( 'a' ) ] = '1';
  var exp = new HashMap();
  exp.set( _.props.implicit.prototype, Object.getPrototypeOf( Function ) );
  var got = _.props.onlyImplicit( src );
  test.identical( got, exp );

  /* */

  test.case = 'array';
  var src = [];
  src.a = '1';
  src[ Symbol.for( 'a' ) ] = '1';
  var exp = new HashMap();
  exp.set( _.props.implicit.prototype, Array.prototype );
  var got = _.props.onlyImplicit( src );
  test.identical( got, exp );

  /* */

  test.case = 'arguments array';
  var src = _.argumentsArray.make();
  src.a = '1';
  src[ Symbol.for( 'a' ) ] = '1';
  var exp = new HashMap();
  exp.set( _.props.implicit.prototype, Object.prototype );
  var got = _.props.onlyImplicit( src );
  test.identical( got, exp );

  /* */

}

//

function aptLeft( test )
{

  /* */

  test.case = 'left';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  var got = _.props.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 1 )
    return val + 10;
  });
  var exp = [ 11, 'a', 0, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'middle';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  var got = _.props.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 2 )
    return val + 10;
  });
  var exp = [ 12, 'b', 1, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'right';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  var got = _.props.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 3 )
    return val + 10;
  });
  var exp = [ 13, 'c', 2, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      3,
      'c',
      2,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'no';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  var got = _.props.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ 3, 'c', 2, false ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      3,
      'c',
      2,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'empty';
  var src = Object.create( Object.create( null ) );
  var ops = [];
  var got = _.props.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ undefined, undefined, -1, false ];
  test.identical( got, exp );
  var exp =
  [
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'without callback';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var exp = [ 1, 'a', 0, true ];
  var got = _.props.aptLeft( src );
  test.identical( got, exp );
  var exp = [ 1, 'a', 0, true ];
  var got = _.props.first( src );
  test.identical( got, exp );

  /* */

  test.case = 'without callback, empty';
  var src = Object.create( Object.create( null ) );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.props.aptLeft( src );
  test.identical( got, exp );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.props.first( src );
  test.identical( got, exp );

  /* */

}

//

function aptRight( test )
{

  /* */

  test.case = 'left';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  debugger;
  var got = _.props.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 1 )
    return val + 10;
  });
  debugger;
  var exp = [ 11, 'a', 0, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      1,
      'a',
      0,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'middle';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  var got = _.props.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 2 )
    return val + 10;
  });
  var exp = [ 12, 'b', 1, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'right';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  var got = _.props.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 3 )
    return val + 10;
  });
  var exp = [ 13, 'c', 2, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'no';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var ops = [];
  var got = _.props.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ 1, 'a', 0, false ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      1,
      'a',
      0,
      src,
    ],
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'empty';
  var src = Object.create( Object.create( null ) );
  var ops = [];
  var got = _.props.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ undefined, undefined, -1, false ];
  test.identical( got, exp );
  var exp =
  [
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'without callback';
  var src = Object.create({ a : 1, b : 2, c : 3 });
  var exp = [ 3, 'c', 2, true ];
  var got = _.props.aptRight( src );
  test.identical( got, exp );
  var exp = [ 3, 'c', 2, true ];
  var got = _.props.last( src );
  test.identical( got, exp );

  /* */

  test.case = 'without callback, empty';
  var src = Object.create( Object.create( null ) );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.props.aptRight( src );
  test.identical( got, exp );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.props.last( src );
  test.identical( got, exp );

  /* */

}

//

function filterMapCommonPass( test )
{
  let ops = [];
  let env;

  _.assert( _.routine.is( _.props.filterWithEscape ) );

  namespaceEach({ method : 'filterWithoutEscapeLeft', left : 1 });
  namespaceEach({ method : 'filterWithoutEscapeRight', left : 0 });
  namespaceEach({ method : 'filterWithoutEscape', left : 1 });
  namespaceEach({ method : 'filterWithEscapeLeft', left : 1 });
  namespaceEach({ method : 'filterWithEscapeRight', left : 0 });
  namespaceEach({ method : 'filterWithEscape', left : 1 });
  namespaceEach({ method : 'filter', left : 1 });

  namespaceEach({ method : 'mapWithoutEscapeLeft', left : 1 });
  namespaceEach({ method : 'mapWithoutEscapeRight', left : 0 });
  namespaceEach({ method : 'mapWithoutEscape', left : 1 });
  namespaceEach({ method : 'mapWithEscapeLeft', left : 1 });
  namespaceEach({ method : 'mapWithEscapeRight', left : 0 });
  namespaceEach({ method : 'mapWithEscape', left : 1 });
  namespaceEach({ method : 'map', left : 1 });

  /* - */

  function namespaceEach( env )
  {
    env.namespace = 'props';
    caseEach( env );
    env.namespace = 'map';
    caseEach( env );
    env.namespace = 'aux';
    caseEach( env );
    env.namespace = 'object';
    caseEach( env );
    env.namespace = 'container';
    caseEach( env );
  }

  /* - */

  function caseEach( _env )
  {
    test.case = `${__.entity.exportStringSolo( env )}`;
    env = _env;
    test.identical( _.strType( _[ env.namespace ][ env.method ] ), 'Routine' );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst <> src`;
    clean();
    var src = { a : 1, b : 2, c : 3 };
    var dst = { d : 4 }
    var got = _[ env.namespace ][ env.method ]( dst, src, f1 );
    test.true( got === dst );
    var exp = { d : 4, a : 11, b : 12, c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 };
    test.identical( src, exp );
    var exp = order([ 1, 2, 3 ]);
    test.identical( __.select( ops, '*/#0' ), exp );
    var exp = order([ 'a', 'b', 'c' ]);
    test.identical( __.select( ops, '*/#1' ), exp );
    var exp = order([ 0, 1, 2 ]);
    test.identical( __.select( ops, '*/#2' ), exp );
    var exp = order([ src, src, src ]);
    test.identical( __.select( ops, '*/#3' ), exp );
    var exp = order([ got, got, got ]);
    test.identical( __.select( ops, '*/#4' ), exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === null`;
    clean();
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( null, src, f1 );
    test.true( _[ env.namespace ].is( got ) );
    test.true( got !== src );
    var exp = { a : 11, b : 12, c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 }
    test.identical( src, exp );
    var exp = order([ 1, 2, 3 ]);
    test.identical( __.select( ops, '*/#0' ), exp );
    var exp = order([ 'a', 'b', 'c' ]);
    test.identical( __.select( ops, '*/#1' ), exp );
    var exp = order([ 0, 1, 2 ]);
    test.identical( __.select( ops, '*/#2' ), exp );
    var exp = order([ src, src, src ]);
    test.identical( __.select( ops, '*/#3' ), exp );
    var exp = order([ got, got, got ]);
    test.identical( __.select( ops, '*/#4' ), exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === src`;
    clean();
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( src, src, f1 );
    test.true( got === src );
    var exp = { a : 11, b : 12, c : 13 }
    test.identical( got, exp );
    var exp = order([ 1, 2, 3 ]);
    test.identical( __.select( ops, '*/#0' ), exp );
    var exp = order([ 'a', 'b', 'c' ]);
    test.identical( __.select( ops, '*/#1' ), exp );
    var exp = order([ 0, 1, 2 ]);
    test.identical( __.select( ops, '*/#2' ), exp );
    var exp = order([ src, src, src ]);
    test.identical( __.select( ops, '*/#3' ), exp );
    var exp = order([ got, got, got ]);
    test.identical( __.select( ops, '*/#4' ), exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === self`;
    clean();
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( _.self, src, f1 );
    test.true( got === src );
    var exp = { a : 11, b : 12, c : 13 }
    test.identical( got, exp );
    var exp = order([ 1, 2, 3 ]);
    test.identical( __.select( ops, '*/#0' ), exp );
    var exp = order([ 'a', 'b', 'c' ]);
    test.identical( __.select( ops, '*/#1' ), exp );
    var exp = order([ 0, 1, 2 ]);
    test.identical( __.select( ops, '*/#2' ), exp );
    var exp = order([ src, src, src ]);
    test.identical( __.select( ops, '*/#3' ), exp );
    var exp = order([ got, got, got ]);
    test.identical( __.select( ops, '*/#4' ), exp );

    /* */

  }

  /* - */

  function order( dst )
  {
    if( !env.left )
    dst.reverse();
    return dst;
  }

  /* - */

  function clean()
  {
    ops = [];
  }

  /* - */

  function f1( e, k, c, src )
  {
    ops.push([ ... arguments ]); debugger;
    return e + 10;
  }

  /* - */

}

filterMapCommonPass.rapidity = -1;
filterMapCommonPass.timeOut = 60000;

//

function filterCommonDropping( test )
{
  let env;

  namespaceEach({ method : 'filterWithoutEscapeLeft', left : 1 });
  namespaceEach({ method : 'filterWithoutEscapeRight', left : 0 });
  namespaceEach({ method : 'filterWithoutEscape', left : 1 });
  namespaceEach({ method : 'filterWithEscapeLeft', left : 1 });
  namespaceEach({ method : 'filterWithEscapeRight', left : 0 });
  namespaceEach({ method : 'filterWithEscape', left : 1 });
  namespaceEach({ method : 'filter', left : 1 });

  /* - */

  function namespaceEach( env )
  {
    env.namespace = 'props';
    caseEach( env );
    env.namespace = 'map';
    caseEach( env );
    env.namespace = 'aux';
    caseEach( env );
    env.namespace = 'object';
    caseEach( env );
    env.namespace = 'container';
    caseEach( env );
  }

  /* - */

  function caseEach( _env )
  {
    env = _env;

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst <> src`;
    var src = { a : 1, b : 2, c : 3 };
    var dst = { d : 4 }
    var got = _[ env.namespace ][ env.method ]( dst, src, f1 );
    test.true( got === dst );
    var exp = { d : 4, a : 11, c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 };
    test.identical( src, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === null`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( null, src, f1 );
    test.true( _[ env.namespace ].is( got ) );
    test.true( got !== src );
    var exp = { a : 11, c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 }
    test.identical( src, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === src`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( src, src, f1 );
    test.true( got === src );
    var exp = { a : 11, c : 13 }
    test.identical( got, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === self`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( _.self, src, f1 );
    test.true( got === src );
    var exp = { a : 11, c : 13 }
    test.identical( got, exp );

    /* */

  }

  /* - */

  function f1( e, k, c, src )
  {
    if( e === 2 )
    return undefined;
    return e + 10;
  }

  /* - */

}

filterCommonDropping.rapidity = -1;
filterCommonDropping.timeOut = 60000;

//

function mapCommonReturningUndefined( test )
{
  let env;

  namespaceEach({ method : 'mapWithoutEscapeLeft', left : 1 });
  namespaceEach({ method : 'mapWithoutEscapeRight', left : 0 });
  namespaceEach({ method : 'mapWithoutEscape', left : 1 });
  namespaceEach({ method : 'mapWithEscapeLeft', left : 1 });
  namespaceEach({ method : 'mapWithEscapeRight', left : 0 });
  namespaceEach({ method : 'mapWithEscape', left : 1 });
  namespaceEach({ method : 'map', left : 1 });

  /* - */

  function namespaceEach( env )
  {
    env.namespace = 'props';
    caseEach( env );
    env.namespace = 'map';
    caseEach( env );
    env.namespace = 'aux';
    caseEach( env );
    env.namespace = 'object';
    caseEach( env );
    env.namespace = 'container';
    caseEach( env );
  }

  /* - */

  function caseEach( _env )
  {
    env = _env;

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst <> src`;
    var src = { a : 1, b : 2, c : 3 };
    var dst = { d : 4 }
    var got = _[ env.namespace ][ env.method ]( dst, src, f1 );
    test.true( got === dst );
    var exp = { d : 4, a : 11, b : 2, c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 };
    test.identical( src, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === null`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( null, src, f1 );
    test.true( _[ env.namespace ].is( got ) );
    test.true( got !== src );
    var exp = { a : 11, b : 2, c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 }
    test.identical( src, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === src`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( src, src, f1 );
    test.true( got === src );
    var exp = { a : 11, b : 2, c : 13 }
    test.identical( got, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === self`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( _.self, src, f1 );
    test.true( got === src );
    var exp = { a : 11, b : 2, c : 13 }
    test.identical( got, exp );

    /* */

  }

  /* - */

  function f1( e, k, c, src )
  {
    if( e === 2 )
    return undefined;
    return e + 10;
  }

  /* - */

}

mapCommonReturningUndefined.rapidity = -1;
mapCommonReturningUndefined.timeOut = 60000;

//

function filterMapCommonEscaping( test )
{
  let env;

  namespaceEach({ method : 'filterWithoutEscapeLeft', left : 1, escaping : 0 });
  namespaceEach({ method : 'filterWithoutEscapeRight', left : 0, escaping : 0 });
  namespaceEach({ method : 'filterWithoutEscape', left : 1, escaping : 0 });
  namespaceEach({ method : 'filterWithEscapeLeft', left : 1, escaping : 1 });
  namespaceEach({ method : 'filterWithEscapeRight', left : 0, escaping : 1 });
  namespaceEach({ method : 'filterWithEscape', left : 1, escaping : 1 });
  namespaceEach({ method : 'filter', left : 1, escaping : 0 });

  namespaceEach({ method : 'mapWithoutEscapeLeft', left : 1, escaping : 0 });
  namespaceEach({ method : 'mapWithoutEscapeRight', left : 0, escaping : 0 });
  namespaceEach({ method : 'mapWithoutEscape', left : 1, escaping : 0 });
  namespaceEach({ method : 'mapWithEscapeLeft', left : 1, escaping : 1 });
  namespaceEach({ method : 'mapWithEscapeRight', left : 0, escaping : 1 });
  namespaceEach({ method : 'mapWithEscape', left : 1, escaping : 1 });
  namespaceEach({ method : 'map', left : 1, escaping : 0 });

  /* - */

  function namespaceEach( env )
  {
    env.namespace = 'props';
    caseEach( env );
    env.namespace = 'map';
    caseEach( env );
    env.namespace = 'aux';
    caseEach( env );
    env.namespace = 'object';
    caseEach( env );
    env.namespace = 'container';
    caseEach( env );
  }

  /* - */

  function caseEach( _env )
  {
    env = _env;

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst <> src`;
    var src = { a : 1, b : 2, c : 3 };
    var dst = { d : 4 }
    var got = _[ env.namespace ][ env.method ]( dst, src, f1 );
    test.true( got === dst );
    var exp = { d : 4, a : 11, b : escape( undefined ), c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 };
    test.identical( src, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === null`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( null, src, f1 );
    test.true( _[ env.namespace ].is( got ) );
    test.true( got !== src );
    var exp = { a : 11, b : escape( undefined ), c : 13 }
    test.identical( got, exp );
    var exp = { a : 1, b : 2, c : 3 }
    test.identical( src, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === src`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( src, src, f1 );
    test.true( got === src );
    var exp = { a : 11, b : escape( undefined ), c : 13 }
    test.identical( got, exp );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, pass, dst === self`;
    var src = { a : 1, b : 2, c : 3 };
    var got = _[ env.namespace ][ env.method ]( _.self, src, f1 );
    test.true( got === src );
    var exp = { a : 11, b : escape( undefined ), c : 13 }
    test.identical( got, exp );

    /* */

  }

  /* - */

  function escape( src )
  {
    if( env.escaping )
    return src;
    return _.escape.left( src );
  }

  /* - */

  function f1( e, k, c, src )
  {
    if( e === 2 )
    return _.escape.left( undefined );
    return e + 10;
  }

  /* - */

}

filterMapCommonEscaping.timeOut = 60000;
filterMapCommonEscaping.rapidity = -1;

// --
// define test suite
// --

const Proto =
{

  name : 'Tools.Props.l0.l3',
  silencing : 1,

  tests :
  {

    dichotomy,
    exportStringDiagnosticShallow,
    identicalShallow,

    onlyImplicit,

    aptLeft,
    aptRight,

    filterMapCommonPass,
    filterCommonDropping,
    mapCommonReturningUndefined,
    filterMapCommonEscaping,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
