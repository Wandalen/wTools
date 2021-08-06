( function _l0_l3_Time_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  require( '../Include3.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
// tests
// --

function _sleep( test )
{

  test.case = 'delay - 0';
  var start = _.time.now();
  _.time._sleep( 0 );
  var got = _.time.now() - start;
  test.true( 0 <= got );

  test.case = 'delay - 2';
  var start = _.time.now();
  _.time._sleep( 2 );
  var got = _.time.now() - start;
  test.true( 1 <= got );

  test.case = 'delay - 100';
  var start = _.time.now();
  _.time._sleep( 100 );
  var got = _.time.now() - start;
  test.true( 99 <= got );

  test.case = 'delay - 2000';
  var start = _.time.now();
  _.time._sleep( 2000 );
  var got = _.time.now() - start;
  test.true( 1999 <= got );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.time._sleep() );

  test.case = 'wrong type of delay';
  test.shouldThrowErrorSync( () => _.time._sleep( '10' ) );

  test.case = 'negative value of delay';
  test.shouldThrowErrorSync( () => _.time._sleep( -1 ) );

  test.case = 'Infinity value of delay';
  test.shouldThrowErrorSync( () => _.time._sleep( Infinity ) );

  test.case = 'delay has NaN value';
  test.shouldThrowErrorSync( () => _.time._sleep( NaN ) );
}

_sleep.timeOut = 30000;

//

function sleep( test )
{

  test.case = 'delay - 0';
  var start = _.time.now();
  _.time.sleep( 0 );
  var got = _.time.now() - start;
  test.true( 0 <= got );

  test.case = 'delay - 2';
  var start = _.time.now();
  _.time.sleep( 2 );
  var got = _.time.now() - start;
  test.true( 1 <= got );

  test.case = 'delay - 100';
  var start = _.time.now();
  _.time.sleep( 100 );
  var got = _.time.now() - start;
  test.true( 99 <= got );

  test.case = 'delay - 2000';
  var start = _.time.now();
  _.time.sleep( 2000 );
  var got = _.time.now() - start;
  test.true( 1999 <= got );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.time.sleep() );

  test.case = 'extra arguments';
  var procedure = new __.Procedure();
  procedure.begin();
  test.shouldThrowErrorSync( () => _.time.sleep( 10, procedure, 10 ) );
  procedure.end();

  test.case = 'wrong type of delay';
  test.shouldThrowErrorSync( () => _.time.sleep( '10' ) );

  test.case = 'negative value of delay';
  test.shouldThrowErrorSync( () => _.time.sleep( -1 ) );

  test.case = 'Infinity value of delay';
  test.shouldThrowErrorSync( () => _.time.sleep( Infinity ) );

  test.case = 'delay has NaN value';
  test.shouldThrowErrorSync( () => _.time.sleep( NaN ) );
}

sleep.timeOut = 30000;

//

function timeOutCancelInsideOfCallback( test )
{
  let context = this;
  let visited = [];

  var timer = _.time.begin( 1, () =>
  {
    visited.push( 'v1' );
    _.time.cancel( timer );
    visited.push( 'v2' );
  });

  visited.push( 'v0' );

  return __.time.out( context.t1*15 ).then( () =>
  {
    test.identical( visited, [ 'v0', 'v1', 'v2' ] );
    return null;
  });
}

//

function timeOutCancelOutsideOfCallback( test )
{
  let context = this;
  let visited = [];

  var timer = _.time.begin( context.t1*3, () =>
  {
    visited.push( 'v1' );
  });

  _.time.cancel( timer );
  visited.push( 'v0' );

  return __.time.out( context.t1*15 ).then( () =>
  {
    test.identical( visited, [ 'v0' ] );
    return null;
  });
}

//

function timeOutCancelZeroDelayInsideOfCallback( test )
{
  let context = this;
  let visited = [];

  var timer = _.time.begin( 0, () =>
  {
    visited.push( 'v1' );
    _.time.cancel( timer );
    visited.push( 'v2' );
  });

  visited.push( 'v0' );

  return __.time.out( context.t1*15 ).then( () =>
  {
    test.identical( visited, [ 'v0', 'v1', 'v2' ] );
    return null;
  });
}

//

function timeOutCancelZeroDelayOutsideOfCallback( test )
{
  let context = this;
  let visited = [];

  var timer = _.time.begin( 0, () =>
  {
    visited.push( 'v1' );
  });

  _.time.cancel( timer );
  visited.push( 'v0' );

  return __.time.out( context.t1*15 ).then( () =>
  {
    test.identical( visited, [ 'v0' ] );
    return null;
  });
}

//

function from( test )
{

  test.case = 'from number';
  var got = _.time.from( 1024 );
  test.identical( got, 1024 );

  test.case = 'from Date';
  var date = new Date();
  var got = _.time.from( date );
  var exp = date.getTime();
  test.identical( got, exp );

  test.case = 'from String with date';
  var date = new Date();
  date = date.toString();
  var got = _.time.from( date );
  var exp = Date.parse( date );
  test.identical( got, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.time.from() );

  test.case = 'wrong type of time';
  test.shouldThrowErrorSync( () => _.time.from( [] ) );

  test.case = 'wrong string date';
  test.shouldThrowErrorSync( () => _.time.from( 'some' ) );

}

// --
// declaration
// --

const Proto =
{

  name : 'Tools.Time.l0.l3',
  silencing : 1,

  context :
  {
    t1 : 10,
    t2 : 1000,
  },

  tests :
  {

    _sleep,
    sleep,

    timeOutCancelInsideOfCallback,
    timeOutCancelOutsideOfCallback,

    timeOutCancelZeroDelayInsideOfCallback,
    timeOutCancelZeroDelayOutsideOfCallback,

    from,

  }

}

//

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();

