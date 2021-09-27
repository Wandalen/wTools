( function _l0_l1_Routine_test_s_()
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

// --
// dichotomy
// --

/* qqq : for Rahul

first rewrite using example

  test.case = 'check null';
  var src = null;
  test.identical( _.routine.is( src ), false );
  test.identical( _.routine.like( src ), false );
  test.identical( _.routine.isSync( src ), false );
  test.identical( _.routine.isAsync( src ), false );
  test.identical( _.routine.isTrivial( src ), false );
  test.identical( _.routine.withName( src ), false );
  test.identical( _.routine.isGenerator( src ), false );
  test.identical( _.routine.isSyncGenerator( src ), false );
  test.identical( _.routine.isAsyncGenerator( src ), false );

join into single test routine::dichotomy

  is,
  like,
  isSync,
  isAsync,
  isTrivial,

*/

function dichotomy( test )
{
  test.case = `No Arguments`;
  test.true( !_.routine.is() );
  test.true( !_.routine.like() );
  test.true( !_.routine.isTrivial() );
  test.true( !_.routine.isSync() );
  test.true( !_.routine.isAsync() );
  test.true( !_.routine.isGenerator() );
  test.true( !_.routine.isSyncGenerator() );
  test.true( !_.routine.isAsyncGenerator() );
  test.true( !_.routine.withName() );

  /* */

  test.case = `Null`;
  test.true( !_.routine.is( null ) );
  test.true( !_.routine.like( null ) );
  test.true( !_.routine.isTrivial( null ) );
  test.true( !_.routine.isSync( null ) );
  test.true( !_.routine.isAsync( null ) );
  test.true( !_.routine.isGenerator( null ) );
  test.true( !_.routine.isSyncGenerator( null ) );
  test.true( !_.routine.isAsyncGenerator( null ) );
  test.true( !_.routine.withName( null ) );

  /* */

  test.case = `Nothing`;
  var src = _.nothing;
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Boolean`;
  var src = false;
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Nan`;
  var src = NaN;
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Symbol`;
  var src = Symbol( 'a' ); ;
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Empty Array`;
  var src = [];
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Array`;
  var src = [ 1, 2, 3 ];
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Arguments Array`;
  var src = _.argumentsArray.make( [] );
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Unroll`;
  var src = _.unroll.make( [] );
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Pure Map`;
  var src = Object.create( null );
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Set`;
  var src = new Set( [ 1, 1, 2 ] );
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Map`;
  var src = { a : 'a' };
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Buffer raw`;
  var src = new BufferRaw();
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Buffer Typed`;
  var src = new U8x();
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Number`;
  var src = 123;
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Big Int`;
  var src = 1n;
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `String`;
  var src = 'string';
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Object`;
  var src = Object;
  test.true( _.routine.is( src ) );
  test.true( _.routine.like( src ) );
  test.true( _.routine.isTrivial( src ) );
  test.true( _.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( _.routine.withName( src ) );

  /* */

  test.case = `Instance of constructor`;
  function Constr(){ this.x = 1; return this };
  var src = new Constr();
  test.true( !_.routine.is( src ) );
  test.true( !_.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( !_.routine.withName( src ) );

  /* */

  test.case = `Instance Constructor`;
  function Constr1(){ this.x = 1; return this };
  var src = ( new Constr1() ).constructor;
  test.true( _.routine.is( src ) );
  test.true( _.routine.like( src ) );
  test.true( _.routine.isTrivial( src ) );
  test.true( _.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( _.routine.withName( src ) );

  /* */

  test.case = `Unnamed arrow function`;
  test.true( _.routine.is( () => {} ) );
  test.true( _.routine.like( () => {} ) );
  test.true( _.routine.isTrivial( () => {} ) );
  test.true( _.routine.isSync( () => {} ) );
  test.true( !_.routine.isAsync( () => {} ) );
  test.true( !_.routine.isGenerator( () => {} ) );
  test.true( !_.routine.isSyncGenerator( () => {} ) );
  test.true( !_.routine.isAsyncGenerator( () => {} ) );
  test.true( !_.routine.withName( () => {} ) );

  /* */

  test.case = `Named arrow function`;
  var src = () => {};
  test.true( _.routine.is( src ) );
  test.true( _.routine.like( src ) );
  test.true( _.routine.isTrivial( src ) );
  test.true( _.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( _.routine.withName( src ) );

  /* */

  test.case = `Unnamed Routine`;
  test.true( _.routine.is( function (){} ) );
  test.true( _.routine.like( function (){} ) );
  test.true( _.routine.isTrivial( function (){} ) );
  test.true( _.routine.isSync( function (){} ) );
  test.true( !_.routine.isAsync( function (){} ) );
  test.true( !_.routine.isGenerator( function (){} ) );
  test.true( !_.routine.isSyncGenerator( function (){} ) );
  test.true( !_.routine.isAsyncGenerator( function (){} ) );
  test.true( !_.routine.withName( function (){} ) );

  /* */

  test.case = `Named Routine`;
  test.true( _.routine.is( function a(){} ) );
  test.true( _.routine.like( function a(){} ) );
  test.true( _.routine.isTrivial( function a(){} ) );
  test.true( _.routine.isSync( function a(){} ) );
  test.true( !_.routine.isAsync( function a(){} ) );
  test.true( !_.routine.isGenerator( function a(){} ) );
  test.true( !_.routine.isSyncGenerator( function a(){} ) );
  test.true( !_.routine.isAsyncGenerator( function a(){} ) );
  test.true( _.routine.withName( function a(){} ) );

  /* */

  test.case = `Async unnamed arrow routine`;
  test.true( _.routine.is( async () => {} ) );
  test.true( _.routine.like( async () => {} ) );
  test.true( _.routine.isTrivial( async () => {} ) );
  test.true( !_.routine.isSync( async () => {} ) );
  test.true( _.routine.isAsync( async () => {} ) );
  test.true( !_.routine.isGenerator( async () => {} ) );
  test.true( !_.routine.isSyncGenerator( async () => {} ) );
  test.true( !_.routine.isAsyncGenerator( async () => {} ) );
  test.true( !_.routine.withName( async () => {} ) );

  /* */

  test.case = `Async named arrow routine`;
  var src = async() => {};
  test.true( _.routine.is( src ) );
  test.true( _.routine.like( src ) );
  test.true( _.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( _.routine.isAsync( src ) );
  test.true( !_.routine.isGenerator( src ) );
  test.true( !_.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( _.routine.withName( src ) );

  /* */

  test.case = `Async unnamed routine`;
  test.true( _.routine.is( async function (){} ) );
  test.true( _.routine.like(async function (){} ) );
  test.true( _.routine.isTrivial( async function (){} ) );
  test.true( !_.routine.isSync( async function (){} ) );
  test.true( _.routine.isAsync( async function (){} ) );
  test.true( !_.routine.isGenerator( async function (){} ) );
  test.true( !_.routine.isSyncGenerator( async function (){} ) );
  test.true( !_.routine.isAsyncGenerator( async function (){} ) );
  test.true( !_.routine.withName( async function (){} ) );

  /* */

  test.case = `Async Named Routine`;
  test.true( _.routine.is( async function a(){} ) );
  test.true( _.routine.like(async function a(){} ) );
  test.true( _.routine.isTrivial( async function a(){} ) );
  test.true( !_.routine.isSync( async function a(){} ) );
  test.true( _.routine.isAsync( async function a(){} ) );
  test.true( !_.routine.isGenerator( async function a(){} ) );
  test.true( !_.routine.isSyncGenerator( async function a(){} ) );
  test.true( !_.routine.isAsyncGenerator( async function a(){} ) );
  test.true( _.routine.withName( async function a(){} ) );


  /* */

  test.case = `Unnamed Generator`;
  test.true( !_.routine.is( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.like( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isTrivial( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isSync( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isAsync( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isGenerator( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isSyncGenerator( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isAsyncGenerator( function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.withName( function* () { yield 1; yield 2; yield 3; } ) );

  /* */

  test.case = `Named Generator`;
  test.true( !_.routine.is( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.like( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isTrivial( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isSync( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isAsync( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isGenerator( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isSyncGenerator( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isAsyncGenerator( function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.withName( function* abc() { yield 1; yield 2; yield 3; } ) );

  /* */

  test.case = `Unnamed async generator`;
  test.true( !_.routine.is( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.like( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isTrivial( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isSync( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isAsync( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isGenerator( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isSyncGenerator( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isAsyncGenerator( async function* () { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.withName( async function* () { yield 1; yield 2; yield 3; } ) );


  /* */

  test.case = `Named async generator`;
  test.true( !_.routine.is( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.like( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isTrivial( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isSync( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isAsync( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isGenerator( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( !_.routine.isSyncGenerator( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.isAsyncGenerator( async function* abc() { yield 1; yield 2; yield 3; } ) );
  test.true( _.routine.withName( async function* abc() { yield 1; yield 2; yield 3; } ) );

  /* */

  test.case = `Generator as an object property`;
  var SomeObj =
  {
    *[ Symbol.iterator ] ()
    {
      yield 'a';
      yield 'b';
    }
  }
  var src = SomeObj[ Symbol.iterator ];
  test.true( !_.routine.is( src ) );
  test.true( _.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( _.routine.isGenerator( src ) );
  test.true( _.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( _.routine.withName( src ) );

  /* */

  test.case = `Generator created using Generator Function constructor`;
  var GeneratorFunction = Object.getPrototypeOf( function* (){} ).constructor;
  var src = new GeneratorFunction( 'a', 'yield a * 2' );
  test.true( !_.routine.is( src ) );
  test.true( _.routine.like( src ) );
  test.true( !_.routine.isTrivial( src ) );
  test.true( !_.routine.isSync( src ) );
  test.true( !_.routine.isAsync( src ) );
  test.true( _.routine.isGenerator( src ) );
  test.true( _.routine.isSyncGenerator( src ) );
  test.true( !_.routine.isAsyncGenerator( src ) );
  test.true( _.routine.withName( src ) );
}

function is( test )
{

  test.case = 'without argument';
  var got = _.routine.is();
  test.identical( got, false );

  test.case = 'check null';
  var src = null;
  test.identical( _.routine.is( src ), false );
  test.identical( _.routine.like( src ), false );
  test.identical( _.routine.isSync( src ), false );
  test.identical( _.routine.isAsync( src ), false );
  test.identical( _.routine.isTrivial( src ), false );
  test.identical( _.routine.withName( src ), false );
  test.identical( _.routine.isGenerator( src ), false );
  test.identical( _.routine.isSyncGenerator( src ), false );
  test.identical( _.routine.isAsyncGenerator( src ), false );

  test.case = 'check undefined';
  var got = _.routine.is( undefined );
  test.identical( got, false );

  test.case = 'check _.nothing';
  var got = _.routine.is( _.nothing );
  test.identical( got, false );

  test.case = 'false';
  var got = _.routine.is( false );
  test.identical( got, false );

  test.case = 'NaN';
  var got = _.routine.is( NaN );
  test.identical( got, false );

  test.case = 'Symbol';
  var got = _.routine.is( Symbol( 'a' ) );
  test.identical( got, false );

  test.case = 'array';
  var got = _.routine.is( [] );
  test.identical( got, false );

  test.case = 'arguments array';
  var got = _.routine.is( _.argumentsArray.make( [] ) );
  test.identical( got, false );

  test.case = 'unroll';
  var got = _.routine.is( _.unroll.make( [] ) );
  test.identical( got, false );

  test.case = 'pure map';
  var got = _.routine.is( Object.create( null ) );
  test.identical( got, false );

  test.case = 'Set';
  var got = _.routine.is( new Set( [] ) );
  test.identical( got, false );

  test.case = 'Map';
  var got = _.routine.is( new HashMap( [] ) );
  test.identical( got, false );

  test.case = 'check BufferRaw';
  var got = _.routine.is( new BufferRaw() );
  test.identical( got, false );

  test.case = 'check BufferTyped';
  var got = _.routine.is( new U8x() );
  test.identical( got, false );

  test.case = 'number';
  var got = _.routine.is( 3 );
  test.identical( got, false );

  test.case = 'bigInt';
  var got = _.routine.is( 1n );
  test.identical( got, false );

  test.case = 'string';
  var got = _.routine.is( 'str' );
  test.identical( got, false );

  test.case = 'array';
  var got = _.routine.is( [ null ] );
  test.identical( got, false );

  test.case = 'map';
  var got = _.routine.is( { '' : null } );
  test.identical( got, false );

  test.case = 'check instance of constructor';
  function Constr(){ this.x = 1; return this };
  var src = new Constr();
  var got = _.routine.is( src );
  test.identical( got, false );

  test.case = 'check instance constructor';
  function Constr1(){ this.x = 1; return this };
  var src = new Constr1();
  var got = _.routine.is( src.constructor );
  test.identical( got, true );

  test.case = 'Object';
  var got = _.routine.is( Object );
  test.identical( got, true )

  test.case = 'arrow routine';
  var got = _.routine.is( () => {} );
  test.identical( got, true )

  test.case = 'unnamed routine';
  var got = _.routine.is( function (){} );
  test.identical( got, true )

  test.case = 'named routine';
  var got = _.routine.is( function a(){} );
  test.identical( got, true )

  test.case = 'async arrow routine';
  var got = _.routine.is( async () => {} );
  test.identical( got, true )

  test.case = 'async unnamed routine';
  var got = _.routine.is( async function (){} );
  test.identical( got, true )

  test.case = 'async named routine';
  var got = _.routine.is( async function a(){} );
  test.identical( got, true )

  test.case = `generator`;
  var src = function* abc()
  {
    yield 1;
    yield 2;
    yield 3;
  };
  test.identical( _.routine.is( src ), false );
  test.identical( _.routine.like( src ), true );
  test.identical( _.routine.isSync( src ), false );
  test.identical( _.routine.isAsync( src ), false );
  test.identical( _.routine.isTrivial( src ), false );
  test.identical( _.routine.withName( src ), true );
  test.identical( _.routine.isGenerator( src ), true );
  test.identical( _.routine.isSyncGenerator( src ), true );
  test.identical( _.routine.isAsyncGenerator( src ), false );

  test.case = `async generator`;
  var src = async function* abc()
  {
    yield 1;
    yield 2;
    yield 3;
  };
  test.identical( _.routine.is( src ), false );
  test.identical( _.routine.like( src ), true );
  test.identical( _.routine.isSync( src ), false );
  test.identical( _.routine.isAsync( src ), false );
  test.identical( _.routine.isTrivial( src ), false );
  test.identical( _.routine.withName( src ), true );
  test.identical( _.routine.isGenerator( src ), true );
  test.identical( _.routine.isSyncGenerator( src ), false );
  test.identical( _.routine.isAsyncGenerator( src ), true );

}

//

function like( test )
{

  test.case = 'without argument';
  var got = _.routine.like();
  test.identical( got, false );

  test.case = 'check null';
  var got = _.routine.like( null );
  test.identical( got, false );

  test.case = 'check undefined';
  var got = _.routine.like( undefined );
  test.identical( got, false );

  test.case = 'check _.nothing';
  var got = _.routine.like( _.nothing );
  test.identical( got, false );

  test.case = 'false';
  var got = _.routine.like( false );
  test.identical( got, false );

  test.case = 'NaN';
  var got = _.routine.like( NaN );
  test.identical( got, false );

  test.case = 'Symbol';
  var got = _.routine.like( Symbol( 'a' ) );
  test.identical( got, false );

  test.case = 'array';
  var got = _.routine.like( [] );
  test.identical( got, false );

  test.case = 'arguments array';
  var got = _.routine.like( _.argumentsArray.make( [] ) );
  test.identical( got, false );

  test.case = 'unroll';
  var got = _.routine.like( _.unroll.make( [] ) );
  test.identical( got, false );

  test.case = 'pure map';
  var got = _.routine.like( Object.create( null ) );
  test.identical( got, false );

  test.case = 'Set';
  var got = _.routine.like( new Set( [] ) );
  test.identical( got, false );

  test.case = 'Map';
  var got = _.routine.like( new HashMap( [] ) );
  test.identical( got, false );

  test.case = 'check BufferRaw';
  var got = _.routine.like( new BufferRaw() );
  test.identical( got, false );

  test.case = 'check BufferTyped';
  var got = _.routine.like( new U8x() );
  test.identical( got, false );

  test.case = 'number';
  var got = _.routine.like( 3 );
  test.identical( got, false );

  test.case = 'bigInt';
  var got = _.routine.like( 1n );
  test.identical( got, false );

  test.case = 'string';
  var got = _.routine.like( 'str' );
  test.identical( got, false );

  test.case = 'array';
  var got = _.routine.like( [ null ] );
  test.identical( got, false );

  test.case = 'map';
  var got = _.routine.like( { '' : null } );
  test.identical( got, false );

  test.case = 'check instance of constructor';
  function Constr(){ this.x = 1; return this };
  var src = new Constr();
  var got = _.routine.like( src );
  test.identical( got, false );

  test.case = 'check instance constructor';
  function Constr1(){ this.x = 1; return this };
  var src = new Constr1();
  var got = _.routine.like( src.constructor );
  test.identical( got, true );

  test.case = 'Object';
  var got = _.routine.like( Object );
  test.identical( got, true );

  test.case = 'arrow routine';
  var got = _.routine.like( () => {} );
  test.identical( got, true );

  test.case = 'unnamed routine';
  var got = _.routine.like( function (){} );
  test.identical( got, true );

  test.case = 'named routine';
  var got = _.routine.like( function a(){} );
  test.identical( got, true );

  test.case = 'async arrow routine';
  var got = _.routine.like( async () => {} );
  test.identical( got, true );

  test.case = 'async unnamed routine';
  var got = _.routine.like( async function (){} );
  test.identical( got, true )

  test.case = 'async named routine';
  var got = _.routine.like( async function a(){} );
  test.identical( got, true )
}

//

function isTrivial( test )
{

  var got = _.routine.isTrivial( 1 );
  test.identical( got, false )

  var got = _.routine.isTrivial( '' );
  test.identical( got, false )

  var got = _.routine.isTrivial( {} );
  test.identical( got, false )

  var got = _.routine.isTrivial( [] );
  test.identical( got, false )

  var got = _.routine.isTrivial( () => {} );
  test.identical( got, true )

  var got = _.routine.isTrivial( Object );
  test.identical( got, true )

  var got = _.routine.isTrivial( function () {} );
  test.identical( got, true )

  var got = _.routine.isTrivial( function a() {} );
  test.identical( got, true )

  var got = _.routine.isTrivial( async function () {} );
  test.identical( got, true )

  var got = _.routine.isTrivial( async () => {} );
  test.identical( got, true )

  var got = _.routine.isTrivial( async function a() {} );
  test.identical( got, true )

  function sync1(){}
  var got = _.routine.isTrivial( sync1 );
  test.identical( got, true )

  function sync2(){}
  sync2.map = {};
  var got = _.routine.isTrivial( sync2 );
  test.identical( got, true )

  function async1(){}
  var got = _.routine.isTrivial( async1 );
  test.identical( got, true )

  function async2(){}
  async2.map = {};
  var got = _.routine.isTrivial( async2 );
  test.identical( got, true )

  test.case = 'map';
  var src = Object.create( null );
  var got = _.routine.isTrivial( src );
  test.identical( got, false )

  test.case = 'prototyped';
  var src = Object.create( Object.create( null ) );
  var got = _.routine.isTrivial( src );
  test.identical( got, false )

}

//

function isSync( test )
{

  function sync1(){}
  function sync2(){}
  sync2.map = {};
  function async1(){}
  function async2(){}
  async2.map = {};

  var got = _.routine.isSync( 1 );
  test.identical( got, false )

  var got = _.routine.isSync( '' );
  test.identical( got, false )

  var got = _.routine.isSync( {} );
  test.identical( got, false )

  var got = _.routine.isSync( [] );
  test.identical( got, false )

  var got = _.routine.isSync( Object );
  test.identical( got, true )

  var got = _.routine.isSync( () => {} );
  test.identical( got, true )

  var got = _.routine.isSync( function () {} );
  test.identical( got, true )

  var got = _.routine.isSync( function a() {} );
  test.identical( got, true )

  var got = _.routine.isSync( async function () {} );
  test.identical( got, false )

  var got = _.routine.isSync( async () => {} );
  test.identical( got, false )

  var got = _.routine.isSync( async function a() {} );
  test.identical( got, false )

  var got = _.routine.is( sync1 );
  test.identical( got, true )

  var got = _.routine.is( sync2 );
  test.identical( got, true )

  var got = _.routine.is( async1 );
  test.identical( got, true )

  var got = _.routine.is( async2 );
  test.identical( got, true )

}

//

function isAsync( test )
{

  function sync1(){}
  function sync2(){}
  sync2.map = {};
  function async1(){}
  function async2(){}
  async2.map = {};

  var got = _.routine.isAsync( 1 );
  test.identical( got, false )

  var got = _.routine.isAsync( '' );
  test.identical( got, false )

  var got = _.routine.isAsync( {} );
  test.identical( got, false )

  var got = _.routine.isAsync( [] );
  test.identical( got, false )

  var got = _.routine.isAsync( () => {} );
  test.identical( got, false )

  var got = _.routine.isAsync( Object );
  test.identical( got, false )

  var got = _.routine.isAsync( function () {} );
  test.identical( got, false )

  var got = _.routine.isAsync( function a() {} );
  test.identical( got, false )

  var got = _.routine.isAsync( async function () {} );
  test.identical( got, true )

  var got = _.routine.isAsync( async () => {} );
  test.identical( got, true )

  var got = _.routine.isAsync( async function a() {} );
  test.identical( got, true )

  var got = _.routine.is( sync1 );
  test.identical( got, true )

  var got = _.routine.is( sync2 );
  test.identical( got, true )

  var got = _.routine.is( async1 );
  test.identical( got, true )

  var got = _.routine.is( async2 );
  test.identical( got, true )

}

//

function _join( test )
{
  var context3 = new contextConstructor3();
  var testParam1 = 2;
  var testParam2 = 4;
  var options1 =
  {
    sealing : false,
    routine : testFunction1,
    args : [ testParam2 ], // x
    extending : true
  };
  var options2 =
  {
    sealing : true,
    routine : testFunction2,
    args : [ testParam2 ], // x
    extending : true
  };

  var options3 =
  {
    sealing : false,
    routine : testFunction3,
    args : [ testParam2 ], // x
    context : context3,
    extending : true
  };
  var options4 =
  {
    sealing : false,
    routine : testFunction4,
    args : [ testParam2 ], // x
    context : context3,
    extending : true
  };

  var options5 =
  {
    sealing : true,
    routine : testFunction3,
    args : [ testParam1, testParam2 ], // x
    context : context3,
    extending : true
  };

  var wrongOpt1 =
  {
    sealing : true,
    routine : {},
    args : [ testParam1, testParam2 ], // x
    context : context3,
    extending : true
  };

  var wrongOpt2 =
  {
    sealing : true,
    routine : testFunction3,
    args : 'wrong', // x
    context : context3,
    extending : true
  };

  var expected1 = 6;
  var expected2 = undefined;
  var expected3 = 21;
  var expected5 = 21;

  test.case = 'simple function without context with arguments bind without seal : result check';
  var gotfn = _.routine._join( options1 );
  var got = gotfn( testParam1 );
  test.identical( got, expected1 );

  test.case = 'simple function without context and seal : context test';
  var gotfn = _.routine._join(options2);
  var got = gotfn( testParam1 );
  test.identical( got, expected2 );

  test.case = 'simple function with context and arguments : result check';
  var gotfn = _.routine._join(options3);
  var got = gotfn( testParam1 );
  test.identical( got, expected3 );

  test.case = 'simple function with context and arguments : context check';
  var gotfn = _.routine._join(options4);
  var got = gotfn( testParam1 );
  test.identical( got instanceof contextConstructor3, true );

  test.case = 'simple function with context and arguments : result check, seal == true ';
  var gotfn = _.routine._join(options5);
  var got = gotfn( testParam1 );
  test.identical( got, expected5 );

  test.case = 'simple function with context and arguments : result check, seal == true ';
  var gotfn = _.routine._join(options5);
  var got = gotfn( 0, 0 );
  test.identical( got, expected5 );

  test.case = 'extending';
  function srcRoutine(){}
  srcRoutine.defaults = { a : 10 };
  var gotfn = _.routine.join( undefined, srcRoutine, [] );
  test.identical( gotfn.defaults, srcRoutine.defaults );

  /**/

  if( !Config.debug )
  return;

  test.case = 'missed argument';
  test.shouldThrowErrorSync( function()
  {
    _.routine._join();
  });

  test.case = 'extra argument';
  test.shouldThrowErrorSync( function()
  {
    _.routine._join( options1, options2 );
  });

  test.case = 'passed non callable object';
  test.shouldThrowErrorSync( function()
  {
    _.routine._join( wrongOpt1 );
  });

  test.case = 'passed arguments as primitive value';
  test.shouldThrowErrorSync( function()
  {
    _.routine._join( wrongOpt2 );
  });

  function testFunction1( x, y )
  {
    return x + y;
  }

  function testFunction2( x, y )
  {
    return this;
  }

  function testFunction3( x, y )
  {
    return x + y + this.k;
  }

  function testFunction4( x, y )
  {
    return this;
  }

  function contextConstructor3()
  {
    this.k = 15
  }

  // var context3 = new contextConstructor3(); // xxx

};

//
//
// function routineBind( test )
// {
//
//   var testParam1 = 2,
//     testParam2 = 4,
//     expected1 = 6,
//     expected2 = undefined,
//     expected3 = 21;
//
//   test.case = 'simple function without context with arguments bind : result check';
//   var gotfn = _.routineBind( testFunction1, undefined, [ testParam2 ]);
//   var got = gotfn( testParam1 );
//   test.identical( got, expected1 );
//
//   test.case = 'simple function without context test';
//   var gotfn = _.routineBind(testFunction2, undefined, [ testParam2 ]);
//   var got = gotfn( testParam1 );
//   test.identical( got, expected2 );
//
//   test.case = 'simple function with context and arguments : result check';
//   var gotfn = _.routineBind(testFunction3, context3, [ testParam2 ]);
//   var got = gotfn( testParam1 );
//   test.identical( got, expected3 );
//
//   test.case = 'simple function with context and arguments : context check';
//   var gotfn = _.routineBind(testFunction4, context3, [ testParam2 ]);
//   var got = gotfn( testParam1 );
//   test.identical( got instanceof contextConstructor3, true );
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'missed argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.routineBind();
//   });
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.routineBind( testFunction4, context3, [ testParam2 ], [ testParam1 ] );
//   });
//
//   test.case = 'passed non callable object';
//   test.shouldThrowErrorSync( function()
//   {
//     _.routineBind( {}, context3, [ testParam2 ] );
//   });
//
//   test.case = 'passed arguments as primitive value';
//   test.shouldThrowErrorSync( function()
//   {
//     _.routineBind( testFunction4, context3, testParam2 );
//   });
//
// };

//

function constructorJoin( test )
{
  function srcRoutine()
  {
    var result =
    {
      context : this,
      args : Array.prototype.slice.call( arguments )
    }
    return result;
  }
  srcRoutine.prop = true;

  /* */

  test.case = 'without args';
  var got = _.constructorJoin( srcRoutine );
  test.true( _.routine.is( got ) );
  var result = got();
  test.identical( _.props.keys( srcRoutine ), [ 'prop' ] )
  test.identical( _.props.keys( got ), [] );
  test.identical( result.args, [] );
  test.identical( result.context, srcRoutine );
  test.false( result.context instanceof srcRoutine );

  test.case = 'args - undefined';
  var got = _.constructorJoin( srcRoutine, undefined );
  test.true( _.routine.is( got ) );
  var result = got();
  test.identical( _.props.keys( srcRoutine ), [ 'prop' ] )
  test.identical( _.props.keys( got ), [] );
  test.identical( result.args, [] );
  test.identical( result.context, srcRoutine );
  test.false( result.context instanceof srcRoutine );

  test.case = 'args - null';
  var got = _.constructorJoin( srcRoutine, null );
  test.true( _.routine.is( got ) );
  var result = got();
  test.identical( _.props.keys( srcRoutine ), [ 'prop' ] )
  test.identical( _.props.keys( got ), [] );
  test.identical( result.args, [] );
  test.identical( result.context, srcRoutine );
  test.false( result.context instanceof srcRoutine );

  test.case = 'args - empty array';
  var args = [];
  var got = _.constructorJoin( srcRoutine, args );
  test.true( _.routine.is( got ) );
  var result = new got();
  test.identical( _.props.keys( srcRoutine ), [ 'prop' ] )
  test.identical( _.props.keys( got ), [] );
  test.identical( result.args, args );
  test.notIdentical( result.context, srcRoutine );
  test.true( result.context instanceof srcRoutine );

  test.case = 'args - array with map, returned routine exexute without arguments';
  var args = [ { a : 1 } ];
  var got = _.constructorJoin( srcRoutine, args );
  test.true( _.routine.is( got ) );
  var result = got();
  test.identical( _.props.keys( srcRoutine ), [ 'prop' ] )
  test.identical( _.props.keys( got ), [] );
  test.identical( result.args, args );
  test.identical( result.context, srcRoutine );
  test.false( result.context instanceof srcRoutine );

  test.case = 'args - array with map, returned routine exexute with arguments';
  var args = [ { a : 1 } ];
  var got = _.constructorJoin( srcRoutine, args );
  test.true( _.routine.is( got ) );
  var result = got({ b : 1 });
  test.identical( _.props.keys( srcRoutine ), [ 'prop' ] )
  test.identical( _.props.keys( got ), [] );
  test.identical( result.args, [ { a : 1 }, { b : 1 } ] );
  test.identical( result.context, srcRoutine );
  test.false( result.context instanceof srcRoutine );

  test.case = 'Array contructor, args = U8x buffer, execute without arguments';
  var args = new U8x( [ 1, 2, 3, 4 ] );
  var got = _.constructorJoin( Array, args );
  test.true( _.routine.is( got ) );
  var result = new got();
  test.identical( _.props.keys( got ), [] );
  test.identical( result, [ 1, 2, 3, 4 ] );

  test.case = 'Array contructor, args = U8x buffer, exexute with number';
  var args = new U8x( [ 1, 2, 3, 4 ] );
  var got = _.constructorJoin( Array, args );
  test.true( _.routine.is( got ) );
  var result = new got( 1 );
  test.identical( _.props.keys( got ), [] );
  test.identical( result, [ 1, 2, 3, 4, 1 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.constructorJoin() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.constructorJoin( Array, [ 1, 2 ], [ 1, 2 ] ) );

  test.case = 'wrong type of routine';
  test.shouldThrowErrorSync( () => _.constructorJoin( [], [] ) );

  test.case = 'wrong type of args';
  test.shouldThrowErrorSync( () => _.constructorJoin( srcRoutine, srcRoutine ) );
}

//

function join( test )
{
  function testFunction1( x, y ){ return this }
  function testFunction2( x, y ){ return x + y }
  function testFunction3( x, y ){ return x + y + ( this !== undefined ? this.k : 1 ) }
  function Constr(){ this.k = 15; return this }
  var context = new Constr();

  /* - */

  test.open( 'context - undefined, args - undefined' );

  test.case = 'named function without context, check context';
  var gotfn = _.routine.join( undefined, testFunction1, undefined );
  test.identical( gotfn.name, 'testFunction1' );
  test.identical( gotfn.originalRoutine, testFunction1 );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, undefined );

  test.case = 'named function with arguments bind : result check';
  var gotfn = _.routine.join( undefined, testFunction2, undefined );
  test.identical( gotfn.name, 'testFunction2' );
  test.identical( gotfn.originalRoutine, testFunction2 );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 5 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( undefined, testFunction3, undefined );
  test.identical( gotfn.name, 'testFunction3' );
  test.identical( gotfn.originalRoutine, testFunction3 );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 6 );

  test.case = 'unnamed function without context, check context';
  var gotfn = _.routine.join( undefined, ( x, y ) => x + y, undefined );
  test.identical( gotfn.name, '' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => x + y )( 2, 3 ) );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 5 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( undefined, ( x, y ) => Math.pow( x, y ), undefined );
  test.identical( gotfn.name, '' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => Math.pow( x, y ) )( 2, 3 ) );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 8 );

  test.close( 'context - undefined, args - undefined' );

  /* - */

  test.open( 'context - undefined, args - long' );

  test.case = 'named function without context, check context';
  var gotfn = _.routine.join( undefined, testFunction1, [ 4 ] );
  test.identical( gotfn.name, 'testFunction1' );
  test.identical( gotfn.originalRoutine, testFunction1 );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got, undefined );

  test.case = 'named function with arguments bind : result check';
  var gotfn = _.routine.join( undefined, testFunction2, [ 4 ] );
  test.identical( gotfn.name, 'testFunction2' );
  test.identical( gotfn.originalRoutine, testFunction2 );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got, 6 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( undefined, testFunction3, [ 4 ] );
  test.identical( gotfn.name, 'testFunction3' );
  test.identical( gotfn.originalRoutine, testFunction3 );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 5 );
  test.identical( got, 10 );

  test.case = 'unnamed function without context, check context';
  var gotfn = _.routine.join( undefined, ( x, y ) => x + y, [ 4 ] );
  test.identical( gotfn.name, '__joinedArguments' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => x + y )( 2, 3 ) );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got, 6 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( undefined, ( x, y ) => Math.pow( x, y ), [ 4 ] );
  test.identical( gotfn.name, '__joinedArguments' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => Math.pow( x, y ) )( 2, 3 ) );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got, 16 );

  test.close( 'context - undefined, args - long' );

  /* - */

  test.open( 'context - exists, args - undefined' );

  test.case = 'named function without context, check context';
  var gotfn = _.routine.join( context, testFunction1, undefined );
  test.identical( gotfn.name, 'bound testFunction1' );
  test.identical( gotfn.originalRoutine, testFunction1 );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got instanceof Constr, true );

  test.case = 'named function with arguments bind : result check';
  var gotfn = _.routine.join( context, testFunction2, undefined );
  test.identical( gotfn.name, 'bound testFunction2' );
  test.identical( gotfn.originalRoutine, testFunction2 );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 5 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( context, testFunction3, undefined );
  test.identical( gotfn.name, 'bound testFunction3' );
  test.identical( gotfn.originalRoutine, testFunction3 );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 20 );

  test.case = 'unnamed function without context, check context';
  var gotfn = _.routine.join( context, ( x, y ) => x + y, undefined );
  test.identical( gotfn.name, 'bound ' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => x + y )( 2, 3 ) );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 5 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( context, ( x, y ) => Math.pow( x, y ), undefined );
  test.identical( gotfn.name, 'bound ' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => Math.pow( x, y ) )( 2, 3 ) );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, undefined );
  var got = gotfn( 2, 3 );
  test.identical( got, 8 );

  test.close( 'context - exists, args - undefined' );

  /* - */

  test.open( 'context - exists, args - long' );

  test.case = 'named function without context, check context';
  var gotfn = _.routine.join( context, testFunction1, [ 4 ] );
  test.identical( gotfn.name, 'bound testFunction1' );
  test.identical( gotfn.originalRoutine, testFunction1 );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got instanceof Constr, true );

  test.case = 'named function with arguments bind : result check';
  var gotfn = _.routine.join( context, testFunction2, [ 4 ] );
  test.identical( gotfn.name, 'bound testFunction2' );
  test.identical( gotfn.originalRoutine, testFunction2 );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got, 6 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( context, testFunction3, [ 4 ] );
  test.identical( gotfn.name, 'bound testFunction3' );
  test.identical( gotfn.originalRoutine, testFunction3 );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 5 );
  test.identical( got, 24 );

  test.case = 'unnamed function without context, check context';
  var gotfn = _.routine.join( context, ( x, y ) => x + y, [ 4 ] );
  test.identical( gotfn.name, 'bound ' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => x + y )( 2, 3 ) );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got, 6 );

  test.case = 'unnamed function with arguments bind : result check';
  var gotfn = _.routine.join( context, ( x, y ) => Math.pow( x, y ), [ 4 ] );
  test.identical( gotfn.name, 'bound ' );
  test.identical( gotfn.originalRoutine( 2, 3 ), ( ( x, y ) => Math.pow( x, y ) )( 2, 3 ) );
  test.identical( gotfn.boundContext, context );
  test.identical( gotfn.boundArguments, [ 4 ] );
  var got = gotfn( 2 );
  test.identical( got, 16 );

  test.close( 'context - exists, args - long' );

  /* - */

  test.case = 'extending'
  var srcRoutine = () => {};
  srcRoutine.defaults = { a : 10 };
  var gotfn = _.routine.join( undefined, srcRoutine, [] );
  test.identical( gotfn.defaults, srcRoutine.defaults );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.routine.join() );

  test.case = 'extra argument';
  test.shouldThrowErrorSync( () => _.routine.join( undefined, testFunction4, [ 4 ], [ 2 ] ) );

  test.case = 'wrong type of routine';
  test.shouldThrowErrorSync( () => _.routine.join( undefined, 1, [ 4 ] ) );
  test.shouldThrowErrorSync( () => _.routine.join( undefined, {}, [ 4 ] ) );

  test.case = 'wrong type of args';
  test.shouldThrowErrorSync( () => _.routine.join( undefined, testFunction4, 4 ) );
  test.shouldThrowErrorSync( () => _.routine.join( undefined, testFunction4, null ) );
}

//

function seal( test )
{

  var context3 = new contextConstructor3();
  var testParam1 = 2;
  var testParam2 = 4;
  var expected1 = 6;
  var expected2 = undefined;
  var expected3 = 21;

  test.case = 'simple function with seal arguments : result check';
  var gotfn = _.routine.seal(undefined, testFunction1, [ testParam1, testParam2 ]);
  var got = gotfn( testParam1 );
  test.identical( got, expected1 );

  test.case = 'simple function with seal arguments : context check';
  var gotfn = _.routine.seal(undefined, testFunction2, [ testParam1, testParam2 ]);
  var got = gotfn( testParam1 );
  test.identical( got, expected2 );

  test.case = 'simple function with seal context and arguments : result check';
  var gotfn = _.routine.seal(context3, testFunction3, [ testParam1, testParam2 ]);
  var got = gotfn( testParam1 );
  test.identical( got, expected3 );

  test.case = 'simple function with seal context and arguments : context check';
  var gotfn = _.routine.seal(context3, testFunction4, [ testParam1, testParam2 ]);
  var got = gotfn( testParam1 );
  test.identical( got instanceof contextConstructor3, true );

  test.case = 'simple function with seal context and arguments : result check';
  var gotfn = _.routine.seal(context3, testFunction3, [ testParam1, testParam2 ]);
  var got = gotfn( 0, 0 );
  test.identical( got, expected3 );

  test.case = 'extending';
  function srcRoutine(){}
  srcRoutine.defaults = { a : 10 };
  var gotfn = _.routine.join( undefined, srcRoutine, [] );
  test.identical( gotfn.defaults, srcRoutine.defaults );

  if( !Config.debug )
  return;

  test.case = 'missed argument';
  test.shouldThrowErrorSync( function()
  {
    _.routine.seal();
  });

  test.case = 'extra argument';
  test.shouldThrowErrorSync( function()
  {
    _.routine.seal( context3, testFunction4, [ testParam2 ], [ testParam1 ] );
  });

  test.case = 'passed non callable object';
  test.shouldThrowErrorSync( function()
  {
    _.routine.seal( context3, {}, [ testParam1, testParam2 ] );
  });

  test.case = 'passed arguments as primitive value';
  test.shouldThrowErrorSync( function()
  {
    _.routine.seal( context3, testFunction4, testParam2 );
  });


  function testFunction1( x, y )
  {
    return x + y;
  }

  function testFunction2( x, y )
  {
    return this;
  }

  function testFunction3( x, y )
  {
    return x + y + this.k;
  }

  function testFunction4( x, y )
  {
    return this;
  }

  function contextConstructor3()
  {
    this.k = 15
  }

}

// --
// options
// --

function optionsWithoutUndefined( test )
{

  test.case = 'args - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = {};
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 0, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [];
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.identical( options, [] );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [ {} ];
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = {};
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 0, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [];
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, {} );
  test.identical( options, [] );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [ {} ];
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, {} );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - empty map, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = {};
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 0, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [];
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options, [] );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ {} ];
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = {};
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 0, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [];
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options, [] );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - empty map, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ {} ];
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : undefined, b : undefined };
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : undefined, b : undefined } ];
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : undefined, b : undefined };
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : undefined, b : undefined } ];
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefine JS value, but not undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.optionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with defined values, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  test.case = 'regexp in defaults';
  var testRoutine = () => true;
  testRoutine.defaults = { a : /ab/ };
  var options = {};
  var got = _.routine.optionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : /ab/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'regexp in defaults';
  var defaults = { a : /ab/ };
  var options = {};
  var got = _.routine.optionsWithoutUndefined( { defaults }, options );
  test.identical( got, { a : /ab/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined() );

  test.case = 'not enough arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine ) );

  test.case = 'extra arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, {}, {}, {} ) );

  test.case = 'wrong type of routine';
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( 'wrong', {}, {} ) );

  test.case = 'wrong type of args';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, 'wrong', {} ) );
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, [ 'wrong' ], {} ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, {}, 'wrong' ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  testRoutine.defaults = 'wrong';
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, {} ) );

  test.case = 'args.length > 1';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, [ {}, {} ], {} ) );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  var msg = 'Routine "" does not expect options: "unknown", "b"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( { defaults : {} }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( { defaults : {} }, [ { unknown : true, b : 1 } ] ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( { defaults : { known : 1 } }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( { defaults : { known : 1 } }, [ { unknown : true, b : 1 } ] ), errCallback );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  testRoutine.defaults = { known : 1 };
  var msg = 'Routine "testRoutine" does not expect options: "unknown"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, { unknown : true } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, [ { unknown : true } ] ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : { objectLike : true } };
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( { defaults }, {} ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  testRoutine.defaults = { known : { objectLike : true } };
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, {} ), errCallback );

  test.case = 'defaults has value `undefined`';
  var testRoutine = () => true;
  var msg = 'Options map for routine "" should have no undefined fields, but it does have "known"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : undefined };
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( { defaults }, {} ), errCallback );

  test.case = 'defaults has value `undefined`';
  var testRoutine = () => true;
  testRoutine.defaults = { known : undefined };
  var msg = 'Options map for routine "testRoutine" should have no undefined fields, but it does have "known"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithoutUndefined( testRoutine, {} ), errCallback );

}

//

function optionsWithUndefined( test )
{
  test.case = 'args - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = {};
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 0, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [];
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.identical( options, [] );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [ {} ];
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = {};
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 0, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [];
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, {} );
  test.identical( options, [] );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [ {} ];
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, {} );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - empty map, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = {};
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 0, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [];
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options, [] );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ {} ];
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = {};
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 0, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [];
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options, [] );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - empty map, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ {} ];
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : undefined, b : undefined };
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : undefined, b : undefined } ];
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : undefined, b : undefined };
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : undefined, b : undefined } ];
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefine JS value, but not undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.optionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with defined values, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  test.case = 'undefined default. map';
  var defaults = { known : undefined };
  var options = {};
  var got = _.routine.optionsWithUndefined( { defaults }, options );
  test.identical( got, { known : undefined } );
  test.true( got === options );

  test.case = 'undefined default. routine';
  var testRoutine = () => true;
  testRoutine.defaults = { known : undefined };
  var options = {};
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { known : undefined } );
  test.true( got === options );

  test.case = 'regexp in defaults';
  var testRoutine = () => true;
  testRoutine.defaults = { a : /ab/ };
  var options = {};
  var got = _.routine.optionsWithUndefined( testRoutine, options );
  test.identical( got, { a : /ab/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'regexp in defaults';
  var defaults = { a : /ab/ };
  var options = {};
  var got = _.routine.optionsWithUndefined( { defaults }, options );
  test.identical( got, { a : /ab/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined() );

  test.case = 'not enough arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine ) );

  test.case = 'extra arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, {}, {}, {} ) );

  test.case = 'wrong type of routine';
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( 'wrong', {}, {} ) );

  test.case = 'wrong type of args';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, 'wrong', {} ) );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, [ 'wrong' ], {} ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, {}, 'wrong' ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  testRoutine.defaults = 'wrong';
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, {} ) );

  test.case = 'args.length > 1';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, [ {}, {} ], {} ) );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  var msg = 'Routine "" does not expect options: "unknown", "b"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( { defaults : {} }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( { defaults : {} }, [ { unknown : true, b : 1 } ] ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( { defaults : { known : 1 } }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( { defaults : { known : 1 } }, [ { unknown : true, b : 1 } ] ), errCallback );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  testRoutine.defaults = { known : 1 };
  var msg = 'Routine "testRoutine" does not expect options: "unknown"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, { unknown : true } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, [ { unknown : true } ] ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : { objectLike : true } };
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( { defaults }, {} ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  testRoutine.defaults = { known : { objectLike : true } };
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefined( testRoutine, {} ), errCallback );

}

//

function optionsWithUndefinedTollerant( test )
{
  test.case = 'args - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 0, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [];
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, {} );
  test.identical( options, [] );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [ {} ];
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, {} );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 0, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [];
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, {} );
  test.identical( options, [] );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [ {} ];
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, {} );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - empty map, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 0, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [];
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options, [] );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ {} ];
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 0, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [];
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options, [] );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - empty map, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ {} ];
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : null, b : 1 } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : undefined, b : undefined };
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : undefined, b : undefined } ];
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : undefined, b : undefined };
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : undefined, b : undefined } ];
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : undefined, b : undefined } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefine JS value, but not undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.optionsWithUndefinedTollerant( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with defined values, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  test.case = 'undefined default. map';
  var defaults = { known : undefined };
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( { defaults }, options );
  test.identical( got, { known : undefined } );
  test.true( got === options );

  test.case = 'undefined default. routine';
  var testRoutine = () => true;
  testRoutine.defaults = { known : undefined };
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { known : undefined } );
  test.true( got === options );

  test.case = 'regexp in defaults';
  var testRoutine = () => true;
  testRoutine.defaults = { a : /ab/ };
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.identical( got, { a : /ab/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'regexp in defaults';
  var defaults = { a : /ab/ };
  var options = {};
  var got = _.routine.optionsWithUndefinedTollerant( { defaults }, options );
  test.identical( got, { a : /ab/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  var options = {};
  var defaults = { known : { objectLike : true } };
  var got = _.routine.optionsWithUndefinedTollerant( { defaults }, options );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  var options = {};
  testRoutine.defaults = { known : { objectLike : true } };
  var got = _.routine.optionsWithUndefinedTollerant( testRoutine, options );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant() );

  test.case = 'not enough arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine ) );

  test.case = 'extra arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, {}, {}, {} ) );

  test.case = 'wrong type of routine';
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( 'wrong', {}, {} ) );

  test.case = 'wrong type of args';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, 'wrong', {} ) );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, [ 'wrong' ], {} ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, {}, 'wrong' ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  testRoutine.defaults = 'wrong';
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, {} ) );

  test.case = 'args.length > 1';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, [ {}, {} ], {} ) );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  var msg = 'Routine "" does not expect options: "unknown", "b"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( { defaults : {} }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( { defaults : {} }, [ { unknown : true, b : 1 } ] ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( { defaults : { known : 1 } }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( { defaults : { known : 1 } }, [ { unknown : true, b : 1 } ] ), errCallback );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  testRoutine.defaults = { known : 1 };
  var msg = 'Routine "testRoutine" does not expect options: "unknown"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, { unknown : true } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.optionsWithUndefinedTollerant( testRoutine, [ { unknown : true } ] ), errCallback );

}

//

function assertOptionsWithoutUndefined( test )
{
  test.case = 'args - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = {};
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [ {} ];
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = {};
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [ {} ];
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, {} );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  /* */

  test.case = 'args - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefine JS value, but not undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with defined values, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with defined values, defaults - map with undefined';
  var testRoutine = () => true;
  var defaults = { a : undefined, b : undefined };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with undefined';
  var testRoutine = () => true;
  var defaults = { a : undefined, b : undefined };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with defined values, routine.defaults - map with undefined';
  var testRoutine = () => true;
  testRoutine.defaults = { a : undefined, b : undefined };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with undefined';
  var testRoutine = () => true;
  testRoutine.defaults = { a : undefined, b : undefined };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  test.case = 'regexp in defaults';
  var testRoutine = () => true;
  testRoutine.defaults = { a : /ab/ };
  var options = { a : /cd/ };
  var got = _.routine.assertOptionsWithoutUndefined( testRoutine, options );
  test.identical( got, { a : /cd/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'regexp in defaults';
  var defaults = { a : /ab/ };
  var options = { a : /cd/ };
  var got = _.routine.assertOptionsWithoutUndefined( { defaults }, options );
  test.identical( got, { a : /cd/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined() );

  test.case = 'not enough arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine ) );

  test.case = 'extra arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, {}, {}, {} ) );

  test.case = 'wrong type of routine';
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( 'wrong', {}, {} ) );

  test.case = 'wrong type of args';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, 'wrong', {} ) );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, [ 'wrong' ], {} ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, {}, 'wrong' ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  testRoutine.defaults = 'wrong';
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, {} ) );

  test.case = 'args.length > 1';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, [ {}, {} ], {} ) );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  var msg = 'Routine "" does not expect options: "unknown", "b"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults : {} }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults : {} }, [ { unknown : true, b : 1 } ] ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults : { known : 1 } }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults : { known : 1 } }, [ { unknown : true, b : 1 } ] ), errCallback );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  testRoutine.defaults = { known : 1 };
  var msg = 'Routine "testRoutine" does not expect options: "unknown"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, { unknown : true } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, [ { unknown : true } ] ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : { objectLike : true } };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults }, {} ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  testRoutine.defaults = { known : { objectLike : true } };
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, {} ), errCallback );

  test.case = 'defaults has value `undefined`';
  var testRoutine = () => true;
  var msg = 'Options map for routine "" should have no undefined fields, but it does have "known"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : undefined };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults }, { known : undefined } ), errCallback );

  test.case = 'defaults has value `undefined`';
  var testRoutine = () => true;
  testRoutine.defaults = { known : undefined };
  var msg = 'Options map for routine "testRoutine" should have no undefined fields, but it does have "known"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, { known : undefined } ), errCallback );

  /* */

  test.case = 'does not have option';
  var testRoutine = () => true;
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : 3 };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults }, {} ), errCallback );

  test.case = 'does not have option';
  var testRoutine = () => true;
  testRoutine.defaults = { known : 3 };
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, {} ), errCallback );

  test.case = 'does not have option which is udnefined';
  var testRoutine = () => true;
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : undefined };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults }, {} ), errCallback );

  test.case = 'does not have option which is udnefined';
  var testRoutine = () => true;
  testRoutine.defaults = { known : undefined };
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, {} ), errCallback );

  test.case = 'defaults with non-primitive';
  var testRoutine = () => true;
  testRoutine.defaults = { a : { c : 'c' }, b : [ 'b' ] };
  var options = { a : true, b : 'b' };
  var msg = 'Defaults map should have only primitive elements, but option::a is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( testRoutine, options ), errCallback );

  test.case = 'defaults with non-primitive';
  var testRoutine = () => true;
  var defaults = { a : { c : 'c' }, b : [ 'b' ] };
  var options = [ { a : true, b : 'b' } ];
  var msg = 'Defaults map should have only primitive elements, but option::a is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithoutUndefined( { defaults : defaults }, options ), errCallback );

}

//

function assertOptionsWithUndefined( test )
{
  test.case = 'args - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = {};
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, defaults - empty map';
  var testRoutine = () => true;
  var defaults = {};
  var options = [ {} ];
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, {} );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - empty map, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = {};
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, {} );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - empty map, routine.defaults - empty map';
  var testRoutine = () => true;
  testRoutine.defaults = {};
  var options = [ {} ];
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, {} );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  /* */

  test.case = 'args - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with undefine JS value, but not undefined, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : 0, b : '' };
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with undefine JS value, but not undefined, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : 0, b : '' } ];
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { a : 0, b : '' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  var defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with defined values, routine.defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with options';
  var testRoutine = () => true;
  testRoutine.defaults = { a : null, b : 1 };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'args - map with defined values, defaults - map with undefined';
  var testRoutine = () => true;
  var defaults = { a : undefined, b : undefined };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with undefined';
  var testRoutine = () => true;
  var defaults = { a : undefined, b : undefined };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithUndefined( { defaults : defaults }, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== defaults );

  test.case = 'args - map with defined values, routine.defaults - map with undefined';
  var testRoutine = () => true;
  testRoutine.defaults = { a : undefined, b : undefined };
  var options = { a : true, b : 'b' };
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'args.length === 1 - map with defined values, defaults - map with undefined';
  var testRoutine = () => true;
  testRoutine.defaults = { a : undefined, b : undefined };
  var options = [ { a : true, b : 'b' } ];
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { a : true, b : 'b' } );
  test.identical( options.length, 1 );
  test.true( got === options[ 0 ] );
  test.true( got !== testRoutine.defaults );

  /* */

  test.case = 'undefined default. map';
  var defaults = { known : undefined };
  var options = { known : undefined };
  var got = _.routine.assertOptionsWithUndefined( { defaults }, options );
  test.identical( got, { known : undefined } );
  test.true( got === options );

  test.case = 'undefined default. routine';
  var testRoutine = () => true;
  testRoutine.defaults = { known : undefined };
  var options = { known : undefined };
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { known : undefined } );
  test.true( got === options );

  test.case = 'regexp in defaults';
  var testRoutine = () => true;
  testRoutine.defaults = { a : /ab/ };
  var options = { a : /cd/ };
  var got = _.routine.assertOptionsWithUndefined( testRoutine, options );
  test.identical( got, { a : /cd/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  test.case = 'regexp in defaults';
  var defaults = { a : /ab/ };
  var options = { a : /cd/ };
  var got = _.routine.assertOptionsWithUndefined( { defaults }, options );
  test.identical( got, { a : /cd/ } );
  test.true( got === options );
  test.true( got !== testRoutine.defaults );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined() );

  test.case = 'not enough arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine ) );

  test.case = 'extra arguments';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, {}, {}, {} ) );

  test.case = 'wrong type of routine';
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( 'wrong', {}, {} ) );

  test.case = 'wrong type of args';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, 'wrong', {} ) );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, [ 'wrong' ], {} ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, {}, 'wrong' ) );

  test.case = 'wrong type of defaults';
  var testRoutine = () => true;
  testRoutine.defaults = 'wrong';
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, {} ) );

  test.case = 'args.length > 1';
  var testRoutine = () => true;
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, [ {}, {} ], {} ) );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  var msg = 'Routine "" does not expect options: "unknown", "b"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults : {} }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults : {} }, [ { unknown : true, b : 1 } ] ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults : { known : 1 } }, { unknown : true, b : 1 } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults : { known : 1 } }, [ { unknown : true, b : 1 } ] ), errCallback );

  test.case = 'options has unknown options';
  var testRoutine = () => true;
  testRoutine.defaults = { known : 1 };
  var msg = 'Routine "testRoutine" does not expect options: "unknown"';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, { unknown : true } ), errCallback );
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, [ { unknown : true } ] ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : { objectLike : true } };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults }, {} ), errCallback );

  test.case = 'defaults has objectLike value';
  var testRoutine = () => true;
  testRoutine.defaults = { known : { objectLike : true } };
  var msg = 'Defaults map should have only primitive elements, but option::known is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, {} ), errCallback );

  test.case = 'does not have option';
  var testRoutine = () => true;
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : 3 };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults }, {} ), errCallback );

  test.case = 'does not have option';
  var testRoutine = () => true;
  testRoutine.defaults = { known : 3 };
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, {} ), errCallback );

  test.case = 'does not have option which is udnefined';
  var testRoutine = () => true;
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  var defaults = { known : undefined };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults }, {} ), errCallback );

  test.case = 'does not have option which is udnefined';
  var testRoutine = () => true;
  testRoutine.defaults = { known : undefined };
  var msg = 'Options map does not have option::known';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, {} ), errCallback );

  test.case = 'defaults with non-primitive';
  var testRoutine = () => true;
  testRoutine.defaults = { a : { c : 'c' }, b : [ 'b' ] };
  var options = { a : true, b : 'b' };
  var msg = 'Defaults map should have only primitive elements, but option::a is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( testRoutine, options ), errCallback );

  test.case = 'defaults with non-primitive';
  var testRoutine = () => true;
  var defaults = { a : { c : 'c' }, b : [ 'b' ] };
  var options = [ { a : true, b : 'b' } ];
  var msg = 'Defaults map should have only primitive elements, but option::a is Map.polluted';
  var errCallback = ( err, arg ) =>
  {
    test.identical( arg, undefined );
    test.true( _.error.is( err ) );
    test.equivalent( err.originalMessage, msg );
  };
  test.shouldThrowErrorSync( () => _.routine.assertOptionsWithUndefined( { defaults : defaults }, options ), errCallback );

}

// --
//
// --



//

/* qqq : for Dmytro : split on test routines by number of arguments and extend */
/* qqq : for Dmytro : write test routines for supplement */
/* qqq : for Dmytro : write test for each strategy ( with subroutine act ) */

function routineExtendWithDstIsNull( test )
{
  act({ method : 'extendCloning' });
  act({ method : 'extendInheriting' });
  act({ method : 'extendReplacing' });

  /* */

  function act( env )
  {
    test.case = `${__.entity.exportStringSolo( env )}, srcs is routine with no props`;
    var r = ( arg ) => arg;
    var got = _.routine[ env.method ]( null, r );
    test.true( _.routine.is( got ) );
    test.identical( got.name, r.name );
    test.identical( got( 'str' ), r( 'str' ) );
    test.identical( _.props.keys( got ), [] );
    test.identical( got.originalRoutine, r );
    test.notIdentical( got.toString(), r.toString() );

    test.case = `${__.entity.exportStringSolo( env )}, srcs is routine with its own props`;
    var r = ( arg ) => arg;
    r.original = 0;
    var got = _.routine[ env.method ]( null, r );
    test.true( _.routine.is( got ) );
    test.identical( got.name, r.name );
    test.identical( got( 'str' ), r( 'str' ) );
    test.identical( _.props.keys( got ), [ 'original' ] );
    test.identical( got.original, 0 );
    test.identical( got.originalRoutine, r );
    test.notIdentical( got.toString(), r.toString() );

    test.case = `${__.entity.exportStringSolo( env )}, srcs is vector with routine and map with properties, first is routine`;
    var r1 = ( arg ) => arg;
    r1.original1 = 0;
    var r2 = Object.create( null );
    r2.original2 = 1;
    var got = _.routine[ env.method ]( null, r1, r2 );
    test.true( _.routine.is( got ) );
    test.identical( _.props.keys( got ), [ 'original1', 'original2' ] );
    test.identical( got.original1, 0 );
    test.identical( got.original2, 1 );
    test.identical( got.originalRoutine, r1 );

    test.case = `${__.entity.exportStringSolo( env )}, srcs is routine with body`;
    var rb = ( arg ) => arg;
    rb.defaults = { src : null };
    var r = () => {};
    r.body = rb;
    var got = _.routine[ env.method ]( null, r );
    test.true( _.routine.is( got ) );
    test.identical( _.props.keys( got ), [ 'defaults', 'head', 'body' ] );
    test.identical( got.originalRoutine, undefined );
    test.identical( r({ src : {} }), undefined );
    test.identical( got({ src : {} }), { src : {} } );

    test.case = `${__.entity.exportStringSolo( env )}, srcs is routine with head and body`;
    var rh = ( r, args ) => args[ 0 ];
    var rb = ( arg ) => arg;
    rb.defaults = { src : null };
    var r = () => {};
    r.head = rh;
    r.body = rb;
    var got = _.routine[ env.method ]( null, r );
    test.true( _.routine.is( got ) );
    test.identical( _.props.keys( got ), [ 'defaults', 'head', 'body' ] );
    test.identical( got.originalRoutine, undefined );
    test.identical( r( 'arg' ), undefined );
    test.identical( got( 'arg' ), 'arg' );

    test.case = `${__.entity.exportStringSolo( env )}, srcs is routine with head, body and tail`;
    var rh = ( r, args ) => args[ 0 ];
    var rb = ( arg ) => arg;
    rb.defaults = { src : null };
    var rt = ( arg ) => arg + 1;
    var r = () => {};
    r.head = rh;
    r.body = rb;
    r.tail = rt;
    var got = _.routine[ env.method ]( null, r );
    test.true( _.routine.is( got ) );
    test.identical( _.props.keys( got ), [ 'defaults', 'head', 'body', 'tail' ] );
    test.identical( got.originalRoutine, undefined );
    test.identical( r( 'arg' ), undefined );
    test.identical( got( 'arg' ), 'arg1' );

    test.case = `${__.entity.exportStringSolo( env )}, srcs is vector with routine ( head, body and tail ) and map`;
    var rh = ( r, args ) => args[ 0 ];
    var rb = ( arg ) => arg;
    rb.defaults = { src : null };
    var rt = ( arg ) => arg + 1;
    var r = () => {};
    r.head = rh;
    r.body = rb;
    r.tail = rt;
    var got = _.routine[ env.method ]( null, r, { name : 'simple' } );
    test.true( _.routine.is( got ) );
    test.identical( _.props.keys( got ), [ 'defaults', 'head', 'body', 'tail' ] );
    test.identical( got.originalRoutine, undefined );
    test.identical( got.name, 'simple' );
    test.identical( r( 'arg' ), undefined );
    test.identical( got( 'arg' ), 'arg1' );

    /* qqq : for Dmytro : bad : don't use routines from modules as test assets */ /* aaa : Dmytro : done */

    /* - */

    if( Config.debug )
    {
      test.case = 'srcs - undefined';
      test.shouldThrowErrorSync( () => _.routine[ env.method ]( null, undefined ) );

      test.case = 'srcs - null';
      test.shouldThrowErrorSync( () => _.routine[ env.method ]( null, null ) );

      test.case = 'srcs - map';
      test.shouldThrowErrorSync( () => _.routine[ env.method ]( null, { str : 'abc' } ) );
    }
  }
}

//

function routineExtendWithSingleDst( test )
{
  act({ method : 'extendCloning' });
  act({ method : 'extendInheriting' });
  act({ method : 'extendReplacing' });

  /* */

  function act( env )
  {
    test.case = 'single dst';
    var r = ( arg ) => arg;
    var got = _.routine[ env.method ]( r );
    test.identical( got, r );

    test.case = 'single dst is routine, has properties';
    var r = ( arg ) => arg;
    r.a = 0;
    r.b = 3;
    var got = _.routine[ env.method ]( r );
    test.identical( got, r );
    test.identical( got.a, 0 );
    test.identical( got.b, 3 );

    test.case = 'single dst is routine, has hiden properties';
    var r = ( arg ) => arg;
    var props =
    {
      a : { value : 0, enumerable : true, writable : false },
      b : { value : { a : 2 }, enumerable : false, writable : false },
    };
    Object.defineProperties( r, props );
    var got = _.routine[ env.method ]( r );
    test.identical( got, r );
    test.identical( _.props.keys( got ), [ 'a' ] );
    test.identical( got.a, 0 );
    test.identical( got.b, { a : 2 } );
  }
}

//

function routineExtend( test )
{
  test.case = 'dst has properties, src map has different properties';
  var dst = ( o ) =>
  {
  };
  dst.a = 0;
  dst.b = 0;
  var got = _.routine.extend( dst, { c : 1, d : 1, e : { s : 1 } } );
  test.identical( typeof got, 'function' );
  test.identical( got.a, 0 );
  test.identical( got.b, 0 );
  test.identical( got.c, 1 );
  test.identical( got.e, { s : 1 } );

  test.case = 'dst has properties, src map has the same properties';
  var dst = ( o ) =>
  {
  };
  dst.a = 0;
  dst.b = 0;
  var got = _.routine.extend( dst, { a : 1, b : 1 } );
  test.identical( typeof got, 'function' );
  test.identical( got.a, 1 );
  test.identical( got.b, 1 );

  /* */

  test.case = 'dst has non-writable properties';
  var dst = ( o ) =>
  {
  };
  var props =
  {
    'a' :
    {
      enumerable : true,
      writable : false,
      value : 0,
    },
    'b' :
    {
      enumerable : true,
      writable : false,
      value : 0,
    }
  };
  Object.defineProperties( dst, props );
  var got = _.routine.extend( dst, { a : 3, b : 2 } );
  test.identical( typeof got, 'function' );
  test.identical( got.a, 0 );
  test.identical( got.b, 0 );

  test.case = 'src has non-writable properties';
  var dst = ( o ) =>
  {
  };
  dst.a = 0;
  dst.b = 0;
  var src = {};
  var props =
  {
    'a' :
    {
      enumerable : true,
      writable : false,
      value : 3,
    },
    'b' :
    {
      enumerable : true,
      writable : false,
      value : 2,
    }
  };
  Object.defineProperties( src, props );
  var got = _.routine.extend( dst, src );
  test.identical( typeof got, 'function' );
  test.identical( got.a, 3 );
  test.identical( got.b, 2 );

  test.case = 'src is an array';
  test.shouldThrowErrorSync( () =>
  {
    var dst = ( o ) =>
    {
    };
    var got = _.routine.extend( dst, [ 'a', 1 ] );
    test.identical( typeof got, 'function' );
    test.identical( got[ 0 ], 'a' );
    test.identical( got[ 1 ], 1 );
  });

  test.open( 'few extends');

  test.case = 'null extends other routine, null extends result';
  var src = _.routine.extend( null, _.routine.s.compose );
  var got = _.routine.extend( null, src );
  /* qqq : for Dmytro : don't use routiens from the module as data for testing */
  test.identical( _.props.extend( null, got.head ), _.props.extend( null, _.routine.s.compose.head ) );
  test.identical( _.props.extend( null, got.body ), _.props.extend( null, _.routine.s.compose.body ) );
  test.identical( typeof got, 'function' );
  /* qqq : for Dmytro : lack of checks */

  // test.case = 'src extends routine, result extends map ';
  // var src1 =
  // {
  //   head : _.routine.unite.head,
  //   body : _.routine.unite.body,
  //   a : 'str',
  //   b : { b : 3 },
  // };
  // var src = _.routine.extend( null, _.routine.s.compose );
  // var got = _.routine.extend( src, src1 );
  // test.identical( _.props.extend( null, got.head ), _.props.extend( null, _.routine.unite.head ) );
  // test.identical( _.props.extend( null, got.body ), _.props.extend( null, _.routine.unite.body ) );
  // test.identical( got.b, { b : 3 } );
  // test.true( got.a === 'str' );
  // test.identical( typeof got, 'function' );
  /* qqq : for Dmytro : bad : dont use routines from modules as test assets */

  // test.case = 'dst extends map, dst extends other map';
  // var dst = () =>
  // {
  // };
  // var src1 =
  // {
  //   head : _.routine.s.compose.head,
  //   body : _.routine.s.compose.body,
  //   a : [ 'str' ],
  //   c : { d : 2 },
  // };
  // var src = _.routine.extend( dst, { c : {}, b : 'str' } );
  // var got = _.routine.extend( dst, src1 );
  // test.identical( _.props.extend( null, got.head ), _.props.extend( null, _.routine.s.compose.head ) );
  // test.identical( _.props.extend( null, got.body ), _.props.extend( null, _.routine.s.compose.body ) );
  // test.identical( got.a, [ 'str' ] );
  // test.identical( got.b, 'str' );
  // test.identical( got.c, { d : 2 } );
  // test.identical( typeof got, 'function' );
  /* qqq : for Dmytro : bad : dont use routines from modules as test assets */

  // test.case = 'dst has map property, dst extends other map';
  // var dst = () =>
  // {
  // };
  // dst.map = { a : 'str' };
  // var src1 =
  // {
  //   head : _.routine.s.compose.head,
  //   body : _.routine.s.compose.body,
  //   a : [ 'str' ],
  //   map : { d : 2 },
  // };
  // var src = _.routine.extend( dst, { c : {} } );
  // var got = _.routine.extend( dst, src1 );
  // test.identical( _.props.extend( null, got.head ), _.props.extend( null, _.routine.s.compose.head ) );
  // test.identical( _.props.extend( null, got.body ), _.props.extend( null, _.routine.s.compose.body ) );
  // test.identical( got.a, [ 'str' ] );
  // var expectedMap = { d : 2 };
  // expectedMap.a = 'str';
  // test.identical( got.map, expectedMap );
  // test.equivalent( got.c, {} );
  // test.identical
  // (
  //   __.mapBut_( null, _.props.onlyExplicit( got.c ), [ '__proto__' ] ),
  //   __.mapBut_( null, _.props.onlyExplicit( Object.create( null ) ), [ '__proto__' ] )
  // );
  // test.identical( typeof got, 'function' );
  /* qqq : for Dmytro : bad : dont use routines from modules as test assets */

  test.case = 'dst extends routine1, src extends routine, dst extends src';
  var dst = () =>
  {
  };
  var src = () =>
  {
  };
  var routine = () =>
  {
  };
  routine.a = 0;
  routine.b = [ 'str' ];
  var routine1 = () =>
  {
  };
  routine1.a = 2;
  routine1.c = 'str';
  var src1 = _.routine.extend( src, routine );
  var src2 = _.routine.extend( dst, routine1 );
  var got = _.routine.extend( src2, src1 )
  test.identical( got.a, 0 );
  test.identical( got.b, [ 'str' ] );
  test.identical( got.c, 'str' );
  test.identical( dst.a, got.a );
  test.identical( dst.b, got.b );
  test.identical( dst.c, got.c );
  test.identical( typeof got, 'function' );

  test.case = 'dst extends map, src extends map, dst extends src';
  var dst = () =>
  {
  };
  var src = () =>
  {
  };
  var src1 = _.routine.extend( src, { o1 : 1, o2 : 'str' } );
  var src2 = _.routine.extend( dst, { o3 : 'o3', o1 : 'map' } );
  var got = _.routine.extend( src2, src1 )
  test.identical( got.o1, 1 );
  test.identical( got.o2, 'str' );
  test.identical( got.o3, 'o3' );
  test.identical( dst.a, got.a );
  test.identical( dst.b, got.b );
  test.identical( dst.c, got.c );
  test.identical( typeof got, 'function' );

  test.case = 'extend by map';
  var dst = () =>
  {
  };
  Object.defineProperties( dst, {
    'b' : {
      value : { a : 2 },
      enumerable : true,
      writable : true,
    }
  });
  var got = _.routine.extend( dst );
  test.equivalent( got.b, { a : 2 } );

  test.case = 'extend by map';
  var dst = ( o ) =>
  {
  };
  dst.b = { map : 2 };
  var got = _.routine.extend( dst, { b : { map : 3 } } );
  test.equivalent( got.b, { map : 3 } );

  test.close( 'few extends');

  test.case = 'sanitare';
  var exp =
  {
    'defaults' :
    {
      'head' : null,
      'body' : null,
      'tail' : null,
      'name' : null,
      'strategy' : null,
    }
  }
  test.identical( _.props.extend( null, _.routine.unite.body ), exp );
  var exp =
  {
    'defaults' :
    {
      'head' : null,
      'body' : null,
      'tail' : null,
      'name' : null,
      'strategy' : 'replacing',
    }
  }
  test.identical( __.mapBut_( null, _.routine.unite, [ 'body', 'head' ] ), exp );

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.routine.extend();
  });

  test.case = 'three arguments';
  test.shouldThrowErrorSync( function()
  {
    _.routine.extend( null, { a : 1 }, { b : 2 });
  });

  test.case = 'single dst is null';
  test.shouldThrowErrorSync( function()
  {
    _.routine.extend( null );
  });

  // test.case = 'second arg has not head and body properties';
  // test.shouldThrowErrorSync( function()
  // {
  //   _.routine.extend( null, _.unrollIs );
  // });

  test.case = 'second arg is primitive';
  test.shouldThrowErrorSync( function()
  {
    _.routine.extend( _.unrollIs, 'str' );
  });

  test.shouldThrowErrorSync( function()
  {
    _.routine.extend( _.unrollIs, 1 );
  });

  test.case = 'dst is not routine or null';
  test.shouldThrowErrorSync( function()
  {
    _.routine.extend( 1, { a : 1 } );
  });

  test.shouldThrowErrorSync( function()
  {
    _.routine.extend( 'str', { a : 1 } );
  });
}

//

function extendSpecial( test )
{

  test.case = 'assumption';

  var obj = { f1, f2 : f1.bind() }
  test.true( obj.f1() === obj );
  test.true( obj.f2() === undefined );

  /* */

  act({ method : 'extendCloning' });
  act({ method : 'extendInheriting' });
  act({ method : 'extendReplacing' });

  /* */

  function f1()
  {
    return this;
  }

  /* */

  function act( env )
  {

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, basic`;

    function routine1a( o )
    {
      return o.a + o.b;
    }
    routine1a.defaults =
    {
      a : 1,
      b : 3,
    }

    var routine1b = _.routine.extendInheriting( null, routine1a );
    test.true( routine1a !== routine1b );
    test.true( routine1a.defaults !== routine1b.defaults );
    test.true( _.prototype.has( routine1b.defaults, routine1a.defaults ) );
    test.equivalent( routine1a.defaults, routine1b.defaults );

    var united = _.routine.unite( null, routine1b );
    test.identical( united(), 4 );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, triple`;

    function routine2a( o )
    {
      return o.a + o.b;
    }
    var defaults2a = routine2a.defaults =
    {
      a : 1,
      b : 3,
    }

    function routine2b( o )
    {
      return o.a + o.b;
    }
    var defaults2b = routine2b.defaults =
    {
      a : 2,
      b : 5,
    }

    var routine2c = _.routine[ env.method ]( null, routine2a, routine2b );
    test.true( routine2c !== routine2a );
    test.true( routine2c !== routine2b );
    test.true( routine2a.defaults === defaults2a );
    test.true( routine2b.defaults === defaults2b );
    test.true( !_.prototype.has( routine2c.defaults, routine2a.defaults ) );

    if( env.method === 'extendCloning' )
    test.true( !_.prototype.has( routine2c.defaults, routine2b.defaults ) );
    else
    test.true( _.prototype.has( routine2c.defaults, routine2b.defaults ) );

    if( env.method === 'extendReplacing' )
    {
      test.true( routine2c.defaults === routine2b.defaults );
    }
    else
    {
      test.true( routine2c.defaults !== routine2b.defaults );
    }

    var exp =
    {
      a : 1,
      b : 3,
    }
    test.equivalent( routine2a.defaults, exp );

    var exp =
    {
      a : 2,
      b : 5,
    }
    test.equivalent( routine2b.defaults, exp );

    var exp =
    {
      a : 2,
      b : 5,
    }
    test.equivalent( routine2c.defaults, exp );

    var united = _.routine.unite( null, routine2c ); /* qqq : for Dmytro : write such test routine for _.routine.unite() */
    test.identical( united(), 7 );

    /* */

  }

}

//

function extendBodyInstanicing( test )
{

  act({ method : 'extendCloning' });
  act({ method : 'extendInheriting' });
  act({ method : 'extendReplacing' });
  act({ method : 'extend' });

  /* */

  function act( env )
  {

    test.case = `${__.entity.exportStringSolo( env )}`;

    function routine1Dst( o )
    {
      return o.a + o.b;
    }

    function routine1a_body( o )
    {
      return o.a + o.b + 1;
    }
    var defaults1a = routine1a_body.defaults =
    {
      a : 1,
      b : 3,
    }

    var routine1a = _.routine.uniteReplacing( null, routine1a_body );

    test.true( routine1a.body.defaults === defaults1a );
    test.true( routine1a.defaults === defaults1a );

    _.routine[ env.method ]( routine1Dst, routine1a );
    test.identical( env.method === 'extendReplacing', routine1Dst.body === routine1a_body );
    test.true( routine1a.body === routine1a_body );

  }

}

//

/* qqq : for Dmytro : extend */
function extendThrowing( test )
{

  act({ method : 'extendCloning' });
  act({ method : 'extendInheriting' });
  act({ method : 'extendReplacing' });
  act({ method : 'extend' });

  /* */

  function act( env )
  {

    test.case = 'assumption';
    var got = _.routine[ env.method ]( f1 );
    test.true( got === f1 );

    if( Config.debug )
    return test.true( true );

    test.case = 'undefined second arg';
    test.shouldThrowErrorSync( () => _.routine[ env.method ]( f1, undefined ) );
    test.case = 'undefined first arg';
    test.shouldThrowErrorSync( () => _.routine[ env.method ]( undefined, f1 ) );

    test.case = 'sanitare';
    test.identical( _.props.extend( null, f1 ), {} );

  }

  function f1()
  {
  }

}

//

function routineDefaults( test )
{

  test.case = 'make new routine';

  function add1_head( routine, args )
  {
    return _.routine.options( routine, args );
  }
  function add1_body( o )
  {
    o = _.routine.assertOptions( add1, arguments );
    return o.a + o.b;
  }
  add1_body.defaults =
  {
    a : 1,
    b : 3,
  }
  let add1 = _.routine.unite( add1_head, add1_body );

  test.description = 'control call';
  var got = add1();
  test.identical( got, 4 );

  test.description = 'generate';
  let add2 = _.routine.defaults( null, add1, { b : 5 } );
  test.true( add1 !== add2 );
  test.true( add1.defaults !== add2.defaults );
  test.true( _.aux.is( add1.defaults ) );
  test.true( _.aux.is( add2.defaults ) );
  test.true( add1.defaults.b === 3 );
  test.true( add2.defaults.b === 5 );

  test.description = 'trivial call';
  var got = add2();
  test.identical( got, 6 );

  /* */

  test.case = 'adjust routine defaults';

  function add3_head( routine, args )
  {
    return _.routine.options( routine, args );
  }
  function add3_body( o )
  {
    o = _.routine.assertOptions( add1, arguments );
    return o.a + o.b;
  }
  add3_body.defaults =
  {
    a : 1,
    b : 3,
  }
  let add3 = _.routine.unite( add3_head, add3_body );

  test.description = 'control call';
  var got = add3();
  test.identical( got, 4 );

  test.description = 'generate';
  let add4 = _.routine.defaults( add3, { b : 5 } );
  test.true( add3 === add4 );
  test.true( add3.defaults === add4.defaults );
  test.true( _.aux.is( add3.defaults ) );
  test.true( _.aux.is( add4.defaults ) );
  test.true( add3.defaults.b === 5 );
  test.true( add4.defaults.b === 5 );

  test.description = 'trivial call';
  var got = add4();
  test.identical( got, 6 );

}

//

// qqq : for Dmytro : extend the routine. introduce subroutine for strategies. maybe split?
function uniteBasic( test )
{
  function headObject( rotine, args )
  {
    let o = Object.create( null );
    o.args = args;
    return o;
  }

  function headUnroll( rotine, args )
  {
    return _.unroll.make( args );
  }

  function bodyObject( o )
  {
    return _.array.make( o.args );
  }
  bodyObject.defaults =
  {
    args : null,
  }

  function bodyUnroll()
  {
    return _.array.make( arguments );
  }
  bodyUnroll.defaults =
  {
    args : null,
  }

  function tail( result )
  {
    result[ 0 ] += 1;
    return result;
  }

  /* - */

  test.open( 'only body' );

  test.case = 'head - undefined, body expects map';
  var routine = _.routine.unite( undefined, bodyObject );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine({ args : _.argumentsArray.make([ 1, 2 ]) });
  test.true( _.arrayIs( got ) );
  test.identical( got, [ 1, 2 ] );

  // test.case = 'head - undefined, body expects unroll';
  // var routine = _.routine.unite( undefined, bodyUnroll );
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 1, 2 ] );

  test.case = 'head - null, body expects map';
  var routine = _.routine.unite( null, bodyObject );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine({ args : _.argumentsArray.make([ 1, 2 ]) });
  test.true( _.arrayIs( got ) );
  test.identical( got, [ 1, 2 ] );


  // test.case = 'head - null, body expects unroll';
  // var routine = _.routine.unite( null, bodyUnroll );
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 1, 2 ] );

  /* */


  // test.case = 'head - null';
  // var routine = _.routine.unite({ head : null, body : bodyUnroll });
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 1, 2 ] );

  // test.case = 'head - undefined';
  // var routine = _.routine.unite({ head : undefined, body : bodyUnroll });
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 1, 2 ] );

  test.close( 'only body' );

  /* - */

  test.open( 'head and body' );

  test.case = 'make from arguments, routine expects map';
  var routine = _.routine.unite( headObject, bodyObject );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  test.case = 'make from arguments, routine expects unroll';
  var routine = _.routine.unite( headUnroll, bodyUnroll );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyUnroll' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  /* */

  test.case = 'make from map, routine expects map';
  var routine = _.routine.unite({ head : headObject, body : bodyObject });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  test.case = 'make from map, routine expects unroll';
  var routine = _.routine.unite({ head : headUnroll, body : bodyUnroll });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyUnroll' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  test.close( 'head and body' );

  /* - */

  test.open( 'body and tail' );

  test.case = 'head - undefined, body expects map';
  var routine = _.routine.unite( undefined, bodyObject, tail );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine({ args : _.argumentsArray.make([ 1, 2 ]) });
  test.true( _.arrayIs( got ) );
  test.identical( got, [ 2, 2 ] );


  // test.case = 'head - undefined, body expects unroll';
  // var routine = _.routine.unite( undefined, bodyUnroll, tail );
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 2, 2 ] );

  test.case = 'head - null, body expects map';
  var routine = _.routine.unite( null, bodyObject, tail );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine({ args : _.argumentsArray.make([ 1, 2 ]) });
  test.true( _.arrayIs( got ) );
  test.identical( got, [ 2, 2 ] );


  // test.case = 'head - null, body expects unroll';
  // var routine = _.routine.unite( null, bodyUnroll, tail );
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 2, 2 ] );

  /* */


  // test.case = 'head - null';
  // var routine = _.routine.unite({ head : null, body : bodyUnroll, tail });
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 2, 2 ] );


  // test.case = 'head - undefined';
  // var routine = _.routine.unite({ head : undefined, body : bodyUnroll, tail });
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var got = routine( _.unroll.make([ 1, 2 ]) );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 2, 2 ] );

  /* */

  var tailUseOptions = ( result, o ) =>
  {
    result[ 0 ] += 1;
    __.arrayAppend( result, o );
    return result;
  }

  test.case = 'head - null, tail use options map';
  var routine = _.routine.unite( null, bodyObject, tailUseOptions );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var o = { args : _.argumentsArray.make([ 1, 2 ]) };
  var got = routine( o );
  test.true( _.arrayIs( got ) );
  test.identical( got, [ 2, 2, o ] );


  // test.case = 'head - null, tail use options map';
  // var routine = _.routine.unite({ head : null, body : bodyUnroll, tail : tailUseOptions });
  // test.true( _.routine.is( routine ) );
  // test.identical( routine.name, 'bodyUnroll' );
  // test.identical( routine.defaults, { args : null } );
  // var o = _.unroll.make([ 1, 2 ]);
  // var got = routine( o );
  // test.true( _.arrayIs( got ) );
  // test.identical( got, [ 2, 2, o ] );

  test.close( 'body and tail' );

  /* - */

  test.open( 'head, body and tail' );

  test.case = 'head, body and tail, with map';
  var routine = _.routine.unite( headObject, bodyObject, tail );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 2, 2 ] );

  test.case = 'head, body and tail, with unroll';
  var routine = _.routine.unite( headUnroll, bodyUnroll, tail );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyUnroll' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 2, 2 ] );

  /* */

  test.case = 'head, body and tail, with map';
  var routine = _.routine.unite({ head : headObject, body : bodyObject, tail });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 2, 2 ] );

  test.case = 'head, body and tail, with unroll';
  var routine = _.routine.unite({ head : headUnroll, body : bodyUnroll, tail });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyUnroll' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 2, 2 ] );

  /* */

  var tailUseOptions = ( result, o ) =>
  {
    result[ 0 ] += 1;
    __.arrayAppend( result, o );
    return result;
  }

  test.case = 'head - null, tail use options map';
  var routine = _.routine.unite( headObject, bodyObject, tailUseOptions );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.true( _.arrayIs( got ) );
  test.identical( got, [ 2, 2, { args : _.argumentsArray.make([ 1, 2 ]) } ] );

  test.case = 'head - null, tail use options map';
  var routine = _.routine.unite({ head : headUnroll, body : bodyUnroll, tail : tailUseOptions });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyUnroll' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.true( _.arrayIs( got ) );
  test.identical( got, [ 2, 2, _.unroll.make([ 1, 2 ]) ] );

  test.close( 'head, body and tail' );

  /* - */

  test.open( 'names' );

  test.case = 'name defined by field, head and body';
  var routine = _.routine.unite({ head : headObject, body : bodyObject, name : 'someName' });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'someName' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  test.case = 'name defined by field, head, body and tail, with map';
  var routine = _.routine.unite({ head : headObject, body : bodyObject, tail, name : 'someName' });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'someName' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 2, 2 ] );

  /* */

  test.case = 'routine body name has postfix `_body`';
  function someRoutine_body( o )
  {
    return _.array.make( o.args );
  }
  someRoutine_body.defaults = { args : null };

  var routine = _.routine.unite( headObject, someRoutine_body );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'someRoutine' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  test.close( 'names' );

  /* - */

  test.open( 'composed head' );

  function headComposeObject( routine, args )
  {
    let o = Object.create( null );
    o.args = args[ 0 ].args;
    return o;
  }

  test.case = 'compose head and body, with map';
  var routine = _.routine.unite( [ headObject, headComposeObject ], bodyObject );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  test.case = 'compose head, body and tail, with map';
  var routine = _.routine.unite( [ headObject, headComposeObject ], bodyObject, tail );
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 2, 2 ] );

  /* */

  test.case = 'compose head and body, with map';
  var routine = _.routine.unite({ head : [ headObject, headComposeObject ], body : bodyObject });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 1, 2 ] );

  test.case = 'compose head, body and tail, with map';
  var routine = _.routine.unite({ head : [ headObject, headComposeObject ], body : bodyObject, tail });
  test.true( _.routine.is( routine ) );
  test.identical( routine.name, 'bodyObject' );
  test.identical( routine.defaults, { args : null } );
  var got = routine( 1, 2 );
  test.identical( got, [ 2, 2 ] );

  test.close( 'composed head' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.routine.unite() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.routine.unite( headObject ) );

  test.case = 'wrong type of body routine';
  test.shouldThrowErrorSync( () => _.routine.unite( headObject, null ) );

  test.case = 'wrong type of head';
  test.shouldThrowErrorSync( () => _.routine.unite( 'a', bodyObject ) );

  test.case = 'wrong type of body';
  test.shouldThrowErrorSync( () => _.routine.unite( headObject, 1 ) );

  test.case = 'wrong type of tail routine';
  test.shouldThrowErrorSync( () => _.routine.unite( headObject, bodyObject, 'tail' ) );

  test.case = 'body routine without defaults';
  function bodyWithoutDefaults( o )
  {
    return o.args;
  }
  test.shouldThrowErrorSync( () => _.routine.unite( headObject, bodyWithoutDefaults ) );

  test.case = 'body routine has no name';
  test.shouldThrowErrorSync( () => _.routine.unite( headObject, ( o ) => o.args ) );
}

//

function uniteInstancing( test )
{

  act({ method : 'uniteCloning' });
  act({ method : 'uniteInheriting' });
  act({ method : 'uniteReplacing' });
  act({ method : 'unite' });

  /* */

  function act( env )
  {

    test.case = `${__.entity.exportStringSolo( env )}`;

    function r1_head( routine, args )
    {
      let o = args.length ? args[ 0 ] : null;
      return _.routine.options_( routine, o );
    }

    function r1_body( o )
    {
      return o.a + o.b + 1;
    }

    var extra1 = r1_body.extra =
    {
    }

    var defaults1 = r1_body.defaults =
    {
      a : 1,
      b : 3,
    }

    var r1 = _.routine[ env.method ]( r1_head, r1_body );

    test.identical( env.method === 'uniteReplacing' || env.method === 'unite', r1_body === r1.body );

    test.true( r1.defaults === r1.body.defaults );
    test.identical( env.method === 'uniteReplacing' || env.method === 'unite', r1.defaults === r1_body.defaults );
    test.true( r1_body.defaults === defaults1 );
    test.identical( env.method === 'uniteCloning', !_.prototype.has( r1.defaults, defaults1 ) );

    test.true( r1.extra === r1.body.extra );
    test.identical( env.method === 'uniteReplacing' || env.method === 'unite', r1.extra === r1_body.extra );
    test.true( r1_body.extra === extra1 );
    test.identical( env.method === 'uniteCloning', !_.prototype.has( r1.extra, extra1 ) );

    test.identical( r1(), 5 );

  }

}

//

function uniteWithNumberInsteadOfHead( test )
{

  act({ method : 'uniteCloning' });
  act({ method : 'uniteInheriting' });
  act({ method : 'uniteReplacing' });
  act({ method : 'unite' });

  /* */

  function act( env )
  {

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, 1 arg`;

    function r1_body( arg, o )
    {
      test.identical( arguments.length, 2 );
      return arg + o.a + o.b;
    }

    var defaults1 = r1_body.defaults =
    {
      a : 1,
      b : 3,
    }

    var r1 = _.routine[ env.method ]( 1, r1_body );
    test.identical( r1( 10 ), 14 );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, 2 args`;

    function r2_body( arg1, arg2, o )
    {
      test.identical( arguments.length, 3 );
      debugger;
      return arg1 + arg2 + o.a + o.b;
    }

    var defaults1 = r2_body.defaults =
    {
      a : 1,
      b : 3,
    }

    var r2 = _.routine[ env.method ]( 2, r2_body );
    test.identical( r2( 10, 20 ), 34 );

    /* */

  }

}

// --
// etc
// --

function composeBasic( test )
{
  test.case = 'empty';
  var counter = 0;
  var routines = [];
  var composition = _.routine.s.compose( routines );
  var got = composition( 1, 2, 3 );
  var expected = [];
  test.identical( got, expected );
  test.identical( counter, 0 );

  /* */

  test.case = 'single routine';
  var counter = 0;
  var routines = routineUnrolling;
  var composition = _.routine.s.compose( routines );
  var got = composition( 1, 2, 3 );
  var expected = [ 1, 2, 3, 16 ];
  test.identical( got, expected );
  test.identical( counter, 16 );

  /* - */

  test.open( 'unrolling:1' )

  test.case = 'without chainer';
  var counter = 0;
  var routines = [ null, routineUnrolling, null, r2, null ];
  var composition = _.routine.s.compose( routines );
  var got = composition( 1, 2, 3 );
  var expected = [ 1, 2, 3, 16, 128 ];
  test.identical( got, expected );
  test.identical( counter, 128 );

  /* */

  test.case = 'with chainer';
  var counter = 0;
  var routines = [ null, routineUnrolling, null, r2, null ];
  var composition = _.routine.s.compose( routines, chainer1 );
  var got = composition( 1, 2, 3 );
  var expected = [ 1, 2, 3, 16, 160 ];
  test.identical( got, expected );
  test.identical( counter, 160 );

  /* */

  test.case = 'with chainer and break';
  var counter = 0;
  var routines = [ null, routineUnrolling, null, _break, null, r2, null ];
  var composition = _.routine.s.compose( routines, chainer1 );
  var got = composition( 1, 2, 3 );
  var expected = [ 1, 2, 3, 16, _.dont ];
  test.identical( got, expected );
  test.identical( counter, 16 );

  test.close( 'unrolling:1' )

  /* - */

  test.open( 'unrolling:0' )

  test.case = 'without chainer';
  var counter = 0;
  var routines = [ null, routineNotUnrolling, null, r2, null ];
  var composition = _.routine.s.compose( routines );
  var got = composition( 1, 2, 3 );
  var expected = [ [ 1, 2, 3, 16 ], 128 ];
  test.identical( got, expected );
  test.identical( counter, 128 );

  /* */

  test.case = 'with chainer';
  var counter = 0;
  var routines = [ null, routineNotUnrolling, null, r2, null ];
  var composition = _.routine.s.compose( routines, chainer1 );
  var got = composition( 1, 2, 3 );
  var expected = [ [ 1, 2, 3, 16 ], 160 ];
  test.identical( got, expected );
  test.identical( counter, 160 );

  /* */

  test.case = 'with chainer and break';
  var counter = 0;
  var routines = [ null, routineNotUnrolling, null, _break, null, r2, null ];
  var composition = _.routine.s.compose( routines, chainer1 );
  var got = composition( 1, 2, 3 );
  var expected = [ [ 1, 2, 3, 16 ], _.dont ];
  test.identical( got, expected );
  test.identical( counter, 16 );

  test.close( 'unrolling:0' )

  /* - */

  if( !Config.debug )
  return;

  test.case = 'bad arguments';

  test.shouldThrowErrorSync( () => _.routine.s.composeAll() );
  test.shouldThrowErrorSync( () => _.routine.s.composeAll( routines, function(){}, function(){} ) );

  /* - */

  function routineUnrolling()
  {
    counter += 10;
    for( var a = 0 ; a < arguments.length ; a++ )
    counter += arguments[ a ];
    return _.unrollAppend( _.unroll.make( null ), _.unroll.make( arguments ), counter );
  }

  function routineNotUnrolling()
  {
    counter += 10;
    for( var a = 0 ; a < arguments.length ; a++ )
    counter += arguments[ a ];
    let result = __.arrayAppendArrays( null, arguments );
    return __.arrayAppend( result, counter );
  }

  function r2()
  {
    counter += 100;
    for( var a = 0 ; a < arguments.length ; a++ )
    counter += 2*arguments[ a ];
    return counter;
  }

  function _break()
  {
    return _.dont;
  }

  function chainer1( /* args, result, o, k */ )
  {
    let args = arguments[ 0 ];
    let result = arguments[ 1 ];
    let o = arguments[ 2 ];
    let k = arguments[ 3 ];
    if( result !== _.dont )
    return _.unroll.as( result );
    return result;
  }
}

// --
//
// --

const Proto =
{

  name : 'Tools.Routine.l0.l1',
  silencing : 1,

  tests :
  {

    // dichotomy
    dichotomy,
    is,
    like,
    isSync,
    isAsync,
    isTrivial,

    // joiner

    _join,
    constructorJoin,
    join,
    seal,

    // options

    optionsWithoutUndefined, /* qqq : make templating test subroutine act() */
    optionsWithUndefined, /* qqq : make templating test subroutine act() */
    optionsWithUndefinedTollerant, /* qqq : make templating test subroutine act() */
    assertOptionsWithoutUndefined, /* qqq : make templating test subroutine act() */
    assertOptionsWithUndefined, /* qqq : make templating test subroutine act() */


    // amend

    /* routineExtend_old, */
    routineExtendWithDstIsNull,
    routineExtendWithSingleDst,
    routineExtend,
    extendSpecial,
    extendBodyInstanicing,
    extendThrowing,
    routineDefaults,

    // unite

    uniteBasic,
    uniteInstancing,
    uniteWithNumberInsteadOfHead,

    // etc

    composeBasic,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
