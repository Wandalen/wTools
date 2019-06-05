( function _Diagnostics_test_s_( ) {

'use strict';

if( typeof module !== 'undefined' )
{
  let _ = require( '../../Tools.s' );
  _.include( 'wTesting' );
}

var _global = _global_;
var _ = _global_.wTools;

//

function _err( test )
{

  var errObj1 = new Error( 'err obj for tesst' );
  var errMsg2 = errObj1.message;

  var errMsg1 = 'some error message',
    strName = 'Diagnostics.test.s',
    errObj2 = new Error( 'Error #3' ),
    errMsg3 = errObj2.message,
    optionsObj3 =
  {
      level : 1,
      args : [ errObj1 ]
  },

    optionsObj4 =
  {
      level : 3,
      args : [ errMsg1, errObj2 ]
  };

  /* - */

  test.case = 'single string passed as args property : result should be Error obj';
  var optionsObj2 =
  {
    level : 1,
    args : [ errMsg1 ]
  };
  var got = _._err( optionsObj2 );
  test.identical( got instanceof Error, true );

  test.case = 'single string passed as args property : result message should contains passed string';
  var expectMsg = new RegExp( errMsg1 );
  test.identical( expectMsg.test( got.message ), true );

  test.case = 'single string passed as args property : result message should contains file name';
  var expectFileName = new RegExp( strName );
  test.identical( expectFileName.test( got.message ), true );

  test.case = 'single error instance passed as args property : result should be Error obj';
  var got = _._err( optionsObj3 );
  test.identical( got instanceof Error, true );

  test.case = 'single error instance passed as args property : result message should contains passed string';
  var expectMsg = new RegExp( errMsg2 );
  test.identical( expectMsg.test( got.message ), true );

  test.case = 'single error instance passed as args property : result message should contains file name';
  test.identical( _.strHas( got.message, errObj1.location.path ), true );

  /* - */

  test.open( 'several error instances/messages passed as args property' );
  var got = _._err( optionsObj4 );
  test.identical( got instanceof Error, true );

  test.case = 'result message should contains all passed string';
  var expectMsg1 = new RegExp( errMsg3 );
  var expectMsg2 = new RegExp( errMsg1 );
  test.identical( [ expectMsg1.test( got.message ), expectMsg2.test( got.message ) ], [ true, true ] );

  test.case = 'result message should contains file name';
  var expectFileName = new RegExp( strName );
  test.identical( expectFileName.test( got.message ), true );

  test.close( 'several error instances/messages passed as args property' );

  /* - */

  var optionsObj1 =
  {
    level : 1,
    args : null
  };

  if( !Config.debug )
  return;

  test.case = 'missed argument';
  test.shouldThrowError( function( )
  {
    _._err( );
  } );

  test.case = 'extra argument';
  test.shouldThrowError( function( )
  {
    _._err( optionsObj1, optionsObj2 );
  } );

  test.case = 'options.args not array';
  test.shouldThrowError( function( )
  {
    _._err( optionsObj1 );
  } );

}

//

function err( test )
{
  var errMsg1 = 'some error message',
    strName = 'Diagnostics.test.s',
    errObj1 = new Error( 'err obj for tesst' ),
    errMsg2 = errObj1.message,
    errObj2 = new Error( 'Error #3' ),
    errMsg3 = errObj2.message;

  test.case = 'single string passed as args property : result should be Error obj';
  var got = _.err( errMsg1 );
  test.identical( got instanceof Error, true );

  test.case = 'single string passed as args property : result message should contains passed string';
  var expectMsg = new RegExp( errMsg1 );
  test.identical( expectMsg.test( got.message ), true );

  test.case = 'single string passed as args property : result message should contains file name';
  var expectFileName = new RegExp( strName );
  test.identical( expectFileName.test( got.message ), true );

  test.case = 'single error instance passed as args property : result should be Error obj';
  var got = _.err( errObj1 );
  test.identical( got instanceof Error, true );

  test.case = 'single error instance passed as args property : result message should contains passed string';
  var expectMsg = new RegExp( errMsg2 );
  test.identical( expectMsg.test( got.message ), true );

  test.case = 'single error instance passed as args property : result message should contains file name';
  test.identical( _.strHas( got.message,errObj1.location.path ), true );

  test.case = 'several error instances/messages passed as args property : result should be Error obj';
  var got = _.err( errObj2, errMsg1 );
  test.identical( got instanceof Error, true );

  test.case = 'several error instances/messages passed as args : result message should contains all ' +
    'passed string';
  var expectMsg1 = new RegExp( errMsg3 ),
    expectMsg2 = new RegExp( errMsg1 );
  test.identical( [ expectMsg1.test( got.message ), expectMsg2.test( got.message ) ], [ true, true ] );

  test.case = 'several error instances/messages passed as args property : result message should contains ' +
    'file name';
  var expectFileName = new RegExp( strName );
  test.identical( expectFileName.test( got.message ), true );

};

//

function errLog( test )
{
  var errMsg1 = 'some error message',
    strName = 'Diagnostics.test.s',
    errObj1 = new Error( 'err obj for tesst' ),
    errMsg2 = errObj1.message;


  test.case = 'single string passed as args property : result should be Error obj';
  var got = _.errLog( errMsg1 );
  test.identical( got instanceof Error, true );

  test.case = 'single string passed as args property : result message should contains passed string';
  var expectMsg = new RegExp( errMsg1 );
  test.identical( expectMsg.test( got.message ), true );

  test.case = 'single string passed as args property : result message should contains file name';
  var expectFileName = new RegExp( strName );
  test.identical( expectFileName.test( got.message ), true );

  test.case = 'single error instance passed as args property : result should be Error obj';
  var got = _.errLog( errObj1 );
  test.identical( got instanceof Error, true );

  test.case = 'single error instance passed as args property : result message should contains passed string';
  var expectMsg = new RegExp( errMsg2 );
  test.identical( expectMsg.test( got.message ), true );

  test.case = 'single error instance passed as args property : result message should contains file name';
  test.identical( _.strHas( got.message,errObj1.location.path ), true );

}
    
//
    
function sureMapHasExactly( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapHasExactly( srcMap, screenMap ), true );
  test.identical( _.sureMapHasExactly( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapHasExactly( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapHasExactly( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
  
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapHasExactly( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should have no fields : "d"' ), true );
    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b }; 
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapHasExactly( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b }; 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasExactly( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "d"' ), true );
    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b }; 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasExactly( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "d"' ), true );
    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b }; 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasExactly( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "d"' ), true );
    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b }; 
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapHasExactly( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}
    
//
    
function sureMapOwnExactly( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapOwnExactly( srcMap, screenMap ), true );
  test.identical( _.sureMapOwnExactly( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapOwnExactly( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapOwnExactly( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapOwnExactly( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should own no fields : "d"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapOwnExactly( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnExactly( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnExactly( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnExactly( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "d"' ), true );
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapOwnExactly( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}

//
    
function sureMapHasOnly( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapHasOnly( srcMap, screenMap ), true );
  test.identical( _.sureMapHasOnly( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapHasOnly( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapHasOnly( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapHasOnly( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should have no fields : "d"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapHasOnly( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasOnly( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasOnly( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasOnly( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "d"' ), true );
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapHasOnly( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}

//
  
function sureMapOwnOnly( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapOwnOnly( srcMap, screenMap ), true );
  test.identical( _.sureMapOwnOnly( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapOwnOnly( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapOwnOnly( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapOwnOnly( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should own no fields : "d"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapOwnOnly( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnOnly( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnOnly( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnOnly( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "d"' ), true );
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapOwnOnly( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}
    
//
  
function sureMapHasAll( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapHasAll( srcMap, screenMap ), true );
  test.identical( _.sureMapHasAll( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapHasAll( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapHasAll( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapHasAll( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should have fields : "name"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapHasAll( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "name"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasAll( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "name"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasAll( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "name"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasAll( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "name"' ), true );
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapHasAll( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}
    
//
  
function sureMapOwnAll( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapOwnAll( srcMap, screenMap ), true );
  test.identical( _.sureMapOwnAll( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapOwnAll( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapOwnAll( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapOwnAll( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should own fields : "name"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapOwnAll( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "name"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnAll( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "name"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnAll( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "name"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnAll( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "name"' ), true );
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapOwnAll( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}
    
//
  
function sureMapHasNone( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'e' : 13, 'f' : 77, 'g' : 3, 'h' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapHasNone( srcMap, screenMap ), true );
  test.identical( _.sureMapHasNone( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapHasNone( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapHasNone( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapHasNone( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should have no fields : "a", "b", "c"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapHasNone( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "a", "b", "c"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasNone( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "a", "b", "c"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasNone( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "a", "b", "c"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasNone( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "a", "b", "c"' ), true );
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapHasNone( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}
    
//
  
function sureMapOwnNone( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'e' : 13, 'f' : 77, 'g' : 3, 'h' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapOwnNone( srcMap, screenMap ), true );
  test.identical( _.sureMapOwnNone( srcMap, screenMap, msg ), true );
  test.identical( _.sureMapOwnNone( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.sureMapOwnNone( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapOwnNone( srcMap, screenMaps )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should own no fields : "a", "b", "c"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapOwnNone( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "a", "b", "c"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnNone( srcMap, screenMaps, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "a", "b", "c"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnNone( srcMap, screenMaps, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "a", "b", "c"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapOwnNone( srcMap, screenMaps, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "a", "b", "c"' ), true );
    
  test.case = 'check error message, five or more arguments';
  try
  {
    _.sureMapOwnNone( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );
}

//
  
function sureMapHasNoUndefine( test )
{        
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var otherMap = { 'd' : undefined };
  var msg = function(){ return srcMap.a + srcMap.b };    
  
  test.case = 'correct input';
  test.identical( _.sureMapHasNoUndefine( srcMap), true );
  test.identical( _.sureMapHasNoUndefine( srcMap, msg ), true );
  test.identical( _.sureMapHasNoUndefine( srcMap, msg, 'msg' ), true );
  test.identical( _.sureMapHasNoUndefine( srcMap, () => 'This is ' + 'explanation' ), true ); 
    
  test.case = 'check error message, no msg';
  try
  {
    _.sureMapHasNoUndefine( otherMap )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Object should have no undefines, but has : "d"' ), true );
    
  test.case = 'check error message, msg routine';
  try
  {
    _.sureMapHasNoUndefine( otherMap, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
    
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasNoUndefine( otherMap, 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasNoUndefine( otherMap, 'msg', msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'msg 90 "d"' ), true );
    
  test.case = 'check error message, msg string';
  try
  {
    _.sureMapHasNoUndefine( otherMap, () => 'This is ' + 'explanation' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'This is explanation "d"' ), true );
    
  test.case = 'check error message, four or more arguments';
  try
  {
    _.sureMapHasNoUndefine( srcMap, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects one, two or three arguments' ), true );
}
    
//
    
function assertMapHasFields( test )
{    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };
    
  test.case = 'correct input';
  test.identical( _.assertMapHasFields( srcMap, screenMap ), true );
  test.identical( _.assertMapHasFields( srcMap, screenMap, msg ), true );
  test.identical( _.assertMapHasFields( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.assertMapHasFields( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
  
  test.case = 'check error message assertMapHasFields';
  try
  {
    _.assertMapHasFields( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
  test.case = 'check error message, four or more arguments';
  try
  {
    _.assertMapHasFields( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );  
}
    
//
    
function assertMapOwnFields( test )
{    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };
    
  test.case = 'correct input';
  test.identical( _.assertMapOwnFields( srcMap, screenMap ), true );
  test.identical( _.assertMapOwnFields( srcMap, screenMap, msg ), true );
  test.identical( _.assertMapOwnFields( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.assertMapOwnFields( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
  
  test.case = 'check error message assertMapHasFields';
  try
  {
    _.assertMapOwnFields( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
  test.case = 'check error message, four or more arguments';
  try
  {
    _.assertMapOwnFields( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );  
}
    
//
    
function assertMapHasOnly( test )
{    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var msg = function(){ return srcMap.a + screenMaps.b };
    
  test.case = 'correct input';
  test.identical( _.assertMapHasOnly( srcMap, screenMap ), true );
  test.identical( _.assertMapHasOnly( srcMap, screenMap, msg ), true );
  test.identical( _.assertMapHasOnly( srcMap, screenMap, msg, 'msg' ), true );
  test.identical( _.assertMapHasOnly( srcMap, screenMap, () => 'This is ' + 'explanation' ), true ); 
  
  test.case = 'check error message assertMapHasFields';
  try
  {
    _.assertMapHasOnly( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true ); 
    
  test.case = 'check error message, four or more arguments';
  try
  {
    _.assertMapHasOnly( srcMap, screenMaps, msg, 'msg', 'msg' )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, 'Expects two, three or four arguments' ), true );  
}
    
//
    
function assertSurer( test )
{    
  var srcMap = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var srcMap1 = { 'a' : 13, 'b' : 77, 'c' : 3, 'd' : 'Mikle' };
  var screenMaps = { 'a' : 13, 'b' : 77, 'c' : 3, 'name' : 'Hello' };
  var otherMap = { 'd' : undefined };
  var msg = function(){ return srcMap.a + screenMaps.b };   
    
  test.case = 'check error message assertMapOwnOnly';
  try
  {
    _.assertMapOwnOnly( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "d"' ), true );
    
//  test.case = 'check error message sureMapHasAll';
//  try
//  {
//    _.sureMapHasAll( srcMap, screenMaps, msg )
//  }
//  catch ( e )
//  {
//    err = e;
//  }
//  test.identical( err instanceof Error, true );
//  test.identical( _.strHas( err.message, '90 "name"' ), true );
//
//  test.case = 'check sureMapOwnAll';
//  try
//  {
//    _.sureMapOwnAll( srcMap, screenMaps, msg )
//  }
//  catch ( e )
//  {
//    err = e;
//  }
//  test.identical( err instanceof Error, true );
//  test.identical( _.strHas( err.message, '90 "name"' ), true );
   
  test.case = 'check error message assertMapHasNone';
  try
  {
    _.assertMapHasNone( srcMap, screenMaps, msg )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );
  test.identical( _.strHas( err.message, '90 "a", "b", "c"' ), true );
   
//  test.case = 'check sureMapOwnNone';
//  try
//  {
//    _.sureMapOwnNone( srcMap, screenMaps, msg )
//  }
//  catch ( e )
//  {
//    err = e;
//  }
//  test.identical( err instanceof Error, true );
//  test.identical( _.strHas( err.message, '90 "a", "b", "c"' ), true );  
//    
//  test.case = 'check sureMapHasNoUndefine';
//  try
//  {
//    _.sureMapHasNoUndefine( otherMap, msg )
//  }
//  catch ( e )
//  {
//    err = e;
//  }
//  test.identical( err instanceof Error, true );
//  test.identical( _.strHas( err.message, 'Object 90 : "d"' ), true );
}
    
//

function assert( test )
{
  var err;
  var msg1 = 'short error description';
  var rgMsg1 = new RegExp( msg1 );

  test.case = 'assert successful condition';
  var got = _.assert( 5 === 5 );
  test.identical( got, true );

  test.case = 'passed failure condition : should generates exception';
  try
  {
    _.assert( 5 != 5 )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( err instanceof Error, true );

  test.case = 'passed failure condition with passed message : should generates exception with message';
  try
  {
    _.assert( false, msg1 )
  }
  catch ( e )
  {
    err = e;
  }
  test.identical( rgMsg1.test( err.message ), true );
};

//

function diagnosticStack( test )
{
  function function1( )
  {
    return function2( );
  }

  function function2( )
  {
    return function3( );
  }

  function function3( )
  {
    debugger;
    return _.diagnosticStack();
  }

  /* - */

  test.case = 'trivial';
  var expectedTrace = [ 'function3', 'function2', 'function1', 'Diagnostics.test.s' ];
  var got = function1();
  got = got.split( '\n' );
  debugger;
  expectedTrace.forEach( function( expectedStr, i )
  {
    var expectedRegexp = new RegExp( expectedStr );
    test.description = expectedStr;
    test.identical( expectedRegexp.test( got[ i ] ), true );
  });
  debugger;

  /* - */

  // test.case = 'second';
  // var got = function1();
  // debugger;
  // got = got.split( '\n' ).slice( -5, -1 ).join( '\n' );
  // debugger;
  // expectedTrace.forEach( function( expectedStr, i )
  // {
  //   var expectedRegexp = new RegExp( expectedStr );
  //   test.identical( expectedRegexp.test( got[ i ] ), true );
  // });

};

//

var Self =
{

  name : 'Tools/base/l1/Diagnostics',
  silencing : 1,

  tests :
  {

    _err   : _err,
    err    : err,
    errLog : errLog,
    
    sureMapHasExactly : sureMapHasExactly,
    sureMapOwnExactly : sureMapOwnExactly,
    sureMapHasOnly : sureMapHasOnly,
    sureMapOwnOnly : sureMapOwnOnly,
    sureMapHasAll : sureMapHasAll,
    sureMapOwnAll : sureMapOwnAll,
    sureMapHasNone : sureMapHasNone,
    sureMapOwnNone : sureMapOwnNone,
    sureMapHasNoUndefine : sureMapHasNoUndefine,
    
    assertMapHasFields : assertMapHasFields,
    assertMapOwnFields : assertMapOwnFields,
    assertMapHasOnly : assertMapHasOnly,
    assertSurer : assertSurer,

    assert : assert,
    diagnosticStack  : diagnosticStack

  }

}

Self = wTestSuite( Self );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

} )( );
