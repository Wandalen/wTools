( function _l0_l9_Event_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _ = _global_.wTools;

// --
// chain
// --

function chainIs( test )
{
  test.case = 'without arguments';
  var got = _.event.chainIs();
  test.identical( got, false );

  test.case = 'src - null';
  var got = _.event.chainIs( null );
  test.identical( got, false );

  test.case = 'src - string';
  var got = _.event.chainIs( 'begin' );
  test.identical( got, false );

  test.case = 'src - array of strings';
  var got = _.event.chainIs([ 'begin', 'end' ]);
  test.identical( got, false );

  test.case = 'src - object with field `chain`';
  var src = Object.create( null );
  src.chain = [ 'begin', 'end' ];
  var got = _.event.chainIs( src );
  test.identical( got, false );

  test.case = 'src - instance of class with field `chain`';
  function Chain()
  {
    this.chain = arguments;
    return this;
  }
  var src = new Chain( 'begin', 'end' );
  var got = _.event.chainIs( src );
  test.identical( got, false );

  test.case = 'src - instance of _.event.Chain';
  var got = _.event.chainIs( _.event.Chain( 'begin' ) );
  test.identical( got, true );
}

//

function Chain( test )
{
  test.case = 'single string';
  var got = _.event.Chain( 'begin' );
  test.true( _.event.chainIs( got ) );
  test.true( _.longIs( got.chain ) );
  test.identical( got.chain.length, 1 );
  test.identical( got.chain[ 0 ], 'begin' );

  test.case = 'single Chain';
  var chain = _.event.Chain( 'begin', 'end' );
  var got = _.event.Chain( chain );
  test.true( _.event.chainIs( got ) );
  test.true( got === chain );
  test.true( _.longIs( got.chain ) );
  test.identical( got.chain.length, 2 );
  test.identical( got.chain, [ 'begin', 'end' ] );

  /* */

  test.case = 'a few strings';
  var got = _.event.Chain( 'begin', 'end', 'error' );
  test.true( _.event.chainIs( got ) );
  test.true( _.longIs( got.chain ) );
  test.identical( got.chain.length, 3 );
  test.identical( got.chain[ 0 ], 'begin' );
  test.identical( got.chain[ 1 ], 'end' );
  test.identical( got.chain[ 2 ], 'error' );

  test.case = 'a few Names';
  var name1 = 'begin';
  var name2 = 'end';
  var name3 = 'error';
  var got = _.event.Chain( name1, name2, name3 );
  test.true( _.event.chainIs( got ) );
  test.true( _.longIs( got.chain ) );
  test.identical( got.chain.length, 3 );
  test.equivalent( got.chain, [ name1, name2, name3 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.event.Chain() );

  test.case = 'wrong type of element';
  test.shouldThrowErrorSync( () => _.event.Chain( null ) );
  test.shouldThrowErrorSync( () => _.event.Chain( 'begin', undefined ) );

  test.case = 'first argument is a Chain, arguments.length > 1';
  test.shouldThrowErrorSync( () => _.event.Chain( _.event.Chain( 'begin' ), 'end' ) );
}

// --
// event
// --

function on( test )
{
  var self = this;

  /* - */

  test.open( 'option first - 0' );

  test.case = 'no callback for events';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'single callback for single event, single event is given';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  /* */

  test.case = 'single callback for single event, a few events are given';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, 1 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, 1 ] );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  /* */

  test.case = 'single callback for each events in event handler, a few events are given';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var got = _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, 1 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, 1, -2, -3 ] );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  /* */

  test.case = 'several events handler with similar events, events are given in second handler';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var ehandler2 =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } } );
  _.event.eventGive( ehandler2, { event : 'event', args : [] } );
  _.event.eventGive( ehandler2, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( ehandler2, { event : 'event2', args : [] } );
  _.event.eventGive( ehandler2, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'event handler has a few handlers for different events - once and regular';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event2' : onEvent2 } } );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, 1 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, 1, -2 ] );

  test.case = 'event handler has a few handlers for single event - once and regular';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent2 } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1, 2 ] );

  test.close( 'option first - 0' );

  /* - */

  test.open( 'option first - 1' );

  test.case = 'callback for once added before other callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent2 }, 'first' : 1 } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ -0, 1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ -0, 1, 2 ] );

  /* */

  test.case = 'callback for once added after other callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent2 }, 'first' : 1 } );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ -0, 1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ -0, 1, -2 ] );

  test.close( 'option first - 1' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.event.on() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.event.on( { events : { begin : [] } } ) );

  test.case = 'wrong type of edispatcher';
  test.shouldThrowErrorSync( () => _.event.on( 'wrong', { callbackMap : { begin : () => true } } ) );

  test.case = 'wrong type of edispatcher.events';
  test.shouldThrowErrorSync( () => _.event.on( { events : null }, { callbackMap : { begin : () => true } } ) );

  test.case = 'wrong type of options map o';
  test.shouldThrowErrorSync( () => _.event.on( { events : { begin : [] } }, 'wrong' ) );

  test.case = 'extra options in options map o';
  test.shouldThrowErrorSync( () => _.event.on( { events : { begin : [] } }, { wrong : {} } ) );

  test.case = 'not known event in callbackMap';
  test.shouldThrowErrorSync( () =>
  {
    _.event.on( { events : { begin : [] } }, { callbackMap : { unknown : () => 'unknown' } } );
  });
}

//

function onCheckDescriptor( test )
{
  test.case = 'descriptor, check fields, not used method off and properties';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.on( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, -1 ] );

  /* */

  test.case = 'descriptor, use method off without arguments';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.on( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );

  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event2.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'descriptor, set property enabled to false and call off';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.on( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );

  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event.enabled = false;
  descriptor.event.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event2.enabled = false;
  descriptor.event2.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'descriptor, set property enabled to false';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.on( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );

  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event.enabled = false;
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event2.enabled = false;
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
}

//

function onWithChain( test )
{
  test.open( 'with string names' );

  test.case = 'chain with single step';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : [ onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, 1 ] );
  test.identical( edispatcher.events.event[ 0 ].native, onEvent );

  /* */

  test.case = 'chain with two steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : [ 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, 1 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );

  /* */

  test.case = 'chain with three steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [], 'event3' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : [ 'event3', 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );

  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, 1 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );

  test.close( 'with string names' );

  /* - */

  test.open( 'with instances of Name' );

  test.case = 'chain with two steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : [ 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, 1 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );

  /* */

  test.case = 'chain with three steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [], 'event3' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : [ 'event3', 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );

  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, 1 ] );
  test.identical( edispatcher.events.event2[ 0 ].native, onEvent );

  test.close( 'with instances of Name' );
}

//

function onCallWithHeadRoutine( test )
{
  test.case = 'head calls with options map';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  var o = _.event.onHead( _.event.on, [ { 'callbackMap' : { 'event' : onEvent } } ] );
  _.event.on( edispatcher, o );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, 1 ] );

  test.case = 'head calls with chain and callback';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  var chain = _.event.Chain( 'event2', 'event' );
  var o = _.event.onHead( _.event.on, [ chain, onEvent ] )
  _.event.on( edispatcher, o );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );

  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
}

//

function once( test )
{
  var self = this;

  /* - */

  test.open( 'option first - 0' );

  test.case = 'no callback for events';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'single callback for single event, single event is given';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );

  /* */

  test.case = 'single callback for single event, a few events are given';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );

  /* */

  test.case = 'single callback for each events in event handler, a few events are given';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, -1 ] );

  /* */

  test.case = 'several events handler with similar events, events are given in second handler';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var ehandler2 =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } } );
  _.event.eventGive( ehandler2, { event : 'event', args : [] } );
  _.event.eventGive( ehandler2, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( ehandler2, { event : 'event2', args : [] } );
  _.event.eventGive( ehandler2, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'event handler has a few handlers for different events - once and regular';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event2' : onEvent2 } } );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, -1, -2 ] );

  test.case = 'event handler has a few handlers for single event - once and regular';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent2 } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1, -2 ] );

  test.close( 'option first - 0' );

  /* - */

  test.open( 'option first - 1' );

  test.case = 'callback for once added before other callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent }, 'first' : 1 } );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent2 } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1, -2 ] );

  /* */

  test.case = 'callback for once added after other callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent2 } } );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent }, 'first' : 1 } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1, -2 ] );

  test.close( 'option first - 1' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.event.once() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.event.once( { events : { begin : [] } } ) );

  test.case = 'wrong type of edispatcher';
  test.shouldThrowErrorSync( () => _.event.once( 'wrong', { callbackMap : { begin : () => true } } ) );

  test.case = 'wrong type of edispatcher.events';
  test.shouldThrowErrorSync( () => _.event.once( { events : null }, { callbackMap : { begin : () => true } } ) );

  test.case = 'wrong type of options map o';
  test.shouldThrowErrorSync( () => _.event.once( { events : { begin : [] } }, 'wrong' ) );

  test.case = 'extra options in options map o';
  test.shouldThrowErrorSync( () => _.event.once( { events : { begin : [] } }, { wrong : {} } ) );

  test.case = 'not known event in callbackMap';
  test.shouldThrowErrorSync( () =>
  {
    _.event.once( { events : { begin : [] } }, { callbackMap : { unknown : () => 'unknown' } } );
  });
}

//

function onceCheckDescriptor( test )
{
  test.case = 'descriptor, check fields, not used method off and properties';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.once( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, -1 ] );

  /* */

  test.case = 'descriptor, use method off without arguments';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.once( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );

  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event2.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'descriptor, set property enabled to false and call off';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.once( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );

  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event.enabled = false;
  descriptor.event.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event2.enabled = false;
  descriptor.event2.off();
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.false( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'descriptor, set property enabled to false';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  var callbackMap = { 'callbackMap' : { 'event' : onEvent, 'event2' : onEvent2 } };
  var descriptor = _.event.once( edispatcher, callbackMap );

  test.identical( _.props.keys( descriptor ), [ 'event', 'event2' ] );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event.enabled, true );
  test.identical( descriptor.event.first, false );
  test.equivalent( descriptor.event.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event.off ) );
  test.identical( _.props.keys( descriptor.event ), [ 'off', 'enabled', 'first', 'callbackMap' ] );
  test.identical( descriptor.event2.enabled, true );
  test.identical( descriptor.event2.first, false );
  test.equivalent( descriptor.event2.callbackMap, callbackMap.callbackMap );
  test.true( _.routine.is( descriptor.event2.off ) );

  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event.enabled = false;
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );

  descriptor.event2.enabled = false;
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } ) );
  test.true( _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } ) );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
}

//

function onceWithChain( test )
{
  test.open( 'with string names' );

  test.case = 'chain with single step';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : [ onEvent ] } } );
  test.identical( edispatcher.events.event[ 0 ].native, onEvent );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event, [] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event, [] );

  /* */

  test.case = 'chain with few steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : [ 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.true( _.routine.is( edispatcher.events.event2[ 0 ] ) );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );

  /* */

  test.case = 'chain with three steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [], 'event3' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : [ 'event3', 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );

  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.true( _.routine.is( edispatcher.events.event2[ 0 ] ) );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );

  test.close( 'with string names' );

  /* - */

  test.open( 'with instances of Name' );

  test.case = 'chain with few steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : [ 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.true( _.routine.is( edispatcher.events.event2[ 0 ] ) );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );

  /* */

  test.case = 'chain with three steps';
  var edispatcher = { events : { 'event' : [], 'event2' : [], 'event3' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : [ 'event3', 'event2', onEvent ] } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );

  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event3', args : [] } );
  test.identical( result, [] );
  test.true( _.routine.is( edispatcher.events.event2[ 0 ] ) );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0 ] );
  test.identical( edispatcher.events.event2, [] );

  test.close( 'with instances of Name' );
}

//

function onceCallWithHeadRoutine( test )
{
  test.case = 'head calls with options map';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  var o = _.event.onHead( _.event.on, [ { 'callbackMap' : { 'event' : onEvent } } ] );
  _.event.once( edispatcher, o );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );

  test.case = 'head calls with chain and callback';
  var edispatcher = { events : { 'event' : [], 'event2' : [] } };
  var result = [];
  var onEvent = () => result.push( result.length );
  var chain = _.event.Chain( 'event2', 'event' );
  var o = _.event.onHead( _.event.on, [ chain, onEvent ] )
  _.event.once( edispatcher, o );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [] );

  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
}

//

function off( test )
{
  test.case = 'off all callbacks in event, events not exist';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [] },
  };
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );

  _.event.off( edispatcher, { 'callbackMap' : { 'event' : null } } );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );

  /* */

  test.case = 'off all callbacks in event, events exist';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [ onEvent, onEvent2 ] },
  };
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  _.event.off( edispatcher, { 'callbackMap' : { 'event' : null } } );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );

  /* */

  test.case = 'off individual callback from event';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [ onEvent, onEvent2 ] },
  };
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  _.event.off( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  /* */

  test.case = 'off individual callback added by routine on';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [] },
  };
  _.event.on( edispatcher, { callbackMap : { 'event' : onEvent } } )
  _.event.on( edispatcher, { callbackMap : { 'event' : onEvent2 } } )
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  _.event.off( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  /* */

  test.case = 'off individual callback added by routine once';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [] },
  };
  _.event.once( edispatcher, { callbackMap : { 'event' : onEvent } } )
  _.event.once( edispatcher, { callbackMap : { 'event' : onEvent2 } } )
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  _.event.off( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.event.off() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.event.off({ events : { event : [] } }) );

  test.case = 'extra arguments';
  var onEvent = () => true;
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( { events : { event : [ onEvent ] } }, { callbackMap : { event : onEvent } }, { extra : true } );
  });

  test.case = 'wrong type of edispatcher';
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( [ { events : { event : [ onEvent ] } } ], { callbackMap : { event : onEvent } } );
  });

  test.case = 'wrong type of edispatcher.events';
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( { events : [ { event : [ onEvent ] } ] }, { callbackMap : { event : onEvent } } );
  });

  test.case = 'unknown event';
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( { events : { event : [ onEvent ] } }, { callbackMap : { e1 : onEvent } } );
  });

  test.case = 'event added twice and try to remove once strictly';
  var onEvent = () => true;
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( { events : { event : [ onEvent, onEvent ] } }, { callbackMap : { event : onEvent } } );
  });

  test.case = 'try to remove not existed callback';
  var onEvent = () => true;
  var onEvent2 = () => false;
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( { events : { event : [ onEvent, onEvent ] } }, { callbackMap : { event : onEvent2 } } );
  });

  test.case = 'event added twice by routine on, try to remove once strictly';
  var onEvent = () => true;
  var handler = { events : { event : [] } };
  test.shouldThrowErrorSync( () =>
  {
    _.event.on( handler, { callbackMap : { event : onEvent } } );
    _.event.on( handler, { callbackMap : { event : onEvent } } );
    _.event.off( handler, { callbackMap : { event : onEvent } } );
  });

  test.case = 'event added twice by routine on, try to remove once strictly';
  var onEvent = () => true;
  var handler = { events : { event : [] } };
  test.shouldThrowErrorSync( () =>
  {
    _.event.once( handler, { callbackMap : { event : onEvent } } );
    _.event.once( handler, { callbackMap : { event : onEvent } } );
    _.event.off( handler, { callbackMap : { event : onEvent } } );
  });

  test.case = 'event added twice by routine on and once, try to remove once strictly';
  var onEvent = () => true;
  var handler = { events : { event : [] } };
  test.shouldThrowErrorSync( () =>
  {
    _.event.on( handler, { callbackMap : { event : onEvent } } );
    _.event.once( handler, { callbackMap : { event : onEvent } } );
    _.event.off( handler, { callbackMap : { event : onEvent } } );
  });

  test.case = 'wrong type of options map';
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( { events : { event : [ onEvent ] } }, [ { callbackMap : { event : onEvent } } ] );
  });

  test.case = 'wrong type of o.callbackMap';
  test.shouldThrowErrorSync( () =>
  {
    _.event.off( { events : { event : [ onEvent ] } }, { callbackMap : [ { event : onEvent } ] } );
  });
}

//

function eventHasHandler( test )
{
  test.case = 'handler has no callbacks';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [] },
  };
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );

  /* */

  test.case = 'handler has single callback for event';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [ onEvent ] },
  };
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );

  /* */

  test.case = 'handler has several callbacks for event';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [ onEvent, onEvent2 ] },
  };
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, true );

  /* */

  test.case = 'several events in event handler, different callbacks for each event';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [ onEvent ], 'event2' : [ onEvent2 ] },
  };
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent } );
  test.identical( got, false );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event2', eventHandler : onEvent2 } );
  test.identical( got, true );

  /* */

  test.case = 'event added by routine on';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [] },
  };
  _.event.on( edispatcher, { callbackMap : { event : onEvent } } )
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );

  /* */

  test.case = 'event added by routine once';
  var onEvent = () => 'true';
  var onEvent2 = () => 'false';
  var edispatcher =
  {
    events : { 'event' : [] },
  };
  _.event.once( edispatcher, { callbackMap : { event : onEvent } } )
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent } );
  test.identical( got, true );
  var got = _.event.eventHasHandler( edispatcher, { eventName : 'event', eventHandler : onEvent2 } );
  test.identical( got, false );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.eventHasHandler() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.eventHasHandler({ events : { event : [] } }) );

  test.case = 'extra arguments';
  var onEvent = () => true;
  test.shouldThrowErrorSync( () =>
  {
    _.eventHasHandler( { events : { event : [] } }, { eventName : 'event', eventHandler : onEvent }, { extra : {} } );
  });

  test.case = 'wrong type of edispatcher';
  var onEvent = () => true;
  var handler = { events : { event : [] } };
  test.shouldThrowErrorSync( () =>
  {
    _.eventHasHandler( [ handler ], { eventName : 'event', eventHandler : onEvent } );
  });

  test.case = 'wrong type of edispatcher.events';
  var onEvent = () => true;
  test.shouldThrowErrorSync( () =>
  {
    _.eventHasHandler( { events : [ { event : [] } ] }, { eventName : 'event', eventHandler : onEvent } );
  });

  test.case = 'unknown event';
  var onEvent = () => true;
  test.shouldThrowErrorSync( () =>
  {
    _.eventHasHandler( { events : { event : [] } }, { eventName : 'e1', eventHandler : onEvent } );
  });

  test.case = 'unknown property in options map';
  var onEvent = () => true;
  test.shouldThrowErrorSync( () =>
  {
    _.eventHasHandler( { events : { event : [] } }, { eventName : 'event', eventHandler : onEvent, unknown : true } );
  });

  test.case = 'wrong type of o.eventName';
  var onEvent = () => true;
  test.shouldThrowErrorSync( () =>
  {
    _.eventHasHandler( { events : { event : [] } }, { eventName : [ 'event' ], eventHandler : onEvent } );
  });

  test.case = 'wrong type of o.eventHandler';
  test.shouldThrowErrorSync( () =>
  {
    _.eventHasHandler( { events : { event : [] } }, { eventName : 'event', eventHandler : 'onEvent' } );
  });
}

//

function eventGiveHead( test )
{
  var edispatcher = { events : { event : [] } };

  /* */

  test.case = 'args contains single string';
  var got = _.event.eventGiveHead( edispatcher, _.event.eventGive, [ 'event' ] );
  test.identical( _.props.keys( got ), [ 'event', 'onError', 'args' ] );
  test.identical( got.event, 'event' );
  test.identical( got.args, [ 'event' ] );
  test.true( _.routine.is( got.onError ) );

  test.case = 'args contains many arguments';
  var got = _.event.eventGiveHead( edispatcher, _.event.eventGive, [ 'event', 1, 2 ] );
  test.identical( _.props.keys( got ), [ 'event', 'onError', 'args' ] );
  test.identical( got.event, 'event' );
  test.identical( got.args, [ 'event', 1, 2 ] );
  test.true( _.routine.is( got.onError ) );

  /* */

  test.case = 'args contains options map with only field event';
  var got = _.event.eventGiveHead( edispatcher, _.event.eventGive, [ { event : 'event' } ] );
  test.identical( _.props.keys( got ), [ 'event', 'onError', 'args' ] );
  test.identical( got.event, 'event' );
  test.identical( got.args.length, 1 );
  test.identical( got.args[ 0 ], got );
  test.true( _.routine.is( got.onError ) );

  test.case = 'args contains options map with fields event and onError';
  var onError = ( err ) => { _.errAttend( err ); return 0 };
  var got = _.event.eventGiveHead( edispatcher, _.event.eventGive, [ { event : 'event', onError } ] );
  test.identical( _.props.keys( got ), [ 'event', 'onError', 'args' ] );
  test.identical( got.event, 'event' );
  test.identical( got.args.length, 1 );
  test.identical( got.args[ 0 ], got );
  test.identical( got.onError, onError );

  test.case = 'args contains options map with fields event and args';
  var got = _.event.eventGiveHead( edispatcher, _.event.eventGive, [ { event : 'event', args : [ 1, 2 ] } ] );
  test.identical( _.props.keys( got ), [ 'event', 'args', 'onError' ] );
  test.identical( got.event, 'event' );
  test.identical( got.args, [ 1, 2 ] );
  test.true( _.routine.is( got.onError ) );

  test.case = 'args contains options map with fields event, args and onError';
  var onError = ( err ) => { _.errAttend( err ); return 0 };
  var got = _.event.eventGiveHead( edispatcher, _.event.eventGive, [ { event : 'event', args : [ 1, 2 ], onError } ] );
  test.identical( _.props.keys( got ), [ 'event', 'args', 'onError' ] );
  test.identical( got.event, 'event' );
  test.identical( got.args, [ 1, 2 ] );
  test.identical( got.onError, onError );

  /* */

  test.case = 'options map in args[ 0 ] contains fields of routine.defaults';
  var routine = ( arg ) => arg;
  routine.defaults = { event : null, args : null, onError : null, additional : null };
  var got = _.event.eventGiveHead( edispatcher, routine, [ { event : 'event', additional : 1 } ] );
  test.identical( _.props.keys( got ), [ 'event', 'additional', 'onError', 'args' ] );
  test.identical( got.event, 'event' );
  test.identical( got.additional, 1 );
  test.identical( got.args.length, 1 );
  test.identical( got.args[ 0 ], got );
  test.true( _.routine.is( got.onError ) );

  /* - */

  if( !Config.debug )
  return;

  var handler = { events : { event : [] } };

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler ) );
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.event.eventGive ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.event.eventGive, [ 'event' ], [ 'extra' ] ) );

  test.case = 'edispatcher has no field events';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( {}, _.event.eventGive, [ 'event' ] ) );

  test.case = 'routine has no defaults';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.long.is, [ 'event' ] ) );

  test.case = 'args - empty array';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.event.eventGive, [] ) );

  test.case = 'wrong type of args[ 0 ]';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.event.eventGive, [ 1 ] ) );
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.event.eventGive, [ [] ] ) );

  test.case = 'optons map in args[ 0 ] has extra fields in relation to routine';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.event.eventGive, [ { event : 'exit', unknown : 1 } ] ) );

  test.case = 'optons map in args[ 0 ] has field args, args length > 1';
  test.shouldThrowErrorSync( () => _.event.eventGiveHead( handler, _.event.eventGive, [ { event : 'exit', args : [] }, 1 ] ) );
}

//

function eventGive( test )
{
  test.open( 'event type - on' );

  test.case = 'give event for event without callbacks';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'give event for event with single callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, 1 ] );

  /* */

  test.case = 'give event for event with several callbacks for event';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent2 } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1, 2, -3 ] );

  /* */

  test.case = 'give event, use default args in callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = ( arg ) => result.push( arg );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 'event', 1 ] } );
  test.identical( result, [ 'event' ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 'event2', 2 ] } );
  test.identical( result, [ 'event', 'event2' ] );

  /* */

  test.case = 'give event, use not default args in callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = ( e1, e2 ) => result.push( e1, e2 );
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 1, 2 ] } );
  test.identical( result, [ 1, 2 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 1, 2, undefined, undefined ] );

  test.close( 'event type - on' );

  /* - */

  test.open( 'event type - once' );

  test.case = 'give event for event without callbacks';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [] );

  /* */

  test.case = 'give event for event with single callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0 ] );

  /* */

  test.case = 'give event for event with several callbacks for event';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = () => result.push( result.length );
  var onEvent2 = () => result.push( -1 * result.length );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent2 } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event2', args : [] } );
  test.identical( result, [ 0, -1 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 0, -1 ] );

  /* */

  test.case = 'give event, use default args in callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = ( arg ) => result.push( arg );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 'event1', 1 ] } );
  test.identical( result, [ 'event1' ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 'event2', 2 ] } );
  test.identical( result, [ 'event1' ] );

  /* */

  test.case = 'give event, use not default args in callback';
  var edispatcher =
  {
    events : { 'event' : [], 'event2' : [] },
  };
  var result = [];
  var onEvent = ( e1, e2 ) => result.push( e1, e2 );
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 1, 2 ] } );
  test.identical( result, [ 1, 2 ] );
  _.event.eventGive( edispatcher, { event : 'event', args : [] } );
  test.identical( result, [ 1, 2 ] );

  test.close( 'event type - once' );

  /* - */

  test.case = 'event - on, callback throws error, onError attends error';
  var edispatcher =
  {
    events : { event : [] },
  };
  var result = [];
  var onEvent = () => { throw _.err( 'event err' ) };
  var onError = ( err ) => { _.error.attend( err ); result.push( err ) };
  _.event.on( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 1, 2 ], onError } );
  test.identical( result.length, 1 );
  test.true( _.error.is( result[ 0 ] ) );

  test.case = 'event - once, callback throws error, onError attends error';
  var edispatcher =
  {
    events : { event : [] },
  };
  var result = [];
  var onEvent = () => { throw _.err( 'event err' ) };
  var onError = ( err ) => { _.error.attend( err ); result.push( err ) };
  _.event.once( edispatcher, { 'callbackMap' : { 'event' : onEvent } } );
  _.event.eventGive( edispatcher, { event : 'event', args : [ 1, 2 ], onError } );
  test.identical( result.length, 1 );
  test.true( _.error.is( result[ 0 ] ) );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.event.eventGive() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.event.eventGive({ events : { 'event' : [] } }) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.event.eventGive( { events : { 'event' : [] } }, 'event', 'extra' ) );

  test.case = 'unknown event in event handler';
  test.shouldThrowErrorSync( () => _.event.eventGive( { events : { 'event' : [] } }, 'event2' ) );

  test.case = 'wrong type of o.args';
  test.shouldThrowErrorSync( () => _.event.eventGive( { events : { 'event' : [] } }, { event : 'event', args : 'wrong' } ) );

  test.case = 'callback throws error';
  var onErrorCallback = ( err, arg ) =>
  {
    test.true( _.error.is( err ) );
    test.identical( err.originalMessage, 'Unknown event undefined' );
  };
  test.shouldThrowErrorSync( () =>
  {
    var handler = { events : { event : [ () => { throw _.err( 'err' ) } ] } };
    _.event.eventGive( handler, 'event' );
  }, onErrorCallback );
  // qqq2 : for Dmytro : ask /* aaa : Dmytro : fixed */

  test.case = 'callback throws error, onError handler does not attend error';
  var onErrorCallback = ( err, arg ) =>
  {
    test.true( _.error.is( err ) );
    test.identical( err.originalMessage, 'event err' );
  };
  test.shouldThrowErrorSync( () =>
  {
    var handler = { events : { event : [ () => { throw _.err( 'event err' ) } ] } };
    _.event.eventGive( handler, { event : 'event', args : [], onError : ( err ) => { throw _.err( err ) } } );
  }, onErrorCallback );
}

// --
// declare
// --

/* aaa2 : for Dmytro : repair */ /* Dmytro : regressions is fixed */
const Proto =
{

  name : 'Tools.Event.l0.l3',
  silencing : 1,
  enabled : 1,

  tests :
  {

    // chain

    chainIs,
    Chain,

    // event

    on,
    onCheckDescriptor,
    onWithChain,
    onCallWithHeadRoutine,

    once,
    onceCheckDescriptor,
    onceWithChain,
    onceCallWithHeadRoutine,

    off,

    eventHasHandler,
    eventGiveHead,
    eventGive,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
