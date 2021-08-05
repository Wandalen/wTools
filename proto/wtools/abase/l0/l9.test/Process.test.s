( function _l0_l9_Process_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
// events
// --

function onWithArguments( test )
{
  const self = this;
  const a = test.assetFor( false );
  a.fileProvider.dirMake( a.abs( '.' ) );

  /* - */

  a.ready.then( () =>
  {
    test.case = 'no callbacks for events';
    return null;
  });
  var program = a.program( withoutCallbacks );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[]' ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'single callback for single event, single event is given';
    return null;
  });
  var program = a.program( callbackForAvailable );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ] ]' ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'single callback for single event, a few events are given';
    return null;
  });
  var program = a.program( callbackForAvailableDouble );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ], [ \'available\', \'arg\' ] ]' ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'single callback for single event, a few events are given';
    return null;
  });
  var program = a.program( callbacksForEvents );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    var exp =
`[ 'available', 'arg' ],
  'uncaughtError1',
  [ 'available', 'arg' ],
  'uncaughtError3'`;
    test.identical( _.strCount( op.output, exp ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'throw uncaught error';
    return null;
  });
  var program = a.program( uncaughtError );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    var exp = 'exit';
    test.identical( _.strCount( op.output, exp ), 0 );
    return null;
  });

  /* - */

  return a.ready;

  // test.case = 'no callback for events';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // /* aaa : for Dmytro : bad : ??
  // bad!
  // rewrite
  // discuss first
  // */
  // /* Dmytro : fixed */
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [] );
  //
  // /* */
  //
  // test.case = 'single callback for single event, single event is given';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on( 'event1', onEvent );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0 ] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0 ] );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  // got.event1.off();
  //
  // /* */
  //
  // test.case = 'single callback for single event, a few events are given';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on( 'event1', onEvent );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0 ] );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0, 1 ] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0, 1 ] );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  // got.event1.off();
  //
  // /* */
  //
  // test.case = 'single callback for each events in event handler, a few events are given';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on( 'event1', onEvent );
  // var got2 = _.process.on( 'event2', onEvent2 );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0 ] );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0, 1 ] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0, 1, -2, -3 ] );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  // got.event1.off();
  // got2.event2.off();

  /* */

  function withoutCallbacks()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbackForAvailable()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.on( 'available', ( ... args ) => result.push( args ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbackForAvailableDouble()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.on( 'available', ( ... args ) => result.push( args ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbacksForEvents()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.on( 'available', ( ... args ) => result.push( args ) );
    _.process.on( 'uncaughtError', ( e ) => result.push( e + result.length ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function uncaughtError()
  {
    const _ = require( toolsPath );
    _.process.on( 'uncaughtError', ( o ) => _.errAttend( o.err ) );
    throw _.err( 'Error' );
    console.log( 'exit' );
  }
}

//

function onWithOptionsMap( test )
{
  const self = this;
  const a = test.assetFor( false );
  const con = __.take( null );
  a.fileProvider.dirMake( a.abs( '.' ) );

  /* - */

  con.then( () =>
  {
    test.case = 'no callbacks for events';
    var program = a.program( withoutCallbacks );
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      test.identical( _.strCount( op.output, '[]' ), 1 );
      return null;
    });
  });

  /* - */

  con.then( () =>
  {
    test.open( 'single callback for event' );
    return null;
  });

  con.then( () =>
  {
    test.case = 'single callback for single event, single event is given';
    var o =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : false,
    };
    var program = a.program({ entry : callbackForAvailable, locals : { o, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ] ]' ), 1 );
      return null;
    });
  });

  /* */

  con.then( () =>
  {
    test.case = 'single callback for single event, a few events are given';
    var o =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : false,
    };
    var program = a.program({ entry : callbackForAvailableDouble, locals : { o, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ], [ \'available\', \'arg\' ] ]' ), 1 );
      return null;
    });
  });

  /* */

  con.then( () =>
  {
    test.case = 'single callback for single event, a few events are given';
    var o =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : false,
    };
    var program = a.program({ entry : callbacksForEvents, locals : { o, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      var exp =
`[ 'available', 'arg' ],
  'uncaughtError1',
  [ 'available', 'arg' ],
  'uncaughtError3'`;
      test.contains( _.strCount( op.output, exp ), 1 );
      return null;
    });
  });

  /* */

  con.then( () =>
  {
    test.case = 'throw uncaught error';
    var o =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : false,
    };
    var program = a.program({ entry : uncaughtError, locals : { o, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      var exp = 'exit';
      test.identical( _.strCount( op.output, exp ), 0 );
      return null;
    });
  });

  con.then( () =>
  {
    test.close( 'single callback for event' );
    return null;
  });

  /* - */

  con.then( () =>
  {
    test.open( 'options map with option first' );
    return null;
  });

  con.then( () =>
  {
    test.case = 'callback1.first - false, callback2.first - false';
    var o1 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : false,
    };
    var o2 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( result.length ) },
      first : false,
    };
    var program = a.program({ entry : severalCallbacks, locals : { o1, o2, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ], 1 ]' ), 1 );
      return null;
    });
  });

  /* */

  con.then( () =>
  {
    test.case = 'callback1.first - true, callback2.first - false';
    var o1 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : true,
    };
    var o2 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( result.length ) },
      first : false,
    };
    var program = a.program({ entry : severalCallbacks, locals : { o1, o2, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ], 1 ]' ), 1 );
      return null;
    });
  });

  /* */

  con.then( () =>
  {
    test.case = 'callback1.first - false, callback2.first - true';
    var o1 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : false,
    };
    var o2 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( result.length ) },
      first : true,
    };
    var program = a.program({ entry : severalCallbacks, locals : { o1, o2, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      test.identical( _.strCount( op.output, '[ 0, [ \'available\', \'arg\' ] ]' ), 1 );
      return null;
    });
  });

  /* */

  con.then( () =>
  {
    test.case = 'callback1.first - true, callback2.first - true';
    var o1 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( args ) },
      first : true,
    };
    var o2 =
    {
      callbackMap : { 'available' : ( ... args ) => result.push( result.length ) },
      first : true,
    };
    var program = a.program({ entry : severalCallbacks, locals : { o1, o2, result : [] } });
    return program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      test.identical( _.strCount( op.output, '[ 0, [ \'available\', \'arg\' ] ]' ), 1 );
      return null;
    });
  });

  con.then( () =>
  {
    test.close( 'options map with option first' );
    return null;
  });

  // test.open( 'option first - 0' );
  //
  // test.case = 'no callback for events';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [] );
  //
  // /* */
  //
  // test.case = 'single callback for single event, single event is given';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on({ 'callbackMap' : { 'event1' : onEvent } });
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0 ] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0 ] );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  // got.event1.off();
  //
  // /* */
  //
  // test.case = 'single callback for single event, a few events are given';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on({ 'callbackMap' : { 'event1' : onEvent } } );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0 ] );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0, 1 ] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0, 1 ] );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  // got.event1.off();
  //
  // /* */
  //
  // test.case = 'single callback for each events in event handler, a few events are given';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on({ 'callbackMap' : { 'event1' : onEvent, 'event2' : onEvent2 } });
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0 ] );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ 0, 1 ] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0, 1, -2, -3 ] );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  // got.event1.off();
  // got.event2.off();
  //
  // test.close( 'option first - 0' );
  //
  // /* - */
  //
  // test.open( 'option first - 1' );
  //
  // test.case = 'callback added before other callback';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on({ 'callbackMap' : { 'event1' : onEvent } });
  // var got2 = _.process.on({ 'callbackMap' : { 'event1' : onEvent2 }, 'first' : 1 });
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ -0, 1 ] );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ -0, 1, -2, 3 ] );
  // got.event1.off();
  // got2.event1.off();
  //
  // /* */
  //
  // test.case = 'callback added after other callback';
  //
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var onEvent2 = () => result.push( -1 * result.length );
  // var got = _.process.on({ 'callbackMap' : { 'event1' : onEvent2 }, 'first' : 1 });
  // var got2 = _.process.on({ 'callbackMap' : { 'event1' : onEvent } });
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ -0, 1 ] );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [ -0, 1, -2, 3 ] );
  //
  // test.close( 'option first - 1' );

  /* - */

  if( Config.debug )
  con.then( () =>
  {
    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _.process.on() );

    test.case = 'wrong type of callback';
    test.shouldThrowErrorSync( () => _.process.on( 'event1', {} ) );

    test.case = 'wrong type of event name';
    test.shouldThrowErrorSync( () => _.process.on( [], () => 'str' ) );

    test.case = 'wrong type of options map o';
    test.shouldThrowErrorSync( () => _.process.on( 'wrong' ) );

    test.case = 'extra options in options map o';
    test.shouldThrowErrorSync( () => _.process.on({ callbackMap : {}, wrong : {} }) );

    test.case = 'not known event in callbackMap';
    test.shouldThrowErrorSync( () => _.process.on({ callbackMap : { unknown : () => 'unknown' } }) );
    return null;
  });

  /* - */

  return con;

  /* */

  function withoutCallbacks()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbackForAvailable()
  {
    const _ = require( toolsPath );
    _.process.on( o );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbackForAvailableDouble()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.on( 'available', ( ... args ) => result.push( args ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbacksForEvents()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.on( 'available', ( ... args ) => result.push( args ) );
    _.process.on( 'uncaughtError', ( e ) => result.push( e + result.length ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function uncaughtError()
  {
    const _ = require( toolsPath );
    _.process.on( 'uncaughtError', ( o ) => _.errAttend( o.err ) );
    throw _.err( 'Error' );
    console.log( 'exit' );
  }

  /* */

  function severalCallbacks()
  {
    const _ = require( toolsPath );
    _.process.on( o1 );
    _.process.on( o2 );
    _.process.eventGive( 'available', 'arg' );
    console.log( result );
  }
}

onWithOptionsMap.timeOut = 30000;

//

function onWithChain( test )
{
  const self = this;
  const a = test.assetFor( false );
  a.fileProvider.dirMake( a.abs( '.' ) );

  /* - */

  a.ready.then( () =>
  {
    test.case = 'chain in args';
    return null;
  });
  var program = a.program( chainInArgs );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ [ \'uncaughtError\', \'arg\' ] ]' ), 1 );
    return null;
  });

  a.ready.then( () =>
  {
    test.case = 'chain in map';
    return null;
  });
  var program = a.program( chainInMap );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ [ \'uncaughtError\', \'arg\' ] ]' ), 1 );
    return null;
  });

  // test.case = 'call with arguments';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var got = _.process.on( _.event.Chain( 'event1', 'event2' ), onEvent );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent } ) );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0 ] );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent } ) );
  // _.event.off( _.process._edispatcher, { callbackMap : { event2 : null } } );
  //
  // /* */
  //
  // test.case = 'call with options map';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var got = _.process.on({ callbackMap : { event1 : [ _.event.Name( 'event2' ), onEvent ] } });
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent } ) );
  // _.event.eventGive( _.process._edispatcher, 'event1' );
  // test.identical( result, [] );
  // _.event.eventGive( _.process._edispatcher, 'event2' );
  // test.identical( result, [ 0 ] );
  // test.false( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event2', eventHandler : onEvent } ) );
  // _.event.off( _.process._edispatcher, { callbackMap : { event2 : null } } );

  /* - */

  return a.ready

  /* */

  function chainInArgs()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.on( _.event.Chain( 'available', 'uncaughtError' ), ( ... args ) => result.push( args ) );
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function chainInMap()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.on({ callbackMap : { 'available' : [ _.event.Name( 'uncaughtError' ), ( ... args ) => result.push( args ) ] } });
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }
}

//

function onCheckDescriptor( test )
{
  const self = this;
  const a = test.assetFor( false );
  a.fileProvider.dirMake( a.abs( '.' ) );

  /* - */

  a.ready.then( () =>
  {
    test.case = 'from arguments';
    return null;
  });
  var program = a.program( callbackInArgs );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ \'available\' ]' ), 1 );
    test.identical( _.strCount( op.output, '[ \'off\', \'enabled\', \'first\', \'callbackMap\' ]' ), 1 );
    test.identical( _.strCount( op.output, 'descriptor.enabled : true' ), 1 );
    test.identical( _.strCount( op.output, 'descriptor.first : false' ), 1 );
    test.identical( _.strCount( op.output, 'descriptor.callbackMap : available' ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'from map';
    return null;
  });
  var program = a.program( callbackInMap );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ \'available\' ]' ), 1 );
    test.identical( _.strCount( op.output, '[ \'off\', \'enabled\', \'first\', \'callbackMap\' ]' ), 1 );
    test.identical( _.strCount( op.output, 'descriptor.enabled : true' ), 1 );
    test.identical( _.strCount( op.output, 'descriptor.first : true' ), 1 );
    test.identical( _.strCount( op.output, 'descriptor.callbackMap : available' ), 1 );
    return null;
  });

  // test.case = 'call with arguments';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var descriptor = _.process.on( 'event1', onEvent );
  // test.identical( _.props.keys( descriptor ), [ 'event1' ] );
  // test.identical( _.props.keys( descriptor.event1 ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  // test.identical( descriptor.event1.enabled, true );
  // test.identical( descriptor.event1.first, 0 );
  // test.equivalent( descriptor.event1.callbackMap, { event1 : onEvent } );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // descriptor.event1.off();
  //
  // /* */
  //
  // test.case = 'call with arguments';
  // var result = [];
  // var onEvent = () => result.push( result.length );
  // var descriptor = _.process.on({ callbackMap : { 'event1' : onEvent } });
  // test.identical( _.props.keys( descriptor ), [ 'event1' ] );
  // test.identical( _.props.keys( descriptor.event1 ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  // test.identical( descriptor.event1.enabled, true );
  // test.identical( descriptor.event1.first, 0 );
  // test.equivalent( descriptor.event1.callbackMap, { event1 : onEvent } );
  // test.true( _.event.eventHasHandler( _.process._edispatcher, { eventName : 'event1', eventHandler : onEvent } ) );
  // descriptor.event1.off();

  /* - */

  return a.ready;

  /* */

  function callbackInArgs()
  {
    const _ = require( toolsPath );
    const descriptor = _.process.on( 'available', ( ... args ) => result.push( args ) );
    console.log( _.props.keys( descriptor ) );
    console.log( _.props.keys( descriptor.available ) );
    console.log( `descriptor.enabled : ${ descriptor.available.enabled }` );
    console.log( `descriptor.first : ${ descriptor.available.first }` );
    console.log( `descriptor.callbackMap : ${ _.props.keys( descriptor.available.callbackMap ) }` );
  }

  /* */

  function callbackInMap()
  {
    const _ = require( toolsPath );
    const descriptor = _.process.on({ callbackMap : { 'available' : ( ... args ) => result.push( args ) }, first : true });
    console.log( _.props.keys( descriptor ) );
    console.log( _.props.keys( descriptor.available ) );
    console.log( `descriptor.enabled : ${ descriptor.available.enabled }` );
    console.log( `descriptor.first : ${ descriptor.available.first }` );
    console.log( `descriptor.callbackMap : ${ _.props.keys( descriptor.available.callbackMap ) }` );
  }
}

//

function onceWithArguments( test )
{
  const self = this;
  const a = test.assetFor( false );
  a.fileProvider.dirMake( a.abs( '.' ) );

  /* - */

  a.ready.then( () =>
  {
    test.case = 'no callbacks for events';
    return null;
  });
  var program = a.program( withoutCallbacks );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[]' ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'single callback for single event, single event is given';
    return null;
  });
  var program = a.program( callbackForAvailable );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ] ]' ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'single callback for single event, a few events are given';
    return null;
  });
  var program = a.program( callbackForAvailableDouble );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '[ [ \'available\', \'arg\' ] ]' ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'single callback for single event, a few events are given';
    return null;
  });
  var program = a.program( callbacksForEvents );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    var exp = `[ [ 'available', 'arg' ], 'uncaughtError1' ]`;
    test.identical( _.strCount( op.output, exp ), 1 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'throw uncaught error';
    return null;
  });
  var program = a.program( uncaughtError );
  program.start();
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    var exp = 'exit';
    test.identical( _.strCount( op.output, exp ), 0 );
    return null;
  });

  /* - */

  return a.ready;

  /* */

  function withoutCallbacks()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbackForAvailable()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.once( 'available', ( ... args ) => result.push( args ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbackForAvailableDouble()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.once( 'available', ( ... args ) => result.push( args ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function callbacksForEvents()
  {
    const _ = require( toolsPath );
    const result = [];
    _.process.once( 'available', ( ... args ) => result.push( args ) );
    _.process.once( 'uncaughtError', ( e ) => result.push( e + result.length ) );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    _.process.eventGive( 'available', 'arg' );
    _.process.eventGive( 'uncaughtError', 'arg' );
    console.log( result );
  }

  /* */

  function uncaughtError()
  {
    const _ = require( toolsPath );
    _.process.once( 'uncaughtError', ( o ) => _.errAttend( o.err ) );
    throw _.err( 'Error' );
    console.log( 'exit' );
  }
}

//

function ready( test )
{
  let t1 = 100;
  let ready =  new __.Consequence().take( null );

  /* */

  ready.then( () =>
  {
    test.case = 'without arguments, must not throw error, no side effects only minimal delay';
    _.process.ready();
    return null;
  });

  /* */

  ready.then( () =>
  {
    test.case = 'only onReady, no timeOut';
    var arr = [];
    var onReady = () => arr.push( 1 );
    _.process.ready( onReady );

    return __.time.out( t1, () =>
    {
      test.identical( arr, [ 1 ] );
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'timeOut and onReady, onReady - null';
    var arr = [];
    _.process.ready( t1, null );
    test.identical( arr, [] );

    return __.time.out( t1 * 2, () =>
    {
      test.identical( arr, [] );
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'timeOut and onReady';
    var arr = [];
    var onReady = () => arr.push( 1 );
    _.process.ready( t1, onReady );
    test.identical( arr, [] );

    return __.time.out( t1 * 2, () =>
    {
      test.identical( arr, [ 1 ] );
      return null;
    });
  });

  /* */

  ready.then( () =>
  {
    test.case = 'only onReady, no timeOut';
    var arr = [];
    var onReady = () => arr.push( 1 );
    _.process.ready({ onReady });

    return __.time.out( t1, () =>
    {
      test.identical( arr, [ 1 ] );
      return null;
    });
  });

  ready.then( () =>
  {
    test.case = 'timeOut and onReady';
    var arr = [];
    var onReady = () => arr.push( 1 );
    _.process.ready({ timeOut : t1, onReady });
    test.identical( arr, [] );

    return __.time.out( t1 * 2, () =>
    {
      test.identical( arr, [ 1 ] );
      return null;
    });
  });

  /* - */

  if( !Config.debug )
  return;

  test.case = 'single arg is not a routine';
  test.shouldThrowErrorSync( () => _.process.ready( 1 ) );

  test.case = 'wrong type of timeOut';
  test.shouldThrowErrorSync( () => _.process.ready( 'wrong', () => 'ready' ) );
  test.shouldThrowErrorSync( () => _.process.ready( 10.5, () => 'ready' ) );
  test.shouldThrowErrorSync( () => _.process.ready( Infinity, () => 'ready' ) );

  test.case = 'wrong type of onReady';
  test.shouldThrowErrorSync( () => _.process.ready( 10, 'wrong' ) );

  test.case = 'not allowed options procedure';
  test.shouldThrowErrorSync( () => _.process.ready({ timeOut : 10, procedure : 'procedure', onReady : () => 'ready' }) );

  return ready;
}

// --
// declare
// --

const Proto =
{

  name : 'Tools.Process.l0.l9',
  silencing : 1,
  enabled : 1,
  routineTimeOut : 15000,

  tests :
  {

    // events

    onWithArguments,
    onWithOptionsMap,
    onWithChain,
    onCheckDescriptor,

    onceWithArguments,

    //

    ready,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();

