( function _l0_l3_Container_test_s()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( '../Include3.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
// exporter
// --

function exportStringDiagnosticShallow( test )
{

  test.case = 'unroll';
  var src = _.unroll.make([ 2, 3, 4 ]);
  var expected = '{- Array.unroll with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make();
  var expected = '{- ArgumentsArray with 0 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var expected = '{- ArgumentsArray with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'array';
  var src = [ 2, 3, 4 ];
  var expected = '{- Array with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var expected = '{- countableConstructorPolluted.countable with 2 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var expected = '{- countableConstructorPolluted.countable.constructible with 2 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = `object countable - empty, non-vector`;
  var src = __.diagnostic.objectMake({ elements : [], countable : 1, new : 0 });
  var expected = '{- Map.polluted.countable with 0 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = `object countable - non empty, non-vector`;
  var src = __.diagnostic.objectMake({ elements : [ '1', '2', '3' ], countable : 1, new : 0 });
  var expected = '{- Map.polluted.countable with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Global & GlobalReal';
  var src = _global_;
  var expected = '{- Aux.polluted.prototyped with ';
  test.true( _.strHas( _.container.exportStringDiagnosticShallow( src ), expected ) );

  test.case = 'Global & GlobalDerived';
  var src = Object.create( _global_ );
  var expected = '{- Aux.polluted.prototyped with ';
  test.true( _.strHas( _.container.exportStringDiagnosticShallow( src ), expected ) );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  Object.setPrototypeOf( src, { b : 2 } )
  var expected = '{- Aux.polluted.prototyped with 2 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted with `exportString` method';
  var src = { a : 1 };
  var proto =
  {
    b : 2,
    exportString : () => 'Custom string transformation'
  }
  Object.setPrototypeOf( src, proto )
  var expected = '{- Aux.polluted.prototyped with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  var expected = '{- Map.pure with 0 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with `exportString` method';
  var src = Object.create( null );
  src.exportString = () => 'Custom string transformation'
  var expected = '{- Map.pure with 1 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var expected = '{- Map.pure with 2 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src = {};
  var expected = '{- Map.polluted with 0 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src = { a : 1, b : 2, c : 3 };
  var expected = '{- Map.polluted with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'HashMap';
  var src = new HashMap();
  var expected = '{- HashMap with 0 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'HashMap with 2 elems';
  var src = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  var expected = '{- HashMap with 2 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Set & SetLike';
  var src = new Set();
  var expected = '{- Set with 0 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Set with 3 elems';
  var src = new Set([ 1, 2, 3 ]);
  var expected = '{- Set with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var expected = '{- BufferNode with 3 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var expected = '{- I8x with 20 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferBytes & BufferTyped';
  var src = new U8x( 20 );
  var expected = '{- U8x with 20 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'escape';
  var src = _.escape.make( 1 );
  var expected = '{- Escape.countable.constructible with 1 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'interval & BufferTyped';
  var src = new F32x( 2 );
  var expected = '{- F32x with 2 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  test.case = 'pair';
  var src = _.pair.make();
  var expected = '{- Array with 2 elements -}';
  test.identical( _.container.exportStringDiagnosticShallow( src ), expected );

  /* - */

  function routine () {}

}

// --
// container interface
// --

function lengthOf( test )
{
  test.case = 'number';
  var src = 1;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'bool & boolLike & fuzzy';
  var src = true;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'boolLike & number & fuzzyLike';
  var src = 1;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'fuzzy';
  var src = _.maybe;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'bigint';
  var src = 10n;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'str & regexpLike';
  var src = 'str';
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'regexp & objectLike & constructible & constructibleLike';
  var src = /hello/g;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make();
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'unroll';
  var src = _.unroll.make([ 2, 3, 4 ]);
  test.identical( _.container.lengthOf( src ), 3 );

  test.case = 'array';
  var src = [ 2, 3, 4 ];
  test.identical( _.container.lengthOf( src ), 3 );

  test.case = 'long & longLike';
  var src = _.long.make([ 1, 2 ]);
  test.identical( _.container.lengthOf( src ), 2 );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  test.identical( _.container.lengthOf( src ), 2 );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  test.identical( _.container.lengthOf( src ), 2 );

  // test.case = 'Global & GlobalReal';
  // var src = global;
  // // test.identical( _.container.lengthOf( src ), 46 ); /* Dmytro : utility Testing uses garbage collector, direct call of nodejs uses not */
  // test.identical( _.container.lengthOf( src ), 46 + ( global.gc ? 1 : 0 ) );
  //
  // test.case = 'Global & GlobalDerived';
  // var src = Object.create( global );
  // // test.identical( _.container.lengthOf( src ), 46 ); /* Dmytro : utility Testing uses garbage collector, direct call of nodejs uses not */
  // test.identical( _.container.lengthOf( src ), 46 + ( global.gc ? 1 : 0 ) );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src = { [ Symbol.iterator ] : 1 };
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  Object.setPrototypeOf( src, { b : 2 } )
  test.identical( _.container.lengthOf( src ), 2 );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src = {};
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'HashMap';
  var src = new HashMap();
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'Set & SetLike';
  var src = new Set();
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  test.identical( _.container.lengthOf( src ), 3 );

  test.case = 'BufferRaw';
  var src = new BufferRaw( 'str' );
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'BufferRawShared';
  var src = new BufferRawShared( 'str' );
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  test.identical( _.container.lengthOf( src ), 20 );

  test.case = 'BufferView';
  var src = new BufferView( new BufferRaw( 20 ) )
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'BufferBytes & BufferTyped';
  var src = new U8x( 20 );
  test.identical( _.container.lengthOf( src ), 20 );

  test.case = 'err';
  var src = _.err( 'error' );
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'escape';
  var src = _.escape.make( 1 );
  test.identical( _.container.lengthOf( src ), 1 );

  test.case = 'interval & BufferTyped';
  var src = new F32x( 2 );
  test.identical( _.container.lengthOf( src ), 2 );

  test.case = 'pair';
  var src = _.pair.make();
  test.identical( _.container.lengthOf( src ), 2 );

  test.case = 'path & str';
  var src = '/a/b/';
  test.identical( _.container.lengthOf( src ), 0 );

  // test.case = 'propertyTransformer & filter';
  // var src = _.props.condition[ 'dstAndSrcOwn' ];
  // test.identical( _.container.lengthOf( src ), 0 );
  //
  // test.case = 'propertyTransformer & mapper';
  // var src = _.props.mapper[ 'assigning' ];
  // test.identical( _.container.lengthOf( src ), 1 );

  test.case = 'routine & routineLike';
  var src = routine;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'timer';
  var src = _.time._begin( Infinity );
  test.identical( _.container.lengthOf( src ), 9 );
  _.time.cancel( src );

  test.case = 'date & objectLike';
  var src = new Date();
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'countable object';
  var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
  test.identical( _.container.lengthOf( src ), 3 );

  test.case = 'non-countable object';
  var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 0 });
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'null';
  var src = null;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'undefined';
  var src = undefined;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'Symbol null';
  var src = _.null;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'Symbol undefined';
  var src = _.undefined;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'Symbol Nothing';
  var src = _.nothing;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'primitive';
  var src = 5;
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'Symbol';
  var src = Symbol( 'a' );
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'ConsequenceLike & promiseLike & promise';
  var src = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'stream';
  var src = require( 'stream' ).Readable();
  test.identical( _.container.lengthOf( src ), 0 );

  // test.case = 'console';
  // var src = console;
  // test.identical( _.container.lengthOf( src ), 24 );
  //
  // test.case = 'printerLike';
  // var src = _global.logger;
  // test.identical( _.container.lengthOf( src ), 0 );

  test.case = 'process';
  var src = process;
  test.identical( _.container.lengthOf( src ), 0 );

  /* - */

  function routine () {}
}

lengthOf.timeOut = 10000;

// --
// container interface
// --

function elementWithCardinal( test ) /* xxx : types that cause error marked with - 'else _.assert( 0 );' and commented */
{

  /* */

  test.case = 'map';
  var src = { a : 1, b : 2 };
  debugger;
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  debugger;
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 'b', true ] );
  var got = _.container.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'hashMap';
  var src = new HashMap();
  src.set( 'a', 1 );
  src.set( 'b', 2 );
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 'b', true ] );
  var got = _.container.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'array';
  var src = [ 1, 2 ];
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 1, true ] );
  var got = _.container.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'set';
  var src = new Set([ 1, 2 ]);
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 1, true ] );
  var got = _.container.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'escape';
  var src = new _.Escape( 'abc' );
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 'abc', 0, true ] );
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.container.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );
  var got5 = _.container.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'unroll';
  var src = _.unroll.make([ 1, 2, 3 ]);
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );
  var got5 = _.container.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'long & longLike';
  var src = _.long.make([ 1, 2, 3 ]);
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );
  var got5 = _.container.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ '1', 0, true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ '10', 1, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  var got4 = _.container.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ '1', 0, true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ '10', 1, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  var got4 = _.container.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'Global & GlobalReal';
  // var src = global;
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( got, 1 );

  /* else _.assert( 0 ); */
  // test.case = 'Global & GlobalDerived';
  // var src = Object.create( global );
  // var got = _.container.elementWithCardinal( src, 1 );
  // test.identical( _.object.isBasic( got ), true );

  /* else _.assert( 0 ); */
  // test.case = 'Object & ObjectLike & Container & ContainerLike';
  // var src = { [ Symbol.iterator ] : 1, a : 1 };
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( got, [ 0, 1 ] );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 1, false ] );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  src.a = 1;
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 1, false ] );

  test.case = 'HashMap';
  var objRef = { a : 1 };
  var src = new HashMap([ [ 'a', 1 ], [ true, false ], [ objRef, { a : 2 } ] ]);
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ false, true, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ { 'a' : 2 }, { 'a' : 1 }, true ] );
  var got4 = _.container.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'Set & SetLike';
  var objRef = { a : 1 };
  var src = new Set([ 'a', 1, true, objRef ]);
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 'a', 0, true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ 1, 1, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ true, 2, true ] );
  var got4 = _.container.elementWithCardinal( src, 3 );
  test.identical( got4, [ { 'a' : 1 }, 3, true ] );
  var got5 = _.container.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 115, 0, true ] );
  var got = _.container.elementWithCardinal( src, 3 );
  test.identical( got, [ undefined, 3, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'BufferRaw';
  // var src = new BufferRaw( 10 );
  // var got = _.container.elementWithCardinal( src, 1 );
  // test.identical( got, 10 );

  /* else _.assert( 0 ); */
  // test.case = 'BufferRawShared';
  // var src = new BufferRawShared( 15 );
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( got, 15 );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 0, 0, true ] );
  var got = _.container.elementWithCardinal( src, 19 );
  test.identical( got, [ 0, 19, true ] );
  var got = _.container.elementWithCardinal( src, 20 );
  test.identical( got, [ undefined, 20, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'BufferView';
  // var src = new BufferView( new BufferRaw( 20 ) )
  // var got = _.container.elementWithCardinal( src, 1 );
  // test.identical( got, 20 );

  /* else _.assert( 0 ); */
  // test.case = 'err';
  // var src = _.err( 'error' );
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( _.strIs( got ), true );

  test.case = 'pair';
  var src = _.pair.make([ 1, 2 ]);
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithCardinal( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'propertyTransformer & filter';
  // var src = _.props.condition[ 'dstAndSrcOwn' ];
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( got, { 'propertyCondition' : true, 'propertyTransformer' : true, 'functor' : true } );

  /* else _.assert( 0 ); */
  // test.case = 'propertyTransformer & mapper';
  // var src = _.props.mapper[ 'assigning' ];
  // var got = _.container.elementWithCardinal( src, 1 );
  // test.identical( got, { 'propertyMapper' : true, 'propertyTransformer' : true, 'functor' : true } );

  /* else _.assert( 0 ); */
  // test.case = 'routine & routineLike';
  // var src = routine;
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( got, 'routine' );

  test.case = 'timer';
  var src = _.time._begin( Infinity );
  var got = _.container.elementWithCardinal( src, 0 );
  test.identical( got, [ undefined, 'onTime', true ] );
  var got = _.container.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 'onCancel', true ] );
  var got = _.container.elementWithCardinal( src, 100 );
  test.identical( got, [ undefined, 100, false ] );
  _.time.cancel( src );

  /* else _.assert( 0 ); */
  // test.case = 'date & objectLike';
  // var src = new Date( 1000 );
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( got, true );

  /* else _.assert( 0 ); */
  // test.case = 'stream';
  // var src = require( 'stream' ).Readable();
  // var got = _.container.elementWithCardinal( src, 1 );
  // test.identical( got, true );

  /* else _.assert( 0 ); */
  // test.case = 'process';
  // var src = process;
  // var got = _.container.elementWithCardinal( src, 0 );
  // test.identical( got, undefined );

  test.case = 'key < 0';
  var src = { a : 1, b : 2 };
  var got = _.container.elementWithCardinal( src, -1 );
  test.identical( got, [ undefined, -1, false ] );

  test.case = 'number';
  var src = { a : 1, b : 2 };
  var got = _.container.elementWithCardinal( 1, 0 );
  test.identical( got, [ undefined, 0, false ] );

  test.case = 'string';
  var src = { a : 1, b : 2 };
  var got = _.container.elementWithCardinal( 'abc', 0 );
  test.identical( got, [ undefined, 0, false ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.container.elementWithCardinal() );

  test.case = 'to many args'
  test.shouldThrowErrorSync( () => _.container.elementWithCardinal( [ 1, 2, 3 ], 1, 2 ) );

  // test.case = 'container = number'
  // test.shouldThrowErrorSync( () => _.container.elementWithCardinal( 1, 0 ) );
  //
  // test.case = 'container = string'
  // test.shouldThrowErrorSync( () => _.container.elementWithCardinal( 'abc', 0 ) );

  /* - */

  // function _iterate()
  // {
  //
  //   let iterator = Object.create( null );
  //   iterator.next = next;
  //   iterator.index = 0;
  //   iterator.instance = this;
  //   return iterator;
  //
  //   function next()
  //   {
  //     let result = Object.create( null );
  //     result.done = this.index === this.instance.elements.length;
  //     if( result.done )
  //     return result;
  //     result.value = this.instance.elements[ this.index ];
  //     this.index += 1;
  //     return result;
  //   }
  //
  // }

  /* */

  function routine () {}

  /* */

  function Obj1( o )
  {
    _.props.extend( this, o );
    return this;
  }

}

//

function elementWithKeyArgImplicit( test )
{

  test.case = 'object';
  var src = new Obj1({});
  var got = _.container.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'map.polluted';
  var src = {};
  var got = _.container.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'map.pure';
  var src = Object.create( null );
  var got = _.container.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'buffer';
  var src = new F32x([ 1, 2, 3 ]);
  var got = _.container.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var got = _.container.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'array';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.container.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'countable';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.container.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  /* - */

  // function _iterate()
  // {
  //
  //   let iterator = Object.create( null );
  //   iterator.next = next;
  //   iterator.index = 0;
  //   iterator.instance = this;
  //   return iterator;
  //
  //   function next()
  //   {
  //     let result = Object.create( null );
  //     result.done = this.index === this.instance.elements.length;
  //     if( result.done )
  //     return result;
  //     result.value = this.instance.elements[ this.index ];
  //     this.index += 1;
  //     return result;
  //   }
  //
  // }

  /* */

  function routine () {}

  /* */

  function Obj1( o )
  {
    _.props.extend( this, o );
    return this;
  }

}

//

function elementWithKey( test )
{

  test.case = 'string';
  var src = 'abc';
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );

  test.case = 'number';
  var src = 100;
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ undefined, 0, false ] );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );

  var got2 = _.container.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'unroll';
  var src = _.unroll.make([ 1, 2, 3 ]);
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'long & longLike';
  var src = _.long.make([ 1, 2, 3 ]);
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.container.elementWithKey( src, 'elements' );
  // test.identical( got, [ [ '1', '10' ], 'elements', true ] );
  test.identical( got, [ undefined, 'elements', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'vector & vectorLike wit 3 elems';
  var src = __.diagnostic.objectMake({ element1 : '1', element2 : 1, countable : 1, length : 2 });
  var got = _.container.elementWithKey( src, 'element1' );
  // test.identical( got, [ '1', 'element1', true ] );
  test.identical( got, [ undefined, 'element1', false ] );
  var got2 = _.container.elementWithKey( src, 'element2' );
  // test.identical( got2, [ 1, 'element2', true ] );
  test.identical( got2, [ undefined, 'element2', false ] );
  var got3 = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got3, [ undefined, 'non-exists', false ] );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.container.elementWithKey( src, 'elements' );
  // test.identical( got, [ [ '1', '10' ], 'elements', true ] );
  test.identical( got, [ undefined, 'elements', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'countable, empty key';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  src[ '' ] = 'empty';
  var got = _.container.elementWithKey( src, '' );
  test.identical( got, [ undefined, '', false ] );
  // test.identical( got, [ 'empty', '', true ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ '1', 0, true ] );

  test.case = 'buffer';
  var src = new F32x([ 1, 2, 3 ]);
  src[ '' ] = 'empty';
  test.identical( src.length, 3 );
  var got = _.container.elementWithKey( src, '' );
  test.identical( got, [ undefined, '', false ] );
  var got = _.container.elementWithKey( src, 'length' );
  test.identical( got, [ undefined, 'length', false ] );
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );

  test.case = 'Global & GlobalReal';
  var src = global;
  var got = _.container.elementWithKey( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Global & GlobalDerived';
  var src = Object.create( global );
  var got = _.container.elementWithKey( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src = { [ Symbol.iterator ] : 1, a : 1 };
  var got = _.container.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  var got = _.container.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  src.a = 1;
  var got = _.container.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'HashMap';
  var objRef = { a : 1 };
  var src = new HashMap([ [ 'a', 1 ], [ true, false ], [ objRef, { a : 2 } ] ]);
  var got = _.container.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got2 = _.container.elementWithKey( src, true );
  test.identical( got2, [ false, true, true ] );
  var got3 = _.container.elementWithKey( src, objRef );
  test.identical( got3, [ { 'a' : 2 }, objRef, true ] );
  var got4 = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got4, [ undefined, 'non-exists', false ] );

  test.case = 'Set & SetLike';
  var objRef = { a : 1 };
  var src = new Set([ 'a', 1, true, objRef ]);
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  // var got = _.container.elementWithKey( src, 0 );
  // test.identical( got, [ 'a', 0, true ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.container.elementWithKey( src, 1 );
  test.identical( got, [ 1, 1, true ] );
  var got = _.container.elementWithKey( src, 'a' );
  test.identical( got, [ 'a', 'a', true ] );
  var got2 = _.container.elementWithKey( src, 1 );
  test.identical( got2, [ 1, 1, true ] );
  // var got3 = _.container.elementWithKey( src, 2 );
  // test.identical( got3, [ true, 2, true ] );
  var got3 = _.container.elementWithKey( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  // var got4 = _.container.elementWithKey( src, 3 );
  // test.identical( got4, [ objRef, 3, true ] );
  // var got4 = _.container.elementWithKey( src,  );
  // var got4 = _.container.elementWithKey( src, undefined );
  // test.identical( got4, [ undefined, 3, true ] );
  var got4 = _.container.elementWithKey( src, undefined );
  test.identical( got4, [ undefined, undefined, false ] );
  var got4 = _.container.elementWithKey( src, 4 );
  test.identical( got4, [ undefined, 4, false ] );
  var got5 = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got5, [ undefined, 'non-exists', false ] );

  test.case = 'Set with undefined';
  var src = new Set([ undefined, 1, 2, 3 ]);
  var got = _.container.elementWithKey( src, undefined );
  test.identical( got, [ undefined, undefined, true ] );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var got = _.container.elementWithKey( src, '0' );
  // test.identical( got, [ 115, '0', true ] );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRaw';
  var src = new BufferRaw( 10 );
  test.true( !_.countable.is( src ) );
  var got = _.container.elementWithKey( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRawShared';
  var src = new BufferRawShared( 15 );
  test.true( !_.countable.is( src ) );
  var got = _.container.elementWithKey( src, 'byteLength' );
  // test.identical( got, [ 15, 'byteLength', true ] );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ 0, 0, true ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferView';
  var src = new BufferView( new BufferRaw( 20 ) )
  var got = _.container.elementWithKey( src, 'byteLength' );
  // test.identical( got, [ 20, 'byteLength', true ] );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'err';
  var src = _.err( 'error' );
  var got = _.container.elementWithKey( src, 'originalMessage' );
  test.identical( got[ 0 ], undefined );
  test.identical( got[ 1 ], 'originalMessage' );
  test.identical( got[ 2 ], false );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'escape';
  var src = _.escape.make( 1 );
  var got = _.container.elementWithKey( src, 'val' );
  // test.identical( got, [ 1, 'val', true ] );
  test.identical( got, [ undefined, 'val', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'pair';
  var src = _.pair.make([ 1, 2 ]);
  var got = _.container.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got = _.container.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'propertyTransformer & filter';
  var src = _.props.condition[ 'dstAndSrcOwn' ];
  var got = _.container.elementWithKey( src, 'group' );
  test.identical( got, [ undefined, 'group', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  // test.case = 'propertyTransformer & mapper';
  // var src = _.props.mapper[ 'assigning' ];
  // var got = _.container.elementWithKey( src, 'group' );
  // test.identical( got, [ undefined, 'group', false ] );
  // var got = _.container.elementWithKey( src, 'non-exists' );
  // test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'routine & routineLike';
  var src = routine;
  var got = _.container.elementWithKey( src, 'name' );
  test.identical( got, [ undefined, 'name', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'timer';
  var src = _.time._begin( Infinity );
  var got = _.container.elementWithKey( src, 'type' );
  test.identical( got, [ 'delay', 'type', true ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );
  _.time.cancel( src );

  test.case = 'date & objectLike';
  var src = new Date( 1000 );
  var got = _.container.elementWithKey( src, 'getTime' );
  test.identical( got[ 0 ], undefined );
  test.identical( got[ 1 ], 'getTime' );
  test.identical( got[ 2 ], false );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'stream';
  var src = require( 'stream' ).Readable();
  var got = _.container.elementWithKey( src, 'readable' );
  test.identical( got, [ undefined, 'readable', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'process';
  var src = process;
  var got = _.container.elementWithKey( src, 'cwd' );
  test.identical( got, [ undefined, 'cwd', false ] );
  var got = _.container.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.container.elementWithKey() );

  test.case = '1 arg'
  test.shouldThrowErrorSync( () => _.container.elementWithKey( [] ) );

  test.case = 'too many args'
  test.shouldThrowErrorSync( () => _.container.elementWithKey( {}, 'a', 'b' ) );

  /* - */

  // function _iterate()
  // {
  //
  //   let iterator = Object.create( null );
  //   iterator.next = next;
  //   iterator.index = 0;
  //   iterator.instance = this;
  //   return iterator;
  //
  //   function next()
  //   {
  //     let result = Object.create( null );
  //     result.done = this.index === this.instance.elements.length;
  //     if( result.done )
  //     return result;
  //     result.value = this.instance.elements[ this.index ];
  //     this.index += 1;
  //     return result;
  //   }
  //
  // }

  /* */

  function routine () {}

  /* */

  function Obj1( o )
  {
    _.props.extend( this, o );
    return this;
  }

}

//

function elementWithImplicitArgImplicit( test )
{

  /* */

  test.case = 'prototype object';
  var src = new Obj1({});
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype map.polluted';
  var src = {};
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.prototype, _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype map.pure';
  var src = Object.create( null );
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ null, _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype buffer';
  var src = new F32x([ 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype array';
  var src = [ 1, 2, 3 ];
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype arguments array';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.prototype, _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype countable';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  /* - */

  test.case = 'constructor object';
  var src = new Obj1({});
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object.getPrototypeOf( src ).constructor, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor map.polluted';
  var src = {};
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor map.pure';
  var src = Object.create( null );
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ null, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor buffer';
  var src = new F32x([ 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object.getPrototypeOf( src ).constructor, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor array';
  var src = [ 1, 2, 3 ];
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object.getPrototypeOf( src ).constructor, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor arguments array';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor countable';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ src.constructor, _.props.implicit.constructor, true ];
  test.identical( got, expected );
  test.true( _.routineIs( expected.constructor ) );

  /* - */

  function routine () {}

  /* */

  function Obj1( o )
  {
    _.props.extend( this, o );
    return this;
  }

}

//

function elementWithImplicit( test )
{

  /* */

  test.case = 'number';
  var src = 13;
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.container.elementWithImplicit( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  test.identical( got, [ Number.prototype, _.props.implicit.prototype, true ] );
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  test.identical( got, [ Number, _.props.implicit.constructor, true ] );

  /* */

  test.case = 'undefined';
  var src = undefined;
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.container.elementWithImplicit( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  test.identical( got, [ undefined, _.props.implicit.prototype, false ] );
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  test.identical( got, [ undefined, _.props.implicit.constructor, false ] );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.container.elementWithImplicit( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.container.elementWithImplicit( src, _.props.implicit.prototype );
  test.identical( got, [ undefined, _.props.implicit.prototype, false ] );
  var got = _.container.elementWithImplicit( src, _.props.implicit.constructor );
  test.identical( got, [ undefined, _.props.implicit.constructor, false ] );

  /* */

  test.case = 'string';
  var src = 'abc';
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );

  test.case = 'number';
  var src = 100;
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ undefined, 0, false ] );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );

  var got2 = _.container.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'unroll';
  var src = _.unroll.make([ 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'long & longLike';
  var src = _.long.make([ 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.container.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.container.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.container.elementWithImplicit( src, 'elements' );
  test.identical( got, [ undefined, 'elements', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'vector & vectorLike wit 3 elems';
  var src = __.diagnostic.objectMake({ element1 : '1', element2 : 1, countable : 1, length : 2 });
  var got = _.container.elementWithImplicit( src, 'element1' );
  test.identical( got, [ undefined, 'element1', false ] );
  var got2 = _.container.elementWithImplicit( src, 'element2' );
  test.identical( got2, [ undefined, 'element2', false ] );
  var got3 = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got3, [ undefined, 'non-exists', false ] );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.container.elementWithImplicit( src, 'elements' );
  test.identical( got, [ undefined, 'elements', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'countable, empty key';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  src[ '' ] = 'empty';
  var got = _.container.elementWithImplicit( src, '' );
  test.identical( got, [ undefined, '', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ '1', 0, true ] );

  test.case = 'buffer';
  var src = new F32x([ 1, 2, 3 ]);
  src[ '' ] = 'empty';
  test.identical( src.length, 3 );
  var got = _.container.elementWithImplicit( src, '' );
  test.identical( got, [ undefined, '', false ] );
  var got = _.container.elementWithImplicit( src, 'length' );
  test.identical( got, [ undefined, 'length', false ] );
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );

  test.case = 'Global & GlobalReal';
  var src = global;
  var got = _.container.elementWithImplicit( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Global & GlobalDerived';
  var src = Object.create( global );
  var got = _.container.elementWithImplicit( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src = { [ Symbol.iterator ] : 1, a : 1 };
  var got = _.container.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  var got = _.container.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  src.a = 1;
  var got = _.container.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'HashMap';
  var objRef = { a : 1 };
  var src = new HashMap([ [ 'a', 1 ], [ true, false ], [ objRef, { a : 2 } ] ]);
  var got = _.container.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got2 = _.container.elementWithImplicit( src, true );
  test.identical( got2, [ false, true, true ] );
  var got3 = _.container.elementWithImplicit( src, objRef );
  test.identical( got3, [ { 'a' : 2 }, objRef, true ] );
  var got4 = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got4, [ undefined, 'non-exists', false ] );

  test.case = 'Set & SetLike';
  var objRef = { a : 1 };
  var src = new Set([ 'a', 1, true, objRef ]);
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.container.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.container.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.container.elementWithImplicit( src, 1 );
  test.identical( got, [ 1, 1, true ] );
  var got = _.container.elementWithImplicit( src, 'a' );
  test.identical( got, [ 'a', 'a', true ] );
  var got2 = _.container.elementWithImplicit( src, 1 );
  test.identical( got2, [ 1, 1, true ] );
  var got3 = _.container.elementWithImplicit( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  var got4 = _.container.elementWithImplicit( src, undefined );
  test.identical( got4, [ undefined, undefined, false ] );
  var got4 = _.container.elementWithImplicit( src, 4 );
  test.identical( got4, [ undefined, 4, false ] );
  var got5 = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got5, [ undefined, 'non-exists', false ] );

  test.case = 'Set with undefined';
  var src = new Set([ undefined, 1, 2, 3 ]);
  var got = _.container.elementWithImplicit( src, undefined );
  test.identical( got, [ undefined, undefined, true ] );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRaw';
  var src = new BufferRaw( 10 );
  var got = _.container.elementWithImplicit( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRawShared';
  var src = new BufferRawShared( 15 );
  var got = _.container.elementWithImplicit( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ 0, 0, true ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferView';
  var src = new BufferView( new BufferRaw( 20 ) )
  var got = _.container.elementWithImplicit( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'err';
  var src = _.err( 'error' );
  var got = _.container.elementWithImplicit( src, 'originalMessage' );
  test.identical( got, [ undefined, 'originalMessage', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'escape';
  var src = _.escape.make( 1 );
  var got = _.container.elementWithImplicit( src, 'val' );
  test.identical( got, [ undefined, 'val', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'pair';
  var src = _.pair.make([ 1, 2 ]);
  var got = _.container.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.container.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got = _.container.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.container.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'propertyTransformer & filter';
  var src = _.props.condition[ 'dstAndSrcOwn' ];
  var got = _.container.elementWithImplicit( src, 'group' );
  test.identical( got, [ undefined, 'group', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'routine & routineLike';
  var src = routine;
  var got = _.container.elementWithImplicit( src, 'name' );
  test.identical( got, [ undefined, 'name', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'timer';
  var src = _.time._begin( Infinity );
  var got = _.container.elementWithImplicit( src, 'type' );
  test.identical( got, [ 'delay', 'type', true ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );
  _.time.cancel( src );

  test.case = 'date & objectLike';
  var src = new Date( 1000 );
  var got = _.container.elementWithImplicit( src, 'getTime' );
  test.identical( got, [ undefined, 'getTime', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'stream';
  var src = require( 'stream' ).Readable();
  var got = _.container.elementWithImplicit( src, 'readable' );
  test.identical( got, [ undefined, 'readable', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'process';
  var src = process;
  var got = _.container.elementWithImplicit( src, 'cwd' );
  test.identical( got, [ undefined, 'cwd', false ] );
  var got = _.container.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = `object countable - non empty, non-vector, not the same array`;
  var src = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 });
  test.identical( _.container.elementWithImplicit( src, 0 ), [ '1', 0, true ] );
  test.identical( _.container.elementWithImplicit( src, 2 ), [ '3', 2, true ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.container.elementWithImplicit() );

  test.case = 'to many args'
  test.shouldThrowErrorSync( () => _.container.elementWithImplicit( {}, 'a', 'b' ) );

  /* - */

  function routine () {}

  /* */

  function Obj1( o )
  {
    _.props.extend( this, o );
    return this;
  }

}

//

/* qqq : rewrite and extend */
function empty( test )
{

  test.case = 'empty array';
  var dst = [];
  var got = _.container.empty( dst );
  var exp = [];
  test.identical( got, exp );
  test.true( got === dst );

  test.case = 'filled array';
  var dst = [ 1, null, undefined, 'str', '', false, {}, [], [ 1 ], { a : 1 } ];
  var got = _.container.empty( dst );
  var exp = [];
  test.identical( got, exp );
  test.true( got === dst );

  test.case = 'empty unroll';
  var dst = _.unroll.make( [] );
  var got = _.container.empty( dst );
  var exp = _.unroll.make( [] );
  test.identical( got, exp );
  test.true( got === dst );

  test.case = 'filled unroll';
  var dst = _.unroll.make( [ 1, null, undefined, 'str', '', false, {}, [], [ 1 ], { a : 1 } ] );
  var got = _.container.empty( dst );
  var exp = _.unroll.make( [] );
  test.identical( got, exp );
  test.true( got === dst );

  /* */

  test.case = 'empty Set';
  var dst = new Set();
  var got = _.container.empty( dst );
  var exp = new Set();
  test.identical( got, exp );
  test.true( got === dst );

  test.case = 'filled Set';
  var dst = new Set( [ 1, null, undefined, 'str', '', false, {}, [], [ 1 ], { a : 1 } ] );
  var got = _.container.empty( dst );
  var exp = new Set();
  test.identical( got, exp );
  test.true( got === dst );

  /* */

  test.case = 'empty Map';
  var dst = new HashMap();
  var got = _.container.empty( dst );
  var exp = new HashMap();
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );
  test.true( got === dst );

  test.case = 'filled Map';
  var dst = new HashMap( [ [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ 'str', 'str' ], [ '', '' ], [ false, false ], [ {}, {} ], [ [], [] ] ] );
  var got = _.container.empty( dst );
  var exp = new HashMap();
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );
  test.true( got === dst );

  /* */

  test.case = 'empty simple map';
  var dst = {};
  var got = _.container.empty( dst );
  var exp = {};
  test.identical( got, exp );
  test.true( got === dst );

  test.case = 'filled simple map';
  var dst = { '1' : 1, 'null' : null, 'str' : 'str', '' : '', 'false' : false };
  var got = _.container.empty( dst );
  var exp = {};
  test.identical( got, exp );
  test.true( got === dst );

  test.case = 'empty pure map';
  var dst = Object.create( null );
  var got = _.container.empty( dst );
  var exp = Object.create( null );
  test.identical( got, exp );
  test.true( got === dst );

  test.case = 'filled simple map';
  var dst = Object.create( null );
  dst[ '1' ] = 1;
  dst[ 'null' ] = null;
  dst[ 'undefined' ] = undefined;
  var got = _.container.empty( dst );
  var exp = Object.create( null );
  test.identical( got, exp );
  test.true( got === dst );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'wrong type of dst';
  test.shouldThrowErrorSync( () => _.container.empty() );
  test.shouldThrowErrorSync( () => _.container.empty( 'wrong' ) );
  test.shouldThrowErrorSync( () => _.container.empty( 1 ) );
  function Constr()
  {
    this.x = 1;
    return this
  };
  test.shouldThrowErrorSync( () => _.container.empty( new Constr() ) );

  test.case = 'not resizable longs';
  test.shouldThrowErrorSync( () => _.container.empty( _.argumentsArray.make( [] ) ) );
  test.shouldThrowErrorSync( () => _.container.empty( new U8x() ) );
  test.shouldThrowErrorSync( () => _.container.empty( new F64x() ) );

  test.case = 'dst is WeakMap';
  test.shouldThrowErrorSync( () => _.container.empty( new WeakMap() ) );
}

// --
// define test suite
// --

const Proto =
{

  name : 'Tools.Container.l0.l3',
  silencing : 1,

  tests :
  {

    // exporter

    exportStringDiagnosticShallow,

    // container interface

    lengthOf,

    elementWithCardinal,
    elementWithKeyArgImplicit,
    elementWithKey,
    elementWithImplicitArgImplicit,
    elementWithImplicit,

    empty,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
