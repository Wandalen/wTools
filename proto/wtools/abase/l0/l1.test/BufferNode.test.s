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

function bufferNodeIs( test )
{
  if( !Config.interpreter === 'njs' )
  return;

  act({ method : 'nodeIs' });
  act({ method : 'nodeIsOLd' });
  act({ method : 'nodeIsUsingFunctor' });

  function act( env )
  {
    test.case = `${__.entity.exportStringSolo( env )}, node buffer`;
    var src = BufferNode.alloc( 10 );
    var got =  _.bufferNode[ env.method ]( src );
    test.identical( got, true );

    test.case = `${__.entity.exportStringSolo( env )}, buffer node from array`;
    var src = BufferNode.from( [ 2, 4 ] );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, true );

    test.case = `${__.entity.exportStringSolo( env )}, extra arguments`;
    var src = BufferNode.from( [ 1, 2 ] );
    var got = _.bufferNode[ env.method ]( src, new SharedArrayBuffer( 1 ) );
    test.identical( got, true );

    test.case = `${__.entity.exportStringSolo( env )}, node buffer with chracter encoding`;
    var src = BufferNode.from( 'tést', 'latin1' );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, true );

    test.case = `${__.entity.exportStringSolo( env )}, Uninitialized node buffer`;
    var src = BufferNode.allocUnsafe( 10 );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, true );

    test.case = `${__.entity.exportStringSolo( env )}, Node Buffer using truncated bytes`;
    var src = BufferNode.from( [ 257, 257.5, -255, '1' ] );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, true );

    test.case = `${__.entity.exportStringSolo( env )}, typed array`;
    var src = new F32x( [ 1, 2 ] );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, false );

    test.case = `${__.entity.exportStringSolo( env )}, U8xClamped Array`;
    var src = new U8xClamped( 10*10*4 );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, false );

    test.case = `${__.entity.exportStringSolo( env )}, array buffer`;
    var src = new BufferRaw( [ 1, 2 ] );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, false );

    test.case = `${__.entity.exportStringSolo( env )}, shared array buffer`;
    var src = new SharedArrayBuffer( [ 1, 2 ] );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, false );

    test.case = `${__.entity.exportStringSolo( env )}, view buffer, BufferRaw`;
    var src = new BufferView( new BufferRaw( [ 10 ] ) );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, false );

    test.case = `${__.entity.exportStringSolo( env )}, view buffer, SharedArrayBuffer`;
    var src = new BufferView( new SharedArrayBuffer( [ 10 ] ) );
    var got = _.bufferNode[ env.method ]( src );
    test.identical( got, false );

    test.case = `${__.entity.exportStringSolo( env )}, no arguments`;
    var got = _.bufferNode[ env.method ]();
    test.identical( got, false );
  }
}

//

function bufferNodeIsPerformance( test )
{
  /* Average of 10 runs of 5 million iterations of 14 input variants
     Values below are in seconds
  ╔════════════════════════╤══════╤═════════╤══════════════════╗
  ║                        │nodeIs│nodeIsOld│nodeIsUsingFunctor║
  ╟────────────────────────┼──────┼─────────┼──────────────────╢
  ║Windows-10-20H2, 10.24.1│ 2.728│  2.506  │       2.687      ║
  ╟────────────────────────┼──────┼─────────┼──────────────────╢
  ║Windows-10-20H2, 14.17.0│ 3.007│  2.889  │       2.910      ║
  ╟────────────────────────┼──────┼─────────┼──────────────────╢
  ║    Linux-Kos, 12.9.1   │      │         │                  ║
  ╚════════════════════════╧══════╧═════════╧══════════════════╝
  */

  if( !Config.interpreter === 'njs' )
  return;

  debugger; /* eslint-disable-line no-debugger */
  var debugFlag = Config.debug;
  Config.debug = false;

  /* */

  act({ method : 'nodeIs' });
  act({ method : 'nodeIsOLd' });
  act({ method : 'nodeIsUsingFunctor' });

  /* */

  Config.debug = debugFlag;
  debugger; /* eslint-disable-line no-debugger */

  /* */

  function act( data )
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
    env.aNodeBuffer = BufferNode.alloc( 10 );
    env.bufferNodeFromArray = BufferNode.from( [ 2, 4 ] );
    env.nodeBufferWithCharacterEncoding = BufferNode.from( 'tést', 'latin1' );
    env.uninitalizedNodeBuffer = BufferNode.allocUnsafe( 10 );
    env.truncatedNodeBuffer = BufferNode.from( [ 257, 257.5, -255, '1' ] );

    env.aTypedArray = new F32x( [ 1, 2 ] );
    env.U8xClampedArray = new U8xClamped( 10*10*4 );
    env.bufferRaw = new BufferRaw( [ 1, 2 ] );
    env.sharedArrayBuffer = new SharedArrayBuffer( [ 1, 2 ] );
    env.bufferViewFromBufferRaw = new BufferView( new BufferRaw( [ 10 ] ) );
    env.BufferViewFromSharedArrayBuffer = new BufferView( new SharedArrayBuffer( [ 10 ] ) );

    return env;
  }

  /* */

  function run( env )
  {
    let r = [];
    r.push( _.bufferNode[ env.name ]( env.aNodeBuffer ) );
    r.push( _.bufferNode[ env.name ]( env.bufferNodeFromArray ) );
    r.push( _.bufferNode[ env.name ]( env.nodeBufferWithCharacterEncoding ) );
    r.push( _.bufferNode[ env.name ]( env.uninitalizedNodeBuffer ) );
    r.push( _.bufferNode[ env.name ]( env.truncatedNodeBuffer ) );
    r.push( _.bufferNode[ env.name ]( env.aNodeBuffer, env.bufferNodeFromArray ) );

    r.push( _.bufferNode[ env.name ]( env.aTypedArray ) );
    r.push( _.bufferNode[ env.name ]( env.U8xClampedArray ) );
    r.push( _.bufferNode[ env.name ]( env.bufferRaw ) );
    r.push( _.bufferNode[ env.name ]( env.sharedArrayBuffer ) );
    r.push( _.bufferNode[ env.name ]( env.bufferViewFromBufferRaw ) );
    r.push( _.bufferNode[ env.name ]( env.BufferViewFromSharedArrayBuffer ) );
    r.push( _.bufferNode[ env.name ]() );
    r.push( _.bufferNode[ env.name ]( 123 ) );

    return r;
  }
}

bufferNodeIsPerformance.timeOut = 1e7;
bufferNodeIsPerformance.experimental = true;

// --
// declaration
// --

const Proto =
{

  name : 'Tools.BufferNode.l0.l1',
  silencing : 1,
  enabled : 1,
  routineTimeOut : 10000,

  tests :
  {

    bufferNodeIs,
    bufferNodeIsPerformance

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
