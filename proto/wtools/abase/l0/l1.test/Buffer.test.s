( function _l0_l1_Buffer_test_s_()
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
// buffer
// --

function bufferRawIs( test )
{
  test.case = 'array buffer';
  var src = new BufferRaw( [ 1, 2 ] );
  var got = _.bufferRawIs( src );
  test.identical( got, true );
  test.false( _.bufferTypedIs( src ) );
  test.false( _.bufferNodeIs( src ) );
  test.false( _.bufferViewIs( src ) );

  test.case = 'shared array buffer';
  var src = new SharedArrayBuffer( [ 1, 2 ] );
  var got = _.bufferRawIs( src );
  test.identical( got, true );
  test.false( _.bufferTypedIs( src ) );
  test.false( _.bufferNodeIs( src ) );
  test.false( _.bufferViewIs( src ) );

  test.case = 'typed array';
  var src = new F32x( [ 1, 2 ] );
  var got = _.bufferRawIs( src );
  test.identical( got, false );

  var src = new U8xClamped( 10*10*4 );
  var got = _.bufferRawIs( src );
  test.identical( got, false );

  if( Config.interpreter === 'njs' )
  {
    test.case = 'node buffer';
    var src1 = BufferNode.alloc( 10 );
    var got1 = _.bufferRawIs( src1 );
    test.identical( got1, false );

    var src1 = BufferNode.from( [ 2, 4 ] );
    var got1 = _.bufferRawIs( src1 );
    test.identical( got1, false );
  }

  test.case = 'view buffer, BufferRaw';
  var src = new BufferView( new BufferRaw( [ 10 ] ) );
  var got = _.bufferRawIs( src );
  test.identical( got, false );

  test.case = 'view buffer, SharedArrayBuffer';
  var src = new BufferView( new SharedArrayBuffer( [ 10 ] ) );
  var got = _.bufferRawIs( src );
  test.identical( got, false );

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  var got = _.bufferRawIs();
  test.identical( got, false );

  test.case = 'extra arguments';
  var src = new BufferRaw( [ 1, 2 ] );
  var got = _.bufferRawIs( src, new U8x( 1 ) );
  test.identical( got, true );
}

//

function bufferTypedIs( test )
{
  test.case = 'typed array';
  var src = new F32x( [ 1, 2 ] );
  var got = _.bufferTypedIs( src );
  test.identical( got, true );
  test.false( _.bufferRawIs( src ) );
  test.false( _.bufferNodeIs( src ) );
  test.false( _.bufferViewIs( src ) );

  var src = new U8xClamped( 10*10*4 );
  var got = _.bufferTypedIs( src );
  test.identical( got, true );
  test.false( _.bufferRawIs( src ) );
  test.false( _.bufferNodeIs( src ) );
  test.false( _.bufferViewIs( src ) );

  test.case = 'array buffer';
  var src = new BufferRaw( [ 1, 2 ] );
  var got = _.bufferTypedIs( src );
  test.identical( got, false );

  test.case = 'shared array buffer';
  var src = new SharedArrayBuffer( [ 1, 2 ] );
  var got = _.bufferTypedIs( src );
  test.identical( got, false );

  if( Config.interpreter === 'njs' )
  {
    test.case = 'node buffer';
    var src1 = BufferNode.alloc( 10 );
    var got1 = _.bufferTypedIs( src1 );
    test.identical( got1, false );

    var src1 = BufferNode.from( [ 2, 4 ] );
    var got1 = _.bufferTypedIs( src1 );
    test.identical( got1, false );
  }

  test.case = 'view buffer, BufferRaw';
  var src = new BufferView( new BufferRaw( [ 10 ] ) );
  var got = _.bufferTypedIs( src );
  test.identical( got, false );

  test.case = 'view buffer, SharedArrayBuffer';
  var src = new BufferView( new SharedArrayBuffer( [ 10 ] ) );
  var got = _.bufferTypedIs( src );
  test.identical( got, false );

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  var got = _.bufferTypedIs();
  test.identical( got, false );

  test.case = 'extra arguments';
  var src = new I16x( [ 1, 2 ] );
  var got = _.bufferTypedIs( src, new SharedArrayBuffer( 1 ) );
  test.identical( got, true );
}

//

function bufferViewIs( test )
{
  test.case = 'view buffer, BufferRaw';
  var src = new BufferView( new BufferRaw( [ 10 ] ) );
  var got = _.bufferViewIs( src );
  test.identical( got, true );
  test.false( _.bufferRawIs( src ) );
  test.false( _.bufferNodeIs( src ) );
  test.false( _.bufferTypedIs( src ) );

  test.case = 'view buffer, SharedArrayBuffer';
  var src = new BufferView( new SharedArrayBuffer( [ 10 ] ) );
  var got = _.bufferViewIs( src );
  test.identical( got, true );
  test.false( _.bufferRawIs( src ) );
  test.false( _.bufferNodeIs( src ) );
  test.false( _.bufferTypedIs( src ) );

  test.case = 'typed array';
  var src = new F32x( [ 1, 2 ] );
  var got = _.bufferViewIs( src );
  test.identical( got, false );

  var src = new U8xClamped( 10*10*4 );
  var got = _.bufferViewIs( src );
  test.identical( got, false );

  test.case = 'array buffer';
  var src = new BufferRaw( [ 1, 2 ] );
  var got = _.bufferViewIs( src );
  test.identical( got, false );

  test.case = 'shared array buffer';
  var src = new SharedArrayBuffer( [ 1, 2 ] );
  var got = _.bufferViewIs( src );
  test.identical( got, false );

  if( Config.interpreter === 'njs' )
  {
    test.case = 'node buffer';
    var src1 = BufferNode.alloc( 10 );
    var got1 = _.bufferViewIs( src1 );
    test.identical( got1, false );

    var src1 = BufferNode.from( [ 2, 4 ] );
    var got1 = _.bufferViewIs( src1 );
    test.identical( got1, false );
  }

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  var got = _.bufferViewIs();
  test.identical( got, false );

  test.case = 'extra arguments';
  var src = new BufferView( new BufferRaw( [ 1, 2 ] ) );
  var got = _.bufferViewIs( src, new U8x( 1 ) );
  test.identical( got, true );
}

//

function bufferNodeIs( test )
{
  if( !Config.interpreter === 'njs' )
  return;

  test.case = 'node buffer';
  var src = BufferNode.alloc( 10 );
  var got = _.bufferNodeIs( src );
  test.identical( got, true );
  test.false( _.bufferRawIs( src ) );
  test.false( _.bufferTypedIs( src ) );
  test.false( _.bufferViewIs( src ) );

  var src = BufferNode.from( [ 2, 4 ] );
  var got = _.bufferNodeIs( src );
  test.identical( got, true );
  test.false( _.bufferRawIs( src ) );
  test.false( _.bufferTypedIs( src ) );
  test.false( _.bufferViewIs( src ) );

  test.case = 'typed array';
  var src = new F32x( [ 1, 2 ] );
  var got = _.bufferNodeIs( src );
  test.identical( got, false );

  var src = new U8xClamped( 10*10*4 );
  var got = _.bufferNodeIs( src );
  test.identical( got, false );

  test.case = 'array buffer';
  var src = new BufferRaw( [ 1, 2 ] );
  var got = _.bufferNodeIs( src );
  test.identical( got, false );

  test.case = 'shared array buffer';
  var src = new SharedArrayBuffer( [ 1, 2 ] );
  var got = _.bufferNodeIs( src );
  test.identical( got, false );

  test.case = 'view buffer, BufferRaw';
  var src = new BufferView( new BufferRaw( [ 10 ] ) );
  var got = _.bufferNodeIs( src );
  test.identical( got, false );

  test.case = 'view buffer, SharedArrayBuffer';
  var src = new BufferView( new SharedArrayBuffer( [ 10 ] ) );
  var got = _.bufferNodeIs( src );
  test.identical( got, false );

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  var got = _.bufferNodeIs();
  test.identical( got, false );

  test.case = 'extra arguments';
  var src = BufferNode.from( [ 1, 2 ] );
  var got = _.bufferNodeIs( src, new SharedArrayBuffer( 1 ) );
  test.identical( got, true );
}

//

function bufferAnyIs( test )
{

  bufferAnyIsTemplate({ method : 'anyIs' });
  // bufferAnyIsTemplate({ method : 'anyIsOld' });
  // bufferAnyIsTemplate({ method : 'anyIsUsingInstanceOf' });

  function bufferAnyIsTemplate( env )
  {
    test.case = `${__.entity.exportStringSolo( env )}, check null`;
    var got = _.buffer[ env.method ]( null );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check undefined`;
    var got = _.buffer[ env.method ]( undefined );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check zero`;
    var got = _.buffer[ env.method ]( 0 );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check false`;
    var got = _.buffer[ env.method ]( false );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check NaN`;
    var got = _.buffer[ env.method ]( NaN );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check Symbol`;
    var got = _.buffer[ env.method ]( Symbol( 'a' ) );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty map`;
    var got = _.buffer[ env.method ]( {} );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty array`;
    var got = _.buffer[ env.method ]( [] );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check string`;
    var got = _.buffer[ env.method ]( 'string' );
    var expected = false;
    test.identical( got, expected );

    /* BufferTyped */

    test.case = `${__.entity.exportStringSolo( env )}, check U8x constructor`;
    var got = _.buffer[ env.method ]( U8x );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty U8x instance`;
    var src = new U8x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty U16x instance`;
    var src = new U16x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty U32x instance`;
    var src = new U32x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty U64x instance`;
    var src = new U64x( 2 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

  /**/

    test.case = `${__.entity.exportStringSolo( env )}, check empty I8x instance`;
    var src = new I8x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty I16x instance`;
    var src = new I16x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty I32x instance`;
    var src = new I32x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty I64x instance`;
    var src = new I64x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

  /**/

    test.case = `${__.entity.exportStringSolo( env )}, check empty F32x instance`;
    var src = new F32x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, check empty F64x instance`;
    var src = new F32x( 5 );
    var got = _.buffer[ env.method ]( src );
    var expected = true;
    test.identical( got, expected );

  /* BufferNode */

  if( Config.interpreter === 'njs' )
  {
    test.case = `${__.entity.exportStringSolo( env )}, check empty BufferNode instance`;
    var src1 = BufferNode.alloc( 5 );
    var got1 = _.buffer[ env.method ]( src1 );
    var expected1 = true;
    test.identical( got1, expected1 );

    test.case = `${__.entity.exportStringSolo( env )}, check BufferNode instance`;
    var src1 = BufferNode.allocUnsafe( 5 );
    var got1 = _.buffer[ env.method ]( src1 );
    var expected1 = true;
    test.identical( got1, expected1 );

    test.case = `${__.entity.exportStringSolo( env )}, check BufferNode from string`;
    var src1 = BufferNode.from( 'str' );
    var got1 = _.buffer[ env.method ]( src1 );
    var expected1 = true;
    test.identical( got1, expected1 );
  }

  /* BufferRaw */

  test.case = `${__.entity.exportStringSolo( env )}, check BufferRaw constructor`;
  var got = _.buffer[ env.method ]( BufferRaw );
  var expected = false;
  test.identical( got, expected );

  test.case = `${__.entity.exportStringSolo( env )}, check empty BufferRaw instance`;
  var src = new BufferRaw( 5 );
  var got = _.buffer[ env.method ]( src );
  var expected = true;
  test.identical( got, expected );

  test.case = `${__.entity.exportStringSolo( env )}, check BufferRawShared instance`;
  var src = new BufferRawShared( 5 );
  var got = _.buffer[ env.method ]( src );
  var expected = true;
  test.identical( got, expected );

  /* BufferView */

  test.case = `${__.entity.exportStringSolo( env )}, check BufferView constructor`;
  var got = _.buffer[ env.method ]( BufferView );
  var expected = false;
  test.identical( got, expected );

  test.case = `${__.entity.exportStringSolo( env )}, check BufferView instance`;
  var got = _.buffer[ env.method ]( new BufferView( new ArrayBuffer(16) ) );
  var expected = true;
  test.identical( got, expected );

  /**/

  test.case = `${__.entity.exportStringSolo( env )}, check without argument`;
  var got = _.buffer[ env.method ]();
  var expected = false;
  test.identical( got, expected );

  test.case = `${__.entity.exportStringSolo( env )}, check extra arguments`;
  var got = _.buffer[ env.method ]( new U8x( [ 1, 2, 3 ] ), new BufferRaw( 5 ) );
  var expected = true;
  test.identical( got, expected );

  /**/
  test.case = `${__.entity.exportStringSolo( env )}, demonstrates a bug in is`;
  var got = _.buffer[ env.method ]( { byteLength : 6 } );
  var expected = false;
  test.identical( got, expected );
  }
}

//

function bufferBytesIs( test )
{

  test.case = 'check null';
  var got = _.bufferBytesIs( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check undefined';
  var got = _.bufferBytesIs( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check zero';
  var got = _.bufferBytesIs( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check false';
  var got = _.bufferBytesIs( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check NaN';
  var got = _.bufferBytesIs( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check Symbol';
  var got = _.bufferBytesIs( Symbol( 'a' ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check empty map';
  var got = _.bufferBytesIs( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check empty array';
  var got = _.bufferBytesIs( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check string';
  var got = _.bufferBytesIs( 'string' );
  var expected = false;
  test.identical( got, expected );

  /* BufferTyped */

  test.case = 'check U8x constructor';
  var got = _.bufferBytesIs( U8x );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check U8x instance';
  var src = new U8x( [ 1, 2 ] );
  var got = _.bufferBytesIs( src );
  var expected = true;
  test.identical( got, expected );

  test.case = 'check U16x instance';
  var src = new U16x( [ 2, 3 ] );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check U32x instance';
  var src = new U32x( [ 1, 2 ] );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check empty U64x instance';
  var src = new U64x( 2 );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  /**/

  test.case = 'check I8x constructor';
  var got = _.bufferBytesIs( I8x );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check empty I8x instance';
  var src = new I8x( 5 );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check I16x instance';
  var src = new I16x( [ 1, 2 ] );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check I32x instance';
  var src = new I32x( [ 1, 2 ] );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check empty I64x instance';
  var src = new I64x( 5 );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  /**/

  test.case = 'check F32x instance';
  var src = new F32x( [ 2, 1 ] );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check empty F64x instance';
  var src = new F32x( 5 );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  /* BufferNode */

  if( Config.interpreter === 'njs' )
  {
    test.case = 'check empty BufferNode instance';
    var src1 = BufferNode.alloc( 5 );
    var got1 = _.bufferBytesIs( src1 );
    var expected1 = false;
    test.identical( got1, expected1 );

    test.case = 'check BufferNode instance';
    var src1 = BufferNode.allocUnsafe( 5 );
    var got1 = _.bufferBytesIs( src1 );
    var expected1 = false;
    test.identical( got1, expected1 );

    test.case = 'check BufferNode from string';
    var src1 = BufferNode.from( 'str' );
    var got1 = _.bufferBytesIs( src1 );
    var expected1 = false;
    test.identical( got1, expected1 );
  }

  /* BufferRaw */

  test.case = 'check empty BufferRaw instance';
  var src = new BufferRaw( 5 );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'check BufferRawShared instance';
  var src = new BufferRawShared( 5 );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  /* BufferView */

  test.case = 'check BufferView instance';
  var src = new BufferView( new U8x( [ 1, 2, 3 ] ).buffer );
  var got = _.bufferBytesIs( src );
  var expected = false;
  test.identical( got, expected );

  /**/

  test.case = 'check without argument';
  var got = _.bufferBytesIs();
  var expected = false;
  test.identical( got, expected );

  test.case = 'check extra arguments';
  var got = _.bufferBytesIs( new U8x( [ 1, 2, 3 ] ), new BufferRaw( 5 ) );
  var expected = true;
  test.identical( got, expected );

}

//

function makeCommon( test )
{
  methodEach({ tools : 'default', type : 'Array' });
  methodEach({ tools : 'Array', type : 'Array' });
  methodEach({ tools : 'F32x', type : 'F32x' });

  /* - */

  function methodEach( env )
  {
    env.method = 'make';
    act( env );
    env.method = 'cloneShallow';
    act( env );
  }

  /* */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `no args`;
      var got = long[ env.method ]();
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `length`;
      var got = long[ env.method ]( 3 );
      test.true( got instanceof Constructor );
      test.true( got.length === 3 );

      /* */

      test.case = `empty array`;
      var src = [];
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `filled array`;
      var src = [ 2, 3 ];
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );
      test.identical( got, Constructor.from([ 2, 3 ]) );

      /* */

      test.case = `empty unroll`;
      var src = _.unroll.make( [] );
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `filled unroll`;
      var src = _.unroll.make([ 2, 3 ]);
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );
      test.identical( got, Constructor.from([ 2, 3 ]) );

      /* */

      test.case = `empty argumentsArray`;
      var src = _.argumentsArray.make( [] );
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `filled argumentsArray`;
      var src = _.argumentsArray.make([ 2, 3 ]);
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 2, 3 ]) );
    }

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long[ env.method ]( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );

    test.case = `filled typed buffer`;
    var src = _.f32x.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 2, 3 ]) );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw();
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );

    test.case = `filled raw buffer`;
    var src = _.u8x.make([ 2, 3 ]).buffer;
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 2, 3 ]).buffer );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw() );
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );

    test.case = `filled view buffer`;
    var src = new BufferView( _.u8x.make([ 2, 3 ]).buffer );
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 2, 3 ]).buffer ) );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long[ env.method ]( src );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 0 );

      test.case = `filled node buffer`;
      var src = BufferNode.from([ 2, 3 ]);
      var got = long[ env.method ]( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 2, 3 ]) );
    }

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `null and length - number`;
      var got = long[ env.method ]( null, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and length - array`;
      var got = long[ env.method ]( null, [ 1, 2 ] );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and length - raw buffer`;
      var got = long[ env.method ]( null, new BufferRaw( 2 ) );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( null, length );
      test.true( got instanceof Constructor );
      test.identical( got.length, 3 );

      /* */

      test.case = `empty array and length`;
      var src = [];
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `empty unroll and length`;
      var src = _.unroll.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `empty argumentsArray and length`;
      var src = _.argumentsArray.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `empty typed buffer and length`;
      var src = _.u8x.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof U8x );
      test.identical( got.length, 2 );

      test.case = `empty raw buffer and length`;
      var src = new BufferRaw( 0 );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof BufferRaw );
      test.identical( got.byteLength, 2 );

      test.case = `empty view buffer and length`;
      var src = new BufferView( new BufferRaw( 0 ) );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof BufferView );
      test.identical( got.byteLength, 2 );

      if( Config.interpreter === 'njs' )
      {
        test.case = `empty node buffer and length`;
        var src = BufferNode.alloc( 0 );
        var got = long[ env.method ]( src, 2 );
        test.true( got instanceof BufferNode );
        test.identical( got.length, 2 );
      }

      /* */

      test.case = `non-empty array and length longer`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3, 4, 0 ]) );

      test.case = `non-empty array and length shorter`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3 ]) );

      /* */

      test.case = `non-empty unroll and length longer`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3, 4, 0 ]) );

      test.case = `non-empty unroll and length shorter`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3 ]) );

      /* */

      test.case = `non-empty argumentsArray and length longer`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3, 4, 0 ]) );

      test.case = `non-empty argumentsArray and length shorter`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3 ]) );

      /* */

      test.case = `non-empty typed buffer and length longer`;
      var src = _.u8x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof U8x );
      test.identical( got, _.u8x.make([ 3, 4, 0 ]) );

      test.case = `non-empty typed buffer and length shorter`;
      var src = _.f32x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof F32x );
      test.identical( got, _.f32x.make([ 3 ]) );

      /* */

      test.case = `non-empty raw buffer and length longer`;
      var src = _.u8x.make([ 3, 4 ]).buffer;
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof BufferRaw );
      test.identical( got, _.u8x.make([ 3, 4, 0 ]).buffer );

      test.case = `non-empty typed buffer and length shorter`;
      var src = _.u8x.make([ 3, 4 ]).buffer;
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof BufferRaw );
      test.identical( got, _.u8x.make([ 3 ]).buffer );

      /* */

      test.case = `non-empty view buffer and length longer`;
      var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof BufferView );
      test.identical( got, new BufferView( _.u8x.make([ 3, 4, 0 ]).buffer ) );

      test.case = `non-empty view buffer and length shorter`;
      var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof BufferView );
      test.identical( got, new BufferView( _.u8x.make([ 3 ]).buffer ) );

      /* */

      if( Config.interpreter === 'njs' )
      {
        test.case = `non-empty view buffer and length longer`;
        var src = BufferNode.from([ 3, 4 ]);
        var got = long[ env.method ]( src, 3 );
        test.true( got instanceof BufferNode );
        test.identical( got, BufferNode.from([ 3, 4, 0 ]) );

        test.case = `non-empty view buffer and length shorter`;
        var src = BufferNode.from([ 3, 4 ]);
        var got = long[ env.method ]( src, 1 );
        test.true( got instanceof BufferNode );
        test.identical( got, BufferNode.from([ 3 ]) );
      }

      /* */

      test.case = `array and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( [ 0, 0, 0, 0 ], length );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    }

    /* */

    if( Config.debug )
    {
      if( env.method === 'cloneShallow' )
      {
        test.case = 'without arguments';
        test.shouldThrowErrorSync( () => long[ env.method ]() );

        test.case = 'extra arguments';
        test.shouldThrowErrorSync( () => long[ env.method ]( [], 1 ) );
      }
      else
      {
        test.case = 'extra arguments';
        test.shouldThrowErrorSync( () => long[ env.method ]( [], 1, 1 ) );

        test.case = 'wrong type of length';
        test.shouldThrowErrorSync( () => long[ env.method ]( [ 1, 2, 3 ], null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( 3, null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( Array, null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( null, null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( [ 1, 2, 3 ], undefined ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( 3, undefined ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( Array, undefined ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( null, undefined ) );

        test.case = 'wrong type of src';
        test.shouldThrowErrorSync( () => long[ env.method ]( 'wrong', 3 ) );
      }

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long[ env.method ]( undefined ) );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.tools ].buffer;
  }
}

//

function makeCommonWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    methodEach({ tools : 'default', type });
    methodEach({ tools : 'Array', type });
    methodEach({ tools : 'F32x', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function methodEach( env )
  {
    env.method = 'make';
    act( env );
    env.method = 'cloneShallow';
    act( env );
  }

  /* */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `no args`;
      var got = long[ env.method ]();
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `length`;
      var got = long[ env.method ]( 3 );
      test.true( got instanceof Constructor );
      test.true( got.length === 3 );

      /* */

      test.case = `empty array`;
      var src = [];
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `filled array`;
      var src = [ 2, 3 ];
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );
      test.identical( got, Constructor.from([ 2, 3 ]) );

      /* */

      test.case = `empty unroll`;
      var src = _.unroll.make( [] );
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `filled unroll`;
      var src = _.unroll.make([ 2, 3 ]);
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );
      test.identical( got, Constructor.from([ 2, 3 ]) );

      /* */

      test.case = `empty argumentsArray`;
      var src = _.argumentsArray.make( [] );
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `filled argumentsArray`;
      var src = _.argumentsArray.make([ 2, 3 ]);
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 2, 3 ]) );
    }

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long[ env.method ]( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );

    test.case = `filled typed buffer`;
    var src = _.f32x.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 2, 3 ]) );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw();
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );

    test.case = `filled raw buffer`;
    var src = _.u8x.make([ 2, 3 ]).buffer;
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 2, 3 ]).buffer );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw() );
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );

    test.case = `filled view buffer`;
    var src = new BufferView( _.u8x.make([ 2, 3 ]).buffer );
    var got = long[ env.method ]( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 2, 3 ]).buffer ) );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long[ env.method ]( src );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 0 );

      test.case = `filled node buffer`;
      var src = BufferNode.from([ 2, 3 ]);
      var got = long[ env.method ]( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 2, 3 ]) );
    }

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `null and length - number`;
      var got = long[ env.method ]( null, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and length - array`;
      var got = long[ env.method ]( null, [ 1, 2 ] );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and length - raw buffer`;
      var got = long[ env.method ]( null, new BufferRaw( 2 ) );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( null, length );
      test.true( got instanceof Constructor );
      test.identical( got.length, 3 );

      /* */

      test.case = `empty array and length`;
      var src = [];
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `empty unroll and length`;
      var src = _.unroll.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `empty argumentsArray and length`;
      var src = _.argumentsArray.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `empty typed buffer and length`;
      var src = _.u8x.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof U8x );
      test.identical( got.length, 2 );

      test.case = `empty raw buffer and length`;
      var src = new BufferRaw( 0 );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof BufferRaw );
      test.identical( got.byteLength, 2 );

      test.case = `empty view buffer and length`;
      var src = new BufferView( new BufferRaw( 0 ) );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof BufferView );
      test.identical( got.byteLength, 2 );

      if( Config.interpreter === 'njs' )
      {
        test.case = `empty node buffer and length`;
        var src = BufferNode.alloc( 0 );
        var got = long[ env.method ]( src, 2 );
        test.true( got instanceof BufferNode );
        test.identical( got.length, 2 );
      }

      /* */

      test.case = `non-empty array and length longer`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3, 4, 0 ]) );

      test.case = `non-empty array and length shorter`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3 ]) );

      /* */

      test.case = `non-empty unroll and length longer`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3, 4, 0 ]) );

      test.case = `non-empty unroll and length shorter`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3 ]) );

      /* */

      test.case = `non-empty argumentsArray and length longer`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3, 4, 0 ]) );

      test.case = `non-empty argumentsArray and length shorter`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 3 ]) );

      /* */

      test.case = `non-empty typed buffer and length longer`;
      var src = _.u8x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof U8x );
      test.identical( got, _.u8x.make([ 3, 4, 0 ]) );

      test.case = `non-empty typed buffer and length shorter`;
      var src = _.f32x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof F32x );
      test.identical( got, _.f32x.make([ 3 ]) );

      /* */

      test.case = `non-empty raw buffer and length longer`;
      var src = _.u8x.make([ 3, 4 ]).buffer;
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof BufferRaw );
      test.identical( got, _.u8x.make([ 3, 4, 0 ]).buffer );

      test.case = `non-empty typed buffer and length shorter`;
      var src = _.u8x.make([ 3, 4 ]).buffer;
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof BufferRaw );
      test.identical( got, _.u8x.make([ 3 ]).buffer );

      /* */

      test.case = `non-empty view buffer and length longer`;
      var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof BufferView );
      test.identical( got, new BufferView( _.u8x.make([ 3, 4, 0 ]).buffer ) );

      test.case = `non-empty view buffer and length shorter`;
      var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof BufferView );
      test.identical( got, new BufferView( _.u8x.make([ 3 ]).buffer ) );

      /* */

      if( Config.interpreter === 'njs' )
      {
        test.case = `non-empty view buffer and length longer`;
        var src = BufferNode.from([ 3, 4 ]);
        var got = long[ env.method ]( src, 3 );
        test.true( got instanceof BufferNode );
        test.identical( got, BufferNode.from([ 3, 4, 0 ]) );

        test.case = `non-empty view buffer and length shorter`;
        var src = BufferNode.from([ 3, 4 ]);
        var got = long[ env.method ]( src, 1 );
        test.true( got instanceof BufferNode );
        test.identical( got, BufferNode.from([ 3 ]) );
      }

      /* */

      test.case = `array and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( [ 0, 0, 0, 0 ], length );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.type ].buffer;
  }
}

makeCommonWithLongDescriptor.timeOut = 60000;

// function bufferMakeWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//
//   var list =
//   [
//     array,
//     unroll,
//   ];
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeBuffer )
//   {
//     test.case = 'src - empty long, not ins';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - null';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, null );
//     var expected = _.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - undefined';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, undefined );
//     var expected = _.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - number';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - empty array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, [] );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, [ 1, 2, 3 ] );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, not ins';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMake( src );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins < src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMake( src, 2 );
//     var expected = makeBuffer( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins > src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMake( src, 4 );
//     var expected = makeBuffer( [ 1, 2, 3, undefined ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - array, ins.length > src.length';
//     var src = makeBuffer( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.bufferMake( src, ins );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//   }
//
//   test.case = 'src - null';
//   var got = _.bufferMake( null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMake( null, null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMake( null, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - number';
//   var got = _.bufferMake( null, 5 );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - long';
//   var got = _.bufferMake( null, new U8x( 5 ) );
//   var expected = _.tools.defaultBufferTyped.make( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - number, ins - null';
//   var got = _.bufferMake( 5, null );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - number, ins - undefined';
//   var got = _.bufferMake( 5, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - U8x constructor, ins - number';
//   var got = _.bufferMake( U8x, 5 );
//   var expected = new U8x( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - F32x constructor, ins - long';
//   var ins = [ 1, 2, 3 ];
//   var got = _.bufferMake( F32x, ins );
//   var expected = new F32x( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var got = _.bufferMake( Array, 5 );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var ins = [ 1, 1, 1, 1, 1 ];
//   var got = _.bufferMake( Array, ins );
//   var expected = Array.from( [ 1, 1, 1, 1, 1 ] );
//   test.identical( got, expected );
//   test.true( got !== ins );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.bufferMake( [ 1, 2, 3 ], 4, 'extra' ) );
//
//   test.case = 'wrong type of src';
//   test.shouldThrowErrorSync( () => _.bufferMake( undefined ) );
//   test.shouldThrowErrorSync( () => _.bufferMake( 'wrong', 1 ) );
//   test.shouldThrowErrorSync( () => _.bufferMake( 1, 1 ) );
//
//   test.case = 'wrong type of ins';
//   test.shouldThrowErrorSync( () => _.bufferMake( [ 1, 2, 3 ], 'wrong' ) );
//   test.shouldThrowErrorSync( () => _.bufferMake( [ 1, 2, 3 ], Infinity  ) );
// }
//
// //
//
// function bufferMakeWithArgumentsArray( test )
// {
//   test.case = 'src - empty long, not ins';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMake( src );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - null';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMake( src, null );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - undefined';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMake( src, undefined );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - number';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMake( src, 2 );
//   var expected = _.tools.defaultBufferTyped.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - empty array';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMake( src, [] );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - array';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMake( src, [ 1, 2, 3 ] );
//   var expected = _.tools.defaultBufferTyped.make( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, not ins';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.bufferMake( src );
//   var expected = _.tools.defaultBufferTyped.make( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, ins - number, ins < src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.bufferMake( src, 2 );
//   var expected = _.tools.defaultBufferTyped.make( [ 1, 2 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, ins - number, ins > src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.bufferMake( src, 4 );
//   var expected = _.tools.defaultBufferTyped.make( [ 1, 2, 3, 0 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, ins - array, ins.length > src.length';
//   var src = _.argumentsArray.make( [ 0, 1 ] );
//   var ins = [ 1, 2, 3 ];
//   var got = _.bufferMake( src, ins );
//   var expected = _.tools.defaultBufferTyped.make( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//   test.true( got !== ins );
//   test.true( got !== src );
//
//   /* */
//
//   test.case = 'src - null';
//   var got = _.bufferMake( null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMake( null, null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMake( null, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - number';
//   var got = _.bufferMake( null, 5 );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - long';
//   var got = _.bufferMake( null, _.argumentsArray.make( 5 ) );
//   var expected = _.tools.defaultBufferTyped.make( new Array( 5 ) );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - number, ins - null';
//   var got = _.bufferMake( 5, null );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - number, ins - undefined';
//   var got = _.bufferMake( 5, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - U8x constructor, ins - number';
//   var got = _.bufferMake( U8x, 5 );
//   var expected = new U8x( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - F32x constructor, ins - long';
//   var ins = _.argumentsArray.make( [ 1, 2, 3 ] )
//   var got = _.bufferMake( F32x, ins );
//   var expected = new F32x( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var got = _.bufferMake( Array, 5 );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var ins = _.argumentsArray.make( [ 1, 1, 1, 1, 1 ] );
//   var got = _.bufferMake( Array, ins );
//   var expected = Array.from( [ 1, 1, 1, 1, 1 ] );
//   test.identical( got, expected );
//   test.true( got !== ins );
// }
//
// //
//
// function bufferMakeWithBuffers( test )
// {
//   var bufferTyped = ( buf ) =>
//   {
//     let name = buf.name;
//     return { [ name ] : function( src ){ return new buf( src ) } }[ name ];
//   };
//   var bufferNode = ( src ) => _.number.is( src ) ? BufferNode.alloc( src ) : BufferNode.from( src );
//   var bufferRaw = ( src ) => new U8x( src ).buffer;
//   var bufferView = ( src ) => new BufferView( bufferRaw( src ) );
//
//   /* lists */
//
//   var typedList =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//   var list = [ bufferRaw, bufferView ];
//   // var list = []; // xxx
//
//   for( let i = 0; i < typedList.length; i++ )
//   list.push( bufferTyped( typedList[ i ] ) );
//   if( Config.interpreter === 'njs' )
//   list.push( bufferNode );
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeBuffer )
//   {
//     test.case = 'src - empty buffer, not ins';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor );
//
//     test.case = 'src - empty buffer, ins - null';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, null );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - empty buffer, ins - undefined';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, undefined );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - empty buffer, ins - number';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - empty buffer, ins - empty array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, [] );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - empty buffer, ins - array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMake( src, [ 1, 2, 3 ] );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - filled buffer, not ins';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMake( src );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - filled buffer, ins - number, ins < src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMake( src, 2 );
//     var expected = makeBuffer( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - filled buffer, ins - number, ins > src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMake( src, 4 );
//     var expected = makeBuffer( [ 1, 2, 3, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//
//     test.case = 'src - filled buffer, ins - array, ins.length > src.length';
//     var src = makeBuffer( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.bufferMake( src, ins );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( got instanceof src.constructor )
//   }
//
//   test.case = 'src - null';
//   var got = _.bufferMake( null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMake( null, null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMake( null, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - number';
//   var got = _.bufferMake( null, 5 );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - long';
//   var got = _.bufferMake( null, new U8x( 5 ) );
//   var expected = _.tools.defaultBufferTyped.make( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - number, ins - null';
//   var got = _.bufferMake( 5, null );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - number, ins - undefined';
//   var got = _.bufferMake( 5, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - U8x constructor, ins - number';
//   var got = _.bufferMake( U8x, 5 );
//   var expected = new U8x( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - F32x constructor, ins - long';
//   var ins = [ 1, 2, 3 ];
//   var got = _.bufferMake( F32x, ins );
//   var expected = new F32x( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var got = _.bufferMake( Array, 5 );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var ins = [ 1, 1, 1, 1, 1 ];
//   var got = _.bufferMake( Array, ins );
//   var expected = Array.from( [ 1, 1, 1, 1, 1 ] );
//   test.identical( got, expected );
//   test.true( got !== ins );
// }
//
// //
//
// function bufferMakeWithArrayAndUnrollLongDescriptor( test )
// {
//   let times = 4;
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
//   /* test subroutine */
//
//   function testRun( long )
//   {
//     test.case = 'without arguments';
//     var got = long.bufferMake();
//     var expected = _.defaultBufferTyped.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - empty long, not ins';
//     var src = [];
//     var got = long.bufferMake( src );
//     var expected = _.defaultBufferTyped.make( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     // test.case = 'src - empty long, ins - null';
//     // var src = [];
//     // var got = long.bufferMake( src, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( got !== src );
//
//     // test.case = 'src - empty long, ins - undefined';
//     // var src = [];
//     // var got = long.bufferMake( src, undefined );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( got !== src );
//
//     test.case = 'src - empty long, ins - number';
//     var src = [];
//     var got = long.bufferMake( src, 2 );
//     var expected = _.defaultBufferTyped.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - empty array';
//     var src = [];
//     var got = long.bufferMake( src, [] );
//     var expected = _.defaultBufferTyped.make( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - array';
//     var src = [];
//     var got = long.bufferMake( src, [ 1, 2, 3 ] );
//     var expected = _.defaultBufferTyped.make([ 1, 2, 3 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, not ins';
//     var src = [ 1, 2, 3 ];
//     var got = long.bufferMake( src );
//     var expected = _.defaultBufferTyped.make([ 1, 2, 3 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins < src.length';
//     var src = [ 1, 2, 3 ];
//     var got = long.bufferMake( src, 2 );
//     var expected = _.defaultBufferTyped.make([ 1, 2 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     // test.case = 'src - filled long, ins - number, ins > src.length';
//     // var src = [ 1, 2, 3 ];
//     // var got = long.bufferMake( src, 4 );
//     // var expected = [ 1, 2, 3, undefined ];
//     // test.identical( got, expected );
//     // test.true( got !== src );
//
//     test.case = 'src - filled long, ins - array, ins.length > src.length';
//     var src = [ 0, 1 ];
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMake( src, ins );
//     var expected = _.defaultBufferTyped.make([ 1, 2, 3 ]);
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     /* */
//
//     test.case = 'src - null';
//     var got = long.bufferMake( null ); /* aaa2 : for Dmytro : should create typed buffer! */ /* Dmytro :routines make, makeUndefined, makeZeroed, makeEmpty creates typed buffers */
//     var expected = _.defaultBufferTyped.make( 0 );
//     test.identical( got, expected );
//
//     // test.case = 'src - null, ins - null';
//     // var got = long.bufferMake( null, null );
//     // var expected = long.defaultLong.make( 0 );
//     // test.identical( got, expected );
//
//     // test.case = 'src - null, ins - null';
//     // var got = long.bufferMake( null, undefined );
//     // var expected = long.defaultLong.make( 0 );
//     // test.identical( got, expected );
//
//     test.case = 'src - null, ins - number';
//     var got = long.bufferMake( null, 5 );
//     var expected = _.defaultBufferTyped.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - long';
//     var got = long.bufferMake( null, new U8x( 5 ) );
//     var expected = _.defaultBufferTyped.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     // test.case = 'src - number, ins - null';
//     // var got = long.bufferMake( 5, null );
//     // var expected = long.defaultLong.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src - number, ins - undefined';
//     // var got = long.bufferMake( 5, undefined );
//     // var expected = long.defaultLong.make( 5 );
//     // test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - U8x constructor, ins - number';
//     var got = long.bufferMake( U8x, 5 );
//     var expected = new U8x( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - F32x constructor, ins - long';
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMake( F32x, ins );
//     var expected = new F32x( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//
//     test.case = 'src - U8x constructor, ins - number';
//     var got = long.bufferMake( U8x, 5 );
//     var expected = _.u8x.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - U8x constructor, ins - number';
//     var ins = [ 1, 1, 1 ];
//     var got = long.bufferMake( U8x, ins );
//     var expected = _.u8x.make([ 1, 1, 1 ]);
//     test.identical( got, expected );
//     test.true( got !== ins );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'extra argument';
//       test.shouldThrowErrorSync( () => long.bufferMake( [ 1, 2, 3 ], 4, 'extra' ) );
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long.bufferMake( 'wrong', 1 ) );
//       test.shouldThrowErrorSync( () => long.bufferMake( 1, 1 ) );
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long.bufferMake( [ 1, 2, 3 ], 'wrong' ) );
//       test.shouldThrowErrorSync( () => long.bufferMake( [ 1, 2, 3 ], Infinity  ) );
//     }
//   }
// }
//
// bufferMakeWithArrayAndUnrollLongDescriptor.timeOut = 15000;
//
// //
//
// function bufferMakeWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
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
//     test.case = 'src - empty long, not ins';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMake( src );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - null';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMake( src, null );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - undefined';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMake( src, undefined );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - number';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMake( src, 2 );
//     var expected = long.defaultLong.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - empty array';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMake( src, [] );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - array';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMake( src, [ 1, 2, 3 ] );
//     var expected = long.defaultLong.make( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, not ins';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.bufferMake( src );
//     var expected = long.defaultLong.make( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins < src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.bufferMake( src, 2 );
//     var expected = long.defaultLong.make( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins > src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.bufferMake( src, 4 );
//     var expected = long.defaultLong.make( [ 1, 2, 3, undefined ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - array, ins.length > src.length';
//     var src = _.argumentsArray.make( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMake( src, ins );
//     var expected = long.defaultLong.make( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     /* */
//
//     test.case = 'src - null';
//     var got = long.bufferMake( null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMake( null, null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMake( null, undefined );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - number';
//     var got = long.bufferMake( null, 5 );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - long';
//     var got = long.bufferMake( null, _.argumentsArray.make( 5 ) );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - number, ins - null';
//     var got = long.bufferMake( 5, null );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - number, ins - undefined';
//     var got = long.bufferMake( 5, undefined );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - U8x constructor, ins - number';
//     var got = long.bufferMake( U8x, 5 );
//     var expected = new U8x( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - F32x constructor, ins - long';
//     var ins = _.argumentsArray.make( [ 1, 2, 3 ] )
//     var got = long.bufferMake( F32x, ins );
//     var expected = new F32x( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var got = long.bufferMake( Array, 5 );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var ins = _.argumentsArray.make( [ 1, 1, 1, 1, 1 ] );
//     var got = long.bufferMake( Array, ins );
//     var expected = Array.from( [ 1, 1, 1, 1, 1 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//   }
// }
//
// bufferMakeWithArgumentsArrayLongDescriptor.timeOut = 10000;
//
// //
//
// function bufferMakeWithBuffersLongDescriptor( test )
// {
//   var bufferTyped = ( buf ) =>
//   {
//     let name = buf.name;
//     return { [ name ] : function( src ){ return new buf( src ) } }[ name ];
//   };
//   var bufferNode = ( src ) => _.number.is( src ) ? BufferNode.alloc( src ) : BufferNode.from( src );
//   var bufferRaw = ( src ) => new U8x( src ).buffer;
//   var bufferView = ( src ) => new BufferView( bufferRaw( src ) );
//
//   /* lists */
//
//   var typedList =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//   var list = [ bufferRaw, bufferView ];
//
//   for( let i = 0; i < typedList.length; i++ )
//   list.push( bufferTyped( typedList[ i ] ) );
//   if( Config.interpreter === 'njs' )
//   list.push( bufferNode );
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, long - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, long - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeBuffer )
//   {
//     test.case = 'src - empty buffer, not ins';
//     var src = makeBuffer( [] );
//     var got = long.bufferMake( src );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - null';
//     var src = makeBuffer( [] );
//     var got = long.bufferMake( src, null );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - undefined';
//     var src = makeBuffer( [] );
//     var got = long.bufferMake( src, undefined );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - number';
//     var src = makeBuffer( [] );
//     var got = long.bufferMake( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - empty array';
//     var src = makeBuffer( [] );
//     var got = long.bufferMake( src, [] );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - array';
//     var src = makeBuffer( [] );
//     var got = long.bufferMake( src, [ 1, 2, 3 ] );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, not ins';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMake( src );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - number, ins < src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMake( src, 2 );
//     var expected = makeBuffer( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - number, ins > src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMake( src, 4 );
//     var expected = makeBuffer( [ 1, 2, 3, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - array, ins.length > src.length';
//     var src = makeBuffer( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMake( src, ins );
//     var expected = makeBuffer( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     /* */
//
//     test.case = 'src - null';
//     var got = long.bufferMake( null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMake( null, null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMake( null, undefined );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - number';
//     var got = long.bufferMake( null, 5 );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - long';
//     var got = long.bufferMake( null, new U8x( 5 ) );
//     var expected = long.defaultLong.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - number, ins - null';
//     var got = long.bufferMake( 5, null );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - number, ins - undefined';
//     var got = long.bufferMake( 5, undefined );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - U8x constructor, ins - number';
//     var got = long.bufferMake( U8x, 5 );
//     var expected = new U8x( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - F32x constructor, ins - long';
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMake( F32x, ins );
//     var expected = new F32x( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var got = long.bufferMake( Array, 5 );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var ins = [ 1, 1, 1, 1, 1 ];
//     var got = long.bufferMake( Array, ins );
//     var expected = Array.from( [ 1, 1, 1, 1, 1 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//   }
// }
//
// bufferMakeWithBuffersLongDescriptor.timeOut = 15000;

//

function makeUndefined( test )
{
  act({ tools : 'default', type : 'Array' });
  act({ tools : 'Array', type : 'Array' });
  act({ tools : 'F32x', type : 'F32x' });

  /* - */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = `no args`;
    var got = long.makeUndefined();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `length`;
    var got = long.makeUndefined( 3 );
    test.true( got instanceof Constructor );
    test.true( got.length === 3 );

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `filled array`;
    var src = [ 2, 3 ];
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );
    test.identical( got, Constructor.from([ 0, 0 ]) );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `filled unroll`;
    var src = _.unroll.make([ 2, 3 ]);
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );
    test.identical( got, Constructor.from([ 0, 0 ]) );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `filled argumentsArray`;
    var src = _.argumentsArray.make([ 2, 3 ]);
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0 ]) );

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long.makeUndefined( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );

    test.case = `filled typed buffer`;
    var src = _.f32x.make([ 2, 3 ]);
    var got = long.makeUndefined( src );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 0, 0 ]) );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw();
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );

    test.case = `filled raw buffer`;
    var src = _.u8x.make([ 2, 3 ]).buffer;
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 0, 0 ]).buffer );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw() );
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );

    test.case = `filled view buffer`;
    var src = new BufferView( _.u8x.make([ 2, 3 ]).buffer );
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 0, 0 ]).buffer ) );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long.makeUndefined( src );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 0 );

      test.case = `filled node buffer`;
      var src = BufferNode.from([ 2, 3 ]);
      var got = long.makeUndefined( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 0, 0 ]) );
    }

    /* */

    test.case = `null and length - number`;
    var got = long.makeUndefined( null, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `null and length - array`;
    var got = long.makeUndefined( null, [ 1, 2 ] );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `null and length - raw buffer`;
    var got = long.makeUndefined( null, new BufferRaw( 2 ) );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `null and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeUndefined( null, length );
    test.true( got instanceof Constructor );
    test.identical( got.length, 3 );

    /* */

    test.case = `empty array and length`;
    var src = [];
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `empty unroll and length`;
    var src = _.unroll.make( [] );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `empty argumentsArray and length`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `empty typed buffer and length`;
    var src = _.u8x.make( [] );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof U8x );
    test.identical( got.length, 2 );

    test.case = `empty raw buffer and length`;
    var src = new BufferRaw( 0 );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 2 );

    test.case = `empty view buffer and length`;
    var src = new BufferView( new BufferRaw( 0 ) );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 2 );

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer and length`;
      var src = BufferNode.alloc( 0 );
      var got = long.makeUndefined( src, 2 );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 2 );
    }

    /* */

    test.case = `non-empty array and length longer`;
    var src = [ 3, 4 ];
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    test.case = `non-empty array and length shorter`;
    var src = [ 3, 4 ];
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0 ]) );

    /* */

    test.case = `non-empty unroll and length longer`;
    var src = _.unroll.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    test.case = `non-empty unroll and length shorter`;
    var src = _.unroll.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0 ]) );

    /* */

    test.case = `non-empty argumentsArray and length longer`;
    var src = _.argumentsArray.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    test.case = `non-empty argumentsArray and length shorter`;
    var src = _.argumentsArray.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0 ]) );

    /* */

    test.case = `non-empty typed buffer and length longer`;
    var src = _.u8x.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof U8x );
    test.identical( got, _.u8x.make([ 0, 0, 0 ]) );

    test.case = `non-empty typed buffer and length shorter`;
    var src = _.f32x.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 0 ]) );

    /* */

    test.case = `non-empty raw buffer and length longer`;
    var src = _.u8x.make([ 3, 4 ]).buffer;
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 0, 0, 0 ]).buffer );

    test.case = `non-empty typed buffer and length shorter`;
    var src = _.u8x.make([ 3, 4 ]).buffer;
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 0 ]).buffer );

    /* */

    test.case = `non-empty view buffer and length longer`;
    var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 0, 0, 0 ]).buffer ) );

    test.case = `non-empty view buffer and length shorter`;
    var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 0 ]).buffer ) );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `non-empty view buffer and length longer`;
      var src = BufferNode.from([ 3, 4 ]);
      var got = long.makeUndefined( src, 3 );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 0, 0, 0 ]) );

      test.case = `non-empty view buffer and length shorter`;
      var src = BufferNode.from([ 3, 4 ]);
      var got = long.makeUndefined( src, 1 );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 0 ]) );
    }

    /* */

    test.case = `array and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeUndefined( [ 0, 0, 0, 0 ], length );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    /* */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.makeUndefined( [], 1, 1 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.makeUndefined( undefined ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( 'wrong', 3 ) );

      test.case = 'wrong type of length';
      test.shouldThrowErrorSync( () => long.makeUndefined( [ 1, 2, 3 ], null ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( 3, null ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( Array, null ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( null, null ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( [ 1, 2, 3 ], undefined ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( 3, undefined ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( Array, undefined ) );
      test.shouldThrowErrorSync( () => long.makeUndefined( null, undefined ) );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.tools ].buffer;
  }
}

//

function makeUndefinedWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    act({ tools : 'default', type });
    act({ tools : 'Array', type });
    act({ tools : 'F32x', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = `no args`;
    var got = long.makeUndefined();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `length`;
    var got = long.makeUndefined( 3 );
    test.true( got instanceof Constructor );
    test.true( got.length === 3 );

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `filled array`;
    var src = [ 2, 3 ];
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );
    test.identical( got, Constructor.from([ 0, 0 ]) );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `filled unroll`;
    var src = _.unroll.make([ 2, 3 ]);
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );
    test.identical( got, Constructor.from([ 0, 0 ]) );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `filled argumentsArray`;
    var src = _.argumentsArray.make([ 2, 3 ]);
    var got = long.makeUndefined( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0 ]) );

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long.makeUndefined( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );

    test.case = `filled typed buffer`;
    var src = _.f32x.make([ 2, 3 ]);
    var got = long.makeUndefined( src );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 0, 0 ]) );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw();
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );

    test.case = `filled raw buffer`;
    var src = _.u8x.make([ 2, 3 ]).buffer;
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 0, 0 ]).buffer );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw() );
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );

    test.case = `filled view buffer`;
    var src = new BufferView( _.u8x.make([ 2, 3 ]).buffer );
    var got = long.makeUndefined( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 0, 0 ]).buffer ) );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long.makeUndefined( src );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 0 );

      test.case = `filled node buffer`;
      var src = BufferNode.from([ 2, 3 ]);
      var got = long.makeUndefined( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 0, 0 ]) );
    }

    /* */

    test.case = `null and length - number`;
    var got = long.makeUndefined( null, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `null and length - array`;
    var got = long.makeUndefined( null, [ 1, 2 ] );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `null and length - raw buffer`;
    var got = long.makeUndefined( null, new BufferRaw( 2 ) );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `null and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeUndefined( null, length );
    test.true( got instanceof Constructor );
    test.identical( got.length, 3 );

    /* */

    test.case = `empty array and length`;
    var src = [];
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `empty unroll and length`;
    var src = _.unroll.make( [] );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `empty argumentsArray and length`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof Constructor );
    test.identical( got.length, 2 );

    test.case = `empty typed buffer and length`;
    var src = _.u8x.make( [] );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof U8x );
    test.identical( got.length, 2 );

    test.case = `empty raw buffer and length`;
    var src = new BufferRaw( 0 );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 2 );

    test.case = `empty view buffer and length`;
    var src = new BufferView( new BufferRaw( 0 ) );
    var got = long.makeUndefined( src, 2 );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 2 );

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer and length`;
      var src = BufferNode.alloc( 0 );
      var got = long.makeUndefined( src, 2 );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 2 );
    }

    /* */

    test.case = `non-empty array and length longer`;
    var src = [ 3, 4 ];
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    test.case = `non-empty array and length shorter`;
    var src = [ 3, 4 ];
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0 ]) );

    /* */

    test.case = `non-empty unroll and length longer`;
    var src = _.unroll.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    test.case = `non-empty unroll and length shorter`;
    var src = _.unroll.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0 ]) );

    /* */

    test.case = `non-empty argumentsArray and length longer`;
    var src = _.argumentsArray.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    test.case = `non-empty argumentsArray and length shorter`;
    var src = _.argumentsArray.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0 ]) );

    /* */

    test.case = `non-empty typed buffer and length longer`;
    var src = _.u8x.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof U8x );
    test.identical( got, _.u8x.make([ 0, 0, 0 ]) );

    test.case = `non-empty typed buffer and length shorter`;
    var src = _.f32x.make([ 3, 4 ]);
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 0 ]) );

    /* */

    test.case = `non-empty raw buffer and length longer`;
    var src = _.u8x.make([ 3, 4 ]).buffer;
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 0, 0, 0 ]).buffer );

    test.case = `non-empty typed buffer and length shorter`;
    var src = _.u8x.make([ 3, 4 ]).buffer;
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 0 ]).buffer );

    /* */

    test.case = `non-empty view buffer and length longer`;
    var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
    var got = long.makeUndefined( src, 3 );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 0, 0, 0 ]).buffer ) );

    test.case = `non-empty view buffer and length shorter`;
    var src = new BufferView( _.u8x.make([ 3, 4 ]).buffer );
    var got = long.makeUndefined( src, 1 );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 0 ]).buffer ) );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `non-empty view buffer and length longer`;
      var src = BufferNode.from([ 3, 4 ]);
      var got = long.makeUndefined( src, 3 );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 0, 0, 0 ]) );

      test.case = `non-empty view buffer and length shorter`;
      var src = BufferNode.from([ 3, 4 ]);
      var got = long.makeUndefined( src, 1 );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 0 ]) );
    }

    /* */

    test.case = `array and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeUndefined( [ 0, 0, 0, 0 ], length );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 0, 0, 0 ]) );

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.type ].buffer;
  }
}

makeUndefinedWithLongDescriptor.timeOut = 30000;

// function bufferMakeUndefinedWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//
//   var list =
//   [
//     array,
//     unroll,
//   ];
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeBuffer )
//   {
//     test.case = 'src - empty long, not ins';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - null';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, null );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - undefined';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, undefined );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - number';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - empty array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, [] );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, [ 1, 2, 3 ] );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, not ins';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMakeUndefined( src );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins < src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins > src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMakeUndefined( src, 4 );
//     var expected = makeBuffer( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - array, ins.length > src.length';
//     var src = makeBuffer( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.bufferMakeUndefined( src, ins );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//   }
//
//   test.case = 'src - null';
//   var got = _.bufferMakeUndefined( null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMakeUndefined( null, null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMakeUndefined( null, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - number';
//   var got = _.bufferMakeUndefined( null, 5 );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - long';
//   var got = _.bufferMakeUndefined( null, new U8x( 5 ) );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - number, ins - null';
//   var got = _.bufferMakeUndefined( 5, null );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - number, ins - undefined';
//   var got = _.bufferMakeUndefined( 5, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - U8x constructor, ins - number';
//   var got = _.bufferMakeUndefined( U8x, 5 );
//   var expected = new U8x( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - F32x constructor, ins - long';
//   var ins = [ 1, 2, 3 ];
//   var got = _.bufferMakeUndefined( F32x, ins );
//   var expected = new F32x( 3 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var got = _.bufferMakeUndefined( Array, 5 );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var ins = [ 1, 1, 1, 1, 1 ];
//   var got = _.bufferMakeUndefined( Array, ins );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//   test.true( got !== ins );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.bufferMakeUndefined( [ 1, 2, 3 ], 4, 'extra' ) );
//
//   test.case = 'wrong type of src';
//   test.shouldThrowErrorSync( () => _.bufferMakeUndefined( undefined ) );
//   test.shouldThrowErrorSync( () => _.bufferMakeUndefined( 'wrong', 1 ) );
//   test.shouldThrowErrorSync( () => _.bufferMakeUndefined( 1, 1 ) );
//
//   test.case = 'wrong type of ins';
//   test.shouldThrowErrorSync( () => _.bufferMakeUndefined( [ 1, 2, 3 ], 'wrong' ) );
//   test.shouldThrowErrorSync( () => _.bufferMakeUndefined( [ 1, 2, 3 ], Infinity  ) );
// }
//
// //
//
// function bufferMakeUndefinedWithArgumentsArray( test )
// {
//   test.case = 'src - empty long, not ins';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMakeUndefined( src );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - null';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMakeUndefined( src, null );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - undefined';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMakeUndefined( src, undefined );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - number';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMakeUndefined( src, 2 );
//   var expected = _.tools.defaultBufferTyped.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - empty array';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMakeUndefined( src, [] );
//   var expected = _.tools.defaultBufferTyped.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - empty long, ins - array';
//   var src = _.argumentsArray.make( [] );
//   var got = _.bufferMakeUndefined( src, [ 1, 2, 3 ] );
//   var expected = _.tools.defaultBufferTyped.make( 3 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, not ins';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.bufferMakeUndefined( src );
//   var expected = _.tools.defaultBufferTyped.make( 3 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, ins - number, ins < src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.bufferMakeUndefined( src, 2 );
//   var expected = _.tools.defaultBufferTyped.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, ins - number, ins > src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.bufferMakeUndefined( src, 4 );
//   var expected = _.tools.defaultBufferTyped.make( 4 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long, ins - array, ins.length > src.length';
//   var src = _.argumentsArray.make( [ 0, 1 ] );
//   var ins = [ 1, 2, 3 ];
//   var got = _.bufferMakeUndefined( src, ins );
//   var expected = _.tools.defaultBufferTyped.make( 3 );
//   test.identical( got, expected );
//   test.true( got !== ins );
//   test.true( got !== src );
//
//   /* */
//
//   test.case = 'src - null';
//   var got = _.bufferMakeUndefined( null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMakeUndefined( null, null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMakeUndefined( null, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - number';
//   var got = _.bufferMakeUndefined( null, 5 );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - long';
//   var got = _.bufferMakeUndefined( null, _.argumentsArray.make( 5 ) );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - number, ins - null';
//   var got = _.bufferMakeUndefined( 5, null );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - number, ins - undefined';
//   var got = _.bufferMakeUndefined( 5, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - U8x constructor, ins - number';
//   var got = _.bufferMakeUndefined( U8x, 5 );
//   var expected = new U8x( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - F32x constructor, ins - long';
//   var ins = _.argumentsArray.make( [ 1, 2, 3 ] )
//   var got = _.bufferMakeUndefined( F32x, ins );
//   var expected = new F32x( 3 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var got = _.bufferMakeUndefined( Array, 5 );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var ins = _.argumentsArray.make( [ 1, 1, 1, 1, 1 ] );
//   var got = _.bufferMakeUndefined( Array, ins );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//   test.true( got !== ins );
// }
//
// //
//
// function bufferMakeUndefinedWithBuffers( test )
// {
//   var bufferTyped = ( buf ) =>
//   {
//     let name = buf.name;
//     return { [ name ] : function( src ){ return new buf( src ) } }[ name ];
//   };
//   var bufferNode = ( src ) => _.number.is( src ) ? BufferNode.alloc( src ) : BufferNode.from( src );
//   var bufferRaw = ( src ) => new U8x( src ).buffer;
//   var bufferView = ( src ) => new BufferView( bufferRaw( src ) );
//
//   /* lists */
//
//   var typedList =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//   var list = [ bufferRaw, bufferView ];
//
//   for( let i = 0; i < typedList.length; i++ )
//   list.push( bufferTyped( typedList[ i ] ) );
//   if( Config.interpreter === 'njs' )
//   list.push( bufferNode );
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeBuffer )
//   {
//     test.case = 'src - empty buffer, not ins';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - null';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, null );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - undefined';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, undefined );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - number';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - empty array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, [] );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - array';
//     var src = makeBuffer( [] );
//     var got = _.bufferMakeUndefined( src, [ 1, 2, 3 ] );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, not ins';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMakeUndefined( src );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - number, ins < src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - number, ins > src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = _.bufferMakeUndefined( src, 4 );
//     var expected = makeBuffer( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - array, ins.length > src.length';
//     var src = makeBuffer( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.bufferMakeUndefined( src, ins );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//   }
//
//   test.case = 'src - null';
//   var got = _.bufferMakeUndefined( null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMakeUndefined( null, null );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - null';
//   var got = _.bufferMakeUndefined( null, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - number';
//   var got = _.bufferMakeUndefined( null, 5 );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - null, ins - long';
//   var got = _.bufferMakeUndefined( null, new U8x( 5 ) );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - number, ins - null';
//   var got = _.bufferMakeUndefined( 5, null );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - number, ins - undefined';
//   var got = _.bufferMakeUndefined( 5, undefined );
//   var expected = _.tools.defaultBufferTyped.make( 5 );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'src - U8x constructor, ins - number';
//   var got = _.bufferMakeUndefined( U8x, 5 );
//   var expected = new U8x( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - F32x constructor, ins - long';
//   var ins = [ 1, 2, 3 ];
//   var got = _.bufferMakeUndefined( F32x, ins );
//   var expected = new F32x( 3 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var got = _.bufferMakeUndefined( Array, 5 );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src - Array constructor, ins - number';
//   var ins = [ 1, 1, 1, 1, 1 ];
//   var got = _.bufferMakeUndefined( Array, ins );
//   var expected = new Array( 5 );
//   test.identical( got, expected );
//   test.true( got !== ins );
// }
//
// //
//
// function bufferMakeUndefinedWithArrayAndUnrollLongDescriptor( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//
//   var list =
//   [
//     array,
//     unroll,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, buffer - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, buffer - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeBuffer )
//   {
//     test.case = 'src - empty long, not ins';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - null';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, null );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - undefined';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, undefined );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - number';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - empty array';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, [] );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - array';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, [ 1, 2, 3 ] );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, not ins';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins < src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins > src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src, 4 );
//     var expected = makeBuffer( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - array, ins.length > src.length';
//     var src = makeBuffer( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMakeUndefined( src, ins );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     /* */
//
//     test.case = 'src - null';
//     var got = long.bufferMakeUndefined( null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMakeUndefined( null, null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMakeUndefined( null, undefined );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - number';
//     var got = long.bufferMakeUndefined( null, 5 );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - long';
//     var got = long.bufferMakeUndefined( null, new U8x( 5 ) );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - number, ins - null';
//     var got = long.bufferMakeUndefined( 5, null );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - number, ins - undefined';
//     var got = long.bufferMakeUndefined( 5, undefined );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - U8x constructor, ins - number';
//     var got = long.bufferMakeUndefined( U8x, 5 );
//     var expected = new U8x( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - F32x constructor, ins - long';
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMakeUndefined( F32x, ins );
//     var expected = new F32x( 3 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var got = long.bufferMakeUndefined( Array, 5 );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var ins = [ 1, 1, 1, 1, 1 ];
//     var got = long.bufferMakeUndefined( Array, ins );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'without arguments';
//       test.shouldThrowErrorSync( () => long.bufferMakeUndefined() );
//
//       test.case = 'extra argument';
//       test.shouldThrowErrorSync( () => long.bufferMakeUndefined( [ 1, 2, 3 ], 4, 'extra' ) );
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long.bufferMakeUndefined( 'wrong', 1 ) );
//       test.shouldThrowErrorSync( () => long.bufferMakeUndefined( 1, 1 ) );
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long.bufferMakeUndefined( [ 1, 2, 3 ], 'wrong' ) );
//       test.shouldThrowErrorSync( () => long.bufferMakeUndefined( [ 1, 2, 3 ], Infinity  ) );
//     }
//   }
// }
//
// bufferMakeUndefinedWithArrayAndUnrollLongDescriptor.timeOut = 15000;
//
// //
//
// function bufferMakeUndefinedWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
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
//     test.case = 'src - empty long, not ins';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMakeUndefined( src );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - null';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMakeUndefined( src, null );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - undefined';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMakeUndefined( src, undefined );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - number';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMakeUndefined( src, 2 );
//     var expected = long.defaultLong.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - empty array';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMakeUndefined( src, [] );
//     var expected = long.defaultLong.make( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty long, ins - array';
//     var src = _.argumentsArray.make( [] );
//     var got = long.bufferMakeUndefined( src, [ 1, 2, 3 ] );
//     var expected = long.defaultLong.make( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, not ins';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src );
//     var expected = long.defaultLong.make( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins < src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src, 2 );
//     var expected = long.defaultLong.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - number, ins > src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src, 4 );
//     var expected = long.defaultLong.make( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long, ins - array, ins.length > src.length';
//     var src = _.argumentsArray.make( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMakeUndefined( src, ins );
//     var expected = long.defaultLong.make( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     /* */
//
//     test.case = 'src - null';
//     var got = long.bufferMakeUndefined( null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMakeUndefined( null, null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMakeUndefined( null, undefined );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - number';
//     var got = long.bufferMakeUndefined( null, 5 );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - long';
//     var got = long.bufferMakeUndefined( null, _.argumentsArray.make( 5 ) );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - number, ins - null';
//     var got = long.bufferMakeUndefined( 5, null );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - number, ins - undefined';
//     var got = long.bufferMakeUndefined( 5, undefined );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - U8x constructor, ins - number';
//     var got = long.bufferMakeUndefined( U8x, 5 );
//     var expected = new U8x( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - F32x constructor, ins - long';
//     var ins = _.argumentsArray.make( [ 1, 2, 3 ] )
//     var got = long.bufferMakeUndefined( F32x, ins );
//     var expected = new F32x( 3 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var got = long.bufferMakeUndefined( Array, 5 );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var ins = _.argumentsArray.make( [ 1, 1, 1, 1, 1 ] );
//     var got = long.bufferMakeUndefined( Array, ins );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//   }
// }
//
// bufferMakeUndefinedWithArgumentsArrayLongDescriptor.timeOut = 10000;
//
// //
//
// function bufferMakeUndefinedWithBuffersLongDescriptor( test )
// {
//   var bufferTyped = ( buf ) =>
//   {
//     let name = buf.name;
//     return { [ name ] : function( src ){ return new buf( src ) } }[ name ];
//   };
//   var bufferNode = ( src ) => _.number.is( src ) ? BufferNode.alloc( src ) : BufferNode.from( src );
//   var bufferRaw = ( src ) => new U8x( src ).buffer;
//   var bufferView = ( src ) => new BufferView( bufferRaw( src ) );
//
//   /* lists */
//
//   var typedList =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//   var list = [ bufferRaw, bufferView ];
//
//   for( let i = 0; i < typedList.length; i++ )
//   list.push( bufferTyped( typedList[ i ] ) );
//   if( Config.interpreter === 'njs' )
//   list.push( bufferNode );
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, long - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, long - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeBuffer )
//   {
//     test.case = 'src - empty buffer, not ins';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - null';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, null );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - undefined';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, undefined );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - number';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - empty array';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, [] );
//     var expected = makeBuffer( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - empty buffer, ins - array';
//     var src = makeBuffer( [] );
//     var got = long.bufferMakeUndefined( src, [ 1, 2, 3 ] );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, not ins';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - number, ins < src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src, 2 );
//     var expected = makeBuffer( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - number, ins > src.length';
//     var src = makeBuffer( [ 1, 2, 3 ] );
//     var got = long.bufferMakeUndefined( src, 4 );
//     var expected = makeBuffer( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled buffer, ins - array, ins.length > src.length';
//     var src = makeBuffer( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMakeUndefined( src, ins );
//     var expected = makeBuffer( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     /* */
//
//     test.case = 'src - null';
//     var got = long.bufferMakeUndefined( null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMakeUndefined( null, null );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - null';
//     var got = long.bufferMakeUndefined( null, undefined );
//     var expected = long.defaultLong.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - number';
//     var got = long.bufferMakeUndefined( null, 5 );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - null, ins - long';
//     var got = long.bufferMakeUndefined( null, new U8x( 5 ) );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - number, ins - null';
//     var got = long.bufferMakeUndefined( 5, null );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - number, ins - undefined';
//     var got = long.bufferMakeUndefined( 5, undefined );
//     var expected = long.defaultLong.make( 5 );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'src - U8x constructor, ins - number';
//     var got = long.bufferMakeUndefined( U8x, 5 );
//     var expected = new U8x( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - F32x constructor, ins - long';
//     var ins = [ 1, 2, 3 ];
//     var got = long.bufferMakeUndefined( F32x, ins );
//     var expected = new F32x( 3 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var got = long.bufferMakeUndefined( Array, 5 );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src - Array constructor, ins - number';
//     var ins = [ 1, 1, 1, 1, 1 ];
//     var got = long.bufferMakeUndefined( Array, ins );
//     var expected = new Array( 5 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//   }
// }
//
// bufferMakeUndefinedWithBuffersLongDescriptor.timeOut = 15000;

//

function makeEmpty( test )
{
  act({ tools : 'default', type : 'Array' });
  act({ tools : 'Array', type : 'Array' });
  act({ tools : 'F32x', type : 'F32x' });

  /* */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = `no args`;
    var got = long.makeEmpty();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `array - filled`;
    var src = [ 2, 2 ];
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `unroll - filled`;
    var src = _.unroll.make([ 2, 2 ]);
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `argumentsArray - filled`;
    var src = _.argumentsArray.make([ 2, 2 ]);
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `typed buffer - filled`;
    var src = _.f32x.make([ 2, 2 ]);
    var got = long.makeEmpty( src );
    test.true( got instanceof F32x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw( 0 );
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    test.case = `raw buffer - filled`;
    var src = _.u8x.make([ 2, 2 ]).buffer;
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw( 0 ) );
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    test.case = `view buffer - filled`;
    var src = new BufferView( _.u8x.make([ 2, 2 ]).buffer );
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long.makeEmpty( src );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 0 );
      test.true( got !== src );

      test.case = `raw buffer - filled`;
      var src = BufferNode.from([ 2, 2 ]);
      var got = long.makeEmpty( src );
      test.true( got instanceof BufferNode );
      test.identical( got.byteLength, 0 );
      test.true( got !== src );
    }

    /* */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.makeEmpty( [], 1 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.makeEmpty( undefined ) );
      test.shouldThrowErrorSync( () => long.makeEmpty( null ) );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.tools ].buffer;
  }
}

//

function makeEmptyWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    act({ tools : 'default', type });
    act({ tools : 'Array', type });
    act({ tools : 'F32x', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = `no args`;
    var got = long.makeEmpty();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `array - filled`;
    var src = [ 2, 2 ];
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `unroll - filled`;
    var src = _.unroll.make([ 2, 2 ]);
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `argumentsArray - filled`;
    var src = _.argumentsArray.make([ 2, 2 ]);
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `typed buffer - filled`;
    var src = _.f32x.make([ 2, 2 ]);
    var got = long.makeEmpty( src );
    test.true( got instanceof F32x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw( 0 );
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    test.case = `raw buffer - filled`;
    var src = _.u8x.make([ 2, 2 ]).buffer;
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferRaw );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw( 0 ) );
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    test.case = `view buffer - filled`;
    var src = new BufferView( _.u8x.make([ 2, 2 ]).buffer );
    var got = long.makeEmpty( src );
    test.true( got instanceof BufferView );
    test.identical( got.byteLength, 0 );
    test.true( got !== src );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long.makeEmpty( src );
      test.true( got instanceof BufferNode );
      test.identical( got.length, 0 );
      test.true( got !== src );

      test.case = `raw buffer - filled`;
      var src = BufferNode.from([ 2, 2 ]);
      var got = long.makeEmpty( src );
      test.true( got instanceof BufferNode );
      test.identical( got.byteLength, 0 );
      test.true( got !== src );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.type ].buffer;
  }
}

//

function makeFilling( test )
{
  act({ tools : 'default', type : 'Array' });
  act({ tools : 'Array', type : 'Array' });
  act({ tools : 'F32x', type : 'F32x' });

  /* */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = 'value - null, length - number';
    var got = long.makeFilling( null, 3 );
    var expected = Constructor.from([ null, null, null ]);
    test.identical( got, expected );

    test.case = `value - number, length - filled array`;
    var length = [ 1, 2, 3 ];
    var got = long.makeFilling( 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = `value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = 'type - null, value - number, length - number';
    var got = long.makeFilling( null, 1, 3 );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - long';
    var got = long.makeFilling( null, 1, new U8x( 3 ) );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - countable';
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( null, 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - U8x constructor, value - number, length - number`;
    var got = long.makeFilling( U8x, 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - BufferRaw constructor, value - number, length - long`;
    var got = long.makeFilling( BufferRaw, 10, new I16x( 3 ) );
    var expected = _.u8x.make([ 10, 10, 10 ]).buffer;
    test.identical( got, expected );

    /* */

    test.case = `type - empty array, value - number, length - number`;
    var got = long.makeFilling( [], 10, 3 );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled array, value - number, length - long`;
    var got = long.makeFilling( [ 1, 2, 3, 4 ], 10, new I16x( 3 ) );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty unroll, value - number, length - number`;
    var got = long.makeFilling( _.unroll.make( [] ), 10, 3 );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled unroll, value - number, length - long`;
    var got = long.makeFilling( _.unroll.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty argumentsArray, value - number, length - number`;
    var got = long.makeFilling( _.argumentsArray.make( [] ), 10, 3 );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled argumentsArray, value - number, length - long`;
    var got = long.makeFilling( _.argumentsArray.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty typed array, value - number, length - number`;
    var got = long.makeFilling( _.u8x.make( [] ), 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled typed array, value - number, length - long`;
    var got = long.makeFilling( _.f32x.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = _.f32x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty raw buffer, value - number, length - number`;
    var got = long.makeFilling( new BufferRaw( 0 ), 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]).buffer;
    test.identical( got, expected );

    test.case = `type - filled raw buffer, value - number, length - long`;
    var got = long.makeFilling( _.u8x.make([ 1, 2, 3, 4 ]).buffer, 10, new I16x( 3 ) );
    var expected = _.u8x.make([ 10, 10, 10 ]).buffer;
    test.identical( got, expected );

    /* */

    test.case = `type - empty view buffer, value - number, length - number`;
    var got = long.makeFilling( new BufferView( new BufferRaw( 0 ) ), 10, 3 );
    var expected = new BufferView( _.u8x.make([ 10, 10, 10 ]).buffer );
    test.identical( got, expected );

    test.case = `type - filled raw buffer, value - number, length - long`;
    var got = long.makeFilling( new BufferView( _.u8x.make([ 1, 2, 3, 4 ]).buffer ), 10, new I16x( 3 ) );
    var expected = new BufferView( _.u8x.make([ 10, 10, 10 ]).buffer );
    test.identical( got, expected );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `type - empty node buffer, value - number, length - number`;
      var got = long.makeFilling( BufferNode.alloc( 0 ), 10, 3 );
      var expected = BufferNode.from([ 10, 10, 10 ]);
      test.identical( got, expected );

      test.case = `type - filled raw buffer, value - number, length - long`;
      var got = long.makeFilling( BufferNode.from([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
      var expected = BufferNode.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty typed array, value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( _.u8x.make( [] ), 10, length );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.makeFilling() );

      test.case = 'not enough arguments';
      test.shouldThrowErrorSync( () => long.makeFilling( null ) );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2, 3 ], 4, 4, 'extra' ) );

      test.case = 'wrong type of type argument';
      test.shouldThrowErrorSync( () => long.makeFilling( {}, 2, 2 ) );
      test.shouldThrowErrorSync( () => long.makeFilling( undefined, 2, 2 ) );

      test.case = 'wrong type of value';
      test.shouldThrowErrorSync( () => long.makeFilling( undefined, 1 ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [], undefined, 1 ) );

      test.case = 'wrong type of length';
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], 'wrong' ) );
      test.shouldThrowErrorSync( () => long.makeFilling( 1, 'wrong' ) );
      test.shouldThrowErrorSync( () => long.makeFilling( Array, 'wrong' ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], null ) );
      test.shouldThrowErrorSync( () => long.makeFilling( 1, null ) );
      test.shouldThrowErrorSync( () => long.makeFilling( Array, null ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], undefined ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], 2, undefined ) );
    }

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.tools ].buffer;
  }
}

//

function makeFillingWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    act({ tools : 'default', type });
    act({ tools : 'Array', type });
    act({ tools : 'F32x', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = 'value - null, length - number';
    var got = long.makeFilling( null, 3 );
    var expected = Constructor.from([ null, null, null ]);
    test.identical( got, expected );

    test.case = `value - number, length - filled array`;
    var length = [ 1, 2, 3 ];
    var got = long.makeFilling( 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = `value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = 'type - null, value - number, length - number';
    var got = long.makeFilling( null, 1, 3 );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - long';
    var got = long.makeFilling( null, 1, new U8x( 3 ) );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - countable';
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( null, 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - U8x constructor, value - number, length - number`;
    var got = long.makeFilling( U8x, 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - BufferRaw constructor, value - number, length - long`;
    var got = long.makeFilling( BufferRaw, 10, new I16x( 3 ) );
    var expected = _.u8x.make([ 10, 10, 10 ]).buffer;
    test.identical( got, expected );

    /* */

    test.case = `type - empty array, value - number, length - number`;
    var got = long.makeFilling( [], 10, 3 );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled array, value - number, length - long`;
    var got = long.makeFilling( [ 1, 2, 3, 4 ], 10, new I16x( 3 ) );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty unroll, value - number, length - number`;
    var got = long.makeFilling( _.unroll.make( [] ), 10, 3 );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled unroll, value - number, length - long`;
    var got = long.makeFilling( _.unroll.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty argumentsArray, value - number, length - number`;
    var got = long.makeFilling( _.argumentsArray.make( [] ), 10, 3 );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled argumentsArray, value - number, length - long`;
    var got = long.makeFilling( _.argumentsArray.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = Constructor.from([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty typed array, value - number, length - number`;
    var got = long.makeFilling( _.u8x.make( [] ), 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled typed array, value - number, length - long`;
    var got = long.makeFilling( _.f32x.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = _.f32x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty raw buffer, value - number, length - number`;
    var got = long.makeFilling( new BufferRaw( 0 ), 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]).buffer;
    test.identical( got, expected );

    test.case = `type - filled raw buffer, value - number, length - long`;
    var got = long.makeFilling( _.u8x.make([ 1, 2, 3, 4 ]).buffer, 10, new I16x( 3 ) );
    var expected = _.u8x.make([ 10, 10, 10 ]).buffer;
    test.identical( got, expected );

    /* */

    test.case = `type - empty view buffer, value - number, length - number`;
    var got = long.makeFilling( new BufferView( new BufferRaw( 0 ) ), 10, 3 );
    var expected = new BufferView( _.u8x.make([ 10, 10, 10 ]).buffer );
    test.identical( got, expected );

    test.case = `type - filled raw buffer, value - number, length - long`;
    var got = long.makeFilling( new BufferView( _.u8x.make([ 1, 2, 3, 4 ]).buffer ), 10, new I16x( 3 ) );
    var expected = new BufferView( _.u8x.make([ 10, 10, 10 ]).buffer );
    test.identical( got, expected );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `type - empty node buffer, value - number, length - number`;
      var got = long.makeFilling( BufferNode.alloc( 0 ), 10, 3 );
      var expected = BufferNode.from([ 10, 10, 10 ]);
      test.identical( got, expected );

      test.case = `type - filled raw buffer, value - number, length - long`;
      var got = long.makeFilling( BufferNode.from([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
      var expected = BufferNode.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty typed array, value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( _.u8x.make( [] ), 10, length );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.type ].buffer;
  }
}

//

function from( test )
{
  act({ tools : 'default', type : 'Array' });
  act({ tools : 'Array', type : 'Array' });
  act({ tools : 'F32x', type : 'F32x' });

  /* - */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = 'null';
    var got = long.from( null );
    test.identical( got, Constructor.from( [] ) );
    test.true( got instanceof Constructor );

    test.case = 'number';
    var src = 2;
    var got = long.from( src );
    var value = _.buffer.typedIs( Constructor.from( [] ) ) ? 0 : undefined;
    test.identical( got, Constructor.from([ value, value ]) );
    test.true( got instanceof Constructor );

    /* */

    test.case = 'empty array';
    var src = [];
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from( [] ) );
    test.true( got !== src );


    test.case = 'filled array';
    var src = [ 1, 2, 3 ];
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    test.true( got !== src );

    /* */

    test.case = 'empty unroll';
    var src = _.unroll.make( [] );
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from( [] ) );
    test.true( got !== src );

    test.case = 'filled unroll';
    var src = _.unroll.make([ 1, 2, 3 ]);
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    test.true( got !== src );

    /* */

    test.case = 'empty argumentsArray';
    var src = _.argumentsArray.make( [] );
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from( [] ) );
    test.true( got !== src );

    test.case = 'filled argumentsArray';
    var src = _.argumentsArray.make([ 1, 2, 3 ]);
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    test.true( got !== src );

    /* */

    test.case = 'empty BufferTyped';
    var src = new U8x( [] );
    var got = long.from( src );
    test.identical( got, new U8x( [] ) );
    test.true( _.u8x.is( got ) );
    test.true( got === src );

    var src = new I16x( [] );
    var got = long.from( src );
    test.identical( got, new I16x( [] ) );
    test.true( _.i16x.is( got ) );
    test.true( got === src );

    test.case = 'filled BufferTyped';
    var src = new F32x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F32x([ 1, 2, 3 ]) );
    test.true( _.f32x.is( got ) );
    test.true( got === src );

    var src = new F64x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F64x([ 1, 2, 3 ]) );
    test.true( _.f64x.is( got ) );
    test.true( got === src );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw( 0 );
    var got = long.from( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, new BufferRaw( 0 ) );
    test.true( got === src );

    test.case = `raw buffer - filled`;
    var src = _.u8x.make([ 2, 2 ]).buffer;
    var got = long.from( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 2, 2 ]).buffer );
    test.true( got === src );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw( 0 ) );
    var got = long.from( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( new BufferRaw( 0 ) ) );
    test.true( got === src );

    test.case = `view buffer - filled`;
    var src = new BufferView( _.u8x.make([ 2, 2 ]).buffer );
    var got = long.from( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 2, 2 ]).buffer ) );
    test.true( got === src );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long.from( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.alloc( 0 ) );
      test.true( got === src );

      test.case = `raw buffer - filled`;
      var src = BufferNode.from([ 2, 2 ]);
      var got = long.from( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 2, 2 ]) );
      test.true( got === src );
    }

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.from() );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.from( 1, [] ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.from( 'str' ) );
      test.shouldThrowErrorSync( () => long.from( { 1 : 2 } ) );
    }

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.tools ].buffer;
  }
}

//

function fromWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    act({ tools : 'default', type });
    act({ tools : 'Array', type });
    act({ tools : 'F32x', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = _.bufferTyped.default.InstanceConstructor;

    /* */

    test.case = 'null';
    var got = long.from( null );
    test.identical( got, Constructor.from( [] ) );
    test.true( got instanceof Constructor );

    test.case = 'number';
    var src = 2;
    var got = long.from( src );
    var value = _.buffer.typedIs( Constructor.from( [] ) ) ? 0 : undefined;
    test.identical( got, Constructor.from([ value, value ]) );
    test.true( got instanceof Constructor );

    /* */

    test.case = 'empty array';
    var src = [];
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from( [] ) );
    test.true( got !== src );


    test.case = 'filled array';
    var src = [ 1, 2, 3 ];
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    test.true( got !== src );

    /* */

    test.case = 'empty unroll';
    var src = _.unroll.make( [] );
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from( [] ) );
    test.true( got !== src );

    test.case = 'filled unroll';
    var src = _.unroll.make([ 1, 2, 3 ]);
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    test.true( got !== src );

    /* */

    test.case = 'empty argumentsArray';
    var src = _.argumentsArray.make( [] );
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from( [] ) );
    test.true( got !== src );

    test.case = 'filled argumentsArray';
    var src = _.argumentsArray.make([ 1, 2, 3 ]);
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 1, 2, 3 ]) );
    test.true( got !== src );

    /* */

    test.case = 'empty BufferTyped';
    var src = new U8x( [] );
    var got = long.from( src );
    test.identical( got, new U8x( [] ) );
    test.true( _.u8x.is( got ) );
    test.true( got === src );

    var src = new I16x( [] );
    var got = long.from( src );
    test.identical( got, new I16x( [] ) );
    test.true( _.i16x.is( got ) );
    test.true( got === src );

    test.case = 'filled BufferTyped';
    var src = new F32x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F32x([ 1, 2, 3 ]) );
    test.true( _.f32x.is( got ) );
    test.true( got === src );

    var src = new F64x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F64x([ 1, 2, 3 ]) );
    test.true( _.f64x.is( got ) );
    test.true( got === src );

    /* */

    test.case = `empty raw buffer`;
    var src = new BufferRaw( 0 );
    var got = long.from( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, new BufferRaw( 0 ) );
    test.true( got === src );

    test.case = `raw buffer - filled`;
    var src = _.u8x.make([ 2, 2 ]).buffer;
    var got = long.from( src );
    test.true( got instanceof BufferRaw );
    test.identical( got, _.u8x.make([ 2, 2 ]).buffer );
    test.true( got === src );

    /* */

    test.case = `empty view buffer`;
    var src = new BufferView( new BufferRaw( 0 ) );
    var got = long.from( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( new BufferRaw( 0 ) ) );
    test.true( got === src );

    test.case = `view buffer - filled`;
    var src = new BufferView( _.u8x.make([ 2, 2 ]).buffer );
    var got = long.from( src );
    test.true( got instanceof BufferView );
    test.identical( got, new BufferView( _.u8x.make([ 2, 2 ]).buffer ) );
    test.true( got === src );

    /* */

    if( Config.interpreter === 'njs' )
    {
      test.case = `empty node buffer`;
      var src = BufferNode.alloc( 0 );
      var got = long.from( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.alloc( 0 ) );
      test.true( got === src );

      test.case = `raw buffer - filled`;
      var src = BufferNode.from([ 2, 2 ]);
      var got = long.from( src );
      test.true( got instanceof BufferNode );
      test.identical( got, BufferNode.from([ 2, 2 ]) );
      test.true( got === src );
    }

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.buffer;
    return _.withLong[ env.type ].buffer;
  }
}

//

function bufferRawFromTyped( test )
{

  var buffer1 = new BufferRaw(10);
  var view1 = new I8x( buffer1 );
  test.case = 'returns the same length of typed array';
  var got = _.bufferRawFromTyped( view1 );
  var expected = got; // [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
  test.identical( got, expected );

  var buffer2 = new BufferRaw(10);
  var view2 = new I8x( buffer2, 2 );
  test.case = 'returns the new sub typed array';
  var got = _.bufferRawFromTyped( view2 );
  var expected = got; // [ 0, 0, 0, 0, 0, 0 ]
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.bufferRawFromTyped();
  });

  test.case = 'arguments are wrong';
  test.shouldThrowErrorSync( function()
  {
    _.bufferRawFromTyped( 'wrong argument' );
  });

}

//

function bufferRawFrom( test )
{
  test.case = 'from array';
  var src = [ 1, 2, 3, 4, 5 ];
  var got = _.bufferRawFrom( src );
  var expected = new U8x( src ).buffer;
  test.identical( got, expected );
  test.true( got !== src );

  var src = [ 1, 2, 3, 4, '5' ];
  var got = _.bufferRawFrom( src );
  var expected = new U8x( src ).buffer;
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  test.case = 'from BufferRaw';
  var src = new BufferRaw( 3 );
  var got = _.bufferRawFrom( src );
  var expected = new BufferRaw( 3 );
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'from BufferTyped.buffer';
  var src = new F32x( 3 ).buffer;
  var got = _.bufferRawFrom( src );
  var expected = new BufferRaw( 12 );
  test.identical( got, expected );
  test.true( got === src );

  /* */

  if( Config.interpreter === 'njs' )
  {
    test.case = 'empty BufferNode';
    var src1 = BufferNode.alloc( 10 );
    var got1 = _.bufferRawFrom( src1 );
    var expected1 = new BufferRaw( 10 );
    test.identical( got1, expected1 );
    test.true( got1 !== src1 );

    test.case = 'BufferNode from string';
    var src1 = BufferNode.from( 'str' );
    var got1 = _.bufferRawFrom( src1 );
    var expected1 = new U8x( [ 115, 116, 114 ] ).buffer;
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );

    test.case = 'BufferNode, buffer.length < result.length';
    var buffer1 = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
    var src1 = BufferNode.from( buffer1, 0, 3 );
    var got1 = _.bufferRawFrom( src1 );
    var expected1 = new U8x( [ 1, 2, 3 ] ).buffer;
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );

    test.case = 'BufferNode, offset from src1';
    var buffer1 = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
    var src1 = BufferNode.from( buffer1, 3 );
    var got1 = _.bufferRawFrom( src1 );
    var expected1 = new U8x( [ 4, 5 ] ).buffer;
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );

    test.case = 'BufferNode, offset from src1, buffer.length < result.length';
    var buffer1 = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
    var src1 = BufferNode.from( buffer1, 2, 2 );
    var got1 = _.bufferRawFrom( src1 );
    var expected1 = new U8x( [ 3, 4 ] ).buffer;
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );
  }

  /* */

  test.case = 'from BufferTyped, not offset';
  var src = new U8x( 3 );
  var got = _.bufferRawFrom( src );
  var expected = new BufferRaw( 3 );
  test.identical( got, expected );
  test.true( got === src.buffer );

  test.case = 'from BufferTyped, offset';
  var buffer = new F32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new I16x( buffer, 8 );
  var got = _.bufferRawFrom( src );
  var expected = new F32x( [ 3, 4, 5 ] ).buffer;
  test.identical( got, expected );
  test.true( got !== src.buffer );

  test.case = 'from BufferTyped, not offset, buffer.length < result.length';
  var buffer = new U16x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new F32x( buffer, 0, 2 );
  var got = _.bufferRawFrom( src );
  var expected = new U8x( [ 1, 0, 2, 0, 3, 0, 4, 0 ] ).buffer;
  test.identical( got, expected );
  test.true( got !== src.buffer );

  test.case = 'from BufferTyped, offset, buffer.length < result.length';
  var buffer = new U32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new U8x( buffer, 4, 4 );
  var got = _.bufferRawFrom( src );
  var expected = new U8x( [ 2, 0, 0, 0 ] ).buffer;
  test.identical( got, expected );
  test.true( got !== src.buffer );

  /* */

  test.case = 'from BufferView, not offset';
  var src = new BufferView( new BufferRaw( 10 ) );
  var got = _.bufferRawFrom( src );
  var expected = new BufferRaw( 10 );
  test.identical( got, expected );
  test.true( got === src.buffer );

  test.case = 'from BufferView, not offset, buffer.length < result.length';
  var buffer = new F32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new BufferView( buffer, 0, 4 );
  var got = _.bufferRawFrom( src );
  var expected = new F32x( [ 1 ] ).buffer;
  test.identical( got, expected );
  test.true( got !== src.buffer );

  test.case = 'from BufferView, offset';
  var buffer = new U32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new BufferView( buffer, 4, 4 );
  var got = _.bufferRawFrom( src );
  var expected = new U8x( [ 2, 0, 0, 0 ] ).buffer;
  test.identical( got, expected );
  test.true( got !== src.buffer );

  /* */

  test.case = 'from string';
  var src = 'abc';
  var got = _.bufferRawFrom( src );
  var expected = new U8x( [ 97, 98, 99 ] ).buffer;
  test.identical( got, expected );

  /*
  zzz aaa Dmytro: making buffer from file need tests, now condition has _.assert( 0, 'not tested' )
  */

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.bufferRawFrom() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.bufferRawFrom( [ 1, 2 ], 2 ) );

  test.case = 'wrong buffer type';
  test.shouldThrowErrorSync( () => _.bufferRawFrom( 5 ) );
  test.shouldThrowErrorSync( () => _.bufferRawFrom( {} ) );
}

//

function bufferBytesFrom( test )
{
  test.case = 'from array';
  var src = [ 1, 2, 3, 4, 5 ];
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( src );
  test.identical( got, expected );
  test.true( got !== src );

  var src = [ 1, 2, 3, 4, '5' ];
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( src );
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  test.case = 'from BufferRaw';
  var src = new BufferRaw( 3 );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( 3 );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferTyped.buffer';
  var src = new F32x( 3 ).buffer;
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( 12 );
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  if( Config.interpreter === 'njs' )
  {
    test.case = 'empty BufferNode';
    var src1 = BufferNode.alloc( 10 );
    var got1 = _.bufferBytesFrom( src1 );
    var expected1 = new U8x( 10 );
    test.identical( got1, expected1 );
    test.true( got1 !== src1 );

    test.case = 'BufferNode from string';
    var src1 = BufferNode.from( 'str' );
    var got1 = _.bufferBytesFrom( src1 );
    var expected1 = new U8x( [ 115, 116, 114 ] );
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );

    test.case = 'BufferNode, buffer.length < result.length';
    var buffer1 = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
    var src1 = BufferNode.from( buffer1, 0, 3 );
    var got1 = _.bufferBytesFrom( src1 );
    var expected1 = new U8x( [ 1, 2, 3 ] );
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );

    test.case = 'BufferNode, offset from src1';
    var buffer1 = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
    var src1 = BufferNode.from( buffer1, 3 );
    var got1 = _.bufferBytesFrom( src1 );
    var expected1 = new U8x( [ 4, 5 ] );
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );

    test.case = 'BufferNode, offset from src1, buffer.length < result.length';
    var buffer1 = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
    var src1 = BufferNode.from( buffer1, 2, 2 );
    var got1 = _.bufferBytesFrom( src1 );
    var expected1 = new U8x( [ 3, 4 ] );
    test.identical( got1, expected1 );
    test.true( got1 !== expected1 );
  }

  /* */

  test.case = 'from BufferBytes';
  var src = new U8x( 3 );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( 3 );
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'from BufferBytes, not offset';
  var src = new U8x( [ 1, 2, 3, 4, 5 ] );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( [ 1, 2, 3, 4, 5 ] );
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'from BufferTyped, offset';
  var buffer = new F32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new I16x( buffer, 8 );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( [ 0, 0, 64, 64, 0, 0, 128, 64, 0, 0, 160, 64 ] );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferTyped, not offset, buffer.length < result.length';
  var buffer = new U16x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new F32x( buffer, 0, 2 );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( [ 1, 0, 2, 0, 3, 0, 4, 0 ] );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferTyped, offset, buffer.length < result.length';
  var buffer = new U32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new U8x( buffer, 4, 4 );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( [ 2, 0, 0, 0 ] );
  test.identical( got, expected );
  test.true( got === src );

  /* */

  test.case = 'from BufferView, not offset';
  var src = new BufferView( new BufferRaw( 10 ) );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( 10 );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferView, not offset, buffer.length < result.length';
  var buffer = new F32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new BufferView( buffer, 0, 4 );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( [ 0, 0, 128, 63 ] );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferView, offset';
  var buffer = new U32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new BufferView( buffer, 4, 4 );
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( [ 2, 0, 0, 0 ] );
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  test.case = 'from string';
  var src = 'abc';
  var got = _.bufferBytesFrom( src );
  var expected = new U8x( [ 97, 98, 99 ] );
  test.identical( got, expected );

  /*
  zzz aaa Dmytro: making buffer from file need tests, now condition has _.assert( 0, 'not tested' )
  */

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.bufferBytesFrom() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.bufferBytesFrom( [ 1, 2 ], 2 ) );

  test.case = 'wrong buffer type';
  test.shouldThrowErrorSync( () => _.bufferBytesFrom( 5 ) );
  test.shouldThrowErrorSync( () => _.bufferBytesFrom( {} ) );
}

//

function bufferNodeFrom( test )
{
  if( Config.interpreter !== 'njs' )
  return;

  test.case = 'from array';
  var src = [ 1, 2, 3, 4, 5 ];
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( src );
  test.identical( got, expected );
  test.true( got !== src );

  var src = [ 1, 2, 3, 4, '5' ];
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( src );
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  test.case = 'from BufferRaw, buffer.byteLength = 0';
  var src = new BufferRaw();
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.alloc( 0 );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferRaw';
  var src = new BufferRaw( 3 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.alloc( 3 );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferTyped.buffer';
  var src = new F32x( 3 ).buffer;
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.alloc( 12 );
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  test.case = 'empty BufferNode';
  var src = BufferNode.alloc( 10 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.alloc( 10 );
  test.identical( got, expected );
  test.true( got === src );

  test.case = 'BufferNode from string';
  var src = BufferNode.from( 'str' );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 115, 116, 114 ] );
  test.identical( got, expected );
  test.true( got !== expected );

  test.case = 'BufferNode, buffer.length < result.length';
  var buffer = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = BufferNode.from( buffer, 0, 3 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 1, 2, 3 ] );
  test.identical( got, expected );
  test.true( got !== expected );

  test.case = 'BufferNode, offset from src';
  var buffer = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = BufferNode.from( buffer, 3 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 4, 5 ] );
  test.identical( got, expected );
  test.true( got !== expected );

  test.case = 'BufferNode, offset from src, buffer.length < result.length';
  var buffer = new U8x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = BufferNode.from( buffer, 2, 2 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 3, 4 ] );
  test.identical( got, expected );
  test.true( got !== expected );

  /* */

  test.case = 'from empty BufferBytes';
  var src = new U8x();
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.alloc( 0 );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferBytes';
  var src = new U8x( 3 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.alloc( 3 );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferBytes, not offset';
  var src = new U8x( [ 1, 2, 3, 4, 5 ] );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 1, 2, 3, 4, 5 ] );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferTyped, offset';
  var buffer = new F32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new I16x( buffer, 8 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 0, 0, 64, 64, 0, 0, 128, 64, 0, 0, 160, 64 ] );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferTyped, not offset, buffer.length < result.length';
  var buffer = new U16x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new F32x( buffer, 0, 2 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 1, 0, 2, 0, 3, 0, 4, 0 ] );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferTyped, offset, buffer.length < result.length';
  var buffer = new U32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new U8x( buffer, 4, 4 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 2, 0, 0, 0 ] );
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  test.case = 'from BufferView, not offset';
  var src = new BufferView( new BufferRaw( 10 ) );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.alloc( 10 );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferView, not offset, buffer.length < result.length';
  var buffer = new F32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new BufferView( buffer, 0, 4 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 0, 0, 128, 63 ] );
  test.identical( got, expected );
  test.true( got !== src );

  test.case = 'from BufferView, offset';
  var buffer = new U32x( [ 1, 2, 3, 4, 5 ] ).buffer;
  var src = new BufferView( buffer, 4, 4 );
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 2, 0, 0, 0 ] );
  test.identical( got, expected );
  test.true( got !== src );

  /* */

  test.case = 'from string';
  var src = 'abc';
  var got = _.bufferNodeFrom( src );
  var expected = BufferNode.from( [ 97, 98, 99 ] );
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.bufferNodeFrom() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.bufferNodeFrom( [ 1, 2 ], 2 ) );

  test.case = 'wrong buffer type';
  test.shouldThrowErrorSync( () => _.bufferNodeFrom( 5 ) );
  test.shouldThrowErrorSync( () => _.bufferNodeFrom( {} ) );

}

//

function isPerformance( test )
{
  /* Average of 10 runs of 5 million iterations of 28 input variants
     Values below are in seconds
  ╔═══════════════════╤════════╤════════════════════╗
  ║                   │anyIsOld│anyIsUsingInstanceOf║
  ╟───────────────────┼────────┼────────────────────╢
  ║ **Njs : v10.24.1**│8.575   │        6.500       ║
  ╟───────────────────┼────────┼────────────────────╢
  ║ **Njs : v14.17.0**│8.427   │        5.661       ║
  ╟───────────────────┼────────┼────────────────────╢
  ║Kos : Njs : v12.9.1│8.857   │        6.227       ║
  ╚═══════════════════╧════════╧════════════════════╝
  */

  debugger; /* eslint-disable-line no-debugger */
  var debugFlag = Config.debug;
  Config.debug = false;

  /* */

  isPerformanceTemplate( { method : 'anyIs' } );
  // isPerformanceTemplate( { method : 'anyIsOld' } );
  // isPerformanceTemplate( { method : 'anyIsUsingInstanceOf' } );

  /* */

  Config.debug = debugFlag;
  debugger; /* eslint-disable-line no-debugger */

  /* */

  function isPerformanceTemplate( data )
  {
    test.case = `${data.method}`;
    var took, time;
    var env = initializeVariables();

    time = _.time.now();
    for( let i = env.times; i > 0; i-- )
    {
      env.name = data.method;
      run( env );
    }
    took = __.time.spent( time );

    console.log( `${env.times} iterations of ${test.case} took : ${took} on ${process.version}` );
    test.identical( true, true );
  }

  /* */

  function initializeVariables()
  {
    var env = {};
    env.times = 5000000;
    env.aSymbol = Symbol( 'a' );
    env.anEmptyMap = {};
    env.anEmptyArray = [];
    env.aString = 'string';
    env.U8xconstructor = U8x;
    env.emptyU8xInstance = new U8x( 5 );
    env.emptyU16xInstance = new U16x( 5 );
    env.emptyU32xInstance = new U32x( 5 );
    env.emptyU64xInstance = new U64x( 5 );
    env.emptyI8xInstance = new I8x( 5 );
    env.emptyI16xInstance = new I16x( 5 );
    env.emptyI32xInstance = new I32x( 5 );
    env.emptyI64xInstance = new I64x( 5 );
    env.emptyF32xInstance = new F32x( 5 );
    if( Config.interpreter === 'njs' )
    {
      env.emptyBufferNodeInstance = BufferNode.alloc( 5 );
      env.BufferNodeInstance = BufferNode.allocUnsafe( 5 );
      env.BufferNodeFromString = BufferNode.from( 'str' );
    }
    env.BufferRawconstructor = BufferRaw;
    env.emptyBufferRawInstance = new BufferRaw( 5 );
    env.BufferRawSharedInstance = new BufferRawShared( 5 );
    env.BufferViewConstructor = BufferView;

    return env;
  }

  /* */

  function run( env )
  {
    _.buffer[ env.name ]( null );
    _.buffer[ env.name ]( undefined );
    _.buffer[ env.name ]( 0 );
    _.buffer[ env.name ]( false );
    _.buffer[ env.name ]( NaN );
    _.buffer[ env.name ]( env.aSymbol );
    _.buffer[ env.name ]( env.anEmptyMap );
    _.buffer[ env.name ]( env.anEmptyArray );
    _.buffer[ env.name ]( env.aString );
    _.buffer[ env.name ]( env.U8xconstructor );
    _.buffer[ env.name ]( env.emptyU8xInstance );
    _.buffer[ env.name ]( env.emptyU16xInstance );
    _.buffer[ env.name ]( env.emptyU32xInstance );
    _.buffer[ env.name ]( env.emptyU64xInstance );
    _.buffer[ env.name ]( env.emptyI8xInstance );
    _.buffer[ env.name ]( env.emptyI16xInstance );
    _.buffer[ env.name ]( env.emptyI32xInstance );
    _.buffer[ env.name ]( env.emptyI64xInstance );
    _.buffer[ env.name ]( env.emptyF32xInstance );
    if( Config.interpreter === 'njs' )
    {
      _.buffer[ env.name ]( env.BufferNodeInstance );
      _.buffer[ env.name ]( env.BufferNodeInstance );
      _.buffer[ env.name ]( env.BufferNodeFromString );
    }
    _.buffer[ env.name ]( env.BufferRawconstructor );
    _.buffer[ env.name ]( env.emptyBufferRawInstance );
    _.buffer[ env.name ]( env.BufferRawSharedInstance );
    _.buffer[ env.name ]( env.BufferViewConstructor );
    _.buffer[ env.name ]();
    _.buffer[ env.name ]( new U8x( [ 1, 2, 3 ] ), new BufferRaw( 5 ) );
  }
}

isPerformance.timeOut = 1e7;
isPerformance.experimental = true;

// --
// declaration
// --

const Proto =
{

  name : 'Tools.Buffer.l0.l1',
  silencing : 1,
  enabled : 1,
  routineTimeOut : 20000,

  tests :
  {

    // buffer, l0/l5

    bufferRawIs,
    bufferTypedIs,
    bufferViewIs,
    bufferNodeIs,
    bufferAnyIs,
    isPerformance,
    bufferBytesIs,
    // constructorIsBuffer,

    // buffer, l0/l8

    makeCommon,
    makeCommonWithLongDescriptor,
    // bufferMakeWithArrayAndUnroll, /* Dmytro : all coverage in routines  makeCommon* */
    // bufferMakeWithArgumentsArray, /* Dmytro : all coverage in routines  makeCommon* */
    // bufferMakeWithBuffers, /* Dmytro : all coverage in routines  makeCommon* */
    // bufferMakeWithArrayAndUnrollLongDescriptor, /* aaa2 : for Dmytro : fix pelase */ /* Dmytro : fixed, all coverage in routines  makeCommon* */
    // bufferMakeWithArgumentsArrayLongDescriptor, /* aaa2 : for Dmytro : fix pelase */ /* Dmytro : fixed, all coverage in routines  makeCommon* */
    // bufferMakeWithBuffersLongDescriptor, /* aaa2 : for Dmytro : fix pelase */ /* Dmytro : fixed, all coverage in routines  makeCommon* */

    makeUndefined,
    makeUndefinedWithLongDescriptor,
    // bufferMakeUndefinedWithArrayAndUnroll, /* Dmytro : all coverage in routines  makeUndefined* */
    // bufferMakeUndefinedWithArgumentsArray, /* Dmytro : all coverage in routines  makeUndefined* */
    // bufferMakeUndefinedWithBuffers, /* Dmytro : all coverage in routines  makeUndefined* */
    // bufferMakeUndefinedWithArrayAndUnrollLongDescriptor, /* aaa2 : for Dmytro : fix pelase */ /* Dmytro : fixed, all coverage in routines  makeUndefined* */
    // bufferMakeUndefinedWithArgumentsArrayLongDescriptor, /* aaa2 : for Dmytro : fix pelase */ /* Dmytro : fixed, all coverage in routines  makeUndefined* */
    // bufferMakeUndefinedWithBuffersLongDescriptor, /* aaa2 : for Dmytro : fix pelase */ /* Dmytro : fixed, all coverage in routines  makeUndefined* */

    makeEmpty,
    makeEmptyWithLongDescriptor,

    makeFilling,
    makeFillingWithLongDescriptor,

    from,
    fromWithLongDescriptor,

    // bufferCoerceFrom,
    bufferRawFromTyped,
    bufferRawFrom,
    bufferBytesFrom,
    bufferNodeFrom,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
