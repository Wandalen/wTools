( function _l0_l9_Functional_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
  _.include( 'wSelector' ); /* xxx : qqq : remove */
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

_.assert( _.routineIs( _.select ) );

//--
// scalar
//--

function scalarAppend( test )
{
  test.case = 'dst is undefined, src = undefined';
  var dst = undefined;
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is undefined, src is longLike';
  var dst = undefined;
  var src = [ null, '', 1, [], [ 1, [ 2 ] ] ];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null, '', 1, [], [ 1, [ 2 ] ] ] );

  test.case = 'dst is undefined, src = unroll';
  var dst = undefined;
  var src = _.unroll.make( [ null, '', 1 ] );
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src = argumentsArray';
  var dst = undefined;
  var src = _.argumentsArray.make( [ null, '', 1 ] );
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src is buffer';
  var dst = undefined;
  var src = new F32x( [ 0, 2, 10 ] );
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 0, 2, 10 ] );

  test.case = 'dst is undefined, src is string';
  var dst = undefined;
  var src = 'str';
  var got = _.scalarAppend( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is undefined, src is map';
  var dst = undefined;
  var src = { 'a' : 'str' };
  var got = _.scalarAppend( dst, src );
  test.identical( got, { 'a' : 'str' } );

  /* dst is array */

  test.case = 'dst is empty array, src is empty array';
  var dst = [];
  var src = [];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty array, src is null';
  var dst = [];
  var src = null;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is array, src = undefined';
  var dst = [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ];
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is array, src is string';
  var dst = [ 1, null, 'str', [] ];
  var src = 'str';
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], 'str' ] );
  test.true( got === dst );

  test.case = 'dst is array, src is map';
  var dst = [ 1, null, 'str', [] ];
  var src = { 'a' : 1 };
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], { 'a' : 1 } ] );
  test.true( got === dst );

  test.case = 'dst is array, src = array';
  var dst = [ 1, null, 'str', [] ];
  var src = [ 'src', 2, undefined ];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], 'src', 2, undefined ] );
  test.true( got === dst );

  /* dst is unroll */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.unroll.make( [] );
  var src = [];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.unroll.make( [] );
  var src = null;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.unroll.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is string';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], 'str' ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is map';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = { 'a' : 1 };
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], { 'a' : 1 } ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = array';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = [ 'src', 2, undefined ];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], 'src', 2, undefined ] );
  test.true( got === dst );

  /* dst is argumentsArray */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.argumentsArray.make( [] );
  var src = [];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.argumentsArray.make( [] );
  var src = null;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.argumentsArray.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );

  test.case = 'dst is unroll, src is string';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], 'str' ] );

  test.case = 'dst is unroll, src is map';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = { 'a' : 1 };
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], { 'a' : 1 } ] );

  test.case = 'dst is unroll, src = array';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = [ 'src', 2, undefined ];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, null, 'str', [], 'src', 2, undefined ] );

  /* dst is buffer */

  test.case = 'dst is empty buffer, src = undefined';
  var dst = new U8x();
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty buffer, src is empty array';
  var dst = new U8x();
  var src = [];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is buffer, src = undefined';
  var dst = new U8x( [ 1, 2, 0, 78 ] );
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is number';
  var dst = new I16x( [ 1, 2, 0, 78 ] );
  var src = 32;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, 2, 0, 78, 32 ] );

  test.case = 'dst is buffer, src is empty array';
  var dst = new U16x( [ 1, 2, 0, 78 ] );
  var src = [];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src = array';
  var dst = new I32x( [ 1, 2, 0, 78 ] );
  var src = [ 'str', null, undefined ];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, 2, 0, 78, 'str', null, undefined ] );

  test.case = 'dst is buffer, src is buffer';
  var dst = new U32x( [ 1, 2, 0, 78 ] );
  var src = new F32x( [ 1, 2, 3, 4 ] );
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, 2, 0, 78, 1, 2, 3, 4 ] );

  /* dst not undefined, not longLike */

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = null;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null, null ] );

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, null );

  test.case = 'dst is null, src is empty array';
  var dst = null;
  var src = [];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = '';
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 'str', '' ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = undefined;
  var got = _.scalarAppend( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is number, src is string';
  var dst = 1;
  var src = [ '', 2, [], [ { 'a' : 2 } ] ];
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ 1, '', 2, [], [ { 'a' : 2 } ] ] );

  test.case = 'dst is map, src is buffer';
  var dst = { 'a' : 1 };
  var src = new U8x( [ 10, 20, 30 ] );
  var got = _.scalarAppend( dst, src );
  test.identical( got, [ { 'a' : 1 }, 10, 20, 30 ] );

  /* */

  test.case = 'dst === src';
  var arr = [ 1, 2, 'str' ];
  var dst = arr;
  var src = arr;
  var got = _.scalarAppend( dst, src );
  test.identical( got, [  1, 2, 'str',  1, 2, 'str' ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'wrong arguments.length';
  test.shouldThrowErrorSync( () => _.scalarAppend() );
  test.shouldThrowErrorSync( () => _.scalarAppend( 1 ) );
  test.shouldThrowErrorSync( () => _.scalarAppend( 1, 2, 'str' ) );
}

//

function scalarAppendOnce( test )
{
  test.case = 'dst is undefined, src = undefined';
  var dst = undefined;
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is undefined, src is longLike';
  var dst = undefined;
  var src = [ null, '', 1, [], [ 1, [ 2 ] ], null, 1 ];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null, '', 1, [], [ 1, [ 2 ] ] ] );

  test.case = 'dst is undefined, src = unroll';
  var dst = undefined;
  var src = _.unroll.make( [ null, '', 1 , null, '', 1 ] );
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src = argumentsArray';
  var dst = undefined;
  var src = _.argumentsArray.make( [ null, '', 1 , null, '', 1 ] );
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src is buffer';
  var dst = undefined;
  var src = new F32x( [ 0, 2, 10, 0, 2, 10 ] );
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 0, 2, 10 ] );

  test.case = 'dst is undefined, src is string';
  var dst = undefined;
  var src = 'str';
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is undefined, src is map';
  var dst = undefined;
  var src = { 'a' : 'str' };
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, { 'a' : 'str' } );

  /* dst is array */

  test.case = 'dst is empty array, src is empty array';
  var dst = [];
  var src = [];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty array, src is null';
  var dst = [];
  var src = null;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is array, src = undefined';
  var dst = [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ], [ 1, [ 2 ] ] ];
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is array, src is string';
  var dst = [ 1, null, 'str', [], 1 ];
  var src = 'str';
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], 1 ] );
  test.true( got === dst );

  test.case = 'dst is array, src is map';
  var dst = [ 1, null, 'str', [], { 'a' : 1 } ];
  var src = { 'a' : 1 };
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], { 'a' : 1 }, { 'a' : 1 } ] );
  test.true( got === dst );

  test.case = 'dst is array, src = array';
  var dst = [ 1, null, 'str', [] ];
  var src = [ 'str', 1, undefined ];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], undefined ] );
  test.true( got === dst );

  /* dst is unroll */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.unroll.make( [] );
  var src = [];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.unroll.make( [] );
  var src = null;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.unroll.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is string';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is map';
  var dst = _.unroll.make( [ 1, null, 'str', [], { 'a' : 1 } ] );
  var src = { 'a' : 1 };
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], { 'a' : 1 }, { 'a' : 1 } ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = array';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = [ 'str', 1, undefined ];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], undefined ] );
  test.true( got === dst );

  /* dst is argumentsArray */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.argumentsArray.make( [] );
  var src = [];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.argumentsArray.make( [] );
  var src = null;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.argumentsArray.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );

  test.case = 'dst is unroll, src is string';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [] ] );

  test.case = 'dst is unroll, src is map';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [], { 'a' : 1 } ] );
  var src = { 'a' : 1 };
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], { 'a' : 1 }, { 'a' : 1 } ] );

  test.case = 'dst is unroll, src = array';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = [ 'str', 1, undefined ];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], undefined ] );

  /* dst is buffer */

  test.case = 'dst is empty buffer, src = undefined';
  var dst = new U8x();
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty buffer, src is empty array';
  var dst = new U8x();
  var src = [];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is buffer, src = undefined';
  var dst = new U8x( [ 1, 2, 0, 78 ] );
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is number';
  var dst = new I16x( [ 1, 2, 0, 78 ] );
  var src = 0;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is empty array';
  var dst = new U16x( [ 1, 2, 0, 78 ] );
  var src = [];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src = array';
  var dst = new I32x( [ 1, 2, 0, 78 ] );
  var src = [ 'str', 2, 78 ];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78, 'str' ] );

  test.case = 'dst is buffer, src is buffer';
  var dst = new U32x( [ 1, 2, 0, 78 ] );
  var src = new F32x( [ 1, 2, 3, 4 ] );
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78, 3, 4 ] );

  /* dst not undefined, not longLike */

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = null;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, null );

  test.case = 'dst is null, src is empty array';
  var dst = null;
  var src = [];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = 'str';
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 'str' ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = undefined;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is number, src is string';
  var dst = 1;
  var src = [ '', 1, [], [ { 'a' : 2 } ] ];
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ 1, '', [], [ { 'a' : 2 } ] ] );

  test.case = 'dst is map, src is buffer';
  var dst = { 'a' : 1 };
  var src = new U8x( [ 10, 20, 30 ] );
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [ { 'a' : 1 }, 10, 20, 30 ] );

  /* */

  test.case = 'dst === src';
  var arr = [ 1, 2, 'str' ];
  var dst = arr;
  var src = arr;
  var got = _.scalarAppendOnce( dst, src );
  test.identical( got, [  1, 2, 'str' ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'wrong arguments.length';
  test.shouldThrowErrorSync( () => _.scalarAppendOnce() );
  test.shouldThrowErrorSync( () => _.scalarAppendOnce( 1 ) );
  test.shouldThrowErrorSync( () => _.scalarAppendOnce( 1, 2, 'str' ) );
}

//

function scalarPrepend( test )
{
  test.case = 'dst is undefined, src = undefined';
  var dst = undefined;
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is undefined, src is longLike';
  var dst = undefined;
  var src = [ null, '', 1, [], [ 1, [ 2 ] ] ];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null, '', 1, [], [ 1, [ 2 ] ] ] );

  test.case = 'dst is undefined, src = unroll';
  var dst = undefined;
  var src = _.unroll.make( [ null, '', 1 ] );
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src = argumentsArray';
  var dst = undefined;
  var src = _.argumentsArray.make( [ null, '', 1 ] );
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src is buffer';
  var dst = undefined;
  var src = new F32x( [ 0, 2, 10 ] );
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 0, 2, 10 ] );

  test.case = 'dst is undefined, src is string';
  var dst = undefined;
  var src = 'str';
  var got = _.scalarPrepend( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is undefined, src is map';
  var dst = undefined;
  var src = { 'a' : 'str' };
  var got = _.scalarPrepend( dst, src );
  test.identical( got, { 'a' : 'str' } );

  /* dst is array */

  test.case = 'dst is empty array, src is empty array';
  var dst = [];
  var src = [];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty array, src is null';
  var dst = [];
  var src = null;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is array, src = undefined';
  var dst = [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ];
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is array, src is string';
  var dst = [ 1, null, 'str', [] ];
  var src = 'str';
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 'str', 1, null, 'str', [] ] );
  test.true( got === dst );

  test.case = 'dst is array, src is map';
  var dst = [ 1, null, 'str', [] ];
  var src = { 'a' : 1 };
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ { 'a' : 1 }, 1, null, 'str', [] ] );
  test.true( got === dst );

  test.case = 'dst is array, src = array';
  var dst = [ 1, null, 'str', [] ];
  var src = [ 'src', 2, undefined ];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 'src', 2, undefined, 1, null, 'str', [] ] );
  test.true( got === dst );

  /* dst is unroll */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.unroll.make( [] );
  var src = [];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.unroll.make( [] );
  var src = null;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.unroll.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is string';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 'str', 1, null, 'str', [] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is map';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = { 'a' : 1 };
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ { 'a' : 1 }, 1, null, 'str', [] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = array';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = [ 'src', 2, undefined ];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 'src', 2, undefined, 1, null, 'str', [] ] );
  test.true( got === dst );

  /* dst is argumentsArray */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.argumentsArray.make( [] );
  var src = [];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.argumentsArray.make( [] );
  var src = null;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.argumentsArray.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );

  test.case = 'dst is unroll, src is string';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 'str', 1, null, 'str', [] ] );

  test.case = 'dst is unroll, src is map';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = { 'a' : 1 };
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ { 'a' : 1 }, 1, null, 'str', [] ] );

  test.case = 'dst is unroll, src = array';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = [ 'src', 2, undefined ];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 'src', 2, undefined, 1, null, 'str', [] ] );

  /* dst is buffer */

  test.case = 'dst is empty buffer, src = undefined';
  var dst = new U8x();
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty buffer, src is empty array';
  var dst = new U8x();
  var src = [];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [] );

  test.case = 'dst is buffer, src = undefined';
  var dst = new U8x( [ 1, 2, 0, 78 ] );
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is number';
  var dst = new I16x( [ 1, 2, 0, 78 ] );
  var src = 32;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 32, 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is empty array';
  var dst = new U16x( [ 1, 2, 0, 78 ] );
  var src = [];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src = array';
  var dst = new I32x( [ 1, 2, 0, 78 ] );
  var src = [ 'str', null, undefined ];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 'str', null, undefined, 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is buffer';
  var dst = new U32x( [ 1, 2, 0, 78 ] );
  var src = new F32x( [ 1, 2, 3, 4 ] );
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 1, 2, 3, 4, 1, 2, 0, 78 ] );

  /* dst not undefined, not longLike */

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = null;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null, null ] );

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, null );

  test.case = 'dst is null, src is empty array';
  var dst = null;
  var src = [];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = '';
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ '', 'str' ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = undefined;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is number, src is string';
  var dst = 1;
  var src = [ '', 2, [], [ { 'a' : 2 } ] ];
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ '', 2, [], [ { 'a' : 2 } ], 1 ] );

  test.case = 'dst is map, src is buffer';
  var dst = { 'a' : 1 };
  var src = new U8x( [ 10, 20, 30 ] );
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [ 10, 20, 30, { 'a' : 1 } ] );

  /* */

  test.case = 'dst === src';
  var arr = [ 1, 2, 'str' ];
  var dst = arr;
  var src = arr;
  var got = _.scalarPrepend( dst, src );
  test.identical( got, [  1, 2, 'str',  1, 2, 'str' ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'wrong arguments.length';
  test.shouldThrowErrorSync( () => _.scalarPrepend() );
  test.shouldThrowErrorSync( () => _.scalarPrepend( 1 ) );
  test.shouldThrowErrorSync( () => _.scalarPrepend( 1, 2, 'str' ) );
}

//

function scalarPrependOnce( test )
{
  test.case = 'dst is undefined, src = undefined';
  var dst = undefined;
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is undefined, src is longLike';
  var dst = undefined;
  var src = [ null, '', 1, [], [ 1, [ 2 ] ], null, 1 ];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ '', [], [ 1, [ 2 ] ], null, 1 ] );

  test.case = 'dst is undefined, src = unroll';
  var dst = undefined;
  var src = _.unroll.make( [ null, '', 1 , null, '', 1 ] );
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src = argumentsArray';
  var dst = undefined;
  var src = _.argumentsArray.make( [ null, '', 1 , null, '', 1 ] );
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ null, '', 1 ] );

  test.case = 'dst is undefined, src is buffer';
  var dst = undefined;
  var src = new F32x( [ 0, 2, 10, 0, 2, 10 ] );
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 0, 2, 10 ] );

  test.case = 'dst is undefined, src is string';
  var dst = undefined;
  var src = 'str';
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is undefined, src is map';
  var dst = undefined;
  var src = { 'a' : 'str' };
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, { 'a' : 'str' } );

  /* dst is array */

  test.case = 'dst is empty array, src is empty array';
  var dst = [];
  var src = [];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty array, src is null';
  var dst = [];
  var src = null;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is array, src = undefined';
  var dst = [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ], [ 1, [ 2 ] ] ];
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is array, src is string';
  var dst = [ 1, null, 'str', [], 1 ];
  var src = 'str';
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [], 1 ] );
  test.true( got === dst );

  test.case = 'dst is array, src is map';
  var dst = [ 1, null, 'str', [], { 'a' : 1 } ];
  var src = { 'a' : 1 };
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ { 'a' : 1 }, 1, null, 'str', [], { 'a' : 1 } ] );
  test.true( got === dst );

  test.case = 'dst is array, src = array';
  var dst = [ 1, null, 'str', [] ];
  var src = [ 'str', 1, undefined ];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ undefined, 1, null, 'str', [] ] );
  test.true( got === dst );

  /* dst is unroll */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.unroll.make( [] );
  var src = [];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [] );
  test.true( got === dst );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.unroll.make( [] );
  var src = null;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.unroll.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is string';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [] ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src is map';
  var dst = _.unroll.make( [ 1, null, 'str', [], { 'a' : 1 } ] );
  var src = { 'a' : 1 };
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ { 'a' : 1 }, 1, null, 'str', [], { 'a' : 1 } ] );
  test.true( got === dst );

  test.case = 'dst is unroll, src = array';
  var dst = _.unroll.make( [ 1, null, 'str', [] ] );
  var src = [ 'str', 1, undefined ];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ undefined, 1, null, 'str', [] ] );
  test.true( got === dst );

  /* dst is argumentsArray */

  test.case = 'dst is empty unroll, src is empty array';
  var dst = _.argumentsArray.make( [] );
  var src = [];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty unroll, src is null';
  var dst = _.argumentsArray.make( [] );
  var src = null;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is unroll, src = undefined';
  var dst = _.argumentsArray.make( [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, null, 'str', '', 1, [], [ 1, [ 2 ] ] ] );

  test.case = 'dst is unroll, src is string';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = 'str';
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, null, 'str', [] ] );

  test.case = 'dst is unroll, src is map';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [], { 'a' : 1 } ] );
  var src = { 'a' : 1 };
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ { 'a' : 1 }, 1, null, 'str', [], { 'a' : 1 } ] );

  test.case = 'dst is unroll, src = array';
  var dst = _.argumentsArray.make( [ 1, null, 'str', [] ] );
  var src = [ 'str', 1, undefined ];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ undefined, 1, null, 'str', [] ] );

  /* dst is buffer */

  test.case = 'dst is empty buffer, src = undefined';
  var dst = new U8x();
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is empty buffer, src is empty array';
  var dst = new U8x();
  var src = [];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [] );

  test.case = 'dst is buffer, src = undefined';
  var dst = new U8x( [ 1, 2, 0, 78 ] );
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is number';
  var dst = new I16x( [ 1, 2, 0, 78 ] );
  var src = 0;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is empty array';
  var dst = new U16x( [ 1, 2, 0, 78 ] );
  var src = [];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src = array';
  var dst = new I32x( [ 1, 2, 0, 78 ] );
  var src = [ 'str', 2, 78 ];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 'str', 1, 2, 0, 78 ] );

  test.case = 'dst is buffer, src is buffer';
  var dst = new U32x( [ 1, 2, 0, 78 ] );
  var src = new F32x( [ 1, 2, 3, 4 ] );
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 3, 4, 1, 2, 0, 78 ] );

  /* dst not undefined, not longLike */

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = null;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is null, src is null';
  var dst = null;
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, null );

  test.case = 'dst is null, src is empty array';
  var dst = null;
  var src = [];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ null ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = 'str';
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 'str' ] );

  test.case = 'dst is string, src is string';
  var dst = 'str';
  var src = undefined;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, 'str' );

  test.case = 'dst is number, src is string';
  var dst = 1;
  var src = [ '', 1, [], [ { 'a' : 2 } ] ];
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ '', [], [ { 'a' : 2 } ], 1 ] );

  test.case = 'dst is map, src is buffer';
  var dst = { 'a' : 1 };
  var src = new U8x( [ 10, 20, 30 ] );
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [ 10, 20, 30, { 'a' : 1 } ] );

  /* */

  test.case = 'dst === src';
  var arr = [ 1, 2, 'str' ];
  var dst = arr;
  var src = arr;
  var got = _.scalarPrependOnce( dst, src );
  test.identical( got, [  1, 2, 'str' ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'wrong arguments.length';
  test.shouldThrowErrorSync( () => _.scalarPrependOnce() );
  test.shouldThrowErrorSync( () => _.scalarPrependOnce( 1 ) );
  test.shouldThrowErrorSync( () => _.scalarPrependOnce( 1, 2, 'str' ) );
}

//

function scalarToVector( test )
{

  test.case = 'nothing';
  var got = _.scalarToVector( [], 0 );
  var expected = [];
  test.identical( got, expected );

  test.case = 'static array';
  var got = _.scalarToVector( 3, 7 );
  var expected = [ 3, 3, 3, 3, 3, 3, 3 ];
  test.identical( got, expected );

  test.case = 'original array';
  var got = _.scalarToVector( [ 3, 7, 13 ], 3 );
  var expected = [ 3, 7, 13 ];
  test.identical( got, expected );

  /**/

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.scalarToVector();
  });

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( function()
  {
    _.scalarToVector( [ 1, 2, 3 ] );
  });

  test.case = 'extra argument';
  test.shouldThrowErrorSync( function()
  {
    _.scalarToVector( [ 1, 2, 3 ], 3, 'redundant argument' );
  });

  test.case = 'wrong type of arguments';
  test.shouldThrowErrorSync( function()
  {
    _.scalarToVector('wrong argument', 'wrong argument');
  });

  test.case = 'second argument too much';
  test.shouldThrowErrorSync( function()
  {
    _.scalarToVector( [ 1, 2, 3 ], 4 );
  });

  test.case = 'first three arguments are not wrapped into array';
  test.shouldThrowErrorSync( function()
  {
    _.scalarToVector( 1, 2, 3, 3 );
  });
};

// --
// tests
// --

function entityTimeExperiment( test )
{
  var srcArray = new U8x( 500000000 );
  var onEach = ( e, k, src ) => e;

  /* */

  testTime( entityEachLongsFor, 20, srcArray, onEach );
  testTime( entityEachLongsForEach, 20, srcArray, onEach );

  testTime( entityAnyLongsFor, 20, srcArray, onEach );
  testTime( entityAnyLongsSome, 20, srcArray, onEach );

  for( let i = srcArray.length; i >= 0; i-- )
  srcArray[ i ] = ( Math.random() + 0.01 ) * 250;
  testTime( entityAllLongsFor, 30, srcArray, onEach );
  testTime( entityAllLongsEvery, 30, srcArray, onEach );

  /*|         |                                Running time                                 |
    |         |---------------------------|------------------------|------------------------|
    |  NodeJS | for     | Array.forEach() | for    | Array.every() | for    | Array.some()  |
    |---------|---------|-----------------|--------|---------------|--------|---------------|
    |  v8     | 17,3 s  | 128,8 s         | 33.7 s | 205,5 s       | 21,9 s | 134,8 s       |
    |         | 17,3 s  | 128,9 s         | 33.5 s | 204,8 s       | 21,8 s | 143,3 s       |
    |---------|---------|-----------------|--------|---------------|--------|---------------|
    |  v9     | 17,2 s  | 129,2 s         | 32,8 s | 203,2 s       | 21,9 s | 134,0 s       |
    |         | 17,4 s  | 129,3 s         | 35.7 s | 204,5 s       | 21,8 s | 133,7 s       |
    |---------|---------|-----------------|--------|---------------|--------|---------------|
    |  v10    | 12,8 s  | 137,1 s         | 26,9 s | 208,6 s       | 17,2 s | 142,6 s       |
    |         | 13,0 s  | 138,8 s         | 26,6 s | 210,1 s       | 17,1 s | 143.2 s       |
    |---------|---------|-----------------|--------|---------------|--------|---------------|
    |  v12    | 8,5 s   | 184.5 s         | 26,9 s | 369,7 s       | 13,0 s | 216,5 s       |
    |         | 8,4 s   | 184,4 s         | 26,7 s | 329,1 s       | 26,7 s | 223,4 s       |
  */


  /* testing subroutine */

  function testTime( func, times, arg1, arg2 )
  {
    var timeStart = _.time.now();

    for( let i = times; i > 0; i-- )
    var result = func( arg1, arg2 );

    var timeEnd = _.time.now();

    test.true( timeEnd - timeStart > 30000 )
    console.log( timeEnd - timeStart );
  }

  /* */

  function entityEachLongsFor( src, onEach )
  {
    for( let k = 0 ; k < src.length ; k++ )
    onEach( src[ k ], k, src );

    return src
  }

  /* */

  function entityEachLongsForEach( src, onEach )
  {
    src.forEach( onEach );

    return src;
  }

  /* */

  function entityAllLongsFor( src, onEach )
  {
    let result;
    if( _.routine.is( onEach ) )
    {
      for( let k = 0 ; k < src.length ; k++ )
      {
        result = onEach( src[ k ], k, src );
        if( !result )
        return result;
      }
    }
    else
    {
      for( let k = 0 ; k < src.length ; k++ )
      {
        result = src[ k ];
        if( !result )
        return result;
      }
    }

    return true;
  }

  /* */

  function entityAllLongsEvery( src, onEach )
  {
    let result;
    if( _.routine.is( onEach ) )
    result = src.every( onEach );
    else
    result = src.every();

    return result;
  }

  /* */

  function entityAnyLongsFor( src, onEach )
  {
    let result;
    if( _.routine.is( onEach ) )
    {
      for( let k = 0 ; k < src.length ; k++ )
      {
        result = onEach( src[ k ], k, src );
        if( result )
        return result;
      }
    }
    else
    {
      for( let k = 0 ; k < src.length ; k++ )
      {
        result = src[ k ];
        if( result )
        return result;
      }
    }

    return true;
  }

  /* */

  function entityAnyLongsSome( src, onEach )
  {
    let result;
    if( _.routine.is( onEach ) )
    result = src.some( onEach );
    else
    result = src.some();

    return result;
  }

}
entityTimeExperiment.experimental = 1;
entityTimeExperiment.timeOut = 1800000;

//

function entityEach( test )
{
  test.open( 'src is an ArrayLike');

  test.case = 'empty arrayLike objects';
  var got;
  var src = [];
  _.entityEach( src, ( v ) => got = typeof v );
  test.identical( got, undefined );

  var got = [];
  var src = _.unroll.make( 0 );
  _.entityEach( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, [] );
  test.false( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  var got = [];
  var src = _.argumentsArray.make( 0 );
  _.entityEach( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  var got = [];
  var src = new Array( 0 );
  _.entityEach( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, src );
  test.true( _.arrayIs( got ) );

  var got = [];
  var src = new F32x( 0 );
  _.entityEach( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  /* */

  test.case = 'not empty arrayLike objects';

  var got = [];
  var src = [ 0, 1, 2 ];
  _.entityEach( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );

  var got = [];
  var src = _.unroll.make( [ 0, 1, _.unroll.make( [ 2 ] ) ] );
  _.entityEach( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.false( _.unrollIs( got ) );
  test.true( _.arrayIs( src ) );

  var got = [];
  var src = _.argumentsArray.make( [ 0, 1, 2 ] );
  _.entityEach( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.true( _.longIs( src ) );

  var got = [];
  var src = new Array( 0, 1, 2 );
  _.entityEach( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.true( _.longIs( src ) );

  var got = [];
  var src = new F32x( [ 0, 1, 2 ] );
  _.entityEach( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.true( _.longIs( src ) );

  /* */

  test.case = 'routine counter';

  var onEach = function( v, i )
  {
    if( _.strIs( v ) && i >= 0 )
    got += 10;
    else
    got -= 1;
  }

  var got = 0;
  _.entityEach( 1, onEach );
  test.identical( got, -1 );

  var got = 0;
  _.entityEach( 'abc', onEach );
  test.identical( got, -1 );

  var got = 0;
  _.entityEach( [ 'abc', 1, 'ab', 'a' ], onEach );
  test.identical( got, 29 );

  var got = 0;
  _.entityEach( [ { 'a' : 1 }, { 'b' : 2 } ], onEach );
  test.identical( got, -2 );

  var got = 0;
  var src = _.unroll.from( [ 1, 'str', _.unroll.make( [ 2, 'str' ] ) ] );
  _.entityEach( src, onEach );
  test.identical( got, 18 );

  var got = 0;
  var src = _.argumentsArray.make( [ 1, 'str', [ 2, 'str' ] ] );
  _.entityEach( src, onEach );
  test.identical( got, 8 );

  var got = 0;
  var src = new Array( 1, 'str', [ 2, 'str' ] );
  _.entityEach( src, onEach );
  test.identical( got, 8 );

  var got = 0;
  var src = new F32x( [ 1, 1, [ 2 ] ] );
  _.entityEach( src, onEach );
  test.identical( got, -3 );

  test.case = 'Third argument in onEach'
  var onEach = function( v, i, src )
  {
    if( _.longIs( src ) )
    got = src;
    else
    got += 10;
  }

  var got;
  var src = [ 0, 1, 3, 5 ];
  _.entityEach( src, onEach );
  test.identical( got, src );

  var got;
  var src = _.unroll.make( [ 0, 1, _.unroll.from( [ 3, 5 ] ) ] );
  _.entityEach( src, onEach );
  test.identical( got, [ 0, 1, 3, 5 ] );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  var got;
  var src = _.argumentsArray.make( [ 0, 1, [ 3, 5 ] ] );
  _.entityEach( src, onEach );
  test.identical( got, src );
  test.true( _.longIs( got ) );

  var got;
  var src = new Array( 1, 2, null, true );
  _.entityEach( src, onEach );
  test.identical( got, src );
  test.true( _.longIs( got ) );

  var got;
  var src = new F32x( [ 1, 2, 1, 3 ] );
  _.entityEach( src, onEach );
  test.equivalent( got, [ 1, 2, 1, 3 ] );
  test.true( _.longIs( got ) );

  test.close( 'src is an ArrayLike');

  /* - */

  test.open( 'src is an object' );

  var got = {};
  _.entityEach( {}, ( v ) => got = v );
  test.identical( got, {} );

  var got = {};
  _.entityEach( Object.create( null ), ( v, k ) => got[ k ] = v + k );
  test.identical( got, {} );

  var got = {};
  _.entityEach( { 'a' : 1, 'b' : 3, 'c' : 5 }, ( v, k ) => got[ k ] = v * v + k );
  test.identical( got, { 'a' : '1a', 'b' : '9b', 'c' : '25c' } );

  /* */

  test.case = 'routine counter';

  var onEach = function( v, k )
  {
    if( _.strIs( v ) && k )
    got += 10;
    else
    got -= 1;
  }

  var got = 0;
  _.entityEach( 'abc', onEach );
  test.identical( got, -1 );

  var got = 0;
  _.entityEach( { 'a' : 'abc', 'b' : { 'a' : 1 }, 'c' : [ null ], 'd' : undefined }, onEach );
  test.identical( got, 7 );

  var got = 0;
  _.entityEach( { 'a' : 'abc', 'b' : 1, 'c' : 'ab', 'd' : 'a' }, onEach );
  test.identical( got, 29 );

  /* */

  test.case = 'Third argument in onEach'
  var onEach = function( v, k, src )
  {
    if( _.object.isBasic( src ) )
    got = src;
    else
    got += 10;
  }

  var got = {};
  var src = { 'ak' : 'a', 'bk' : 'b', 'ck' : 'c' };
  _.entityEach( src, onEach );
  test.identical( got, src );

  test.close( 'src is an object' );

  /* - */

  test.case = 'src is not ArrayLike or ObjectLike';

  var got;
  _.entityEach( null, ( v ) => got = typeof v );
  test.identical( got, 'object' );

  var got;
  _.entityEach( 1, ( v ) => got = typeof v );
  test.identical( got, 'number' );

  var got;
  _.entityEach( 'a', ( v ) => got = v + 2 );
  test.identical( got, 'a2' );

  var got;
  _.entityEach( function b(){ return 'a'}, ( v ) => got = typeof v );
  test.identical( got, 'function' );

  var got;
  _.entityEach( function b(){ return 'a'}, ( v, i ) => got = typeof v + ' ' + typeof i );
  test.identical( got, 'function undefined' );

  /* */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityEach() );

  test.case = 'only one argument';
  test.shouldThrowErrorSync( () => _.entityEach( [ 'a' ] ) );

  test.case = 'too many arguments';
  test.shouldThrowErrorSync( () => _.entityEach( [ 'a' ], ( a ) => a, ( b ) => b ) );

  test.case = 'onEach has more then three args';
  test.shouldThrowErrorSync( () => _.entityEach( [ 1 ], ( a, b, c, d ) => a + b + c + d ) );

  test.case = 'onEach is not a routine';
  test.shouldThrowErrorSync( () => _.entityEach( { 'a' : 2 }, [] ) );
}

//
//
// function entityEachKey( test )
// {
//   test.open( 'src is an ArrayLike');
//
//   test.case = 'empty arrayLike objects'
//   var got;
//   var src = [];
//   _.entityEachKey( src, ( v ) => got = typeof v );
//   test.identical( got, undefined );
//
//   var got = [];
//   var src = _.unroll.make( 0 );
//   _.entityEachKey( src, ( v, i ) => got[ i ] = v + i );
//   test.identical( got, [] );
//   test.false( _.unrollIs( got ) );
//   test.true( _.arrayIs( got ) );
//
//   var got = [];
//   var src = _.argumentsArray.make( 0 );
//   _.entityEachKey( src, ( v, i ) => got[ i ] = v + i );
//   test.identical( got, [] );
//   test.true( _.longIs( src ) );
//
//   var got = [];
//   var src = new Array( 0 );
//   _.entityEachKey( src, ( v, i ) => got[ i ] = v + i );
//   test.identical( got, src );
//   test.true( _.longIs( src ) );
//
//   var got = [];
//   var src = new F32x( 0 );
//   _.entityEachKey( src, ( v, i ) => got[ i ] = v + i );
//   test.identical( got, [] );
//
//   /* */
//
//   test.case = 'not empty arrayLike objects';
//
//   var got = [];
//   var src = [ 0, 1, 2 ];
//   _.entityEachKey( src, ( v, u, i ) => got[ i ] = v * v );
//   test.identical( got, [ 0, 1, 4 ] );
//
//   var got = [];
//   var src = _.unroll.make( [ 0, 1, _.unroll.make( [ 2 ] ) ] );
//   _.entityEachKey( src, ( v, u, i ) => got[ i ] = v * v );
//   test.identical( got, [ 0, 1, 4 ] );
//   test.false( _.unrollIs( got ) );
//   test.true( _.arrayIs( src ) );
//
//   var got = [];
//   var src = _.argumentsArray.make( [ 0, 1, 2 ] );
//   _.entityEachKey( src, ( v, u, i ) => got[ i ] = v * v );
//   test.identical( got, [ 0, 1, 4 ] );
//   test.true( _.longIs( src ) );
//
//   var got = [];
//   var src = new Array( 0, 1, 2 );
//   _.entityEachKey( src, ( v, u, i ) => got[ i ] = v * v );
//   test.identical( got, [ 0, 1, 4 ] );
//   test.true( _.longIs( src ) );
//
//   var got = [];
//   var src = new F32x( [ 0, 1, 2 ] );
//   _.entityEachKey( src, ( v, u, i ) => got[ i ] = v * v );
//   test.identical( got, [ 0, 1, 4 ] );
//   test.true( _.longIs( src ) );
//
//   /* */
//
//   test.case = 'routine counter';
//
//   var onEach = function( v, u )
//   {
//     if( _.strIs( v ) && u === undefined )
//     got += 10;
//     else
//     got -= 1;
//   }
//
//   var got = 0;
//   _.entityEachKey( 1, onEach );
//   test.identical( got, -1 );
//
//   var got = 0;
//   _.entityEachKey( 'abc', onEach );
//   test.identical( got, 10 );
//
//   var got = 0;
//   _.entityEachKey( [ 'abc', 1, 'ab', 'a',  { 'a' : 1 } ], onEach );
//   test.identical( got, 28 );
//
//   var got = 0;
//   var src = _.unroll.from( [ 1, 'str', _.unroll.make( [ { 'a' : 'abc' }, 'str' ] ) ] );
//   _.entityEachKey( src, onEach );
//   test.identical( got, 18 );
//
//   var got = 0;
//   var src = _.argumentsArray.make( [ 1, 'str', [ [ 'abc' ], 'str' ] ] );
//   _.entityEachKey( src, onEach );
//   test.identical( got, 8 );
//
//   var got = 0;
//   var src = new Array( 1, 'str', [ 2, 'str' ] );
//   _.entityEachKey( src, onEach );
//   test.identical( got, 8 );
//
//   var got = 0;
//   var src = new F32x( [ 1, 1, [ 2 ] ] );
//   _.entityEachKey( src, onEach );
//   test.identical( got, -3 );
//
//   test.case = 'Third argument in onEach'
//   var onEach = function( v, u, i )
//   {
//     if( _.longIs( arguments[ 3 ] ) )
//     got = src;
//     else
//     got += 10;
//   }
//
//   var got;
//   var src = [ 0, 1, 3, 5 ];
//   _.entityEachKey( src, onEach );
//   test.identical( got, src );
//
//   var got;
//   var src = _.unroll.make( [ 0, 1, _.unroll.from( [ 3, 5 ] ) ] );
//   _.entityEachKey( src, onEach );
//   test.identical( got, [ 0, 1, 3, 5 ] );
//   test.true( _.unrollIs( got ) );
//   test.true( _.arrayIs( got ) );
//
//   var got;
//   var src = _.argumentsArray.make( [ 0, 1, [ 3, 5 ] ] );
//   _.entityEachKey( src, onEach );
//   test.identical( got, src );
//   test.true( _.longIs( got ) );
//
//   var got;
//   var src = new Array( 1, 2, null, true );
//   _.entityEachKey( src, onEach );
//   test.identical( got, src );
//   test.true( _.longIs( got ) );
//
//   var got;
//   var src = new F32x( [ 1, 2, 1, 3 ] );
//   _.entityEachKey( src, onEach );
//   test.equivalent( got, [ 1, 2, 1, 3 ] );
//   test.true( _.longIs( got ) );
//
//   test.close( 'src is an ArrayLike');
//
//   /* - */
//
//   test.open( 'src is an ObjectLike' );
//
//   test.case = 'not onEach';
//   var got = {};
//   _.entityEachKey( {}, ( v ) => got = v );
//   test.identical( got, {} );
//
//   var got = {};
//   _.entityEachKey( Object.create( null ), ( v, k ) => got[ k ] = v + k );
//   test.identical( got, {} );
//
//   var got = {};
//   _.entityEachKey( { 'a' : 1, 'b' : 3, 'c' : 5 }, ( k, v ) => got[ k ] = v * v + k );
//   test.identical( got, { 'a' : '1a', 'b' : '9b', 'c' : '25c' } );
//
//   /* */
//
//   test.case = 'routine counter';
//   var onEach = function( k, v )
//   {
//     if( _.strIs( v ) && k )
//     got += 10;
//     else
//     got -= 1;
//   }
//
//   var got = 0;
//   _.entityEachKey( 'abc', onEach );
//   test.identical( got, -1 );
//
//   var got = 0;
//   _.entityEachKey( { 'a' : 'abc' }, onEach );
//   test.identical( got, 10 );
//
//   var got = 0;
//   _.entityEachKey( { 'a' : 'abc', 'b' : 1, 'c' : 'ab', 'd' : null }, onEach );
//   test.identical( got, 18 );
//
//   var got = 0;
//   _.entityEachKey( { 'a' : [ 'a', 'b' ], 'b' : { 'e' : 1 } }, onEach );
//   test.identical( got, -2 );
//
//   /* */
//
//   test.case = 'Third argument in onEach'
//   var onEach = function( v, k, i )
//   {
//     got[ i ] = v + k;
//   }
//
//   var got = {};
//   _.entityEachKey( { 'ak' : 'a', 'bk' : 'b', 'ck' : 'c' }, onEach );
//   test.identical( got, { '0' : 'a1', '1' : 'b2', '2' : 'c3' } );
//
//   test.close( 'src is an ObjectLike' );
//
//   /* - */
//
//   test.case = 'src is not ArrayLike or ObjectLike';
//
//   var got;
//   _.entityEachKey( null, ( v ) => got = typeof v );
//   test.identical( got, 'object' );
//
//   var got;
//   _.entityEachKey( 1, ( v ) => got = typeof v );
//   test.identical( got, 'number' );
//
//   var got;
//   _.entityEachKey( 'a', ( v ) => got = v + 2 );
//   test.identical( got, 'a2' );
//
//   var got;
//   _.entityEachKey( function b(){ return 'a'}, ( v, i ) => got = typeof v + ' ' + typeof i );
//   test.identical( got, 'function undefined' );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.entityEachKey() );
//
//   test.case = 'only one argument';
//   test.shouldThrowErrorSync( () => _.entityEachKey( [ 'a' ] ) );
//
//   test.case = 'too many arguments';
//   test.shouldThrowErrorSync( () => _.entityEachKey( [ 'a' ], ( a ) => a, ( b ) => b ) );
//
//   test.case = 'onEach has more then three arg';
//   test.shouldThrowErrorSync( () => _.entityEachKey( [ 1 ], ( a, b, c, d ) => a + b + c + d ) );
//
//   test.case = 'onEach is not a routine';
//   test.shouldThrowErrorSync( () => _.entityEachKey( { 'a' : 2 }, [] ) );
// }

//

function entityEachOwn( test )
{
  test.open( 'src is an ArrayLike');

  test.case = 'empty arrayLike objects';
  var got;
  var src = [];
  _.entityEachOwn( src, ( v ) => got = typeof v );
  test.identical( got, undefined );

  var got = [];
  var src = _.unroll.make( 0 );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, [] );
  test.false( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  var got = [];
  var src = _.argumentsArray.make( 0 );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  var got = [];
  var src = new Array( 0 );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, src );
  test.true( _.arrayIs( got ) );

  var got = [];
  var src = new F32x( 0 );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v + i );
  test.identical( got, [] );
  test.true( _.arrayIs( got ) );

  /* */

  test.case = 'not empty arrayLike objects';

  var got = [];
  var src = [ 0, 1, 2 ];
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );

  var got = [];
  var src = _.unroll.make( [ 0, 1, _.unroll.make( [ 2 ] ) ] );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.false( _.unrollIs( got ) );
  test.true( _.arrayIs( src ) );

  var got = [];
  var src = _.argumentsArray.make( [ 0, 1, 2 ] );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.true( _.longIs( src ) );

  var got = [];
  var src = new Array( 0, 1, 2 );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.true( _.longIs( src ) );

  var got = [];
  var src = new F32x( [ 0, 1, 2 ] );
  _.entityEachOwn( src, ( v, i ) => got[ i ] = v * v + i );
  test.identical( got, [ 0, 2, 6 ] );
  test.true( _.longIs( src ) );

  /* */

  test.case = 'routine counter';

  var onEach = function( v, i )
  {
    if( _.strIs( v ) && i >= 0 )
    got += 10;
    else
    got -= 1;
  }

  var got = 0;
  _.entityEachOwn( 1, onEach );
  test.identical( got, -1 );

  var got = 0;
  _.entityEachOwn( 'abc', onEach );
  test.identical( got, -1 );

  var got = 0;
  _.entityEachOwn( [ 'abc', 1, 'ab', 'a' ], onEach );
  test.identical( got, 29 );

  var got = 0;
  _.entityEachOwn( [ { 'a' : 1 }, { 'b' : 2 } ], onEach );
  test.identical( got, -2 );

  var got = 0;
  var src = _.unroll.from( [ 1, 'str', _.unroll.make( [ 2, 'str' ] ) ] );
  _.entityEachOwn( src, onEach );
  test.identical( got, 18 );

  var got = 0;
  var src = _.argumentsArray.make( [ 1, 'str', [ 2, 'str' ] ] );
  _.entityEachOwn( src, onEach );
  test.identical( got, 8 );

  var got = 0;
  var src = new Array( 1, 'str', [ 2, 'str' ] );
  _.entityEachOwn( src, onEach );
  test.identical( got, 8 );

  var got = 0;
  var src = new F32x( [ 1, 1, [ 2 ] ] );
  _.entityEachOwn( src, onEach );
  test.identical( got, -3 );

  test.case = 'Third argument in onEach'
  var onEach = function( v, i, src )
  {
    if( _.longIs( src ) )
    got = src;
    else
    got += 10;
  }

  var got;
  var src = [ 0, 1, 3, 5 ];
  _.entityEachOwn( src, onEach );
  test.identical( got, src );

  var got;
  var src = _.unroll.make( [ 0, 1, _.unroll.from( [ 3, 5 ] ) ] );
  _.entityEachOwn( src, onEach );
  test.identical( got, [ 0, 1, 3, 5 ] );
  test.true( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  var got;
  var src = _.argumentsArray.make( [ 0, 1, [ 3, 5 ] ] );
  _.entityEachOwn( src, onEach );
  test.identical( got, src );
  test.true( _.longIs( got ) );

  var got;
  var src = new Array( 1, 2, null, true );
  _.entityEachOwn( src, onEach );
  test.identical( got, src );
  test.true( _.longIs( got ) );

  var got;
  var src = new F32x( [ 1, 2, 1, 3 ] );
  _.entityEachOwn( src, onEach );
  test.equivalent( got, [ 1, 2, 1, 3 ] );
  test.true( _.longIs( got ) );

  test.close( 'src is an ArrayLike');

  /* - */

  test.open( 'src is an ObjectLike' );

  var got ={};
  _.entityEachOwn( {}, ( v ) => got = v );
  test.identical( got, {} );

  var got = {};
  var src = Object.create( null );
  var src2 = Object.create( src );
  _.entityEachOwn( src, ( v, k ) => got[ k ] = v + k );
  test.identical( got, {} );
  _.entityEachOwn( src2, ( v, k ) => got[ k ] = v + k );
  test.identical( got, {} );

  var got = {};
  var src = Object.create( null );
  src.a = 1;
  src.b = 3;
  var src2 = Object.create( src );
  _.entityEachOwn( src2, ( v, k ) => got[ k ] = v * v + k );
  test.identical( got, {} );
  _.entityEachOwn( src, ( v, k ) => got[ k ] = v * v + k );
  test.identical( got, { 'a' : '1a', 'b' : '9b' } );

  test.case = 'routine counter';

  var onEach = function( v, k )
  {
    if( _.strIs( v ) && k )
    got += 10;
    else
    got -= 1;
  }

  var got = 0;
  _.entityEachOwn( 1, onEach );
  test.identical( got, -1 );

  var got = 0;
  _.entityEachOwn( { 'a' : 'abc', 'b' : 1, 'c' : 'ab', 'd' : null }, onEach );
  test.identical( got, 18 );

  var got = 0;
  var src = { 'name' : 'object', 'toString' : 1, 'toSource' : null };
  var src2 = Object.create( src );
  _.entityEachOwn( src2, onEach );
  test.identical( got, 0 );
  _.entityEachOwn( src, onEach );
  test.identical( got, 8 );

  var got = 0;
  var src = Object.create( null );
  src.a = [ 'a', 'b' ];
  src.b = { 'a' : 1 };
  var src2 = Object.create( src );
  src2.c = 'str';
  test.identical( src.a, src2.a );
  test.identical( src.b, src2.b );
  _.entityEachOwn( src, onEach );
  test.identical( got, -2 );
  _.entityEachOwn( src2, onEach );
  test.identical( got, 8 );

  test.case = 'Third argument in onEach'
  var onEach = function( v, k, src )
  {
    if( _.object.isBasic( src ) )
    got = src;
    else
    got += 10;
  }

  var got = {};
  var src = Object.create( null );
  src.a = 'str';
  var src2 = Object.create( src );
  _.entityEachOwn( src2, onEach );
  test.identical( got, {} );
  _.entityEachOwn( src, onEach );
  test.identical( got, src );

  test.close( 'src is an ObjectLike' );

  /* - */

  test.case = 'src is not ArrayLike or ObjectLike';

  var got;
  _.entityEachOwn( null, ( v ) => got = typeof v );
  test.identical( got, 'object' );

  var got;
  _.entityEachOwn( 1, ( v ) => got = typeof v );
  test.identical( got, 'number' );

  var got;
  _.entityEachOwn( 'a', ( v ) => got = v + 2 );
  test.identical( got, 'a2' );

  var got;
  _.entityEachOwn( function b(){ return 'a'}, ( v, i ) => got = typeof v + ' ' + typeof i );
  test.identical( got, 'function undefined' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityEachOwn() );

  test.case = 'only one argument';
  test.shouldThrowErrorSync( () => _.entityEachOwn( [ 'a' ] ) );

  test.case = 'too many arguments';
  test.shouldThrowErrorSync( () => _.entityEachOwn( [ 'a' ], ( a ) => a, ( b ) => b ) );

  test.case = 'onEach has more then three arg';
  test.shouldThrowErrorSync( () => _.entityEachOwn( [ 1 ], ( a, b, c, d ) => a + b + c + d ) );

  test.case = 'onEach is not a routine';
  test.shouldThrowErrorSync( () => _.entityEachOwn( { 'a' : 2 }, [] ) );
}

//

function entityOnlyOnlyDst( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOnly( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, _.nothing';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var got = _.entityOnly( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityOnly( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityOnly( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOnly( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOnly( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOnly( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityOnly( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityOnly( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOnly( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOnly( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOnly( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOnly( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOnly( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOnly( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOnly( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOnly( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOnly( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityOnly( dst, undefined, '*/x' );
  test.true( dst === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = 'dst';
  // var dst = 'dst';
  // var got = _.entityOnly( dst, undefined, '*/length' );
  // test.true( dst === got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOnly( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOnly( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOnly( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityOnly() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entityOnly( [ 1, 2 ], [ 0, 1 ], ( e ) => e, 'extra' ) );

  test.case = 'wrong onEach type';
  test.shouldThrowErrorSync( () => _.entityOnly( [ 1, 2 ], [ 0, 1 ], 'wrong' ) );
  test.shouldThrowErrorSync( () => _.entityOnly( [ 1, 2 ], [ 0, 1 ], [] ) );

  test.case = 'onEach.length > 3';
  test.shouldThrowErrorSync( () => _.entityOnly( [ 1, 2 ], [ 0, 1 ], ( a, b, c, d ) => a - b + c - d ) );
}

//

function entityOnlyOnlySrc( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOnly( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityOnly( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityOnly( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOnly( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOnly( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOnly( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityOnly( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityOnly( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOnly( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOnly( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOnly( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOnly( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOnly( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOnly( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOnly( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOnly( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOnly( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityOnly( null, src, '*/x' );
  test.true( src === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'src - str';
  // var exp = 'src';
  // var src = 'src';
  // var got = _.entityOnly( null, src, '*/length' );
  // test.true( src === got );
  // test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOnly( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOnly( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOnly( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityOnlyBothSame( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = src;
  var got = _.entityOnly( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = src;
  var got = _.entityOnly( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = exp;
  var got = _.entityOnly( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityOnly( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityOnly( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = dst;
  var got = _.entityOnly( dst, src, '*/x' );
  test.true( src === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = 'dst';
  // var dst = 'dst';
  // var src = dst;
  // var got = _.entityOnly( dst, src, '*/length' );
  // test.true( src === got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOnly( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOnly( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOnly( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityOnlyBoth( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map, src - map';
  var exp = { 'true' : true, 'null' : null, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'true' : true, 'null' : null, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ false, 0, 'str', [ 1 ] ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ] ];
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ false, 0, 'str', [ 1 ], _.nothing ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], _.nothing, { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ], _.nothing ];
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - obj, src - number';
  var exp = undefined;
  var dst = new Constructor1();
  var src = 0;
  var got = _.entityOnly( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = 'src';
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - obj';
  var exp = 'dst';
  var dst = 'dst';
  var src = new Constructor1();
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityOnly( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - str';
  var exp = '';
  var dst = '';
  var src = 'src';
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - undefined';
  var exp = undefined;
  var dst = false;
  var src = undefined;
  var got = _.entityOnly( dst, src );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - number';
  var exp = false;
  var dst = false;
  var src = 2;
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - false';
  var exp = undefined;
  var dst = 0;
  var src = false;
  var got = _.entityOnly( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - true';
  var exp = 0;
  var dst = 0;
  var src = true;
  var got = _.entityOnly( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map, src - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'dst::empty' };
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, 2, 3 ];
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src  = 'src';
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, dst - str';
  var exp = undefined;
  var dst = '';
  var src = 'src';
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - true';
  var exp = undefined;
  var dst = false;
  var src  = true;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - 1';
  var exp = undefined;
  var dst = 0;
  var src = 1;
  var got = _.entityOnly( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map, src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty' };
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty', 'nothing' : _.nothing };
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ true, 1, 'str', [ 1 ] ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ] ];
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var exp = undefined;
  var dst = new Constructor1();
  var src = false;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = 'src';
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = undefined;
  var dst = 'dst';
  var src = null;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = '';
  var dst = '';
  var src = true;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = false;
  var dst = false;
  var src = 1;
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityOnly( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map, src - map';
  var exp = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { '0' : null, 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 0, false, 1, 'str', [ 1 ] ];
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var exp = undefined;
  var dst = new Constructor1();
  var src = false;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = undefined;
  var dst = 'dst';
  var src = null;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = undefined;
  var dst = '';
  var src = true;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = undefined;
  var dst = false;
  var src = 1;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, src - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f2';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 1 }, 'b' : { 'f1' : false, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOnly( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1';
  var exp = [ { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f2';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : true }, { 'f1' : false, 'f2' : false }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOnly( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - str';
  var exp = undefined;
  var dst = new Constructor1();
  var src = 'src';
  var got = _.entityOnly( dst, src, '*/x' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - 0';
  var exp = undefined;
  var dst = 'dst';
  var src = 0;
  var got = _.entityOnly( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - 1';
  var exp = undefined;
  var dst = '';
  var src = 1;
  var got = _.entityOnly( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src = true';
  var exp = undefined;
  var dst = false;
  var src = true;
  var got = _.entityOnly( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - null';
  var exp = undefined;
  var dst = 0;
  var src = null;
  var got = _.entityOnly( dst, src, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityOnlyDiffTypes( test )
{
  test.shouldThrowErrorSync
  ( () =>
  {
    /* 'Dmytro' : need clarification. Array prototype has some properties, the access to this properties provides by  arr[ property ].
    the example below show this effect */

    test.case = 'dst - map, src - array';
    var exp = { 'map' : { 'a' : 0 } };
    var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ false, 0, 'null', true, 1, 'str', [ 1 ], { 'a' : 0 }, '' ];
    var got = _.entityOnly( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 0 ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { '1' : 1, 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var got = _.entityOnly( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = {};
    var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'maps' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ true, false, 1, 0, 'str', [], {} ];
    var got = _.entityOnly( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ false, 1 ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityOnly( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
    var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ 1, 2, 3, 4 ];
    var got = _.entityOnly( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ true, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityOnly( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with maps, */f1';
    var exp = { '1' : { 'f1' : false, 'f2' : 3 } };
    var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ { 'f1' : 3, 'f2' : 0 }, { 'f1' : 1, 'f2' : 3 }, { 'f1' : 0, 'f2' : 'str' } ];
    var got = _.entityOnly( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with map, */f2';
    var exp = { '1' : { 'f1' : false, 'f2' : 3 } };
    var dst = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, '2' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ 0, { 'f1' : 1, 'f2' : 1 }, 2, 3 ];
    var got = _.entityOnly( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f1';
    var exp = [ { 'f1' : 1, 'f2' : 0 } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityOnly( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f2';
    var exp = [ { 'f1' : false, 'f2' : 3 } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityOnly( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = new Constructor1();
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityOnly( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = 0;
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityOnly( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = undefined;
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityOnly( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = undefined;
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityOnly( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

}

//

function entityOnlyOnlySets( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var got = _.entityOnly( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityOnly( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var got = _.entityOnly( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOnly( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityOnly( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = dst;
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has same unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, false, 'some', 7 ] );
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has same unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str', _.nothing ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, false, 'some', 7, _.nothing ] );
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has different unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, false, 'str' ] );
  var dst = new Set( [ 0, 1, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, 'some', 7 ] );
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 3, 4 ], { 'b' : 3 }, 'src' ], 8 );
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 3, null, undefined, [ 0 ], '', { 'a' : 0 }, false, 'str' ] );
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 0 ], { 'a' : 0 }, 'str' ] );
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [ [ 2 ], { 'a' : 3 } ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', false, { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 } ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 0 ], { 'a' : 0 }, 'str' ] );
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [ [ 2 ], { 'a' : 3 } ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ { 'a' : 3 }, 0, 1, null, undefined, [ 2 ], '', false, 'str' ] );
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [ [ 2 ], { 'a' : 3 } ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both' );
}

//

function entityOnlyOnlyHashMaps( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap();
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap();
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityOnly( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap();
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap();
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityOnly( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap();
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap();
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = dst;
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 0, 0 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 0 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, true ], [ [ 2 ], [ 1 ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ null, null ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ]);
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 1 ], [ null, 'null' ], [ undefined, 'undefined' ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 2 ], [ [ 2 ], [ '2' ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'src' ] ] );
  var got = _.entityOnly( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'b', { 'f1' : false, 'f2' : 3 } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 0, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityOnly( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both' );
}

/* end of entityOnly* routines */

//

function entityButOnlyDst( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false, 0 ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, _.nothing';
  var exp = [ false, 0 ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = exp;
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityBut( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var got = _.entityBut( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var dst = new Constructor1();
  var got = _.entityBut( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got.x, 1 );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityBut( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var got = _.entityBut( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var got = _.entityBut( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var got = _.entityBut( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false, 0 ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var got = _.entityBut( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var dst = new Constructor1();
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got.x, 1 );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityBut( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityBut( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityBut( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : false, 'f2' : 3 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityBut( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : 1, 'f2' : 0 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityBut( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityBut( dst, undefined, '*/x' );
  test.true( dst !== got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = undefined;
  // var dst = 'dst';
  // var got = _.entityBut( dst, undefined, '*/length' );
  // test.true( dst !== got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var got = _.entityBut( dst, undefined, '*/length' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var got = _.entityBut( dst, undefined, '*/length' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var got = _.entityBut( dst, undefined, '*/length' );
  test.true( dst === got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityBut() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entityBut( [ 1, 2 ], [ 0, 1 ], ( e ) => e, 'extra' ) );

  test.case = 'wrong onEach type';
  test.shouldThrowErrorSync( () => _.entityBut( [ 1, 2 ], [ 0, 1 ], 'wrong' ) );
  test.shouldThrowErrorSync( () => _.entityBut( [ 1, 2 ], [ 0, 1 ], [] ) );

  test.case = 'onEach.length > 3';
  test.shouldThrowErrorSync( () => _.entityBut( [ 1, 2 ], [ 0, 1 ], ( a, b, c, d ) => a - b + c - d ) );
}

//

function entityButOnlySrc( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityBut( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ false, 0 ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityBut( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityBut( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = '';
  var src = '';
  var got = _.entityBut( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = false;
  var src = false;
  var got = _.entityBut( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = 0;
  var src = 0;
  var got = _.entityBut( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var src = new Constructor1();
  var got = _.entityBut( null, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got.x, 1 );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityBut( null, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = '';
  var src = '';
  var got = _.entityBut( null, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = false;
  var src = false;
  var got = _.entityBut( null, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = 0;
  var src = 0;
  var got = _.entityBut( null, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ false, 0 ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = '';
  var src = '';
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = false;
  var src = false;
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = 0;
  var src = 0;
  var got = _.entityBut( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ false ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var src = new Constructor1();
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got.x, 1 );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = '';
  var src = '';
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = false;
  var src = false;
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = 0;
  var src = 0;
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - map, */f1';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityBut( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f1, _.nothing';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 } };
  var src = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityBut( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f2';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityBut( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f1';
  var exp = [ { 'f1' : false, 'f2' : 3 } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityBut( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f2';
  var exp = [ { 'f1' : 1, 'f2' : 0 } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityBut( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityBut( null, src, '*/x' );
  test.true( src !== got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'src - str';
  // var exp = undefined;
  // var src = 'src';
  // var got = _.entityBut( null, src, '*/length' );
  // test.true( src !== got );
  // test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = '';
  var src = '';
  var got = _.entityBut( null, src, '*/length' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = false;
  var src = false;
  var got = _.entityBut( null, src, '*/length' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = 0;
  var src = 0;
  var got = _.entityBut( null, src, '*/length' );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityButBothSame( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = src;
  var got = _.entityBut( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = src;
  var got = _.entityBut( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false, 0 ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got.x, 1 );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false, 0 ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ false ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got.x, 1 );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityBut( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : false, 'f2' : 3 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : 1, 'f2' : 0 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityBut( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityBut( dst, src, '*/x' );
  test.true( src !== got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = undefined;
  // var dst = 'dst';
  // var src = dst;
  // var got = _.entityBut( dst, src, '*/length' );
  // test.true( src !== got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = '';
  var dst = '';
  var src = dst;
  var got = _.entityBut( dst, src, '*/length' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = false;
  var dst = false;
  var src = dst;
  var got = _.entityBut( dst, src, '*/length' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = 0;
  var dst = 0;
  var src = dst;
  var got = _.entityBut( dst, src, '*/length' );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityButBoth( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map, src - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ true, 1, { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ] ];
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, _.nothing';
  var exp = [ true, 1, { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], _.nothing, { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ], _.nothing ];
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityBut( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - number';
  var dst = new Constructor1();
  var src = 0;
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityBut( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - obj';
  var exp = undefined;
  var dst = 'dst';
  var src = new Constructor1();
  var got = _.entityBut( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = '';
  var dst = '';
  var src = null;
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - str';
  var exp = undefined;
  var dst = '';
  var src = 'src';
  var got = _.entityBut( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - undefined';
  var exp = false;
  var dst = false;
  var src = undefined;
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - false, src - number';
  var exp = undefined;
  var dst = false;
  var src = 2;
  var got = _.entityBut( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - false';
  var exp = 0;
  var dst = 0;
  var src = false;
  var got = _.entityBut( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - true';
  var exp = undefined;
  var dst = 0;
  var src = true;
  var got = _.entityBut( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map, src - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'dst::empty' };
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, 2, 3 ];
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src  = 'src';
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, dst - str';
  var exp = '';
  var dst = '';
  var src = 'src';
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - true';
  var exp = false;
  var dst = false;
  var src  = true;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - 1';
  var exp = 0;
  var dst = 0;
  var src = 1;
  var got = _.entityBut( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map, src - map';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty' };
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'false' : false, 'zero' : 0 };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty', 'nothing' : _.nothing };
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ false, 0, { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ] ];
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, nothing';
  var exp = [ false, 0, { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], _.nothing, { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], _.nothing ];
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var dst = new Constructor1();
  var src = false;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = 'dst';
  var dst = 'dst';
  var src = null;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = '';
  var dst = '';
  var src = null;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = undefined;
  var dst = '';
  var src = true;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = false;
  var dst = false;
  var src = 0;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = undefined;
  var dst = false;
  var src = 1;
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = 0;
  var dst = 0;
  var src = '';
  var got = _.entityBut( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map, src - map';
  var exp = { '' : 'str::empty' };
  var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { '0' : null, 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ false ];
  var dst = [ false, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 0, false, 1, 'str', [ 1 ] ];
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var dst = new Constructor1();
  var src = false;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = 'src';
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = 'dst';
  var dst = 'dst';
  var src = null;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = '';
  var dst = '';
  var src = null;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = '';
  var dst = '';
  var src = true;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = false;
  var dst = false;
  var src = 0;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = false;
  var dst = false;
  var src = 1;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = 0;
  var dst = 0;
  var src = '';
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, src - map, */f1';
  var exp = { 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f1, _.nothing';
  var exp = { 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 1 }, 'b' : { 'f1' : false, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityBut( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : true }, { 'f1' : false, 'f2' : false }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityBut( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - str';
  var dst = new Constructor1();
  var src = 'src';
  var got = _.entityBut( dst, src, '*/x' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - 0';
  var exp = 'dst';
  var dst = 'dst';
  var src = 0;
  var got = _.entityBut( dst, src, '*/length' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - 1';
  var exp = '';
  var dst = '';
  var src = 1;
  var got = _.entityBut( dst, src, '*/length' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src = true';
  var exp = false;
  var dst = false;
  var src = true;
  var got = _.entityBut( dst, src, '*/length' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - null';
  var exp = 0;
  var dst = 0;
  var src = null;
  var got = _.entityBut( dst, src, '*/length' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityButDiffTypes( test )
{
  test.shouldThrowErrorSync
  ( () =>
  {
    /* 'Dmytro' : need clarification. Array prototype has some properties, the access to this properties provides by  arr[ property ].
    the example below show this effect */

    test.case = 'dst - map, src - array';
    var exp = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], '' : 'str::empty' };
    var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ false, 0, 'null', true, 1, 'str', [ 1 ], { 'a' : 0 }, '' ];
    var got = _.entityBut( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ false, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { '1' : 1, 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var got = _.entityBut( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'maps' : { 'a' : 0 }, '' : 'str::empty' };
    var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'maps' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ true, false, 1, 0, 'str', [], {} ];
    var got = _.entityBut( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 0, true, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityBut( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = undefined;
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityBut( dst, src, ( e, k ) => e );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = undefined;
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityBut( dst, src, ( e, k ) => e );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { '' : 'str::empty' };
    var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ 1, 2, 3, 4 ];
    var got = _.entityBut( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ true ];
    var dst = [ true, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityBut( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = new Constructor1();
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityBut( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = 0;
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityBut( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with maps, */f1';
    var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
    var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ { 'f1' : 3, 'f2' : 0 }, { 'f1' : 1, 'f2' : 3 }, { 'f1' : 0, 'f2' : 'str' } ];
    var got = _.entityBut( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with map, */f2';
    var exp = { '0' : { 'f1' : 1, 'f2' : 0 }, '2' : { 'f1' : [], 'f2' : 'str' } };
    var dst = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, '2' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ 0, { 'f1' : 1, 'f2' : 1 }, 2, 3 ];
    var got = _.entityBut( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f1';
    var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityBut( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f2';
    var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityBut( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });
}

//

function entityButOnlySets( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 0, null, undefined, '', false ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var got = _.entityBut( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 0, null, undefined, '', false ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : false, 'f2' : 3 } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityBut( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 0, null, undefined, '', false ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 0, null, undefined, '', false ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value, _.nothing';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var got = _.entityBut( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : false, 'f2' : 3 } ] );
  var src = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityBut( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 0, null, undefined, '', false ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 0, null, undefined, '', false ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value, _.nothing';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : false, 'f2' : 3 } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = dst;
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has same unnecessary elements';
  var exp = new Set( [ 0, null, undefined, [ 2 ], '', { 'a' : 3 }, false ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, false, 'some', 7 ] );
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has different unnecessary elements';
  var exp = new Set( [ 0, undefined, [ 2 ], '', { 'a' : 3 }, false ] );
  var dst = new Set( [ 0, 1, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, 'some', 7 ] );
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 3, 4 ], { 'b' : 3 }, 'src', 8 ] );
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 2 ], { 'a' : 3 }, '', false, 'str' ] );
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ [ 2 ], { 'a' : 3 } ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 0 ], { 'a' : 0 }, 'str' ] );
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 2 ], { 'a' : 3 }, '', false, 'str' ] );
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both' );
}

//

function entityButOnlyHashMaps( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'b', { 'f1' : false, 'f2' : 3 } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityBut( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value, _.nothing';
  var exp = new HashMap( [ [ 0, 0 ], [ undefined, undefined ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, _.nothing ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'b', { 'f1' : false, 'f2' : 3 } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityBut( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'b', { 'f1' : false, 'f2' : 3 } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'b', { 'f1' : false, 'f2' : 3 } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : _.nothing, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = dst;
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 0 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ null, _.nothing ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, _.nothing ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 0 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, true ], [ [ 2 ], [ 1 ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 0, 0 ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 0, 0 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 1 ], [ null, 'null' ], [ undefined, 'undefined' ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 2 ], [ [ 2 ], [ '2' ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'src' ] ] );
  var got = _.entityBut( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 0, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityBut( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both' );
}

/* end of entityBut* routines */

//

function entityAndOnlyDst( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityAnd( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, _.nothing';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var got = _.entityAnd( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityAnd( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityAnd( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityAnd( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityAnd( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityAnd( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityAnd( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityAnd( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityAnd( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityAnd( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityAnd( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityAnd( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityAnd( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityAnd( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityAnd( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1, _.nothing';
  var exp = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityAnd( dst, undefined, '*/x' );
  test.true( dst === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = 'dst';
  // var dst = 'dst';
  // var got = _.entityAnd( dst, undefined, '*/length' );
  // test.true( dst === got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityAnd( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityAnd( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityAnd( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityAnd() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entityAnd( [ 1, 2 ], [ 0, 1 ], ( e ) => e, 'extra' ) );

  test.case = 'wrong onEach type';
  test.shouldThrowErrorSync( () => _.entityAnd( [ 1, 2 ], [ 0, 1 ], 'wrong' ) );
  test.shouldThrowErrorSync( () => _.entityAnd( [ 1, 2 ], [ 0, 1 ], [] ) );

  test.case = 'onEach.length > 3';
  test.shouldThrowErrorSync( () => _.entityAnd( [ 1, 2 ], [ 0, 1 ], ( a, b, c, d ) => a - b + c - d ) );
}

//

function entityAndOnlySrc( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityAnd( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityAnd( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityAnd( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityAnd( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityAnd( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityAnd( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityAnd( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityAnd( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityAnd( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityAnd( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityAnd( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityAnd( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityAnd( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityAnd( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityAnd( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityAnd( null, src, '*/x' );
  test.true( src === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'src - str';
  // var exp = 'src';
  // var src = 'src';
  // var got = _.entityAnd( null, src, '*/length' );
  // test.true( src === got );
  // test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityAnd( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityAnd( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityAnd( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityAndBothSame( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = src;
  var got = _.entityAnd( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = src;
  var got = _.entityAnd( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, _.nothing';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = exp;
  var got = _.entityAnd( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityAnd( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityAnd( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = dst;
  var got = _.entityAnd( dst, src, '*/x' );
  test.true( src === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = 'dst';
  // var dst = 'dst';
  // var src = dst;
  // var got = _.entityAnd( dst, src, '*/length' );
  // test.true( src === got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityAnd( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityAnd( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityAnd( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityAndBoth( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map, src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityAnd( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ 'str', [ 1 ] ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ] ];
  var got = _.entityAnd( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, _.nothing';
  var exp = [ 'str', [ 1 ], _.nothing ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], _.nothing, { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ], _.nothing ];
  var got = _.entityAnd( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityAnd( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - obj, src - number';
  var exp = undefined;
  var dst = new Constructor1();
  var src = 0;
  var got = _.entityAnd( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = 'src';
  var got = _.entityAnd( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - obj';
  var exp = 'dst';
  var dst = 'dst';
  var src = new Constructor1();
  var got = _.entityAnd( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityAnd( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - str';
  var exp = undefined;
  var dst = '';
  var src = 'src';
  var got = _.entityAnd( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - undefined';
  var exp = undefined;
  var dst = false;
  var src = undefined;
  var got = _.entityAnd( dst, src );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - number';
  var exp = undefined;
  var dst = false;
  var src = 2;
  var got = _.entityAnd( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - true';
  var exp = undefined;
  var dst = 0;
  var src = true;
  var got = _.entityAnd( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map, src - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'dst::empty' };
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, 2, 3 ];
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, dst - str';
  var exp = undefined;
  var dst = '';
  var src = 'src';
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - true';
  var exp = undefined;
  var dst = false;
  var src  = true;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - 1';
  var exp = undefined;
  var dst = 0;
  var src = 1;
  var got = _.entityAnd( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map, src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty' };
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty', 'nothing' : _.nothing };
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ true, 1, 'str', [ 1 ] ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ] ];
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var exp = undefined;
  var dst = new Constructor1();
  var src = false;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = 'src';
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = undefined;
  var dst = 'dst';
  var src = null;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - false';
  var exp = undefined;
  var dst = 0;
  var src = false;
  var got = _.entityAnd( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = undefined;
  var dst = '';
  var src = true;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = undefined;
  var dst = false;
  var src = 1;
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityAnd( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map, src - map';
  var exp = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { '0' : null, 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 0, false, 1, 'str', [ 1 ] ];
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var exp = undefined;
  var dst = new Constructor1();
  var src = false;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = undefined;
  var dst = 'dst';
  var src = null;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = undefined;
  var dst = '';
  var src = true;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = undefined;
  var dst = false;
  var src = 1;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, src - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f2';
  var exp = { 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 1 }, 'b' : { 'f1' : false, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityAnd( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1';
  var exp = [ { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1, _.nothing';
  var exp = [ { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f2';
  var exp = [ { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : true }, { 'f1' : false, 'f2' : false }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityAnd( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - str';
  var exp = undefined;
  var dst = new Constructor1();
  var src = 'src';
  var got = _.entityAnd( dst, src, '*/x' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - 0';
  var exp = undefined;
  var dst = 'dst';
  var src = 0;
  var got = _.entityAnd( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - 1';
  var exp = undefined;
  var dst = '';
  var src = 1;
  var got = _.entityAnd( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src = true';
  var exp = undefined;
  var dst = false;
  var src = true;
  var got = _.entityAnd( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - null';
  var exp = undefined;
  var dst = 0;
  var src = null;
  var got = _.entityAnd( dst, src, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityAndDiffTypes( test )
{
  test.shouldThrowErrorSync
  ( () =>
  {
    /* 'Dmytro' : need clarification. Array prototype has some properties, the access to this properties provides by  arr[ property ].
    the example below show this effect */

    test.case = 'dst - map, src - array';
    var exp = { 'map' : { 'a' : 0 } };
    var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ false, 0, 'null', true, 1, 'str', [ 1 ], { 'a' : 0 }, '' ];
    var got = _.entityAnd( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { '1' : 1, 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var got = _.entityAnd( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = {};
    var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'maps' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ true, false, 1, 0, 'str', [], {} ];
    var got = _.entityAnd( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 1 ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityAnd( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = new Constructor1();
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityAnd( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = undefined;
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityAnd( dst, src, ( e, k ) => e );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
    var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ 1, 2, 3, 4 ];
    var got = _.entityAnd( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ true, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityAnd( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = undefined;
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityAnd( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = undefined;
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityAnd( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with maps, */f1';
    var exp = {};
    var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ { 'f1' : 3, 'f2' : 0 }, { 'f1' : 1, 'f2' : 3 }, { 'f1' : 0, 'f2' : 'str' } ];
    var got = _.entityAnd( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with map, */f2';
    var exp = { '1' : { 'f1' : false, 'f2' : 3 } };
    var dst = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, '2' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ 0, { 'f1' : 1, 'f2' : 1 }, 2, 3 ];
    var got = _.entityAnd( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f1';
    var exp = [ { 'f1' : 1, 'f2' : 0 } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityAnd( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f2';
    var exp = [ { 'f1' : false, 'f2' : 3 } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityAnd( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });
}

//

function entityAndOnlySets( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var got = _.entityAnd( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityAnd( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str', _.nothing ] );
  var got = _.entityAnd( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityAnd( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined, _.nothing';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str', _.nothing ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = dst;
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has same unnecessary elements';
  var exp = new Set( [ 1, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, false, 'some', 7 ] );
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has different unnecessary elements';
  var exp = new Set( [1, 'str' ] );
  var dst = new Set( [ 0, 1, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, 'some', 7 ] );
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 3, 4 ], { 'b' : 3 }, 'src', 8 ] );
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 2 ], { 'a' : 3 }, '', false, 'str' ] );
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 0 ], { 'a' : 0 }, 'str' ] );
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 2 ], { 'a' : 3 }, '', false, 'str' ] );
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [] );
  var dst = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector, _.nothing';
  var exp = new Set( [] );
  var dst = new Set( [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both' );
}

//

function entityAndOnlyHashMaps( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, _.nothing';
  var exp = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, _.nothing ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityAnd( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, _.nothing ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, _.nothing ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityAnd( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value, _.nothing';
  var exp = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = dst;
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 0 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, true ], [ [ 2 ], [ 1 ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 1 ], [ null, 'null' ], [ undefined, 'undefined' ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 2 ], [ [ 2 ], [ '2' ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'src' ] ] );
  var got = _.entityAnd( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector, _.nothing';
  var exp = new HashMap( [ [ _.nothing, { 'f1' : 1, 'f2' : 0 } ] ] );
  var dst = new HashMap( [ [ _.nothing, { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ _.nothing, { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityAnd( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both' );
}

/* end of entityAnd* routines */

//

function entityOrOnlyDst( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOr( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, _.nothing';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var got = _.entityOr( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityOr( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityOr( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOr( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOr( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOr( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityOr( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityOr( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOr( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOr( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOr( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOr( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOr( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOr( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOr( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1, _.nothing';
  var exp = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var got = _.entityOr( dst, undefined, '*/x' );
  test.true( dst === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = 'dst';
  // var dst = 'dst';
  // var got = _.entityOr( dst, undefined, '*/length' );
  // test.true( dst === got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityOr( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityOr( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityOr( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityOr() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entityOr( [ 1, 2 ], [ 0, 1 ], ( e ) => e, 'extra' ) );

  test.case = 'wrong onEach type';
  test.shouldThrowErrorSync( () => _.entityOr( [ 1, 2 ], [ 0, 1 ], 'wrong' ) );
  test.shouldThrowErrorSync( () => _.entityOr( [ 1, 2 ], [ 0, 1 ], [] ) );

  test.case = 'onEach.length > 3';
  test.shouldThrowErrorSync( () => _.entityOr( [ 1, 2 ], [ 0, 1 ], ( a, b, c, d ) => a - b + c - d ) );
}

//

function entityOrOnlySrc( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOr( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityOr( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityOr( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOr( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOr( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOr( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityOr( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityOr( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOr( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOr( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOr( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOr( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOr( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOr( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOr( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityOr( null, src, '*/x' );
  test.true( src === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'src - str';
  // var exp = 'src';
  // var src = 'src';
  // var got = _.entityOr( null, src, '*/length' );
  // test.true( src === got );
  // test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityOr( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityOr( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityOr( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityOrBothSame( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = src;
  var got = _.entityOr( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = src;
  var got = _.entityOr( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = exp;
  var got = _.entityOr( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - _.nothing';
  var exp = _.nothing;
  var dst = _.nothing;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityOr( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityOr( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = new Constructor1();
  var dst = exp;
  var src = dst;
  var got = _.entityOr( dst, src, '*/x' );
  test.true( src === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str';
  // var exp = 'dst';
  // var dst = 'dst';
  // var src = dst;
  // var got = _.entityOr( dst, src, '*/length' );
  // test.true( src === got );
  // test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityOr( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityOr( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityOr( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityOrBoth( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map, src - map';
  var exp = { 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ true, 1, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ] ];
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, _.nothing';
  var exp = [ true, 1, true, 1, 'str', [ 1 ], _.nothing, { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], _.nothing, { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ], _.nothing ];
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - obj, src - number';
  var dst = new Constructor1();
  var src = 0;
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = 'src';
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - obj';
  var exp = 'dst';
  var dst = 'dst';
  var src = new Constructor1();
  var got = _.entityOr( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityOr( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - str';
  var exp = 'src';
  var dst = '';
  var src = 'src';
  var got = _.entityOr( dst, src );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false, src - undefined';
  var exp = undefined;
  var dst = false;
  var src = undefined;
  var got = _.entityOr( dst, src );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - _.nothing';
  var exp = _.nothing;
  var dst = false;
  var src = _.nothing;
  var got = _.entityOr( dst, src );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - number';
  var exp = 2;
  var dst = false;
  var src = 2;
  var got = _.entityOr( dst, src );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - false';
  var exp = undefined;
  var dst = 0;
  var src = false;
  var got = _.entityOr( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - true';
  var exp = true;
  var dst = 0;
  var src = true;
  var got = _.entityOr( dst, src );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map, src - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'dst::empty' };
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, 2, 3 ];
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src  = 'src';
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, dst - str';
  var exp = undefined;
  var dst = '';
  var src = 'src';
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - true';
  var exp = undefined;
  var dst = false;
  var src  = true;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - 1';
  var exp = undefined;
  var dst = 0;
  var src = 1;
  var got = _.entityOr( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map, src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty' };
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty', 'nothing' : _.nothing };
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ] ];
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var dst = new Constructor1();
  var src = false;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = 'dst';
  var dst = 'dst';
  var src = 'src';
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = 'dst';
  var dst = 'dst';
  var src = null;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = true;
  var dst = '';
  var src = true;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = 1;
  var dst = false;
  var src = 1;
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityOr( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map, src - map';
  var exp = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { '0' : null, 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var dst = [ false, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 0, false, 1, 'str', [ 1 ] ];
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var exp = undefined;
  var dst = new Constructor1();
  var src = false;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = undefined;
  var dst = 'dst';
  var src = null;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = undefined;
  var dst = '';
  var src = true;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = undefined;
  var dst = false;
  var src = 1;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, src - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f2';
  var exp = {  'a' : { 'f1' : 1, 'f2' : 1 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 1 }, 'b' : { 'f1' : false, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityOr( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1, _.nothing';
  var exp = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f2';
  var exp = [ { 'f1' : 1, 'f2' : true }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : true }, { 'f1' : false, 'f2' : false }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityOr( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - str';
  var dst = new Constructor1();
  var src = 'src';
  var got = _.entityOr( dst, src, '*/x' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  // 'Dmytro' : delimeter '*/length' check length of string as property

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str, src - 0';
  // var exp = 'dst';
  // var dst = 'dst';
  // var src = 0;
  // var got = _.entityOr( dst, src, '*/length' );
  // test.true( dst === got );
  // test.true( src !== got );
  // test.identical( got, exp );

  test.case = 'dst - empty str, src - 1';
  var exp = undefined;
  var dst = '';
  var src = 1;
  var got = _.entityOr( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src = true';
  var exp = undefined;
  var dst = false;
  var src = true;
  var got = _.entityOr( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - null';
  var exp = undefined;
  var dst = 0;
  var src = null;
  var got = _.entityOr( dst, src, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityOrDiffTypes( test )
{
  test.shouldThrowErrorSync
  ( () =>
  {
    /* 'Dmytro' : need clarification. Array prototype has some properties, the access to this properties provides by  arr[ property ].
    the example below show this effect */

    test.case = 'dst - map, src - array';
    var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ false, 0, 'null', true, 1, 'str', [ 1 ], { 'a' : 0 }, '' ];
    var got = _.entityOr( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 1, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { '1' : 1, 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var got = _.entityOr( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = {};
    var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'maps' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ true, false, 1, 0, 'str', [], {} ];
    var got = _.entityOr( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 1 ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityOr( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityOr( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got.x, 1 );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = [ 1 ];
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityOr( dst, src, ( e, k ) => e );
    test.true( dst !== got );
    test.true( src === got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
    var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ 1, 2, 3, 4 ];
    var got = _.entityOr( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ true, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityOr( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityOr( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got.x, 1 );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = undefined;
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityOr( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with maps, */f1';
    var exp = {};
    var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ { 'f1' : 3, 'f2' : 0 }, { 'f1' : 1, 'f2' : 3 }, { 'f1' : 0, 'f2' : 'str' } ];
    var got = _.entityOr( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with map, */f2';
    var exp = { '1' : { 'f1' : false, 'f2' : 3 } };
    var dst = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, '2' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ 0, { 'f1' : 1, 'f2' : 1 }, 2, 3 ];
    var got = _.entityOr( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f1';
    var exp = [ { 'f1' : 1, 'f2' : 0 } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityOr( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f2';
    var exp = [ { 'f1' : false, 'f2' : 3 } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityOr( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });
}

//

function entityOrOnlySets( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str', _.nothing ] );
  var got = _.entityOr( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityOr( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var got = _.entityOr( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityOr( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = dst;
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has same unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, false, 'some', 7 ] );
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has different unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, 'some', 7 ] );
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 3, 4 ], { 'b' : 3 }, 'src', 8 ] );
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 0 ], { 'a' : 3 }, '', false, 'str' ] );
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 0 ], { 'a' : 0 }, 'src' ] );
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 0 ], { 'a' : 0 }, '', false, 'str' ] );
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 0 ], { 'a' : 0 }, 'str' ] );
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector, _.nothing';
  var exp = new Set( [ { 'f1' : _.nothing, 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : _.nothing, 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : _.nothing, 'f2' : 'str' } ] );
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both' );
}

//

function entityOrOnlyHashMaps( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, _.nothing';
  var exp = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, _.nothing ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityOr( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityOr( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value, _.nothing';
  var exp = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = dst;
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 0, 1 ], [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 0 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, true ], [ [ 2 ], [ 1 ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ null, 'null' ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 1 ], [ null, 'null' ], [ undefined, 'undefined' ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 2 ], [ [ 2 ], [ '2' ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'src' ] ] );
  var got = _.entityOr( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector, _.nothing';
  var exp = new HashMap( [ [ 'a', { 'f1' : _.nothing, 'f2' : 0 } ], [ 'b', { 'f1' : _.nothing, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : _.nothing, 'f2' : 0 } ], [ 'b', { 'f1' : _.nothing, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityOr( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both' );
}

/* end of entityOr* routines */

//

function entityXorOnlyDst( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityXor( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, _.nothing';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var got = _.entityXor( dst );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityXor( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityXor( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityXor( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityXor( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityXor( dst );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( dst, undefined, ( e, k ) => undefined );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityXor( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityXor( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityXor( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityXor( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityXor( dst, undefined, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityXor( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( dst, undefined, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityXor( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityXor( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityXor( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityXor( dst, undefined, ( e, k ) => e );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = {};
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityXor( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = {};
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityXor( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = {};
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityXor( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1, _.nothing';
  var exp = [];
  var dst = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( dst, undefined, '*/f1' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( dst, undefined, '*/f2' );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var got = _.entityXor( dst, undefined, '*/x' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var got = _.entityXor( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var got = _.entityXor( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var got = _.entityXor( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var got = _.entityXor( dst, undefined, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  test.case = 'dst - map, onEach uses external variable';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty'};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var count = 1;
  var onEach = function( e )
  {
    if( count === 1 )
    count--;
    else
    count++;
    return count;
  }
  var got = _.entityXor( dst, undefined, onEach );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str, onEach uses external variable';
  var exp = '';
  var dst = '';
  var count = 1;
  var onEach = function( e )
  {
    if( count === 1 )
    count--;
    else
    count++;
    return count;
  }
  var got = _.entityXor( dst, undefined, onEach );
  test.true( dst === got );
  test.identical( got, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityXor() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.entityXor( [ 1, 2 ], [ 0, 1 ], ( e ) => e, 'extra' ) );

  test.case = 'wrong onEach type';
  test.shouldThrowErrorSync( () => _.entityXor( [ 1, 2 ], [ 0, 1 ], 'wrong' ) );
  test.shouldThrowErrorSync( () => _.entityXor( [ 1, 2 ], [ 0, 1 ], [] ) );

  test.case = 'onEach.length > 3';
  test.shouldThrowErrorSync( () => _.entityXor( [ 1, 2 ], [ 0, 1 ], ( a, b, c, d ) => a - b + c - d ) );
}

//

function entityXorOnlySrc( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { 'false' : true, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : true, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'false' : true, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : true, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityXor( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( null, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityXor( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityXor( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = '';
  var src = '';
  var got = _.entityXor( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = false;
  var src = false;
  var got = _.entityXor( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = 0;
  var src = 0;
  var got = _.entityXor( null, src );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityXor( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityXor( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityXor( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityXor( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityXor( null, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, _.nothing';
  var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = 'src';
  var src = 'src';
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityXor( null, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = [ 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = undefined;
  var src = new Constructor1();
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = undefined;
  var src = 'src';
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - map, */f1';
  var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityXor( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f1, _.nothing';
  var exp = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityXor( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map, */f2';
  var exp = { 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityXor( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( null, src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array, */f2';
  var exp = [ { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( null, src, '*/f2' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - obj';
  var exp = new Constructor1();
  var src = exp;
  var got = _.entityXor( null, src, '*/x' );
  test.true( src === got );
  test.identical( got, exp );

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'src - str';
  // var exp = 'src';
  // var src = 'src';
  // var got = _.entityXor( null, src, '*/length' );
  // test.true( src === got );
  // test.identical( got, exp );

  test.case = 'src - empty str';
  var exp = undefined;
  var src = '';
  var got = _.entityXor( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - false';
  var exp = undefined;
  var src = false;
  var got = _.entityXor( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - 0';
  var exp = undefined;
  var src = 0;
  var got = _.entityXor( null, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityXorBothSame( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map';
  var exp = {};
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var dst = src;
  var got = _.entityXor( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = {};
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var dst = src;
  var got = _.entityXor( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - _.nothing';
  var exp = undefined;
  var dst = _.nothing;
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, _.nothing';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, */f1';
  var exp = {};
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f1, _.nothing';
  var exp = {};
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - map, */f2';
  var exp = {};
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = dst;
  var got = _.entityXor( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f1';
  var exp = [];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - array, */f2';
  var exp = [];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = dst;
  var got = _.entityXor( dst, src, '*/f2' );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = dst;
  var got = _.entityXor( dst, src, '*/x' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str';
  var exp = undefined;
  var dst = 'dst';
  var src = dst;
  var got = _.entityXor( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str';
  var exp = undefined;
  var dst = '';
  var src = dst;
  var got = _.entityXor( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false';
  var exp = undefined;
  var dst = false;
  var src = dst;
  var got = _.entityXor( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0';
  var exp = undefined;
  var dst = 0;
  var src = dst;
  var got = _.entityXor( dst, src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityXorBoth( test )
{

  function Constructor1()
  {
    this.x = 1;
    return this;
  }

  /* - */

  test.open( 'no onEach' );

  test.case = 'dst - map, src - map';
  var exp = { 'null' : 'null' };
  var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = { 'false' : _.nothing, 'null' : 'null' };
  var dst = { 'false' : _.nothing, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ true, 1, true, 1, { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 1, false, 0, 'str', [ 1 ] ];
  var got = _.entityXor( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ true, 1, true, 1, { 'a' : 0 }, _.nothing ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, _.nothing ];
  var src = [ true, 1, false, 0, 'str', [ 1 ] ];
  var got = _.entityXor( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - number';
  var dst = new Constructor1();
  var src = 0;
  var got = _.entityXor( dst, src );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - obj';
  var exp = undefined;
  var dst = 'dst';
  var src = new Constructor1();
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - str';
  var exp = 'src';
  var dst = '';
  var src = 'src';
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false, src - undefined';
  var exp = undefined;
  var dst = false;
  var src = undefined;
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - number';
  var exp = 2;
  var dst = false;
  var src = 2;
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - false';
  var exp = undefined;
  var dst = 0;
  var src = false;
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - true';
  var exp = true;
  var dst = 0;
  var src = true;
  var got = _.entityXor( dst, src );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* - */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'dst - map, src - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'dst::empty' };
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 }, 2, 3 ];
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - obj';
  var exp = undefined;
  var dst = new Constructor1();
  var src = new Constructor1();
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src  = 'src';
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, dst - str';
  var exp = undefined;
  var dst = '';
  var src = 'src';
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - true';
  var exp = undefined;
  var dst = false;
  var src  = true;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - 1';
  var exp = undefined;
  var dst = 0;
  var src = 1;
  var got = _.entityXor( dst, src, ( e, k ) => undefined );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* - */

  test.open( 'onEach - routine returning element' );

  test.case = 'dst - map, src - map';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty' };
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, _.nothing';
  var exp = {};
  var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty', 'nothing' : _.nothing };
  var src = { 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'src::empty', 'nothing' : _.nothing };
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ { 'a' : 0 } ];
  var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ false, 0, true, 1, 'str', [ 1 ] ];
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var dst = new Constructor1();
  var src = false;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = 'dst';
  var dst = 'dst';
  var src = null;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst === got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = true;
  var dst = '';
  var src = true;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = 1;
  var dst = false;
  var src = 1;
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src === got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityXor( dst, src, ( e, k ) => e );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* - */

  test.open( 'onEach - routine returning key' );

  test.case = 'dst - map, src - map';
  var exp = {};
  var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var src = { '0' : null, 'false' : 0, 'zero' : false, 'true' : 1, 'one' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array';
  var exp = [ { 'a' : 0 } ];
  var dst = [ false, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
  var src = [ true, 0, false, 1, 'str', [ 1 ] ];
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - false';
  var exp = undefined;
  var dst = new Constructor1();
  var src = false;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - str';
  var exp = undefined;
  var dst = 'dst';
  var src = 'src';
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - str, src - null';
  var exp = undefined;
  var dst = 'dst';
  var src = null;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - null';
  var exp = undefined;
  var dst = '';
  var src = null;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - empty str, src - true';
  var exp = undefined;
  var dst = '';
  var src = true;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 0';
  var exp = undefined;
  var dst = false;
  var src = 0;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src - 1';
  var exp = undefined;
  var dst = false;
  var src = 1;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src = empty string';
  var exp = undefined;
  var dst = 0;
  var src = '';
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'dst - map, src - map, */f1';
  var exp = { 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : 1, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f1, _.nothing';
  var exp = { 'b' : { 'f1' : _.nothing, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var dst = { 'a' : { 'f1' : _.nothing, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 3, 'f2' : 0 }, 'b' : { 'f1' : _.nothing, 'f2' : 3 }, 'c' : { 'f1' : 0, 'f2' : 'str' } };
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - map, src - map, */f2';
  var exp = {  'a' : { 'f1' : 1, 'f2' : 1 }, 'b' : { 'f1' : false, 'f2' : 3 } };
  var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, 'b' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var src = { 'a' : { 'f1' : 1, 'f2' : 1 }, 'b' : { 'f1' : false, 'f2' : 0 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
  var got = _.entityXor( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1';
  var exp = [ { 'f1' : 1, 'f2' : 0 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f1, _.nothing';
  var exp = [ { 'f1' : _.nothing, 'f2' : 0 } ];
  var dst = [ { 'f1' : _.nothing, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 0, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - array, src - array, */f2';
  var exp = [ { 'f1' : 1, 'f2' : true }, { 'f1' : false, 'f2' : 3 } ];
  var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
  var src = [ { 'f1' : 1, 'f2' : true }, { 'f1' : false, 'f2' : false }, { 'f1' : [], 'f2' : 'str' } ];
  var got = _.entityXor( dst, src, '*/f2' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - _.nothing, src - str';
  var exp = undefined;
  var dst = _.nothing;
  var src = 'src';
  var got = _.entityXor( dst, src, '*/x' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - obj, src - str';
  var dst = new Constructor1();
  var src = 'src';
  var got = _.entityXor( dst, src, '*/x' );
  test.true( dst === got );
  test.true( src !== got );
  test.identical( got.x, 1 );

  // 'Dmytro' : delimeter '*/length' check length of string as property

  /* xxx qqq : probably after changes in wSelector or connected modules */
  // test.case = 'dst - str, src - 0';
  // var exp = 'dst';
  // var dst = 'dst';
  // var src = 0;
  // var got = _.entityXor( dst, src, '*/length' );
  // test.true( dst === got );
  // test.true( src !== got );
  // test.identical( got, exp );

  test.case = 'dst - empty str, src - 1';
  var exp = undefined;
  var dst = '';
  var src = 1;
  var got = _.entityXor( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - false, src = true';
  var exp = undefined;
  var dst = false;
  var src = true;
  var got = _.entityXor( dst, src, '*/length' );
  test.true( dst !== got );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'dst - 0, src - null';
  var exp = undefined;
  var dst = 0;
  var src = null;
  var got = _.entityXor( dst, src, '*/length' );
  test.true( dst !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function entityXorDiffTypes( test )
{
  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], '' : 'str::empty' };
    var dst = { 'false' : false, 'zero' : 0, 'null' : null, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ false, 0, 'null', true, 1, 'str', [ 1 ], { 'a' : 0 }, '' ];
    var got = _.entityXor( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 1, 1, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { '1' : 1, 'false' : false, 'zero' : 0, 'null' : 'null', 'true' : true, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var got = _.entityXor( dst, src );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], '' : 'str::empty' };
    var dst = { 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'maps' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ true, false, 1, 0, 'str', [], {} ];
    var got = _.entityXor( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 'str', true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ false, 0, true, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityXor( dst, src, ( e, k ) => e );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = undefined;
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityXor( dst, src, ( e, k ) => e );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = [ 1 ];
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityXor( dst, src, ( e, k ) => e );
    test.true( dst !== got );
    test.true( src === got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array';
    var exp = { 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 } };
    var dst = { '0' : 'zero', 'false' : false, 'zero' : 0, 'true' : true, 'one' : 1, 'str' : 'str', 'arr' : [ 1 ], 'map' : { 'a' : 0 }, '' : 'str::empty' };
    var src = [ 1, 2, 3, 4 ];
    var got = _.entityXor( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map';
    var exp = [ 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var dst = [ true, 0, false, 1, 'str', [ 1 ], { 'a' : 0 } ];
    var src = { 'a' : 2, 'b' : 3, '0' : 'str', '2' : false, '3' : true };
    var got = _.entityXor( dst, src, ( e, k ) => k );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - obj, src - map';
    var exp = undefined;
    var dst = new Constructor1();
    var src = { 'a' : 0 };
    var got = _.entityXor( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - 0, src = array';
    var exp = [ 1 ];
    var dst = 0;
    var src = [ 1 ];
    var got = _.entityXor( dst, src, ( e, k ) => k );
    test.true( dst !== got );
    test.true( src === got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with maps, */f1';
    var exp = { 'a' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
    var dst = { 'a' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'c' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ { 'f1' : 3, 'f2' : 0 }, { 'f1' : 1, 'f2' : 3 }, { 'f1' : 0, 'f2' : 'str' } ];
    var got = _.entityXor( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - map, src - array with map, */f2';
    var exp = {};
    var dst = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, '2' : { 'f1' : [], 'f2' : 'str' } };
    var src = [ 0, { 'f1' : 1, 'f2' : 1 }, 2, 3 ];
    var got = _.entityXor( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f1';
    var exp = [ { 'f1' : [], 'f2' : 'str' } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityXor( dst, src, '*/f1' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });

  test.shouldThrowErrorSync
  ( () =>
  {
    test.case = 'dst - array, src - map, */f2';
    var exp = [ { 'f1' : [], 'f2' : 'str' } ];
    var dst = [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ];
    var src = { '0' : { 'f1' : 1, 'f2' : 0 }, '1' : { 'f1' : false, 'f2' : 3 }, 'a' : { 'f1' : [], 'f2' : 'str' } };
    var got = _.entityXor( dst, src, '*/f2' );
    test.true( dst === got );
    test.true( src !== got );
    test.identical( got, exp );
  });
}

//

function entityXorOnlySets( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, _.nothing';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str', _.nothing ] );
  var got = _.entityXor( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityXor( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var got = _.entityXor( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var got = _.entityXor( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value, _.nothing';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', _.nothing ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [] );
  var dst = new Set( [ { 'f1' : 1, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : [], 'f2' : 'str' } ] );
  var src = dst;
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has same unnecessary elements';
  var exp = new Set( [ [ 2 ], { 'a' : 3 }, [ 2 ], { 'a' : 3 }, 'some', 7 ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, false, 'some', 7 ] );
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has different unnecessary elements';
  var exp = new Set( [ [ 2 ], { 'a' : 3 }, [ 2 ], { 'a' : 3 }, 'some', 7 ] );
  var dst = new Set( [ 0, 1, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 1, 0, [ 2 ], null, 'str', undefined, '', { 'a' : 3 }, 'some', 7 ] );
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', 2, [ 3, 4 ], { 'b' : 3 }, 'src', 8 ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 3, 4 ], { 'b' : 3 }, 'src', 8 ] );
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 4, [ 0 ], { 'a' : 3 } ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 0 ], { 'a' : 3 }, '', false, 'str' ] );
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str', 2, [ 0 ], { 'a' : 0 }, 'src' ] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 2, [ 0 ], { 'a' : 0 }, 'src' ] );
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new Set( [ 4, [ 0 ], { 'a' : 0 }, 'str' ] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 4, null, undefined, [ 0 ], { 'a' : 0 }, '', false, 'str' ] );
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new Set( [ 1, [ 0 ], { 'a' : 0 }, 'str'] );
  var dst = new Set( [ 0, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 0 ], { 'a' : 0 }, 'str' ] );
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var src = new Set( [ 0, 1, null, undefined, [ 2 ], '', { 'a' : 3 }, false, 'str' ] );
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new Set( [] );
  var dst = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var src = new Set( [ 1, [ 2 ], { 'a' : 3 }, 'str' ] );
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector';
  var exp = new Set( [ { 'f1' : 1, 'f2' : 'str' }, { 'f1' : 1, 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.case = 'onEach is selector, _.nothing';
  var exp = new Set( [ { 'f1' : _.nothing, 'f2' : 'str' }, { 'f1' : 1, 'f2' : 'str' } ] );
  var dst = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : _.nothing, 'f2' : 'str' } ] );
  var src = new Set( [ { 'f1' : undefined, 'f2' : 0 }, { 'f1' : false, 'f2' : 3 }, { 'f1' : 1, 'f2' : 'str' } ] );
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got ], [ ... exp ] );

  test.close( 'both' );
}

//

function entityXorOnlyHashMaps( test )
{
  test.open( 'only dst' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, _.nothing';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, _.nothing ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, undefined, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityXor( dst, undefined, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only dst' );

  /* - */

  test.open( 'only src' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, _.nothing';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], , _.nothing ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], , _.nothing ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src, ( e ) => e );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src, ( e, k ) => k );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var src = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( null, src, ( e ) => undefined );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var got = _.entityXor( null, src, '*/f1' );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'only src' );

  /* - */

  test.open( 'both same' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value, _.nothing';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, _.nothing ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = dst;
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( got === dst );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both same' );

  /* - */

  test.open( 'both' );

  test.case = 'has unnecessary elements';
  var exp = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 0, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 0 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ [ 2 ], [ 1 ] ], [ { 'a' : 3 }, { 'a' : 0 } ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, true ], [ [ 2 ], [ 1 ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ null, 'null' ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns value';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, src, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ] ] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, 'null' ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns key';
  var exp = new HashMap( [ [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ] ] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 4 ], [ [ 2 ], [ 1 ] ], [ { 'a' : 0 }, { 'a' : 0 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, src, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'has unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 0, 0 ], [ 1, 1 ], [ null, null ], [ undefined, undefined ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 0, 1 ], [ 1, 1 ], [ null, 'null' ], [ undefined, 'undefined' ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'without unnecessary elements, onEach returns undefined';
  var exp = new HashMap( [] );
  var dst = new HashMap( [ [ 1, 1 ], [ [ 2 ], [ 2 ] ], [ { 'a' : 3 }, { 'a' : 3 } ], [ 'str', 'str' ] ] );
  var src = new HashMap( [ [ 1, 2 ], [ [ 2 ], [ '2' ] ], [ { 'a' : 3 }, { 'a' : 0 } ], [ 'str', 'src' ] ] );
  var got = _.entityXor( dst, src, ( e ) => undefined );
  test.true( got === dst );
  test.true( got !== src );
  test.equivalent( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector';
  var exp = new HashMap( [ [ 'c', { 'f1' : [], 'f2' : 'str' } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : [], 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'onEach is selector, _.nothing';
  var exp = new HashMap( [ [ 'c', { 'f1' : _.nothing, 'f2' : 'str' } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ] ] );
  var dst = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : false, 'f2' : 3 } ], [ 'c', { 'f1' : _.nothing, 'f2' : 'str' } ] ] );
  var src = new HashMap( [ [ 'a', { 'f1' : 1, 'f2' : 0 } ], [ 'b', { 'f1' : 1, 'f2' : 3 } ], [ 'c', { 'f1' : undefined, 'f2' : 'str' } ] ] );
  var got = _.entityXor( dst, src, '*/f1' );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.close( 'both' );
}

/* end of entityXor* routines */

//

function entityAll( test )
{
  test.open( 'onEach is routine' );

  test.case = 'Set';

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAll( src, ( v, i, c ) => v && c.size > 3 );
  test.identical( got, true );

  var src = new Set( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, false );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAll( src, () => undefined );
  test.identical( got, undefined );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4, _.nothing ] );
  var got = _.entityAll( src, ( v ) => v );
  test.identical( got, true );

  /* */

  test.case = 'Map';

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityAll( src, ( v, k ) => v === k );
  test.identical( got, false );

  var src = new HashMap( [ [ 'a', 'a' ], [ '4', '4' ], [ 'true', 'true' ] ] );
  var got = _.entityAll( src, ( v, k ) => v === k );
  test.identical( got, true );

  var src = new HashMap( [ [ 1, 2 ], [ 2, 3 ], [ 'a', null ] ] );
  var got = _.entityAll( src, ( v, k ) => v !== k );
  test.identical( got, true );

  var src = new HashMap( [ [ 'a', 'a' ], [ 'c', 'str' ], [ 'd', 'd' ] ] );
  var got = _.entityAll( src, ( v, k ) => typeof v === typeof k );
  test.identical( got, true );

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityAll( src, ( v, k, src ) => src.size !== v );
  test.identical( got, true );

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', _.nothing ] ] );
  var got = _.entityAll( src, ( v, k, src ) => v );
  test.identical( got, true );

  /* */

  test.case = 'array';

  var got = _.entityAll( [ 1, 'str', undefined ], ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var got = _.entityAll( [ 1, 'str', { 'a' : 2 }, 4 ], ( v, i ) => !!v && i + 2 < 6 );
  test.identical( got, true );

  var got = _.entityAll( [ 1, 'str', { 'a' : 2 }, false ], ( v, i ) => !!v && i + 2 < 6 );
  test.identical( got, false );

  var got = _.entityAll( [ 1, 'str', 3, null ], () => undefined );
  test.identical( got, undefined );

  var got = _.entityAll( [ 1, 'str', 3, _.nothing ], ( v ) => v );
  test.identical( got, true );

  /* */

  test.case = 'unroll';

  var src = _.unroll.from( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), undefined, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, false );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAll( src, () => undefined );
  test.identical( got, undefined );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4, _.nothing ] );
  var got = _.entityAll( src, ( v ) => v );
  test.identical( got, true );

  /* */

  test.case = 'argument array';

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, false );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAll( src, () => undefined );
  test.identical( got, undefined );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4, _.nothing ] );
  var got = _.entityAll( src, ( v ) => v );
  test.identical( got, true );

  /* */

  test.case = 'F32x';

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = new F32x( [ 1, 2, [ 8 ], false, 4 ] );
  var got = _.entityAll( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, false );

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityAll( src, () => undefined );
  test.identical( got, undefined );

  /* */

  test.case = 'Aux';

  var got = _.entityAll( { '1' : 2, 'c' : 4, 'a' : undefined }, ( v, k ) => v === k );
  test.identical( got, false );

  var got = _.entityAll( { 'a' : 'a', '4' : '4', 'true' : 'true' }, ( v, k ) => v === k );
  test.identical( got, true );

  var got = _.entityAll( { '1' : 2, '2' : 3, 'a' : null }, ( v, k ) => v !== k );
  test.identical( got, true );

  var got = _.entityAll( { 'a' : 1, 'b' : 3, 'c' : true }, ( v, k ) => v !== k );
  test.identical( got, true );

  var got = _.entityAll( { 'a' : 'a', 'b' : 'str' }, ( v, k ) => typeof v === typeof k );
  test.identical( got, true );

  var got = _.entityAll( { 'a' : 1, 'b' : false }, ( v, k ) => v === k );
  test.identical( got, false );

  var got = _.entityAll( { 'a' : 1, 'b' : false }, ( v, k, src ) => src.length !== v );
  test.identical( got, true );

  var got = _.entityAll( { 'a' : 1, 'b' : _.nothing }, ( v, k, src ) => v );
  test.identical( got, true );

  /* */

  test.case = 'no ArrayLike, no Aux'

  var got = _.entityAll( undefined, ( src, u ) => src !== u );
  test.identical( got, false );

  var got = _.entityAll( null, ( src, u ) => src === u );
  test.identical( got, false );

  var got = _.entityAll( 1, ( src, u ) => src === u );
  test.identical( got, false );

  var got = _.entityAll( 'str', ( src, u ) => src === u );
  test.identical( got, false );

  var got = _.entityAll( false, ( src, u ) => src === u );
  test.identical( got, false );

  var got = _.entityAll( true, ( src, u ) => src !== u );
  test.identical( got, true );

  var got = _.entityAll( true, ( src, u, u2 ) => src !== u2 );
  test.identical( got, true );

  var got = _.entityAll( _.nothing, ( src, u, u2 ) => src !== u2 );
  test.identical( got, true );

  test.close( 'onEach is routine' );

  /* - */

  test.open( 'onEach is undefined' );

  test.case = 'Set';

  var src = new Set( [ 1, 'str', undefined ] );
  var got = _.entityAll( src );
  test.identical( got, undefined );

  var src = new Set( [ 1, 2, [ false ], 3, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, true );

  var src = new Set( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, false );

  /* */

  test.case = 'Map';

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityAll( src );
  test.identical( got, undefined );

  var src = new HashMap( [ [ 'a', 1 ], [ '4', [ false ] ], [ 'true', 'true' ] ] );
  var got = _.entityAll( src );
  test.identical( got, true );

  var src = new HashMap( [ [ 1, 2 ], [ 2, 3 ], [ 'a', '' ] ] );
  var got = _.entityAll( src );
  test.identical( got, '' );

  /* */

  test.case = 'array';

  var got = _.entityAll( [ 1, 'str', undefined ] );
  test.identical( got, undefined );

  var got = _.entityAll( [ 1, 'str', { 'a' : 2 }, 4 ] );
  test.identical( got, true );

  var got = _.entityAll( [ 1, 'str', { 'a' : 2 }, false ] );
  test.identical( got, false );

  /* */

  test.case = 'unroll';

  var src = _.unroll.from( [ 1, 2, _.unroll.from( [ 'str' ] ), null, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, null );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, true );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), false, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, false );

  /* */

  test.case = 'argument array';

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], undefined, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, undefined );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, true );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, false );

  /* */

  test.case = 'F32x';

  var src = new F32x( [ null, 2, [ 8 ], 3, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, 0 );

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityAll( src );
  test.identical( got, true );

  var src = new F32x( [ 1, 2, [ 8 ], 'str', 4 ] );
  var got = _.entityAll( src );
  test.identical( got, NaN );

  /* */

  test.case = 'Aux';

  var got = _.entityAll( { '1' : 2, 'c' : 4, 'a' : undefined } );
  test.identical( got, undefined );

  var got = _.entityAll( { '1' : 2, '2' : 3, 'a' : null } );
  test.identical( got, null );

  var got = _.entityAll( { 'a' : 1, 'b' : 3, 'c' : true } );
  test.identical( got, true );

  var got = _.entityAll( { 'a' : 'a', 'b' : 'str' } );
  test.identical( got, true );

  var got = _.entityAll( { 'a' : 1, 'b' : false } );
  test.identical( got, false );

  /* */

  test.case = 'no ArrayLike, no Aux'

  var got = _.entityAll( undefined );
  test.identical( got, undefined );

  var got = _.entityAll( null );
  test.identical( got, null );

  var got = _.entityAll( 1 );
  test.identical( got, true );

  var got = _.entityAll( 'str' );
  test.identical( got, true );

  var got = _.entityAll( false );
  test.identical( got, false );

  var got = _.entityAll( true );
  test.identical( got, true );

  test.close( 'onEach is undefined' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityAll() );

  test.case = 'too many arguments';
  test.shouldThrowErrorSync( () => _.entityAll( [ 'a' ], ( a ) => a, ( b ) => b ) );

  test.case = 'onEach has more then three arg';
  test.shouldThrowErrorSync( () => _.entityAll( [ 1 ], ( a, b, c, d ) => a + b + c + d ) );

  test.case = 'onEach is not a routine';
  test.shouldThrowErrorSync( () => _.entityAll( { 'a' : 2 }, [] ) );
}

//

function entityAny( test )
{
  test.open( 'onEach is routine' );

  test.case = 'Set';

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src, ( v, i ) => v >= 5 );
  test.identical( got, undefined );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src, ( v, i, c ) => v > 3 && c.size > 3 );
  test.identical( got, true );

  var src = new Set( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityAny( src, ( v, i ) =>  !v && !i );
  test.identical( got, true );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src, ( v ) => undefined );
  test.identical( got, undefined );

  var src = new Set( [ _.nothing, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src, ( v ) => v );
  test.identical( got, _.nothing );

  /* */

  test.case = 'Map';

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityAny( src, ( v, k ) => v === k );
  test.identical( got, undefined );

  var src = new HashMap( [ [ 'a', 'a' ], [ '4', '4' ], [ 'true', 'true' ] ] );
  var got = _.entityAny( src, ( v, k ) => v === k );
  test.identical( got, true );

  var src = new HashMap( [ [ 1, 2 ], [ 2, 3 ], [ 'a', null ] ] );
  var got = _.entityAny( src, ( v, k ) => v !== k );
  test.identical( got, true );

  var src = new HashMap( [ [ 'a', 1 ], [ 'c', true ], [ 'd', 'd' ] ] );
  var got = _.entityAny( src, ( v, k ) => typeof v === typeof k );
  test.identical( got, true );

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityAny( src, ( v, k, src ) => src.size === v );
  test.identical( got, undefined );

  var src = new HashMap( [ [ 1, _.nothing ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityAny( src, ( v, k, src ) => v );
  test.identical( got, _.nothing );

  /* */

  test.case = 'array';

  var got = _.entityAny( [ 1, 'str', undefined ], ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, true );

  var got = _.entityAny( [ false, false, undefined ], ( v, i ) => !!v && i + 2 < 6 );
  test.identical( got, undefined );

  var got = _.entityAny( [ 1, 'str', { 'a' : 2 }, false ], ( v, i ) => !!v && i + 2 < 6 );
  test.identical( got, true );

  var got = _.entityAny( [ 1, 'str', 3, null ], () => undefined );
  test.identical( got, undefined );

  var got = _.entityAny( [ _.nothing, 1, 'str', 3, null ], ( v ) => v );
  test.identical( got, _.nothing );

  /* */

  test.case = 'unroll';

  var src = _.unroll.from( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, true );

  var src = _.unroll.make( [ undefined, false, _.unroll.from( null ) ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, undefined );

  var src = _.unroll.make( [ undefined, false, _.unroll.from( [ 'str' ] ) ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAny( src, () => undefined );
  test.identical( got, undefined );

  var src = _.unroll.make( [ _.nothing, 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAny( src, ( v ) => v );
  test.identical( got, _.nothing );

  /* */

  test.case = 'argument array';

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, true );

  var src = _.argumentsArray.make( [ false, null, undefined ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, undefined );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src, () => undefined );
  test.identical( got, undefined );

  var src = _.argumentsArray.make( [ _.nothing, 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src, ( v ) => v );
  test.identical( got, _.nothing );

  /* */

  test.case = 'F32x';

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, true );

  var src = new F32x( [ 'a', undefined, false, null ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, undefined );

  var src = new F32x( [ 1, 2, [ 8 ], false, 4 ] );
  var got = _.entityAny( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityAny( src, () => undefined );
  test.identical( got, undefined );

  /* */

  test.case = 'Aux';

  var got = _.entityAny( { '1' : 2, 'c' : 4, 'a' : undefined }, ( v, k ) => v === k );
  test.identical( got, undefined );

  var got = _.entityAny( { 'a' : false, '4' : false, 'true' : 'true' }, ( v, k ) => v === k );
  test.identical( got, true );

  var got = _.entityAny( { '1' : 2, '2' : 3, 'a' : null }, ( v, k ) => typeof v === typeof k );
  test.identical( got, undefined );

  var got = _.entityAny( { 'a' : [], 'b' : 'str' }, ( v, k ) => typeof v === typeof k );
  test.identical( got, true );

  var got = _.entityAny( { 'a' : 1, 'b' : false }, ( v, k ) => v === k );
  test.identical( got, undefined );

  var got = _.entityAny( { 'a' : 1, 'b' : false }, ( v, k, u ) => v !== u );
  test.identical( got, true );

  var got = _.entityAny( { 'a' : _.nothing, 'b' : false }, ( v, k, u ) => v );
  test.identical( got, _.nothing );

  /* */

  test.case = 'no ArrayLike, no ObjectLike'

  var got = _.entityAny( undefined, ( src, u ) => src !== u );
  test.identical( got, undefined );

  var got = _.entityAny( null, ( src, u ) => src === u );
  test.identical( got, undefined );

  var got = _.entityAny( 1, ( src, u ) => src !== u );
  test.identical( got, true );

  var got = _.entityAny( 'str', ( src, u ) => src === u );
  test.identical( got, undefined );

  var got = _.entityAny( false, ( src, u ) => src !== u );
  test.identical( got, true );

  var got = _.entityAny( true, ( src, u ) => src !== u );
  test.identical( got, true );

  var got = _.entityAny( true, ( src, u, u2 ) => src !== u2 );
  test.identical( got, true );

  var got = _.entityAny( _.nothing, ( src, u, u2 ) => src );
  test.identical( got, _.nothing );

  test.close( 'onEach is routine' );

  /* - */

  test.open( 'onEach is undefined' );

  test.case = 'Set';

  var src = new Set( [ false, undefined, '' ] );
  var got = _.entityAny( src );
  test.identical( got, undefined );

  var src = new Set( [ 1, 2, [ false ], 3, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, 1 );

  var src = new Set( [ '', null, undefined, false, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, 4 );

  /* */

  test.case = 'Map';

  var src = new HashMap( [ [ 1, '' ], [ 'c', null ], [ 'a', undefined ] ] );
  var got = _.entityAny( src );
  test.identical( got, undefined );

  var src = new HashMap( [ [ 'a', 1 ], [ '4', [ false ] ], [ 'true', 'true' ] ] );
  var got = _.entityAny( src );
  test.identical( got, 1 );

  var src = new HashMap( [ [ 1, false ], [ 2, 0 ], [ 'a', '' ] ] );
  var got = _.entityAny( src );
  test.identical( got, undefined );

  /* */

  test.case = 'array';

  var got = _.entityAny( [ 1, 'str', undefined ] );
  test.identical( got, 1 );

  var got = _.entityAny( [ 'str', 1, { 'a' : 2 }, 4 ] );
  test.identical( got, 'str' );

  var got = _.entityAny( [ false, null, undefined ] );
  test.identical( got, undefined );

  /* */

  test.case = 'unroll';

  var src = _.unroll.from( [ false, 2, _.unroll.from( [ 'str' ] ), null, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, 2 );

  var src = _.unroll.make( [ undefined, false, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, 'str' );

  var src = _.unroll.make( [ null, undefined, false ] );
  var got = _.entityAny( src );
  test.identical( got, undefined );

  /* */

  test.case = 'argument array';

  var src = _.argumentsArray.make( [ null, false, [ 'str' ], undefined, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, [ 'str' ] );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, 1 );

  var src = _.argumentsArray.make( [ null, false, undefined ] );
  var got = _.entityAny( src );
  test.identical( got, undefined );

  /* */

  test.case = 'F32x';

  var src = new F32x( [ 5, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, 5 );

  var src = new F32x( [ undefined, [ 8 ], 3, 4 ] );
  var got = _.entityAny( src );
  test.identical( got, 8 );

  var src = new F32x( [ 'str', undefined, { 'a' : 2 } ] );
  var got = _.entityAny( src );
  test.identical( got, undefined );

  /* */

  test.case = 'ObjectLike';

  var got = _.entityAny( { '1' : 2, 'c' : 4, 'a' : undefined } );
  test.identical( got, 2 );

  var got = _.entityAny( { '1' : 2, '2' : 3, 'a' : null } );
  test.identical( got, 2 );

  var got = _.entityAny( { 'a' : false, 'b' : null, 'c' : undefined } );
  test.identical( got, undefined );

  var got = _.entityAny( { 'a' : 'a', 'b' : 'str' } );
  test.identical( got, 'a' );

  var got = _.entityAny( { 'a' : 1, 'b' : false } );
  test.identical( got, 1 );

  /* */

  test.case = 'no ArrayLike, no ObjectLike'

  var got = _.entityAny( undefined );
  test.identical( got, undefined );

  var got = _.entityAny( null );
  test.identical( got, undefined );

  var got = _.entityAny( 1 );
  test.identical( got, 1 );

  var got = _.entityAny( 'str' );
  test.identical( got, 'str' );

  var got = _.entityAny( false );
  test.identical( got, undefined );

  var got = _.entityAny( true );
  test.identical( got, true );

  test.close( 'onEach is undefined' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityAny() );

  test.case = 'too many arguments';
  test.shouldThrowErrorSync( () => _.entityAny( [ 'a' ], ( a ) => a, ( b ) => b ) );

  test.case = 'onEach has more then three arg';
  test.shouldThrowErrorSync( () => _.entityAny( [ 1 ], ( a, b, c, d ) => a + b + c + d ) );

  test.case = 'onEach is not a routine';
  test.shouldThrowErrorSync( () => _.entityAny( { 'a' : 2 }, [] ) );

}

//

function entityNone( test )
{
  test.open( 'onEach is routine' );

  test.case = 'Set';

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityNone( src, ( v, i ) => v >= 5 );
  test.identical( got, true );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityNone( src, ( v, i, c ) => v > 3 && c.size > 3 );
  test.identical( got, false );

  var src = new Set( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityNone( src, ( v, i ) =>  !v && !i );
  test.identical( got, false );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityNone( src, ( v ) => undefined );
  test.identical( got, true );

  var src = new Set( [ 1, 2, [ 'str' ], 3, 4, _.nothing ] );
  var got = _.entityNone( src, ( v ) => v );
  test.identical( got, false );

  /* */

  test.case = 'Map';

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityNone( src, ( v, k ) => v === k );
  test.identical( got, true );

  var src = new HashMap( [ [ 'a', 'a' ], [ '4', '4' ], [ 'true', 'true' ] ] );
  var got = _.entityNone( src, ( v, k ) => v === k );
  test.identical( got, false );

  var src = new HashMap( [ [ 1, 2 ], [ 2, 3 ], [ 'a', null ] ] );
  var got = _.entityNone( src, ( v, k ) => v !== k );
  test.identical( got, false );

  var src = new HashMap( [ [ 'a', 1 ], [ 'c', true ], [ 'd', 'd' ] ] );
  var got = _.entityNone( src, ( v, k ) => typeof v === typeof k );
  test.identical( got, false );

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', undefined ] ] );
  var got = _.entityNone( src, ( v, k, src ) => src.size === v );
  test.identical( got, true );

  var src = new HashMap( [ [ 1, 2 ], [ 'c', 4 ], [ 'a', _.nothing ] ] );
  var got = _.entityNone( src, ( v, k, src ) => v );
  test.identical( got, false );

  /* */

  test.case = 'array';

  var got = _.entityNone( [ 1, 'str', undefined ], ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var got = _.entityNone( [ false, false, undefined ], ( v, i ) => !!v && i + 2 < 6 );
  test.identical( got, true );

  var got = _.entityNone( [ 1, 'str', { 'a' : 2 }, false ], ( v, i ) => !!v && i + 2 < 6 );
  test.identical( got, false );

  var got = _.entityNone( [ 1, 'str', 3, null ], () => undefined );
  test.identical( got, true );

  var got = _.entityNone( [ 1, 'str', 3, _.nothing ], ( v ) => v );
  test.identical( got, false );

  /* */

  test.case = 'unroll';

  var src = _.unroll.from( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var src = _.unroll.make( [ undefined, false, _.unroll.from( null ) ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = _.unroll.make( [ undefined, false, _.unroll.from( [ 'str' ] ) ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, false );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityNone( src, () => undefined );
  test.identical( got, true );

  var src = _.unroll.make( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4, _.nothing ] );
  var got = _.entityNone( src, ( v ) => v );
  test.identical( got, false );

  /* */

  test.case = 'argument array';

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var src = _.argumentsArray.make( [ false, null, undefined ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], false, 4 ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, false );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityNone( src, () => undefined );
  test.identical( got, true );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4, _.nothing ] );
  var got = _.entityNone( src, ( v ) => v );
  test.identical( got, false );

  /* */

  test.case = 'F32x';

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 4 );
  test.identical( got, false );

  var src = new F32x( [ 'a', undefined, false, null ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, true );

  var src = new F32x( [ 1, 2, [ 8 ], false, 4 ] );
  var got = _.entityNone( src, ( v, i ) => !!v && i + 2 < 7 );
  test.identical( got, false );

  var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
  var got = _.entityNone( src, () => undefined );
  test.identical( got, true );

  /* */

  test.case = 'ObjectLike';

  var got = _.entityNone( { '1' : 2, 'c' : 4, 'a' : undefined }, ( v, k ) => v === k );
  test.identical( got, true );

  var got = _.entityNone( { 'a' : false, '4' : false, 'true' : 'true' }, ( v, k ) => v === k );
  test.identical( got, false );

  var got = _.entityNone( { '1' : 2, '2' : 3, 'a' : null }, ( v, k ) => typeof v === typeof k );
  test.identical( got, true );

  var got = _.entityNone( { 'a' : [], 'b' : 'str' }, ( v, k ) => typeof v === typeof k );
  test.identical( got, false );

  var got = _.entityNone( { 'a' : 1, 'b' : false }, ( v, k ) => v === k );
  test.identical( got, true );

  var got = _.entityNone( { 'a' : 1, 'b' : false }, ( v, k, src ) => src.length === v );
  test.identical( got, true );

  var got = _.entityNone( { 'a' : 1, 'b' : _.nothing }, ( v, k, src ) => v );
  test.identical( got, false );

  /* */

  test.case = 'no ArrayLike, no ObjectLike'

  var got = _.entityNone( undefined, ( src, u ) => src !== u );
  test.identical( got, true );

  var got = _.entityNone( null, ( src, u ) => src === u );
  test.identical( got, true );

  var got = _.entityNone( 1, ( src, u ) => src !== u );
  test.identical( got, false );

  var got = _.entityNone( 'str', ( src, u ) => src === u );
  test.identical( got, true );

  var got = _.entityNone( false, ( src, u ) => src !== u );
  test.identical( got, false );

  var got = _.entityNone( true, ( src, u ) => src !== u );
  test.identical( got, false );

  var got = _.entityNone( true, ( src, u, u2 ) => src !== u2 );
  test.identical( got, false );

  var got = _.entityNone( _.nothing, ( src, u, u2 ) => src );
  test.identical( got, false );

  test.close( 'onEach is routine' );

  /* - */

  test.open( 'onEach is undefined' );

  test.case = 'Set';

  var src = new Set( [ false, undefined, '' ] );
  var got = _.entityNone( src );
  test.identical( got, true );

  var src = new Set( [ 1, 2, [ false ], 3, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = new Set( [ '', null, undefined, false, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  /* */

  test.case = 'Map';

  var src = new HashMap( [ [ 1, '' ], [ 'c', null ], [ 'a', undefined ] ] );
  var got = _.entityNone( src );
  test.identical( got, true );

  var src = new HashMap( [ [ 'a', 1 ], [ '4', [ false ] ], [ 'true', 'true' ] ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = new HashMap( [ [ 1, false ], [ 2, 0 ], [ 'a', '' ] ] );
  var got = _.entityNone( src );
  test.identical( got, true );

  /* */

  test.case = 'array';

  var got = _.entityNone( [ 1, 'str', undefined ] );
  test.identical( got, false );

  var got = _.entityNone( [ 'str', 1, { 'a' : 2 }, 4 ] );
  test.identical( got, false );

  var got = _.entityNone( [ false, null, undefined ] );
  test.identical( got, true );

  /* */

  test.case = 'unroll';

  var src = _.unroll.from( [ false, 2, _.unroll.from( [ 'str' ] ), null, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = _.unroll.make( [ undefined, false, _.unroll.from( [ 'str' ] ), 3, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = _.unroll.make( [ null, undefined, false ] );
  var got = _.entityNone( src );
  test.identical( got, true );

  /* */

  test.case = 'argument array';

  var src = _.argumentsArray.make( [ null, false, [ 'str' ], undefined, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = _.argumentsArray.make( [ null, false, undefined ] );
  var got = _.entityNone( src );
  test.identical( got, true );

  /* */

  test.case = 'F32x';

  var src = new F32x( [ 5, 2, [ 'str' ], 3, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = new F32x( [ undefined, [ 8 ], 3, 4 ] );
  var got = _.entityNone( src );
  test.identical( got, false );

  var src = new F32x( [ 'str', undefined, { 'a' : 2 } ] );
  var got = _.entityNone( src );
  test.identical( got, true );

  /* */

  test.case = 'Aux';

  var got = _.entityNone( { '1' : 2, 'c' : 4, 'a' : undefined } );
  test.identical( got, false );

  var got = _.entityNone( { '1' : 2, '2' : 3, 'a' : null } );
  test.identical( got, false );

  var got = _.entityNone( { 'a' : false, 'b' : null, 'c' : undefined } );
  test.identical( got, true );

  var got = _.entityNone( { 'a' : 'a', 'b' : 'str' } );
  test.identical( got, false );

  var got = _.entityNone( { 'a' : 1, 'b' : false } );
  test.identical( got, false );

  /* */

  test.case = 'no ArrayLike, no Aux'

  var got = _.entityNone( undefined );
  test.identical( got, true );

  var got = _.entityNone( null );
  test.identical( got, true );

  var got = _.entityNone( 1 );
  test.identical( got, false );

  var got = _.entityNone( 'str' );
  test.identical( got, false );

  var got = _.entityNone( false );
  test.identical( got, true );

  var got = _.entityNone( true );
  test.identical( got, false );

  test.close( 'onEach is undefined' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityNone() );

  test.case = 'too many arguments';
  test.shouldThrowErrorSync( () => _.entityNone( [ 'a' ], ( a ) => a, ( b ) => b ) );

  test.case = 'onEach has more then three arguments';
  test.shouldThrowErrorSync( () => _.entityNone( [ 1 ], ( a, b, c, d ) => a + b + c + d ) );

  test.case = 'onEach is not a routine';
  test.shouldThrowErrorSync( () => _.entityNone( { 'a' : 2 }, [] ) );
}

//

function _filter_functor( test )
{
  test.case = 'routine, levels - 1';
  var src = ( e, k, s ) => e;
  var condition = _._filter_functor( src, 1 );
  var obj = { 'a' : 1, 'b' : 2 };
  var got = condition( obj, 'a', 1 );
  test.true( got === obj );
  test.identical( got, obj );

  test.case = 'passed element is original object, levels - 1';
  var src = { 'a' : 1, 'b' : 2 };
  var condition = _._filter_functor( src, 1 );
  var got = condition( src, 'a', src );
  test.true( got === src );
  test.identical( got, { 'a' : 1, 'b' : 2 } );

  test.case = 'passed element is original object, levels - -1';
  var src = { 'a' : 1, 'b' : 2 };
  var condition = _._filter_functor( src, -1 );
  var got = condition( src, 'a', src );
  test.true( got === src );
  test.identical( got, { 'a' : 1, 'b' : 2 } );

  test.case = 'passed element is not object, levels - 1';
  var src = { 'a' : 1, 'b' : 2 };
  var condition = _._filter_functor( src, 1 );
  var got = condition( 1, 'a', src );
  test.identical( got, undefined );

  /* */

  test.case = 'src - 1 level, obj - copy of src, levels - 0';
  var src = { 'a' : 1, 'b' : 2 };
  var condition = _._filter_functor( src, 0 );
  var obj = { 'a' : 1, 'b' : 2 };
  var got = condition( obj, 'a', src );
  test.identical( got, undefined );

  test.case = 'callback identicalWith returns true, levels - 0';
  var identicalWith = ( e ) => true;
  var src = { 'a' : 1, 'b' : 2, 'identicalWith' : identicalWith };
  var condition = _._filter_functor( src, 0 );
  var obj = { 'c' : 1, 'identicalWith' : identicalWith };
  var got = condition( obj, 'a', src );
  test.true( got === obj );
  test.identical( got, obj );

  test.case = 'src - level 1, obj - copy of src, levels - 1';
  var src = { 'a' : 1, 'b' : 2 };
  var condition = _._filter_functor( src, 1 );
  var obj = { 'a' : 1, 'b' : 2 };
  var got = condition( obj, 'a', src );
  test.true( got === obj );
  test.identical( got, obj );

  test.case = 'src - level 2, obj - copy of src, levels - 1';
  var src = { 'a' : { 'c' : 2 }, 'b' : 2 };
  var condition = _._filter_functor( src, 1 );
  var obj = { 'a' : { 'c' : 2 }, 'b' : 2 };
  var got = condition( obj, 'a', src );
  test.identical( got, undefined );

  test.case = 'src - level 2, obj - copy of src, levels - 2';
  var src = { 'a' : { 'c' : 2 }, 'b' : 2 };
  var condition = _._filter_functor( src, 2 );
  var obj = { 'a' : { 'c' : 2 }, 'b' : 2 };
  var got = condition( obj, 'a', src );
  test.true( got === obj );
  test.identical( got, obj );

  test.case = 'src - level, obj - level 2, levels - 2';
  var src = { 'a' : { 'c' : 2 }, 'b' : 2 };
  var condition = _._filter_functor( src, 2 );
  var obj = { 'a' : { 'd' : 2 }, 'b' : 2 };
  var got = condition( obj, 'a', src );
  test.identical( got, undefined );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _._filter_functor() );

  test.case = 'one argument';
  test.shouldThrowErrorSync( () => _._filter_functor( { 'a' : 1 } ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _._filter_functor( { 'a' : 1 }, 1, 1 ) );

  test.case = 'wrong type of condition';
  test.shouldThrowErrorSync( () => _._filter_functor( 'wrong', 1 ) );
  test.shouldThrowErrorSync( () => _._filter_functor( [ 1, 2 ], 1 ) );

  test.case = 'wrong quantity arguments in maked routine';
  var condition = _._filter_functor( { 'a' : 1 }, 1 );
  test.shouldThrowErrorSync( () => condition() );
  test.shouldThrowErrorSync( () => condition( { 'a' : 1 } ) );
  test.shouldThrowErrorSync( () => condition( { 'a' : 1 }, 'a' ) );
  test.description = 'check of routine';
  test.mustNotThrowError( () => condition( { 'a' : 1 }, 'a', { 'a' : 1 } ) );
}

//

function entityMapCountables( test )
{
  const makerChecker =
  [
    { maker : 'long', checker : 'long' },
    { maker : 'array', checker : 'array' },
    { maker : 'unroll', checker : 'unroll' },
    { maker : 'countable', checker : 'countable' },
    { maker : 'vector', checker : 'vector' },
    { maker : 'argumentsArray', checker : 'argumentsArray' },

    // { maker : 'f32', checker : 'f32' }, /* Dmytro : does not work */
    // { maker : 'set', checker : 'set' }, /* Dmytro : does not work */
  ];

  _.each( makerChecker, ( env ) =>
  {
    env.inplace = false;
    run( env );
  });

  _.each( makerChecker, ( env ) =>
  {
    env.inplace = true;
    if( env.maker === 'argumentsArray' )
    env.checker = 'argumentsArray';
    run( env );
  });

  /* */

  function run( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    test.case = 'empty container';
    var src = _[ env.maker ].make( [] );
    var got = _.map_( env.inplace ? src : null, src, ( e ) => true );
    test.identical( got, _[ env.checker ].make( [] ) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns 1';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.map_( env.inplace ? src : null, src, ( e ) => 1 );
    test.identical( got, _[ env.checker ].make([ 1, 1, 1 ]) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns element';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.map_( env.inplace ? src : null, src, ( e ) => e );
    test.identical( got, _[ env.checker ].make([ 1, 2, 3 ]) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns key + 1';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.map_( env.inplace ? src : null, src, ( e, k ) => k );
    test.identical( got, _[ env.checker ].make([ 0, 1, 2 ]) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback checks container length/size';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.map_( env.inplace ? src : null, src, ( e, k, c ) => c.length || c.size );
    test.identical( got, _[ env.checker ].make([ 3, 3, 3 ]) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns undefined';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.map_( env.inplace ? src : null, src, ( e ) => undefined );
    var result = env.inplace ? [ 1, 2, 3 ] : [ undefined, undefined, undefined ];
    test.identical( got, _[ env.checker ].make( result ) );
    test.identical( src === got, env.inplace );

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.map_() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.map_( [] ) );
  test.shouldThrowErrorSync( () => _.map_( null, [] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.map_( null, [], ( e ) => e, 'extra' ) );

  test.case = 'wrong type of callback';
  test.shouldThrowErrorSync( () => _.map_( null, [], 'wrong' ) );
}

//

function entityMapObjects( test )
{
  const namespaces =
  [
    'props',
    'map',
    'aux',
    'object',
    'container',
  ];

  _.each( namespaces, ( namespace ) =>
  {
    const env = { namespace };
    env.inplace = false;
    run( env );
  });

  _.each( namespaces, ( namespace ) =>
  {
    const env = { namespace };
    env.inplace = true;
    run( env );
  });

  /* */

  function run( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    test.case = 'empty container';
    var src = _[ env.namespace ].make({});
    var got = _.map_( env.inplace ? src : null, src, ( e ) => true );
    test.identical( got, {} );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns 1';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.map_( env.inplace ? src : null, src, ( e ) => 1 );
    test.identical( got, { a : 1, b : 1, c : 1 } );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns element';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.map_( env.inplace ? src : null, src, ( e ) => e );
    test.identical( got, { a : 1, b : 2, c : 3 } );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns key + 1';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.map_( env.inplace ? src : null, src, ( e, k ) => k + 1 );
    test.identical( got, { a : 'a1', b : 'b1', c : 'c1' } );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback checks container';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.map_( env.inplace ? src : null, src, ( e, k, c ) => c.c );
    test.identical( got, _[ env.namespace ].make({ a : 3, b : 3, c : 3 }) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns undefined';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.map_( env.inplace ? src : null, src, ( e ) => undefined );
    var exp = env.inplace ? { a : 1, b : 2, c : 3 } : {};
    test.identical( got, exp );
    test.identical( src === got, env.inplace );

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }
}

//

// function entityMap( test )
// {
//   test.open( 'src is arrayLike' );
//
//   test.case = 'simple test with mapping array by sqr';
//   var got = _.container.map_( null, [ 3, 4, 5 ], ( v, i, ent ) => v * v );
//   test.identical( got, [ 9, 16, 25 ] );
//
//   test.case = 'array';
//   var src = [ 1, 2, null, 'str' ];
//   var got = _.container.map_( null, src, ( v, i, s ) => v + i );
//   test.identical( got, [ 1, 3, 2, 'str3' ] );
//
//   test.case = 'unroll';
//   var src = _.unroll.from( [ 1, 2, _.unroll.from( [ 'str' ] ), 3, 4 ] );
//   var got = _.container.map_( null, src, ( v, i, s ) => v + i );
//   test.identical( got, [ 1, 3, 'str2', 6, 8 ] );
//   test.true( _.arrayIs( got ) );
//   test.false( _.unrollIs( got ) );
//
//   test.case = 'argument array';
//   var src = _.argumentsArray.make( [ 1, 2, [ 'str' ], 3, 4 ] );
//   var got = _.container.map_( null, src, ( v, i, s ) => v + i );
//   test.identical( got, [ 1, 3, 'str2', 6, 8 ] );
//   test.true( _.arrayIs( got ) );
//
//   test.case = 'Array';
//   var src = new Array( 1, 2, [ 'str' ], 3, 4 );
//   var got = _.container.map_( null, src, ( v, i, s ) => v + i );
//   test.identical( got, [ 1, 3, 'str2', 6, 8 ] );
//   test.true( _.arrayIs( got ) );
//
//   test.case = 'F32x';
//   var src = new F32x( [ 1, 2, [ 8 ], 3, 4 ] );
//   var got = _.container.map_( null, src, ( v, i, s ) => v + i );
//   test.equivalent( got, [ 1, 3, 10, 6, 8 ] );
//   test.true( _.longIs( got ) );
//
//   test.close( 'src is arrayLike' );
//
//   /* - */
//
//   test.open( 'src is objectLike' );
//
//   test.case = 'simple test with mapping object by sqr';
//   var got = _.container.map_( null, { '3' : 3, '4' : 4, '5' : 5 }, ( v, i, ent ) => v * v );
//   test.identical( got, { '3' : 9, '4' : 16, '5' : 25 } );
//   test.true( _.mapIs( got ) );
//
//   var src = { 'a' : 1, 'b' : 2, 'c' : null, 'd' : 'str' };
//   var got = _.container.map_( null, src, ( v, k, s ) => v + k );
//   test.identical( got, { 'a' : '1a', 'b' : '2b', 'c' : 'nullc', 'd' : 'strd' } );
//   test.true( _.mapIs( got ) );
//
//   test.case = 'simple test with mapping object by sqr : check callback arguments';
//   var callback = function( v, i, ent )
//   {
//     if( externEnt )
//     externEnt = ent;
//     return v * v + i;
//   };
//   var externEnt = {};
//   var got = _.container.map_( null, Object.assign( {}, { 'a' : 1, 'b' : 3, 'c' : 4 } ), callback );
//   test.identical( externEnt, { 'a' : 1, 'b' : 3, 'c' : 4 } );
//
//   test.case = 'mapping object by sqr : source object should be unmodified';
//   test.identical( Object.is( got, Object.assign( {}, { 'a' : 1, 'b' : 3, 'c' : 4 } ) ), false );
//
//   test.close( 'src is objectLike' );
//
//   /* - */
//
//   test.open( 'not longLike, not aixiliary' );
//
//   test.case = 'number';
//   var got = _.container.map_( null, 2, ( v, u, u2 ) => v + v );
//   test.identical( got, 4 );
//
//   test.case = 'string';
//   var got = _.container.map_( null, 'a', ( v, u, u2 ) => v + v );
//   test.identical( got, 'aa' );
//
//   test.case = 'Set';
//   var src = new Set([ 1, 2 ]);
//   var got = _.container.map_( null, src, ( v, u, u2 ) => v );
//   test.identical( got, src );
//
//   test.case = 'object, onEach returns undefined';
//   function constr()
//   {
//     this.a = 1;
//     this.b = 3;
//     this.c = 4;
//     return this;
//   }
//   var src = new constr();
//   var got = _.container.map_( null, src, ( e, i, ent ) => undefined );
//   test.identical( got, src );
//   test.true( got === src );
//
//   test.close( 'not longLike, not aixiliary' );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'missed arguments';
//   test.shouldThrowErrorSync( () => _.container.map_( null ) );
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.container.map_( null, [ 1, 3 ], callback1, callback2 ) );
//
//   test.case = 'second argument is not routine';
//   test.shouldThrowErrorSync( () => _.container.map_( null, [ 1, 2 ], {} ) );
// }
//
// //
//
// function entityMapDifferentCallbacks( test )
// {
//   test.open( 'longLike' );
//
//   test.case = 'onEach returns element';
//   var src = [ 3, -1, 2 ];
//   var got = _.container.map_( null, src, ( v, i, ent ) => v );
//   test.identical( got, [ 3, -1, 2 ] );
//   test.true( got !== src );
//
//   test.case = 'onEach returns key';
//   var src = [ 3, -1, 2 ];
//   var got = _.container.map_( null, src, ( v, i, ent ) => i );
//   test.identical( got, [ 0, 1, 2 ] );
//   test.true( got !== src );
//
//   test.case = 'onEach checks entity';
//   var src = [ 3, -1, 2 ];
//   var got = _.container.map_( null, src, ( v, i, ent ) => ent.length > 2 ? v : i );
//   test.identical( got, [ 3, -1, 2 ] );
//   test.true( got !== src );
//
//   test.case = 'onEach returns undefined';
//   var src = [ 3, -1, 2 ];
//   var got = _.container.map_( null, src, ( v, i, ent ) => undefined );
//   test.identical( got, [ 3, -1, 2 ] );
//   test.true( got !== src );
//
//   test.close( 'longLike' );
//
//   /* - */
//
//   test.open( 'aixiliary' );
//
//   test.case = 'onEach returns element';
//   var src = { 'a' : 1, 'b' : 'a', 'c' : -1 };
//   var got = _.container.map_( null, src, ( v, i, ent ) => v );
//   test.identical( got, { 'a' : 1, 'b' : 'a', 'c' : -1 } );
//   test.true( got !== src );
//
//   test.case = 'onEach returns key';
//   var src = { 'a' : 1, 'b' : 'a', 'c' : -1 };
//   var got = _.container.map_( null, src, ( v, i, ent ) => i );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
//   test.true( got !== src );
//
//   test.case = 'onEach checks entity';
//   var src = { 'a' : 1, 'b' : 'a', 'c' : -1 };
//   var got = _.container.map_( null, src, ( v, i, ent ) => ent.a > 2 ? v : i );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
//   test.true( got !== src );
//
//   test.case = 'onEach returns undefined';
//   var src = { 'a' : 1, 'b' : 'a', 'c' : -1 };
//   var got = _.container.map_( null, src, ( v, i, ent ) => undefined );
//   test.identical( got, { 'a' : 1, 'b' : 'a', 'c' : -1 } );
//   test.true( got !== src );
//
//   test.close( 'aixiliary' );
//
//   /* - */
//
//   test.open( 'primitive' );
//
//   test.case = 'onEach returns element';
//   var src = 'str';
//   var got = _.container.map_( null, src, ( v, i, ent ) => v + 1 );
//   test.identical( got, 'str1' );
//   test.true( got !== src );
//
//   test.case = 'onEach returns key';
//   var src = 'str';
//   var got = _.container.map_( null, src, ( v, i, ent ) => i );
//   test.identical( got, 'str' );
//   test.true( got === src );
//
//   test.case = 'onEach checks entity';
//   var src = 'str';
//   var got = _.container.map_( null, src, ( v, i, ent ) => ent === undefined ? v : i );
//   test.identical( got, 'str' );
//   test.true( got === src );
//
//   test.case = 'onEach returns undefined';
//   var src = 'str';
//   var got = _.container.map_( null, src, ( v, i, ent ) => undefined );
//   test.identical( got, 'str' );
//   test.true( got === src );
//
//   test.close( 'primitive' );
// }

// //
//
// function entityMapWithoutDst_( test )
// {
//   test.open( 'onEach returns element' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, _.unroll.make( [ 1, 2, 3, 4, 'str' ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 1, 2, 3, 4, 'str' ] );
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, _.argumentsArray.make( [ 1, 2, 3, 4, 'str' ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new U8x( [ 1, 2, 3, 4, 5 ] );
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, new U8x( [ 1, 2, 3, 4, 5 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, { 'a' : 1, 'b' : 2, 'c' : 'str' } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = 2;
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, { 'a' : 1, 'b' : 2 } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, null );
//
//   test.case = 'undefined';
//   var src = undefined;
//   var got = _.container.map_( src, ( e ) => String( e ) );
//   test.identical( got, 'undefined' );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, 'str' );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, new Set() );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.map_( src, ( e ) => e );
//   test.identical( got, new BufferRaw() );
//
//   test.close( 'onEach returns element' );
//
//   /* - */
//
//   test.open( 'onEach returns key' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] );
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new I16x( [ 1, 2, 3, 4, 5 ] );
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, new I16x( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = 2;
//   var got = _.container.map_( src, ( e, k ) => k );
//   test.identical( got, { 'a' : 'a', 'b' : 'b' } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.map_( src, ( e, k ) => String( k ) );
//   test.identical( got, 'undefined' );
//
//   test.case = 'undefined';
//   var src = undefined;
//   var got = _.container.map_( src, ( e, k ) => String( k ) );
//   test.identical( got, 'undefined' );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.map_( src, ( e, k ) => String( k ) );
//   test.identical( got, 'undefined' );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.map_( src, ( e, k ) => String( k ) );
//   test.identical( got, 'undefined' );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.map_( src, ( e, k ) => String( k ) );
//   test.identical( got, 'undefined' );
//
//   test.close( 'onEach returns key' );
//
//   /* - */
//
//   test.open( 'onEach checks container' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] );
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new F64x( [ 1, 2, 3, 4, 5 ] );
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, new F64x( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = 2;
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, { 'a' : 'a', 'b' : 'b' } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, null );
//
//   test.case = 'undefined';
//   var src = undefined;
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : String( e ) );
//   test.identical( got, 'undefined' );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'str' );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, new Set() );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.map_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, new BufferRaw() );
//
//   test.close( 'onEach checks container' );
//
//   /* - */
//
//   test.open( 'onEach returns undefined' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, 2, 3 ] );
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, _.unroll.make( [ 1, 2, 3 ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 0, 1, 2 ] );
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, _.argumentsArray.make( [ 0, 1, 2 ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new F64x( [ 1, 2, 3 ] );
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, new F64x( [ 1, 2, 3 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, { 'a' : 1, 'b' : 2, 'c' : 'str' } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, Object.create( null ) );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = 2;
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, { 'a' : 1, 'b' : 2 } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, null );
//
//   test.case = 'undefined';
//   var src = undefined;
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, 'str' );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, new Set() );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.map_( src, ( e, k, c ) => undefined );
//   test.identical( got, new BufferRaw() );
//
//   test.close( 'onEach returns undefined' );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.container.map_() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.container.map_( 'str' ) );
//   test.shouldThrowErrorSync( () => _.container.map_( [ 1, 2 ] ) );
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.container.map_( [ 1, 3 ], [ 1, 2 ], ( e ) => e, 'extra' ) );
//   test.shouldThrowErrorSync( () => _.container.map_( { 'a' : 1 }, { 'b' : 2 }, ( e ) => e, 'extra' ) );
//
//   test.case = 'wrong type of onEach';
//   test.shouldThrowErrorSync( () => _.container.map_( [ 1, 2 ], {} ) );
//   test.shouldThrowErrorSync( () => _.container.map_( null, [ 1, 2 ], {} ) );
//
//   test.case = 'wrong dst container';
//   test.shouldThrowErrorSync( () => _.container.map_( {}, [ 1, 2 ], ( e ) => e ) );
//   test.shouldThrowErrorSync( () => _.container.map_( [], { 'a' : 1, 'b' : 2 }, ( e ) => e ) );
//   test.shouldThrowErrorSync( () => _.container.map_( new BufferRaw(), { 'a' : 1, 'b' : 2 }, ( e ) => e ) );
//
//   test.case = 'dst is not resizable long';
//   test.shouldThrowErrorSync( () => _.container.map_( new F32x(), { 'a' : 1, 'b' : 2 }, ( e ) => e ) );
//
//   test.case = 'not map like, not long like, cannot extend map';
//   test.shouldThrowErrorSync( () => _.container.map_( {}, 1, ( e ) => e ) );
// }

//

function entityMapDstNull_( test )
{
  test.open( 'onEach returns element' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, _.unroll.make( [ 1, 2, 3, 4, 'str' ] ) );
  test.true( got !== src );
  test.true( _.arrayIs( got ) );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 1, 2, 3, 4, 'str' ] );
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, _.argumentsArray.make([ 1, 2, 3, 4, 'str' ]) );
  test.true( got !== src );

  test.case = 'BufferTyped';
  var src = new U8x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, new U8x( [ 1, 2, 3, 4, 5 ] ) );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, { 'a' : 1, 'b' : 2, 'c' : 'str' } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, { 'a' : 1, 'b' : 2 } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, null );

  test.case = 'undefined';
  var src = undefined;
  var got = _.container.map_( null, src, ( e ) => String( e ) );
  test.identical( got, 'undefined' );

  test.case = 'string';
  var src = 'str';
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, 'str' );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, new Set() );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.map_( null, src, ( e ) => e );
  test.identical( got, new BufferRaw() );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
  test.true( got !== src );
  test.true( _.arrayIs( got ) );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] );
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, _.argumentsArray.make([ 0, 1, 2, 3, 4 ]) );
  test.true( got !== src );

  test.case = 'BufferTyped';
  var src = new I16x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, new I16x( [ 0, 1, 2, 3, 4 ] ) );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var got = _.container.map_( null, src, ( e, k ) => k );
  test.identical( got, { 'a' : 'a', 'b' : 'b' } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.map_( null, src, ( e, k ) => String( k ) );
  test.identical( got, 'undefined' );

  test.case = 'undefined';
  var src = undefined;
  var got = _.container.map_( null, src, ( e, k ) => String( k ) );
  test.identical( got, 'undefined' );

  test.case = 'string';
  var src = 'str';
  var got = _.container.map_( null, src, ( e, k ) => String( k ) );
  test.identical( got, 'undefined' );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.map_( null, src, ( e, k ) => String( k ) );
  test.identical( got, 'undefined' );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.map_( null, src, ( e, k ) => String( k ) );
  test.identical( got, 'undefined' );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach checks container' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
  test.true( got !== src );
  test.true( _.arrayIs( got ) );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] );
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, _.argumentsArray.make([ 0, 1, 2, 3, 4 ]) );
  test.true( got !== src );

  test.case = 'BufferTyped';
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new F64x( [ 0, 1, 2, 3, 4 ] ) );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { 'a' : 'a', 'b' : 'b' } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, null );

  test.case = 'undefined';
  var src = undefined;
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : String( e ) );
  test.identical( got, 'undefined' );

  test.case = 'string';
  var src = 'str';
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, 'str' );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new Set() );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.map_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new BufferRaw() );

  test.close( 'onEach checks container' );
}

//

function entityMapDstNotNull_( test )
{
  test.open( 'onEach returns element' );

  test.case = 'dst - argumentsArray, empty array';
  var dst = _.argumentsArray.make( [ 1, 2, 3 ] );
  var src = [];
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, _.argumentsArray.make( [ 1, 2, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst - array, unroll';
  var dst = [ 'a', 'b', 'c', 'd', 'e', 'f' ];
  var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, _.unroll.make( [ 1, 2, 3, 4, 'str', 'f' ] ) );
  test.true( got === dst );

  test.case = 'dst - unroll, argumentsArray';
  var dst = _.unroll.make( [ 'a', 'b', 'c', 'd', 'e', 'f' ] );
  var src = _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] );
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, [ 0, 1, 2, 3, 4, 'f' ] );
  test.true( got === dst );

  test.case = 'dst - BufferTyped, BufferTyped';
  var dst = new U16x( 10 );
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, new U16x( [ 1, 2, 3, 4, 5, 0, 0, 0, 0, 0 ] ) );
  test.true( got === dst );

  /* */

  test.case = 'dst - map, empty map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = {};
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, { 'a' : 2, 'd' : 2 } );
  test.true( got === dst );

  test.case = 'dst - pure map, map';
  var dst = Object.create( null );
  dst.a = 2;
  dst.d = 2;
  var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, { 'a' : 1, 'b' : 2, 'c' : 'str', 'd' : 2 } );
  test.true( got === dst );

  test.case = 'dst - map, empty pure map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = Object.create( null );
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, { 'a' : 2, 'd' : 2 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, { 'a' : 1, 'b' : 2, 'd' : 2 } );
  test.true( got === dst );

  /* */

  test.case = 'dst - array, null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, [ 1, 2, null ] );
  test.true( got === dst );

  test.case = 'dst - map, undefined';
  var dst = { 'a' : 1 };
  var src = undefined;
  var got = _.container.map_( dst, src, ( e ) => { return { 'e' : e } } );
  test.identical( got, { 'a' : 1, 'undefined' : undefined } );
  test.true( got === dst );

  test.case = 'dst - undefined, string';
  var dst = undefined;
  var src = 'str';
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, 'str' );

  test.case = 'dst - emty unroll, Set';
  var dst = _.unroll.make( [] );
  var src = new Set();
  var got = _.container.map_( dst, src, ( e ) => e );
  test.identical( got, [ new Set() ] );
  test.true( got === dst );

  test.case = 'dst - empty pure map, BufferRaw';
  var dst = Object.create( null );
  var src = new BufferRaw();
  var got = _.container.map_( dst, src, ( e ) => [ e ] );
  test.identical( got, { '0' : new BufferRaw() } );
  test.true( got === dst );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'dst - argumentsArray, empty array';
  var dst = _.argumentsArray.make( [ 1, 2, 3 ] );
  var src = [];
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, _.argumentsArray.make( [ 1, 2, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst - array, unroll';
  var dst = [ 'a', 'b', 'c', 'd', 'e', 'f' ];
  var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4, 'f' ] ) );
  test.true( got === dst );

  test.case = 'dst - unroll, argumentsArray';
  var dst = _.unroll.make( [ 'a', 'b', 'c', 'd', 'e', 'f' ] );
  var src = _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] );
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, [ 0, 1, 2, 3, 4, 'f' ] );
  test.true( got === dst );

  test.case = 'dst - BufferTyped, BufferTyped';
  var dst = new U16x( 10 );
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, new U16x( [ 0, 1, 2, 3, 4, 0, 0, 0, 0, 0 ] ) );
  test.true( got === dst );

  /* */

  test.case = 'dst - map, empty map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = {};
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, { 'a' : 2, 'd' : 2 } );
  test.true( got === dst );

  test.case = 'dst - pure map, map';
  var dst = Object.create( null );
  dst.a = 2;
  dst.d = 2;
  var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c', 'd' : 2 } );
  test.true( got === dst );

  test.case = 'dst - map, empty pure map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = Object.create( null );
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, { 'a' : 2, 'd' : 2 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var got = _.container.map_( dst, src, ( e, k ) => k );
  test.identical( got, { 'a' : 'a', 'b' : 'b', 'd' : 2 } );
  test.true( got === dst );

  /* */

  test.case = 'dst - array, null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.map_( dst, src, ( e, k ) => String( k ) );
  test.identical( got, [ 1, 2, 'undefined' ] );
  test.true( got === dst );

  test.case = 'dst - map, undefined';
  var dst = { 'a' : 1 };
  var src = undefined;
  var got = _.container.map_( dst, src, ( e, k ) => { return { 'k' : k } } );
  test.identical( got, { 'a' : 1, 'undefined' : undefined } );
  test.true( got === dst );

  test.case = 'dst - undefined, string';
  var dst = undefined;
  var src = 'str';
  var got = _.container.map_( dst, src, ( e, k ) => String( k ) );
  test.identical( got, 'undefined' );

  test.case = 'dst - emty unroll, Set';
  var dst = _.unroll.make( [] );
  var src = new Set();
  var got = _.container.map_( dst, src, ( e, k ) => String( k ) );
  test.identical( got, [ 'undefined' ] );
  test.true( got === dst );

  test.case = 'dst - empty pure map, BufferRaw';
  var dst = Object.create( null );
  var src = new BufferRaw();
  var got = _.container.map_( dst, src, ( e, k ) => [ k ] );
  test.identical( got, { '0' : undefined } );
  test.true( got === dst );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach checks container' );

  test.case = 'dst - argumentsArray, empty array';
  var dst = _.argumentsArray.make( [ 1, 2, 3 ] );
  var src = [];
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, _.argumentsArray.make( [ 1, 2, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst - array, unroll';
  var dst = [ 'a', 'b', 'c', 'd', 'e', 'f' ];
  var src = _.unroll.make( [ 1, 2, 3, 4, 'str' ] );
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4, 'f' ] ) );
  test.true( got === dst );

  test.case = 'dst - unroll, argumentsArray';
  var dst = _.unroll.make( [ 'a', 'b', 'c', 'd', 'e', 'f' ] );
  var src = _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] );
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 0, 1, 2, 3, 4, 'f' ] );
  test.true( got === dst );

  test.case = 'dst - BufferTyped, BufferTyped';
  var dst = new U16x( 10 );
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new U16x( [ 0, 1, 2, 3, 4, 0, 0, 0, 0, 0 ] ) );
  test.true( got === dst );

  /* */

  test.case = 'dst - map, empty map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = {};
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { 'a' : 2, 'd' : 2 } );
  test.true( got === dst );

  test.case = 'dst - pure map, map';
  var dst = Object.create( null );
  dst.a = 2;
  dst.d = 2;
  var src = { 'a' : 1, 'b' : 2, 'c' : 'str' };
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c', 'd' : 2 } );
  test.true( got === dst );

  test.case = 'dst - map, empty pure map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = Object.create( null );
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { 'a' : 2, 'd' : 2 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { 'a' : 2, 'd' : 2 };
  var src = Object.create( null );
  src.a = 1;
  src.b = 2;
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { 'a' : 'a', 'b' : 'b', 'd' : 2 } );
  test.true( got === dst );

  /* */

  test.case = 'dst - array, null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 1, 2, null ] );
  test.true( got === dst );

  test.case = 'dst - map, undefined';
  var dst = { 'a' : 1 };
  var src = undefined;
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : { 'e' : e } );
  test.identical( got, { 'a' : 1, 'undefined' : undefined } );
  test.true( got === dst );

  test.case = 'dst - undefined, string';
  var dst = undefined;
  var src = 'str';
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, 'str' );

  test.case = 'dst - emty unroll, Set';
  var dst = _.unroll.make( [] );
  var src = new Set();
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ new Set() ] );
  test.true( got === dst );

  test.case = 'dst - empty pure map, BufferRaw';
  var dst = Object.create( null );
  var src = new BufferRaw();
  var got = _.container.map_( dst, src, ( e, k, c ) => c ? k : [ e ] );
  test.identical( got, { '0' : new BufferRaw() } );
  test.true( got === dst );

  test.close( 'onEach checks container' );
}

//

/*
TODO : need to check actuality of this test | 'Dmytro' : it works correctly
*/

// function entityFilter( test )
// {
//   test.open( 'onEach is routine' );
//
//   test.case = 'number';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   var got = _.filter_( null, 3, callback );
//   test.identical( got, Math.sqrt( 3 ) );
//
//   test.case = 'string';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   var got = _.filter_( null, 'str', ( v ) => v + ' ' + v );
//   test.identical( got, 'str str' );
//
//   test.case = 'array';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   var got = _.filter_( null, [ 9, -16, 25, 36, -49 ], callback );
//   test.identical( got, [ 3, 5, 6 ] );
//   test.notIdentical( got, [ 3, 4, 5, 6, 7 ] );
//
//   test.case = 'unroll';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   var src = _.unroll.make( [ 9, _.unroll.make( [ -16, 25, _.unroll.from( [ 36, -49 ] ) ] ) ] );
//   var got = _.filter_( null, src, callback );
//   test.identical( got, [ 3, 5, 6 ] );
//   test.notIdentical( got, [ 3, 4, 5, 6, 7 ] );
//   test.false( _.unrollIs( got) );
//
//   test.case = 'argumentsArray';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   var src = _.argumentsArray.make( [ 9, -16, 25, 36, -49 ] );
//   var got = _.filter_( null, src, callback );
//   test.identical( got, [ 3, 5, 6 ] );
//
//   test.case = 'BufferTyped to Array';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   var src = new F32x( [ 9, -16, 25, 36, -49 ] );
//   var src = Array.from( src );
//   var got = _.filter_( null, src, callback );
//   test.identical( got, [ 3, 5, 6 ] );
//   test.notIdentical( got, [ 3, 4, 5, 6, 7 ] );
//
//   test.case = 'aixiliary';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   var got = _.filter_( null, { '3' : 9, '4' : 16, '5' : 25, '6' : -36 }, callback );
//   test.identical( got, { '3' : 3, '4' : 4, '5' : 5 } );
//   test.notIdentical( got, { '3' : 3, '4' : 4, '5' : 5, '6' : 6 } );
//
//   test.case = 'callback in routine';
//   var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
//   function testFn2()
//   {
//     return _.filter_( null, arguments, callback );
//   }
//   var got = testFn2( 9, -16, 25, 36, -49 );
//   test.identical( got, [ 3, 5, 6 ] );
//
//   test.case = 'src is array, filter makes unrolls';
//   var onEach = ( e, i, s ) => _.unroll.make( [ e ] );
//   var src = [ 1, [ 2, 3 ], [ 'str', null, undefined ] ];
//   var got = _.filter_( null, src, onEach );
//   test.identical( got, [ 1, [ 2, 3 ], [ 'str', null, undefined ] ] );
//   test.false( _.unrollIs( got ) );
//   test.true( _.arrayIs( got ) );
//
//   test.case = 'src is array, filter check equality';
//   var onEach = ( e, i, s ) => e === i;
//   var src = [ 0, 2, 2, [ 'str', null ], undefined ];
//   var got = _.filter_( null, src, onEach );
//   test.identical( got, [ true, false, true, false, false ] );
//   test.notIdentical( got, [ true, false, true, false, false, false ] );
//   test.true( _.arrayIs( got ) );
//
//   test.close( 'onEach is routine' );
//
//   /* - */
//
//   test.case = 'onEach is objectLike - condition, one entry';
//   var callback = { '3' : 9 };
//   var got = _.filter_( null, { 'a' : { '3' : 9 }, 'b' : { '3' : 4 } }, callback );
//   test.identical( got, { 'a' : { '3' : 9 } } );
//
//   test.case = 'onEach is objectLike - condition, a few entry';
//   var callback = { '3' : 9 };
//   var src = { 'a' : { '3' : 9 }, 'b' : { '3' : 4 }, 'c' : { '3' : 9 }, 'd' : { '3' : 9 } };
//   var got = _.filter_( null, src, callback );
//   test.identical( got, { 'a' : { '3' : 9 }, 'c' : { '3' : 9 }, 'd' : { '3' : 9 } } );
//
//   test.case = 'onEach is objectLike - condition, entry nested to next level';
//   var callback = { '3' : 9 };
//   var src = { 'a' : { 'b' : { '3' : 9 } } };
//   var got = _.filter_( null, src, callback );
//   test.identical( got, {} );
//   test.notIdentical( got, { 'a' : { 'b' : { '3' : 9 } } } );
//
//   test.case = 'onEach is objectLike - routine, entry nested to next level';
//   var onEach = ( e ) => true;
//   var callback = { '3' : onEach };
//   var src = { 'a' : { '3' : 9 } };
//   var got = _.filter_( null, src, callback );
//   test.identical( got, {} );
//   test.notIdentical( got, { 'a' : { '3' : 9 } } );
//
//   test.case = 'onEach is objectLike - condition, identical entry';
//   var onEach = ( e ) => true;
//   var callback = { '3' : onEach };
//   var src = { 'a' : { '3' : onEach } };
//   var got = _.filter_( null, src, callback );
//   test.identical( got, { 'a' : { '3' : onEach } } );
//   test.notIdentical( got, {} );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'missed arguments';
//   test.shouldThrowErrorSync( () => _.filter_( null ) );
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.filter_( null, [ 1, 3 ], () => true, 1 ) );
//
//   test.case = 'onEach is not routine';
//   test.shouldThrowErrorSync( () => _.filter_( null, [ 1, 3 ], 'callback' ) );
// }

function entityFilterCountables( test )
{
  const makerChecker =
  [
    { maker : 'long', checker : 'long' },
    { maker : 'array', checker : 'array' },
    { maker : 'unroll', checker : 'unroll' },
    { maker : 'countable', checker : 'countable' },
    { maker : 'vector', checker : 'vector' },
    { maker : 'argumentsArray', checker : 'array' },

    // { maker : 'f32', checker : 'f32' }, /* Dmytro : does not work */
    // { maker : 'set', checker : 'set' }, /* Dmytro : does not work */
  ];

  _.each( makerChecker, ( env ) =>
  {
    env.inplace = false;
    run( env );
  });

  _.each( makerChecker, ( env ) =>
  {
    env.inplace = true;
    if( env.maker === 'argumentsArray' )
    env.checker = 'argumentsArray';
    run( env );
  });

  /* */

  function run( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    test.case = 'empty container';
    var src = _[ env.maker ].make( [] );
    var got = _.filter_( env.inplace ? src : null, src, ( e ) => true );
    test.identical( got, _[ env.checker ].make( [] ) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns 1';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.filter_( env.inplace ? src : null, src, ( e ) => 1 );
    test.identical( got, _[ env.checker ].make([ 1, 1, 1 ]) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns element';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.filter_( env.inplace ? src : null, src, ( e ) => e );
    test.identical( got, _[ env.checker ].make([ 1, 2, 3 ]) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns key + 1';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.filter_( env.inplace ? src : null, src, ( e, k ) => k + 1 );
    test.identical( got, _[ env.checker ].make([ 1, 2, 3 ]) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback checks container length/size';
    var src = _[ env.maker ].make([ 1, 2, 3 ]);
    var got = _.filter_( env.inplace ? src : null, src, ( e, k, c ) => c.length || c.size );
    test.identical( got, _[ env.checker ].make([ 3, 3, 3 ]) );
    test.identical( src === got, env.inplace );

    if( env.maker === 'argumentsArray' && !env.inplace )
    {
      test.case = 'filled container, callback returns undefined';
      var src = _[ env.maker ].make([ 1, 2, 3 ]);
      var got = _.filter_( env.inplace ? src : null, src, ( e ) => undefined );
      test.identical( got, _[ env.checker ].make( [] ) );
      test.identical( src === got, env.inplace );
    }

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.filter_() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.filter_( [] ) );
  test.shouldThrowErrorSync( () => _.filter_( null, [] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.filter_( null, [], ( e ) => e, 'extra' ) );

  test.case = 'wrong type of callback';
  test.shouldThrowErrorSync( () => _.filter_( null, [], 'wrong' ) );
}

//

function entityFilterObjects( test )
{
  const namespaces =
  [
    'props',
    'map',
    'aux',
    'object',
    'container',
  ];

  _.each( namespaces, ( namespace ) =>
  {
    const env = { namespace };
    env.inplace = false;
    run( env );
  });

  _.each( namespaces, ( namespace ) =>
  {
    const env = { namespace };
    env.inplace = true;
    run( env );
  });

  /* */

  function run( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    test.case = 'empty container';
    var src = _[ env.namespace ].make({});
    var got = _.filter_( env.inplace ? src : null, src, ( e ) => true );
    test.identical( got, {} );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns 1';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.filter_( env.inplace ? src : null, src, ( e ) => 1 );
    test.identical( got, { a : 1, b : 1, c : 1 } );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns element';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.filter_( env.inplace ? src : null, src, ( e ) => e );
    test.identical( got, { a : 1, b : 2, c : 3 } );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns key + 1';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.filter_( env.inplace ? src : null, src, ( e, k ) => k + 1 );
    test.identical( got, { a : 'a1', b : 'b1', c : 'c1' } );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback checks container';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.filter_( env.inplace ? src : null, src, ( e, k, c ) => c.c );
    test.identical( got, _[ env.namespace ].make({ a : 3, b : 3, c : 3 }) );
    test.identical( src === got, env.inplace );

    test.case = 'filled container, callback returns undefined';
    var src = _[ env.namespace ].make({ a : 1, b : 2, c : 3 });
    var got = _.filter_( env.inplace ? src : null, src, ( e ) => undefined );
    test.identical( got, {} );
    test.identical( src === got, env.inplace );

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }
}

// //
//
// function entityFilterWithoutDst_( test )
// {
//   test.open( 'onEach returns element' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, _.unroll.make( [ 1, 3, 'str' ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 1, 2, 3, 4, 'str' ] );
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, _.argumentsArray.make( [ 1, 2, 3, 4, 'str' ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new U8x( [ 1, 2, 3, 4, 5 ] );
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, new U8x( [ 1, 2, 3, 4, 5 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : undefined, 'c' : 'str', 'd' : undefined };
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, { 'a' : 1, 'c' : 'str' } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = undefined;
//   src.c = undefined;
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, { 'a' : 1 } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, 'str' );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, new Set() );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.filter_( src, ( e ) => e );
//   test.identical( got, new BufferRaw() );
//
//   test.close( 'onEach returns element' );
//
//   /* - */
//
//   test.open( 'onEach returns key' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new I16x( [ 1, 2, 3, 4, 5 ] );
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, new I16x( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : undefined, 'c' : 'str', 'd' : undefined };
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c', 'd' : 'd' } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = undefined;
//   src.c = undefined;
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.filter_( src, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.close( 'onEach returns key' );
//
//   /* - */
//
//   test.open( 'onEach checks container' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, _.argumentsArray.make( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new F64x( [ 1, 2, 3, 4, 5 ] );
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, new F64x( [ 0, 1, 2, 3, 4 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : undefined, 'c' : 'str', 'd' : undefined };
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c', 'd' : 'd' } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = undefined;
//   src.c = undefined;
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, { 'a' : 'a', 'b' : 'b', 'c' : 'c' } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'str' );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, new Set() );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.filter_( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, new BufferRaw() );
//
//   test.close( 'onEach checks container' );
//
//   /* - */
//
//   test.open( 'onEach returns undefined' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, _.unroll.make( [] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : undefined, 'c' : 'str', 'd' : undefined };
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = undefined;
//   src.c = undefined;
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.filter_( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.close( 'onEach returns undefined' );
//
//   /* - */
//
//   test.open( 'onEach is map like' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ { 'a' : { 'a' : 1 } }, { 'a' : 1 }, 3, undefined, { 'a' : { 'a' : { 'a' : 1 } } } ] );
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, _.unroll.make( [ { 'a' : 1 } ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ { 'a' : 1 }, { 'a' : 1 }, { 'a' : 1 }, { 'a' : 1 }, { 'a' : 1 } ] );
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, _.argumentsArray.make( [ { 'a' : 1 }, { 'a' : 1 }, { 'a' : 1 }, { 'a' : 1 }, { 'a' : 1 } ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : { 'a' : 1 }, 'b' : { 'a' : { 'a' : 1 } }, 'c' : 'str', 'd' : undefined };
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, { 'a' : { 'a' : 1 } } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = { 'a' : 1 };
//   src.b = { 'a' : { 'a' : 1 } };
//   src.c = undefined;
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, { 'a' : { 'a' : 1 } } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.filter_( src, { 'a' : 1 } );
//   test.identical( got, undefined );
//
//   test.close( 'onEach is map like' );
//
//   /* - */
//
//   test.open( 'onEach returns unroll' );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, [] );
//   test.true( got === src );
//
//   test.case = 'unroll';
//   var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, _.unroll.make( [ 1, 1, undefined, undefined, 3, 3, undefined, undefined, 'str', 'str' ] ) );
//   test.true( got === src );
//   test.true( _.unrollIs( got ) );
//
//   test.case = 'argumentsArray';
//   var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] ) );
//   test.true( got === src );
//
//   test.case = 'BufferTyped';
//   var src = new I16x( [ 1, 2, 3, 4, 5 ] );
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, new I16x( [ 1, 2, 3, 4, 5 ] ) );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'empty map';
//   var src = {};
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = { 'a' : 1, 'b' : undefined, 'c' : 'str', 'd' : undefined };
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, { 'a' : [ 1, 1 ], 'b' : [ undefined, undefined ], 'c' : [ 'str', 'str' ], 'd' : [ undefined, undefined ] } );
//   test.true( got === src );
//
//   test.case = 'empty pure map';
//   var src = Object.create( null );
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, {} );
//   test.true( got === src );
//
//   test.case = 'map';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = undefined;
//   src.c = undefined;
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, { 'a' : [ 1, 1 ], 'b' : [ undefined, undefined ], 'c' : [ undefined, undefined ] } );
//   test.true( got === src );
//
//   /* */
//
//   test.case = 'null';
//   var src = null;
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, [ null, null ] );
//
//   test.case = 'string';
//   var src = 'str';
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, [ 'str', 'str' ] );
//
//   test.case = 'Set';
//   var src = new Set();
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, [ new Set(), new Set() ] );
//
//   test.case = 'BufferRaw';
//   var src = new BufferRaw();
//   var got = _.container.filter_( src, ( e ) => _.unroll.make( [ e, e ] ) );
//   test.identical( got, [ new BufferRaw(), new BufferRaw() ] );
//
//   test.close( 'onEach returns unroll' );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.container.filter_() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.container.filter_( 'str' ) );
//   test.shouldThrowErrorSync( () => _.container.filter_( [ 1, 2 ] ) );
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.container.filter_( [ 1, 3 ], [ 1, 2 ], ( e ) => e, 'extra' ) );
//   test.shouldThrowErrorSync( () => _.container.filter_( { 'a' : 1 }, { 'b' : 2 }, ( e ) => e, 'extra' ) );
//
//   test.case = 'src is not resizable long, cannot delete element';
//   test.shouldThrowErrorSync( () => _.container.filter_( _.argumentsArray.make( [ undefined, undefined ], ( e ) => e ) ) );
//
//   test.case = 'wrong type of onEach';
//   test.shouldThrowErrorSync( () => _.container.filter_( [ 1, 2 ], [] ) );
//   test.shouldThrowErrorSync( () => _.container.filter_( null, [ 1, 2 ], [] ) );
//
//   test.case = 'wrong dst container';
//   test.shouldThrowErrorSync( () => _.container.filter_( {}, [ 1, 2 ], ( e ) => e ) );
//   test.shouldThrowErrorSync( () => _.container.filter_( [], { 'a' : 1, 'b' : 2 }, ( e ) => e ) );
//   test.shouldThrowErrorSync( () => _.container.filter_( new BufferRaw(), { 'a' : 1, 'b' : 2 }, ( e ) => e ) );
//
//   test.case = 'dst is not resizable long';
//   test.shouldThrowErrorSync( () => _.container.filter_( new F32x(), { 'a' : 1, 'b' : 2 }, ( e ) => e ) );
//
//   test.case = 'not map like, not long like, cannot extend map';
//   test.shouldThrowErrorSync( () => _.container.filter_( {}, 1, ( e ) => e ) );
// }

/* qqq for Dmytro : bad coverage

for example :
commands = _.filter_( null, 'string', ( command ) => xxx )

*/

function entityFilterDstNull_( test )
{
  test.open( 'onEach returns element' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'string';
  var src = 'abc';
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, 'abc' );
  test.true( got === src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, _.unroll.make( [ 1, 3, 'str' ] ) );
  test.true( got !== src );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, [ 1, 3, 'str' ] );
  test.true( got !== src );

  test.case = 'BufferTyped';
  var src = new U8x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, new U8x( [] ) );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, { a : 1, c : 'str' } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, { a : 1 } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, null );

  test.case = 'string';
  var src = 'str';
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, 'str' );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, new Set() );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.filter_( null, src, ( e ) => e );
  test.identical( got, new BufferRaw() );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
  test.true( got !== src );
  test.true( _.unrollIs( got ) );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, [ 0, 1, 2, 3, 4 ] );
  test.true( got !== src );

  test.case = 'BufferTyped';
  var src = new I16x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, new I16x( [] ) );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, { a : 'a', b : 'b', c : 'c', d : 'd' } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, { a : 'a', b : 'b', c : 'c' } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, null );

  test.case = 'string';
  var src = 'str';
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, null );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, null );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.filter_( null, src, ( e, k ) => k );
  test.identical( got, null );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach checks container' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, _.unroll.make( [ 0, 1, 2, 3, 4 ] ) );
  test.true( got !== src );
  test.true( _.unrollIs( got ) );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 0, 1, 2, 3, 4 ] );
  test.true( got !== src );

  test.case = 'BufferTyped';
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new F64x( [] ) );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { a : 'a', b : 'b', c : 'c', d : 'd' } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { a : 'a', b : 'b', c : 'c' } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, null );

  test.case = 'string';
  var src = 'str';
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, 'str' );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new Set() );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.filter_( null, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new BufferRaw() );

  test.close( 'onEach checks container' );

  /* - */

  test.open( 'onEach returns undefined' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, _.unroll.make( [] ) );
  test.true( got !== src );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, [] );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, {} );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, undefined );

  test.case = 'string';
  var src = 'str';
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, null );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, null );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.filter_( null, src, ( e ) => undefined );
  test.identical( got, null );

  test.close( 'onEach returns undefined' );

  /* - */

  test.open( 'onEach is map like' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ { a : { a : 1 } }, { a : 1 }, 3, undefined, { a : { a : { a : 1 } } } ] );
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, _.unroll.make( [ { a : 1 } ] ) );
  test.true( got !== src );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, [] );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { a : { a : 1 }, b : { a : { a : 1 } }, c : 'str', d : undefined };
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, { a : { a : 1 } } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = { a : 1 };
  src.b = { a : { a : 1 } };
  src.c = undefined;
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, { a : { a : 1 } } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, undefined );

  test.case = 'string';
  var src = 'str';
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, null );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, null );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.filter_( null, src, { a : 1 } );
  test.identical( got, null );

  test.close( 'onEach is map like' );

  /* - */

  test.open( 'onEach returns unroll' );

  test.case = 'empty array';
  var src = [];
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [] );
  test.true( got !== src );

  test.case = 'unroll';
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, _.unroll.make( [ 1, 1, undefined, undefined, 3, 3, undefined, undefined, 'str', 'str' ] ) );
  test.true( got !== src );
  test.true( _.unrollIs( got ) );

  test.case = 'argumentsArray';
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ 1, 1, undefined, undefined, 3, 3, undefined, undefined, 'str', 'str' ] );
  test.true( got !== src );

  test.case = 'BufferTyped';
  var src = new I16x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, new I16x( [] ) );
  test.true( got !== src );

  /* */

  test.case = 'empty map';
  var src = {};
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, { a : [ 1, 1 ], b : [ undefined, undefined ], c : [ 'str', 'str' ], d : [ undefined, undefined ] } );
  test.true( got !== src );

  test.case = 'empty pure map';
  var src = Object.create( null );
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, {} );
  test.true( got !== src );

  test.case = 'map';
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, { a : [ 1, 1 ], b : [ undefined, undefined ], c : [ undefined, undefined ] } );
  test.true( got !== src );

  /* */

  test.case = 'null';
  var src = null;
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ null, null ] );

  test.case = 'string';
  var src = 'str';
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ 'str', 'str' ] );

  test.case = 'Set';
  var src = new Set();
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ new Set(), new Set() ] );

  test.case = 'BufferRaw';
  var src = new BufferRaw();
  var got = _.container.filter_( null, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ new BufferRaw(), new BufferRaw() ] );

  test.close( 'onEach returns unroll' );
}

//

function entityFilterDstNotNull_( test )
{
  test.open( 'onEach returns element' );

  test.case = 'empty array';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = [];
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'unroll';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, [ 1, 3, 'str', 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'argumentsArray';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, [ 1, 3, 'str', 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'BufferTyped';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = new U8x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, [ 1, 2, 3, 4, 5, 7 ] );
  test.true( got === dst );

  /* */

  test.case = 'empty map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = {};
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, { a : 1, b : undefined, c : 'str', e : 1 } );
  test.true( got === dst );

  test.case = 'empty pure map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, { a : 1, b : undefined, e : 1 } );
  test.true( got === dst );

  /* */

  test.case = 'null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, [ 1, 2, null ] );
  test.true( got === dst );

  test.case = 'string';
  var dst = { a : 1 };
  var src = 'str';
  var got = _.container.filter_( dst, src, ( e ) => [ e ] );
  test.identical( got, { a : 1, 0 : 'str' } );
  test.true( got === dst );

  test.case = 'Set';
  var dst = false;
  var src = new Set();
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, new Set() );

  test.case = 'BufferRaw';
  var dst = _.unroll.make( [] );
  var src = new BufferRaw();
  var got = _.container.filter_( dst, src, ( e ) => e );
  test.identical( got, [ new BufferRaw() ] );
  test.true( got === dst );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'empty array';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = [];
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'unroll';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, [ 0, 1, 2, 3, 4, 7 ] );
  test.true( got === dst );

  test.case = 'argumentsArray';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, [ 0, 1, 2, 3, 4, 7 ] );
  test.true( got === dst );

  test.case = 'BufferTyped';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, [ 0, 1, 2, 3, 4, 7 ] );
  test.true( got === dst );

  /* */

  test.case = 'empty map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = {};
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, { a : 'a', b : 'b', c : 'c', d : 'd', e : 1 } );
  test.true( got === dst );

  test.case = 'empty pure map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, { a : 'a', b : 'b', c : 'c', e : 1 } );
  test.true( got === dst );

  /* */

  test.case = 'null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, [ 1, 2, null ] );
  test.true( got === dst );

  test.case = 'string';
  var dst = { a : 1 };
  var src = 'str';
  var got = _.container.filter_( dst, src, ( e, k ) => [ k ] );
  test.identical( got, { a : 1, 0 : null } );
  test.true( got === dst );

  test.case = 'Set';
  var dst = false;
  var src = new Set();
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, null );

  test.case = 'BufferRaw';
  var dst = _.unroll.make( [] );
  var src = new BufferRaw();
  var got = _.container.filter_( dst, src, ( e, k ) => k );
  test.identical( got, [ null ] );
  test.true( got === dst );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach checks container' );

  test.case = 'empty array';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = [];
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'unroll';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 0, 1, 2, 3, 4, 7 ] );
  test.true( got === dst );

  test.case = 'argumentsArray';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 0, 1, 2, 3, 4, 7 ] );
  test.true( got === dst );

  test.case = 'BufferTyped';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 0, 1, 2, 3, 4, 7 ] );
  test.true( got === dst );

  /* */

  test.case = 'empty map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = {};
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { a : 'a', b : 'b', c : 'c', d : 'd', e : 1 } );
  test.true( got === dst );

  test.case = 'empty pure map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, { a : 'a', b : 'b', c : 'c', e : 1 } );
  test.true( got === dst );

  /* */

  test.case = 'null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ 1, 2, null ] );
  test.true( got === dst );

  test.case = 'string';
  var dst = { a : 1 };
  var src = 'str';
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? [ k ] : [ e ] );
  test.identical( got, { a : 1, 0 : 'str' } );
  test.true( got === dst );

  test.case = 'Set';
  var dst = false;
  var src = new Set();
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, new Set() );

  test.case = 'BufferRaw';
  var dst = _.unroll.make( [] );
  var src = new BufferRaw();
  var got = _.container.filter_( dst, src, ( e, k, c ) => c ? k : e );
  test.identical( got, [ new BufferRaw() ] );
  test.true( got === dst );

  test.close( 'onEach checks container' );

  /* - */

  test.open( 'onEach returns undefined' );

  test.case = 'empty array';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = [];
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'unroll';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'argumentsArray';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'BufferTyped';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = new F64x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  /* */

  test.case = 'empty map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = {};
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'empty pure map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  /* */

  test.case = 'null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, [ 1, 2 ] );
  test.true( got === dst );

  test.case = 'string';
  var dst = { a : 1 };
  var src = 'str';
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, { a : 1 } );
  test.true( got === dst );

  test.case = 'Set';
  var dst = false;
  var src = new Set();
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, false );

  test.case = 'BufferRaw';
  var dst = _.unroll.make( [] );
  var src = new BufferRaw();
  var got = _.container.filter_( dst, src, ( e ) => undefined );
  test.identical( got, [] );
  test.true( got === dst );

  test.close( 'onEach returns undefined' );

  /* - */

  test.open( 'onEach is map like' );

  test.case = 'empty array';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = [];
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'unroll';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.unroll.make( [ { a : { a : 1 } }, { a : 1 }, 3, undefined, { a : { a : { a : 1 } } } ] );
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, _.unroll.make( [ { a : 1 }, 7, 7, 7, 7, 7 ] ) );
  test.true( got === dst );

  test.case = 'argumentsArray';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.argumentsArray.make( [ 1, { a : 1 }, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, [ { a : 1 }, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'BufferTyped';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = new I32x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );


  /* */

  test.case = 'empty map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = {};
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = { a : { a : 1 }, b : { a : { a : 1 } }, c : 'str', d : undefined };
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, { a : { a : 1 }, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'empty pure map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  src.a = { a : 1 };
  src.b = { a : { a : 1 } };
  src.c = undefined;
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, { a : { a : 1 }, b : undefined, e : 1 } );
  test.true( got === dst );

  /* */

  test.case = 'null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, [ 1, 2 ] );
  test.true( got === dst );

  test.case = 'string';
  var dst = { a : 1 };
  var src = 'str';
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, { a : 1 } );
  test.true( got === dst );

  test.case = 'Set';
  var dst = false;
  var src = new Set();
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, false );

  test.case = 'BufferRaw';
  var dst = _.unroll.make( [] );
  var src = new BufferRaw();
  var got = _.container.filter_( dst, src, { a : 1 } );
  test.identical( got, [] );
  test.true( got === dst );


  test.close( 'onEach is map like' );

  /* - */

  test.open( 'onEach returns unroll' );

  test.case = 'empty array';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = [];
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ 7, 7, 7, 7, 7, 7 ] );
  test.true( got === dst );

  test.case = 'unroll';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.unroll.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, _.unroll.make( [ 1, 1, undefined, undefined, 3, 3, undefined, undefined, 'str', 'str', 7 ] ) );
  test.true( got === dst );

  test.case = 'argumentsArray';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = _.argumentsArray.make( [ 1, undefined, 3, undefined, 'str' ] );
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ 1, 1, undefined, undefined, 3, 3, undefined, undefined, 'str', 'str', 7 ] );
  test.true( got === dst );

  test.case = 'BufferTyped';
  var dst = [ 7, 7, 7, 7, 7, 7 ];
  var src = new I16x( [ 1, 2, 3, 4, 5 ] );
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 7 ] );
  test.true( got === dst );

  /* */

  test.case = 'empty map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = {};
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = { a : 1, b : undefined, c : 'str', d : undefined };
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, { a : [ 1, 1 ], b : [ undefined, undefined ], c : [ 'str', 'str' ], d : [ undefined, undefined ], e : 1 } );
  test.true( got === dst );

  test.case = 'empty pure map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, { a : undefined, b : undefined, e : 1 } );
  test.true( got === dst );

  test.case = 'map';
  var dst = { a : undefined, b : undefined, e : 1 };
  var src = Object.create( null );
  src.a = 1;
  src.b = undefined;
  src.c = undefined;
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, { a : [ 1, 1 ], b : [ undefined, undefined ], c : [ undefined, undefined ], e : 1 } );
  test.true( got === dst );

  /* */

  test.case = 'null';
  var dst = [ 1, 2 ];
  var src = null;
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ 1, 2, [ null, null ] ] );
  test.true( got === dst );

  test.case = 'string';
  var dst = { a : 1 };
  var src = 'str';
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, { a : 1, 0 : 'str', 1 : 'str' } );
  test.true( got === dst );

  test.case = 'Set';
  var dst = false;
  var src = new Set();
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ new Set(), new Set() ] );

  test.case = 'BufferRaw';
  var dst = _.unroll.make( [] );
  var src = new BufferRaw();
  var got = _.container.filter_( dst, src, ( e ) => _.unroll.make( [ e, e ] ) );
  test.identical( got, [ [ new BufferRaw(), new BufferRaw() ] ] );
  test.true( got === dst );


  test.close( 'onEach returns unroll' );
}

//

function entityFilterDeep( test )
{
  test.open( 'onEach is routine' );

  test.case = 'array';
  var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
  var got = _.entityFilterDeep( [ 9, -16, 25, 36, -49 ], callback );
  test.identical( got, [ 3, 5, 6 ] );
  test.notIdentical( got, [ 3, 4, 5, 6, 7 ] );

  test.case = 'unroll';
  var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
  var src = _.unroll.make( [ 9, _.unroll.make( [ -16, 25, _.unroll.from( [ 36, -49 ] ) ] ) ] );
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, [ 3, 5, 6 ] );
  test.notIdentical( got, [ 3, 4, 5, 6, 7 ] );
  test.false( _.unrollIs( got) );

  test.case = 'argumentsArray';
  var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
  var src = _.argumentsArray.make( [ 9, -16, 25, 36, -49 ] );
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, [ 3, 5, 6 ] );

  test.case = 'BufferTyped to Array';
  var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
  var src = new F32x( [ 9, -16, 25, 36, -49 ] );
  var src = Array.from( src );
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, [ 3, 5, 6 ] );
  test.notIdentical( got, [ 3, 4, 5, 6, 7 ] );

  test.case = 'aixiliary';
  var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
  var got = _.entityFilterDeep( { '3' : 9, '4' : 16, '5' : 25, '6' : -36 }, callback );
  test.identical( got, { '3' : 3, '4' : 4, '5' : 5 } );
  test.notIdentical( got, { '3' : 3, '4' : 4, '5' : 5, '6' : 6 } );

  test.case = 'callback in routine';
  var callback = ( v, i, s ) => v < 0 ? undefined : Math.sqrt( v );
  function testFn1()
  {
    return _.entityFilterDeep( arguments, callback );
  }
  var got = testFn1( 9, -16, 25, 36, -49 );
  test.identical( got, [ 3, 5, 6 ] );

  test.case = 'src is array with single element, filter make unrolls';
  var onEach = ( e, i, s ) => _.unroll.make( [ e ] );
  var src = [ 1 ];
  var got = _.entityFilterDeep( src, onEach );
  test.identical( got, [ 1 ] );
  test.false( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  test.case = 'src is array with single element, filter make unrolls';
  var onEach = ( e, i, s ) => _.unroll.make( [ e ] );
  var src = [ 1, 2 ];
  var got = _.entityFilterDeep( src, onEach );
  test.identical( got, [ 1, 2 ] );
  test.false( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  test.case = 'src is array, filter make unrolls';
  var onEach = ( e, i, s ) => _.unroll.make( [ e ] );
  var src = [ 1, [ 2, 3 ], [ 'str', null, undefined ] ];
  var got = _.entityFilterDeep( src, onEach );
  test.identical( got, [ 1, [ 2, 3 ], [ 'str', null, undefined ] ] );
  test.false( _.unrollIs( got ) );
  test.true( _.arrayIs( got ) );

  test.case = 'src is array, filter check equality';
  var onEach = ( e, i, s ) => e === i;
  var src = [ 0, 2, 2, [ 'str', null ], undefined ];
  var got = _.entityFilterDeep( src, onEach );
  test.identical( got, [ true, false, true, false, false ] );
  test.notIdentical( got, [ true, false, true, false, false, false ] );
  test.true( _.arrayIs( got ) );

  test.close( 'onEach is routine' );

  /* - */

  test.case = 'onEach is objectLike - condition, one entry';
  var callback = { '3' : 9 };
  var got = _.entityFilterDeep( { a : { '3' : 9 }, b : { '3' : 4 } }, callback );
  test.identical( got, { a : { '3' : 9 } } );

  test.case = 'onEach is objectLike - condition, a few entry';
  var callback = { '3' : 9 };
  var src = { a : { '3' : 9 }, b : { '4' : 4 }, c : { '3' : 9 }, d : { '3' : 9 } };
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, { a : { '3' : 9 }, c : { '3' : 9 }, d : { '3' : 9 } } );

  test.case = 'onEach is objectLike - condition, entry nested to next levels';
  var callback = { '3' : 9 };
  var src = { a : { a : { b : { c : { '3' : 9, '4' : 6 } } } } };
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, {} );
  test.notIdentical( got, { a : { a : { b : { c : { '3' : 9, '4' : 6 } } } } } );

  test.case = 'onEach is objectLike - routine, entry nested to next level';
  var onEach = ( e ) => true;
  var callback = { '3' : onEach };
  var src = { a : { b : { '3' : 9 } } };
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, { a : { b : { '3' : 9 } } } );
  test.notIdentical( got, {} );

  test.case = 'onEach is objectLike - routine, entry nested to next level';
  var onEach = ( e ) =>
  {
    for( let k in e )
    {
      e[ k ] = e[ k ] + 5;
      if( e[ k ] !== 10 )
      return false;
    }
    return true;
  }
  var callback = { '3' : onEach };
  var src = { a : { b : 5, c : 5, d : 5 } };
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, { a : { b : 5, c : 5, d : 5 } } );
  test.notIdentical( got, {} );

  test.case = 'onEach is objectLike - condition, identical entry';
  var onEach = ( e ) => true;
  var callback = { '3' : onEach };
  var src = { a : { '3' : onEach } };
  var got = _.entityFilterDeep( src, callback );
  test.identical( got, { a : { '3' : onEach } } );
  test.notIdentical( got, {} );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'missed arguments';
  test.shouldThrowErrorSync( () => _.entityFilterDeep() );

  test.case = 'extra argument';
  test.shouldThrowErrorSync( () => _.entityFilterDeep( [ 1, 3 ], () => true, 1 ) );

  test.case = 'onEach is not routine';
  test.shouldThrowErrorSync( () => _.entityFilterDeep( [ 1, 3 ], 'callback' ) );

  test.case = 'src is not arrayLike or aixiliary';
  test.shouldThrowErrorSync( () => _.entityFilterDeep( undefined, ( e ) => e ) );
}

// //
//
// function first( test )
// {
//   test.open( 'onEach returns element' );
//
//   test.case = 'empty array';
//   var got = _.first( [], ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'argumentsArray with undefined';
//   var got = _.first( _.argumentsArray.make( [ undefined, undefined, undefined ] ), ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'unroll has defined elements';
//   var got = _.first( _.unroll.make( [ undefined, 'str', 1 ] ), ( e ) => e );
//   test.identical( got, 'str' );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.first( new U8x( 10 ), ( e ) => e );
//   test.identical( got, 0 );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.first( {}, ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.first( { a : undefined, b : undefined, c : undefined }, ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var got = _.first( { a : undefined, b : 'str', c : 1 }, ( e ) => e );
//   test.identical( got, 'str' );
//
//   test.case = 'empty pure map';
//   var got = _.first( Object.create( null ), ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.first( src, ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = 'str';
//   var got = _.first( src, ( e ) => e );
//   test.identical( got, 'str' );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.first( null, ( e ) => e );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var got = _.first( 'str', ( e ) => e );
//   test.identical( got, 'str' );
//
//   test.case = 'number';
//   var got = _.first( 1, ( e ) => e );
//   test.identical( got, 1 );
//
//   test.case = 'boolean - false';
//   var got = _.first( false, ( e ) => e );
//   test.identical( got, false );
//
//   test.close( 'onEach returns element' );
//
//   /* - */
//
//   test.open( 'onEach returns key' );
//
//   test.case = 'empty unroll';
//   var got = _.first( _.unroll.make( [] ), ( e, k ) => k );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.first( [ undefined, undefined, undefined ], ( e, k ) => k );
//   test.identical( got, 0 );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.first( _.argumentsArray.make( [ undefined, 'str', 1 ] ), ( e, k ) => k );
//   test.identical( got, 0 );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.first( new I32x( 10 ), ( e, k ) => k );
//   test.identical( got, 0 );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.first( {}, ( e, k ) => k );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.first( { a : undefined, b : undefined, c : undefined }, ( e, k ) => k );
//   test.identical( got, 'a' );
//
//   test.case = 'map has defined elements';
//   var got = _.first( { a : undefined, b : 'str', c : 1 }, ( e, k ) => k );
//   test.identical( got, 'a' );
//
//   test.case = 'empty pure map';
//   var got = _.first( Object.create( null ), ( e, k ) => k );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.first( src, ( e, k ) => k );
//   test.identical( got, 'a' );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = 'str';
//   var got = _.first( src, ( e, k ) => k );
//   test.identical( got, 'a' );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.first( null, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var got = _.first( 'str', ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'number';
//   var got = _.first( 1, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'boolean - false';
//   var got = _.first( false, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.close( 'onEach returns key' );
//
//   /* - */
//
//   test.open( 'onEach checks container' );
//
//   test.case = 'empty unroll';
//   var got = _.first( _.unroll.make( [] ), ( e, k, c ) => c ? k : e );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.first( [ undefined, undefined, undefined ], ( e, k, c ) => c ? k : e );
//   test.identical( got, 0 );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.first( _.argumentsArray.make( [ undefined, 'str', 1 ] ), ( e, k, c ) => c ? k : e );
//   test.identical( got, 0 );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.first( new I32x( 10 ), ( e, k, c ) => c ? k : e );
//   test.identical( got, 0 );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.first( {}, ( e, k, c ) => c ? k : e );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.first( { a : undefined, b : undefined, c : undefined }, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'a' );
//
//   test.case = 'map has defined elements';
//   var got = _.first( { a : undefined, b : 'str', c : 1 }, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'a' );
//
//   test.case = 'empty pure map';
//   var got = _.first( Object.create( null ), ( e, k, c ) => c ? k : e );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.first( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'a' );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = 'str';
//   var got = _.first( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'a' );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.first( null, ( e, k, c ) => c ? k : e );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var got = _.first( 'str', ( e, k, c ) => c ? k : e );
//   test.identical( got, 'str' );
//
//   test.case = 'number';
//   var got = _.first( 1, ( e, k, c ) => c ? k : e );
//   test.identical( got, 1 );
//
//   test.case = 'boolean - false';
//   var got = _.first( false, ( e, k, c ) => c ? k : e );
//   test.identical( got, false );
//
//   test.close( 'onEach checks container' );
//
//   /* - */
//
//   test.open( 'onEach returns undefined' );
//
//   test.case = 'empty unroll';
//   var got = _.first( _.unroll.make( [] ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.first( [ undefined, undefined, undefined ], ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.first( _.argumentsArray.make( [ undefined, 'str', 1 ] ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.first( new I32x( 10 ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.first( {}, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.first( { a : undefined, b : undefined, c : undefined }, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var got = _.first( { a : undefined, b : 'str', c : 1 }, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'empty pure map';
//   var got = _.first( Object.create( null ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.first( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = 'str';
//   var got = _.first( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.first( null, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'string';
//   var got = _.first( 'str', ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'number';
//   var got = _.first( 1, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'boolean - false';
//   var got = _.first( false, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.close( 'onEach returns undefined' );
//
//   /* - */
//
//   test.open( 'onEach is objectLike' );
//
//   test.case = 'empty unroll';
//   var got = _.first( _.unroll.make( [] ), { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.first( [ undefined, undefined, undefined ], { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.first( _.argumentsArray.make( [ { b : 1 }, 'str', { a : 1 } ] ), { a : 1 } );
//   test.identical( got, { a : 1 } );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.first( new I32x( 10 ), { a : 1 } );
//   test.identical( got, undefined );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.first( {}, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.first( { a : undefined, b : undefined, c : undefined }, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var got = _.first( { a : undefined, b : 'str', c : { a : 1 } }, { a : 1 } );
//   test.identical( got, { a : 1 } );
//
//   test.case = 'empty pure map';
//   var got = _.first( Object.create( null ), { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.first( src, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = { a : 1 };
//   var got = _.first( src, { a : 1 } );
//   test.identical( got, { a : 1 } );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.first( null, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'string';
//   var got = _.first( 'str', { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'number';
//   var got = _.first( 1, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'boolean - false';
//   var got = _.first( false, { a : 1 } );
//   test.identical( got, undefined );
//
//
//   test.close( 'onEach is objectLike' );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.first() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.first( 'str' ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.first( 'str', ( e ) => e, 'extra' ) );
//
//   test.case = 'onEach is not a routine';
//   test.shouldThrowErrorSync( () => _.first( [ 1, 2, 3 ], 1 ) );
// }
//
// //
//
// function last( test )
// {
//   test.open( 'onEach returns element' );
//
//   test.case = 'empty array';
//   var got = _.last( [], ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'argumentsArray with undefined';
//   var got = _.last( _.argumentsArray.make( [ undefined, undefined, undefined ] ), ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'unroll has defined elements';
//   var got = _.last( _.unroll.make( [ undefined, 'str', 1 ] ), ( e ) => e );
//   test.identical( got, 1 );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.last( new U8x( 10 ), ( e ) => e );
//   test.identical( got, 0 );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.last( {}, ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.last( { a : undefined, b : undefined, c : undefined }, ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var got = _.last( { a : undefined, b : 'str', c : 1 }, ( e ) => e );
//   test.identical( got, 1 );
//
//   test.case = 'empty pure map';
//   var got = _.last( Object.create( null ), ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.last( src, ( e ) => e );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = 1;
//   src.b = 'str';
//   var got = _.last( src, ( e ) => e );
//   test.identical( got, 'str' );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.last( null, ( e ) => e );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var got = _.last( 'str', ( e ) => e );
//   test.identical( got, 'str' );
//
//   test.case = 'number';
//   var got = _.last( 1, ( e ) => e );
//   test.identical( got, 1 );
//
//   test.case = 'boolean - false';
//   var got = _.last( false, ( e ) => e );
//   test.identical( got, false );
//
//   test.close( 'onEach returns element' );
//
//   /* - */
//
//   test.open( 'onEach returns key' );
//
//   test.case = 'empty unroll';
//   var got = _.last( _.unroll.make( [] ), ( e, k ) => k );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.last( [ undefined, undefined, undefined ], ( e, k ) => k );
//   test.identical( got, 2 );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.last( _.argumentsArray.make( [ undefined, 'str', 1 ] ), ( e, k ) => k );
//   test.identical( got, 2 );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.last( new I32x( 10 ), ( e, k ) => k );
//   test.identical( got, 9 );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.last( {}, ( e, k ) => k );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.last( { a : undefined, b : undefined, c : undefined }, ( e, k ) => k );
//   test.identical( got, 'c' );
//
//   test.case = 'map has defined elements';
//   var got = _.last( { a : undefined, b : 'str', c : 1 }, ( e, k ) => k );
//   test.identical( got, 'c' );
//
//   test.case = 'empty pure map';
//   var got = _.last( Object.create( null ), ( e, k ) => k );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.last( src, ( e, k ) => k );
//   test.identical( got, 'b' );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = 'str';
//   var got = _.last( src, ( e, k ) => k );
//   test.identical( got, 'b' );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.last( null, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var got = _.last( 'str', ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'number';
//   var got = _.last( 1, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.case = 'boolean - false';
//   var got = _.last( false, ( e, k ) => k );
//   test.identical( got, null );
//
//   test.close( 'onEach returns key' );
//
//   /* - */
//
//   test.open( 'onEach checks container' );
//
//   test.case = 'empty unroll';
//   var got = _.last( _.unroll.make( [] ), ( e, k, c ) => c ? k : e );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.last( [ undefined, undefined, undefined ], ( e, k, c ) => c ? k : e );
//   test.identical( got, 2 );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.last( _.argumentsArray.make( [ undefined, 'str', 1 ] ), ( e, k, c ) => c ? k : e );
//   test.identical( got, 2 );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.last( new I32x( 10 ), ( e, k, c ) => c ? k : e );
//   test.identical( got, 9 );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.last( {}, ( e, k, c ) => c ? k : e );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.last( { a : undefined, b : undefined, c : undefined }, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'c' );
//
//   test.case = 'map has defined elements';
//   var got = _.last( { a : undefined, b : 'str', c : 1 }, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'c' );
//
//   test.case = 'empty pure map';
//   var got = _.last( Object.create( null ), ( e, k, c ) => c ? k : e );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.last( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'b' );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = 'str';
//   var got = _.last( src, ( e, k, c ) => c ? k : e );
//   test.identical( got, 'b' );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.last( null, ( e, k, c ) => c ? k : e );
//   test.identical( got, null );
//
//   test.case = 'string';
//   var got = _.last( 'str', ( e, k, c ) => c ? k : e );
//   test.identical( got, 'str' );
//
//   test.case = 'number';
//   var got = _.last( 1, ( e, k, c ) => c ? k : e );
//   test.identical( got, 1 );
//
//   test.case = 'boolean - false';
//   var got = _.last( false, ( e, k, c ) => c ? k : e );
//   test.identical( got, false );
//
//   test.close( 'onEach checks container' );
//
//   /* - */
//
//   test.open( 'onEach returns undefined' );
//
//   test.case = 'empty unroll';
//   var got = _.last( _.unroll.make( [] ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.last( [ undefined, undefined, undefined ], ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.last( _.argumentsArray.make( [ undefined, 'str', 1 ] ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.last( new I32x( 10 ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.last( {}, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.last( { a : undefined, b : undefined, c : undefined }, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var got = _.last( { a : undefined, b : 'str', c : 1 }, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'empty pure map';
//   var got = _.last( Object.create( null ), ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.last( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = 'str';
//   var got = _.last( src, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.last( null, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'string';
//   var got = _.last( 'str', ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'number';
//   var got = _.last( 1, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.case = 'boolean - false';
//   var got = _.last( false, ( e ) => undefined );
//   test.identical( got, undefined );
//
//   test.close( 'onEach returns undefined' );
//
//   /* - */
//
//   test.open( 'onEach is objectLike' );
//
//   test.case = 'empty unroll';
//   var got = _.last( _.unroll.make( [] ), { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'array with undefined';
//   var got = _.last( [ undefined, undefined, undefined ], { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'argumentsArray has defined elements';
//   var got = _.last( _.argumentsArray.make( [ { a : 1 }, 'str', { b : 1 } ] ), { a : 1 } );
//   test.identical( got, { a : 1 } );
//
//   test.case = 'BufferTyped with zeros';
//   var got = _.last( new I32x( 10 ), { a : 1 } );
//   test.identical( got, undefined );
//
//   /* */
//
//   test.case = 'empty map';
//   var got = _.last( {}, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'map with undefined';
//   var got = _.last( { a : undefined, b : undefined, c : undefined }, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var got = _.last( { a : { a : 1 }, b : undefined, c : { b : 1 } }, { a : 1 } );
//   test.identical( got, { a : 1 } );
//
//   test.case = 'empty pure map';
//   var got = _.last( Object.create( null ), { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'pure map with undefined';
//   var src = Object.create( null );
//   src.a = undefined;
//   src.b = undefined;
//   var got = _.last( src, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'map has defined elements';
//   var src = Object.create( null );
//   src.a = { a : 1 };
//   src.b = undefined;
//   var got = _.last( src, { a : 1 } );
//   test.identical( got, { a : 1 } );
//
//   /* */
//
//   test.case = 'null';
//   var got = _.last( null, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'string';
//   var got = _.last( 'str', { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'number';
//   var got = _.last( 1, { a : 1 } );
//   test.identical( got, undefined );
//
//   test.case = 'boolean - false';
//   var got = _.last( false, { a : 1 } );
//   test.identical( got, undefined );
//
//
//   test.close( 'onEach is objectLike' );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.last() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.last( 'str' ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.last( 'str', ( e ) => e, 'extra' ) );
//
//   test.case = 'onEach is not a routine';
//   test.shouldThrowErrorSync( () => _.last( [ 1, 2, 3 ], 1 ) );
// }

//

function index( test )
{
  test.open( 'without onEach' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.index( src );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.index( src );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.index( src );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.index( src );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.index( src );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.index( src );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.index( src );
  var exp = {};
  test.identical( got, exp );

  test.close( 'without onEach' );

  /* - */

  test.open( 'onEach returns undefined' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.index( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.index( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.index( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.index( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.index( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.index( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.index( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns undefined' );

  /* - */

  test.open( 'onEach returns element' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.index( src, ( e, k ) => e );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.index( src, ( e, k ) => e );
  var exp = { a : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.index( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.index( src, ( e, k ) => e );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.index( src, ( e, k ) => e );
  var exp = { a : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.index( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.index( src, ( e, k ) => e );
  var exp = { src : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.index( src, ( e, k ) => k );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.index( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.index( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.index( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.index( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.index( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.index( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach returns map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.index( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.index( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns map' );

  /* - */

  test.open( 'onEach returns unroll' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.index( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.index( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : 'a'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.index( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : 'c'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.index( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.index( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : 'a'
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.index( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : 'c'
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.index( src, onEach );
  var exp = { undefined2 : 'src', x : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns unroll' );

  /* - */

  test.open( 'onEach returns unroll with map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.index( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.index( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.index( src, onEach );
  var exp = { '[object Object]' : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns unroll with map' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.index( src, '*/f1' );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'aa.f1', f2 : 'aa.f2' }, c : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  var got = _.index( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'bb.f1', f2 : 'bb.f2' }, c : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  var got = _.index( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' }, 'bb.f1' : { f1 : 'bb.f1', f2 : 'bb.f2' }, 'cc.f1' : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.index( src, '*/f1' );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' } ];
  var got = _.index( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'a.f2' } };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.index( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'a.f2' }, 'b.f1' : { f1 : 'b.f1', f2 : 'b.f2' }, 'c.f1' : { f1 : 'c.f1', f2 : 'c.f2' } };
  test.identical( got, exp );

  test.case = 'src - str';
  var src = 'src';
  var got = _.index( src, '*/src' );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function indexExtending( test )
{
  test.open( 'without onEach' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexExtending( src );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexExtending( src );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexExtending( src );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexExtending( src );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexExtending( src );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexExtending( src );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexExtending( src );
  var exp = { src : undefined };
  test.identical( got, exp );

  test.close( 'without onEach' );

  /* - */

  test.open( 'onEach returns undefined' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexExtending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexExtending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexExtending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexExtending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexExtending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexExtending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexExtending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns undefined' );

  /* - */

  test.open( 'onEach returns element' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexExtending( src, ( e, k ) => e );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexExtending( src, ( e, k ) => e );
  var exp = { a : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexExtending( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexExtending( src, ( e, k ) => e );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexExtending( src, ( e, k ) => e );
  var exp = { a : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexExtending( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexExtending( src, ( e, k ) => e );
  var exp = { src : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexExtending( src, ( e, k ) => k );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexExtending( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexExtending( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexExtending( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexExtending( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexExtending( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexExtending( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach returns map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexExtending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : 'a'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : 'c'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexExtending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : 'a'
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : 'c'
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexExtending( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns map' );

  /* - */

  test.open( 'onEach returns unroll' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexExtending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : 'a'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : 'c'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexExtending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : 'a'
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : 'c'
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexExtending( src, onEach );
  var exp = { undefined2 : 'src', x : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns unroll' );

  /* - */

  test.open( 'onEach returns unroll with map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexExtending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a2',
    'bk2' : 'a2',
    'ck2' : 'a2',
    'xk' : 'ck'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a2',
    'bk2' : 'b2',
    'ck2' : 'c2',
    'xk' : 'ck'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexExtending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a2',
    '12' : 'a2',
    '22' : 'a2',
    'xk' : 2
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexExtending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a2',
    '12' : 'b2',
    '22' : 'c2',
    'xk' : 2
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexExtending( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  test.identical( got, exp );

  test.close( 'onEach returns unroll with map' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexExtending( src, '*/f1' );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'aa.f1', f2 : 'aa.f2' }, c : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  var got = _.indexExtending( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'bb.f1', f2 : 'bb.f2' }, c : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  var got = _.indexExtending( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' }, 'bb.f1' : { f1 : 'bb.f1', f2 : 'bb.f2' }, 'cc.f1' : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexExtending( src, '*/f1' );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' } ];
  var got = _.indexExtending( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'a.f2' } };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.indexExtending( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'a.f2' }, 'b.f1' : { f1 : 'b.f1', f2 : 'b.f2' }, 'c.f1' : { f1 : 'c.f1', f2 : 'c.f2' } };
  test.identical( got, exp );

  test.case = 'src - str';
  var src = 'src';
  var got = _.indexExtending( src, '*/src' );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function indexSupplementing( test )
{
  test.open( 'without onEach' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexSupplementing( src );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexSupplementing( src );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexSupplementing( src );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexSupplementing( src );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexSupplementing( src );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexSupplementing( src );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexSupplementing( src );
  var exp = { src : undefined };
  test.identical( got, exp );

  test.close( 'without onEach' );

  /* - */

  test.open( 'onEach returns undefined' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexSupplementing( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexSupplementing( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexSupplementing( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexSupplementing( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexSupplementing( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexSupplementing( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexSupplementing( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns undefined' );

  /* - */

  test.open( 'onEach returns element' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexSupplementing( src, ( e, k ) => e );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexSupplementing( src, ( e, k ) => e );
  var exp = { a : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexSupplementing( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexSupplementing( src, ( e, k ) => e );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexSupplementing( src, ( e, k ) => e );
  var exp = { a : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexSupplementing( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexSupplementing( src, ( e, k ) => e );
  var exp = { src : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexSupplementing( src, ( e, k ) => k );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexSupplementing( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexSupplementing( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexSupplementing( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexSupplementing( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexSupplementing( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexSupplementing( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach returns map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexSupplementing( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : 'a'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : 'a'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexSupplementing( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : 'a'
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : 'a'
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexSupplementing( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns map' );

  /* - */

  test.open( 'onEach returns unroll' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexSupplementing( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : 'a'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : 'a'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexSupplementing( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : 'a'
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : 'a'
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexSupplementing( src, onEach );
  var exp = { undefined2 : 'src', x : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns unroll' );

  /* - */

  test.open( 'onEach returns unroll with map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexSupplementing( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a2',
    'bk2' : 'a2',
    'ck2' : 'a2',
    'xk' : 'ak'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a2',
    'bk2' : 'b2',
    'ck2' : 'c2',
    'xk' : 'ak'
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexSupplementing( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a2',
    '12' : 'a2',
    '22' : 'a2',
    'xk' : 0
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexSupplementing( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a2',
    '12' : 'b2',
    '22' : 'c2',
    'xk' : 0
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexSupplementing( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  test.identical( got, exp );

  test.close( 'onEach returns unroll with map' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexSupplementing( src, '*/f1' );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'aa.f1', f2 : 'aa.f2' }, c : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  var got = _.indexSupplementing( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'bb.f1', f2 : 'bb.f2' }, c : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  var got = _.indexSupplementing( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' }, 'bb.f1' : { f1 : 'bb.f1', f2 : 'bb.f2' }, 'cc.f1' : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexSupplementing( src, '*/f1' );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with same elements';
  var src = [ { f1 : 'a.f1', f2 : 'f2' }, { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' } ];
  var got = _.indexSupplementing( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'f2' } };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.indexSupplementing( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'a.f2' }, 'b.f1' : { f1 : 'b.f1', f2 : 'b.f2' }, 'c.f1' : { f1 : 'c.f1', f2 : 'c.f2' } };
  test.identical( got, exp );

  test.case = 'src - str';
  var src = 'src';
  var got = _.indexSupplementing( src, '*/src' );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function indexAppending( test )
{
  test.open( 'without onEach' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexAppending( src );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexAppending( src );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexAppending( src );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexAppending( src );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexAppending( src );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexAppending( src );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexAppending( src );
  var exp = { src : undefined };
  test.identical( got, exp );

  test.close( 'without onEach' );

  /* - */

  test.open( 'onEach returns undefined' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexAppending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexAppending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexAppending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexAppending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexAppending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexAppending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexAppending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns undefined' );

  /* - */

  test.open( 'onEach returns element' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexAppending( src, ( e, k ) => e );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexAppending( src, ( e, k ) => e );
  var exp = { a : [ 'a', 'a', 'a' ] };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexAppending( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexAppending( src, ( e, k ) => e );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexAppending( src, ( e, k ) => e );
  var exp = { a : [ 'a', 'a', 'a' ] };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexAppending( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexAppending( src, ( e, k ) => e );
  var exp = { src : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexAppending( src, ( e, k ) => k );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexAppending( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexAppending( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexAppending( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexAppending( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexAppending( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexAppending( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach returns map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexAppending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : [ 'a', 'b', 'c' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexAppending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : [ 'a', 'b', 'c' ]
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexAppending( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns map' );

  /* - */

  test.open( 'onEach returns unroll' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexAppending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : [ 'a', 'b', 'c' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexAppending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : [ 'a', 'b', 'c' ]
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexAppending( src, onEach );
  var exp = { undefined2 : 'src', x : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns unroll' );

  /* - */

  test.open( 'onEach returns unroll with map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexAppending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a2',
    'bk2' : 'a2',
    'ck2' : 'a2',
    'xk' : [ 'ak', 'bk', 'ck' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a2',
    'bk2' : 'b2',
    'ck2' : 'c2',
    'xk' : [ 'ak', 'bk', 'ck' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexAppending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a2',
    '12' : 'a2',
    '22' : 'a2',
    'xk' : [ 0, 1, 2 ]
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexAppending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a2',
    '12' : 'b2',
    '22' : 'c2',
    'xk' : [ 0, 1, 2 ]
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexAppending( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  test.identical( got, exp );

  test.close( 'onEach returns unroll with map' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexAppending( src, '*/f1' );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'aa.f1', f2 : 'aa.f2' }, c : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  var got = _.indexAppending( src, '*/f1' );
  var exp = { 'aa.f1' : [ { f1 : 'aa.f1', f2 : 'aa.f2' }, { f1 : 'aa.f1', f2 : 'aa.f2' }, { f1 : 'aa.f1', f2 : 'aa.f2' } ] };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'bb.f1', f2 : 'bb.f2' }, c : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  var got = _.indexAppending( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' }, 'bb.f1' : { f1 : 'bb.f1', f2 : 'bb.f2' }, 'cc.f1' : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexAppending( src, '*/f1' );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with same elements';
  var src = [ { f1 : 'a.f1', f2 : 'f2' }, { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' } ];
  var got = _.indexAppending( src, '*/f1' );
  var exp = { 'a.f1' : [ { f1 : 'a.f1', f2 : 'f2' }, { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' } ] };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.indexAppending( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'a.f2' }, 'b.f1' : { f1 : 'b.f1', f2 : 'b.f2' }, 'c.f1' : { f1 : 'c.f1', f2 : 'c.f2' } };
  test.identical( got, exp );

  test.case = 'src - str';
  var src = 'src';
  var got = _.indexAppending( src, '*/src' );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function indexPrepending( test )
{
  test.open( 'without onEach' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexPrepending( src );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexPrepending( src );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexPrepending( src );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexPrepending( src );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexPrepending( src );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexPrepending( src );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexPrepending( src );
  var exp = { src : undefined };
  test.identical( got, exp );

  test.close( 'without onEach' );

  /* - */

  test.open( 'onEach returns undefined' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexPrepending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexPrepending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexPrepending( src, ( e, k ) => undefined );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexPrepending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexPrepending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexPrepending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexPrepending( src, ( e, k ) => undefined );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns undefined' );

  /* - */

  test.open( 'onEach returns element' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexPrepending( src, ( e, k ) => e );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexPrepending( src, ( e, k ) => e );
  var exp = { a : [ 'a', 'a', 'a' ] };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexPrepending( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexPrepending( src, ( e, k ) => e );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexPrepending( src, ( e, k ) => e );
  var exp = { a : [ 'a', 'a', 'a' ] };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexPrepending( src, ( e, k ) => e );
  var exp = { a : 'a', b : 'b', c : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexPrepending( src, ( e, k ) => e );
  var exp = { src : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns element' );

  /* - */

  test.open( 'onEach returns key' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexPrepending( src, ( e, k ) => k );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var got = _.indexPrepending( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'a', ck : 'a' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.indexPrepending( src, ( e, k ) => k );
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexPrepending( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var got = _.indexPrepending( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'a', 2 : 'a' };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var got = _.indexPrepending( src, ( e, k ) => k );
  var exp = { 0 : 'a', 1 : 'b', 2 : 'c' };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var got = _.indexPrepending( src, ( e, k ) => k );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach returns key' );

  /* - */

  test.open( 'onEach returns map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexPrepending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : [ 'c', 'b', 'a' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexPrepending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : [ 'c', 'b', 'a' ]
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => { return { [ k ] : e, [ k + '2' ] : e, 'x' : e } };
  var got = _.indexPrepending( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns map' );

  /* - */

  test.open( 'onEach returns unroll' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexPrepending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a',
    'bk2' : 'a',
    'ck2' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a',
    'bk2' : 'b',
    'ck2' : 'c',
    'x' : [ 'c', 'b', 'a' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexPrepending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a',
    '12' : 'a',
    '22' : 'a',
    'x' : [ 'a', 'a', 'a' ]
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a',
    '12' : 'b',
    '22' : 'c',
    'x' : [ 'c', 'b', 'a' ]
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ k, k + '2', 'x' ] );
  var got = _.indexPrepending( src, onEach );
  var exp = { undefined2 : 'src', x : 'src' };
  test.identical( got, exp );

  test.close( 'onEach returns unroll' );

  /* - */

  test.open( 'onEach returns unroll with map' );

  test.case = 'src - empty map';
  var src = {};
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexPrepending( src, onEach );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical values';
  var src = { ak : 'a', bk : 'a', ck : 'a' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'a',
    'ck' : 'a',
    'ak2' : 'a2',
    'bk2' : 'a2',
    'ck2' : 'a2',
    'xk' : [ 'ck', 'bk', 'ak' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical values';
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    'ak' : 'a',
    'bk' : 'b',
    'ck' : 'c',
    'ak2' : 'a2',
    'bk2' : 'b2',
    'ck2' : 'c2',
    'xk' : [ 'ck', 'bk', 'ak' ]
  };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexPrepending( src, onEach );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with identical elements';
  var src = [ 'a', 'a', 'a' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'a',
    '2' : 'a',
    '02' : 'a2',
    '12' : 'a2',
    '22' : 'a2',
    'xk' : [ 2, 1, 0 ]
  };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ 'a', 'b', 'c' ];
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexPrepending( src, onEach );
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '02' : 'a2',
    '12' : 'b2',
    '22' : 'c2',
    'xk' : [ 2, 1, 0 ]
  };
  test.identical( got, exp );

  test.case = 'src - primitive';
  var src = 'src';
  var onEach = ( e, k ) => _.unroll.make( [ { [ k ] : e }, { [ k + '2' ] : e + '2' }, { 'xk' : k } ] );
  var got = _.indexPrepending( src, onEach );
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  test.identical( got, exp );

  test.close( 'onEach returns unroll with map' );

  /* - */

  test.open( 'onEach - selector' );

  test.case = 'src - empty map';
  var src = {};
  var got = _.indexPrepending( src, '*/f1' );
  var exp = {};
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'aa.f1', f2 : 'aa.f2' }, c : { f1 : 'aa.f1', f2 : 'aa.f2' } };
  var got = _.indexPrepending( src, '*/f1' );
  var exp = { 'aa.f1' : [  { f1 : 'aa.f1', f2 : 'aa.f2' }, { f1 : 'aa.f1', f2 : 'aa.f2' }, { f1 : 'aa.f1', f2 : 'aa.f2' } ] };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - map with not identical entries';
  var src = { a : { f1 : 'aa.f1', f2 : 'aa.f2' }, b : { f1 : 'bb.f1', f2 : 'bb.f2' }, c : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  var got = _.indexPrepending( src, '*/f1' );
  var exp = { 'aa.f1' : { f1 : 'aa.f1', f2 : 'aa.f2' }, 'bb.f1' : { f1 : 'bb.f1', f2 : 'bb.f2' }, 'cc.f1' : { f1 : 'cc.f1', f2 : 'cc.f2' } };
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - empty array';
  var src = [];
  var got = _.indexPrepending( src, '*/f1' );
  var exp = {};
  test.identical( got, exp );

  test.case = 'src - array with same elements';
  var src = [ { f1 : 'a.f1', f2 : 'f2' }, { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' } ];
  var got = _.indexPrepending( src, '*/f1' );
  var exp = { 'a.f1' : [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'a.f1', f2 : 'f2' } ] };
  test.identical( got, exp );

  test.case = 'src - array with not identical elements';
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.indexPrepending( src, '*/f1' );
  var exp = { 'a.f1' : { f1 : 'a.f1', f2 : 'a.f2' }, 'b.f1' : { f1 : 'b.f1', f2 : 'b.f2' }, 'c.f1' : { f1 : 'c.f1', f2 : 'c.f2' } };
  test.identical( got, exp );

  test.case = 'src - str';
  var src = 'src';
  var got = _.indexPrepending( src, '*/src' );
  var exp = {};
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function indexAppendingExperiment( test )
{
  test.case = 'src - array, a few nodes with same selection';
  var src = [ { f1 : 'a.f1' }, { f1 : 'a.f1' }, { f1 : 'a.f1' } ];
  var got = _.indexAppending( src, '*/f1' );
  var exp = { 'a.f1' : [ { f1 : 'a.f1' }, { f1 : 'a.f1' }, { f1 : 'a.f1' } ] };
  test.true( src !== got );
  test.identical( got, exp );
}
indexAppendingExperiment.experimental = 1;

//

function remap( test )
{

  /* */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remap( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { a : undefined, b : undefined, c : undefined };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remap( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remap( src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remap( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = {};
  var src = [ 'a', 'b', 'c' ];
  var got = _.remap( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remap( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remap( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a', '1' : 'b', '2' : 'c' };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remap( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remap( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'ak' : 'ak', 'bk' : 'bk', 'ck' : 'ck' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remap( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 0, '1' : 1, '2' : 2 };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remap( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remap( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* */

  test.open( 'onEach - routine returning map' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : { 'ak' : 'a', 'ak2' : 'a', 'x' : 'a' },
    'bk' : { 'bk' : 'b', 'bk2' : 'b', 'x' : 'b' },
    'ck' : { 'ck' : 'c', 'ck2' : 'c', 'x' : 'c' }
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remap( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : { '0' : 'a', '02' : 'a', 'x' : 'a' },
    '1' : { '1' : 'b', '12' : 'b', 'x' : 'b' },
    '2' : { '2' : 'c', '22' : 'c', 'x' : 'c' }
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remap( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remap( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning map' );

  /* */

  test.open( 'onEach - routine returning unroll' );

  test.case = 'src - map';
  var exp = { 'ak' : 'x', 'bk' : 'x', 'ck' : 'x' }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remap( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'x', '1' : 'x', '2' : 'x' }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remap( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remap( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll' );

  /* */

  test.open( 'onEach - routine returning unroll of maps' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : { 'xk' : 'ak' },
    'bk' : { 'xk' : 'bk' },
    'ck' : { 'xk' : 'ck' }
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remap( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : { 'xk' : 0 },
    '1' : { 'xk' : 1 },
    '2' : { 'xk' : 2 }
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remap( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remap( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll of maps' );

  /* */

  test.open( 'onEach - selector' );

  test.case = 'src - map';
  var exp = { 'a' : 'a.f1', 'b' : 'b.f1', 'c' : 'c.f1' };
  var src = { a : { f1 : 'a.f1', f2 : 'a.f2' }, b : { f1 : 'b.f1', f2 : 'b.f2' }, c : { f1 : 'c.f1', f2 : 'c.f2' } };
  var got = _.remap( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a.f1', '1' : 'b.f1', '2' : 'c.f1' };
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.remap( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remap( src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* */

} /* end of function remap */

//

function remapExtending( test )
{

  /* */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapExtending( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { a : undefined, b : undefined, c : undefined };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapExtending( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapExtending( src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapExtending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = {};
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapExtending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapExtending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapExtending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a', '1' : 'b', '2' : 'c' };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapExtending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapExtending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'ak' : 'ak', 'bk' : 'bk', 'ck' : 'ck' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapExtending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 0, '1' : 1, '2' : 2 };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapExtending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapExtending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* */

  test.open( 'onEach - routine returning map' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a',
    'x' : 'c',
    'bk' : 'b',
    'bk2' : 'b',
    'ck' : 'c',
    'ck2' : 'c'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapExtending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b',
    '22' : 'c',
    '02' : 'a',
    'x' : 'c'
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapExtending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' }
  var src = 'src';
  var got = _.remapExtending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning map' );

  /* */

  test.open( 'onEach - routine returning unroll' );

  test.case = 'src - map';
  var exp = { 'ak' : 'x', 'bk' : 'x', 'ck' : 'x' }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapExtending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'x', '1' : 'x', '2' : 'x' }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapExtending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapExtending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll' );

  /* */

  test.open( 'onEach - routine returning unroll of maps' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a2',
    'xk' : 'ck',
    'bk' : 'b',
    'bk2' : 'b2',
    'ck' : 'c',
    'ck2' : 'c2'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapExtending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b2',
    '22' : 'c2',
    '02' : 'a2',
    'xk' : 2
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapExtending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  var src = 'src';
  var got = _.remapExtending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll of maps' );

  /* */

  test.open( 'onEach - selector' );

  test.case = 'src - map';
  var exp = { 'a' : 'a.f1', 'b' : 'b.f1', 'c' : 'c.f1' };
  var src = { a : { f1 : 'a.f1', f2 : 'a.f2' }, b : { f1 : 'b.f1', f2 : 'b.f2' }, c : { f1 : 'c.f1', f2 : 'c.f2' } };
  var got = _.remapExtending( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a.f1', '1' : 'b.f1', '2' : 'c.f1' };
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.remapExtending( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapExtending( src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* */

} /* end of function remapExtending */

//

function remapSupplementing( test )
{
  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapSupplementing( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { a : undefined, b : undefined, c : undefined };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapSupplementing( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapSupplementing( src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapSupplementing( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = {};
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapSupplementing( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapSupplementing( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapSupplementing( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a', '1' : 'b', '2' : 'c' };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapSupplementing( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapSupplementing( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'ak' : 'ak', 'bk' : 'bk', 'ck' : 'ck' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapSupplementing( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 0, '1' : 1, '2' : 2 };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapSupplementing( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapSupplementing( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* */

  test.open( 'onEach - routine returning map' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a',
    'x' : 'a',
    'bk' : 'b',
    'bk2' : 'b',
    'ck' : 'c',
    'ck2' : 'c'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapSupplementing( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b',
    '22' : 'c',
    '02' : 'a',
    'x' : 'a'
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapSupplementing( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' }
  var src = 'src';
  var got = _.remapSupplementing( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning map' );

  /* */

  test.open( 'onEach - routine returning unroll' );

  test.case = 'src - map';
  var exp = { 'ak' : 'x', 'bk' : 'x', 'ck' : 'x' }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapSupplementing( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'x', '1' : 'x', '2' : 'x' }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapSupplementing( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapSupplementing( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll' );

  /* */

  test.open( 'onEach - routine returning unroll of maps' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a2',
    'xk' : 'ak',
    'bk' : 'b',
    'bk2' : 'b2',
    'ck' : 'c',
    'ck2' : 'c2'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapSupplementing( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b2',
    '22' : 'c2',
    '02' : 'a2',
    'xk' : 0
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapSupplementing( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  var src = 'src';
  var got = _.remapSupplementing( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll of maps' );

  /* */

  test.open( 'onEach - selector' );

  test.case = 'src - map';
  var exp = { 'a' : 'a.f1', 'b' : 'b.f1', 'c' : 'c.f1' };
  var src = { a : { f1 : 'a.f1', f2 : 'a.f2' }, b : { f1 : 'b.f1', f2 : 'b.f2' }, c : { f1 : 'c.f1', f2 : 'c.f2' } };
  var got = _.remapSupplementing( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a.f1', '1' : 'b.f1', '2' : 'c.f1' };
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.remapSupplementing( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapSupplementing( src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );
}

//

function remapAppending( test )
{

  /* */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapAppending( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { a : undefined, b : undefined, c : undefined };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapAppending( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapAppending( src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapAppending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = {};
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapAppending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapAppending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapAppending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a', '1' : 'b', '2' : 'c' };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapAppending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapAppending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'ak' : 'ak', 'bk' : 'bk', 'ck' : 'ck' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapAppending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 0, '1' : 1, '2' : 2 };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapAppending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapAppending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* */

  test.open( 'onEach - routine returning map' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a',
    'x' : [ 'a', 'b', 'c' ],
    'bk' : 'b',
    'bk2' : 'b',
    'ck' : 'c',
    'ck2' : 'c'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapAppending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b',
    '22' : 'c',
    '02' : 'a',
    'x' : [ 'a', 'b', 'c' ],
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapAppending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' }
  var src = 'src';
  var got = _.remapAppending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning map' );

  /* */

  test.open( 'onEach - routine returning unroll' );

  test.case = 'src - map';
  var exp = { 'ak' : 'x', 'bk' : 'x', 'ck' : 'x' }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapAppending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'x', '1' : 'x', '2' : 'x' }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapAppending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapAppending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll' );

  /* */

  test.open( 'onEach - routine returning unroll of maps' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a2',
    'xk' : [ 'ak', 'bk', 'ck' ],
    'bk' : 'b',
    'bk2' : 'b2',
    'ck' : 'c',
    'ck2' : 'c2'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapAppending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b2',
    '22' : 'c2',
    '02' : 'a2',
    'xk' : [ 0, 1, 2 ],
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapAppending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  var src = 'src';
  var got = _.remapAppending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll of maps' );

  /* */

  test.open( 'onEach - selector' );

  test.case = 'src - map';
  var exp = { 'a' : 'a.f1', 'b' : 'b.f1', 'c' : 'c.f1' };
  var src = { a : { f1 : 'a.f1', f2 : 'a.f2' }, b : { f1 : 'b.f1', f2 : 'b.f2' }, c : { f1 : 'c.f1', f2 : 'c.f2' } };
  var got = _.remapAppending( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a.f1', '1' : 'b.f1', '2' : 'c.f1' };
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.remapAppending( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapAppending( src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* */

} /* end of function remapAppending */

//

function remapPrepending( test )
{

  /* */

  test.open( 'no onEach' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapPrepending( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { a : undefined, b : undefined, c : undefined };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapPrepending( src );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapPrepending( src );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'no onEach' );

  /* */

  test.open( 'onEach - routine returning undefined' );

  test.case = 'src - map';
  var exp = {};
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapPrepending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = {};
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapPrepending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapPrepending( src, ( e, k ) => undefined );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning undefined' );

  /* */

  test.open( 'onEach - routine returning element' );

  test.case = 'src - map';
  var exp = { ak : 'a', bk : 'b', ck : 'c' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapPrepending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a', '1' : 'b', '2' : 'c' };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapPrepending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapPrepending( src, ( e, k ) => e );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning element' );

  /* */

  test.open( 'onEach - routine returning key' );

  test.case = 'src - map';
  var exp = { 'ak' : 'ak', 'bk' : 'bk', 'ck' : 'ck' };
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapPrepending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 0, '1' : 1, '2' : 2 };
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapPrepending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( _.entity.lengthOf( got ), _.entity.lengthOf( src ) );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {}
  var src = 'src';
  var got = _.remapPrepending( src, ( e, k ) => k );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning key' );

  /* */

  test.open( 'onEach - routine returning map' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a',
    'x' : [ 'c', 'b', 'a' ],
    'bk' : 'b',
    'bk2' : 'b',
    'ck' : 'c',
    'ck2' : 'c'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapPrepending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b',
    '22' : 'c',
    '02' : 'a',
    'x' : [ 'c', 'b', 'a' ],
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapPrepending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src', 'x' : 'src' }
  var src = 'src';
  var got = _.remapPrepending( src, ( e, k ) => { return { [ k ] : e, [ k+'2' ] : e, 'x' : e } } );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning map' );

  /* */

  test.open( 'onEach - routine returning unroll' );

  test.case = 'src - map';
  var exp = { 'ak' : 'x', 'bk' : 'x', 'ck' : 'x' }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapPrepending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'x', '1' : 'x', '2' : 'x' }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapPrepending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapPrepending( src, ( e, k ) => _.unroll.make([ k, k+'2', 'x' ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll' );

  /* */

  test.open( 'onEach - routine returning unroll of maps' );

  test.case = 'src - map';
  var exp =
  {
    'ak' : 'a',
    'ak2' : 'a2',
    'xk' : [ 'ck', 'bk', 'ak' ],
    'bk' : 'b',
    'bk2' : 'b2',
    'ck' : 'c',
    'ck2' : 'c2'
  }
  var src = { ak : 'a', bk : 'b', ck : 'c' };
  var got = _.remapPrepending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp =
  {
    '0' : 'a',
    '1' : 'b',
    '2' : 'c',
    '12' : 'b2',
    '22' : 'c2',
    '02' : 'a2',
    'xk' : [ 2, 1, 0 ],
  }
  var src = [ 'a', 'b', 'c' ];
  var got = _.remapPrepending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = { 'undefined' : 'src', 'undefined2' : 'src2', 'xk' : undefined };
  var src = 'src';
  var got = _.remapPrepending( src, ( e, k ) => _.unroll.make([ { [ k ] : e }, { [ k+'2' ] : e + '2' }, { 'xk' : k } ]) );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - routine returning unroll of maps' );

  /* */

  test.open( 'onEach - selector' );

  test.case = 'src - map';
  var exp = { 'a' : 'a.f1', 'b' : 'b.f1', 'c' : 'c.f1' };
  var src = { a : { f1 : 'a.f1', f2 : 'a.f2' }, b : { f1 : 'b.f1', f2 : 'b.f2' }, c : { f1 : 'c.f1', f2 : 'c.f2' } };
  var got = _.remapPrepending( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - array';
  var exp = { '0' : 'a.f1', '1' : 'b.f1', '2' : 'c.f1' };
  var src = [ { f1 : 'a.f1', f2 : 'a.f2' }, { f1 : 'b.f1', f2 : 'b.f2' }, { f1 : 'c.f1', f2 : 'c.f2' } ];
  var got = _.remapPrepending( src, '*/f1' );
  test.true( src !== got );
  test.identical( got, exp );

  test.case = 'src - str';
  var exp = {};
  var src = 'src';
  var got = _.remapPrepending( src, '*/length' );
  test.true( src !== got );
  test.identical( got, exp );

  test.close( 'onEach - selector' );

  /* */

} /* end of function remapPrepending */

// --
//
// --

function _entityMost( test )
{
  test.open( 'returnMax - false' );

  test.case = 'src - array, without onEvaluate';
  var o =
  {
    src : [ 3, 1, 9, 0, 5 ],
    returnMax : 0
  }
  var exp = { index : 3, key : 3, value : 0, element : 0 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEach';
  var o =
  {
    src : [ 3, -4, 9, -16, 5, -2 ],
    returnMax : 0,
    onEach : ( e ) => e * e
  }
  var exp = { index : 5, key : 5, value : 4, element : -2 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - unroll, without onEvaluate';
  var o =
  {
    src : _.unroll.make( [ 3, 1, 9, 0, 5 ] ),
    returnMax : 0
  }
  var exp = { index : 3, key : 3, value : 0, element : 0 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - unroll, onEach';
  var o =
  {
    src : _.unroll.make( [ 3, -4, 9, -16, 5, -2 ] ),
    returnMax : 0,
    onEach : ( e ) => e * e
  }
  var exp = { index : 5, key : 5, value : 4, element : -2 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - BufferTyped, without onEvaluate';
  var o =
  {
    src : new U16x( [ 3, 1, 9, 0, 5 ] ),
    returnMax : 0
  }
  var exp = { index : 3, key : 3, value : 0, element : 0 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - BufferTyped, onEach';
  var o =
  {
    src : new F64x( [ 3, -4, 9, -16, 5, -2 ] ),
    returnMax : 0,
    onEach : ( e ) => e * e
  }
  var exp = { index : 5, key : 5, value : 4, element : -2 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - aixiliary, without onEvaluate';
  var o =
  {
    src : { a : 1, b : 0, c : 3, d : 2 },
    returnMax : 0
  }
  var exp = { index : 1, key : 'b', value : 0, element : 0 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - aixiliary, onEach';
  var o =
  {
    src : { a : 5, b : 1, c : 3, d : 2 },
    returnMax : 0,
    onEach : ( e ) => e * e
  }
  var exp = { index : 1, key : 'b', value : 1, element : 1 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.close( 'returnMax - false' );

  /* - */

  test.open( 'returnMax - true' );

  test.case = 'src - array, without onEvaluate';
  var o =
  {
    src : [ 3, 1, 9, 0, 5 ],
    returnMax : 1
  }
  var exp = { index : 2, key : 2, value : 9, element : 9 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEach';
  var o =
  {
    src : [ 3, -4, 9, -16, 5, -2 ],
    returnMax : 1,
    onEach : ( e ) => e * e
  }
  var exp = { index : 3, key : 3, value : 256, element : -16 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - unroll, without onEvaluate';
  var o =
  {
    src : _.unroll.make( [ 3, 1, 9, 0, 5 ] ),
    returnMax : 1
  }
  var exp = { index : 2, key : 2, value : 9, element : 9 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - unroll, onEach';
  var o =
  {
    src : _.unroll.make( [ 3, -4, 9, -16, 5, -2 ] ),
    returnMax : 1,
    onEach : ( e ) => e * e
  }
  var exp = { index : 3, key : 3, value : 256, element : -16 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - BufferTyped, without onEvaluate';
  var o =
  {
    src : new U16x( [ 3, 1, 9, 0, 5 ] ),
    returnMax : 1
  }
  var exp = { index : 2, key : 2, value : 9, element : 9 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - BufferTyped, onEach';
  var o =
  {
    src : new F64x( [ 3, -4, 9, -16, 5, -2 ] ),
    returnMax : 1,
    onEach : ( e ) => e * e
  }
  var exp = { index : 3, key : 3, value : 256, element : -16 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - aixiliary, without onEvaluate';
  var o =
  {
    src : { a : 1, b : 0, c : 3, d : 2 },
    returnMax : 1
  }
  var exp = { index : 2, key : 'c', value : 3, element : 3 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - aixiliary, onEach';
  var o =
  {
    src : { a : 5, b : 1, c : 3, d : 2 },
    returnMax : 1,
    onEach : ( e ) => e * e
  }
  var exp = { index : 0, key : 'a', value : 25, element : 5 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.close( 'returnMax - true' );

  /* - */

  test.case = 'src - array, onEach, onEvaluate1.length - 1';
  var o =
  {
    src : [ 1, 5, 6, 5 ],
    onEvaluate : ( e ) => e > 5
  }
  var exp = { index : 2, key : 2, value : 6, element : 6 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEach, onEvaluate1.length - 1';
  var o =
  {
    src : [ 1, 5, 6, 1, 7 ],
    onEach : ( e ) => e + 1,
    onEvaluate : ( e ) => e > 5
  }
  var exp = { index : 4, key : 4, value : 8, element : 7 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEvaluate.length - 2';
  var o =
  {
    src : [ 3, 5, 6, 6 ],
    onEvaluate : ( v, prev ) => v > prev + 1
  }
  var exp = { index : 1, key : 1, value : 5, element : 5 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEach, onEvaluate.length - 2';
  var o =
  {
    src : [ 3, 5, 6, 8 ],
    onEach : ( e ) => e - 1,
    onEvaluate : ( v, prev ) => v > prev + 2
  }
  var exp = { index : 2, key : 2, value : 5, element : 6 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  /* */

  test.case = 'src - aixiliary, onEvaluate1.length - 1';
  var o =
  {
    src : { a : 0, b : 2, c : 3, d : 4 },
    onEvaluate : ( e ) => e > 5
  }
  var exp = { index : -1, key : undefined, value : undefined, element : undefined };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - aixiliary, onEach, onEvaluate1.length - 1';
  var o =
  {
    src : { a : 0, b : 6, c : 3, d : 4 },
    onEach : ( e ) => e + 1,
    onEvaluate : ( e ) => e > 5
  }
  var exp = { index : 1, key : 'b', value : 7, element : 6 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEvaluate.length - 2';
  var o =
  {
    src : { a : 0, b : 1, c : 3, d : 4 },
    onEvaluate : ( v, prev ) => v > prev + 1
  }
  var exp = { index : 2, key : 'c', value : 3, element : 3 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEvaluate.length - 2';
  var o =
  {
    src : { a : 0, b : 1, c : 3, d : 4, e : 5 },
    onEvaluate : ( v, prev ) => v > prev + 1
  }
  var exp = { index : 4, key : 'e', value : 5, element : 5 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  test.case = 'src - array, onEach, onEvaluate.length - 2';
  var o =
  {
    src : { a : 0, b : 2, c : 3, d : 4 },
    onEach : ( e ) => e - 1,
    onEvaluate : ( v, prev ) => v > prev + 1
  }
  var exp = { index : 3, key : 'd', value : 3, element : 4 };
  var got = _._entityMost( o );
  test.true( got !== o );
  test.identical( got, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _._entityMost() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _._entityMost( { src : [ 1, 2 ], returnMax : 0 }, 'extra' ) );

  test.case = 'wrong type of o';
  test.shouldThrowErrorSync( () => _._entityMost( [ { src : [ 1, 2 ], returnMax : 1 } ] ) );

  test.case = 'unnecessary fields in o';
  test.shouldThrowErrorSync( () => _._entityMost( [ { srcs : [ 1, 2 ], returnMax : 1 } ] ) );

  test.case = 'wrong type of o.src';
  test.shouldThrowErrorSync( () => _._entityMost( [ { src : 'wrong', returnMax : 1 } ] ) );

  test.case = 'wrong length of onEach';
  test.shouldThrowErrorSync( () => _._entityMost( [ { src : [ 1, 2 ], returnMax : 1, onEach : () => 2 } ] ) );
  test.shouldThrowErrorSync( () => _._entityMost( [ { src : [ 1, 2 ], returnMax : 1, onEach : ( a, b, c ) => a + b } ] ) );

  test.case = 'wrong length of onEvaluate';
  test.shouldThrowErrorSync( () => _._entityMost( [ { src : [ 1, 2 ], onEvaluate : () => 2 } ] ) );
  test.shouldThrowErrorSync( () => _._entityMost( [ { src : [ 1, 2 ],  onEvaluate : ( a, b, c ) => a + b } ] ) );

  test.case = 'without onEvaluate, without returnMax';
  test.shouldThrowErrorSync( () => _._entityMost( [ { src : [ 1, 2 ] } ] ) );

};

//

function entityMin( test )
{
  test.case = 'array, without onEvaluate';
  var src = [ 3, 1, 9, 0, 5 ];
  var exp = { index : 3, key : 3, value : 0, element : 0 };
  var got = _.entityMin( src );
  test.identical( got, exp );

  test.case = 'array, onEvaluate';
  var src = [ 3, -4, 9, -16, 5, -2 ];
  var exp = { index : 5, key : 5, value : 4, element : -2 };
  var got = _.entityMin( src, ( e ) => e * e );
  test.identical( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'unroll, without onEvaluate';
  var src = _.unroll.make( [ 3, 1, 9, 0, 5 ] );
  var exp = { index : 3, key : 3, value : 0, element : 0 };
  var got = _.entityMin( src );
  test.identical( got, exp );

  test.case = 'unroll, onEvaluate';
  var src = _.unroll.make( [ 3, -4, 9, -16, 5, -2 ] );
  var exp = { index : 5, key : 5, value : 4, element : -2 };
  var got = _.entityMin( src, ( e ) => e * e );
  test.identical( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'argumentsArray, without onEvaluate';
  var src = _.argumentsArray.make( [ 3, 1, 9, 0, 5 ] );
  var exp = { index : 3, key : 3, value : 0, element : 0 };
  var got = _.entityMin( src );
  test.identical( got, exp );

  test.case = 'argumentsArray, onEvaluate';
  var src = _.argumentsArray.make( [ 3, -4, 9, -16, 5, -2 ] );
  var exp = { index : 5, key : 5, value : 4, element : -2 };
  var got = _.entityMin( src, ( e ) => e * e );
  test.equivalent( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'BufferTyped, without onEvaluate';
  var src = new U8x( [ 3, 1, 9, 0, 5 ] );
  var exp = { index : 3, key : 3, value : 0, element : 0 };
  var got = _.entityMin( src );
  test.identical( got, exp );

  test.case = 'BufferTyped, onEvaluate';
  var src = new I32x( [ 3, -4, 9, -16, 5, -2 ] );
  var exp = { index : 5, key : 5, value : 4, element : -2 };
  var got = _.entityMin( src, ( e ) => e * e );
  test.equivalent( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'map, without onEvaluate';
  var src = { a : 25, b : 16, c : 9 };
  var exp = { index : 2, key : 'c', value : 9, element : 9  };
  var got = _.entityMin( src );
  test.identical( got, exp );

  test.case = 'map, onEvaluate';
  var src = { a : 25, b : 16, c : 9 };
  var exp = { index : 2, key : 'c', value : 81, element : 9  };
  var got = _.entityMin( src, ( e ) => e * e );
  test.identical( got, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityMin() );

  test.case = 'extra argument';
  test.shouldThrowErrorSync( () => _.entityMin( [ 1, 3 ], sqr, true ) );

  test.case = 'onEvaluate is not routine';
  test.shouldThrowErrorSync( () => _.entityMin( [ 1, 3 ], 'wrong' ) );
}

//

function entityMax( test )
{

  test.case = 'array, without onEvaluate';
  var src = [ 3, 1, 9, 0, 5 ];
  var exp = { index : 2, key : 2, value : 9, element : 9 };
  var got = _.entityMax( src );
  test.identical( got, exp );

  test.case = 'array, onEvaluate';
  var src = [ 3, -4, 9, -16, 5, -2 ];
  var exp = { index : 3, key : 3, value : 256, element : -16 };
  var got = _.entityMax( src, ( e ) => e * e );
  test.identical( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'unroll, without onEvaluate';
  var src = _.unroll.make( [ 3, 1, 9, 0, 5 ] );
  var exp = { index : 2, key : 2, value : 9, element : 9 };
  var got = _.entityMax( src );
  test.identical( got, exp );

  test.case = 'unroll, onEvaluate';
  var src = _.unroll.make( [ 3, -4, 9, -16, 5, -2 ] );
  var exp = { index : 3, key : 3, value : 256, element : -16 };
  var got = _.entityMax( src, ( e ) => e * e );
  test.identical( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'argumentsArray, without onEvaluate';
  var src = _.argumentsArray.make( [ 3, 1, 9, 0, 5 ] );
  var exp = { index : 2, key : 2, value : 9, element : 9 };
  var got = _.entityMax( src );
  test.identical( got, exp );

  test.case = 'argumentsArray, onEvaluate';
  var src = _.argumentsArray.make( [ 3, -4, 9, -16, 5, -2 ] );
  var exp = { index : 3, key : 3, value : 256, element : -16 };
  var got = _.entityMax( src, ( e ) => e * e );
  test.equivalent( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'BufferTyped, without onEvaluate';
  var src = new U8x( [ 3, 1, 9, 0, 5 ] );
  var exp = { index : 2, key : 2, value : 9, element : 9 };
  var got = _.entityMax( src );
  test.identical( got, exp );

  test.case = 'BufferTyped, onEvaluate';
  var src = new I32x( [ 3, -4, 9, -16, 5, -2 ] );
  var exp = { index : 3, key : 3, value : 256, element : -16 };
  var got = _.entityMax( src, ( e ) => e * e );
  test.equivalent( src, [ 3, -4, 9, -16, 5, -2 ] );
  test.identical( got, exp );

  test.case = 'map, without onEvaluate';
  var src = { a : 25, b : 16, c : 9 };
  var exp = { index : 0, key : 'a', value : 25, element : 25  };
  var got = _.entityMax( src );
  test.identical( got, exp );

  test.case = 'map, onEvaluate';
  var src = { a : 25, b : 16, c : 9 };
  var exp = { index : 0, key : 'a', value : 625, element : 25  };
  var got = _.entityMax( src, ( e ) => e * e );
  test.identical( got, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.entityMax() );

  test.case = 'extra argument';
  test.shouldThrowErrorSync( () => _.entityMax( [ 1, 3 ], sqr, true ) );

  test.case = 'onEvaluate is not routine';
  test.shouldThrowErrorSync( () => _.entityMax( [ 1, 3 ], 'wrong' ) );
}

// --
//
// --

const Proto =
{

  name : 'Tools.Functional.l0.l9',
  silencing : 1,
  enabled : 1,
  routineTimeOut : 10000,

  tests :
  {

    // scalar

    scalarAppend,
    scalarAppendOnce,
    scalarPrepend,
    scalarPrependOnce,

    scalarToVector,

    /* */

    entityTimeExperiment,

    entityEach,
    // entityEachKey, /* xxx */
    entityEachOwn,

    entityOnlyOnlyDst,
    entityOnlyOnlySrc,
    entityOnlyBothSame,
    entityOnlyBoth,
    entityOnlyDiffTypes,
    entityOnlyOnlySets,
    entityOnlyOnlyHashMaps,

    entityButOnlyDst,
    entityButOnlySrc,
    entityButBothSame,
    entityButBoth,
    entityButDiffTypes,
    entityButOnlySets,
    entityButOnlyHashMaps,

    /*
                            only / but            and                      or                         xor                          xand
    value for evaluator       src              dst, src                 dst, src                    dst, src                    dst, src
    value for dst             dst                dst                    first +                     first +                   dst if positive
    */

    entityAndOnlyDst,
    entityAndOnlySrc,
    entityAndBothSame,
    entityAndBoth,
    entityAndDiffTypes,
    entityAndOnlySets,
    entityAndOnlyHashMaps,

    entityOrOnlyDst,
    entityOrOnlySrc,
    entityOrBothSame,
    entityOrBoth,
    entityOrDiffTypes,
    entityOrOnlySets,
    entityOrOnlyHashMaps,

    entityXorOnlyDst,
    entityXorOnlySrc,
    entityXorBothSame,
    entityXorBoth,
    entityXorDiffTypes,
    entityXorOnlySets,
    entityXorOnlyHashMaps,

    // entityXandOnlyDst, /* qqq : for Dmytro : implement */
    // entityXandOnlySrc, /* qqq : for Dmytro : implement */
    // entityXandBothSame, /* qqq : for Dmytro : implement */
    // entityXandBoth, /* qqq : for Dmytro : implement */

    entityAll,
    entityAny,
    entityNone,

    /* */

    _filter_functor,

    // entityMap, /* qqq2 for Dmytro : rewrite please */ /* aaa : Done */
    // entityMapDifferentCallbacks, /* qqq2 for Dmytro : rewrite please */ /* aaa : Done */
    entityMapCountables,
    entityMapObjects,

    // entityMapWithoutDst_, /* xxx : restore later */
    entityMapDstNull_,
    entityMapDstNotNull_,

    // entityFilter,  /* qqq2 for Dmytro : rewrite please */ /* aaa : Done */
    entityFilterCountables,
    entityFilterObjects,

    // entityFilterWithoutDst_, /* xxx : restore later */
    entityFilterDstNull_,
    entityFilterDstNotNull_,

    entityFilterDeep,

    /* */

    // first,
    // last,

    index,
    indexExtending,
    indexSupplementing,
    indexAppending,
    indexAppendingExperiment,
    indexPrepending,

    remap,
    remapExtending,
    remapSupplementing,
    remapAppending,
    remapPrepending,

    /* */

    _entityMost,
    entityMin,
    entityMax,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
