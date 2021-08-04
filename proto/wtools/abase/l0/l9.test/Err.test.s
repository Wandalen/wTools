( function _l0_l9_Err_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _ = _global_.wTools;
const __ = _globals_.testing.wTools;
const fileProvider = __.fileProvider;
const path = fileProvider.path;

// --
// context
// --

function onSuiteBegin()
{
  let context = this;

  context.suiteTempPath = path.tempOpen( path.join( __dirname, '../..' ), 'err' );
  context.assetsOriginalPath = path.join( __dirname, '_asset' );

}

//

function onSuiteEnd()
{
  let context = this;
  _.assert( _.strHas( context.suiteTempPath, '/err-' ) )
  path.tempClose( context.suiteTempPath );
}

// --
// tests
// --

function errStackAndSourceTrivial( test )
{
  let context = this;
  let visited = [];

  var args = [ 'str', { num : 1 } ];
  var err = _._err({ args });
  test.true( _.error.is( err ) );

  var errStr = String( err );
  console.log( errStr );
  test.identical( _.strCount( errStr, 'at Object.errStackAndSourceTrivial' ), 2 );
  test.identical( _.strCount( errStr, /\* \d+ :   var err = _\._err\({ args }\);/ ), 1 );

}

//

function errFromStringedError( test )
{
  let context = this;
  let visited = [];

  var msg =
`
Uncaught Error:  = Message of Error#1
No source file found for "W1.js"
Error including source file /workerEnvironment/Worker.js

 = Beautified calls stack
at Object._broInclude (http://127.0.0.1:15000/.starter:6144:17)
at Object._sourceInclude (http://127.0.0.1:15000/.starter:6575:20)
at SourceFile.Worker_js_naked [as nakedCall] (http://127.0.0.1:15000/workerEnvironment/Worker.js:14:1)
at Object._sourceIncludeAct (http://127.0.0.1:15000/.starter:6529:19)
at Worker_js (http://127.0.0.1:15000/workerEnvironment/Worker.js:27:18)
at http://127.0.0.1:15000/workerEnvironment/Worker.js:28:38

 = Throws stack
thrown at Object._broInclude @ http://127.0.0.1:15000/.starter:6144:17
thrown at Object._sourceIncludeAct @ http://127.0.0.1:15000/.starter:6538:15
`;
  var args = [ msg ];
  var o =
  {
    args,
    level : 1,
    fallBackStack : 'at handleError @ http://127.0.0.1:15000/.starter:2918',
    throwLocation :
    {
      filePath : 'http://127.0.0.1:15000/.starter',
      line : 2918,
      col : 23,
    },
  };
  var err = _._err( o );
  test.true( _.error.is( err ) );
  var got = String( err );

  test.identical( _.strCount( got, `= Message of Error#${err.id}` ), 1 );
  test.identical( _.strCount( got, 'Uncaught Error:' ), 1 );
  test.identical( _.strCount( got, 'No source file found for "W1.js"' ), 1 );
  test.identical( _.strCount( got, 'Error including source file /workerEnvironment/Worker.js' ), 1 );

  var exp =
`
 = Beautified calls stack
    at Object._broInclude (http://127.0.0.1:15000/.starter:6144:17)
    at Object._sourceInclude (http://127.0.0.1:15000/.starter:6575:20)
    at SourceFile.Worker_js_naked [as nakedCall] (http://127.0.0.1:15000/workerEnvironment/Worker.js:14:1)
    at Object._sourceIncludeAct (http://127.0.0.1:15000/.starter:6529:19)
    at Worker_js (http://127.0.0.1:15000/workerEnvironment/Worker.js:27:18)
    at http://127.0.0.1:15000/workerEnvironment/Worker.js:28:38
`
  test.true( _.strHas( got, exp ) );

  var exp =
`
 = Throws stack
    thrown at Object._broInclude @ http://127.0.0.1:15000/.starter:6144:17
    thrown at Object._sourceIncludeAct @ http://127.0.0.1:15000/.starter:6538:15
    thrown at Object.errFromStringedError @ ${ o.catchLocation.filePathLineCol }

`
  test.true( _.strHas( got, exp ) );
  test.identical( _.strCount( got, '= Beautified calls stack' ), 1 );
  test.identical( _.strCount( got, '= Throws stack' ), 1 );

}

//

function _errWithArgsIncludedRoutine( test )
{
  let context = this;
  let a = test.assetFor( false );
  a.fileProvider.dirMake( a.abs( '.' ) );
  let locals = { toolsPath : _.module.resolve( 'wTools' ) };
  let programPath1 = a.path.nativize( a.program( testRoutineWithStrLinesSelect ).filePath/*programPath*/ );
  let programPath2 = a.path.nativize( a.program( testRoutineWithoutStrLinesSelect ).filePath/*programPath*/ );

  a.shellNonThrowing( `node ${ programPath1 }` );
  a.ready.then( ( op ) =>
  {
    test.case = 'formatting routine throws error';
    test.notIdentical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'one' ), 1 );
    test.identical( _.strCount( op.output, 'two' ), 1 );
    test.identical( _.strCount( op.output, 'three' ), 1 );
    test.identical( _.strCount( op.output, 'four' ), 1 );
    test.identical( _.strCount( op.output, '1 : function msgRoutine()' ), 1 );
    test.identical( _.strCount( op.output, '2 :     {' ), 1 );
    test.identical( _.strCount( op.output, 'Error throwen by callback for formatting of error string' ), 1 );
    test.identical( _.strCount( op.output, 'at testRoutineWithStrLinesSelect' ), 2 );
    return null;
  });

  a.shellNonThrowing( `node ${ programPath2 }` );
  a.ready.then( ( op ) =>
  {
    console.log( op.output );
    test.case = 'formatting routine throws error';
    test.notIdentical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'one' ), 2 );
    test.identical( _.strCount( op.output, 'two' ), 2 );
    test.identical( _.strCount( op.output, 'three' ), 2 );
    test.identical( _.strCount( op.output, 'four' ), 2 );
    test.identical( _.strCount( op.output, 'function msgRoutine()' ), 1 );
    test.identical( _.strCount( op.output, 'let message =' ), 1 );
    test.identical( _.strCount( op.output, '`;' ), 1 );
    test.identical( _.strCount( op.output, 'throw message;' ), 1 );
    test.identical( _.strCount( op.output, 'Error throwen by callback for formatting of error string' ), 1 );
    test.identical( _.strCount( op.output, 'at testRoutineWithoutStrLinesSelect' ), 2 );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function testRoutineWithStrLinesSelect()
  {
    let _ = require( toolsPath );

    throw _._err( { args : [ msgRoutine ] } );

    /* */

    function msgRoutine()
    {
      let message =
  `
  one
  two
  three
  four

  `;
      throw message;
    }
  }

  /* */

  function testRoutineWithoutStrLinesSelect()
  {
    let _ = require( toolsPath );
    _.strLinesSelect = null;

    throw _._err( { args : [ msgRoutine ] } );

    /* */

    function msgRoutine()
    {
      let message =
  `
  one
  two
  three
  four

  `;
      throw message;
    }
  }
}

_errWithArgsIncludedRoutine.timeOut = 20000;

//

function _errArgsHasError( test )
{

  test.case = 'empty args, throwCallsStack - undefined, catchCallsStack - string';
  var err = _._err( { args : [ new Error() ], catchCallsStack : 'at program\nat _errTrowsError' } );
  test.true( _.error.is( err ) );
  var errStr = String( err );
  test.identical( _.strCount( errStr, 'Error' ), 5 );
  test.identical( _.strCount( errStr, 'at program' ), 1 );
  test.identical( _.strCount( errStr, 'at _errTrowsError' ), 1 );

}

_errArgsHasError.timeOut = 10000;

//

function _errArgsHasRoutine( test )
{

  test.case = 'empty args';
  var unroll = () => _.unroll.make( [ 'error with unroll', 'routine unroll' ] );
  var err = _._err( { args : [ unroll,  new Error( 'Sample' ), new Error( 'next' ) ] } );
  test.true( _.error.is( err ) );
  var errStr = String( err );
  test.identical( _.strCount( errStr, 'error with unroll' ), 2 );
  test.identical( _.strCount( errStr, 'routine unroll' ), 2 );
  test.identical( _.strCount( errStr, 'Sample' ), 2 );
  test.identical( _.strCount( errStr, 'next' ), 2 );
  test.identical( _.strCount( errStr, 'Error' ), 3 );
  test.identical( _.strCount( errStr, 'Object._errArgsHasRoutine' ), 2 );

  test.case = 'empty args, throwCallsStack - undefined, catchCallsStack - undefined, level - 2';
  var unroll = () => _.unroll.make( [ 'error with unroll', 'routine unroll' ] );
  var err = _._err( { args : [ unroll,  new Error( 'Sample' ), new Error( 'next' ) ], level : 2 } );
  test.true( _.error.is( err ) );
  var errStr = String( err );
  test.identical( _.strCount( errStr, 'error with unroll' ), 2 );
  test.identical( _.strCount( errStr, 'routine unroll' ), 2 );
  test.identical( _.strCount( errStr, 'Sample' ), 2 );
  test.identical( _.strCount( errStr, 'next' ), 2 );
  test.identical( _.strCount( errStr, 'Error' ), 3 );
  test.identical( _.strCount( errStr, 'Object._errArgsHasRoutine' ), 1 );

  test.case = 'empty args, throwCallsStack - string';
  var unroll = () => _.unroll.make( [ 'error with unroll', 'routine unroll' ] );
  var err = _._err( { args : [ unroll,  new Error( 'Sample' ), new Error( 'next' ) ], throwCallsStack : 'at program\nat _errTrowsError' } );
  test.true( _.error.is( err ) );
  var errStr = String( err );
  test.identical( _.strCount( errStr, 'error with unroll' ), 1 );
  test.identical( _.strCount( errStr, 'routine unroll' ), 1 );
  test.identical( _.strCount( errStr, 'Sample' ), 1 );
  test.identical( _.strCount( errStr, 'next' ), 1 );
  test.identical( _.strCount( errStr, 'Error' ), 2 );
  test.identical( _.strCount( errStr, 'at program' ), 1 );
  test.identical( _.strCount( errStr, 'at _errTrowsError' ), 1 );

  test.case = 'empty args, throwCallsStack - undefined, catchCallsStack - string';
  var unroll = () => _.unroll.make( [ 'error with unroll', 'routine unroll' ] );
  var err = _._err( { args : [ unroll,  new Error( 'Sample' ), new Error( 'next' ) ], catchCallsStack : 'at program\nat _errTrowsError' } );
  test.true( _.error.is( err ) );
  var errStr = String( err );
  test.identical( _.strCount( errStr, 'error with unroll' ), 1 );
  test.identical( _.strCount( errStr, 'routine unroll' ), 1 );
  test.identical( _.strCount( errStr, 'Sample' ), 1 );
  test.identical( _.strCount( errStr, 'next' ), 1 );
  test.identical( _.strCount( errStr, 'Error' ), 1 );
  test.identical( _.strCount( errStr, 'at program' ), 0 );
  test.identical( _.strCount( errStr, 'at _errTrowsError' ), 0 );

  test.case = 'empty args, throwCallsStack - empty string';
  var unroll = () => _.unroll.make( [ 'error with unroll', 'routine unroll' ] );
  var o = { args : [ unroll,  new Error( 'Sample' ), new Error( 'next' ) ], catchCallsStack : '' };
  var err = _._err( o );
  test.true( _.error.is( err ) );
  var errStr = String( err );
  test.identical( _.strCount( errStr, 'error with unroll' ), 2 );
  test.identical( _.strCount( errStr, 'routine unroll' ), 2 );
  test.identical( _.strCount( errStr, 'Sample' ), 2 );
  test.identical( _.strCount( errStr, 'next' ), 2 );
  test.identical( _.strCount( errStr, 'Error' ), 3 );
  test.identical( _.strCount( err.stack, 'Object._errArgsHasRoutine' ), 2 );

}

//

function _errLocation( test )
{

  test.case = 'args - Error, catchCallsStack and catchLocation';
  var err = _._err
  ({
    args : [ new Error( 'Sample' ) ],
    catchCallsStack : 'at program1\nat _errTrowsError',
    catchLocation : { 'filePath' : 'at @605' }
  });
  test.true( _.error.is( err ) );
  test.identical( _.strCount( err.throwsStack, 'at @605' ), 1 );
  var errStr = String( err );
  test.identical( _.strCount( errStr, 'Sample' ), 2 );
  test.identical( _.strCount( errStr, 'Error' ), 3 );
  test.identical( _.strCount( errStr, 'at Err.test.s *' ), 0 );

}

//

/* qqq : for Dmytro : bad test. option sections influences not only err.sections. improve */
function _errOptionSections( test )
{

  test.case = 'args - Error, without sections option';
  var srcErr = new Error( 'Sample' );
  var err = _._err
  ({
    args : [ srcErr ],
  });
  test.true( _.error.is( err ) );
  test.identical( err.sections, srcErr.sections );
  test.identical( _.props.keys( err.sections ), [ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ] );

  test.case = 'args - Error with sections option, has not head and body';
  var srcErr = new Error( 'Sample' );
  srcErr.section = { 'location' : 'head and body' };
  var err = _._err
  ({
    args : [ srcErr ],
  });
  test.true( _.error.is( err ) );
  test.identical( err.sections, srcErr.sections );
  test.identical( _.props.keys( err.sections ), [ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'exposed' ] );

  test.case = 'args - Error with sections option';
  var srcErr = new Error( 'Sample' );
  srcErr.section = { 'location' : { head : 'location', body : 'at @123' } };
  var err = _._err
  ({
    args : [ srcErr ],
  });
  test.true( _.error.is( err ) );
  test.identical( err.sections, srcErr.sections );
  test.identical( _.props.keys( err.sections ), [ 'location', 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'exposed' ] );

  test.case = 'args - Error, sections option';
  var srcErr = new Error( 'Sample' );
  var err = _._err
  ({
    args : [ srcErr ],
    sections : { 'location' : { head : 'location', body : 'at @123' } }
  });
  test.true( _.error.is( err ) );
  test.identical( err.sections, srcErr.sections );
  test.identical( _.props.keys( err.sections ), [ 'location', 'message', 'combinedStack', 'throwsStack', 'sourceCode' ] );

}

//

function _errSourceCodeForm( test )
{
  test.case = 'args - Error, without option sourceCode';
  var srcErr = new Error( 'Sample' );
  var err = _._err
  ({
    args : [ srcErr ],
  });
  test.true( _.error.is( err ) );
  test.identical( _.strCount( err.sourceCode.code, 'args - Error, without option sourceCode' ), 1 );
  test.identical( _.strCount( err.sourceCode.code, 'var err = _._err' ), 1 );
  test.identical( _.strCount( err.sourceCode.code, 'args : [ srcErr ]' ), 0 );
  test.identical( _.strCount( err.sourceCode.path, 'Err.test.s' ), 1 );

  test.case = 'args - Error, with option sourceCode';
  var srcErr = new Error( 'Sample' );
  var err = _._err
  ({
    args : [ srcErr ],
    usingSourceCode : 0,
  });
  test.true( _.error.is( err ) );
  test.identical( err.sourceCode, null );

  test.case = 'args - Error, with sourceCode';
  var srcErr = new Error( 'Sample' );
  srcErr.sourceCode = 'test.case = "experiment"';
  var err = _._err
  ({
    args : [ srcErr ],
  });
  test.true( _.error.is( err ) );
  test.identical( _.strCount( err.sourceCode, 'test.case = "experiment"' ), 1 );
  test.identical( __.strLinesCount( err.sourceCode ), 1 );
}

//

function _originalMessage( test )
{

  test.case = 'args - different, routine returns routine';
  var abc = () =>
  {
    return () => '#1';
  }
  var err = _._err
  ({
    args : [ new Error( 'Sample' ), 'str', undefined, '', null, false, abc ],
  });
  console.log( err.originalMessage );
  test.true( _.error.is( err ) );
  test.identical( __.strLinesCount( err.originalMessage ), 3 );
  test.identical( _.strCount( err.originalMessage, 'Sample str' ), 1 );
  test.identical( _.strCount( err.originalMessage, 'undefined' ), 1 );
  var got = _.strCount( err.originalMessage, '{- routine.anonymous -}' );
  if( got === 0 )
  got = _.strCount( err.originalMessage, '[ routine without name ]' )
  test.identical( got, 1 );
  console.log( err.originalMessage );

}

//

function errWithExpoesedBasic( test )
{

  /* */

  test.case = 'non-formed error without exposed properties, empty option exposed';
  var err = new Error( 'Error1' );

  var err2 = _._err({ args : [ err ], exposed : {} });
  test.true( _.error.is( err ) );
  test.true( err === err2 );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );

  var exp = `= Exposed`;
  test.identical( _.strCount( err.message, exp ), 0 );

  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );

  console.log( err.message );

  /* */

  test.case = 'non-formed error without exposed properties';
  var err = new Error( 'Error1' );

  var err2 = _.err( err );
  test.true( _.error.is( err ) );
  test.true( err === err2 );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );

  var exp = `= Exposed`;
  test.identical( _.strCount( err.message, exp ), 0 );
  /* zzz : use test.contains here */

  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );

  console.log( err.message );

  /* */

  test.case = 'non-formed error with exposed properties';
  var err = new Error( 'Error1' );
  err.code = 'code1';
  err.requireStack = [ 'some', 'stack' ];

  var err2 = _.err( err );
  test.true( _.error.is( err ) );
  test.true( err === err2 );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );

  var exp = `= Exposed`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = /code : (\'){0,1}code1(\'){0,1}/;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = 'requireStack : ';
  test.identical( _.strCount( err.message, exp ), 1 );

  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'exposed' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );

  /* */

  test.case = 'non-formed error with exposed properties, empty option exposed';
  var err = new Error( 'Error1' );
  err.code = 'code1';
  err.requireStack = [ 'some', 'stack' ];

  var err2 = _._err({ args : [ err ], exposed : {} });
  test.true( _.error.is( err ) );
  test.true( err === err2 );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );

  var exp = `= Exposed`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = /code : (\'){0,1}code1(\'){0,1}/;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = 'requireStack : ';
  test.identical( _.strCount( err.message, exp ), 1 );

  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'exposed' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );

  /* */

  test.case = 'non-formed error with exposed properties, option exposed';
  var err = new Error( 'Error1' );
  err.code = 'code1';
  err.requireStack = [ 'some', 'stack' ];

  var err2 = _._err({ args : [ err ], exposed : { code : 'code2', prop2 : 13 } });
  test.true( _.error.is( err ) );
  test.true( err === err2 );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );

  var exp = `= Exposed`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = /code : (\'){0,1}code2(\'){0,1}/
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = 'prop2 : 13';
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = 'requireStack : ';
  test.identical( _.strCount( err.message, exp ), 1 );

  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'exposed' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );

  /* */

  test.case = 'extra section';
  var err = new Error( 'Error1' );
  err.f1 = 'val1';
  err.f2 = 'val2';

  var err2 = _._err({ args : [ err ], exposed : {} });
  test.true( _.error.is( err ) );
  test.true( err === err2 );

  var err2 = _.error.sectionAdd({ error : err, name : 'extraSection', body : 'this\nis extra\nsection' });
  test.true( err === err2 )

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );

  var exp = `= Exposed`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = /f1 : (\'){0,1}val1(\'){0,1}/
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = /f2 : (\'){0,1}val2(\'){0,1}/
  test.identical( _.strCount( err.message, exp ), 1 );

  var exp =
`= ExtraSection
    this
    is extra
    section`;
  test.identical( _.strCount( err.message, exp ), 1 );

  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'exposed', 'extraSection' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );

  console.log( err.message );

  /* */

}

//

function sectionAdd( test )
{

  /* */

  test.case = 'formed error, explicit head';
  var err = _.err( 'Error1' );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = `= Extra Section 1`;
  test.identical( _.strCount( err.message, exp ), 0 );

  var err2 = _.error.sectionAdd({ error : err, name : 'extraSection', head : 'Extra Section 1', body : 'this\nis extra\nsection' });
  test.true( err === err2 )

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'extraSection' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = `= Extra Section 1`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp =
` = Extra Section 1
    this
    is extra
    section`;
  test.identical( _.strCount( err.message, exp ), 1 );

  test.true( !_.error.isAttended( err ) );
  test.true( !_.error.isLogged( err ) );

  var exp =
` = Extra Section 1
    this
    is extra
    section`;
  test.identical( _.strCount( err.stack, exp ), 1 );

  var exp =
` = Extra Section 1
    this
    is extra
    section`;
  test.identical( _.strCount( err.toString(), exp ), 1 );

  console.log( err );

  /* */

  test.case = 'non-formed error';
  var err = new Error( 'Error1' );

  var err2 = _.error.sectionAdd({ error : err, name : 'extraSection', head : 'Extra Section 1', body : 'this\nis extra\nsection' });
  test.true( err === err2 )

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'extraSection' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = `= Extra Section 1`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp =
` = Extra Section 1
    this
    is extra
    section`;
  test.identical( _.strCount( err.message, exp ), 1 );

  test.true( !_.error.isAttended( err ) );
  test.true( !_.error.isLogged( err ) );

  var exp =
` = Extra Section 1
    this
    is extra
    section`;
  test.identical( _.strCount( err.stack, exp ), 1 );

  var exp =
` = Extra Section 1
    this
    is extra
    section`;
  test.identical( _.strCount( err.toString(), exp ), 1 );

  console.log( err );

  /* */

  test.case = 'formed error, single options map';
  var err = _.err( 'Error1' );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = ` = ExtraSection`;
  test.identical( _.strCount( err.message, exp ), 0 );

  var err2 = _.error.sectionAdd({ error : err, name : 'extraSection', body : 'this\nis extra\nsection' });
  test.true( err === err2 )

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'extraSection' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = ` = ExtraSection`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp =
` = ExtraSection
    this
    is extra
    section`;
  test.identical( _.strCount( err.message, exp ), 1 );

  test.true( !_.error.isAttended( err ) );
  test.true( !_.error.isLogged( err ) );
  console.log( err );

  /* */

  test.case = 'formed error, 2 arguments';
  var err = _.err( 'Error1' );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = ` = ExtraSection`;
  test.identical( _.strCount( err.message, exp ), 0 );

  var err2 = _.error.sectionAdd( err, { name : 'extraSection', body : 'this\nis extra\nsection' });
  test.true( err === err2 )

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'extraSection' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = ` = ExtraSection`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp =
` = ExtraSection
    this
    is extra
    section`;
  test.identical( _.strCount( err.message, exp ), 1 );

  test.true( !_.error.isAttended( err ) );
  test.true( !_.error.isLogged( err ) );
  console.log( err );

  /* */

  test.case = 'formed error, several sections';
  var err = _.err( 'Error1' );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = ` = ExtraSection`;
  test.identical( _.strCount( err.message, exp ), 0 );

  var err2 = _.error.sectionAdd({ error : err, name : 'extraSection', body : 'this\nis extra\nsection' });
  test.true( err === err2 )
  var err2 = _.error.sectionAdd({ error : err, name : 'secondSection', body : 'second text' });
  test.true( err === err2 )

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'extraSection', 'secondSection' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = ` = ExtraSection`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp =
` = ExtraSection
    this
    is extra
    section`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp = ` = SecondSection`;
  test.identical( _.strCount( err.message, exp ), 1 );
  var exp =
` = SecondSection
    second text`;
  test.identical( _.strCount( err.message, exp ), 1 );

  test.true( !_.error.isAttended( err ) );
  test.true( !_.error.isLogged( err ) );

  var exp = ` = SecondSection`;
  test.identical( _.strCount( err.stack, exp ), 1 );
  var exp =
` = SecondSection
    second text`;
  test.identical( _.strCount( err.stack, exp ), 1 );
  var exp = ` = SecondSection`;
  test.identical( _.strCount( err.toString(), exp ), 1 );
  var exp =
` = SecondSection
    second text`;
  test.identical( _.strCount( err.toString(), exp ), 1 );

  console.log( err );

  /* */

}

//

function sectionAddToBrief( test )
{

  /* */

  test.case = 'basic';
  var err = _.errBrief( 'Error1' );

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  test.identical( err.message, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );
  var exp = `= Extra Section 1`;
  test.identical( _.strCount( err.message, exp ), 0 );

  var err2 = _.error.sectionAdd({ error : err, name : 'extraSection', body : 'this\nis extra\nsection' });
  test.true( err === err2 )

  var exp = 'Error1';
  test.identical( err.originalMessage, exp );
  test.identical( err.message, exp );
  var exp = new Set([ 'message', 'combinedStack', 'throwsStack', 'sourceCode', 'extraSection' ]);
  test.identical( new Set( _.props.keys( err.sections ) ), exp );

  test.true( !_.error.isAttended( err ) );
  test.true( !_.error.isLogged( err ) );
  test.true( _.error.isBrief( err ) );

  console.log( err );

  /* */

}

//

function errorFunctorExternal( test )
{
  let context = this;
  let visited = [];
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;

  /* */

  a.appStartNonThrowing({ execPath : filePath/*programPath*/ })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );

    test.identical( _.strCount( op.output, 'ncaught' ), 0 );
    test.identical( _.strCount( op.output, '= Message' ), 1 );
    test.identical( _.strCount( op.output, 'program:9' ), 1 );
    test.identical( _.strCount( op.output, 'program:' ), 2 );
    test.identical( _.strCount( op.output, 'arg1 arg2 abc' ), 1 );

    return null;
  });

  /* */

  return a.ready;

  function program()
  {
    const _ = require( toolsPath );

    let SomeError = _.error.error_functor( 'SomeError', _onSomeError );

    try
    {
      throw SomeError( 'abc' );
    }
    catch( err )
    {
      logger.log( err );
    }

    function _onSomeError( arg )
    {
      let args = [ 'arg1', 'arg2', arg ];
      return args;
    }

  }

}

//

function uncaughtError( test )
{
  let context = this;
  let visited = [];
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;

  /* */

  a.appStartNonThrowing({ execPath : filePath/*programPath*/ })
  .then( ( op ) =>
  {
    test.notIdentical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '- uncaught error -' ), 2 );
    test.identical( _.strCount( op.output, 'Uncaught error' ), 1 );
    test.identical( _.strCount( op.output, '------>' ), 1 );
    test.identical( _.strCount( op.output, '------<' ), 1 );
    test.identical( _.strCount( op.output, '= Process' ), 1 );
    test.identical( _.strCount( op.output, 'Current path :' ), 1 );
    test.identical( _.strCount( op.output, 'Exec path :' ), 1 );
    test.identical( _.strCount( op.output, '= Message of Error#' ), 1 );
    test.identical( _.strCount( op.output, '= Beautified calls stack' ), 1 );
    test.identical( _.strCount( op.output, '= Throws stack' ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function program()
  {
    require( toolsPath );
    throw Error( 'Uncaught error' )
  }

}

uncaughtError.timeOut = 10000;

//

function sourceCode( test )
{
  let context = this;
  let visited = [];
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;

  /* */

  a.appStartNonThrowing({ execPath : filePath/*programPath*/ })
  .then( ( op ) =>
  {
    test.notIdentical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '- uncaught error -' ), 2 );
    test.identical( _.strCount( op.output, '= Source code from' ), 1 );
    test.identical( _.strCount( op.output, `* 5 :     throw Error( 'Uncaught error' );` ), 1 );
    return null;
  });

  return a.ready;

  function program()
  {
    const _ = require( toolsPath );
    _.include( 'wProcess' );
    throw Error( 'Uncaught error' );
  }

}

sourceCode.timeOut = 10000;

//

function errorFunctorBasic( test )
{
  let context = this;

  var SomeError = _.error.error_functor( 'SomeError' );
  var error1 = SomeError( 'abc' );
  test.true( error1 instanceof SomeError );
  test.true( error1 instanceof Error );
  test.identical( error1.originalMessage, 'abc' );
  test.identical( error1.name, 'SomeError' );

}

//

function errorFunctorAttended( test )
{
  let context = this;
  let visited = [];
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;

  /* */

  a.appStartNonThrowing({ execPath : filePath/*programPath*/ })
  .then( ( op ) =>
  {
    test.notIdentical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '- uncaught error -' ), 2 );
    test.identical( _.strCount( op.output, 'Error1' ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function program()
  {
    const _ = require( toolsPath );
    let SomeError = _.error.error_functor( 'SomeError' );
    throw SomeError( 'Error1' );
  }

}

errorFunctorAttended.timeOut = 10000;
errorFunctorAttended.description =
`
Njs stingify inherited error
so error handling mechanism has workaround to avoid setting "attended" and "logged" into true in such case
`

//

function uncaughtErrorBasic( test )
{
  let context = this;
  let visited = [];
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;

  /* */

  a.appStartNonThrowing({ execPath : filePath/*programPath*/ })
  .then( ( op ) =>
  {
    test.notIdentical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, '- uncaught error -' ), 2 );
    test.identical( _.strCount( op.output, 'Error1' ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function program()
  {
    const _ = require( toolsPath );
    throw new Error( 'Error1' );
  }

}

//

function eventUncaughtErrorBasic( test )
{
  let context = this;
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;
  let ready = __.take( null );

  ready.then( () => run( 'sync' ) );
  ready.then( () => run( 'timer' ) );
  ready.then( () => run( 'promise' ) );
  ready.then( () => run( 'consequence' ) );

  return ready;

  /* */

  function run( throwing )
  {
    let ready = __.take( null );

    ready.then( () =>
    {

      test.case = `throwing:${throwing} rethrowing:0 attending:0`;
      a.forkNonThrowing({ execPath : filePath/*programPath*/, args : [ `throwing:${throwing}`, 'rethrowing:0', 'attending:0' ] })
      .then( ( op ) =>
      {
        test.notIdentical( op.exitCode, 0 );
        test.identical( _.strCount( op.output, 'ErrorSync' ), throwing === 'sync' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'ErrorTimer' ), throwing === 'timer' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'ErrorPromise' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'ErrorConsequence' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'Error2' ), 0 );
        test.identical( _.strCount( op.output, 'Error on handing event uncaughtError' ), 0 );
        test.identical( _.strCount( op.output, 'uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 3 : 0 );
        test.identical( _.strCount( op.output, 'uncaught promise error' ), throwing === 'promise' ? 3 : 0 );
        test.identical( _.strCount( op.output, 'uncaught asynchronous error' ), throwing === 'consequence' ? 3 : 0 );
        test.identical( _.strCount( op.output, 'errIs:true' ), 1 );
        test.identical( _.strCount( op.output, 'error.isAttended:false' ), 1 );
        test.identical( _.strCount( op.output, 'error.isWary:false' ), throwing === 'consequence' ? 0 : 1 );
        test.identical( _.strCount( op.output, 'error.isWary:true' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'error.isSuspended:false' ), 1 );
        test.identical( _.strCount( op.output, 'origination:uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught promise error' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught asynchronous error' ), throwing === 'consequence' ? 1 : 0 );
        return null;
      });

      return a.ready;
    });

    /* */

    ready.then( () =>
    {

      test.case = `throwing:${throwing} rethrowing:1 attending:0`;
      a.forkNonThrowing({ execPath : filePath/*programPath*/, args : [ `throwing:${throwing}`, 'rethrowing:1', 'attending:0' ] })
      .then( ( op ) =>
      {
        test.notIdentical( op.exitCode, 0 );
        test.identical( _.strCount( op.output, 'ErrorSync' ), throwing === 'sync' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'ErrorTimer' ), throwing === 'timer' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'ErrorPromise' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'ErrorConsequence' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'Error2' ), 1 );
        test.identical( _.strCount( op.output, 'Error on handing event uncaughtError' ), 1 );
        test.identical( _.strCount( op.output, 'uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 3 : 0 );
        test.identical( _.strCount( op.output, 'uncaught promise error' ), throwing === 'promise' ? 3 : 0 );
        test.identical( _.strCount( op.output, 'uncaught asynchronous error' ), throwing === 'consequence' ? 3 : 0 );
        test.identical( _.strCount( op.output, 'errIs:true' ), 1 );
        test.identical( _.strCount( op.output, 'error.isAttended:false' ), 1 );
        test.identical( _.strCount( op.output, 'error.isWary:false' ), throwing === 'consequence' ? 0 : 1 );
        test.identical( _.strCount( op.output, 'error.isWary:true' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'error.isSuspended:false' ), 1 );
        test.identical( _.strCount( op.output, 'origination:uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught promise error' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught asynchronous error' ), throwing === 'consequence' ? 1 : 0 );
        return null;
      });

      return a.ready;
    });

    /* */

    ready.then( () =>
    {

      test.case = `throwing:${throwing} rethrowing:0 attending:1`;
      a.forkNonThrowing({ execPath : filePath/*programPath*/, args : [ `throwing:${throwing}`, 'rethrowing:0', 'attending:1' ] })
      .then( ( op ) =>
      {
        test.identical( op.exitCode, 0 );
        test.identical( _.strCount( op.output, 'ErrorSync' ), 0 );
        test.identical( _.strCount( op.output, 'ErrorTimer' ), 0 );
        test.identical( _.strCount( op.output, 'ErrorPromise' ), 0 );
        test.identical( _.strCount( op.output, 'ErrorConsequence' ), 0 );
        test.identical( _.strCount( op.output, 'Error2' ), 0 );
        test.identical( _.strCount( op.output, 'Error on handing event uncaughtError' ), 0 );
        test.identical( _.strCount( op.output, 'uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 1 : 0 );
        test.identical( _.strCount( op.output, 'uncaught promise error' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'uncaught asynchronous error' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'errIs:true' ), 1 );
        test.identical( _.strCount( op.output, 'error.isAttended:false' ), 1 );
        test.identical( _.strCount( op.output, 'error.isWary:false' ), throwing === 'consequence' ? 0 : 1 );
        test.identical( _.strCount( op.output, 'error.isWary:true' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'error.isSuspended:false' ), 1 );
        test.identical( _.strCount( op.output, 'origination:uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught promise error' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught asynchronous error' ), throwing === 'consequence' ? 1 : 0 );
        return null;
      });

      return a.ready;
    });

    /* */

    ready.then( () =>
    {

      test.case = `throwing:${throwing} rethrowing:1 attending:1`;
      a.forkNonThrowing({ execPath : filePath/*programPath*/, args : [ `throwing:${throwing}`, 'rethrowing:1', 'attending:1' ] })
      .then( ( op ) =>
      {
        test.notIdentical( op.exitCode, 0 );
        test.identical( _.strCount( op.output, 'ErrorSync' ), 0 );
        test.identical( _.strCount( op.output, 'ErrorTimer' ), 0 );
        test.identical( _.strCount( op.output, 'ErrorPromise' ), 0 );
        test.identical( _.strCount( op.output, 'ErrorConsequence' ), 0 );
        test.identical( _.strCount( op.output, 'Error2' ), 1 );
        test.identical( _.strCount( op.output, 'Error on handing event uncaughtError' ), 1 );
        test.identical( _.strCount( op.output, 'uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 1 : 0 );
        test.identical( _.strCount( op.output, 'uncaught promise error' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'uncaught asynchronous error' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'errIs:true' ), 1 );
        test.identical( _.strCount( op.output, 'error.isAttended:false' ), 1 );
        test.identical( _.strCount( op.output, 'error.isWary:false' ), throwing === 'consequence' ? 0 : 1 );
        test.identical( _.strCount( op.output, 'error.isWary:true' ), throwing === 'consequence' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'error.isSuspended:false' ), 1 );
        test.identical( _.strCount( op.output, 'origination:uncaught error' ), ( throwing === 'sync' || throwing === 'timer' ) ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught promise error' ), throwing === 'promise' ? 1 : 0 );
        test.identical( _.strCount( op.output, 'origination:uncaught asynchronous error' ), throwing === 'consequence' ? 1 : 0 );
        return null;
      });

      return a.ready;
    });

    /* */

    return ready;
  }

  /* */

  function program()
  {
    const _ = require( toolsPath );
    let rethrowing = process.argv.includes( 'rethrowing:1' );
    let attending = process.argv.includes( 'attending:1' );
    let throwingSync = process.argv.includes( `throwing:sync` );
    let throwingTimer = process.argv.includes( `throwing:timer` );
    let throwingPromise = process.argv.includes( `throwing:promise` );
    let throwingConsequence = process.argv.includes( `throwing:consequence` );

    if( throwingConsequence )
    _.include( 'wConsequence' );

    _.process.on( 'uncaughtError', ( e ) =>
    {

      console.log( 'errIs:' + _.error.is( e.err ) );
      console.log( 'error.isAttended:' + _.error.isAttended( e.err ) );
      console.log( 'error.isWary:' + _.error.isWary( e.err ) );
      console.log( 'error.isSuspended:' + _.error.isSuspended( e.err ) );
      console.log( 'origination:' + e.origination );

      if( attending )
      _.error.attend( e.err );

      if( rethrowing )
      throw 'Error2';
    });

    if( throwingSync )
    throw 'ErrorSync';

    if( throwingTimer )
    setTimeout( () =>
    {
      throw 'ErrorTimer'
    }, 1 );

    if( throwingPromise )
    {
      var promise1 = new Promise( ( take, error ) =>
      {
        error( 'ErrorPromise' );
      });
    }

    if( throwingConsequence )
    {
      _.Consequence().error( 'ErrorConsequence' );
    }

  }

}

eventUncaughtErrorBasic.timeOut = 60000;

//

function eventUncaughtErrorOnce( test )
{
  let context = this;
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;
  let ready = __.take( null );

  ready.then( () => run( 'once' ) ); /* aaa2 : for Dmytro : switch on later */ /* Dmytro : switched on */
  ready.then( () => run( 'off' ) );

  return ready;

  /* */

  function run( how )
  {
    let ready = __.take( null );

    ready.then( () =>
    {

      test.case = `how:${how}`;
      a.forkNonThrowing({ execPath : filePath/*programPath*/, args : [ `how:${how}` ] })
      .then( ( op ) =>
      {
        test.notIdentical( op.exitCode, 0 );
        test.identical( _.strCount( op.output, 'ErrorTimer1' ), 0 );
        test.identical( _.strCount( op.output, 'ErrorTimer2' ), 1 );
        test.identical( _.strCount( op.output, 'Error on handing event uncaughtError' ), 0 );
        test.identical( _.strCount( op.output, 'Unknown' ), 0 );
        test.identical( _.strCount( op.output, 'uncaught error' ), 2 );
        test.identical( _.strCount( op.output, 'is not a function' ), 0 );

        return null;
      });

      return a.ready;
    });

    /* */

    return ready;
  }

  /* */

  function program()
  {
    const _ = require( toolsPath );

    let once = process.argv.includes( 'how:once' );
    let off = process.argv.includes( 'how:off' );

    if( once )
    _.process.once( 'uncaughtError', ( e ) => /* aaa : for Dmytro : implement routine _.process.once(). make sure it works */ /* Dmytro : done */
    {
      _.error.attend( e.err );
    });

    if( off )
    _.process.on( 'uncaughtError', handle );

    setTimeout( () =>
    {
      throw 'ErrorTimer1'
    }, 1 );

    setTimeout( () =>
    {
      throw 'ErrorTimer2'
    }, 10 );

    function handle( e )
    {
      _.error.attend( e.err );
      _.process.off( 'uncaughtError', handle );
    }
  }

}

//

function entryProcedureStack( test )
{
  let context = this;
  let a = test.assetFor( false );
  let filePath/*programPath*/ = a.program( program ).filePath/*programPath*/;
  let ready = __.take( null );
  ready.then( () => run() );
  return ready;

  /* */

  function run( how )
  {
    let ready = __.take( null );

    ready.then( () =>
    {

      test.case = `basic`;
      a.forkNonThrowing({ execPath : filePath/*programPath*/, args : [ `how:${how}` ] })
      .then( ( op ) =>
      {
        test.identical( op.exitCode, 0 );
        test.identical( _.strCount( op.output, 'ncaught' ), 0 );
        test.identical( _.strCount( op.output, 'rror' ), 0 );
        test.identical( _.strCount( op.output, 'program' ), 1 );
        if( typeof process !== 'undefined' && process.platform === 'win32' )
        test.identical( _.strCount( op.output, '\\program:' ), 1 );
        else
        test.identical( _.strCount( op.output, '/program:' ), 1 );
        test.identical( _.strCount( op.output, 'at ' ), 1 );
        return null;
      });

      return a.ready;
    });

    /* */

    return ready;
  }

  /* */

  function program()
  {
    const _ = require( toolsPath );
    console.log( _.setup._entryProcedureStack );
  }

}

entryProcedureStack.timeOut = 30000;

// --
// declare
// --

const Proto =
{

  name : 'Tools.Err.l0.l9',
  silencing : 1,

  onSuiteBegin,
  onSuiteEnd,

  context :
  {
    suiteTempPath : null,
    assetsOriginalPath : null,
    appJsPath : null,
  },

  tests :
  {

    errStackAndSourceTrivial,
    errFromStringedError, /* qqq : extend the test routine. low priority problem */
    _errWithArgsIncludedRoutine,
    _errArgsHasError,
    _errArgsHasRoutine,
    _errLocation,
    _errOptionSections,
    _errSourceCodeForm,
    _originalMessage,
    // _errMessageForm,
    // _errOptionConcealedBasic,
    errWithExpoesedBasic,

    sectionAdd,
    sectionAddToBrief,

    // errorFunctorAttended,
    errorFunctorExternal,

    uncaughtError,
    sourceCode,

    errorFunctorBasic,
    errorFunctorAttended,
    uncaughtErrorBasic,
    eventUncaughtErrorBasic,
    eventUncaughtErrorOnce,
    entryProcedureStack,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
