( function _l0_l3_Entity_test_s()
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

//--
// exporter
//--

function exportStringDiagnosticShallow( test )
{

  test.case = 'unroll';
  var src = _.unroll.make([ 2, 3, 4 ]);
  var expected = '{- Array.unroll with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make();
  var expected = '{- ArgumentsArray with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'number';
  var src = 1;
  var expected = '1';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'bool & boolLike & fuzzy';
  var src = true;
  var expected = 'true';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'boolLike & number & fuzzyLike';
  var src = 0;
  var expected = '0';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'fuzzy';
  var src = _.maybe;
  var expected = '{- Symbol maybe -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'bigint';
  var src = 10n;
  var expected = '10n';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'str & regexpLike';
  var src = 'str';
  var expected = 'str';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'regexp & objectLike & constructible & constructibleLike';
  var src = /hello/g;
  var expected = '/hello/g';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make();
  var expected = '{- ArgumentsArray with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var expected = '{- ArgumentsArray with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  /* l5 */
  // test.case = 'unroll';
  // var src = _.unroll.make([ 2, 3, 4 ]);
  // var expected = '{- Array.unroll with 3 elements -}';
  // test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'array';
  var src = [ 2, 3, 4 ];
  var expected = '{- Array with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var expected = '{- countableConstructorPolluted.countable with 2 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var expected = '{- countableConstructorPolluted.countable.constructible with 2 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = `object countable - empty, non-vector`;
  var src = __.diagnostic.objectMake({ new : 0, elements : [], countable : 1 } );
  var expected = '{- Map.polluted.countable with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = `object countable - non empty, non-vector`;
  var src = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var expected = '{- Map.polluted.countable with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Global & GlobalReal';
  var src = _global_;
  var expected = '{- Aux.polluted.prototyped with ';
  test.true( _.strHas( _.entity.exportStringDiagnosticShallow( src ), expected ) );

  test.case = 'Global & GlobalDerived';
  var src = Object.create( _global_ );
  var expected = '{- Aux.polluted.prototyped with ';
  test.true( _.strHas( _.entity.exportStringDiagnosticShallow( src ), expected ) );

  test.case = 'Object & ObjectLike & Container & ContainerLike'; /* qqq for junior : bad : this is aux! lack of Object & Countable cases | aaa : Added. */
  var src = { [ Symbol.iterator ] : 1 };
  var expected = '{- Map.polluted with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & Container & ContainerLike with `exportString` method';
  var src =
  {
    [ Symbol.iterator ] : 1,
    exportString : () => 'Custom string transformation'
  };
  var expected = '{- Map.polluted with 1 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  Object.setPrototypeOf( src, { b : 2 } )
  var expected = '{- Aux.polluted.prototyped with 2 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted with `exportString` method';
  var src = { a : 1 };
  var proto =
  {
    b : 2,
    exportString : () => 'Custom string transformation'
  }
  Object.setPrototypeOf( src, proto )
  var expected = '{- Aux.polluted.prototyped with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  var expected = '{- Map.pure with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with `exportString` method';
  var src = Object.create( null );
  src.exportString = () => 'Custom string transformation'
  var expected = '{- Map.pure with 1 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var expected = '{- Map.pure with 2 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src = {};
  var expected = '{- Map.polluted with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src = { a : 1, b : 2, c : 3 };
  var expected = '{- Map.polluted with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'HashMap';
  var src = new HashMap();
  var expected = '{- HashMap with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'HashMap with 2 elems';
  var src = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  var expected = '{- HashMap with 2 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Set & SetLike';
  var src = new Set();
  var expected = '{- Set with 0 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Set with 3 elems';
  var src = new Set([ 1, 2, 3 ]);
  var expected = '{- Set with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var expected = '{- BufferNode with 3 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferRaw';
  var src = new BufferRaw( 'str' );
  var expected = '{- BufferRaw -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferRawShared';
  var src = new BufferRawShared( 'str' );
  var expected = '{- BufferRawShared -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var expected = '{- I8x with 20 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferView';
  var src = new BufferView( new BufferRaw( 20 ) );
  var expected = '{- DataView.constructible -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'BufferBytes & BufferTyped';
  var src = new U8x( 20 );
  var expected = '{- U8x with 20 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'err';
  var src = _.err( 'error' );
  var expected = '{- Error.constructible -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'escape';
  var src = _.escape.make( 1 );
  var expected = '{- Escape.countable.constructible with 1 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'interval & BufferTyped';
  var src = new F32x( 2 );
  var expected = '{- F32x with 2 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'pair';
  var src = _.pair.make();
  var expected = '{- Array with 2 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'path & str';
  var src = '/a/b/';
  var expected = '/a/b/';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  /* l7 */
  // test.case = 'propertyTransformer & filter';
  // var src = _.props.condition[ 'dstAndSrcOwn' ];
  // var expected = '{- routine dstAndSrcOwn -}';
  // test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  /* l7 */
  // test.case = 'propertyTransformer & mapper';
  // var src = _.props.mapper[ 'assigning' ];
  // var expected = '{- routine assigning -}';
  // test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'routine & routineLike';
  var src = routine;
  var expected = '{- routine routine -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  /* l5 */
  test.case = 'timer';
  var src = _.time._begin( Infinity );
  var expected = '{- Map.pure with 9 elements -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );
  _.time.cancel( src );

  test.case = 'date & objectLike';
  var src = new Date( '2021-02-19T11:26:42.840Z' );
  var expected = '2021-02-19T11:26:42.840Z';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'null';
  var src = null;
  var expected = 'null';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'undefined';
  var src = undefined;
  var expected = 'undefined';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Symbol null';
  var src = _.null;
  var expected = '{- Symbol null -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Symbol undefined';
  var src = _.undefined;
  var expected = '{- Symbol undefined -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Symbol Nothing';
  var src = _.nothing;
  var expected = '{- Symbol nothing -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'primitive';
  var src = 5;
  var expected = '5';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'Symbol';
  var src = Symbol( 'a' );
  var expected = '{- Symbol a -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'ConsequenceLike & promiseLike & promise';
  var src = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  var expected = '{- Promise.constructible -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'stream';
  var src = require( 'stream' ).Readable();
  var expected = '{- Readable.constructible -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  // test.case = 'console';
  // var src = console;
  // var expected = '{- Console.constructible with 1 elements -}';
  // test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'printerLike';
  var src = new __.Logger();
  var expected = '{- wLoggerTop.constructible -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'printerLike with output to console';
  var src = new __.Logger({ output : console });
  var expected = '{- wLoggerTop.constructible -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

  test.case = 'process';
  var src = process;
  var expected = '{- process.constructible -}';
  test.identical( _.entity.exportStringDiagnosticShallow( src ), expected );

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
  //
  // /* */
  //
  // function countableConstructorPolluted( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
  // }

  function routine () {}

}

// //
//
// function exportStringCodeShallow( test )
// {
//
//   test.case = 'number';
//   var src = 1;
//   var expected = '1';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'bool & boolLike & fuzzy';
//   var src = true;
//   var expected = 'true';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'boolLike & number & fuzzyLike';
//   var src = 0;
//   var expected = '0';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'fuzzy';
//   var src = _.maybe;
//   var expected = '{- Symbol maybe -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'bigint';
//   var src = 10n;
//   var expected = '10n';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'str & regexpLike';
//   var src = 'str';
//   var expected = 'str';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'regexp & objectLike & constructible & constructibleLike';
//   var src = /hello/g;
//   var expected = '/hello/g';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'ArgumentsArray & arrayLike';
//   var src = _.argumentsArray.make();
//   var expected = '{- ArgumentsArray with 0 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'ArgumentsArray & arrayLike with 3 elems';
//   var src = _.argumentsArray.make([ 1, 2, 3 ]);
//   var expected = '{- ArgumentsArray with 3 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'unroll';
//   var src = _.unroll.make([ 2, 3, 4 ]);
//   var expected = '{- Array.unroll with 3 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'array';
//   var src = [ 2, 3, 4 ];
//   var expected = '{- Array with 3 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'long & longLike';
//   var src = _.long.make([ 1, 2 ]);
//   var expected = '{- Array with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'vector & vectorLike';
//   var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1, length : 2 });
//   var expected = '{- countableConstructorPolluted.countable with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'countable & countableLike';
//   var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
//   var expected = '{- countableConstructorPolluted.countable.constructible with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = `object countable - empty, non-vector`;
//   var src = __.diagnostic.objectMake({ new : 0, elements : [], countable : 1 } );
//   var expected = '{- Map.polluted with 8 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = `object countable - non empty, non-vector`;
//   var src = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
//   var expected = '{- Map.polluted with 8 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Global & GlobalReal';
//   var src = global;
//   var expected = '{- Aux.polluted.prototyped with ';
//   test.true( _.strHas( _.entity.exportStringCodeShallow( src ), expected ) );
//
//   test.case = 'Global & GlobalDerived';
//   var src = Object.create( global );
//   var expected = '{- Aux.polluted.prototyped with ';
//   test.true( _.strHas( _.entity.exportStringCodeShallow( src ), expected ) );
//
//   test.case = 'Object & ObjectLike & Container & ContainerLike'; /* qqq for junior : bad : this is aux! lack of Object & Countable cases | aaa : Added. */
//   var src = { [ Symbol.iterator ] : 1 };
//   var expected = '{- Map.polluted with 0 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & Container & ContainerLike with `exportString` method';
//   var src =
//   {
//     [ Symbol.iterator ] : 1,
//     exportString : () => 'Custom string transformation'
//   };
//   var expected = '{- Map.polluted with 1 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
//   var src = { a : 1 };
//   Object.setPrototypeOf( src, { b : 2 } )
//   var expected = '{- Aux.polluted.prototyped with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted with `exportString` method';
//   var src = { a : 1 };
//   var proto =
//   {
//     b : 2,
//     exportString : () => 'Custom string transformation'
//   }
//   Object.setPrototypeOf( src, proto )
//   var expected = '{- Aux.polluted.prototyped with 3 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
//   var src = Object.create( null );
//   var expected = '{- Map.pure with 0 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & auxiliary & map & mapPure with `exportString` method';
//   var src = Object.create( null );
//   src.exportString = () => 'Custom string transformation'
//   var expected = '{- Map.pure with 1 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = 2;
//   var expected = '{- Map.pure with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
//   var src = {};
//   var expected = '{- Map.polluted with 0 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
//   var src = { a : 1, b : 2, c : 3 };
//   var expected = '{- Map.polluted with 3 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'HashMap';
//   var src = new HashMap();
//   var expected = '{- HashMap with 0 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'HashMap with 2 elems';
//   var src = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
//   var expected = '{- HashMap with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Set & SetLike';
//   var src = new Set();
//   var expected = '{- Set with 0 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Set with 3 elems';
//   var src = new Set([ 1, 2, 3 ]);
//   var expected = '{- Set with 3 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'BufferNode';
//   var src = BufferNode.from( 'str' );
//   var expected = '{- BufferNode with 3 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw( 'str' );
//   var expected = '{- BufferRaw -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'BufferRawShared';
//   var src = new BufferRawShared( 'str' );
//   var expected = '{- BufferRawShared -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'BufferTyped';
//   var src = new I8x( 20 );
//   var expected = '{- I8x with 20 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'BufferView';
//   var src = new BufferView( new BufferRaw( 20 ) );
//   var expected = '{- DataView.constructible -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'BufferBytes & BufferTyped';
//   var src = new U8x( 20 );
//   var expected = '{- U8x with 20 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'err';
//   var src = _.err( 'error' );
//   var expected = '{- Error.constructible -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'escape';
//   var src = _.escape.make( 1 );
//   var expected = '{- Escape 1 -}'; /* xxx2 : qqq : bad */
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'interval & BufferTyped';
//   var src = new F32x( 2 );
//   var expected = '{- F32x with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'pair';
//   var src = _.pair.make();
//   var expected = '{- Array with 2 elements -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'path & str';
//   var src = '/a/b/';
//   var expected = '/a/b/';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'routine & routineLike';
//   var src = routine;
//   var expected = '{- routine routine -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   // qqq : xxx : uncomment
//   // test.case = 'timer';
//   // var src = _.time._begin( Infinity );
//   // var expected = '{- Map.pure with 9 elements -}';
//   // test.identical( _.entity.exportStringCodeShallow( src ), expected );
//   // _.time.cancel( src );
//
//   test.case = 'date & objectLike';
//   var src = new Date( '2021-02-19T11:26:42.840Z' );
//   var expected = '2021-02-19T11:26:42.840Z';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'null';
//   var src = null;
//   var expected = 'null';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'undefined';
//   var src = undefined;
//   var expected = 'undefined';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Symbol null';
//   var src = _.null;
//   var expected = '{- Symbol null -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Symbol undefined';
//   var src = _.undefined;
//   var expected = '{- Symbol undefined -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Symbol Nothing';
//   var src = _.nothing;
//   var expected = '{- Symbol nothing -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'primitive';
//   var src = 5;
//   var expected = '5';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'Symbol';
//   var src = Symbol( 'a' );
//   var expected = '{- Symbol a -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'ConsequenceLike & promiseLike & promise';
//   var src = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
//   var expected = '{- Promise.constructible -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'stream';
//   var src = require( 'stream' ).Readable();
//   var expected = '{- Readable.constructible -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'printerLike';
//   var src = new __.Logger();
//   var expected = '{- wLoggerTop.constructible -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'printerLike with output to console';
//   var src = new __.Logger({ output : console });
//   var expected = '{- wLoggerTop.constructible -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   test.case = 'process';
//   var src = process;
//   var expected = '{- process.constructible -}';
//   test.identical( _.entity.exportStringCodeShallow( src ), expected );
//
//   /* - */
//
//   // function _iterate()
//   // {
//   //
//   //   let iterator = Object.create( null );
//   //   iterator.next = next;
//   //   iterator.index = 0;
//   //   iterator.instance = this;
//   //   return iterator;
//   //
//   //   function next()
//   //   {
//   //     let result = Object.create( null );
//   //     result.done = this.index === this.instance.elements.length;
//   //     if( result.done )
//   //     return result;
//   //     result.value = this.instance.elements[ this.index ];
//   //     this.index += 1;
//   //     return result;
//   //   }
//   //
//   // }
//   //
//   // /* */
//   //
//   // function countableConstructor( o )
//   // {
//   //   return countableMake( this, o );
//   // }
//   //
//   // /* */
//   //
//   // function countableMake( dst, o )
//   // {
//   //   if( dst === null )
//   //   dst = Object.create( null );
//   //   _.props.extend( dst, o );
//   //   if( o.countable )
//   //   dst[ Symbol.iterator ] = _iterate;
//   //   return dst;
//   // }
//
//   function routine () {}
//
// }

//

function strPrimitive( test ) /* qqq for junior : extend */
{

  test.case = 'undefined';
  var src = undefined;
  var expected = undefined;
  var got = _.entity.strPrimitive( src );
  test.identical( got, expected );

  test.case = 'null';
  var src = null;
  var expected = undefined;
  var got = _.entity.strPrimitive( src );
  test.identical( got, expected );

  test.case = 'number';
  var src = 13;
  var expected = '13';
  var got = _.entity.strPrimitive( src );
  test.identical( got, expected );

  test.case = 'boolean';
  var src = false;
  var expected = 'false';
  var got = _.entity.strPrimitive( src );
  test.identical( got, expected );

  test.case = 'string';
  var src = 'abc';
  var expected = 'abc';
  var got = _.entity.strPrimitive( src );
  test.identical( got, expected );

}

//

function strTypeWithTraitsBasic( test )
{

  test.case = 'undefined';
  var src = undefined;
  var expected = 'Undefined';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'null';
  var src = null;
  var expected = 'Null';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'number int';
  var src = 13;
  var expected = 'Number';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'number NaN';
  var src = 13;
  var expected = 'Number';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'number Infinity';
  var src = Infinity;
  var expected = 'Number';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'boolean';
  var src = false;
  var expected = 'Boolean';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'string';
  var src = 'abc';
  var expected = 'String';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var expected = 'Array';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'ArgumentsArray';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var expected = 'ArgumentsArray';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'routine';
  var src = () => {};
  var expected = 'Routine';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'symbol';
  var src = Symbol( 'id' );
  var expected = 'Symbol';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'map';
  var src = new HashMap();
  var expected = 'HashMap';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'set';
  var src = new Set();
  var expected = 'Set';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'polluted map';
  var src = { a : 1, b : 2, c : 3 };
  var expected = 'Map.polluted';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.a = 1;
  var expected = 'Map.pure';
  var got = _.entity.strTypeWithTraits( src );
  test.identical( got, expected );

}

//

function strTypeWithoutTraitsBasic( test )
{

  test.case = 'undefined';
  var src = undefined;
  var expected = 'Undefined';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'null';
  var src = null;
  var expected = 'Null';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'number int';
  var src = 13;
  var expected = 'Number';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'number NaN';
  var src = 13;
  var expected = 'Number';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'number Infinity';
  var src = Infinity;
  var expected = 'Number';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'boolean';
  var src = false;
  var expected = 'Boolean';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'string';
  var src = 'abc';
  var expected = 'String';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var expected = 'Array';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'ArgumentsArray';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var expected = 'ArgumentsArray';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'routine';
  var src = () => {};
  var expected = 'Routine';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'symbol';
  var src = Symbol( 'id' );
  var expected = 'Symbol';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'map';
  var src = new HashMap();
  var expected = 'HashMap';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'set';
  var src = new Set();
  var expected = 'Set';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'polluted map';
  var src = { a : 1, b : 2, c : 3 };
  var expected = 'Map';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.a = 1;
  var expected = 'Map';
  var got = _.entity.strTypeWithoutTraits( src );
  test.identical( got, expected );

  /* qqq for Rahul : normalize and consolidate strTypeWithTraitsBasic and strTypeWithoutTraitsBasic. use the test case as example */
  test.case = 'countable map';
  var src = __.diagnostic.objectMake({ new : 0, elements : [], countable : 1 } );
  test.identical( _.entity.strTypeWithoutTraits( src ), 'Map' );
  test.identical( _.entity.strTypeWithTraits( src ), 'Map.polluted.countable' );

}

//

function strTypeWithTraitsGeneratedObject( test )
{

  let sets =
  {
    countable : [ 0, 1 ],
    pure : [ 0, 1 ],
    withOwnConstructor : [ 0, 1 ],
    withConstructor : [ 0, 1 ],
    new : [ 0, 1 ],
  };
  let samples = __.permutation.eachSample({ sets });

  for( let env of samples )
  eachCase( env );

  function eachCase( env )
  {
    var handled = false;
    test.case = `${toStr( env )}`;
    var src = _.diagnostic.objectMake( { elements : [ '1', '10' ], ... env } );

    if( env.new )
    {
      if( _.mapIsPure( src ) )
      test.identical( _.entity.strTypeWithTraits( src ), 'Map.pure' );
      else if( _.mapIsPolluted( src ) )
      test.identical( _.entity.strTypeWithTraits( src ), 'Map.polluted' );
      else if( _.aux.isPure( src ) && _.aux.isPrototyped( src ) )
      {
        if( _.countable.is( src ) )
        test.identical( _.entity.strTypeWithTraits( src ), 'Aux.pure.prototyped.countable' );
        else
        test.identical( _.entity.strTypeWithTraits( src ), 'Aux.pure.prototyped' );
      }
      else if( _.aux.isPolluted( src ) && _.aux.isPrototyped( src ) )
      {
        if( _.countable.is( src ) )
        test.identical( _.entity.strTypeWithTraits( src ), 'Aux.polluted.prototyped.countable' );
        else
        test.identical( _.entity.strTypeWithTraits( src ), 'Aux.polluted.prototyped' );
      }
      else if( env.withConstructor && env.countable && env.pure )
      test.identical( _.entity.strTypeWithTraits( src ), 'countableConstructorPure.countable.constructible' );
      else if( env.withConstructor && env.countable && !env.pure )
      test.identical( _.entity.strTypeWithTraits( src ), 'countableConstructorPolluted.countable.constructible' );
      else if( env.withConstructor && env.pure )
      test.identical( _.entity.strTypeWithTraits( src ), 'countableConstructorPure.constructible' );
      else if( env.withConstructor && !env.pure )
      test.identical( _.entity.strTypeWithTraits( src ), 'countableConstructorPolluted.constructible' );
      else if( env.countable )
      test.identical( _.entity.strTypeWithTraits( src ), 'Object.countable' );
      else
      test.identical( _.entity.strTypeWithTraits( src ), 'Object' );

    }
    else
    {

      if( _.mapIsPure( src ) )
      {
        if( _.countable.is( src ) )
        test.identical( _.entity.strTypeWithTraits( src ), 'Map.pure.countable' );
        else
        test.identical( _.entity.strTypeWithTraits( src ), 'Map.pure' );
      }
      else if( _.mapIsPolluted( src ) )
      {
        if( _.countable.is( src ) )
        test.identical( _.entity.strTypeWithTraits( src ), 'Map.polluted.countable' );
        else
        test.identical( _.entity.strTypeWithTraits( src ), 'Map.polluted' );
      }
      else if( _.aux.isPure( src ) && _.aux.isPrototyped( src ) )
      test.identical( _.entity.strTypeWithTraits( src ), 'Aux.pure.prototyped' );
      else if( _.aux.isPolluted( src ) && _.aux.isPrototyped( src ) )
      test.identical( _.entity.strTypeWithTraits( src ), 'Aux.polluted.prototyped' );
      else if( env.countable )
      test.identical( _.entity.strTypeWithTraits( src ), 'Object.countable' );
      else
      test.identical( _.entity.strTypeWithTraits( src ), 'Object' );

    }

    /* - */

  }

  function toStr( src )
  {
    return __.entity.exportStringSolo( src );
  }

}

//

function strTypeWithoutTraitsGeneratedObject( test )
{

  let sets =
  {
    countable : [ 0, 1 ],
    pure : [ 0, 1 ],
    withOwnConstructor : [ 0, 1 ],
    withConstructor : [ 0, 1 ],
    new : [ 0, 1 ],
  };
  let samples = __.permutation.eachSample({ sets });

  for( let env of samples )
  eachCase( env );

  function eachCase( env )
  {
    var handled = false;
    test.case = `${toStr( env )}`;
    var src = _.diagnostic.objectMake( { elements : [ '1', '10' ], ... env } );

    if( env.new )
    {

      if( _.mapIs( src ) )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'Map' );
      else if( _.aux.is( src ) )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'Aux' );
      else if( env.withConstructor && env.countable && env.pure )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'countableConstructorPure' );
      else if( env.withConstructor && env.countable && !env.pure )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'countableConstructorPolluted' );
      else if( env.withConstructor && env.pure )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'countableConstructorPure' );
      else if( env.withConstructor && !env.pure )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'countableConstructorPolluted' );
      else
      test.identical( _.entity.strTypeWithoutTraits( src ), 'Object' );

    }
    else
    {

      if( _.mapIs( src ) )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'Map' );
      else if( _.aux.is( src ) )
      test.identical( _.entity.strTypeWithoutTraits( src ), 'Aux' );
      else
      test.identical( _.entity.strTypeWithoutTraits( src ), 'Object' );

    }

    /* - */

  }

  function toStr( src )
  {
    return __.entity.exportStringSolo( src );
  }

}

// --
// equaler
// --

function eqShallowBasic( test )
{
  /* */

  test.case = `Null - Undefined`;
  var src1 = null;
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Boolean - Boolean`;
  var src1 = true;
  var src2 = false;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Boolean - Not Boolean`;
  var src1 = true;
  var src2 = false;
  test.true( !_.entity.identicalShallow( src1, 1 ) );
  test.true( !_.entity.identicalShallow( src2, 0 ) );
  test.true( _.entity.equivalentShallow( src1, 1 ) );
  test.true( _.entity.equivalentShallow( src2, 0 ) );

  /* */

  test.case = `Number - Number`;
  var src1 = 1;
  var src2 = 1 + 1e-15;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );

  test.case = `NaN - Infinity`;
  var src1 = NaN;
  var src2 = Infinity;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `+0 - -0`
  var src1 = +0;
  var src2 = -0;
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Number - String`;
  var src1 = 1;
  var src2 = '1';
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Infinity - Null`;
  var src1 = Infinity;
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `NaN - Undefined`;
  var src1 = NaN;
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `NaN - Null`;
  var src1 = NaN;
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Empty Array`;
  var src1 = 3;
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Empty Map`;
  var src1 = 3;
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Non-Empty Array`;
  var src1 = 3;
  var src2 = [ 3 ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Non-Empty Map`;
  var src1 = 3;
  var src2 = { a : 3 };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Date`;
  var src1 = 3;
  var src2 = new Date();
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Typed Array`;
  var src1 = 3;
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - RegExp`;
  var src1 = 3;
  var src2 = /abc/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Routine`;
  var src1 = 3;
  function src2 (){};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Number - Routine returning Number`;
  var src1 = 3;
  var src2 = (function(){return 3;})();
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty String - String`;
  var src1 = '';
  var src2 = 'abc';
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - String`;
  var src1 = 'ab';
  var src2 = 'c';
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - String Object`;
  var src1 = 'a';
  var src2 = String( src1 );
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty String - undefined`;
  var src1 = '';
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Null`;
  var src1 = '';
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - NaN`;
  var src1 = '';
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Number`;
  var src1 = '';
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Empty Array`;
  var src1 = '';
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Empty Map`;
  var src1 = '';
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Non-Empty Array`;
  var src1 = '';
  var src2 = [ '' ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Non-Empty Map`;
  var src1 = '';
  var src2 = { a : '' };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Typed Array`;
  var src1 = '';
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - RegExp`;
  var src1 = '';
  var src2 = /(?:)/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Routine`;
  var src1 = '';
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty String - Date`;
  var src1 = '';
  var src2 = new Date( );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Undefined`;
  var src1 = 'abc';
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Null`;
  var src1 = 'abc';
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - NaN`;
  var src1 = 'abc';
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Number`;
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Empty Array`;
  var src1 = 'abc';
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Empty Map`;
  var src1 = 'abc';
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Non-Empty Array`;
  var src1 = 'abc';
  var src2 = [ '' ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Non Empty Map`;
  var src1 = '';
  var src2 = { a : 'abc' };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Typed Array`;
  var src1 = 'abc';
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - RegExp`;
  var src1 = 'abc';
  var src2 = /(?:)/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - Routine`;
  var src1 = 'abc';
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `String - NaN`;
  var src1 = 'abc';
  var src2 = new Date( );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty RegExp - RegExp`;
  var src1 = /(?:)/;
  var src2 = /abc/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - RegExp without flag`;
  var src1 = /abc/;
  var src2 = /abc/iy;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp with flag - RegExp with same flags in different order`;
  var src1 = /abc/iy;
  var src2 = /abc/yi;
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty RegExp - Date`;
  var src1 = /(?:)/;
  var src2 = new Date();
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Undefined`;
  var src1 = /(?:)/;
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Null`;
  var src1 = /(?:)/;
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - NaN`;
  var src1 = /(?:)/;
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Number`;
  var src1 = /(?:)/;
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Empty Array`;
  var src1 = /(?:)/;
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Empty Map`;
  var src1 = /(?:)/;
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Array includes empty RegExp`;
  var src1 = /(?:)/;
  var src2 = [ /(?:)/ ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Map has Empty RegExp`;
  var src1 = /(?:)/;
  var src2 = { a : /(?:)/ };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Typed Array`;
  var src1 = /(?:)/;
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Routine`;
  var src1 = /(?:)/;
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty RegExp - Date`;
  var src1 = /(?:)/;
  var src2 = new Date( );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Date`;
  var src1 = /abc/;
  var src2 = new Date();
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Undefined`;
  var src1 = /abc/;
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Null`;
  var src1 = /abc/;
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - NaN`;
  var src1 = /abc/;
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Number`;
  var src1 = /abc/;
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Empty Array`;
  var src1 = /abc/;
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Empty Map`;
  var src1 = /abc/;
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Array includes RegExp`;
  var src1 = /abc/;
  var src2 = [ /abc/ ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Map has RegExp`;
  var src1 = /abc/;
  var src2 = { a : /abc/ };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Typed Array`;
  var src1 = /abc/;
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `RegExp - Routine`;
  var src1 = /abc/;
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Routine - Routine`;
  function src1(){};
  function src2(){};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Routine - Date`;
  function src1(){};
  var src2 = new Date();
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Undefined`;
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Null`;
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - NaN`;
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Number`;
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Empty Array`;
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Empty Map`;
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Array includes Routine`;
  var src2 = [ src1 ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Map has Routine`;
  var src2 = { a : src1 };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - Typed Array`;
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Routine - RegExp`;
  var src2 = /(?:)/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Date - Date`;
  var src1 = new Date();
  var src2 = new Date();
  src2.setFullYear( 1987 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Date - String`;
  var src1 = new Date();
  var src2 = '1';
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Undefined`;
  var src1 = new Date();
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Null`;
  var src1 = new Date();
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - NaN`;
  var src1 = new Date();
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Number`;
  var src1 = new Date();
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Empty array`;
  var src1 = new Date();
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Empty Map`;
  var src1 = new Date();
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Array includes Date`;
  var src1 = new Date();
  var src2 = [ src1 ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Map has Date`;
  var src1 = new Date();
  var src2 = { a : src1 };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Typed Array`;
  var src1 = new Date();
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Empty RegExp`;
  var src1 = new Date();
  var src2 = /(?:)/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Date - Routine`;
  var src1 = new Date();
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty Array - Empty Array`;
  var src1 = [];
  var src2 = [];
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same arrays having 0 as only element`;
  var src1 = [ 0 ];
  var src2 = [ 0 ];
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same arrays having 1 as only element`;
  var src1 = [ 1 ];
  var src2 = [ 1 ];
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same arrays having undefined as only element`;
  var src1 = [ undefined ];
  var src2 = [ undefined ];
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same arrays having null as only element`;
  var src1 = [ null ];
  var src2 = [ null ];
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Array has null as only element - Array has undefined as only element`;
  var src1 = [ null ];
  var src2 = [ undefined ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Nested arrays having same elements`;
  var src1 = [ [ 1, 2, 3 ] ];
  var src2 = [ [ 1, 2, 3 ] ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Nested arrays having different elements`;
  var src1 = [ [ 1, 2 ] ];
  var src2 = [ [ 1, 2, 3 ] ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty Array - String`;
  var src1 = [];
  var src2 = '1';
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Undefined`;
  var src1 = [];
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Null`;
  var src1 = [];
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - NaN`;
  var src1 = [];
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Number`;
  var src1 = [];
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Empty Map`;
  var src1 = [];
  var src2 = {};
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Map has Empty Array`;
  var src1 = [];
  var src2 = { a : src1 };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Typed Array`;
  var src1 = [];
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - RegExp`;
  var src1 = [];
  var src2 = /(?:)/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Date`;
  var src1 = [];
  var src2 = new Date();
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Array - Routine`;
  var src1 = [];
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty Map - Empty Map`;
  var src1 = {};
  var src2 = {};
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same Maps having 0 as only value`;
  var src1 = { a : 0 };
  var src2 = { a : 0 };
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same Maps having 1 as only value`;
  var src1 = { a : 1 };
  var src2 = { a : 1 };
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same Maps having undefined as only value`;
  var src1 = { a : undefined };
  var src2 = { a : undefined };
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Same Maps having null as only value`;
  var src1 = { a : null };
  var src2 = { a : null };
  test.true( _.entity.identicalShallow( src1, src2 ) );
  test.true( _.entity.identicalShallow( src2, src1 ) );
  test.true( _.entity.equivalentShallow( src1, src2 ) );
  test.true( _.entity.equivalentShallow( src2, src1 ) );

  test.case = `Maps having unequal no of items`;
  var src1 = { a : 1, b : 1 };
  var src2 = { a : 1 };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Nested maps having same items`;
  var src1 = { a : { b : 1 } };
  var src2 = { a : { b : 1 } };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Nested Maps having different items`;
  var src1 = { a : { b : 1, c : 2 } };
  var src2 = { a : { b : 1 } };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  /* */

  test.case = `Empty Map - String`;
  var src1 = {};
  var src2 = '1';
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Undefined`;
  var src1 = {};
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Null`;
  var src1 = {};
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - NaN`;
  var src1 = {};
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Number`;
  var src1 = {};
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Empty Array`;
  var src1 = {};
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Non-Empty Array having 0 as only element`;
  var src1 = {};
  var src2 = [ 0 ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Non-Empty Array having 1 as only element`;
  var src1 = {};
  var src2 = [ 1 ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Non-Empty map holding a reference to empty map as only value`;
  var src1 = {};
  var src2 = { a : src1 };
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Typed Array`;
  var src1 = {};
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Empty RegExp`;
  var src1 = {};
  var src2 = /(?:)/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Date`;
  var src1 = {};
  var src2 = new Date();
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Empty Map - Routine`;
  var src1 = {};
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map - String`;
  var src1 = { a : '1'};
  var src2 = '1';
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map - Undefined`;
  var src1 = { a : undefined };
  var src2 = undefined;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map - Null`;
  var src1 = { a : null };
  var src2 = null;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map - NaN`;
  var src1 = { a : NaN };
  var src2 = NaN;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map - Number`;
  var src1 = { a :  0 };
  var src2 = 0;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map having empty array as only value - Empty Array`;
  var src1 = { a :  [] };
  var src2 = [];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map having non-empty array as only value - Non-Empty Array having 0 as only element`;
  var src1 = { a :  [ 0 ] };
  var src2 = [ 0 ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map having non-empty array as only value - Non-Empty Array having 1 as only element`;
  var src1 = { a :  [ 1 ] };
  var src2 = [ 1 ];
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map having typed array as only value - Typed Array`;
  var src1 = { a :  new F32x( 3 ) };
  var src2 = new F32x( 3 );
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map having RegExp as only value - RegExp`;
  var src1 = { a :  /(?:)/ };
  var src2 = /(?:)/;
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map having date as only value - Date`;
  var src1 = { a : new Date() };
  var src2 = new Date();
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );

  test.case = `Non-Empty Map having routine as only value - Routine`;
  var src1 = { a : function(){} };
  function src2(){}
  test.true( !_.entity.identicalShallow( src1, src2 ) );
  test.true( !_.entity.identicalShallow( src2, src1 ) );
  test.true( !_.entity.equivalentShallow( src1, src2 ) );
  test.true( !_.entity.equivalentShallow( src2, src1 ) );
}

//

function identicalShallowBasic( test )
{
  /* */

  test.case = 'null - undefined';

  var expected = true;
  var got = _.entity.identicalShallow( null, null );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( undefined, undefined );
  test.identical( got, expected );

  var expected = false
  var got = _.entity.identicalShallow( null, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( undefined, null );
  test.identical( got, expected );

  /* */

  test.case = 'number - number';

  var expected = true;
  var got = _.entity.identicalShallow( 1, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 1, 1 + 1e-15 );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( 0, 0 );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( NaN, NaN );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( Infinity, Infinity );
  test.identical( got, expected );

  /* */

  test.case = 'number - not number';

  var expected = false;
  var got = _.entity.identicalShallow( 1, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 0, '0' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 0, '' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 0, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 0, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( NaN, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( Infinity, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, [ 3 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, { a : 3 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 3, function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'String - String';

  var expected = true;
  var got = _.entity.identicalShallow( '', '' );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( 'abc', 'abc' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', 'abc' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', '' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'ab', 'c' );
  test.identical( got, expected );

  /* */

  test.case = 'String - not String';

  var expected = false;
  var got = _.entity.identicalShallow( '', new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', [ '' ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', { a : '' } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '', function(){} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '0', 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( '1', 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', [ 'abc' ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', { abc : 'abc' } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( 'abc', function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'RegExp - RegExp';

  var expected = true;
  var got = _.entity.identicalShallow( /(?:)/, /(?:)/ );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( /abc/, /abc/ );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( /abc/iy, /abc/yi );
  test.identical( got, expected );

  // var expected = true;
  // var got = _.entity.identicalShallow( new RegExp( 'abc' ), /abc/ );
  // test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( /abc/i, /abc/i );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/i, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/i, /abc/yi );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /ab/, /c/ );
  test.identical( got, expected );

  /* */

  test.case = 'RegExp - not RegExp';

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, [ /(?:)/ ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, { a : /(?:)/ } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /(?:)/, function(){} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, [ /abc/ ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, { a : /abc/ } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( /abc/, function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Routine - Routine';

  function func1(){};
  function func2(){};

  var expected = true;
  var got = _.entity.identicalShallow( func1, func1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, func2 );
  test.identical( got, expected );

  /* */

  test.case = 'Routine - not Routine';

  var expected = false;
  var got = _.entity.identicalShallow( func1, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, [ func1 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, { a : func1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( func1, function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Date - Date';

  var expected = true;
  var src1 = new Date();
  var src2 = new Date( src1 );
  var got = _.entity.identicalShallow( src1, src2 );
  test.identical( got, expected );

  var expected = false;
  var src1 = new Date();
  var src2 = new Date();
  src2.setFullYear( 1987 );
  var got = _.entity.identicalShallow( src1, src2 );
  test.identical( got, expected );

  /* */

  test.case = 'Date - not Date';

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), [ new Date() ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), { a : new Date() } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( new Date(), function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Array - Array';

  var expected = true;
  var got = _.entity.identicalShallow( [], [] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( [ 0 ], [ 0 ] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( [ 1 ], [ 1 ] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( [ undefined ], [ undefined ] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( [ null ], [ null ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ [ 1, 2, 3 ] ], [ [ 1, 2, 3 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ [ 1, 2 ] ], [ [ 1, 2, 3 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ [ 1, 2, 3 ] ], [ [ 1, 2 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ [ 1, 2 ] ], [ [ 1 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ [ 1, 3 ] ], [ 1, 3 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ null ], [ undefined ] );
  test.identical( got, expected );

  /* */

  test.case = 'Array - not Array';

  var expected = false;
  var got = _.entity.identicalShallow( [], '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [], function(){} );
  test.identical( got, expected );

  /* */

  var expected = false;
  var got = _.entity.identicalShallow( [ '1' ], '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ undefined ], undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ null ], null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ NaN ], NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ 0 ], 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ 1 ], 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ {} ], {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ new Date() ], new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ new F32x( 3 ) ], new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ /(?:)/ ], /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( [ function(){} ], function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Map - Map';

  var expected = true;
  var got = _.entity.identicalShallow( {}, {} );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( { a : 0 }, { a : 0 } );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( { a : 1 }, { a : 1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : 1, b : 1 }, { a : 1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : 1 }, { a : 1, b : 1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : undefined, b : 1 }, { a : undefined } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : undefined }, { a : undefined, b : 1 } );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( { a : undefined }, { a : undefined } );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.identicalShallow( { a : null }, { a : null } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : { b : 1 } }, { a : { b : 1 } } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : { b : 1 } }, { a : { b : 1, c : 2 } } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : { b : 1, c : 2 } }, { a : { b : 1 } } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : { b : 1, c : 3 } }, { b : 1, c : 3 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : null }, { a : undefined } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : undefined }, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, { a : undefined } );
  test.identical( got, expected );

  /* */

  test.case = 'Map - not Map';

  var expected = false;
  var got = _.entity.identicalShallow( {}, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, [ 0 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, [ 1 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( {}, function(){} );
  test.identical( got, expected );

  /* */

  var expected = false;
  var got = _.entity.identicalShallow( { a : '1' }, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : undefined }, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : null }, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : NaN }, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : 0 }, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : 1 }, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : {} }, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : new Date() }, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : new F32x( 3 ) }, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : /(?:)/ }, /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.identicalShallow( { a : function(){} }, function(){} );
  test.identical( got, expected );

}

//

function eqShallowAllTypes( test )
{
  test.open( 'identical and equivalent' );

  test.case = 'number';
  var src1 = 1;
  var src2 = 1;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'bool & boolLike & fuzzy';
  var src1 = true;
  var src2 = true;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'boolLike & number & fuzzyLike';
  var src1 = 0;
  var src2 = 0;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'fuzzy';
  var src1 = _.maybe;
  var src2 = _.maybe;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'bigint';
  var src1 = 10n;
  var src2 = 10n;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'str & regexpLike';
  var src1 = 'str';
  var src2 = 'str';
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'regexp & objectLike & constructible & constructibleLike';
  var src1 = /hello/g;
  var src2 = /hello/g;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'ArgumentsArray & arrayLike';
  var src1 = _.argumentsArray.make();
  var src2 = _.argumentsArray.make();
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src1 = _.argumentsArray.make([ 1, 2, 3 ]);
  var src2 = _.argumentsArray.make([ 1, 2, 3 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'unroll';
  var src1 = _.unroll.make([ 2, 3, 4 ]);
  var src2 = _.unroll.make([ 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'array';
  var src1 = [ 2, 3, 4 ];
  var src2 = [ 2, 3, 4 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'long & longLike';
  var src1 = _.long.make([ 1, 2 ]);
  var src2 = _.long.make([ 1, 2 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `object countable - non empty, non-vector, not same array`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '4' ], countable : 1 } );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = `object countable - non empty, non-vector, same array`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'vector & vectorLike';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'countable & countableLike';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Global & GlobalReal';
  var src1 = global;
  var src2 = global;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Global & GlobalDerived';
  var src1 = Object.create( global );
  var src2 = Object.create( global );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src1 = { a : 1 };
  Object.setPrototypeOf( src1, { b : 2 } );
  var src2 = { a : 1 };
  Object.setPrototypeOf( src2, { b : 2 } );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src1 = Object.create( null );
  var src2 = Object.create( null );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src1 = Object.create( null );
  src1.a = 1;
  src1.b = 2;
  var src2 = Object.create( null );
  src2.a = 1;
  src2.b = 2;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src1 = {};
  var src2 = {};
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src1 = { a : 1, b : 2, c : 3 };
  var src2 = { a : 1, b : 2, c : 3 };
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'HashMap';
  var src1 = new HashMap();
  var src2 = new HashMap();
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'HashMap with 2 elems';
  var src1 = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  var src2 = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Set & SetLike';
  var src1 = new Set();
  var src2 = new Set();
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Set with 3 elems';
  var src1 = new Set([ 1, 2, 3 ]);
  var src2 = new Set([ 1, 2, 3 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Set with different elements same set';
  var src1 = new Set([ 1, 2, 3, 3, 3 ]);
  var src2 = new Set([ 1, 2, 3 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'BufferNode';
  var src1 = BufferNode.from( 'str' );
  var src2 = BufferNode.from( 'str' );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'BufferRaw';
  var src1 = new BufferRaw( 5 );
  var src2 = new BufferRaw( 5 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'BufferRawShared';
  var src1 = new BufferRawShared( 10 );
  var src2 = new BufferRawShared( 10 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'BufferTyped';
  var src1 = new I8x( 20 );
  var src2 = new I8x( 20 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'BufferView';
  var src1 = new BufferView( new BufferRaw( 20 ) );
  var src2 = new BufferView( new BufferRaw( 20 ) );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'BufferBytes & BufferTyped';
  var src1 = new U8x( 20 );
  var src2 = new U8x( 20 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'escape';
  var src1 = _.escape.make( 1 );
  var src2 = _.escape.make( 1 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'interval & BufferTyped';
  var src1 = new F32x( 2 );
  var src2 = new F32x( 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'pair';
  var src1 = _.pair.make( 1, 2 );
  var src2 = _.pair.make( 1, 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'path & str';
  var src1 = '/a/b/';
  var src2 = '/a/b/';
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'routine & routineLike';
  var src1 = routine;
  var src2 = routine;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'date & objectLike';
  var src1 = new Date( '2021-02-19T11:26:42.840Z' );
  var src2 = new Date( '2021-02-19T11:26:42.840Z' );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'null';
  var src1 = null;
  var src2 = null;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'undefined';
  var src1 = undefined;
  var src2 = undefined;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Symbol null';
  var src1 = _.null;
  var src2 = _.null;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Symbol undefined';
  var src1 = _.undefined;
  var src2 = _.undefined;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Symbol Nothing';
  var src1 = _.nothing;
  var src2 = _.nothing;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'primitive';
  var src1 = 5;
  var src2 = 5;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'stream';
  var src1 = require( 'stream' ).Readable();
  var src2 = src1;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'printerLike';
  var src1 = _global.logger;
  var src2 = _global.logger;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'console';
  var src1 = console;
  var src2 = console;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'printerLike';
  var src1 = _global.logger;
  var src2 = _global.logger;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'process';
  var src1 = process;
  var src2 = process;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src1 = { [ Symbol.iterator ] : 1 };
  var src2 = { [ Symbol.iterator ] : 1 };
  test.identical( _.entity.identicalShallow( src1, src2 ), true );
  test.identical( _.entity.identicalShallow( src2, src1 ), true );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.close( 'identical and equivalent' );

  /* - */

  test.open( 'neither identical nor equivalent' );

  test.case = 'number';
  var src1 = 1;
  var src2 = 2;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'bool & boolLike & fuzzy';
  var src1 = true;
  var src2 = false;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'boolLike & number & fuzzyLike';
  var src1 = 0;
  var src2 = 1;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'fuzzy';
  var src1 = _.maybe;
  var src2 = 0;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'str & regexpLike';
  var src1 = 'str';
  var src2 = 'str2';
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'regexp & objectLike & constructible & constructibleLike';
  var src1 = /hello/g;
  var src2 = /hello/i;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'ArgumentsArray & arrayLike';
  var src1 = _.argumentsArray.make();
  var src2 = _.argumentsArray.make([ 1 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src1 = _.argumentsArray.make([ 1, 2, 3 ]);
  var src2 = _.argumentsArray.make([ 1, 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'unroll';
  var src1 = _.unroll.make([ 2, 3, 4, 5 ]);
  var src2 = _.unroll.make([ 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'array';
  var src1 = [ [ 2 ], 3, 4 ];
  var src2 = [ [ 2 ], 3, 4 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'long & longLike';
  var src1 = _.long.make([ 1, 2, [ 5 ] ]);
  var src2 = _.long.make([ 1, 2, [ 5 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = `object countable - non empty, non-vector, no iterator`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 0 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 0 } );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'vector & vectorLike not same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '2' ], countable : 1, length : 2 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'countable & countableLike not same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '2' ], countable : 1 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src1 = { a : 1 };
  Object.setPrototypeOf( src1, { b : 2 } );
  var src2 = { b : 1 };
  Object.setPrototypeOf( src2, { b : 2 } );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src1 = Object.create( null );
  src1.a = [ 1 ];
  var src2 = Object.create( null );
  src2.a = [ 1 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src1 = Object.create( null );
  src1.a = [ 1, 2, 3 ];
  src1.b = [ 1, 2, 3 ];
  var src2 = Object.create( null );
  src2.a = [ 1, 2, 3 ];
  src2.b = [ 1, 2, 3 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src1 = { a : 1 };
  var src2 = {};
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src1 = { a : { f : 'hello' }, b : 2, c : 3 };
  var src2 = { a : { f : 'hello' }, b : 2, c : 3 };
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'HashMap';
  var src1 = new HashMap();
  var src2 = new HashMap([ [ 'a', 1 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'HashMap with 2 elems';
  var src1 = new HashMap([ [ 'a', [ 3 ] ], [ 'b', 2 ] ]);
  var src2 = new HashMap([ [ 'a', [ 3 ] ], [ 'b', 2 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Set & SetLike';
  var src1 = new Set();
  var src2 = new Set([ 0 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Set with different elements';
  var src1 = new Set([ 1, 2, 3 ]);
  var src2 = new Set([ 1, 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'BufferNode';
  var src1 = BufferNode.from( 'str' );
  var src2 = BufferNode.from( 'str2' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'BufferRaw';
  var src1 = new BufferRaw( 5 );
  var src2 = new BufferRaw( 6 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'BufferRawShared';
  var src1 = new BufferRawShared( 5 );
  var src2 = new BufferRawShared( 6 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'BufferTyped';
  var src1 = new I8x( 20 );
  var src2 = new I8x( 21 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'BufferView';
  var src1 = new BufferView( new BufferRaw( 20 ) );
  var src2 = new BufferView( new BufferRaw( 21 ) );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'BufferBytes & BufferTyped';
  var src1 = new U8x( 20 );
  var src2 = new U8x( 21 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'err same originalMessage';
  var src1 = _.err( 'error' );
  var src2 = _.err( 'error' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'err diff originalMessage';
  var src1 = _.err( 'error' );
  var src2 = _.err( 'error2' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'escape';
  var src1 = _.escape.make( 1 );
  var src2 = _.escape.make( 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'pair';
  var src1 = _.pair.make( 1, 2 );
  var src2 = _.pair.make( 1, 3 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'timer';
  var src1 = _.time._begin( Infinity );
  var src2 = _.time._begin( Infinity );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );
  _.time.cancel( src1 );
  _.time.cancel( src2 );

  test.case = 'path & str';
  var src1 = '/a/b/';
  var src2 = '/a/b/c/';
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'routine & routineLike';
  var src1 = routine;
  var src2 = () => { 'hello' };
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'timer';
  var src1 = _.time._begin( Infinity );
  var src2 = _.time._begin( 100 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );
  _.time.cancel( src1 );
  _.time.cancel( src2 );

  test.case = 'date & objectLike';
  var src1 = new Date( '2021-02-19T11:26:42.840Z' );
  var src2 = new Date( '2020-02-19T11:26:42.840Z' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'primitive';
  var src1 = 5;
  var src2 = 6;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'Symbol';
  var src1 = Symbol( 'a' );
  var src2 = Symbol( 'a' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'ConsequenceLike & promiseLike & promise';
  var src1 = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  var src2 = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.case = 'stream';
  var src1 = require( 'stream' ).Readable();
  var src2 = require( 'stream' ).Readable();
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src2, src1 ), false );

  test.close( 'neither identical nor equivalent' );

  /* */

  test.open( 'equivalent but not identical');

  test.case = 'bigint and int';
  var src1 = 10n;
  var src2 = 10;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = 'BufferTyped';
  var src1 = new F32x( 2 );
  var src2 = new U8x( 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `set and array`;
  var src1 = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
  var src2 = [ 1, 2, 3 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `buffer typed and array`;
  var src1 = new F32x([ 1, 2, 3 ])
  var src2 = [ 1, 2, 3 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `vectorLike and array`;
  var src1 = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `countable and array`;
  var src1 = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `vector and array`;
  var src1 = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `argumentsArray and array`;
  var src1 = _.argumentsArray.make([ '1', '10' ]);
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.case = `argumentsArray and array`;
  var src1 = _.argumentsArray.make([ '1', '10' ]);
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.identicalShallow( src2, src1 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  test.identical( _.entity.equivalentShallow( src2, src1 ), true );

  test.close( 'equivalent but not identical')

  /* */

  function routine () {}

}

//

function identicalShallowAllTypes( test )
{
  test.open( 'identical' );

  test.case = 'number';
  var src1 = 1;
  var src2 = 1;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'bool & boolLike & fuzzy';
  var src1 = true;
  var src2 = true;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'boolLike & number & fuzzyLike';
  var src1 = 0;
  var src2 = 0;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'fuzzy';
  var src1 = _.maybe;
  var src2 = _.maybe;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'bigint';
  var src1 = 10n;
  var src2 = 10n;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'str & regexpLike';
  var src1 = 'str';
  var src2 = 'str';
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'regexp & objectLike & constructible & constructibleLike';
  var src1 = /hello/g;
  var src2 = /hello/g;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'ArgumentsArray & arrayLike';
  var src1 = _.argumentsArray.make();
  var src2 = _.argumentsArray.make();
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src1 = _.argumentsArray.make([ 1, 2, 3 ]);
  var src2 = _.argumentsArray.make([ 1, 2, 3 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'unroll';
  var src1 = _.unroll.make([ 2, 3, 4 ]);
  var src2 = _.unroll.make([ 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'array';
  var src1 = [ 2, 3, 4 ];
  var src2 = [ 2, 3, 4 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'long & longLike';
  var src1 = _.long.make([ 1, 2 ]);
  var src2 = _.long.make([ 1, 2 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = `object countable - non empty, non-vector, not same array`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '4' ], countable : 1 } );
  var got = _.entity.identicalShallow( src1, src2 );
  test.identical( got, false );

  test.case = `object countable - non empty, non-vector, same array`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var got = _.entity.identicalShallow( src1, src2 );
  test.identical( got, true );

  test.case = 'vector & vectorLike';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.entity.identicalShallow( src1, src2 );
  test.identical( got, true );

  test.case = 'countable & countableLike';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.identicalShallow( src1, src2 );
  test.identical( got, true );

  test.case = `object countable - non empty, non-vector, no iterator`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 0 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 0 } );
  var got = _.entity.identicalShallow( src1, src2 );
  test.identical( got, false );

  test.case = 'Global & GlobalReal';
  var src1 = global;
  var src2 = global;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Global & GlobalDerived';
  var src1 = Object.create( global );
  var src2 = Object.create( global );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src1 = { a : 1 };
  Object.setPrototypeOf( src1, { b : 2 } );
  var src2 = { a : 1 };
  Object.setPrototypeOf( src2, { b : 2 } );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src1 = Object.create( null );
  var src2 = Object.create( null );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src1 = Object.create( null );
  src1.a = 1;
  src1.b = 2;
  var src2 = Object.create( null );
  src2.a = 1;
  src2.b = 2;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src1 = {};
  var src2 = {};
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src1 = { a : 1, b : 2, c : 3 };
  var src2 = { a : 1, b : 2, c : 3 };
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'HashMap';
  var src1 = new HashMap();
  var src2 = new HashMap();
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'HashMap with 2 elems';
  var src1 = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  var src2 = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Set & SetLike';
  var src1 = new Set();
  var src2 = new Set();
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Set with 3 elems';
  var src1 = new Set([ 1, 2, 3 ]);
  var src2 = new Set([ 1, 2, 3 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'BufferNode';
  var src1 = BufferNode.from( 'str' );
  var src2 = BufferNode.from( 'str' );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'BufferRaw';
  var src1 = new BufferRaw( 5 );
  var src2 = new BufferRaw( 5 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'BufferRawShared';
  var src1 = new BufferRawShared( 10 );
  var src2 = new BufferRawShared( 10 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'BufferTyped';
  var src1 = new I8x( 20 );
  var src2 = new I8x( 20 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'BufferView';
  var src1 = new BufferView( new BufferRaw( 20 ) );
  var src2 = new BufferView( new BufferRaw( 20 ) );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'BufferBytes & BufferTyped';
  var src1 = new U8x( 20 );
  var src2 = new U8x( 20 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'escape';
  var src1 = _.escape.make( 1 );
  var src2 = _.escape.make( 1 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'interval & BufferTyped';
  var src1 = new F32x( 2 );
  var src2 = new F32x( 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'pair';
  var src1 = _.pair.make( 1, 2 );
  var src2 = _.pair.make( 1, 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'path & str';
  var src1 = '/a/b/';
  var src2 = '/a/b/';
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  // test.case = 'propertyTransformer & filter';
  // var src1 = _.props.condition[ 'dstAndSrcOwn' ];
  // var src2 = _.props.condition[ 'dstAndSrcOwn' ];
  // test.identical( _.entity.identicalShallow( src1, src2 ), true );
  //
  // test.case = 'propertyTransformer & mapper';
  // var src1 = _.props.mapper[ 'assigning' ];
  // var src2 = _.props.mapper[ 'assigning' ];
  // test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'routine & routineLike';
  var src1 = routine;
  var src2 = routine;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'date & objectLike';
  var src1 = new Date( '2021-02-19T11:26:42.840Z' );
  var src2 = new Date( '2021-02-19T11:26:42.840Z' );
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'null';
  var src1 = null;
  var src2 = null;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'undefined';
  var src1 = undefined;
  var src2 = undefined;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Symbol null';
  var src1 = _.null;
  var src2 = _.null;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Symbol undefined';
  var src1 = _.undefined;
  var src2 = _.undefined;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Symbol Nothing';
  var src1 = _.nothing;
  var src2 = _.nothing;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'primitive';
  var src1 = 5;
  var src2 = 5;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'stream';
  var src1 = require( 'stream' ).Readable();
  var src2 = src1;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'printerLike';
  var src1 = _global.logger;
  var src2 = _global.logger;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'console';
  var src1 = console;
  var src2 = console;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'printerLike';
  var src1 = _global.logger;
  var src2 = _global.logger;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'process';
  var src1 = process;
  var src2 = process;
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.close( 'identical' );

  /* - */

  test.open( 'not identical' );

  test.case = 'number';
  var src1 = 1;
  var src2 = 2;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'bool & boolLike & fuzzy';
  var src1 = true;
  var src2 = false;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'boolLike & number & fuzzyLike';
  var src1 = 0;
  var src2 = 1;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'fuzzy';
  var src1 = _.maybe;
  var src2 = 0;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  /* ? */
  test.case = 'bigint';
  var src1 = 10n;
  var src2 = 10;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'str & regexpLike';
  var src1 = 'str';
  var src2 = 'str2';
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'regexp & objectLike & constructible & constructibleLike';
  var src1 = /hello/g;
  var src2 = /hello/i;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'ArgumentsArray & arrayLike';
  var src1 = _.argumentsArray.make();
  var src2 = _.argumentsArray.make([ 1 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src1 = _.argumentsArray.make([ 1, 2, 3 ]);
  var src2 = _.argumentsArray.make([ 1, 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'unroll';
  var src1 = _.unroll.make([ 2, 3, 4, 5 ]);
  var src2 = _.unroll.make([ 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'array';
  var src1 = [ [ 2 ], 3, 4 ];
  var src2 = [ [ 2 ], 3, 4 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'long & longLike';
  var src1 = _.long.make([ 1, 2, [ 5 ] ]);
  var src2 = _.long.make([ 1, 2, [ 5 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'vector & vectorLike not same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '2' ], countable : 1, length : 2 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'countable & countableLike not same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '2' ], countable : 1 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src1 = { [ Symbol.iterator ] : 1 };
  var src2 = { [ Symbol.iterator ] : 1 };
  test.identical( _.entity.identicalShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src1 = { a : 1 };
  Object.setPrototypeOf( src1, { b : 2 } );
  var src2 = { b : 1 };
  Object.setPrototypeOf( src2, { b : 2 } );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src1 = Object.create( null );
  src1.a = [ 1 ];
  var src2 = Object.create( null );
  src2.a = [ 1 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src1 = Object.create( null );
  src1.a = [ 1, 2, 3 ];
  src1.b = [ 1, 2, 3 ];
  var src2 = Object.create( null );
  src2.a = [ 1, 2, 3 ];
  src2.b = [ 1, 2, 3 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src1 = { a : 1 };
  var src2 = {};
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src1 = { a : { f : 'hello' }, b : 2, c : 3 };
  var src2 = { a : { f : 'hello' }, b : 2, c : 3 };
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'HashMap';
  var src1 = new HashMap();
  var src2 = new HashMap([ [ 'a', 1 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'HashMap with 2 elems';
  var src1 = new HashMap([ [ 'a', [ 3 ] ], [ 'b', 2 ] ]);
  var src2 = new HashMap([ [ 'a', [ 3 ] ], [ 'b', 2 ] ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Set & SetLike';
  var src1 = new Set();
  var src2 = new Set([ 0 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Set with 3 elems';
  var src1 = new Set([ 1, 2, 3 ]);
  var src2 = new Set([ 1, 2, 3, 4 ]);
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'BufferNode';
  var src1 = BufferNode.from( 'str' );
  var src2 = BufferNode.from( 'str2' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'BufferRaw';
  var src1 = new BufferRaw( 5 );
  var src2 = new BufferRaw( 6 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'BufferRawShared';
  var src1 = new BufferRawShared( 5 );
  var src2 = new BufferRawShared( 6 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'BufferTyped';
  var src1 = new I8x( 20 );
  var src2 = new I8x( 21 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'BufferView';
  var src1 = new BufferView( new BufferRaw( 20 ) );
  var src2 = new BufferView( new BufferRaw( 21 ) );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'BufferBytes & BufferTyped';
  var src1 = new U8x( 20 );
  var src2 = new U8x( 21 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'err same originalMessage';
  var src1 = _.err( 'error' );
  var src2 = _.err( 'error' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'err diff originalMessage';
  var src1 = _.err( 'error' );
  var src2 = _.err( 'error2' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'escape';
  var src1 = _.escape.make( 1 );
  var src2 = _.escape.make( 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'interval & BufferTyped';
  var src1 = new F32x( 2 );
  var src2 = new U8x( 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'pair';
  var src1 = _.pair.make( 1, 2 );
  var src2 = _.pair.make( 1, 3 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'timer';
  var src1 = _.time._begin( Infinity );
  var src2 = _.time._begin( Infinity );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  _.time.cancel( src1 );
  _.time.cancel( src2 );

  test.case = 'path & str';
  var src1 = '/a/b/';
  var src2 = '/a/b/c/';
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  // test.case = 'propertyTransformer & filter';
  // var src1 = _.props.condition[ 'dstAndSrcOwn' ];
  // var src2 = _.props.condition[ 'dstNotHasOrSrcNotNull' ];
  // test.identical( _.entity.identicalShallow( src1, src2 ), false );
  //
  // test.case = 'propertyTransformer & mapper';
  // var src1 = _.props.mapper[ 'assigning' ];
  // var src2 = _.props.mapper[ 'removing' ];
  // test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'routine & routineLike';
  var src1 = routine;
  var src2 = () => { 'hello' };
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'timer';
  var src1 = _.time._begin( Infinity );
  var src2 = _.time._begin( 100 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  _.time.cancel( src1 );
  _.time.cancel( src2 );

  test.case = 'date & objectLike';
  var src1 = new Date( '2021-02-19T11:26:42.840Z' );
  var src2 = new Date( '2020-02-19T11:26:42.840Z' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'primitive';
  var src1 = 5;
  var src2 = 6;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'Symbol';
  var src1 = Symbol( 'a' );
  var src2 = Symbol( 'a' );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'ConsequenceLike & promiseLike & promise';
  var src1 = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  var src2 = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.case = 'stream';
  var src1 = require( 'stream' ).Readable();
  var src2 = require( 'stream' ).Readable();
  test.identical( _.entity.identicalShallow( src1, src2 ), false );

  test.close( 'not identical' );

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
  //
  // /* */
  //
  // function countableConstructorPolluted( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
  // }

  function routine () {}

}

//

function equivalentShallowBasic( test )
{
  /* */

  test.case = 'null - undefined';

  var expected = true;
  var got = _.entity.equivalentShallow( null, null );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( undefined, undefined );
  test.identical( got, expected );

  var expected = false
  var got = _.entity.equivalentShallow( null, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( undefined, null );
  test.identical( got, expected );

  /* */

  test.case = 'number - number';

  var expected = true;
  var got = _.entity.equivalentShallow( 1, 1 );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( 1, 1 + 1e-15 );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( 0, 0 );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( NaN, NaN );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( Infinity, Infinity );
  test.identical( got, expected );

  /* */

  test.case = 'number - not number';

  var expected = false;
  var got = _.entity.equivalentShallow( 1, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 0, '0' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 0, '' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 0, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 0, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( NaN, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( Infinity, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, [ 3 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, { a : 3 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 3, function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'String - String';

  var expected = true;
  var got = _.entity.equivalentShallow( '', '' );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( 'abc', 'abc' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', 'abc' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', '' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'ab', 'c' );
  test.identical( got, expected );

  /* */

  test.case = 'String - not String';

  var expected = false;
  var got = _.entity.equivalentShallow( '', new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', [ '' ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', { a : '' } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '', function(){} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '0', 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( '1', 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', [ 'abc' ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', { abc : 'abc' } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( 'abc', function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'RegExp - RegExp';

  var expected = true;
  var got = _.entity.equivalentShallow( /(?:)/, /(?:)/ );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( /abc/, /abc/ );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( /abc/iy, /abc/yi );
  test.identical( got, expected );

  // var expected = true;
  // var got = _.entity.equivalentShallow( new RegExp( 'abc' ), /abc/ );
  // test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( /abc/i, /abc/i );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/i, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/i, /abc/yi );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /ab/, /c/ );
  test.identical( got, expected );

  /* */

  test.case = 'RegExp - not RegExp';

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, [ /(?:)/ ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, { a : /(?:)/ } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /(?:)/, function(){} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, [ /abc/ ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, { a : /abc/ } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( /abc/, function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Routine - Routine';

  function func1(){};
  function func2(){};

  var expected = true;
  var got = _.entity.equivalentShallow( func1, func1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, func2 );
  test.identical( got, expected );

  /* */

  test.case = 'Routine - not Routine';

  var expected = false;
  var got = _.entity.equivalentShallow( func1, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, [ func1 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, { a : func1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( func1, function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Date - Date';

  var expected = true;
  var src1 = new Date();
  var src2 = new Date( src1 );
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, expected );

  var expected = false;
  var src1 = new Date();
  var src2 = new Date();
  src2.setFullYear( 1987 );
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, expected );

  /* */

  test.case = 'Date - not Date';

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), [ new Date() ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), { a : new Date() } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), /abc/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( new Date(), function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Array - Array';

  var expected = true;
  var got = _.entity.equivalentShallow( [], [] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( [ 0 ], [ 0 ] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( [ 1 ], [ 1 ] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( [ undefined ], [ undefined ] );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( [ null ], [ null ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ [ 1, 2, 3 ] ], [ [ 1, 2, 3 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ [ 1, 2 ] ], [ [ 1, 2, 3 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ [ 1, 2, 3 ] ], [ [ 1, 2 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ [ 1, 2 ] ], [ [ 1 ] ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ [ 1, 3 ] ], [ 1, 3 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ null ], [ undefined ] );
  test.identical( got, expected );

  /* */

  test.case = 'Array - not Array';

  var expected = false;
  var got = _.entity.equivalentShallow( [], '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [], function(){} );
  test.identical( got, expected );

  /* */

  var expected = false;
  var got = _.entity.equivalentShallow( [ '1' ], '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ undefined ], undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ null ], null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ NaN ], NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ 0 ], 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ 1 ], 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ {} ], {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ new Date() ], new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ new F32x( 3 ) ], new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ /(?:)/ ], /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( [ function(){} ], function(){} );
  test.identical( got, expected );

  /* */

  test.case = 'Map - Map';

  var expected = true;
  var got = _.entity.equivalentShallow( {}, {} );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( { a : 0 }, { a : 0 } );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( { a : 1 }, { a : 1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : 1, b : 1 }, { a : 1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : 1 }, { a : 1, b : 1 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : undefined, b : 1 }, { a : undefined } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : undefined }, { a : undefined, b : 1 } );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( { a : undefined }, { a : undefined } );
  test.identical( got, expected );

  var expected = true;
  var got = _.entity.equivalentShallow( { a : null }, { a : null } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : { b : 1 } }, { a : { b : 1 } } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : { b : 1 } }, { a : { b : 1, c : 2 } } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : { b : 1, c : 2 } }, { a : { b : 1 } } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : { b : 1, c : 3 } }, { b : 1, c : 3 } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : null }, { a : undefined } );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : undefined }, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, { a : undefined } );
  test.identical( got, expected );

  /* */

  test.case = 'Map - not Map';

  var expected = false;
  var got = _.entity.equivalentShallow( {}, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, [] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, [ 0 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, [ 1 ] );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( {}, function(){} );
  test.identical( got, expected );

  /* */

  var expected = false;
  var got = _.entity.equivalentShallow( { a : '1' }, '1' );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : undefined }, undefined );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : null }, null );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : NaN }, NaN );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : 0 }, 0 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : 1 }, 1 );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : {} }, {} );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : new Date() }, new Date() );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : new F32x( 3 ) }, new F32x( 3 ) );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : /(?:)/ }, /(?:)/ );
  test.identical( got, expected );

  var expected = false;
  var got = _.entity.equivalentShallow( { a : function(){} }, function(){} );
  test.identical( got, expected );

}

//

function equivalentShallowAllTypes( test )
{
  test.open( 'equivalent' );

  test.case = 'number';
  var src1 = 1;
  var src2 = 1;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'bool & boolLike & fuzzy';
  var src1 = true;
  var src2 = true;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'boolLike & number & fuzzyLike';
  var src1 = 0;
  var src2 = 0;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'fuzzy';
  var src1 = _.maybe;
  var src2 = _.maybe;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'bigint';
  var src1 = 10n;
  var src2 = 10n;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'bigint and int';
  var src1 = 10n;
  var src2 = 10;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'str & regexpLike';
  var src1 = 'str';
  var src2 = 'str';
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'regexp & objectLike & constructible & constructibleLike';
  var src1 = /hello/g;
  var src2 = /hello/g;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'regexp & objectLike & constructible & constructibleLike, same source, diff flags';
  var src1 = /hello/g;
  var src2 = /hello/i;
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'ArgumentsArray & arrayLike';
  var src1 = _.argumentsArray.make();
  var src2 = _.argumentsArray.make();
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src1 = _.argumentsArray.make([ 1, 2, 3 ]);
  var src2 = _.argumentsArray.make([ 1, 2, 3 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'unroll';
  var src1 = _.unroll.make([ 2, 3, 4 ]);
  var src2 = _.unroll.make([ 2, 3, 4 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'array';
  var src1 = [ 2, 3, 4 ];
  var src2 = [ 2, 3, 4 ];
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'long & longLike';
  var src1 = _.long.make([ 1, 2 ]);
  var src2 = _.long.make([ 1, 2 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = `object countable - non empty, non-vector, not same array`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '4' ], countable : 1 } );
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, false );

  test.case = `object countable - non empty, non-vector, same array`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 1 } );
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, true );

  test.case = 'vector & vectorLike';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, true );

  test.case = 'countable & countableLike';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, true );

  test.case = `object countable - non empty, non-vector, no iterator`;
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 0 } );
  var src2 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '2', '3' ], countable : 0 } );
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, false );

  test.case = `object countable - non empty, non-vector, no iterator, no elements`;
  var src1 = __.diagnostic.objectMake({ new : 0, countable : 0 } );
  var src2 = __.diagnostic.objectMake({ new : 0, countable : 0 } );
  var got = _.entity.equivalentShallow( src1, src2 );
  test.identical( got, true );

  test.case = 'Global & GlobalReal';
  var src1 = global;
  var src2 = global;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Global & GlobalDerived';
  var src1 = Object.create( global );
  var src2 = Object.create( global );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src1 = { a : 1 };
  Object.setPrototypeOf( src1, { b : 2 } );
  var src2 = { a : 1 };
  Object.setPrototypeOf( src2, { b : 2 } );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src1 = Object.create( null );
  var src2 = Object.create( null );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src1 = Object.create( null );
  src1.a = 1;
  src1.b = 2;
  var src2 = Object.create( null );
  src2.a = 1;
  src2.b = 2;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src1 = {};
  var src2 = {};
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src1 = { a : 1, b : 2, c : 3 };
  var src2 = { a : 1, b : 2, c : 3 };
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'HashMap';
  var src1 = new HashMap();
  var src2 = new HashMap();
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'HashMap with 2 elems';
  var src1 = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  var src2 = new HashMap([ [ 'a', 1 ], [ 'b', 2 ] ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Set & SetLike';
  var src1 = new Set();
  var src2 = new Set();
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Set with 3 elems';
  var src1 = new Set([ 1, 2, 3 ]);
  var src2 = new Set([ 1, 2, 3 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'BufferNode';
  var src1 = BufferNode.from( 'str' );
  var src2 = BufferNode.from( 'str' );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'BufferRaw';
  var src1 = new BufferRaw( 5 );
  var src2 = new BufferRaw( 5 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'BufferRawShared';
  var src1 = new BufferRawShared( 10 );
  var src2 = new BufferRawShared( 10 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'BufferTyped';
  var src1 = new I8x( 20 );
  var src2 = new I8x( 20 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'BufferView';
  var src1 = new BufferView( new BufferRaw( 20 ) );
  var src2 = new BufferView( new BufferRaw( 20 ) );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'BufferBytes & BufferTyped';
  var src1 = new U8x( 20 );
  var src2 = new U8x( 20 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'escape';
  var src1 = _.escape.make( 1 );
  var src2 = _.escape.make( 1 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'interval & BufferTyped';
  var src1 = new F32x( 2 );
  var src2 = new F32x( 2 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'pair';
  var src1 = _.pair.make( 1, 2 );
  var src2 = _.pair.make( 1, 2 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'path & str';
  var src1 = '/a/b/';
  var src2 = '/a/b/';
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  // test.case = 'propertyTransformer & filter';
  // var src1 = _.props.condition[ 'dstAndSrcOwn' ];
  // var src2 = _.props.condition[ 'dstAndSrcOwn' ];
  // test.identical( _.entity.equivalentShallow( src1, src2 ), true );
  //
  // test.case = 'propertyTransformer & mapper';
  // var src1 = _.props.mapper[ 'assigning' ];
  // var src2 = _.props.mapper[ 'assigning' ];
  // test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'routine & routineLike';
  var src1 = routine;
  var src2 = routine;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'date & objectLike';
  var src1 = new Date( '2021-02-19T11:26:42.840Z' );
  var src2 = new Date( '2021-02-19T11:26:42.840Z' );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'null';
  var src1 = null;
  var src2 = null;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'undefined';
  var src1 = undefined;
  var src2 = undefined;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Symbol null';
  var src1 = _.null;
  var src2 = _.null;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Symbol undefined';
  var src1 = _.undefined;
  var src2 = _.undefined;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Symbol Nothing';
  var src1 = _.nothing;
  var src2 = _.nothing;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'primitive';
  var src1 = 5;
  var src2 = 5;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'stream';
  var src1 = require( 'stream' ).Readable();
  var src2 = src1;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'printerLike';
  var src1 = _global.logger;
  var src2 = _global.logger;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'console';
  var src1 = console;
  var src2 = console;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'printerLike';
  var src1 = _global.logger;
  var src2 = _global.logger;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'process';
  var src1 = process;
  var src2 = process;
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.close( 'equivalent' );

  /* - */

  test.open( 'not equivalent' );

  test.case = 'number';
  var src1 = 1;
  var src2 = 2;
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'bool & boolLike & fuzzy';
  var src1 = true;
  var src2 = false;
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'boolLike & number & fuzzyLike';
  var src1 = 0;
  var src2 = 1;
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'fuzzy';
  var src1 = _.maybe;
  var src2 = 0;
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'str & regexpLike';
  var src1 = 'str';
  var src2 = 'str2';
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'ArgumentsArray & arrayLike';
  var src1 = _.argumentsArray.make();
  var src2 = _.argumentsArray.make([ 1 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'ArgumentsArray & arrayLike with 3 elems';
  var src1 = _.argumentsArray.make([ 1, 2, 3 ]);
  var src2 = _.argumentsArray.make([ 1, 2, 3, 4 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'unroll';
  var src1 = _.unroll.make([ 2, 3, 4, 5 ]);
  var src2 = _.unroll.make([ 2, 3, 4 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'array';
  var src1 = [ [ 2 ], 3, 4 ];
  var src2 = [ [ 2 ], 3, 4 ];
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'long & longLike';
  var src1 = _.long.make([ 1, 2, [ 5 ] ]);
  var src2 = _.long.make([ 1, 2, [ 5 ] ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'vector & vectorLike same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '11' ], countable : 1, length : 2 });
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'countable & countableLike same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '11' ], countable : 1 });
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'vector & vectorLike not same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '2' ], countable : 1, length : 2 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'countable & countableLike not same elements';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '2' ], countable : 1 });
  var src2 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src1 = { [ Symbol.iterator ] : 1 };
  var src2 = { [ Symbol.iterator ] : 1 };
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src1 = { a : 1 };
  Object.setPrototypeOf( src1, { b : 2 } );
  var src2 = { b : 1 };
  Object.setPrototypeOf( src2, { b : 2 } );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src1 = Object.create( null );
  src1.a = [ 1 ];
  var src2 = Object.create( null );
  src2.a = [ 1 ];
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure with 2 elems';
  var src1 = Object.create( null );
  src1.a = [ 1, 2, 3 ];
  src1.b = [ 1, 2, 3 ];
  var src2 = Object.create( null );
  src2.a = [ 1, 2, 3 ];
  src2.b = [ 1, 2, 3 ];
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped';
  var src1 = { a : 1 };
  var src2 = {};
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPolluted & map & mapPolluted & mapPrototyped with 3 elems';
  var src1 = { a : { f : 'hello' }, b : 2, c : 3 };
  var src2 = { a : { f : 'hello' }, b : 2, c : 3 };
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'HashMap';
  var src1 = new HashMap();
  var src2 = new HashMap([ [ 'a', 1 ] ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'HashMap with 2 elems';
  var src1 = new HashMap([ [ 'a', [ 3 ] ], [ 'b', 2 ] ]);
  var src2 = new HashMap([ [ 'a', [ 3 ] ], [ 'b', 2 ] ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Set & SetLike';
  var src1 = new Set();
  var src2 = new Set([ 0 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Set with 3 elems';
  var src1 = new Set([ 1, 2, 3 ]);
  var src2 = new Set([ 1, 2, 3, 4 ]);
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'BufferNode';
  var src1 = BufferNode.from( 'str' );
  var src2 = BufferNode.from( 'str2' );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'BufferRaw';
  var src1 = new BufferRaw( 5 );
  var src2 = new BufferRaw( 6 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'BufferRawShared';
  var src1 = new BufferRawShared( 5 );
  var src2 = new BufferRawShared( 6 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'BufferTyped';
  var src1 = new I8x( 20 );
  var src2 = new I8x( 21 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'BufferView';
  var src1 = new BufferView( new BufferRaw( 20 ) );
  var src2 = new BufferView( new BufferRaw( 21 ) );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'BufferBytes & BufferTyped';
  var src1 = new U8x( 20 );
  var src2 = new U8x( 21 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'err same originalMessage';
  var src1 = _.err( 'error' );
  var src2 = _.err( 'error' );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'err diff originalMessage';
  var src1 = _.err( 'error' );
  var src2 = _.err( 'error2' );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'escape';
  var src1 = _.escape.make( 1 );
  var src2 = _.escape.make( 2 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'pair';
  var src1 = _.pair.make( 1, 2 );
  var src2 = _.pair.make( 1, 3 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'timer';
  var src1 = _.time._begin( Infinity );
  var src2 = _.time._begin( Infinity );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  _.time.cancel( src1 );
  _.time.cancel( src2 );

  test.case = 'path & str';
  var src1 = '/a/b/';
  var src2 = '/a/b/c/';
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  // test.case = 'propertyTransformer & filter';
  // var src1 = _.props.condition[ 'dstAndSrcOwn' ];
  // var src2 = _.props.condition[ 'dstNotHasOrSrcNotNull' ];
  // test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  //
  // test.case = 'propertyTransformer & mapper';
  // var src1 = _.props.mapper[ 'assigning' ];
  // var src2 = _.props.mapper[ 'removing' ];
  // test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'routine & routineLike';
  var src1 = routine;
  var src2 = () => { 'hello' };
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'timer';
  var src1 = _.time._begin( Infinity );
  var src2 = _.time._begin( 100 );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );
  _.time.cancel( src1 );
  _.time.cancel( src2 );

  test.case = 'date & objectLike';
  var src1 = new Date( '2021-02-19T11:26:42.840Z' );
  var src2 = new Date( '2020-02-19T11:26:42.840Z' );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'primitive';
  var src1 = 5;
  var src2 = 6;
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'Symbol';
  var src1 = Symbol( 'a' );
  var src2 = Symbol( 'a' );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'ConsequenceLike & promiseLike & promise';
  var src1 = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  var src2 = new Promise( ( resolve, reject ) => { return resolve( 0 ) } );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'stream';
  var src1 = require( 'stream' ).Readable();
  var src2 = require( 'stream' ).Readable();
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.close( 'not equivalent' );

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
  //
  // /* */
  //
  // function countableConstructorPolluted( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
  // }

  function routine () {}

}

//

function equivalentNotIdentical( test )
{
  test.case = 'true and 1';
  var src1 = true;
  var src2 = 1;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'false and 0';
  var src1 = false;
  var src2 = 0;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = '1 and 2, acc : 1, default accuracy';
  var src1 = 1;
  var src2 = 1.00000001;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = '1 and 2, acc : 1';
  var src1 = 1;
  var src2 = 2;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2, { accuracy : 1 } ), true );

  test.case = '10 and 20, acc : 10';
  var src1 = 10;
  var src2 = 20;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2, { accuracy : 10 } ), true );

  test.case = '1 and 1.2, acc : 02';
  var src1 = 1;
  var src2 = 1.2;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2, { accuracy : 0.2 } ), true );

  // test.case = '1n and 2, acc : 1'; /* qqq : not implemented. non-identical bigints with accuracy  */
  // var src1 = 1n;
  // var src2 = 2;
  // test.identical( _.entity.identicalShallow( src1, src2 ), false );
  // test.identical( _.entity.equivalentShallow( src1, src2, { accuracy : 1 } ), true );

  // test.case = '1n and 2n, acc : 1'; /* qqq : not implemented. non-identical bigints with accuracy  */
  // var src1 = 1n;
  // var src2 = 2n;
  // test.identical( _.entity.identicalShallow( src1, src2 ), false );
  // test.identical( _.entity.equivalentShallow( src1, src2, { accuracy : 1 } ), true );

  test.case = 'regexps with diff flags';
  var src1 = /hello/g;
  var src2 = /hello/i;
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), false );

  test.case = 'one string with whitespaces';
  var src1 = 'hello';
  var src2 = ' hello \n';
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'both strings with whitespaces';
  var src1 = '     hello\n\n\n\n\t';
  var src2 = ' \n  \thello \n';
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'BufferTyped F32x and U8x';
  var src1 = new F32x( 2 );
  var src2 = new U8x( 2 );
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'set and array';
  var src1 = new Set([ 1, 2, 3 ]);
  var src2 = [ 1, 2, 3 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'buffer typed and array';
  var src1 = new F32x([ 1, 2, 3 ])
  var src2 = [ 1, 2, 3 ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'vectorLike and array';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'countable and array';
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1 });
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = 'countable made and array';
  var src1 = __.diagnostic.objectMake({ new : 0, elements : [ '1', '10' ], countable : 1 } );
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = `vector and array`;
  var src1 = __.diagnostic.objectMake({ elements : [ '1', '10' ], countable : 1, length : 2 });
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = `argumentsArray and array`;
  var src1 = _.argumentsArray.make([ '1', '10' ]);
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

  test.case = `argumentsArray and array`;
  var src1 = _.argumentsArray.make([ '1', '10' ]);
  var src2 = [ '1', '10' ];
  test.identical( _.entity.identicalShallow( src1, src2 ), false );
  test.identical( _.entity.equivalentShallow( src1, src2 ), true );

}

//

function equal( test )
{
  test.open( 'without callbacks' );

  test.case = 'two undefined';
  var got = _.entity.equal( undefined, undefined );
  test.identical( got, true );

  test.case = 'undefined and null';
  var got = _.entity.equal( undefined, null );
  test.identical( got, false );

  test.case = 'nan and nan';
  var got = _.entity.equal( NaN, NaN );
  test.identical( got, true );

  test.case = 'equal numbers';
  var got = _.entity.equal( 1, 1 );
  test.identical( got, true );

  test.case = 'different numbers';
  var got = _.entity.equal( 1, 2 );
  test.identical( got, false );

  test.case = 'equal strings';
  var got = _.entity.equal( 'str', 'str' );
  test.identical( got, true );

  test.case = 'different strings';
  var got = _.entity.equal( 'str', 'src' );
  test.identical( got, false );

  test.case = 'empty arrays';
  var got = _.entity.equal( [], [] );
  test.identical( got, false );

  test.case = 'equal arrays';
  var got = _.entity.equal( [ 1, 2, 'str', null, undefined ], [ 1, 2, 'str', null, undefined ] );
  test.identical( got, false );

  test.case = 'not equal arrays';
  var got = _.entity.equal( [ 1 ], [ 2, 'str', null, undefined ] );
  test.identical( got, false );

  test.case = 'empty maps';
  var got = _.entity.equal( {}, {} );
  test.identical( got, false );

  test.case = 'equal maps';
  var got = _.entity.equal( { a : 2, b : 'str' }, { a : 2, b : 'str' } );
  test.identical( got, false );

  test.case = 'not equal maps';
  var got = _.entity.equal( { a : 'str' }, { b : 'str' } );
  test.identical( got, false );

  test.close( 'without callbacks' );

  /* - */

  test.open( 'only onEvaluate1' );

  test.case = 'two undefined';
  var got = _.entity.equal( undefined, undefined, ( e ) => e );
  test.identical( got, true );

  test.case = 'undefined and null';
  var got = _.entity.equal( undefined, null, ( e ) => e );
  test.identical( got, false );

  test.case = 'equal numbers';
  var got = _.entity.equal( 1, 1, ( e ) => e );
  test.identical( got, true );

  test.case = 'different numbers';
  var got = _.entity.equal( 1, 2, ( e ) => e === 1 ? e : e - 1 );
  test.identical( got, true );

  test.case = 'equal strings';
  var got = _.entity.equal( 'str', 'str', ( e ) => e );
  test.identical( got, true );

  test.case = 'different strings';
  var got = _.entity.equal( 'str', 'src', ( e ) => typeof e );
  test.identical( got, true );

  test.case = 'empty arrays';
  var got = _.entity.equal( [], [], ( e ) => e.length );
  test.identical( got, true );

  test.case = 'equal arrays';
  var got = _.entity.equal( [ 1, 2, 'str', null, undefined ], [ 1, 2, 'str', null, undefined ], ( e ) => e[ 3 ] );
  test.identical( got, true );

  test.case = 'not equal arrays';
  var got = _.entity.equal( [ 1 ], [ 2, 'str', null, undefined ], ( e ) => e[ 0 ] );
  test.identical( got, false );

  test.case = 'empty maps';
  var got = _.entity.equal( {}, {}, ( e ) => _.mapIs( e ) );
  test.identical( got, true );

  test.case = 'equal maps';
  var got = _.entity.equal( { a : 2, b : 'str' }, { a : 2, b : 'str' }, ( e ) => e.a );
  test.identical( got, true );

  test.case = 'not equal maps';
  var got = _.entity.equal( { a : 'str' }, { b : 'str' }, ( e ) => typeof e.a );
  test.identical( got, false );

  test.close( 'only onEvaluate1' );

  /* - */

  test.open( 'onEvaluate1 is equalizer' );

  test.case = 'two undefined';
  var got = _.entity.equal( undefined, undefined, ( e, ins ) => e === ins );
  test.identical( got, true );

  test.case = 'undefined and null';
  var got = _.entity.equal( undefined, null, ( e, ins ) => e === ins );
  test.identical( got, false );

  test.case = 'equal numbers';
  var got = _.entity.equal( 1, 1, ( e, ins ) => e === ins );
  test.identical( got, true );

  test.case = 'different numbers';
  var got = _.entity.equal( 1, 2, ( e, ins ) => e === ins - 1 );
  test.identical( got, true );

  test.case = 'equal strings';
  var got = _.entity.equal( 'str', 'str', ( e, ins ) => e !== ins );
  test.identical( got, false );

  test.case = 'different strings';
  var got = _.entity.equal( 'str', 'src', ( e, ins ) => typeof e === typeof ins );
  test.identical( got, true );

  test.case = 'empty arrays';
  var got = _.entity.equal( [], [], ( e, ins ) => e.length === ins.length );
  test.identical( got, true );

  test.case = 'equal arrays';
  var got = _.entity.equal( [ 1, 2, 'str', null, undefined ], [ 1, 2, 'str', null, undefined ], ( e, ins ) => e[ 0 ] === ins[ 1 ] );
  test.identical( got, false );

  test.case = 'not equal arrays';
  var got = _.entity.equal( [ 1 ], [ 2, 'str', null, undefined ], ( e, ins ) => e[ 0 ] === ins[ 0 ] - 1 );
  test.identical( got, true );

  test.case = 'empty maps';
  var got = _.entity.equal( {}, {}, ( e, ins ) => _.mapIs( e ) === _.mapIs( ins ) );
  test.identical( got, true );

  test.case = 'equal maps';
  var got = _.entity.equal( { a : 2, b : 'str' }, { a : 2, b : 'str' }, ( e, ins ) => e.a === ins.b );
  test.identical( got, false );

  test.case = 'not equal maps';
  var got = _.entity.equal( { a : 'str' }, { b : 'str' }, ( e, ins ) => e.a === ins.b );
  test.identical( got, true );

  test.close( 'onEvaluate1 is equalizer' );

  /* - */

  test.open( 'onEvaluate1 and onEvaluate2' );

  test.case = 'two undefined';
  var got = _.entity.equal( undefined, undefined, ( e ) => e, ( ins ) => ins );
  test.identical( got, true );

  test.case = 'undefined and null';
  var got = _.entity.equal( undefined, null, ( e ) => e, ( ins ) => ins );
  test.identical( got, false );

  test.case = 'equal numbers';
  var got = _.entity.equal( 1, 1, ( e ) => e, ( ins ) => ins );
  test.identical( got, true );

  test.case = 'different numbers';
  var got = _.entity.equal( 1, 2, ( e ) => e, ( ins ) => ins - 1 );
  test.identical( got, true );

  test.case = 'equal strings';
  var got = _.entity.equal( 'str', 'str', ( e ) => e, ( ins ) => !ins );
  test.identical( got, false );

  test.case = 'different strings';
  var got = _.entity.equal( 'str', 'src', ( e ) => !!e, ( ins ) => !!ins );
  test.identical( got, true );

  test.case = 'empty arrays';
  var got = _.entity.equal( [], [], ( e ) => e.length, ( ins ) => ins.length );
  test.identical( got, true );

  test.case = 'equal arrays';
  var got = _.entity.equal( [ 1, 2, 'str', null, undefined ], [ 1, 2, 'str', null, undefined ], ( e ) => !!e[ 3 ], ( ins ) => !!ins[ 4 ] );
  test.identical( got, true );

  test.case = 'not equal arrays';
  var got = _.entity.equal( [ 4 ], [ 2, 'str', null, undefined ], ( e ) => e[ 0 ], ( ins ) => ins.length );
  test.identical( got, true );

  test.case = 'empty maps';
  var got = _.entity.equal( {}, {}, ( e ) => !!e, ( ins ) => !!ins );
  test.identical( got, true );

  test.case = 'equal maps';
  var got = _.entity.equal( { a : 2, b : 'str' }, { a : 2, b : 'str' }, ( e ) => e.b, ( ins ) => ins.b );
  test.identical( got, true );

  test.case = 'not equal maps';
  var got = _.entity.equal( { a : 'str' }, { b : 'str' }, ( e ) => e.a, ( ins ) => ins.b );
  test.identical( got, true );

  test.close( 'onEvaluate1 and onEvaluate2' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entity.equal() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _.entity.equal( 1 ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'wrong length of onEvaluate1';
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, () => true ) );
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, ( a, b, c ) => a === b - c ) );

  test.case = 'wrong type of onEvaluate1';
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, [ 1 ] ) );
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, 3 ) );

  test.case = 'wrong length of onEvaluate2';
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, ( e ) => e, () => true ) );
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, ( e ) => e, ( a, b, c ) => a + b + c ) );

  test.case = 'wrong type of onEvaluate2';
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, ( e ) => e, [ 1 ] ) );
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, ( e ) => e, [ 2 ] ) );

  test.case = 'using onEvaluate2 without onEvaluate1';
  test.shouldThrowErrorSync( () => _.entity.equal( 1, 2, undefined, ( e ) => e ) );
}

// --
// inspector
// --

function lengthOf( test )
{

  test.case = 'undefined';
  var got = _.entity.lengthOf( undefined );
  test.identical( got, 0 );

  test.case = 'null';
  var got = _.entity.lengthOf( null );
  test.identical( got, 1 );

  test.case = 'false';
  var got = _.entity.lengthOf( false );
  test.identical( got, 1 );

  test.case = 'true';
  var got = _.entity.lengthOf( true );
  test.identical( got, 1 );

  test.case = 'zero';
  var got = _.entity.lengthOf( 0 );
  test.identical( got, 1 );

  test.case = 'number';
  var got = _.entity.lengthOf( 34 );
  test.identical( got, 1 );

  test.case = 'NaN';
  var got = _.entity.lengthOf( NaN );
  test.identical( got, 1 );

  test.case = 'Infinity';
  var got = _.entity.lengthOf( Infinity );
  test.identical( got, 1 );

  test.case = 'empty string';
  var got = _.entity.lengthOf( '' );
  test.identical( got, 1 );

  test.case = 'string';
  var got = _.entity.lengthOf( 'str' );
  test.identical( got, 1 );

  test.case = 'symbol';
  var got = _.entity.lengthOf( Symbol.for( 'x' ) );
  test.identical( got, 1 );

  test.case = 'empty array';
  var got = _.entity.lengthOf( [] );
  test.identical( got, 0 );

  test.case = 'array';
  var got = _.entity.lengthOf( [ [ 23, 17 ], undefined, 34 ] );
  test.identical( got, 3 );

  test.case = 'argumentsArray';
  var got = _.entity.lengthOf( _.argumentsArray.make( [ 1, [ 2, 3 ], 4 ] ) );
  test.identical( got, 3 );

  test.case = 'unroll';
  var got = _.entity.lengthOf( _.argumentsArray.make( [ 1, 2, [ 3, 4 ] ] ) );
  test.identical( got, 3 );

  test.case = 'BufferTyped';
  var got = _.entity.lengthOf( new U8x([ 1, 2, 3 ]) );
  test.identical( got, 3 );

  test.case = 'F32x';
  var got = _.entity.lengthOf( new F32x([ 1, 2, 3 ]) );
  test.identical( got, 3 );

  test.case = 'BufferRaw';
  var got = _.entity.lengthOf( new BufferRaw( 10 ) );
  test.identical( got, 1 );

  test.case = 'BufferView';
  var got = _.entity.lengthOf( new BufferView( new BufferRaw( 10 ) ) );
  test.identical( got, 1 );

  if( Config.interpreter === 'njs' )
  {
    test.case = 'BufferNode';
    var got1 = _.entity.lengthOf( BufferNode.from([ 1, 2, 3, 4 ]) );
    test.identical( got1, 4 );
  }

  test.case = 'Set';
  var got = _.entity.lengthOf( new Set( [ 1, 2, undefined, 4 ] ) );
  test.identical( got, 4 );

  test.case = 'map';
  var got = _.entity.lengthOf( { a : 1, b : 2, c : { d : 3 } } );
  test.identical( got, 3 );

  test.case = 'HashMap';
  var got = _.entity.lengthOf( new HashMap( [ [ undefined, undefined ], [ 1, 2 ], [ '', 'str' ] ] ) );
  test.identical( got, 3 );

  test.case = 'function';
  var got = _.entity.lengthOf( function(){} );
  test.identical( got, 0 );

  test.case = 'function';
  var f1 = function(){}
  f1.x = 1;
  var got = _.entity.lengthOf( f1 );
  test.identical( got, 1 );

  test.case = 'countable object';
  var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
  test.identical( _.entity.lengthOf( src ), 3 );

  test.case = 'non-countable object';
  var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 0 });
  test.identical( _.entity.lengthOf( src ), 9 );

  test.case = 'object';
  var obj1 = new Obj1({});
  var got = _.entity.lengthOf( obj1 );
  test.identical( got, 0 );

  test.case = 'object with iterator, empty';
  var obj1 = new Obj1({ elements : [] });
  obj1[ Symbol.iterator ] = _iterate;
  var got = _.entity.lengthOf( obj1 );
  test.identical( got, 0 );

  test.case = 'object with iterator, empty';
  var obj1 = new Obj1({ elements : [ 'a', 'b', 'c' ] });
  obj1[ Symbol.iterator ] = _iterate;
  var got = _.entity.lengthOf( obj1 );
  test.identical( got, 3 );

  test.case = 'instance of class';
  function Constr1()
  {
    this.a = 34;
    this.b = 's';
    this[ 100 ] = 'sms';
  };
  Constr1.prototype.toString = function()
  {
    console.log('some originalMessage');
  }
  Constr1.prototype.c = 99;
  var got = _.entity.lengthOf( new Constr1() );
  test.identical( got, 5 );

  test.case = 'object, some properties are non enumerable';
  var src = Object.create( null );
  var o =
  {
    'property1' :
    {
      value : true,
      enumerable : false,
      writable : true,
    },
    'property2' : {
      value : 'Hello',
      enumerable : false,
      writable : true,
    },
    'property3' :
    {
      enumerable : true,
      value : 'World',
      writable : true,
    }
  };
  Object.defineProperties( src, o );
  var got = _.entity.lengthOf( src );
  test.identical( got, 1 );

  test.case = 'pure map';
  var src = Object.create( null );
  src.a = 1;
  src.b = 1;
  var got = _.entity.lengthOf( src );
  test.identical( got, 2 );

  test.case = 'polluted map';
  var src = {};
  src.a = 1;
  src.b = 1;
  var got = _.entity.lengthOf( src );
  test.identical( got, 2 );

  test.case = 'pure auxilary';
  var prototype = Object.create( null );
  prototype.a = 0;
  prototype.b = 0;
  var src = Object.create( prototype );
  src.b = 1;
  src.c = 1;
  var got = _.entity.lengthOf( src );
  test.identical( got, 3 );

  test.case = 'polluted auxilary';
  var prototype = {};
  prototype.a = 0;
  prototype.b = 0;
  var src = Object.create( prototype );
  src.b = 1;
  src.c = 1;
  var got = _.entity.lengthOf( src );
  test.identical( got, 3 );

  /* */

  function Obj1( o )
  {
    _.props.extend( this, o );
    return this;
  }

  /* */

  function _iterate()
  {

    let iterator = Object.create( null );
    iterator.next = next;
    iterator.index = 0;
    iterator.instance = this;
    return iterator;

    function next()
    {
      let result = Object.create( null );
      result.done = this.index === this.instance.elements.length;
      if( result.done )
      return result;
      result.value = this.instance.elements[ this.index ];
      this.index += 1;
      return result;
    }

  }

}

// --
// elementor
// --

function elementWithCardinal( test ) /* xxx : types that cause error marked with - 'else _.assert( 0 );' and commented */
{

  /* */

  test.case = 'map';
  var src = { a : 1, b : 2 };
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 'b', true ] );
  var got = _.entity.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'hashMap';
  var src = new HashMap();
  src.set( 'a', 1 );
  src.set( 'b', 2 );
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 'b', true ] );
  var got = _.entity.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'array';
  var src = [ 1, 2 ];
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 1, true ] );
  var got = _.entity.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'set';
  var src = new Set([ 1, 2 ]);
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ 2, 1, true ] );
  var got = _.entity.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'escape';
  var src = new _.Escape( 'abc' );
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 'abc', 0, true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.entity.elementWithCardinal( src, 2 );
  test.identical( got, [ undefined, 2, false ] );

  /* */

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );
  var got5 = _.entity.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'unroll';
  var src = _.unroll.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );
  var got5 = _.entity.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'long & longLike';
  var src = _.long.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );
  var got5 = _.entity.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ '1', 0, true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ '10', 1, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  var got4 = _.entity.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ '1', 0, true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ '10', 1, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  var got4 = _.entity.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'Global & GlobalReal';
  // var src = global;
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( got, 1 );

  /* else _.assert( 0 ); */
  // test.case = 'Global & GlobalDerived';
  // var src = Object.create( global );
  // var got = _.entity.elementWithCardinal( src, 1 );
  // test.identical( _.object.isBasic( got ), true );

  /* else _.assert( 0 ); */
  // test.case = 'Object & ObjectLike & Container & ContainerLike';
  // var src = { [ Symbol.iterator ] : 1, a : 1 };
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( got, [ 0, 1 ] );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 1, false ] );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  src.a = 1;
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 1, false ] );

  test.case = 'HashMap';
  var objRef = { a : 1 };
  var src = new HashMap([ [ 'a', 1 ], [ true, false ], [ objRef, { a : 2 } ] ]);
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 'a', true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ false, true, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ { 'a' : 2 }, { 'a' : 1 }, true ] );
  var got4 = _.entity.elementWithCardinal( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'Set & SetLike';
  var objRef = { a : 1 };
  var src = new Set([ 'a', 1, true, objRef ]);
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 'a', 0, true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ 1, 1, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ true, 2, true ] );
  var got4 = _.entity.elementWithCardinal( src, 3 );
  test.identical( got4, [ { 'a' : 1 }, 3, true ] );
  var got5 = _.entity.elementWithCardinal( src, 4 );
  test.identical( got5, [ undefined, 4, false ] );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 115, 0, true ] );
  var got = _.entity.elementWithCardinal( src, 3 );
  test.identical( got, [ undefined, 3, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'BufferRaw';
  // var src = new BufferRaw( 10 );
  // var got = _.entity.elementWithCardinal( src, 1 );
  // test.identical( got, 10 );

  /* else _.assert( 0 ); */
  // test.case = 'BufferRawShared';
  // var src = new BufferRawShared( 15 );
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( got, 15 );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 0, 0, true ] );
  var got = _.entity.elementWithCardinal( src, 19 );
  test.identical( got, [ 0, 19, true ] );
  var got = _.entity.elementWithCardinal( src, 20 );
  test.identical( got, [ undefined, 20, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'BufferView';
  // var src = new BufferView( new BufferRaw( 20 ) )
  // var got = _.entity.elementWithCardinal( src, 1 );
  // test.identical( got, 20 );

  /* else _.assert( 0 ); */
  // test.case = 'err';
  // var src = _.err( 'error' );
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( _.strIs( got ), true );

  test.case = 'pair';
  var src = _.pair.make([ 1, 2 ]);
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithCardinal( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithCardinal( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );

  /* else _.assert( 0 ); */
  // test.case = 'propertyTransformer & filter';
  // var src = _.props.condition[ 'dstAndSrcOwn' ];
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( got, { 'propertyCondition' : true, 'propertyTransformer' : true, 'functor' : true } );

  /* else _.assert( 0 ); */
  // test.case = 'propertyTransformer & mapper';
  // var src = _.props.mapper[ 'assigning' ];
  // var got = _.entity.elementWithCardinal( src, 1 );
  // test.identical( got, { 'propertyMapper' : true, 'propertyTransformer' : true, 'functor' : true } );

  /* else _.assert( 0 ); */
  // test.case = 'routine & routineLike';
  // var src = routine;
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( got, 'routine' );

  test.case = 'timer';
  var src = _.time._begin( Infinity );
  var got = _.entity.elementWithCardinal( src, 0 );
  test.identical( got, [ undefined, 'onTime', true ] );
  var got = _.entity.elementWithCardinal( src, 1 );
  test.identical( got, [ undefined, 'onCancel', true ] );
  var got = _.entity.elementWithCardinal( src, 100 );
  test.identical( got, [ undefined, 100, false ] );
  _.time.cancel( src );

  /* else _.assert( 0 ); */
  // test.case = 'date & objectLike';
  // var src = new Date( 1000 );
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( got, true );

  /* else _.assert( 0 ); */
  // test.case = 'stream';
  // var src = require( 'stream' ).Readable();
  // var got = _.entity.elementWithCardinal( src, 1 );
  // test.identical( got, true );

  /* else _.assert( 0 ); */
  // test.case = 'process';
  // var src = process;
  // var got = _.entity.elementWithCardinal( src, 0 );
  // test.identical( got, undefined );

  test.case = 'key < 0';
  var src = { a : 1, b : 2 };
  var got = _.entity.elementWithCardinal( src, -1 );
  test.identical( got, [ undefined, -1, false ] );

  test.case = 'number';
  var got = _.entity.elementWithCardinal( 13, 0 );
  test.identical( got, [ 13, 0, true ] );
  var got = _.entity.elementWithCardinal( 13, 1 );
  test.identical( got, [ undefined, 1, false ] );

  test.case = 'string';
  var got = _.entity.elementWithCardinal( 'abc', 0 );
  test.identical( got, [ 'abc', 0, true ] );
  var got = _.entity.elementWithCardinal( 'abc', 1 );
  test.identical( got, [ undefined, 1, false ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.entity.elementWithCardinal() );

  test.case = 'to many args'
  test.shouldThrowErrorSync( () => _.entity.elementWithCardinal( [ 1, 2, 3 ], 1, 2 ) );

  // test.case = 'container = number'
  // test.shouldThrowErrorSync( () => _.entity.elementWithCardinal( 1, 0 ) );
  //
  // test.case = 'container = string'
  // test.shouldThrowErrorSync( () => _.entity.elementWithCardinal( 'abc', 0 ) );

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
  //
  // /* */
  //
  // function countableConstructor( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
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
  var got = _.entity.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'map.polluted';
  var src = {};
  var got = _.entity.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'map.pure';
  var src = Object.create( null );
  var got = _.entity.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'buffer';
  var src = new F32x([ 1, 2, 3 ]);
  var got = _.entity.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var got = _.entity.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'array';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithKey( src, _.props.implicit.prototype );
  var expected = [ undefined, _.props.implicit.prototype, false ];
  test.identical( got, expected );

  test.case = 'countable';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.elementWithKey( src, _.props.implicit.prototype );
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
  //
  // /* */
  //
  // function countableConstructor( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
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
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );

  test.case = 'number';
  var src = 100;
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 100, 0, true ] );
  var got = _.entity.elementWithKey( src, 1 );
  test.identical( got, [ undefined, 1, false ] );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );

  var got2 = _.entity.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'unroll';
  var src = _.unroll.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'long & longLike';
  var src = _.long.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithKey( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithKey( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.entity.elementWithKey( src, 'elements' );
  test.identical( got, [ [ '1', '10' ], 'elements', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'vector & vectorLike wit 3 elems';
  var src = __.diagnostic.objectMake({ new : 1, element1 : '1', element2 : 1, countable : 1, length : 2 });
  var got = _.entity.elementWithKey( src, 'element1' );
  test.identical( got, [ '1', 'element1', true ] );
  var got2 = _.entity.elementWithKey( src, 'element2' );
  test.identical( got2, [ 1, 'element2', true ] );
  var got3 = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got3, [ undefined, 'non-exists', false ] );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.elementWithKey( src, 'elements' );
  test.identical( got, [ [ '1', '10' ], 'elements', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'countable, empty key';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  src[ '' ] = 'empty';
  var got = _.entity.elementWithKey( src, '' );
  test.identical( got, [ 'empty', '', true ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ '1', 0, true ] );

  test.case = 'buffer';
  var src = new F32x([ 1, 2, 3 ]);
  src[ '' ] = 'empty';
  test.identical( src.length, 3 );
  var got = _.entity.elementWithKey( src, '' );
  test.identical( got, [ undefined, '', false ] );
  var got = _.entity.elementWithKey( src, 'length' );
  test.identical( got, [ undefined, 'length', false ] );
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );

  test.case = 'Global & GlobalReal';
  var src = global;
  var got = _.entity.elementWithKey( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Global & GlobalDerived';
  var src = Object.create( global );
  var got = _.entity.elementWithKey( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src = { [ Symbol.iterator ] : 1, a : 1 };
  var got = _.entity.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  var got = _.entity.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  src.a = 1;
  var got = _.entity.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'HashMap';
  var objRef = { a : 1 };
  var src = new HashMap([ [ 'a', 1 ], [ true, false ], [ objRef, { a : 2 } ] ]);
  var got = _.entity.elementWithKey( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got2 = _.entity.elementWithKey( src, true );
  test.identical( got2, [ false, true, true ] );
  var got3 = _.entity.elementWithKey( src, objRef );
  test.identical( got3, [ { 'a' : 2 }, objRef, true ] );
  var got4 = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got4, [ undefined, 'non-exists', false ] );

  test.case = 'Set & SetLike';
  var objRef = { a : 1 };
  var src = new Set([ 'a', 1, true, objRef ]);
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithKey( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithKey( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  // var got = _.entity.elementWithKey( src, 0 );
  // test.identical( got, [ 'a', 0, true ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.entity.elementWithKey( src, 1 );
  test.identical( got, [ 1, 1, true ] );
  var got = _.entity.elementWithKey( src, 'a' );
  test.identical( got, [ 'a', 'a', true ] );
  var got2 = _.entity.elementWithKey( src, 1 );
  test.identical( got2, [ 1, 1, true ] );
  // var got3 = _.entity.elementWithKey( src, 2 );
  // test.identical( got3, [ true, 2, true ] );
  var got3 = _.entity.elementWithKey( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  // var got4 = _.entity.elementWithKey( src, 3 );
  // test.identical( got4, [ objRef, 3, true ] );
  // var got4 = _.entity.elementWithKey( src,  );
  // var got4 = _.entity.elementWithKey( src, undefined );
  // test.identical( got4, [ undefined, 3, true ] );
  var got4 = _.entity.elementWithKey( src, undefined );
  test.identical( got4, [ undefined, undefined, false ] );
  var got4 = _.entity.elementWithKey( src, 4 );
  test.identical( got4, [ undefined, 4, false ] );
  var got5 = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got5, [ undefined, 'non-exists', false ] );

  test.case = 'Set with undefined';
  var src = new Set([ undefined, 1, 2, 3 ]);
  var got = _.entity.elementWithKey( src, undefined );
  test.identical( got, [ undefined, undefined, true ] );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 115, 0, true ] );
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRaw';
  var src = new BufferRaw( 10 );
  var got = _.entity.elementWithKey( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRawShared';
  var src = new BufferRawShared( 15 );
  var got = _.entity.elementWithKey( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 0, 0, true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferView';
  var src = new BufferView( new BufferRaw( 20 ) )
  var got = _.entity.elementWithKey( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'err';
  var src = _.err( 'error' );
  var got = _.entity.elementWithKey( src, 'originalMessage' );
  // test.identical( _.strIs( got[ 0 ] ), true );
  // test.identical( got[ 1 ], 'originalMessage' );
  // test.identical( got[ 2 ], true );
  test.identical( got, [ got[ 0 ], 'originalMessage', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'escape';
  var src = _.escape.make( 1 );
  var got = _.entity.elementWithKey( src, 'val' );
  test.identical( got, [ 1, 'val', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'pair';
  var src = _.pair.make([ 1, 2 ]);
  var got = _.entity.elementWithKey( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithKey( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got = _.entity.elementWithKey( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithKey( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  // test.case = 'propertyTransformer & filter';
  // var src = _.props.condition[ 'dstAndSrcOwn' ];
  // var got = _.entity.elementWithKey( src, 'identity' );
  // test.identical( got, [ { 'propertyCondition' : true, 'propertyTransformer' : true, 'functor' : true }, 'identity', true ] );
  // var got = _.entity.elementWithKey( src, 'non-exists' );
  // test.identical( got, [ undefined, 'non-exists', false ] );
  //
  // test.case = 'propertyTransformer & mapper';
  // var src = _.props.mapper[ 'assigning' ];
  // var got = _.entity.elementWithKey( src, 'identity' );
  // test.identical( got, [ { 'propertyMapper' : true, 'propertyTransformer' : true, 'functor' : true }, 'identity', true ] );
  // var got = _.entity.elementWithKey( src, 'non-exists' );
  // test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'routine & routineLike';
  var src = routine;
  var got = _.entity.elementWithKey( src, 'name' );
  test.identical( got, [ 'routine', 'name', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'timer';
  var src = _.time._begin( Infinity );
  var got = _.entity.elementWithKey( src, 'type' );
  test.identical( got, [ 'delay', 'type', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );
  _.time.cancel( src );

  test.case = 'date & objectLike';
  var src = new Date( 1000 );
  var got = _.entity.elementWithKey( src, 'getTime' );
  test.identical( got, [ src.getTime, 'getTime', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'stream';
  var src = require( 'stream' ).Readable();
  var got = _.entity.elementWithKey( src, 'readable' );
  test.identical( got, [ true, 'readable', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'process';
  var src = process;
  var got = _.entity.elementWithKey( src, 'cwd' );
  test.identical( got, [ src.cwd, 'cwd', true ] );
  var got = _.entity.elementWithKey( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.entity.elementWithKey() );

  test.case = '1 arg'
  test.shouldThrowErrorSync( () => _.entity.elementWithKey( [] ) );

  test.case = 'too many args'
  test.shouldThrowErrorSync( () => _.entity.elementWithKey( {}, 'a', 'b' ) );

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
  //
  // /* */
  //
  // function countableConstructor( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
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

  /* - */

  test.case = 'prototype object';
  var src = new Obj1({});
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype map.polluted';
  var src = {};
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.prototype, _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype map.pure';
  var src = Object.create( null );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ null, _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype buffer';
  var src = new F32x([ 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype array';
  var src = [ 1, 2, 3 ];
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype arguments array';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.prototype, _.props.implicit.prototype, true ];
  test.identical( got, expected );

  test.case = 'prototype countable';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  var expected = [ Object.getPrototypeOf( src ), _.props.implicit.prototype, true ];
  test.identical( got, expected );

  /* - */

  test.case = 'constructor object';
  var src = new Obj1({});
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object.getPrototypeOf( src ).constructor, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor map.polluted';
  var src = {};
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor map.pure';
  var src = Object.create( null );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ null, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor buffer';
  var src = new F32x([ 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object.getPrototypeOf( src ).constructor, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor array';
  var src = [ 1, 2, 3 ];
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object.getPrototypeOf( src ).constructor, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor arguments array';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ Object, _.props.implicit.constructor, true ];
  test.identical( got, expected );

  test.case = 'constructor countable';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  var expected = [ src.constructor, _.props.implicit.constructor, true ];
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
  //
  // /* */
  //
  // function countableConstructor( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
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

function elementWithImplicit( test )
{

  /* */

  test.case = 'number';
  var src = 13;
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 13, 0, true ] );
  var got = _.entity.elementWithImplicit( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  test.identical( got, [ Number.prototype, _.props.implicit.prototype, true ] );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  test.identical( got, [ Number, _.props.implicit.constructor, true ] );

  /* */

  test.case = 'undefined';
  var src = undefined;
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.entity.elementWithImplicit( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  test.identical( got, [ undefined, _.props.implicit.prototype, false ] );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  test.identical( got, [ undefined, _.props.implicit.constructor, false ] );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ null, 0, true ] );
  var got = _.entity.elementWithImplicit( src, 1 );
  test.identical( got, [ undefined, 1, false ] );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.prototype );
  test.identical( got, [ undefined, _.props.implicit.prototype, false ] );
  var got = _.entity.elementWithImplicit( src, _.props.implicit.constructor );
  test.identical( got, [ undefined, _.props.implicit.constructor, false ] );

  /* */

  test.case = 'string';
  var src = 'abc';
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );

  test.case = 'number';
  var src = 100;
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 100, 0, true ] );
  var got = _.entity.elementWithImplicit( src, 1 );
  test.identical( got, [ undefined, 1, false ] );

  test.case = 'ArgumentsArray & arrayLike';
  var src = _.argumentsArray.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );

  var got2 = _.entity.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'unroll';
  var src = _.unroll.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'array';
  var src = [ 1, 2, 3 ];
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'long & longLike';
  var src = _.long.make([ 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got3 = _.entity.elementWithImplicit( src, 2 );
  test.identical( got3, [ 3, 2, true ] );
  var got4 = _.entity.elementWithImplicit( src, 3 );
  test.identical( got4, [ undefined, 3, false ] );

  test.case = 'vector & vectorLike';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1, length : 2 });
  var got = _.entity.elementWithImplicit( src, 'elements' );
  test.identical( got, [ [ '1', '10' ], 'elements', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'vector & vectorLike wit 3 elems';
  var src = __.diagnostic.objectMake({ new : 1, element1 : '1', element2 : 1, countable : 1, length : 2 });
  var got = _.entity.elementWithImplicit( src, 'element1' );
  test.identical( got, [ '1', 'element1', true ] );
  var got2 = _.entity.elementWithImplicit( src, 'element2' );
  test.identical( got2, [ 1, 'element2', true ] );
  var got3 = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got3, [ undefined, 'non-exists', false ] );

  test.case = 'countable & countableLike';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  var got = _.entity.elementWithImplicit( src, 'elements' );
  test.identical( got, [ [ '1', '10' ], 'elements', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'countable, empty key';
  var src = __.diagnostic.objectMake({ new : 1, elements : [ '1', '10' ], countable : 1 });
  src[ '' ] = 'empty';
  var got = _.entity.elementWithImplicit( src, '' );
  test.identical( got, [ 'empty', '', true ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ '1', 0, true ] );

  test.case = 'buffer';
  var src = new F32x([ 1, 2, 3 ]);
  src[ '' ] = 'empty';
  test.identical( src.length, 3 );
  var got = _.entity.elementWithImplicit( src, '' );
  test.identical( got, [ undefined, '', false ] );
  var got = _.entity.elementWithImplicit( src, 'length' );
  test.identical( got, [ undefined, 'length', false ] );
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );

  test.case = 'Global & GlobalReal';
  var src = global;
  var got = _.entity.elementWithImplicit( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Global & GlobalDerived';
  var src = Object.create( global );
  var got = _.entity.elementWithImplicit( src, 'wTools' );
  test.identical( _.object.isBasic( got[ 0 ] ), true );
  test.identical( got[ 1 ], 'wTools' );
  test.identical( got[ 2 ], true );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & Container & ContainerLike';
  var src = { [ Symbol.iterator ] : 1, a : 1 };
  var got = _.entity.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & auxiliaryPrototyped & auxiliaryPolluted';
  var src = { a : 1 };
  var got = _.entity.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'Object & ObjectLike & auxiliary & map & mapPure';
  var src = Object.create( null );
  src.a = 1;
  var got = _.entity.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'HashMap';
  var objRef = { a : 1 };
  var src = new HashMap([ [ 'a', 1 ], [ true, false ], [ objRef, { a : 2 } ] ]);
  var got = _.entity.elementWithImplicit( src, 'a' );
  test.identical( got, [ 1, 'a', true ] );
  var got2 = _.entity.elementWithImplicit( src, true );
  test.identical( got2, [ false, true, true ] );
  var got3 = _.entity.elementWithImplicit( src, objRef );
  test.identical( got3, [ { 'a' : 2 }, objRef, true ] );
  var got4 = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got4, [ undefined, 'non-exists', false ] );

  test.case = 'Set & SetLike';
  var objRef = { a : 1 };
  var src = new Set([ 'a', 1, true, objRef ]);
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got3 = _.entity.elementWithImplicit( src, '2' );
  test.identical( got3, [ undefined, '2', false ] );
  var got4 = _.entity.elementWithImplicit( src, '3' );
  test.identical( got4, [ undefined, '3', false ] );
  // var got = _.entity.elementWithImplicit( src, 0 );
  // test.identical( got, [ 'a', 0, true ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ undefined, 0, false ] );
  var got = _.entity.elementWithImplicit( src, 1 );
  test.identical( got, [ 1, 1, true ] );
  var got = _.entity.elementWithImplicit( src, 'a' );
  test.identical( got, [ 'a', 'a', true ] );
  var got2 = _.entity.elementWithImplicit( src, 1 );
  test.identical( got2, [ 1, 1, true ] );
  // var got3 = _.entity.elementWithImplicit( src, 2 );
  // test.identical( got3, [ true, 2, true ] );
  var got3 = _.entity.elementWithImplicit( src, 2 );
  test.identical( got3, [ undefined, 2, false ] );
  // var got4 = _.entity.elementWithImplicit( src, 3 );
  // test.identical( got4, [ objRef, 3, true ] );
  // var got4 = _.entity.elementWithImplicit( src,  );
  // var got4 = _.entity.elementWithImplicit( src, undefined );
  // test.identical( got4, [ undefined, 3, true ] );
  var got4 = _.entity.elementWithImplicit( src, undefined );
  test.identical( got4, [ undefined, undefined, false ] );
  var got4 = _.entity.elementWithImplicit( src, 4 );
  test.identical( got4, [ undefined, 4, false ] );
  var got5 = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got5, [ undefined, 'non-exists', false ] );

  test.case = 'Set with undefined';
  var src = new Set([ undefined, 1, 2, 3 ]);
  var got = _.entity.elementWithImplicit( src, undefined );
  test.identical( got, [ undefined, undefined, true ] );

  // test.case = 'Set & SetLike';
  // var objRef = { a : 1 };
  // var src = new Set([ 'a', 1, true, objRef ]);
  // var got = _.entity.elementWithImplicit( src, '0' );
  // test.identical( got, [ undefined, '0', false ] );
  // var got2 = _.entity.elementWithImplicit( src, '1' );
  // test.identical( got2, [ undefined, '1', false ] );
  // var got3 = _.entity.elementWithImplicit( src, '2' );
  // test.identical( got3, [ undefined, '2', false ] );
  // var got4 = _.entity.elementWithImplicit( src, '3' );
  // test.identical( got4, [ undefined, '3', false ] );
  // var got = _.entity.elementWithImplicit( src, 0 );
  // test.identical( got, [ 'a', 0, true ] );
  // var got2 = _.entity.elementWithImplicit( src, 1 );
  // test.identical( got2, [ 1, 1, true ] );
  // var got3 = _.entity.elementWithImplicit( src, 2 );
  // test.identical( got3, [ true, 2, true ] );
  // var got4 = _.entity.elementWithImplicit( src, 3 );
  // test.identical( got4, [ objRef, 3, true ] );
  // var got4 = _.entity.elementWithImplicit( src, 4 );
  // test.identical( got4, [ undefined, 4, false ] );
  // var got5 = _.entity.elementWithImplicit( src, 'non-exists' );
  // test.identical( got5, [ undefined, 'non-exists', false ] );

  test.case = 'BufferNode';
  var src = BufferNode.from( 'str' );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 115, 0, true ] );
  var got = _.entity.elementWithImplicit( src, 1 );
  test.identical( got, [ 116, 1, true ] );
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRaw';
  var src = new BufferRaw( 10 );
  var got = _.entity.elementWithImplicit( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferRawShared';
  var src = new BufferRawShared( 15 );
  var got = _.entity.elementWithImplicit( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferTyped';
  var src = new I8x( 20 );
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 0, 0, true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'BufferView';
  var src = new BufferView( new BufferRaw( 20 ) )
  var got = _.entity.elementWithImplicit( src, 'byteLength' );
  test.identical( got, [ undefined, 'byteLength', false ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'err';
  var src = _.err( 'error' );
  var got = _.entity.elementWithImplicit( src, 'originalMessage' );
  test.identical( got, [ 'error', 'originalMessage', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'escape';
  var src = _.escape.make( 1 );
  var got = _.entity.elementWithImplicit( src, 'val' );
  test.identical( got, [ 1, 'val', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'pair';
  var src = _.pair.make([ 1, 2 ]);
  var got = _.entity.elementWithImplicit( src, '0' );
  test.identical( got, [ undefined, '0', false ] );
  var got2 = _.entity.elementWithImplicit( src, '1' );
  test.identical( got2, [ undefined, '1', false ] );
  var got = _.entity.elementWithImplicit( src, 0 );
  test.identical( got, [ 1, 0, true ] );
  var got2 = _.entity.elementWithImplicit( src, 1 );
  test.identical( got2, [ 2, 1, true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  // test.case = 'propertyTransformer & filter';
  // var src = _.props.condition[ 'dstAndSrcOwn' ];
  // var got = _.entity.elementWithImplicit( src, 'identity' );
  // test.identical( got, [ { 'propertyCondition' : true, 'propertyTransformer' : true, 'functor' : true }, 'identity', true ] );
  // var got = _.entity.elementWithImplicit( src, 'non-exists' );
  // test.identical( got, [ undefined, 'non-exists', false ] );
  //
  // test.case = 'propertyTransformer & mapper';
  // var src = _.props.mapper[ 'assigning' ];
  // var got = _.entity.elementWithImplicit( src, 'identity' );
  // test.identical( got, [ { 'propertyMapper' : true, 'propertyTransformer' : true, 'functor' : true }, 'identity', true ] );
  // var got = _.entity.elementWithImplicit( src, 'non-exists' );
  // test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'routine & routineLike';
  var src = routine;
  var got = _.entity.elementWithImplicit( src, 'name' );
  test.identical( got, [ 'routine', 'name', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'timer';
  var src = _.time._begin( Infinity );
  var got = _.entity.elementWithImplicit( src, 'type' );
  test.identical( got, [ 'delay', 'type', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );
  _.time.cancel( src );

  test.case = 'date & objectLike';
  var src = new Date( 1000 );
  var got = _.entity.elementWithImplicit( src, 'getTime' );
  test.identical( got, [ src.getTime, 'getTime', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'stream';
  var src = require( 'stream' ).Readable();
  var got = _.entity.elementWithImplicit( src, 'readable' );
  test.identical( got, [ true, 'readable', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  test.case = 'process';
  var src = process;
  var got = _.entity.elementWithImplicit( src, 'cwd' );
  test.identical( got, [ src.cwd, 'cwd', true ] );
  var got = _.entity.elementWithImplicit( src, 'non-exists' );
  test.identical( got, [ undefined, 'non-exists', false ] );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.entity.elementWithImplicit() );

  test.case = 'to many args'
  test.shouldThrowErrorSync( () => _.entity.elementWithImplicit( {}, 'a', 'b' ) );

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
  //
  // /* */
  //
  // function countableConstructor( o )
  // {
  //   return countableMake( this, o );
  // }
  //
  // /* */
  //
  // function countableMake( dst, o )
  // {
  //   if( dst === null )
  //   dst = Object.create( null );
  //   _.props.extend( dst, o );
  //   if( o.countable )
  //   dst[ Symbol.iterator ] = _iterate;
  //   return dst;
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

// --
// define test suite
// --

const Proto =
{

  name : 'Tools.Entity.l0.l3',
  silencing : 1,
  routineTimeOut : 10000,

  tests :
  {

    // exporter

    exportStringDiagnosticShallow,
    strPrimitive,
    strTypeWithTraitsBasic,
    strTypeWithoutTraitsBasic,
    strTypeWithTraitsGeneratedObject,
    strTypeWithoutTraitsGeneratedObject,

    // equaler
    eqShallowBasic,
    identicalShallowBasic, /* qqq : for Rahul : merge test routines identicalShallowBasic and equivalentShallowBasic into single test routine eqShallowBasic */
    eqShallowAllTypes,
    identicalShallowAllTypes, /* qqq : for Rahul : merge test routines identicalShallowAllTypes and equivalentShallowAllTypes into single test routine eqShallowAllTypes */
    equivalentShallowBasic,
    equivalentShallowAllTypes,
    equivalentNotIdentical,

    equal,

    // inspector

    lengthOf,

    // elementor

    elementWithCardinal,
    elementWithKeyArgImplicit,
    elementWithKey,
    elementWithImplicitArgImplicit,
    elementWithImplicit,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
