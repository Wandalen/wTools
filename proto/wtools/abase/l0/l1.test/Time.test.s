( function _l0_l1_Time_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  require( '../Include1.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
// tests
// --

function timerIs( test )
{
  test.case = 'without argument';
  var got = _.timerIs();
  test.identical( got, false );

  test.case = 'check null';
  var got = _.timerIs( null );
  test.identical( got, false );

  test.case = 'check undefined';
  var got = _.timerIs( undefined );
  test.identical( got, false );

  test.case = 'check _.nothing';
  var got = _.timerIs( _.nothing );
  test.identical( got, false );

  test.case = 'check zero';
  var got = _.timerIs( 0 );
  test.identical( got, false );

  test.case = 'check empty string';
  var got = _.timerIs( '' );
  test.identical( got, false );

  test.case = 'check false';
  var got = _.timerIs( false );
  test.identical( got, false );

  test.case = 'check NaN';
  var got = _.timerIs( NaN );
  test.identical( got, false );

  test.case = 'check Symbol';
  var got = _.timerIs( Symbol( 'a' ) );
  test.identical( got, false );

  test.case = 'check empty array';
  var got = _.timerIs( [] );
  test.identical( got, false );

  test.case = 'check empty arguments array';
  var got = _.timerIs( _.argumentsArray.make( [] ) );
  test.identical( got, false );

  test.case = 'check empty unroll';
  var got = _.timerIs( _.unroll.make( [] ) );
  test.identical( got, false );

  test.case = 'check empty map';
  var got = _.timerIs( {} );
  test.identical( got, false );

  test.case = 'check empty pure map';
  var got = _.timerIs( Object.create( null ) );
  test.identical( got, false );

  test.case = 'check empty Set';
  var got = _.timerIs( new Set( [] ) );
  test.identical( got, false );

  test.case = 'check empty Map';
  var got = _.timerIs( new HashMap( [] ) );
  test.identical( got, false );

  test.case = 'check empty BufferRaw';
  var got = _.timerIs( new BufferRaw() );
  test.identical( got, false );

  test.case = 'check empty BufferTyped';
  var got = _.timerIs( new U8x() );
  test.identical( got, false );

  test.case = 'check number';
  var got = _.timerIs( 3 );
  test.identical( got, false );

  test.case = 'check bigInt';
  var got = _.timerIs( 1n );
  test.identical( got, false );

  test.case = 'check string';
  var got = _.timerIs( 'str' );
  test.identical( got, false );

  test.case = 'check not empty array';
  var got = _.timerIs( [ null ] );
  test.identical( got, false );

  test.case = 'check not empty map';
  var got = _.timerIs( { '' : null } );
  test.identical( got, false );

  test.case = 'check instance of constructor';
  function Constr(){ this.x = 1; return this };
  var src = new Constr();
  var got = _.timerIs( src );
  test.identical( got, false );

  test.case = 'check _begin timer';
  var src = _.time._begin( Infinity );
  var got = _.timerIs( src );
  test.identical( got, true );
  _.time.cancel( src );

  test.case = 'check _finally timer';
  var src = _.time._finally( Infinity, undefined );
  var got = _.timerIs( src );
  test.identical( got, true );
  _.time.cancel( src );

  test.case = 'check _periodic timer';
  var src = _.time._periodic( 0, ( t ) => t.native );
  var got = _.timerIs( src );
  test.identical( got, true );
  _.time.cancel( src );

  test.case = 'check imitation of timer';
  var src = { type : 'timer', time : true, cancel : true };
  var got = _.timerIs( src );
  test.identical( got, true );
}

//

function _begin( test )
{
  let context = this;

  var onTime = () => 0;
  var onCancel = () => -1;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - Infinity' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks';
    var timer = _.time._begin( Infinity );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time._begin( Infinity, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time._begin( Infinity, onTime );
    timer.time();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel';
    var timer = _.time._begin( Infinity, undefined, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, execute method cancel';
    var timer = _.time._begin( Infinity, undefined, onCancel );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var timer = _.time._begin( Infinity, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  ready.finally( () =>
  {
    test.close( 'delay - Infinity' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks';
    var timer = _.time._begin( 0 );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time._begin( 0, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time._begin( 0, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel';
    var timer = _.time._begin( 0, undefined, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, execute method cancel';
    var timer = _.time._begin( 0, undefined, onCancel );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var timer = _.time._begin( 0, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time._begin( 0, onTime );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  })

  .finally( () =>
  {
    test.close( 'delay - 0' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay > 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks, timeout < check time';
    var timer = _.time._begin( context.t1/2 );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'without callbacks, timeout > check time';
    var timer = _.time._begin( context.t1 * 20 );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout < check time';
    var timer = _.time._begin( context.t1/2, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time';
    var timer = _.time._begin( context.t1 * 20, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time, execute method time';
    var timer = _.time._begin( context.t1 * 20, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, timeout < check time';
    var timer = _.time._begin( context.t1/2, undefined, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, timeout < check time, execute method cancel';
    var timer = _.time._begin( context.t1/2, undefined, onCancel );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel, timeout < check time';
    var timer = _.time._begin( context.t1/2, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel, timeout > check time';
    var timer = _.time._begin( context.t1 * 20, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time._begin( context.t1/2, onTime );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  });

  ready.finally( ( err, arg ) =>
  {
    test.close( 'delay > 0' );

    if( err )
    throw err;
    return arg;
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'executes method time twice, should throw error';
    var timer = _.time._begin( Infinity, onTime, onCancel );
    timer.time();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });

    /* aaa2 : user should not call methods of timer | Dmytro : now the other concept is used, public methods can be used */

    /* aaa2 : test should ensure that there is no transitions from final states -2 either +2 to any another state. ask | Dmytro : timer not change state from state 2 to -2. State -2 changes to 2 if user call callback timer.time() */
  });

  ready.then( () =>
  {
    test.case = 'executes method cancel twice, should throw error';
    var timer = _.time._begin( Infinity, onTime, onCancel );
    timer.cancel();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = _.time._begin( Infinity, onTime, onCancel );
    timer.time();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = _.time._begin( Infinity, onTime, onCancel );
    timer.cancel();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  /* */

  return ready;
}

//

function _beginTimerInsideOfCallback( test )
{
  let context = this;

  var onCancel = () => -1;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.then( () =>
  {
    test.case = 'single unlinked timer';
    var result = [];
    var onTime = () =>
    {
      result.push( 1 );
      _.time._begin( context.t1, () => result.push( 2 ) );
      return 1;
    };
    var timer = _.time._begin( context.t1, onTime );

    return __.time.out( context.t1 * 100, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 1 );
      test.identical( result, [ 1, 2 ] );

      return null;
    });
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'a interval timer from simple timer';
    var result = [];
    var timer = _.time._begin( context.t1, onTime );
    function onTime()
    {
      if( result.length < 3 )
      {
        result.push( 1 );
        timer = _.time._begin( context.t1, onTime );
        return 1;
      }
      result.push( -1 );
      return -1;
    }

    return __.time.out( context.t1 * 100, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, -1 );
      test.identical( result, [ 1, 1, 1, -1 ] );

      return null;
    });
  });

  return ready;
}

_beginTimerInsideOfCallback.timeOut = 10000;

//

function _finally( test )
{
  let context = this;

  var onTime = () => 0;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - Infinity' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks';
    var timer = _.time._finally( Infinity, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time._finally( Infinity, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time._finally( Infinity, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method cancel';
    var timer = _.time._finally( Infinity, onTime );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, -2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  ready.finally( () =>
  {
    test.close( 'delay - Infinity' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks';
    var timer = _.time._finally( 0, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time._finally( 0, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time._finally( 0, onTime );
    timer.time();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method cancel';
    var timer = _.time._finally( 0, onTime );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, -2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time._finally( 0, onTime );
    return __.time.out( context.t1 * 10, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  })

  .finally( () =>
  {
    test.close( 'delay - 0' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay > 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks, timeout < check time';
    var timer = _.time._finally( context.t1/2, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'without callbacks, timeout > check time';
    var timer = _.time._finally( context.t1 * 10, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, undefined );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout < check time';
    var timer = _.time._finally( context.t1/2, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time';
    var timer = _.time._finally( context.t1 * 10, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time, execute method cancel';
    var timer = _.time._finally( context.t1 * 10, onTime );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, -2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time';
    var timer = _.time._finally( context.t1 * 10, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time, execute method time';
    var timer = _.time._finally( context.t1 * 10, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time._finally( 0, onTime );
    return __.time.out( context.t1 * 10, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  });

  ready.finally( ( err, arg ) =>
  {
    test.close( 'delay > 0' );

    if( err )
    throw err;
    return arg;
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'executes method time twice, should throw error';
    var timer = _.time._finally( Infinity, onTime );
    timer.time();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method cancel twice, should throw error';
    var timer = _.time._finally( Infinity, onTime );
    timer.cancel();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = _.time._finally( Infinity, onTime );
    timer.time();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = _.time._finally( Infinity, onTime );
    timer.cancel();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  /* */

  return ready;
}

_finally.timeOut = 10000;

//

function _periodic( test )
{
  let context = this;

  var onCancel = () => -1;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return undefined;
    };

    var timer = _.time._periodic( 0, onTime );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, -2 );
      test.identical( timer.result, undefined );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return _.dont;
    };

    var timer = _.time._periodic( 0, onTime );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, -2 );
      test.identical( timer.result, _.dont );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return undefined;
    };

    var timer = _.time._periodic( 0, onTime, onCancel );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .finally( () =>
  {
    test.close( 'delay - 0' );
    return null;
  });
  /* - */

  ready.finally( () =>
  {
    test.open( 'delay > 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
    };

    var timer = _.time._periodic( context.t1/2, onTime );
    return __.time.out( context.t1 * 40, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.true( timer.state === -2 );
      test.identical( timer.result, undefined );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return _.dont;
    };

    var timer = _.time._periodic( context.t1/2, onTime );
    return __.time.out( context.t1 * 40, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, undefined );
      test.identical( timer.state, -2 );
      test.identical( timer.result, _.dont );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
    };

    var timer = _.time._periodic( context.t1/2, onTime, onCancel );
    return __.time.out( context.t1 * 40, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.true( timer.state === -2 );
      test.identical( timer.result, -1 );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  /* - */

  ready.finally( ( err, arg ) =>
  {
    test.close( 'delay > 0' );

    if( err )
    throw err;
    return arg;
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'executes method cancel twice, should throw error';
    var timer = _.time._periodic( 1000, () => 1, () => -1 );
    timer.cancel();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time inside of method cancel, should throw error';
    var timer = _.time._periodic( 1000, () => 1, onCancel );
    function onCancel()
    {
      timer.time();
      return -1;
    };

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  /* */

  return ready;
}

_periodic.timeOut = 10000;

//

function _cancel( test )
{
  let context = this;

  test.open( 'timer - _begin' );

  test.case = 'delay - Infinity';
  var timer = _.time._begin( Infinity );
  var got = _.time._cancel( timer );
  test.identical( got.onTime, undefined );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - Infinity, onTime';
  var onTime = () => 0;
  var timer = _.time._begin( Infinity, onTime );
  var got = _.time._cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - Infinity, onCancel';
  var onCancel = () => -1;
  var timer = _.time._begin( Infinity, undefined, onCancel );
  var got = _.time._cancel( timer );
  test.identical( got.onTime, undefined );
  test.identical( got.onCancel, onCancel );
  test.identical( got.state, -2 );
  test.identical( got.result, -1 );

  test.case = 'delay - Infinity, onTime, onCancel';
  var onTime = () => 0;
  var onCancel = () => -1;
  var timer = _.time._begin( Infinity, onTime, onCancel );
  var got = _.time._cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, onCancel );
  test.identical( got.state, -2 );
  test.identical( got.result, -1 );

  test.close( 'timer - _begin' );

  /* - */

  test.open( 'timer - _finally' );

  test.case = 'delay - Infinity';
  var timer = _.time._finally( Infinity, undefined );
  var got = _.time._cancel( timer );
  test.identical( got.onTime, undefined );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - Infinity, onTime';
  var onTime = () => 0;
  var timer = _.time._finally( Infinity, onTime );
  var got = _.time._cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, onTime );
  test.identical( got.state, -2 );
  test.identical( got.result, 0 );

  test.close( 'timer - _finally' );

  /* - */

  test.open( 'timer - _periodic' );

  test.case = 'delay - 0, onTime';
  var onTime = () => 0;
  var timer = _.time._periodic( context.t2, onTime ) ;
  var got = _.time._cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - 0, onTime, onCancel';
  var onTime = () => 0;
  var onCancel = () => -1;
  var timer = _.time._periodic( context.t2, onTime, onCancel ) ;
  var got = _.time._cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, onCancel );
  test.identical( got.state, -2 );
  test.identical( got.result, -1 );

  test.close( 'timer - _periodic' );
}

//

function timerInBegin( test )
{
  let context = this;
  let ready = new __.Consequence().take( null );

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is begun, init state not changed' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    var got = _.time.timerInBegin( timer );
    test.identical( got, true );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    var got = _.time.timerInBegin( timer );
    test.identical( got, true );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    var got = _.time.timerInBegin( timer );
    test.identical( got, true );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is begun, init state not changed' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer canceling, not finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin, check state inside of callback onCancel';
    var timer = _.time.begin( context.t2 * 5, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally, check state inside of callback onCancel';
    var timer = _.time.finally( context.t2 * 5, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic, check state inside of callback onCancel';
    var timer = _.time.periodic( context.t1, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer canceling, not finished' );
    return null;
  })


  /* - */

  ready.then( () =>
  {
    test.open( 'timer is up, not ended' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return undefined;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.close( 'timer is up, not ended' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is canceled' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInBegin( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInBegin( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInBegin( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is canceled' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, () => 1 );

    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 / 10, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInBegin( timer );
      test.identical( got, false );
      _.time.cancel( timer );
      return null;
    })

  })

  ready.then( () =>
  {
    test.close( 'timer is finished' );
    return null;
  })

  /* - */

  if( Config.debug )
  {
    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _.time.timerInBegin() );

    test.case = 'wront type of timer';
    test.shouldThrowErrorSync( () => _.time.timerInBegin( 'timer' ) );
  }

  return ready;
}

//

function timerInCancelBegun( test )
{
  let context = this;
  let ready = new __.Consequence().take( null );

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is begun, init state not changed' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    var got = _.time.timerInCancelBegun( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    var got = _.time.timerInCancelBegun( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    var got = _.time.timerInCancelBegun( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is begun, init state not changed' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer canceling, not finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin, check state inside of callback onCancel';
    var timer = _.time.begin( context.t2 * 5, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, true );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally, check state inside of callback onCancel';
    var timer = _.time.finally( context.t2 * 5, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, true );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic, check state inside of callback onCancel';
    var timer = _.time.periodic( context.t1, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, true );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer canceling, not finished' );
    return null;
  })


  /* - */

  ready.then( () =>
  {
    test.open( 'timer is up, not ended' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, false );
      return undefined;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.close( 'timer is up, not ended' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is canceled' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInCancelBegun( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInCancelBegun( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInCancelBegun( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is canceled' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, () => 1 );

    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 / 10, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInCancelBegun( timer );
      test.identical( got, false );
      _.time.cancel( timer );
      return null;
    })

  })

  ready.then( () =>
  {
    test.close( 'timer is finished' );
    return null;
  })

  /* - */

  if( Config.debug )
  {
    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _.time.timerInCancelBegun() );

    test.case = 'wront type of timer';
    test.shouldThrowErrorSync( () => _.time.timerInCancelBegun( 'timer' ) );
  }

  return ready;
}

timerInCancelBegun.timeOut = 10000;

//

function timerInCancelEnded( test )
{
  let context = this;
  let ready = new __.Consequence().take( null );

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is begun, init state not changed' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    var got = _.time.timerInCancelEnded( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    var got = _.time.timerInCancelEnded( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    var got = _.time.timerInCancelEnded( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is begun, init state not changed' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer canceling, not finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin, check state inside of callback onCancel';
    var timer = _.time.begin( context.t2 * 5, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally, check state inside of callback onCancel';
    var timer = _.time.finally( context.t2 * 5, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic, check state inside of callback onCancel';
    var timer = _.time.periodic( context.t1, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer canceling, not finished' );
    return null;
  })


  /* - */

  ready.then( () =>
  {
    test.open( 'timer is up, not ended' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return undefined;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.close( 'timer is up, not ended' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is canceled' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInCancelEnded( timer );
    test.identical( got, true );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInCancelEnded( timer );
    test.identical( got, true );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInCancelEnded( timer );
    test.identical( got, true );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is canceled' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, () => 1 );

    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 / 10, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInCancelEnded( timer );
      test.identical( got, false );
      _.time.cancel( timer );
      return null;
    })

  })

  ready.then( () =>
  {
    test.close( 'timer is finished' );
    return null;
  })

  /* - */

  if( Config.debug )
  {
    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _.time.timerInCancelEnded() );

    test.case = 'wront type of timer';
    test.shouldThrowErrorSync( () => _.time.timerInCancelEnded( 'timer' ) );
  }

  return ready;
}

//

function timerInEndBegun( test )
{
  let context = this;
  let ready = new __.Consequence().take( null );

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is begun, init state not changed' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    var got = _.time.timerInEndBegun( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    var got = _.time.timerInEndBegun( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    var got = _.time.timerInEndBegun( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is begun, init state not changed' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer canceling, not finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin, check state inside of callback onCancel';
    var timer = _.time.begin( context.t2 * 5, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally, check state inside of callback onCancel';
    var timer = _.time.finally( context.t2 * 5, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic, check state inside of callback onCancel';
    var timer = _.time.periodic( context.t1, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer canceling, not finished' );
    return null;
  })


  /* - */

  ready.then( () =>
  {
    test.open( 'timer is up, not ended' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, true );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, true );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, true );
      return undefined;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.close( 'timer is up, not ended' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is canceled' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInEndBegun( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInEndBegun( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInEndBegun( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is canceled' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, () => 1 );

    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, false );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 / 10, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInEndBegun( timer );
      test.identical( got, false );
      _.time.cancel( timer );
      return null;
    })

  })

  ready.then( () =>
  {
    test.close( 'timer is finished' );
    return null;
  })

  /* - */

  if( Config.debug )
  {
    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _.time.timerInEndBegun() );

    test.case = 'wront type of timer';
    test.shouldThrowErrorSync( () => _.time.timerInEndBegun( 'timer' ) );
  }

  return ready;
}

//

function timerInEndEnded( test )
{
  let context = this;
  let ready = new __.Consequence().take( null );

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is begun, init state not changed' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    var got = _.time.timerInEndEnded( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    var got = _.time.timerInEndEnded( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    var got = _.time.timerInEndEnded( timer );
    test.identical( got, false );
    _.time.cancel( timer );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is begun, init state not changed' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer canceling, not finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin, check state inside of callback onCancel';
    var timer = _.time.begin( context.t2 * 5, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally, check state inside of callback onCancel';
    var timer = _.time.finally( context.t2 * 5, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic, check state inside of callback onCancel';
    var timer = _.time.periodic( context.t1, () => 1, onCancel );
    _.time.cancel( timer );
    function onCancel()
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, false );
      return got;
    }
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer canceling, not finished' );
    return null;
  })


  /* - */

  ready.then( () =>
  {
    test.open( 'timer is up, not ended' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, false );
      return got;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t1, onTime );
    function onTime()
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, false );
      return undefined;
    }
    return __.time.out( context.t2 / 10 );
  })

  ready.then( () =>
  {
    test.close( 'timer is up, not ended' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is canceled' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInEndEnded( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInEndEnded( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 * 5, () => 1 );
    _.time.cancel( timer );
    var got = _.time.timerInEndEnded( timer );
    test.identical( got, false );
    return null;
  })

  ready.then( () =>
  {
    test.close( 'timer is canceled' );
    return null;
  })

  /* - */

  ready.then( () =>
  {
    test.open( 'timer is finished' );
    return null;
  })

  ready.then( () =>
  {
    test.case = 'timer - begin';
    var timer = _.time.begin( context.t1, () => 1 );

    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, true );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - finally';
    var timer = _.time.finally( context.t1, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, true );
      return null;
    })
  })

  ready.then( () =>
  {
    test.case = 'timer - periodic';
    var timer = _.time.periodic( context.t2 / 10, () => 1 );
    return __.time.out( context.t2, () =>
    {
      var got = _.time.timerInEndEnded( timer );
      test.identical( got, true );
      _.time.cancel( timer );
      return null;
    })

  })

  ready.then( () =>
  {
    test.close( 'timer is finished' );
    return null;
  })

  /* - */

  if( Config.debug )
  {
    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _.time.timerInEndEnded() );

    test.case = 'wront type of timer';
    test.shouldThrowErrorSync( () => _.time.timerInEndEnded( 'timer' ) );
  }

  return ready;
}

timerInEndEnded.timeOut = 10000;

//

function begin( test )
{
  let context = this;

  var onTime = () => 0;
  var onCancel = () => -1;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - Infinity' );
    return null;
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time.begin( Infinity, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time.begin( Infinity, onTime );
    timer.time();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel';
    var timer = _.time.begin( Infinity, undefined, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, execute method cancel';
    var timer = _.time.begin( Infinity, undefined, onCancel );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var timer = _.time.begin( Infinity, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  ready.finally( () =>
  {
    test.close( 'delay - Infinity' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time.begin( 0, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time.begin( 0, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel';
    var timer = _.time.begin( 0, undefined, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, execute method cancel';
    var timer = _.time.begin( 0, undefined, onCancel );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var timer = _.time.begin( 0, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time.begin( 0, onTime );
    return __.time.out( context.t1 * 10, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  })

  .finally( () =>
  {
    test.close( 'delay - 0' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay > 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'onTime, timeout < check time';
    var timer = _.time.begin( context.t1/2, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time';
    var timer = _.time.begin( context.t1 * 10, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time, execute method time';
    var timer = _.time.begin( context.t1 * 10, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, timeout < check time';
    var timer = _.time.begin( context.t1/2, undefined, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onCancel, timeout < check time, execute method cancel';
    var timer = _.time.begin( context.t1/2, undefined, onCancel );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel, timeout < check time';
    var timer = _.time.begin( context.t1/2, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel, timeout > check time';
    var timer = _.time.begin( context.t1 * 10, onTime, onCancel );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time.begin( context.t1/2, onTime );
    return __.time.out( context.t1 * 10, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  });

  ready.finally( ( err, arg ) =>
  {
    test.close( 'delay > 0' );

    if( err )
    throw err;
    return arg;
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'without arguments';
    return __.time.out( 0, () => _.time.begin() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'not enough arguments';
    return __.time.out( 0, () => _.time.begin( 0 ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'wrong type of onTime';
    return __.time.out( 0, () => _.time.begin( 0, [] ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'wrong type of onCancel';
    return __.time.out( 0, () => _.time.begin( 0, () => 1, [] ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method cancel twice, should throw error';
    var timer = _.time.begin( Infinity, onTime, onCancel );
    timer.cancel();
    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = _.time.begin( Infinity, onTime, onCancel );
    timer.time();

    return __.time.out( context.t1, () =>
    {
      timer.cancel()
    })
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time twice, should throw error';
    var timer = _.time.begin( Infinity, onTime, onCancel );
    timer.time();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = __.time.begin( Infinity, onTime, onCancel );
    timer.cancel();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  /* */

  return ready;
}

begin.timeOut = 10000;

//

function beginTimerInsideOfCallback( test )
{
  let context = this;

  var onCancel = () => -1;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.then( () =>
  {
    test.case = 'single unlinked timer';
    var result = [];
    var onTime = () =>
    {
      result.push( 1 );
      _.time.begin( context.t1, () => result.push( 2 ) );
      return 1;
    };
    var timer = _.time.begin( context.t1, onTime );

    return __.time.out( context.t1 * 100, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 1 );
      test.identical( result, [ 1, 2 ] );

      return null;
    });
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'a interval timer from simple timer';
    var result = [];
    var timer = _.time.begin( context.t1, onTime );
    function onTime()
    {
      if( result.length < 3 )
      {
        result.push( 1 );
        timer = _.time.begin( context.t1, onTime );
        return 1;
      }
      result.push( -1 );
      return -1;
    }

    return __.time.out( context.t1 * 100, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, -1 );
      test.identical( result, [ 1, 1, 1, -1 ] );

      return null;
    });
  });

  return ready;
}

beginTimerInsideOfCallback.timeOut = 10000;

//

function finally_( test )
{
  let context = this;

  var onTime = () => 0;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - Infinity' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks';
    var timer = _.time.finally( Infinity, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time.finally( Infinity, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time.finally( Infinity, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method cancel';
    var timer = _.time.finally( Infinity, onTime );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, -2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  ready.finally( () =>
  {
    test.close( 'delay - Infinity' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks';
    var timer = _.time.finally( 0, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime';
    var timer = _.time.finally( 0, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method time';
    var timer = _.time.finally( 0, onTime );
    timer.time();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute method cancel';
    var timer = _.time.finally( 0, onTime );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, -2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time.finally( 0, onTime );
    return __.time.out( context.t1 * 10, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  })

  .finally( () =>
  {
    test.close( 'delay - 0' );
    return null;
  });

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay > 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'without callbacks, timeout < check time';
    var timer = _.time.finally( context.t1/2, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'without callbacks, timeout > check time';
    var timer = _.time.finally( context.t1 * 10, undefined );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, null );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout < check time';
    var timer = _.time.finally( context.t1/2, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time';
    var timer = _.time.finally( context.t1 * 10, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time, execute method cancel';
    var timer = _.time.finally( context.t1 * 10, onTime );
    timer.cancel();
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, -2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time';
    var timer = _.time.finally( context.t1 * 10, onTime );
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 0 );
      test.identical( timer.result, undefined );
      _.time.cancel( timer );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, timeout > check time, execute method time';
    var timer = _.time.finally( context.t1 * 10, onTime );
    timer.time()
    return __.time.out( context.t1, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, 0 );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'only one execution';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
      }
    };

    var timer = _.time.finally( 0, onTime );
    return __.time.out( context.t1 * 10, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onTime );
      test.identical( timer.state, 2 );
      test.identical( timer.result, undefined );
      test.identical( times, 4 );
      test.identical( result, [ 1 ] );

      return null;
    });
  });

  ready.finally( ( err, arg ) =>
  {
    test.close( 'delay > 0' );

    if( err )
    throw err;
    return arg;
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'without arguments';
    return __.time.out( 0, () => _.time.finally() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'not enough arguments';
    return __.time.out( 0, () => _.time.finally( 0 ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'wrong type of onTime';
    return __.time.out( 0, () => _.time.finally( 0, [] ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time twice, should throw error';
    var timer = _.time.finally( Infinity, onTime );
    timer.time();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method cancel twice, should throw error';
    var timer = _.time.finally( Infinity, onTime );
    timer.cancel();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = _.time.finally( Infinity, onTime );
    timer.time();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time and then method cancel, should throw error';
    var timer = _.time.finally( Infinity, onTime );
    timer.cancel();

    return __.time.out( context.t1, () => timer.time() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  /* */

  return ready;
}

finally_.timeOut = 10000;

//

function periodic( test )
{
  let context = this;

  var onCancel = () => -1;
  var ready = new __.Consequence().take( null );

  /* - */

  ready.finally( () =>
  {
    test.open( 'delay - 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return undefined;
    };

    var timer = _.time.periodic( 0, onTime );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, -2 );
      test.identical( timer.result, undefined );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return _.dont;
    };

    var timer = _.time.periodic( 0, onTime );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, -2 );
      test.identical( timer.result, _.dont );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return undefined;
    };

    var timer = _.time.periodic( 0, onTime, onCancel );
    return __.time.out( context.t1 * 20, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.identical( timer.state, -2 );
      test.identical( timer.result, -1 );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .finally( () =>
  {
    test.close( 'delay - 0' );
    return null;
  });
  /* - */

  ready.finally( () =>
  {
    test.open( 'delay > 0' );
    return null;
  })

  .then( function()
  {
    test.case = 'onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
    };

    var timer = _.time.periodic( context.t1/2, onTime );
    return __.time.out( context.t1 * 40, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.true( timer.state === -2 );
      test.identical( timer.result, undefined );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, execute onTime';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
      return _.dont;
    };

    var timer = _.time.periodic( context.t1/2, onTime );
    return __.time.out( context.t1 * 40, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, null );
      test.identical( timer.state, -2 );
      test.identical( timer.result, _.dont );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  .then( function()
  {
    test.case = 'onTime, onCancel';
    var times = 5;
    var result = [];
    var onTime = () =>
    {
      if( times > 0 )
      {
        result.push( 1 );
        times--;
        return true;
      }
    };

    var timer = _.time.periodic( context.t1/2, onTime, onCancel );
    return __.time.out( context.t1 * 40, () =>
    {
      test.identical( timer.onTime, onTime );
      test.identical( timer.onCancel, onCancel );
      test.true( timer.state === -2 );
      test.identical( timer.result, -1 );
      test.identical( times, 0 );
      test.identical( result, [ 1, 1, 1, 1, 1 ] );

      return null;
    });
  })

  /* - */

  ready.finally( ( err, arg ) =>
  {
    test.close( 'delay > 0' );

    if( err )
    throw err;
    return arg;
  });

  /* - */

  ready.then( () =>
  {
    test.case = 'without arguments';
    return __.time.out( 0, () => _.time.periodic() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'not enough arguments';
    return __.time.out( 0, () => _.time.periodic( 0 ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'wrong type of onTime';
    return __.time.out( 0, () => _.time.periodic( 0, [] ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'wrong type of onCancel';
    return __.time.out( 0, () => _.time.periodic( 0, () => 1, [] ) )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method cancel twice, should throw error';
    var timer = _.time.periodic( context.t1 * 100, () => 1, () => -1 );
    timer.cancel();

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'executes method time inside of method cancel, should throw error';
    var timer = _.time.periodic( context.t1 * 100, () => 1, onCancel );
    function onCancel()
    {
      timer.time();
      return -1;
    };

    return __.time.out( context.t1, () => timer.cancel() )
    .finally( ( err, arg ) =>
    {
      if( arg )
      {
        test.true( false );
      }
      else
      {
        _.error.attend( err );
        test.true( true );
      }
      return null;
    });
  });

  /* */

  return ready;
}

periodic.timeOut = 20000;

//

function cancel( test )
{
  let context = this;

  test.open( 'timer - _begin' );

  test.case = 'delay - Infinity';
  var timer = _.time._begin( Infinity );
  var got = _.time.cancel( timer );
  test.identical( got.onTime, undefined );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - Infinity, onTime';
  var onTime = () => 0;
  var timer = _.time._begin( Infinity, onTime );
  var got = _.time.cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - Infinity, onCancel';
  var onCancel = () => -1;
  var timer = _.time._begin( Infinity, undefined, onCancel );
  var got = _.time.cancel( timer );
  test.identical( got.onTime, undefined );
  test.identical( got.onCancel, onCancel );
  test.identical( got.state, -2 );
  test.identical( got.result, -1 );

  test.case = 'delay - Infinity, onTime, onCancel';
  var onTime = () => 0;
  var onCancel = () => -1;
  var timer = _.time._begin( Infinity, onTime, onCancel );
  var got = _.time.cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, onCancel );
  test.identical( got.state, -2 );
  test.identical( got.result, -1 );

  test.close( 'timer - _begin' );

  /* - */

  test.open( 'timer - _finally' );

  test.case = 'delay - Infinity';
  var timer = _.time._finally( Infinity, undefined );
  var got = _.time.cancel( timer );
  test.identical( got.onTime, undefined );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - Infinity, onTime';
  var onTime = () => 0;
  var timer = _.time._finally( Infinity, onTime );
  var got = _.time.cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, onTime );
  test.identical( got.state, -2 );
  test.identical( got.result, 0 );

  test.close( 'timer - _finally' );

  /* - */

  test.open( 'timer - _periodic' );

  test.case = 'delay - 0, onTime';
  var onTime = () => 0;
  var timer = _.time._periodic( context.t2, onTime ) ;
  var got = _.time.cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, undefined );
  test.identical( got.state, -2 );
  test.identical( got.result, undefined );

  test.case = 'delay - 0, onTime, onCancel';
  var onTime = () => 0;
  var onCancel = () => -1;
  var timer = _.time._periodic( context.t2, onTime, onCancel ) ;
  var got = _.time.cancel( timer );
  test.identical( got.onTime, onTime );
  test.identical( got.onCancel, onCancel );
  test.identical( got.state, -2 );
  test.identical( got.result, -1 );

  test.close( 'timer - _periodic' );
}

//

function now( test )
{
  test.case = 'trivial';
  var got = _.time.now();
  test.ge( got, 1629089923250 );
  test.true( _.intIs( got ) );

  test.case = 'call with argument, should not throw error';
  var got = _.time.now( 'wrong' );
  test.ge( got, 1629089923250 );
  test.true( _.intIs( got ) );
}

// --
// declaration
// --

const Proto =
{

  name : 'Tools.Time.l0.l1',
  silencing : 1,

  context :
  {
    t1 : 10,
    t2 : 1000,
  },

  tests :
  {

    //checker

    timerIs,

    // private

    _begin,
    _beginTimerInsideOfCallback,
    _finally,
    _periodic,
    _cancel,

    // public

    timerInBegin,
    timerInCancelBegun,
    timerInCancelEnded,
    timerInEndBegun,
    timerInEndEnded,

    begin,
    beginTimerInsideOfCallback,
    finally : finally_,
    periodic,
    cancel,

    now,
  }

}

//

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();

