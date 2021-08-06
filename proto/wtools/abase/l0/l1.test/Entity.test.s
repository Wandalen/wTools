( function _l0_l1_Entity_test_s()
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

function dichotomy( test )
{

  test.case = 'null';
  var src = null;
  test.identical( _.entity.is( src ), true );
  test.identical( _.entity.like( src ), true );

  test.case = 'undefined';
  var src = undefined;
  test.identical( _.entity.is( src ), true );
  test.identical( _.entity.like( src ), true );

  test.case = 'number';
  var src = 1;
  test.identical( _.entity.is( src ), true );
  test.identical( _.entity.like( src ), true );

  test.case = 'object';
  var src = __.diagnostic.objectMake({});
  test.identical( _.entity.is( src ), true );
  test.identical( _.entity.like( src ), true );

}

// --
// maker
// --

function makeEmpty( test )
{
  test.case = 'null';
  var got = _.entity.makeEmpty( null );
  test.identical( got, null );

  test.case = 'undefined';
  var got = _.entity.makeEmpty( undefined );
  test.identical( got, undefined );

  test.case = 'zero';
  var got = _.entity.makeEmpty( 0 );
  test.identical( got, 0 );

  test.case = 'number';
  var got = _.entity.makeEmpty( 3 );
  test.identical( got, 3 );

  test.case = 'bigInt';
  var got = _.entity.makeEmpty( 1n );
  test.identical( got, 1n );

  test.case = 'empty string';
  var got = _.entity.makeEmpty( '' );
  test.identical( got, '' );

  test.case = 'string';
  var got = _.entity.makeEmpty( 'str' );
  test.identical( got, 'str' );

  test.case = 'false';
  var got = _.entity.makeEmpty( false );
  test.identical( got, false );

  test.case = 'NaN';
  var got = _.entity.makeEmpty( NaN );
  test.identical( got, NaN );

  test.case = 'Symbol';
  var src = Symbol( 'a' );
  var got = _.entity.makeEmpty( src );
  test.identical( got, src );

  test.case = '_.null';
  var got = _.entity.makeEmpty( _.null );
  test.identical( got, _.null );

  test.case = '_.undefined';
  var got = _.entity.makeEmpty( _.undefined );
  test.identical( got, _.undefined );

  test.case = '_.nothing';
  var got = _.entity.makeEmpty( _.nothing );
  test.identical( got, _.nothing );

  test.case = 'empty array';
  var got = _.entity.makeEmpty( [] );
  test.identical( got, [] );

  test.case = 'not empty array';
  var got = _.entity.makeEmpty( [ null, undefined, 1, 2 ] );
  test.identical( got, [] );

  test.case = 'empty argumentArray';
  /* qqq : for junior : rewrite whole test using src and .true checks */
  var src = _.argumentsArray.make( [] );
  var got = _.entity.makeEmpty( src );
  test.identical( got, _.argumentsArray.make( [] ) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( got !== src );

  test.case = 'not empty argumentsArray';
  var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
  var got = _.entity.makeEmpty( src );
  test.identical( got, _.argumentsArray.make( [] ) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( got !== src );

  test.case = 'empty unroll';
  var src = _.unroll.make( [] );
  var got = _.entity.makeEmpty( src );
  test.identical( got, [] );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( got !== src );

  test.case = 'not empty unroll';
  var src = _.unroll.make([ null, undefined, 1, 2 ]);
  var got = _.entity.makeEmpty( src );
  test.identical( got, _.unroll.from( [] ) );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( got !== src );

  test.case = 'BufferTyped - U8x';
  var got = _.entity.makeEmpty( new U8x( 10 ) );
  test.identical( got, new U8x() );

  test.case = 'BufferTyped - I16x';
  var got = _.entity.makeEmpty( new I16x() );
  test.identical( got, new I16x() );

  test.case = 'BufferTyped - F64x';
  var got = _.entity.makeEmpty( new F64x( [ 1, 2 ] ) );
  test.identical( got, new F64x() );

  test.case = 'empty map';
  debugger;
  var got = _.entity.makeEmpty( {} );
  test.identical( got, {} ); debugger;
  test.true( _.mapIsPure( got ) );

  test.case = 'not empty map';
  var got = _.entity.makeEmpty( { '' : null } );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'empty pure map';
  var got = _.entity.makeEmpty( Object.create( null ) );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'empty Set';
  var got = _.entity.makeEmpty( new Set( [] ) );
  test.identical( got, new Set( [] ) );

  test.case = 'Set';
  var got = _.entity.makeEmpty( new Set( [ 1, 'str', false ] ) );
  test.identical( got, new Set( [] ) );

  test.case = 'empty HashMap';
  var got = _.entity.makeEmpty( new HashMap( [] ) );
  test.identical( got, new HashMap( [] ) );

  test.case = 'HashMap';
  var got = _.entity.makeEmpty( new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
  test.identical( got, new HashMap( [] ) );

  test.case = 'BufferRaw, has constructor';
  var got = _.entity.makeEmpty( new BufferRaw() );
  test.identical( got, new BufferRaw( [] ) );

  // test.case = 'constructor';
  // function func(){ return 0 };
  // var got = _.entity.makeEmpty( func );
  // var exp = func;
  // test.equivalent( got(), exp() );

  test.case = 'constructor';
  function Constr(){ this.x = 1; return this };
  var got = _.entity.makeEmpty( new Constr() );
  var exp = new Constr();
  test.identical( got.x, exp.x );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entity.makeEmpty() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entity.makeEmpty( [], 1 ) );
}

//

function makeEmptyLongDescriptor( test )
{
  let times = 4;

  test.case = 'F32x. long';
  var got = _.withLong.F32x.long.makeEmpty();
  test.identical( got, new F32x() );
  var src = [];
  var got = _.withLong.F32x.long.makeEmpty( src );
  test.identical( got, [] );
  test.true( src !== got );
  var src = [ 1, 2, 3 ];
  var got = _.withLong.F32x.long.makeEmpty( src );
  test.identical( got, [] );
  test.true( src !== got );

  test.case = 'F32x. entity';
  var src = [];
  var got = _.withLong.F32x.entity.makeEmpty( src );
  test.identical( got, [] );
  test.true( src !== got );
  var src = [ 1, 2, 3 ];
  var got = _.withLong.F32x.entity.makeEmpty( src );
  test.identical( got, [] );
  test.true( src !== got );

  // for( let e in _.LongDescriptors )
  // {
  //   let name = _.LongDescriptors[ e ].name;
  //   let long = _.withLong[ name ];

  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let name = namespace.TypeName;
    let long = _.withLong[ name ];

    test.open( `long : ${ name }` );
    testRun( long );
    test.close( `long : ${ name }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function testRun( long )
  {
    test.case = 'null';
    var got = long.entity.makeEmpty( null );
    test.identical( got, null );

    test.case = 'undefined';
    var got = long.entity.makeEmpty( undefined );
    test.identical( got, undefined );

    test.case = 'zero';
    var got = long.entity.makeEmpty( 0 );
    test.identical( got, 0 );

    test.case = 'number';
    var got = long.entity.makeEmpty( 3 );
    test.identical( got, 3 );

    test.case = 'bigInt';
    var got = long.entity.makeEmpty( 1n );
    test.identical( got, 1n );

    test.case = 'empty string';
    var got = long.entity.makeEmpty( '' );
    test.identical( got, '' );

    test.case = 'string';
    var got = long.entity.makeEmpty( 'str' );
    test.identical( got, 'str' );

    test.case = 'false';
    var got = long.entity.makeEmpty( false );
    test.identical( got, false );

    test.case = 'NaN';
    var got = long.entity.makeEmpty( NaN );
    test.identical( got, NaN );

    test.case = 'Symbol';
    var src = Symbol( 'a' );
    var got = long.entity.makeEmpty( src );
    test.identical( got, src );

    test.case = '_.null';
    var got = long.entity.makeEmpty( _.null );
    test.identical( got, _.null );

    test.case = '_.undefined';
    var got = long.entity.makeEmpty( _.undefined );
    test.identical( got, _.undefined );

    test.case = '_.nothing';
    var got = long.entity.makeEmpty( _.nothing );
    test.identical( got, _.nothing );

    test.case = 'empty array';
    var got = long.entity.makeEmpty( [] );
    test.identical( got, [] );

    test.case = 'not empty array';
    var got = long.entity.makeEmpty( [ null, undefined, 1, 2 ] );
    test.identical( got, [] );

    /* qqq : for junior : rewrite whole test using src and .true checks */
    test.case = 'empty argumentArray';
    var src = _.argumentsArray.make( [] );
    var got = long.entity.makeEmpty( src );
    test.identical( got, _.argumentsArray.make( [] ) );
    test.true( got !== src );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );

    test.case = 'not empty argumentsArray';
    var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.makeEmpty( src );
    test.identical( got, _.argumentsArray.make( [] ) );
    test.true( got !== src );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );

    test.case = 'empty unroll';
    var src = _.unroll.make( [] );
    var got = long.entity.makeEmpty( src );
    test.identical( got, _.unroll.make([]) );
    test.true( got !== src );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );

    test.case = 'not empty unroll';
    var src = _.unroll.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.makeEmpty( src );
    test.identical( got, _.unroll.make([]) );
    test.true( got !== src );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );

    test.case = 'BufferTyped - U8x';
    var got = long.entity.makeEmpty( new U8x( 10 ) );
    test.identical( got, new U8x() );

    test.case = 'BufferTyped - I16x';
    var got = long.entity.makeEmpty( new I16x() );
    test.identical( got, new I16x() );

    test.case = 'BufferTyped - F64x';
    var got = long.entity.makeEmpty( new F64x( [ 1, 2 ] ) );
    test.identical( got, new F64x() );

    test.case = 'empty map';
    var got = long.entity.makeEmpty( {} );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'not empty map';
    var got = long.entity.makeEmpty( { '' : null } );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'empty pure map';
    var got = long.entity.makeEmpty( Object.create( null ) );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'empty Set';
    var got = long.entity.makeEmpty( new Set( [] ) );
    test.identical( got, new Set( [] ) );

    test.case = 'Set';
    var got = long.entity.makeEmpty( new Set( [ 1, 'str', false ] ) );
    test.identical( got, new Set( [] ) );

    test.case = 'empty HashMap';
    var got = long.entity.makeEmpty( new HashMap( [] ) );
    test.identical( got, new HashMap( [] ) );

    test.case = 'HashMap';
    var got = long.entity.makeEmpty( new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
    test.identical( got, new HashMap( [] ) );

    test.case = 'BufferRaw, has constructor';
    var got = long.entity.makeEmpty( new BufferRaw() );
    test.identical( got, new BufferRaw( [] ) );

    // test.case = 'constructor';
    // function func(){ return 0 };
    // var got = long.entity.makeEmpty( func );
    // var exp = func;
    // test.equivalent( got(), exp() );

    test.case = 'constructor';
    function Constr(){ this.x = 1; return this };
    var got = long.entity.makeEmpty( new Constr() );
    var exp = new Constr();
    test.identical( got.x, exp.x );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.entity.makeEmpty() );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.entity.makeEmpty( [], 1 ) );
    }
  }

  /* */

  function Constr()
  {
    this.x = 1;
    return this;
  };
}

//

function makeUndefined( test )
{
  test.case = 'null';
  var got = _.entity.makeUndefined( null );
  test.identical( got, null );

  test.case = 'undefined';
  var got = _.entity.makeUndefined( undefined );
  test.identical( got, undefined );

  test.case = 'zero';
  var got = _.entity.makeUndefined( 0 );
  test.identical( got, 0 );

  test.case = 'number';
  var got = _.entity.makeUndefined( 3 );
  test.identical( got, 3 );

  test.case = 'bigInt';
  var got = _.entity.makeUndefined( 1n );
  test.identical( got, 1n );

  test.case = 'empty string';
  var got = _.entity.makeUndefined( '' );
  test.identical( got, '' );

  test.case = 'string';
  var got = _.entity.makeUndefined( 'str' );
  test.identical( got, 'str' );

  test.case = 'false';
  var got = _.entity.makeUndefined( false );
  test.identical( got, false );

  test.case = 'NaN';
  var got = _.entity.makeUndefined( NaN );
  test.identical( got, NaN );

  test.case = 'Symbol';
  var src = Symbol( 'a' );
  var got = _.entity.makeUndefined( src );
  test.identical( got, src );

  test.case = '_.null';
  var got = _.entity.makeUndefined( _.null );
  test.identical( got, _.null );

  test.case = '_.undefined';
  var got = _.entity.makeUndefined( _.undefined );
  test.identical( got, _.undefined );

  test.case = '_.nothing';
  var got = _.entity.makeUndefined( _.nothing );
  test.identical( got, _.nothing );

  test.case = 'empty array';
  var got = _.entity.makeUndefined( [] );
  test.identical( got, [] );

  test.case = 'empty array, length';
  var got = _.entity.makeUndefined( [], 4 );
  test.identical( got, [ undefined, undefined, undefined, undefined ] );

  test.case = 'not empty array';
  var got = _.entity.makeUndefined( [ null, undefined, 1, 2 ] );
  test.identical( got, [ undefined, undefined, undefined, undefined ] );

  test.case = 'not empty array, length';
  var got = _.entity.makeUndefined( [ null, undefined, 1, 2 ], 2 );
  test.identical( got, [ undefined, undefined ] );

  /* qqq : for junior : rewrite whole test using src and .true checks */
  test.case = 'empty arguments array';
  var src = _.argumentsArray.make( [] );
  var got = _.entity.makeUndefined( src );
  test.identical( got, _.argumentsArray.make( [] ) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'empty arguments array, length';
  var src = _.argumentsArray.make( [] );
  var got = _.entity.makeUndefined( src, 4 );
  test.identical( got, _.argumentsArray.make([ undefined, undefined, undefined, undefined ]) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'not empty argumentsArray';
  var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
  var got = _.entity.makeUndefined( src );
  test.identical( got, _.argumentsArray.make([ undefined, undefined, undefined, undefined ]) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'not empty argumentsArray, length';
  var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
  var got = _.entity.makeUndefined( src, 2 );
  test.identical( got, _.argumentsArray.make([ undefined, undefined ]) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( src !== got );

  test.case = 'empty unroll';
  var src = _.unroll.make( [] );
  var got = _.entity.makeUndefined( src );
  test.identical( got, [] );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  test.case = 'empty unroll, length';
  var src = _.unroll.make( [] );
  var got = _.entity.makeUndefined( src, 4 );
  test.identical( got, _.unroll.make([ undefined, undefined, undefined, undefined ]) );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  test.case = 'not empty unroll';
  var src = _.unroll.make( [ null, undefined, 1, 2 ] );
  var got = _.entity.makeUndefined( src );
  test.identical( got, _.unroll.make([ undefined, undefined, undefined, undefined ]) );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );
  test.true( src !== got );

  test.case = 'not empty unroll, length';
  var src = _.unroll.make( [ null, undefined, 1, 2 ] );
  var got = _.entity.makeUndefined( src, 2 );
  test.identical( got, _.unroll.make([ undefined, undefined ]) );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  test.case = 'empty BufferTyped - U8x';
  var got = _.entity.makeUndefined( new U8x() );
  test.identical( got, new U8x( 0 ) );

  test.case = 'empty BufferTyped - I16x, length';
  var got = _.entity.makeUndefined( new I16x(), 5 );
  test.identical( got, new I16x( 5 ) );

  test.case = 'BufferTyped - F32x';
  var got = _.entity.makeUndefined( new F32x( 5 ) );
  test.identical( got, new F32x( 5 ) );

  test.case = 'BufferTyped - F32x, length';
  var got = _.entity.makeUndefined( new F32x( 10 ), 5 );
  test.identical( got, new F32x( 5 ) );

  test.case = 'empty map';
  var got = _.entity.makeUndefined( {} );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'empty map, length';
  var got = _.entity.makeUndefined( {}, 4 );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'not empty map';
  var got = _.entity.makeUndefined( { '' : null } );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'not empty map, length';
  var got = _.entity.makeUndefined( { '' : null }, 4 );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'empty pure map';
  var got = _.entity.makeUndefined( Object.create( null ) );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'empty pure map, length';
  var got = _.entity.makeUndefined( Object.create( null ) );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );

  test.case = 'empty Set';
  var got = _.entity.makeUndefined( new Set( [] ) );
  test.identical( got, new Set( [] ) );

  test.case = 'empty Set, length';
  var got = _.entity.makeUndefined( new Set( [] ), 4 );
  test.identical( got, new Set( [] ) );

  test.case = 'Set';
  var got = _.entity.makeUndefined( new Set( [ 1, 'str', false ] ) );
  test.identical( got, new Set( [] ) );

  test.case = 'Set, length';
  var got = _.entity.makeUndefined( new Set( [ 1, 'str', false ] ), 4 );
  test.identical( got, new Set( [] ) );

  test.case = 'empty HashMap';
  var got = _.entity.makeUndefined( new HashMap( [] ) );
  test.identical( got, new HashMap( [] ) );

  test.case = 'empty HashMap, length';
  var got = _.entity.makeUndefined( new HashMap( [] ), 4 );
  test.identical( got, new HashMap( [] ) );

  test.case = 'HashMap';
  var got = _.entity.makeUndefined( new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
  test.identical( got, new HashMap( [] ) );

  test.case = 'HashMap, length';
  var got = _.entity.makeUndefined( new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
  test.identical( got, new HashMap( [] ) );

  test.case = 'BufferRaw, has constructor';
  var got = _.entity.makeUndefined( new BufferRaw() );
  test.identical( got, new BufferRaw( [] ) );

  // test.case = 'constructor';
  // function func(){ return 0 };
  // var got = _.entity.makeUndefined( func );
  // var exp = func;
  // test.equivalent( got(), exp() );

  test.case = 'constructor';
  function Constr(){ this.x = 1; return this };
  var got = _.entity.makeUndefined( new Constr() );
  var exp = new Constr();
  test.identical( got.x, exp.x );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entity.makeUndefined() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entity.makeUndefined( [], 1, 1 ) );
}

//

function makeUndefinedLongDescriptor( test )
{
  test.case = 'F32x. long';
  var got = _.withLong.F32x.long.makeUndefined();
  test.identical( got, new F32x() );
  var src = [];
  var got = _.withLong.F32x.long.makeUndefined( src );
  test.identical( got, [] );
  test.true( src !== got );
  var src = [ 1, 2, 3 ];
  var got = _.withLong.F32x.long.makeUndefined( src );
  test.identical( got, [ undefined, undefined, undefined ] );
  test.true( src !== got );

  test.case = 'F32x. entity';
  var src = [];
  var got = _.withLong.F32x.entity.makeUndefined( src );
  test.identical( got, [] );
  test.true( src !== got );
  var src = [ 1, 2, 3 ];
  var got = _.withLong.F32x.entity.makeUndefined( src );
  test.identical( got, [ undefined, undefined, undefined ] );
  test.true( src !== got );

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
    test.case = 'null';
    var got = long.entity.makeUndefined( null );
    test.identical( got, null );

    test.case = 'undefined';
    var got = long.entity.makeUndefined( undefined );
    test.identical( got, undefined );

    test.case = 'zero';
    var got = long.entity.makeUndefined( 0 );
    test.identical( got, 0 );

    test.case = 'number';
    var got = long.entity.makeUndefined( 3 );
    test.identical( got, 3 );

    test.case = 'bigInt';
    var got = long.entity.makeUndefined( 1n );
    test.identical( got, 1n );

    test.case = 'empty string';
    var got = long.entity.makeUndefined( '' );
    test.identical( got, '' );

    test.case = 'string';
    var got = long.entity.makeUndefined( 'str' );
    test.identical( got, 'str' );

    test.case = 'false';
    var got = long.entity.makeUndefined( false );
    test.identical( got, false );

    test.case = 'NaN';
    var got = long.entity.makeUndefined( NaN );
    test.identical( got, NaN );

    test.case = 'Symbol';
    var src = Symbol( 'a' );
    var got = long.entity.makeUndefined( src );
    test.identical( got, src );

    test.case = '_.null';
    var got = long.entity.makeUndefined( _.null );
    test.identical( got, _.null );

    test.case = '_.undefined';
    var got = long.entity.makeUndefined( _.undefined );
    test.identical( got, _.undefined );

    test.case = '_.nothing';
    var got = long.entity.makeUndefined( _.nothing );
    test.identical( got, _.nothing );

    test.case = 'empty array';
    var got = long.entity.makeUndefined( [] );
    test.identical( got, [] );

    test.case = 'empty array, length';
    var got = long.entity.makeUndefined( [], 4 );
    test.identical( got, [ undefined, undefined, undefined, undefined ] );

    test.case = 'not empty array';
    var got = long.entity.makeUndefined( [ null, undefined, 1, 2 ] );
    test.identical( got, [ undefined, undefined, undefined, undefined ] );

    test.case = 'not empty array, length';
    var got = long.entity.makeUndefined( [ null, undefined, 1, 2 ], 2 );
    test.identical( got, [ undefined, undefined ] );

    /* qqq : for junior : rewrite whole test using src and .true checks */
    test.case = 'empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = long.entity.makeUndefined( src );
    test.identical( got, _.argumentsArray.make( [] ) );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );
    test.true( got !== src );

    test.case = 'empty arguments array, length';
    var src = _.argumentsArray.make( [] );
    var got = long.entity.makeUndefined( src, 4 );
    test.identical( got, _.argumentsArray.make( 4 ) );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );
    test.true( got !== src );

    test.case = 'not empty argumentsArray';
    var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.makeUndefined( src );
    test.identical( got, _.argumentsArray.make( 4 ) );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );
    test.true( got !== src );

    test.case = 'not empty argumentsArray, length';
    var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.makeUndefined( src, 2 );
    test.identical( got, _.argumentsArray.make( 2 ) );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );
    test.true( got !== src );

    test.case = 'empty unroll';
    var src = _.unroll.make( [] );
    var got = long.entity.makeUndefined( src );
    test.identical( got, _.unroll.from([]) );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );
    test.true( got !== src );

    test.case = 'empty unroll, length';
    var src = _.unroll.make( [] );
    var got = long.entity.makeUndefined( src, 4 );
    test.identical( got, _.unroll.from([ undefined, undefined, undefined, undefined ]) );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );
    test.true( got !== src );

    test.case = 'not empty unroll';
    var src = _.unroll.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.makeUndefined( src );
    test.identical( got, _.unroll.from([ undefined, undefined, undefined, undefined ]) );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );
    test.true( got !== src );

    test.case = 'not empty unroll, length';
    var src = _.unroll.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.makeUndefined( src, 2 );
    test.identical( got, _.unroll.from([ undefined, undefined ]) );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );
    test.true( got !== src );

    test.case = 'empty BufferTyped - U8x';
    var got = long.entity.makeUndefined( new U8x() );
    test.identical( got, new U8x( 0 ) );

    test.case = 'empty BufferTyped - I16x, length';
    var got = long.entity.makeUndefined( new I16x(), 5 );
    test.identical( got, new I16x( 5 ) );

    test.case = 'BufferTyped - F32x';
    var got = long.entity.makeUndefined( new F32x( 5 ) );
    test.identical( got, new F32x( 5 ) );

    test.case = 'BufferTyped - F32x, length';
    var got = long.entity.makeUndefined( new F32x( 10 ), 5 );
    test.identical( got, new F32x( 5 ) );

    test.case = 'empty map';
    var got = long.entity.makeUndefined( {} );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'empty map, length';
    var got = long.entity.makeUndefined( {}, 4 );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'not empty map';
    var got = long.entity.makeUndefined( { '' : null } );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'not empty map, length';
    var got = long.entity.makeUndefined( { '' : null }, 4 );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'empty pure map';
    var got = long.entity.makeUndefined( Object.create( null ) );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'empty pure map, length';
    var got = long.entity.makeUndefined( Object.create( null ) );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );

    test.case = 'empty Set';
    var got = long.entity.makeUndefined( new Set( [] ) );
    test.identical( got, new Set( [] ) );

    test.case = 'empty Set, length';
    var got = long.entity.makeUndefined( new Set( [] ), 4 );
    test.identical( got, new Set( [] ) );

    test.case = 'Set';
    var got = long.entity.makeUndefined( new Set( [ 1, 'str', false ] ) );
    test.identical( got, new Set( [] ) );

    test.case = 'Set, length';
    var got = long.entity.makeUndefined( new Set( [ 1, 'str', false ] ), 4 );
    test.identical( got, new Set( [] ) );

    test.case = 'empty HashMap';
    var got = long.entity.makeUndefined( new HashMap( [] ) );
    test.identical( got, new HashMap( [] ) );

    test.case = 'empty HashMap, length';
    var got = long.entity.makeUndefined( new HashMap( [] ), 4 );
    test.identical( got, new HashMap( [] ) );

    test.case = 'HashMap';
    var got = long.entity.makeUndefined( new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
    test.identical( got, new HashMap( [] ) );

    test.case = 'HashMap, length';
    var got = long.entity.makeUndefined( new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
    test.identical( got, new HashMap( [] ) );

    test.case = 'BufferRaw, has constructor';
    var got = long.entity.makeUndefined( new BufferRaw() );
    test.identical( got, new BufferRaw( [] ) );

    // test.case = 'constructor';
    // function func(){ return 0 };
    // var got = long.entity.makeUndefined( func );
    // var exp = func;
    // test.equivalent( got(), exp() );

    test.case = 'constructor';
    function Constr(){ this.x = 1; return this };
    var got = long.entity.makeUndefined( new Constr() );
    var exp = new Constr();
    test.identical( got.x, exp.x );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.entity.makeUndefined() );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.entity.makeUndefined( [], 1, 1 ) );
    }
  }

  /* */

  function Constr()
  {
    this.x = 1;
    return this;
  };
}

makeUndefinedLongDescriptor.timeOut = 10000;

//

function make( test )
{
  test.case = 'null';
  var got = _.entity.make( null );
  test.identical( got, null );

  test.case = 'undefined';
  var got = _.entity.make( undefined );
  test.identical( got, undefined );

  test.case = 'zero';
  var got = _.entity.make( 0 );
  test.identical( got, 0 );

  test.case = 'number';
  var got = _.entity.make( 3 );
  test.identical( got, 3 );

  test.case = 'bigInt';
  var got = _.entity.make( 1n );
  test.identical( got, 1n );

  test.case = 'empty string';
  var got = _.entity.make( '' );
  test.identical( got, '' );

  test.case = 'string';
  var got = _.entity.make( 'str' );
  test.identical( got, 'str' );

  test.case = 'false';
  var got = _.entity.make( false );
  test.identical( got, false );

  test.case = 'NaN';
  var got = _.entity.make( NaN );
  test.identical( got, NaN );

  test.case = 'Symbol';
  var src = Symbol( 'a' );
  var got = _.entity.make( src );
  test.identical( got, src );

  test.case = '_.null';
  var got = _.entity.make( _.null );
  test.identical( got, _.null );

  test.case = '_.undefined';
  var got = _.entity.make( _.undefined );
  test.identical( got, _.undefined );

  test.case = '_.nothing';
  var got = _.entity.make( _.nothing );
  test.identical( got, _.nothing );

  test.case = 'empty array';
  var src = [];
  var got = _.entity.make( src );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'not empty array';
  var src = [ null, undefined, 1, 2 ];
  var got = _.entity.make( src );
  test.identical( got, [ null, undefined, 1, 2 ] );
  test.true( got !== src );

  /* qqq : for junior : rewrite whole test using src and .true checks */
  test.case = 'empty argumentArray';
  var src = _.argumentsArray.make( [] );
  var got = _.entity.make( src );
  test.identical( got, _.argumentsArray.make( [] ) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( got !== src );

  test.case = 'not empty argumentsArray';
  var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
  var got = _.entity.make( src );
  test.identical( got, _.argumentsArray.make([ null, undefined, 1, 2 ]) );
  test.true( !_.array.is( got ) );
  test.true( _.argumentsArray.is( got ) );
  test.true( got !== src );

  test.case = 'empty unroll';
  var src = _.unroll.make( [] );
  var got = _.entity.make( src );
  test.identical( got, _.unroll.make([]) );
  test.true( _.array.is( got ) );
  test.true( _.unroll.is( got ) );
  test.true( got !== src );

  test.case = 'not empty unroll';
  var src = _.unroll.make( [ null, undefined, 1, 2 ] );
  var got = _.entity.make( src );
  test.identical( got, _.unroll.make([ null, undefined, 1, 2 ]) );
  test.true( _.array.is( got ) );
  test.true( _.unroll.is( got ) );
  test.true( got !== src );

  test.case = 'empty BufferTyped - U8x';
  var src = new U8x();
  var got = _.entity.make( src );
  test.identical( got, new U8x( [] ) );
  test.true( got instanceof U8x )
  test.true( got !== src );

  test.case = 'not empty BufferTyped - I16x';
  var src = new I16x( 5 );
  var got = _.entity.make( src );
  test.identical( got, new I16x( [ 0, 0, 0, 0, 0 ] ) );
  test.true( got instanceof I16x )
  test.true( got !== src );

  test.case = 'not empty BufferTyped - F32x';
  var src = new F32x( 5 );
  var got = _.entity.make( src );
  test.identical( got, new F32x( [ 0, 0, 0, 0, 0 ] ) );
  test.true( got instanceof F32x )
  test.true( got !== src );

  test.case = 'not empty BufferTyped - F64x';
  var src = new F64x( 5 );
  var got = _.entity.make( src );
  test.identical( got, new F64x( [ 0, 0, 0, 0, 0 ] ) );
  test.true( got instanceof F64x )
  test.true( got !== src );

  test.case = 'empty map';
  var src = {};
  var got = _.entity.make( src );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'not empty map';
  var src = { '' : null };
  var got = _.entity.make( src );
  test.identical( got, { '' : null } );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.entity.make( src );
  test.identical( got, {} );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'pure map';
  var src = Object.create( null );
  src.a = 2;
  var got = _.entity.make( src );
  test.identical( got, { a : 2 } );
  test.true( _.mapIsPure( got ) );
  test.true( got !== src );

  test.case = 'empty Set';
  var src = new Set( [] );
  var got = _.entity.make( src );
  test.identical( got, new Set( [] ) );
  test.true( got !== src );

  test.case = 'Set';
  var src = new Set( [ 1, 'str', false ] );
  var got = _.entity.make( src );
  test.identical( got, new Set( [ 1, 'str', false ] ) );
  test.true( got !== src );

  test.case = 'empty HashMap';
  var src = new HashMap( [] );
  var got = _.entity.make( src );
  test.identical( got, new HashMap( [] ) );
  test.true( got !== src );

  test.case = 'HashMap';
  var src = new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] );
  var got = _.entity.make( src );
  test.identical( got, new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
  test.true( got !== src );

  test.case = 'BufferRaw, has constructor';
  var got = _.entity.make( new BufferRaw() );
  test.identical( got, new BufferRaw( [] ) );

  // test.case = 'constructor';
  // function func(){ return 0 };
  // var got = _.entity.make( func );
  // var exp = func;
  // test.equivalent( got(), exp() );

  test.case = 'constructor';
  function Constr(){ this.x = 1; return this };
  var got = _.entity.make( new Constr() );
  var exp = new Constr();
  test.identical( got.x, exp.x );

  /* */

  test.case = 'routine for key shallowSymbol';
  function Constr1(){ this.x = 1; return this };
  var src = new Constr1();
  src[ _.class.cloneShallowSymbol ] = () => new Constr1();
  var got = _.entity.make( src );
  var exp = new Constr1();
  test.identical( got.x, exp.x );

  test.case = 'routine for key shallowSymbol';
  function Constr2(){ this.x = 1; return this };
  var src = new Constr2();
  src.cloneShallow = () => new Constr2();
  var got = _.entity.make( src );
  var exp = new Constr1();
  test.identical( got.x, exp.x );

  test.case = 'array and length';
  var src = [ 3 ];
  var got = _.entity.make( src, 2 );
  var exp = [ 3, undefined ];
  test.identical( got, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entity.make() );

  test.case = '3rd arguments';
  test.shouldThrowErrorSync( () => _.entity.make( [], 1, 1 ) );
}

//

function makeLongDescriptor( test )
{

  test.case = 'F32x. long';
  var got = _.withLong.F32x.long.make();
  test.identical( got, new F32x() );
  var src = [];
  var got = _.withLong.F32x.long.make( src );
  test.identical( got, [] );
  test.true( src !== got );
  var src = [ 1, 2, 3 ];
  var got = _.withLong.F32x.long.make( src );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( src !== got );

  test.case = 'F32x. entity';
  var src = [];
  var got = _.withLong.F32x.entity.make( src );
  test.identical( got, [] );
  test.true( src !== got );
  var src = [ 1, 2, 3 ];
  var got = _.withLong.F32x.entity.make( src );
  test.identical( got, [ 1, 2, 3 ] );
  test.true( src !== got );

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
    test.case = 'null';
    var got = long.entity.make( null );
    test.identical( got, null );

    test.case = 'undefined';
    var got = long.entity.make( undefined );
    test.identical( got, undefined );

    test.case = 'zero';
    var got = long.entity.make( 0 );
    test.identical( got, 0 );

    test.case = 'number';
    var got = long.entity.make( 3 );
    test.identical( got, 3 );

    test.case = 'bigInt';
    var got = long.entity.make( 1n );
    test.identical( got, 1n );

    test.case = 'empty string';
    var got = long.entity.make( '' );
    test.identical( got, '' );

    test.case = 'string';
    var got = long.entity.make( 'str' );
    test.identical( got, 'str' );

    test.case = 'false';
    var got = long.entity.make( false );
    test.identical( got, false );

    test.case = 'NaN';
    var got = long.entity.make( NaN );
    test.identical( got, NaN );

    test.case = 'Symbol';
    var src = Symbol( 'a' );
    var got = long.entity.make( src );
    test.identical( got, src );

    test.case = '_.null';
    var got = long.entity.make( _.null );
    test.identical( got, _.null );

    test.case = '_.undefined';
    var got = long.entity.make( _.undefined );
    test.identical( got, _.undefined );

    test.case = '_.nothing';
    var got = long.entity.make( _.nothing );
    test.identical( got, _.nothing );

    test.case = 'empty array';
    var src = [];
    var got = long.entity.make( src );
    test.identical( got, [] );
    test.true( got !== src );

    test.case = 'not empty array';
    var src = [ null, undefined, 1, 2 ];
    var got = long.entity.make( src );
    test.identical( got, [ null, undefined, 1, 2 ] );
    test.true( got !== src );

    /* qqq : for junior : rewrite whole test using src and .true checks */
    test.case = 'empty argumentArray';
    var src = _.argumentsArray.make( [] );
    var got = long.entity.make( src );
    test.identical( got, _.argumentsArray.make( [] ) );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );
    test.true( got !== src );

    test.case = 'not empty argumentsArray';
    var src = _.argumentsArray.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.make( src );
    test.identical( got, _.argumentsArray.make( [ null, undefined, 1, 2 ] ) );
    test.true( !_.array.is( got ) );
    test.true( _.argumentsArray.is( got ) );
    test.true( got !== src );

    test.case = 'empty unroll';
    var src = _.unroll.make( [] );
    var got = long.entity.make( src );
    test.identical( got, _.unroll.from([]) );
    test.true( got !== src );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );

    test.case = 'not empty unroll';
    var src = _.unroll.make( [ null, undefined, 1, 2 ] );
    var got = long.entity.make( src );
    test.identical( got, _.unroll.from([ null, undefined, 1, 2 ]) );
    test.true( got !== src );
    test.true( _.array.is( got ) );
    test.true( _.unroll.is( got ) );

    test.case = 'empty BufferTyped - U8x';
    var src = new U8x();
    var got = long.entity.make( src );
    test.identical( got, new U8x( [] ) );
    test.true( got !== src );

    test.case = 'not empty BufferTyped - I16x';
    var src = new I16x( 5 );
    var got = long.entity.make( src );
    test.identical( got, new I16x( [ 0, 0, 0, 0, 0 ] ) );
    test.true( got !== src );

    test.case = 'not empty BufferTyped - F64x';
    var src = new F64x( 5 );
    var got = long.entity.make( src );
    test.identical( got, new F64x( [ 0, 0, 0, 0, 0 ] ) );
    test.true( got !== src );

    test.case = 'empty map';
    var src = {};
    var got = long.entity.make( src );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );
    test.true( got !== src );

    test.case = 'not empty map';
    var src = { '' : null };
    var got = long.entity.make( src );
    test.identical( got, { '' : null } );
    test.true( _.mapIsPure( got ) );
    test.true( got !== src );

    test.case = 'empty pure map';
    var src = Object.create( null );
    var got = long.entity.make( src );
    test.identical( got, {} );
    test.true( _.mapIsPure( got ) );
    test.true( got !== src );

    test.case = 'pure map';
    var src = Object.create( null );
    src.a = 2;
    var got = long.entity.make( src );
    test.identical( got, { a : 2 } );
    test.true( _.mapIsPure( got ) );
    test.true( got !== src );

    test.case = 'empty Set';
    var src = new Set( [] );
    var got = long.entity.make( src );
    test.identical( got, new Set( [] ) );
    test.true( got !== src );

    test.case = 'Set';
    var src = new Set( [ 1, 'str', false ] );
    var got = long.entity.make( src );
    test.identical( got, new Set( [ 1, 'str', false ] ) );
    test.true( got !== src );

    test.case = 'empty HashMap';
    var src = new HashMap( [] );
    var got = long.entity.make( src );
    test.identical( got, new HashMap( [] ) );
    test.true( got !== src );

    test.case = 'HashMap';
    var src = new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] );
    var got = long.entity.make( src );
    test.identical( got, new HashMap( [ [ 'a', 1 ], [ 'b', 2 ] ] ) );
    test.true( got !== src );

    test.case = 'BufferRaw, has constructor';
    var got = long.entity.make( new BufferRaw() );
    test.identical( got, new BufferRaw( [] ) );

    // test.case = 'constructor';
    // function func(){ return 0 };
    // var got = long.entity.make( func );
    // var exp = func;
    // test.equivalent( got(), exp() );

    test.case = 'constructor';
    function Constr(){ this.x = 1; return this };
    var got = long.entity.make( new Constr() );
    var exp = new Constr();
    test.identical( got.x, exp.x );

    /* */

    test.case = 'routine for key shallowSymbol';
    function Constr1(){ this.x = 1; return this };
    var src = new Constr1();
    src[ _.class.cloneShallowSymbol ] = () => new Constr1();
    var got = long.entity.make( src );
    var exp = new Constr1();
    test.identical( got.x, exp.x );

    test.case = 'routine for key shallowSymbol';
    function Constr2(){ this.x = 1; return this };
    var src = new Constr2();
    src.cloneShallow = () => new Constr2();
    var got = long.entity.make( src );
    var exp = new Constr1();
    test.identical( got.x, exp.x );

    test.case = 'array and length';
    var src = [ 3 ];
    var got = long.entity.make( src, 2 );
    var exp = [ 3, undefined ];
    test.identical( got, exp );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.entity.make() );

      test.case = '3rd arguments';
      test.shouldThrowErrorSync( () => long.entity.make( [], 1 ,1 ) );
    }
  }

  /* */

  function Constr()
  {
    this.x = 1;
    return this;
  };
}

/* qqq2 : for junior : implement 4 separate test routines for
- make / makeEmpty / makeUndefined / cloneShallow
- with long / without long
- elements : 0, 1, 2
- type of argument : Array, ArgumentsArray, Unroll, F32x

/* xxx : cases with Array + 0, 1, 2 elements */

//

function makeCommon( test )
{

  methodEach({ tools : 'default', type : 'Array' });
  methodEach({ tools : 'Array', type : 'Array' });
  methodEach({ tools : 'F32x', type : 'F32x' });

  /* - */

  function methodEach( env )
  {

    env.method = 'makeEmpty';
    act( env );
    env.method = 'makeUndefined';
    act( env );
    env.method = 'make';
    act( env );
    env.method = 'cloneShallow';
    act( env );

  }

  /* - */

  function act( env )
  {

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, empty array`;
    var got = tools( env ).entity[ env.method ]( [] );
    test.true( got instanceof Array );
    test.identical( got.length, 0 );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, 1 element`;
    var got = tools( env ).entity[ env.method ]( [ 2 ] );
    test.true( got instanceof Array );
    if( env.method === 'makeEmpty' )
    test.identical( got.length, 0 );
    else
    test.identical( got.length, 1 );
    if( env.method === 'make' || env.method === 'cloneShallow' )
    test.identical( got, [ 2 ] );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, 2 elements`;
    var got = tools( env ).entity[ env.method ]( [ 2, 3 ] );
    test.true( got instanceof Array );
    if( env.method === 'makeEmpty' )
    test.identical( got.length, 0 );
    else
    test.identical( got.length, 2 );
    if( env.method === 'make' || env.method === 'cloneShallow' )
    test.identical( got, [ 2, 3 ] );

    /* */

    if( env.method !== 'makeEmpty' && env.method !== 'cloneShallow' )
    {
      test.case = `${__.entity.exportStringSolo( env )}, empty and length`;
      var got = tools( env ).entity[ env.method ]( [], 2 );
      test.true( got instanceof Array );
      test.identical( got.length, 2 );
    }

    /* */

    if( env.method !== 'makeEmpty' && env.method !== 'cloneShallow' )
    {
      test.case = `${__.entity.exportStringSolo( env )}, non-empty and length longer`;
      var got = tools( env ).entity[ env.method ]( [ 3, 4 ], 3 );
      test.true( got instanceof Array );
      test.identical( got.length, 3 );
      if( env.method === 'make' )
      test.identical( got, [ 3, 4, undefined ] );
    }

    /* */

    if( env.method !== 'makeEmpty' && env.method !== 'cloneShallow' )
    {
      test.case = `${__.entity.exportStringSolo( env )}, non-empty and length shorter`;
      var got = tools( env ).entity[ env.method ]( [ 3, 4 ], 1 );
      test.true( got instanceof Array );
      test.identical( got.length, 1 );
      if( env.method === 'make' )
      test.identical( got, [ 3 ] );
    }

    /* */

    if( env.method !== 'makeEmpty' && env.method !== 'cloneShallow' )
    {
      test.case = `${__.entity.exportStringSolo( env )}, non-empty and ins longer`;
      var got = tools( env ).entity[ env.method ]( [ 3, 4 ], [ 2, 3, 4 ] );
      test.true( got instanceof Array );
      test.identical( got.length, 3 );
      if( env.method === 'make' )
      test.identical( got, [ 2, 3, 4 ] );
    }

    /* */

    if( env.method !== 'makeEmpty' && env.method !== 'cloneShallow' )
    {
      test.case = `${__.entity.exportStringSolo( env )}, non-empty and ins shorter`;
      var got = tools( env ).entity[ env.method ]( [ 3, 4 ], [ 2 ] );
      test.true( got instanceof Array );
      test.identical( got.length, 1 );
      if( env.method === 'make' )
      test.identical( got, [ 2 ] );
    }

    /* */

  }

  /* */

  function tools( env )
  {
    if( env.tools === 'default' )
    return _;
    return _.withLong[ env.tools ];
  }

  /* - */

}

// --
// define test suite
// --

const Proto =
{

  name : 'Tools.Entity.l0.l1',
  silencing : 1,

  tests :
  {

    // dichotomy

    dichotomy,

    // maker

    makeEmpty,
    makeEmptyLongDescriptor,

    makeUndefined,
    makeUndefinedLongDescriptor,

    make,
    makeLongDescriptor,
    /* qqq : for junior : implement test for cloneShallow */

    makeCommon,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
