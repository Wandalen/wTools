( function _l0_l9_Module_test_s_()
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
  let self = this;

  self.suiteTempPath = path.tempOpen( path.join( __dirname, '../..'  ), 'module' );
  // self.assetsOriginalPath = path.join( __dirname, '_asset' );

}

//

function onSuiteEnd()
{
  let self = this;
  _.assert( _.strHas( self.suiteTempPath, '/module-' ) )
  path.tempClose( self.suiteTempPath );
}

// --
// test routines implementation
// --

function modulePredeclareBasic( test )
{
  let context = this;
  let a = test.assetFor( false );
  let _ToolsPath_ = a.path.nativize( _.module.toolsPathGet() );
  let programRoutine1Path = a.program( programRoutine1 ).filePath;
  let programRoutine2Path = a.program({ entry : programRoutine2, locals : { _ToolsPath_, programRoutine1Path } }).filePath;

  /* */

  a.appStartNonThrowing({ execPath : programRoutine2Path })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    test.identical( _.strCount( op.output, 'programRoutine2.begin' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine1.begin' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine1.end' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine2.end' ), 1 );
    test.identical( _.strCount( op.output, /programRoutine2.begin(.|\n|\r)*programRoutine1.begin(.|\n|\r)*programRoutine1.end(.|\n|\r)*programRoutine2.end(.|\n|\r)*/mg ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function programRoutine1()
  {
    console.log( 'programRoutine1.begin' );
    const _ = require( toolsPath );
    _global_.programRoutine2 = true;
    console.log( 'programRoutine1.end' );
  }

  function programRoutine2()
  {
    console.log( 'programRoutine2.begin' );
    const _ = require( _ToolsPath_ );
    _global_.programRoutine2 = true;
    _.module.predeclare
    ({
      name : 'programRoutine1',
      entryPath : programRoutine1Path,
      basePath : '.',
    });
    _.include( 'programRoutine1' );
    _.include( 'programRoutine1' );
    console.log( 'programRoutine2.end' );
  }

}

//

function modulePredeclareBasic2( test )
{
  let context = this;
  let a = test.assetFor( false );
  let _ToolsPath_ = a.path.nativize( _.module.toolsPathGet() );
  let programRoutine1Path = a.program( programRoutine1 ).filePath;
  let programRoutine2Path = a.program({ entry : programRoutine2, locals : { _ToolsPath_, programRoutine1Path } }).filePath;

  /* */

  a.appStartNonThrowing({ execPath : programRoutine2Path })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    test.identical( _.strCount( op.output, 'programRoutine2.begin' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine1.begin' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine1.end' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine2.end' ), 1 );
    test.identical( _.strCount( op.output, /programRoutine2.begin(.|\n|\r)*programRoutine1.begin(.|\n|\r)*programRoutine1.end(.|\n|\r)*programRoutine2.end(.|\n|\r)*/mg ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function programRoutine1()
  {
    console.log( 'programRoutine1.begin' );
    const _ = require( toolsPath );
    _global_.programRoutine2 = true;
    console.log( 'programRoutine1.end' );
  }

  function programRoutine2()
  {
    console.log( 'programRoutine2.begin' );
    const _ = require( _ToolsPath_ );
    _global_.programRoutine2 = true;
    _.module.predeclare
    ({
      name : 'programRoutine1',
      entryPath : programRoutine1Path,
      basePath : '.',
    });
    _.include( 'programRoutine1' );
    _.include( 'programRoutine1' );
    console.log( 'programRoutine2.end' );
  }

}

//

function moduleExportsUndefined( test )
{
  let context = this;
  let a = test.assetFor( false );
  let programRoutine1Path = a.program( programRoutine1 ).filePath;
  let modulePath = a.path.join( programRoutine1Path, '../module.js' )

  a.fileProvider.fileWrite({ filePath : modulePath, data : `module.exports = undefined;` })

  /* */

  a.appStartNonThrowing({ execPath : programRoutine1Path })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    test.identical( _.strCount( op.output, 'programRoutine1.begin' ), 1 );
    test.identical( _.strCount( op.output, 'importedModule: undefined' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine1.end' ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function programRoutine1()
  {
    console.log( 'programRoutine1.begin' );
    const _ = require( toolsPath );
    var importedModule = require( './module.js' );
    console.log( 'importedModule:', importedModule)
    console.log( 'programRoutine1.end' );
  }
}

moduleExportsUndefined.description  =
`
  Included module returns undefined
`

//

/* qqq : rewrite test with several programs in differen dirs */
/* qqq : write test with several programs in differen dirs and declaring of modules */
function resolveBasic( test )
{

  let context = this;
  var exp = _.path.normalize( __dirname + '../../../../../node_modules/Tools' );
  var got = _.module.resolve( 'wTools' );
  test.identical( got, exp );
  /* xxx : extend by other resolve calls */

}

resolveBasic.description  =
`
  Routine _.module.resolve return path to include path of module::wTools.
`

//

/* xxx : implement and cover _.module.fileStack() */
/* xxx : add testing of section "module files stack" */
function modulingNativeIncludeErrors( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `throwing, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainThrowing ).filePath;
      a.program( throwing1 )

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      test.nil( op.exitCode, 0 );

      var exp =
`    error1
    Module file "${__.path.nativize( a.abs( './mainThrowing' ) )}" failed to include "./throwing1"`
      test.true( _.strHas( op.output, exp ) );

      var exp =
`main
throwing1`
      test.true( _.strHas( op.output, exp ) );


      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `throwing, catching ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainThrowingCatching ).filePath;
      a.program( throwing1 )

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      test.nil( op.exitCode, 0 );

      var exp =
`    error1
    Module file "${__.path.nativize( a.abs( './mainThrowingCatching' ) )}" failed to include "./throwing1"`
      test.true( _.strHas( op.output, exp ) );

      var exp =
`main.begin
throwing1
fileNativeWith( throwing1 ) : undefined
fileWith( throwing1 ) : undefined
throwing1`
      test.true( _.strHas( op.output, exp ) );

      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `syntax error ${__.entity.exportStringSolo( env )}`;

      let syntax1 =
      `
      function syntax1()
      {
        console.log( 'syntax1' );
        console.log( 'a' 'b' 'c' );
      }
      `

      var filePath = a.program( mainSyntax ).filePath;
      a.program({ routineCode : syntax1, name : 'syntax1' })

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      test.nil( op.exitCode, 0 );

      var exp =
`main
--------------- uncaught error --------------->

 = Message of SyntaxError#1
    missing ) after argument list
    Module file "${__.path.nativize( a.abs( './mainSyntax' ) )}" failed to include "./syntax1"`
      test.true( _.strHas( op.output, exp ) );

      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `syntax error catching ${__.entity.exportStringSolo( env )}`;

      let syntax1 =
      `
      function syntax1()
      {
        console.log( 'syntax1' );
        console.log( 'a' 'b' 'c' );
      }
      `

      var filePath = a.program( mainSyntaxCatching ).filePath;
      a.program({ routineCode : syntax1, name : 'syntax1' })

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      test.nil( op.exitCode, 0 );

      var exp =
`main.begin
fileNativeWith( syntax1 ) : undefined
fileWith( syntax1 ) : undefined
--------------- uncaught error --------------->

 = Message of SyntaxError#2
    missing ) after argument list
    Module file "${__.path.nativize( a.abs( './mainSyntaxCatching' ) )}" failed to include "./syntax1"`
      test.true( _.strHas( op.output, exp ) );

      return op;
    });

    /* */

  }

  /* - */

  function mainThrowing()
  {
    console.log( 'main' );
    const _ = require( toolsPath );
    require( './throwing1' );
  }

  /* - */

  function mainThrowingCatching()
  {
    console.log( 'main.begin' );
    const _ = require( toolsPath );
    try
    {
      require( './throwing1' );
    }
    catch( err )
    {
    }
    console.log( 'fileNativeWith( throwing1 ) :', _.module.fileNativeWith( __dirname + '/throwing1' ) );
    console.log( 'fileWith( throwing1 ) :', _.module.fileWith( __dirname + '/throwing1' ) );
    require( './throwing1' );
    console.log( 'main.end' );
  }

  /* - */

  function throwing1()
  {
    console.log( 'throwing1' );
    throw Error( 'error1' );
  }

  /* - */

  function mainSyntax()
  {
    console.log( 'main' );
    const _ = require( toolsPath );
    require( './syntax1' );
  }

  /* - */

  function mainSyntaxCatching()
  {
    console.log( 'main.begin' );
    const _ = require( toolsPath );
    try
    {
      require( './syntax1' );
    }
    catch( err )
    {
    }
    console.log( 'fileNativeWith( syntax1 ) :', _.module.fileNativeWith( __dirname + '/syntax1' ) );
    console.log( 'fileWith( syntax1 ) :', _.module.fileWith( __dirname + '/syntax1' ) );
    require( './syntax1' );
    console.log( 'main.end' );
  }

  /* - */

}

modulingNativeIncludeErrors.description =
`
- module file throwing error does not get neithe native file descriptor nor universal file descriptor
`

//

function modulingSourcePathValid( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  let start = __.process.starter
  ({
    outputCollecting : 1,
    outputPiping : 1,
    inputMirroring : 1,
    throwingExitCode : 0,
    mode : 'fork',
  });

  act({});

  /* */

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;

      let program0 = __.program.make
      ({
        entry : _program0,
        tempPath : a.abs( '.' ),
        moduleFile : _.module.fileWith( 0 ),
      });

      let programRoutine1 = __.program.make
      ({
        entry : _programRoutine1,
        tempPath : a.abs( '.' ),
        moduleFile : _.module.fileWith( 0 ),
      });
      console.log( _.strLinesNumber( programRoutine1.entry.routineCode ) );

      let programRoutine2 = __.program.make
      ({
        entry : _programRoutine2,
        tempPath : a.abs( '.' ),
        moduleFile : _.module.fileWith( 0 ),
      });

      let program3 = __.program.make
      ({
        entry : _program3,
        tempPath : a.abs( '.' ),
        moduleFile : _.module.fileWith( 0 ),
      });

      return start
      ({
        execPath : program0.filePath,
        currentPath : _.path.dir( program0.filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
  `
program0.begin
programRoutine1.begin
programRoutine1.begin : program0 : sourcePath : ${a.abs( '.' )}/_program0
programRoutine1.begin : program0 : requestedSourcePath : null
programRoutine1.begin : program0 : moduleFile : true
programRoutine1.begin : program0 : returned : 0
programRoutine1.begin : program0 : module : null
programRoutine1.begin : programRoutine1 : sourcePath : ${a.abs( '.' )}/_programRoutine1
programRoutine1.begin : programRoutine1 : requestedSourcePath : null
programRoutine1.begin : programRoutine1 : moduleFile : true
programRoutine1.begin : programRoutine1 : returned : [object Object]
programRoutine1.begin : programRoutine1 : module : null
programRoutine2.begin
program3
program3 : programRoutine2 : sourcePath : ${a.abs( '.' )}/_programRoutine2
program3 : programRoutine2 : requestedSourcePath : ./_programRoutine2
program3 : programRoutine2 : moduleFile : true
program3 : programRoutine2 : returned : [object Object]
program3 : programRoutine2 : module : null
program3 : program3 : sourcePath : ${a.abs( '.' )}/_program3
program3 : program3 : requestedSourcePath : ./_program3
program3 : program3 : moduleFile : true
program3 : program3 : returned : 3
program3 : program3 : module : null
programRoutine2.end
programRoutine1.after : program0 : sourcePath : ${a.abs( '.' )}/_program0
programRoutine1.after : program0 : requestedSourcePath : null
programRoutine1.after : program0 : moduleFile : true
programRoutine1.after : program0 : returned : 0
programRoutine1.after : program0 : module : null
programRoutine1.after : programRoutine1 : sourcePath : ${a.abs( '.' )}/_programRoutine1
programRoutine1.after : programRoutine1 : requestedSourcePath : null
programRoutine1.after : programRoutine1 : moduleFile : true
programRoutine1.after : programRoutine1 : returned : 1
programRoutine1.after : programRoutine1 : module : null
programRoutine1.after : programRoutine2 : sourcePath : ${a.abs( '.' )}/_programRoutine2
programRoutine1.after : programRoutine2 : requestedSourcePath : ./_programRoutine2
programRoutine1.after : programRoutine2 : moduleFile : true
programRoutine1.after : programRoutine2 : returned : 2
programRoutine1.after : programRoutine2 : module : null
programRoutine1.after : program3 : sourcePath : ${a.abs( '.' )}/_program3
programRoutine1.after : program3 : requestedSourcePath : ./_program3
programRoutine1.after : program3 : moduleFile : true
programRoutine1.after : program3 : returned : 3
programRoutine1.after : program3 : module : null
programRoutine1.end
program0.end
  `
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

  }

  /* - */

  function _program0()
  {
    console.log( 'program0.begin' );
    module.exports = 0;
    require( './_programRoutine1' );
    console.log( 'program0.end' );
  }

  /* - */

  function _programRoutine1()
  {
    const _ = require( toolsPath );
    console.log( 'programRoutine1.begin' );

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_program0' );
    console.log( `programRoutine1.begin : program0 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `programRoutine1.begin : program0 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `programRoutine1.begin : program0 : moduleFile : ${moduleFile.native === module.parent}` );
    console.log( `programRoutine1.begin : program0 : returned : ${moduleFile.returned}` );
    console.log( `programRoutine1.begin : program0 : module : ${moduleFile.module}` );

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_programRoutine1' );
    console.log( `programRoutine1.begin : programRoutine1 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `programRoutine1.begin : programRoutine1 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `programRoutine1.begin : programRoutine1 : moduleFile : ${moduleFile.native === module}` );
    console.log( `programRoutine1.begin : programRoutine1 : returned : ${moduleFile.returned}` );
    console.log( `programRoutine1.begin : programRoutine1 : module : ${moduleFile.module}` );

    module.exports = 1;
    require( './_programRoutine2' );

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_program0' );
    console.log( `programRoutine1.after : program0 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `programRoutine1.after : program0 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `programRoutine1.after : program0 : moduleFile : ${moduleFile.native === module.parent}` );
    console.log( `programRoutine1.after : program0 : returned : ${moduleFile.returned}` );
    console.log( `programRoutine1.after : program0 : module : ${moduleFile.module}` );

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_programRoutine1' );
    console.log( `programRoutine1.after : programRoutine1 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `programRoutine1.after : programRoutine1 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `programRoutine1.after : programRoutine1 : moduleFile : ${moduleFile.native === module}` );
    console.log( `programRoutine1.after : programRoutine1 : returned : ${moduleFile.returned}` );
    console.log( `programRoutine1.after : programRoutine1 : module : ${moduleFile.module}` );

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_programRoutine2' );
    console.log( `programRoutine1.after : programRoutine2 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `programRoutine1.after : programRoutine2 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `programRoutine1.after : programRoutine2 : moduleFile : ${moduleFile.native === module.children[ 1 ]}` );
    console.log( `programRoutine1.after : programRoutine2 : returned : ${moduleFile.returned}` );
    console.log( `programRoutine1.after : programRoutine2 : module : ${moduleFile.module}` );

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_program3' );
    console.log( `programRoutine1.after : program3 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `programRoutine1.after : program3 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `programRoutine1.after : program3 : moduleFile : ${moduleFile.native === module.children[ 1 ].children[ 0 ]}` );
    console.log( `programRoutine1.after : program3 : returned : ${moduleFile.returned}` );
    console.log( `programRoutine1.after : program3 : module : ${moduleFile.module}` );

    console.log( 'programRoutine1.end' );
  }

  /* - */

  function _programRoutine2()
  {
    const _ = _global_.wTools;
    console.log( 'programRoutine2.begin' );
    require( './_program3' );
    module.exports = 2;
    console.log( 'programRoutine2.end' );
  }

  /* - */

  function _program3()
  {
    const _ = _global_.wTools;
    console.log( 'program3' );
    module.exports = 3;

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_programRoutine2' );
    console.log( `program3 : programRoutine2 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `program3 : programRoutine2 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `program3 : programRoutine2 : moduleFile : ${moduleFile.native === module.parent}` );
    console.log( `program3 : programRoutine2 : returned : ${moduleFile.returned}` );
    console.log( `program3 : programRoutine2 : module : ${moduleFile.module}` );

    var moduleFile = _.module.fileWithResolvedPath( __dirname + '/_program3' );
    console.log( `program3 : program3 : sourcePath : ${moduleFile.sourcePath}` );
    console.log( `program3 : program3 : requestedSourcePath : ${moduleFile.requestedSourcePath}` );
    console.log( `program3 : program3 : moduleFile : ${moduleFile.native === module}` );
    console.log( `program3 : program3 : returned : ${moduleFile.returned}` );
    console.log( `program3 : program3 : module : ${moduleFile.module}` );

  }

  /* - */

}

//

function modulingGlobalNamespaces( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({ adeclaration : 'none', bdeclaration : 'before' });
  act({ adeclaration : 'none', bdeclaration : 'after' });

  act({ adeclaration : 'before', bdeclaration : 'before' });
  act({ adeclaration : 'before', bdeclaration : 'after' });

  act({ adeclaration : 'after', bdeclaration : 'before' });
  act({ adeclaration : 'after', bdeclaration : 'after' });

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : programRoutine1, locals : env }).filePath;
      a.program({ entry : programRoutine2, locals : env });
      a.program({ entry : programRoutine2b, locals : env });
      a.program({ entry : program3, locals : env });
      a.program({ entry : program4, locals : env });
      a.program({ entry : program5, locals : env });
      a.program({ entry : program6, locals : env });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {

      var exp =
`
programRoutine1
programRoutine2
program3
program4
program5.global : space2
program5._global_.wTools : 2
program6 : global : space2
program6 : wTools : space2

program6 : ./programRoutine1 : undefined
program6 : ./programRoutine2 : undefined
program6 : ./programRoutine2b : undefined
program6 : ./program3 : undefined
program6 : ./program4 : undefined
program6 : ./program5 : {- ModuleFile ./program5 -}
  global : space2
  downFiles
    {- ModuleFile ./program4 -}
  upFiles
    {- ModuleFile ${_.module.toolsPathGet()} -}
    {- ModuleFile ./program6 -}
program6 : ./program6 : {- ModuleFile ./program6 -}
  global : space2
  downFiles
    {- ModuleFile ./program5 -}

programRoutine2b

programRoutine1 : ./programRoutine1 : {- ModuleFile ./programRoutine1 -}
  global : real
  upFiles
    {- ModuleFile ${_.module.toolsPathGet()} -}
    {- ModuleFile ./programRoutine2 -}
    {- ModuleFile ./programRoutine2b -}
programRoutine1 : ./programRoutine2 : {- ModuleFile ./programRoutine2 -}
  global : real
  ${ env.adeclaration === 'none' ? '' : 'modules' }
  ${ env.adeclaration === 'none' ? '' : '{- Module Module1 -}' }
  downFiles
    {- ModuleFile ./programRoutine1 -}
  upFiles
    {- ModuleFile ./program3 -}
programRoutine1 : ./programRoutine2b : {- ModuleFile ./programRoutine2b -}
  global : real
  modules
    {- Module Module1 -}
  downFiles
    {- ModuleFile ./programRoutine1 -}
  upFiles
    {- ModuleFile ./program3 -}
programRoutine1 : ./program3 : {- ModuleFile ./program3 -}
  global : real
  modules
    {- Module Module1 -}
  downFiles
    {- ModuleFile ./programRoutine2 -}
    {- ModuleFile ./programRoutine2b -}
  upFiles
    {- ModuleFile ./program4 -}
programRoutine1 : ./program4 : {- ModuleFile ./program4 -}
  virtualEnvironment : space2
  global : real
  modules
    {- Module Module1 -}
  downFiles
    {- ModuleFile ./program3 -}
  upFiles
    {- ModuleFile ./program5 -}
programRoutine1 : ./program5 : undefined
programRoutine1 : ./program6 : undefined
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

  }

  /* - */

  function programRoutine1()
  {
    console.log( 'programRoutine1' );
    const _ = require( toolsPath );
    require( './programRoutine2' );
    require( './programRoutine2b' );

    console.log( '' );
    log( './programRoutine1' );
    log( './programRoutine2' );
    log( './programRoutine2b' );
    log( './program3' );
    log( './program4' );
    log( './program5' );
    log( './program6' );
    console.log( '' );

    function log( filePath )
    {
      let prefix = 'programRoutine1'
      let moduleFile = _.module.fileWith( filePath );
      if( !moduleFile )
      return console.log( `${prefix} : ${filePath} : ${moduleFile}` );
      let output = _.module.fileExportString( moduleFile, { it : { verbosity : 2 } } ).resultExportString();
      output = _.strReplace( output, _.path.normalize( __dirname ), '.' );
      console.log( `${prefix} : ${filePath} : ${output}` );
    }
  }

  /* - */

  function programRoutine2()
  {
    console.log( 'programRoutine2' );
    const _ = _global_.wTools;
    if( adeclaration === 'before' )
    _.module.predeclare( 'Module1', __dirname + '/programRoutine2' );
    require( './program3' );
    if( adeclaration === 'after' )
    _.module.predeclare( 'Module1', __dirname + '/programRoutine2' );
  }

  /* - */

  function programRoutine2b()
  {
    console.log( 'programRoutine2b' );
    const _ = _global_.wTools;
    if( bdeclaration === 'before' )
    _.module.predeclare( 'Module1', __dirname + '/programRoutine2b' );
    require( './program3' );
    if( bdeclaration === 'after' )
    _.module.predeclare( 'Module1', __dirname + '/programRoutine2b' );
  }

  /* - */

  function program3()
  {
    console.log( 'program3' );
    require( './program4' );
  }

  /* - */

  function program4()
  {
    console.log( 'program4' );
    const _ = _global_.wTools;
    _.global.new( 'space2' );
    _.global.open( 'space2' );
    _.module.fileSetEnvironment( module, 'space2' );
    require( './program5' );
    _.global.close( 'space2' );
  }

  /* - */

  function program5()
  {
    console.log( `program5.global : ${_global_.__GLOBAL_NAME__}` );
    console.log( `program5._global_.wTools : ${Object.keys( _global_.wTools ).length}` );
    const _ = require( toolsPath );
    require( './program6' );
  }

  /* - */

  function program6()
  {
    console.log( `program6 : global : ${_global_.__GLOBAL_NAME__}` );
    console.log( `program6 : wTools : ${_global_.wTools.__GLOBAL_NAME__}` );

    const _ = _global_.wTools;

    console.log( '' );
    log( './programRoutine1' );
    log( './programRoutine2' );
    log( './programRoutine2b' );
    log( './program3' );
    log( './program4' );
    log( './program5' );
    log( './program6' );
    console.log( '' );

    function log( filePath )
    {
      let prefix = 'program6'
      let moduleFile = _.module.fileWith( filePath );
      if( !moduleFile )
      return console.log( `${prefix} : ${filePath} : ${moduleFile}` );
      let output = _.module.fileExportString( moduleFile, { it : { verbosity : 2 } } ).resultExportString();
      output = _.strReplace( output, _.path.normalize( __dirname ), '.' );
      console.log( `${prefix} : ${filePath} : ${output}` );
    }
  }

  /* - */

}

modulingGlobalNamespaces.description =
`
- virtual environment in inherited from parent modules
- module is stay in its environment
- no error or unhandled case
`

//

function moduleRedeclare( test )
{
  let a = test.assetFor( false );
  let program1 = a.program( r1 );
  let program2 = a.program( r2 );

  /* */

  program1.start()
  .then( ( op ) =>
  {
    test.case = 'same entry path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{- Module Module1 -}
name : Module1
alias : Module1,module1
entryPath : ${ a.abs( 'r1' ) }
filePath : ${ a.abs( 'r1' ) }
lookPath : ${ a.abs( 'r1' ) },Module1,module1
files : ${ a.abs( 'r1' ) }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  program2.start()
  .then( ( op ) =>
  {
    test.case = 'different entry path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{- Module Module1 -}
name : Module1
alias : Module1,module1
entryPath : ${ a.abs( 'r2' ) },${ a.abs( 'r1' ) }
filePath : ${ a.abs( 'r2' ) },${ a.abs( 'r1' ) }
lookPath : ${ a.abs( 'r2' ) },Module1,module1,${ a.abs( 'r1' ) }
files : ${ a.abs( 'r2' ) }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function r1()
  {
    const _ = require( toolsPath );
    _.debugger = 1;
    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __filename,
    });
    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __filename,
      // entryPath : __dirname + '/r2',
    });
    let module1 = _.module.with( 'Module1' );
    console.log( module1 );
    console.log( `name : ${module1.name}` );
    console.log( `alias : ${module1.alias}` );
    console.log( `entryPath : ${module1.entryPath}` );
    console.log( `filePath : ${module1.filePath}` );
    console.log( `lookPath : ${module1.lookPath}` );
    console.log( `files : ${_.container.keys( module1.files )}` );
  }

  /* */

  function r2()
  {
    const _ = require( toolsPath );
    _.debugger = 1;
    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __filename,
    });
    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __dirname + '/r1',
    });
    let module1 = _.module.with( 'Module1' );
    console.log( module1 );
    console.log( `name : ${module1.name}` );
    console.log( `alias : ${module1.alias}` );
    console.log( `entryPath : ${module1.entryPath}` );
    console.log( `filePath : ${module1.filePath}` );
    console.log( `lookPath : ${module1.lookPath}` );
    console.log( `files : ${_.container.keys( module1.files )}` );
  }

}

moduleRedeclare.description =
`
- redeclaring of a module does not throw an error
`

//

function preload( test )
{
  let context = this;
  let a = test.assetFor( false );
  let _ToolsPath_ = a.path.nativize( _.module.toolsPathGet() );
  let programRoutine1Path = _.path.nativize( a.program( programRoutine1 ).filePath );

  /* */

  a.appStartNonThrowing({ execPath : `-r ${_ToolsPath_} ${programRoutine1Path}` })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    test.identical( _.strCount( op.output, 'program.begin' ), 1 );
    test.identical( _.strCount( op.output, 'program.end' ), 1 );
    test.identical( _.strCount( op.output, _.module.toolsPathGet() ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function programRoutine1()
  {
    console.log( 'program.begin' );
    let _ = _global_.wTools;
    console.log( _.module.toolsPathGet() );
    console.log( 'program.end' );
  }
}

//

function preloadIncludeModule( test )
{
  let context = this;
  let a = test.assetFor( false );
  let _ToolsPath_ = a.path.nativize( _.module.toolsPathGet() );
  let programRoutine1Path = _.path.nativize( a.program( programRoutine1 ).filePath );

  /* */

  a.appStartNonThrowing({ execPath : `-r ${_ToolsPath_} ${programRoutine1Path}` })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    test.identical( _.strCount( op.output, 'program.begin' ), 1 );
    test.identical( _.strCount( op.output, 'program.end' ), 1 );
    test.identical( _.strCount( op.output, 'program is alive : true' ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function programRoutine1()
  {
    console.log( 'program.begin' );
    let _ = _global_.wTools;
    _.include( 'wProcess' );
    console.log( 'program is alive :', _.process.isAlive( process.pid ) );
    console.log( 'program.end' );
  }
}

//

function predeclareBasic( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `predeclare before, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( main ).filePath;

      a.program
      ({
        entry : programRoutine1,
        dirPath : 'dir',
      });

      a.program( programRoutine2 )

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      /*
      extra module file is program
      */
      var exp =
`
main.before / lengthOf( predeclaredWithNameMap ) 2
main.before / lengthOf( predeclaredWithEntryPathMap ) 2
main.before / lengthOf( modulesMap ) ${_.entity.lengthOf( _.module.withName( 'wTools' ).alias )}
main.before / filesMap but tools.files
  ${a.abs( 'main' ) }
main.mid / predeclared.programRoutine1 : Module.constructible
main.mid / predeclared.programRoutine2 : Module.constructible
main.mid / lengthOf( predeclaredWithNameMap ) 4
main.mid / lengthOf( predeclaredWithEntryPathMap ) 4
main.mid / lengthOf( modulesMap ) 2
main.mid / lengthOf( filesMap ) 0
main.mid / isIncluded( Program1 ) false
main.mid / isIncluded( Program2 ) false
programRoutine1 / isIncluded( Program1 ) true
programRoutine1 / isIncluded( Program2 ) false
programRoutine2 / isIncluded( Program1 ) true
programRoutine2 / isIncluded( Program2 ) true
main.after / isIncluded( Program1 ) true
main.after / isIncluded( Program2 ) true
Program1
  ${a.abs( 'dir/programRoutine1' )}
Program2
  ${a.abs( 'programRoutine2' )}
orphans
  ${a.abs( 'main' )}
main.after / lengthOf( predeclaredWithNameMap ) 4
main.after / lengthOf( predeclaredWithEntryPathMap ) 4
main.after / lengthOf( modulesMap ) 4
main.after / lengthOf( filesMap ) 2
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

  }

  /* - */

  function main()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    let wasFilesMap = _.entity.lengthOf( _.module.filesMap );

    console.log( 'main.before / lengthOf( predeclaredWithNameMap )', _.entity.lengthOf( _.module.predeclaredWithNameMap ) );
    console.log( 'main.before / lengthOf( predeclaredWithEntryPathMap )', _.entity.lengthOf( _.module.predeclaredWithEntryPathMap ) );
    console.log( 'main.before / lengthOf( modulesMap )', _.entity.lengthOf( _.module.modulesMap ) );
    var diff = _.arraySet.diff_( null, [ ... _.module.filesMap.keys() ], [ ... _.module.withName( 'wTools' ).files.keys() ] )
    console.log( `main.before / filesMap but tools.files\n  ${diff.join( '\n  ' )}` );

    _.module.predeclare( 'Program1', __dirname + '/dir/programRoutine1' );
    _.module.predeclare( 'Program2', __dirname + '/programRoutine2/' );

    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/dir/programRoutine1/' ) );
    console.log( `main.mid / predeclared.programRoutine1 : ${_.entity.strType( module )}` );
    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/programRoutine2' ) );
    console.log( `main.mid / predeclared.programRoutine2 : ${_.entity.strType( module )}` );

    console.log( 'main.mid / lengthOf( predeclaredWithNameMap )', _.entity.lengthOf( _.module.predeclaredWithNameMap ) );
    console.log( 'main.mid / lengthOf( predeclaredWithEntryPathMap )', _.entity.lengthOf( _.module.predeclaredWithEntryPathMap ) );
    console.log( 'main.mid / lengthOf( modulesMap )', _.entity.lengthOf( _.module.modulesMap ) );
    console.log( 'main.mid / lengthOf( filesMap )', _.entity.lengthOf( _.module.filesMap ) - wasFilesMap );

    console.log( 'main.mid / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'main.mid / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );

    require( './dir/programRoutine1' );

    console.log( 'main.after / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'main.after / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );

    var files = [ ... _.module.withName( 'Program1' ).files.keys() ];
    console.log( `Program1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Program2' ).files.keys() ];
    console.log( `Program2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

    console.log( 'main.after / lengthOf( predeclaredWithNameMap )', _.entity.lengthOf( _.module.predeclaredWithNameMap ) );
    console.log( 'main.after / lengthOf( predeclaredWithEntryPathMap )', _.entity.lengthOf( _.module.predeclaredWithEntryPathMap ) );
    console.log( 'main.after / lengthOf( modulesMap )', _.entity.lengthOf( _.module.modulesMap ) );
    console.log( 'main.after / lengthOf( filesMap )', _.entity.lengthOf( _.module.filesMap ) - wasFilesMap );

  }

  /* - */

  function programRoutine1()
  {
    const _ = _global_.wTools;
    console.log( 'programRoutine1 / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'programRoutine1 / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );
    require( '../programRoutine2' );
  }

  /* - */

  function programRoutine2()
  {
    const _ = _global_.wTools;
    console.log( 'programRoutine2 / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'programRoutine2 / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );
  }

  /* - */

}

//

function predeclarePrime( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  special({ includingWith : 'require' });
  before({ includingWith : 'require' });
  before({ includingWith : 'include' });
  after({ includingWith : 'require', order : 'trp' }); /* tools, require, predeclare */
  after({ includingWith : 'require', order : 'rtp' }); /* require, tools, predeclare */
  after({ includingWith : 'require', order : 'prt' }); /* predeclare, require, tools */

  return ready;

  /* - */

  function localsFrom( env )
  {
    _.assert( _.routineIs( _.global.get ) );
    return _.props.extend( null, env, { get : _.global.get } );
  }

  /* - */

  function special( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `same entry path for both modules, ${__.entity.exportStringSolo( env )}`;
      var filePath = a.program( mainMultipleDeclare ).filePath;
      a.program( common );
      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp = `Module Common2 is trying to register entry path ${a.abs( 'common' )} which is registered for {- Module Common1 -}`;
      test.true( _.strHas( op.output, exp ) );
      test.nil( op.exitCode, 0 );
      return op;
    });

    /* */

  }

  /* - */

  function before( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `before, single level, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainSingleBefore, locals : localsFrom( env ) }).filePath;
      a.program({ entry : single1, locals : localsFrom( env ) });
      a.program({ entry : single2, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main.mid / predeclared.single1 : Module.constructible
main.mid / predeclared.single2 : Module.constructible
main.mid / isIncluded( Single1 ) false
main.mid / isIncluded( Single2 ) false
single1 / isIncluded( Single1 ) true
single1 / isIncluded( Single2 ) false
single2 / isIncluded( Single1 ) true
single2 / isIncluded( Single2 ) true
main.after / isIncluded( Single1 ) true
main.after / isIncluded( Single2 ) true
Single1
  ${a.abs( 'single1' )}
Single2
  ${a.abs( 'single2' )}
orphans
  ${a.abs( 'mainSingleBefore' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `before, deep, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainDeepBefore, locals : localsFrom( env ) }).filePath;
      a.program({ entry : deep1a, locals : localsFrom( env ) });
      a.program({ entry : deep1b, locals : localsFrom( env ) });
      a.program({ entry : deep1c, locals : localsFrom( env ) });
      a.program({ entry : deep1d, locals : localsFrom( env ) });
      a.program({ entry : deep1e, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main.mid / isIncluded( Deep1b ) false
main.mid / isIncluded( Deep1d ) false
deep1a / isIncluded( Deep1b ) false
deep1a / isIncluded( Deep1d ) false
deep1b / isIncluded( Deep1b ) true
deep1b / isIncluded( Deep1d ) false
deep1c / isIncluded( Deep1b ) true
deep1c / isIncluded( Deep1d ) false
deep1d / isIncluded( Deep1b ) true
deep1d / isIncluded( Deep1d ) true
deep1e / isIncluded( Deep1b ) true
deep1e / isIncluded( Deep1d ) true
main.after / isIncluded( Deep1b ) true
main.after / isIncluded( Deep1d ) true
Deep1b
  ${a.abs( 'deep1b' )}
  ${a.abs( 'deep1c' )}
Deep1d
  ${a.abs( 'deep1d' )}
  ${a.abs( 'deep1e' )}
orphans
  ${a.abs( 'mainDeepBefore' )}
  ${a.abs( 'deep1a' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `before, common sub file, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainBeforeCommonSubFile, locals : localsFrom( env ) }).filePath;
      a.program({ entry : common, locals : localsFrom( env ) });
      a.program({ entry : common1, locals : localsFrom( env ) });
      a.program({ entry : common2, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
common
common1 abc
Common1 abc
common2 abc
Common2 abc
Common1
  ${a.abs( 'common1' )}
  ${a.abs( 'common' )}
Common2
  ${a.abs( 'common2' )}
  ${a.abs( 'common' )}
orphans
  ${a.abs( 'mainBeforeCommonSubFile' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `before, common sub file deep, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainBeforeCommonSubFileDeep, locals : localsFrom( env ) }).filePath;
      a.program({ entry : common, locals : localsFrom( env ) });
      a.program({ entry : deep2a, locals : localsFrom( env ) });
      a.program({ entry : deep2b, locals : localsFrom( env ) });
      a.program({ entry : deep3a, locals : localsFrom( env ) });
      a.program({ entry : deep3b, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
common
deep2b.1 abc
deep2b.2 abc
deep2a abc
Deep2 abc
deep3b.1 abc
deep3b.2 abc
deep3a abc
Deep3 abc
deep2b.upFiles
  ${a.abs( 'common' )}
deep3b.upFiles
  ${a.abs( 'common' )}
common.downFiles
  ${a.abs( 'deep2b' )}
  ${a.abs( 'deep3b' )}
Deep2
  ${a.abs( 'deep2a' )}
  ${a.abs( 'deep2b' )}
  ${a.abs( 'common' )}
Deep3
  ${a.abs( 'deep3a' )}
  ${a.abs( 'deep3b' )}
  ${a.abs( 'common' )}
orphans
  ${a.abs( 'mainBeforeCommonSubFileDeep' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `before, branching1, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainBranchingBefore1, locals : localsFrom( env ) }).filePath;
      a.program({ entry : branching1a, locals : localsFrom( env ) });
      a.program({ entry : branching1b, locals : localsFrom( env ) });
      a.program({ entry : branching2a, locals : localsFrom( env ) });
      a.program({ entry : branching2b, locals : localsFrom( env ) });
      a.program({ entry : branchingCommon, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main.mid / isIncluded( Branching1 ) false
main.mid / isIncluded( Branching2 ) false
branching1a
branching1b
branchingCommon
branching2b
branching2a
main.after / isIncluded( Branching1 ) true
main.after / isIncluded( Branching2 ) true
Branching1
  ${a.abs( 'branching1a' )}
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
Branching2
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2a' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
orphans
  ${a.abs( 'mainBranchingBefore1' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `before, branching2, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainBranchingBefore2, locals : localsFrom( env ) }).filePath;
      a.program({ entry : branching1a, locals : localsFrom( env ) });
      a.program({ entry : branching1b, locals : localsFrom( env ) });
      a.program({ entry : branching2a, locals : localsFrom( env ) });
      a.program({ entry : branching2b, locals : localsFrom( env ) });
      a.program({ entry : branchingCommon, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main.mid / isIncluded( Branching1 ) false
main.mid / isIncluded( Branching2 ) false
branching2a
branching2b
branchingCommon
branching1b
branching1a
main.after / isIncluded( Branching1 ) true
main.after / isIncluded( Branching2 ) true
Branching1
  ${a.abs( 'branching1a' )}
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
Branching2
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2a' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
orphans
  ${a.abs( 'mainBranchingBefore2' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

  }

  /* - */

  function after( env )
  {

    ready.then( () =>
    {
      test.case = `after, single, top first, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainSingleAfterTopFirst, locals : localsFrom( env ) }).filePath;
      a.program({ entry : singleAfter1, locals : localsFrom( env ) });
      a.program({ entry : singleAfter2, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
singleAfter1
singleAfter2
main.after / predeclared.singleAfter1 : Module.constructible
main.after / predeclared.singleAfter2 : Module.constructible
main.after / isIncluded( Single1 ) true
main.after / isIncluded( Single2 ) true
Single1
  ${a.abs( 'singleAfter1' )}
Single2
  ${a.abs( 'singleAfter2' )}
orphans
  ${a.abs( 'mainSingleAfterTopFirst' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `after, single, bottom first, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainSingleAfterBottomFirst, locals : localsFrom( env ) }).filePath;
      a.program({ entry : singleAfter1, locals : localsFrom( env ) });
      a.program({ entry : singleAfter2, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
singleAfter1
singleAfter2
main.after / predeclared.singleAfter1 : Module.constructible
main.after / predeclared.singleAfter2 : Module.constructible
main.after / isIncluded( Single1 ) true
main.after / isIncluded( Single2 ) true
Single1
  ${a.abs( 'singleAfter1' )}
Single2
  ${a.abs( 'singleAfter2' )}
orphans
  ${a.abs( 'mainSingleAfterBottomFirst' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `after, deep, b, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainDeepAfterB, locals : localsFrom( env ) }).filePath;
      a.program({ entry : deep11a, locals : localsFrom( env ) });
      a.program({ entry : deep11b, locals : localsFrom( env ) });
      a.program({ entry : deep11c, locals : localsFrom( env ) });
      a.program({ entry : deep11d, locals : localsFrom( env ) });
      a.program({ entry : deep11e, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
deep11a
deep11b
deep11c
deep11d
deep11e
main.after / isIncluded( Deep1b ) true
main.after / isIncluded( Deep1d ) true
Deep1b
  ${a.abs( 'deep11b' )}
  ${a.abs( 'deep11c' )}
Deep1d
  ${a.abs( 'deep11d' )}
  ${a.abs( 'deep11e' )}
orphans
  ${a.abs( 'mainDeepAfterB' )}
  ${a.abs( 'deep11a' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `after, deep, d, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainDeepAfterD, locals : localsFrom( env ) }).filePath;
      a.program({ entry : deep11a, locals : localsFrom( env ) });
      a.program({ entry : deep11b, locals : localsFrom( env ) });
      a.program({ entry : deep11c, locals : localsFrom( env ) });
      a.program({ entry : deep11d, locals : localsFrom( env ) });
      a.program({ entry : deep11e, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
deep11a
deep11b
deep11c
deep11d
deep11e
main.after / isIncluded( Deep1b ) true
main.after / isIncluded( Deep1d ) true
Deep1b
  ${a.abs( 'deep11b' )}
  ${a.abs( 'deep11c' )}
Deep1d
  ${a.abs( 'deep11d' )}
  ${a.abs( 'deep11e' )}
orphans
  ${a.abs( 'mainDeepAfterD' )}
  ${a.abs( 'deep11a' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `after, common sub file, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainAfterCommonSubFile, locals : localsFrom( env ) }).filePath;
      a.program({ entry : common, locals : localsFrom( env ) });
      a.program({ entry : common1, locals : localsFrom( env ) });
      a.program({ entry : common2, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
common
common1 abc
Common1 abc
common2 abc
Common2 abc
common1.upFiles
  ${a.abs( 'common' )}
common2.upFiles
  ${a.abs( 'common' )}
common.downFiles
  ${a.abs( 'common1' )}
  ${a.abs( 'common2' )}
Common1
  ${a.abs( 'common1' )}
  ${a.abs( 'common' )}
Common2
  ${a.abs( 'common2' )}
  ${a.abs( 'common' )}
orphans
  ${a.abs( 'mainAfterCommonSubFile' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `after, common sub file deep, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainAfterCommonSubFileDeep, locals : localsFrom( env ) }).filePath;
      a.program({ entry : common, locals : localsFrom( env ) });
      a.program({ entry : deep2a, locals : localsFrom( env ) });
      a.program({ entry : deep2b, locals : localsFrom( env ) });
      a.program({ entry : deep3a, locals : localsFrom( env ) });
      a.program({ entry : deep3b, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
common
deep2b.1 abc
deep2b.2 abc
deep2a abc
Deep2 abc
deep3b.1 abc
deep3b.2 abc
deep3a abc
Deep3 abc
deep2b.upFiles
  ${a.abs( 'common' )}
deep3b.upFiles
  ${a.abs( 'common' )}
common.downFiles
  ${a.abs( 'deep2b' )}
  ${a.abs( 'deep3b' )}
Deep2
  ${a.abs( 'deep2a' )}
  ${a.abs( 'deep2b' )}
  ${a.abs( 'common' )}
Deep3
  ${a.abs( 'deep3a' )}
  ${a.abs( 'deep3b' )}
  ${a.abs( 'common' )}
orphans
  ${a.abs( 'mainAfterCommonSubFileDeep' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `after, branching1, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainBranchingAfter1, locals : localsFrom( env ) }).filePath;
      a.program({ entry : branching1a, locals : localsFrom( env ) });
      a.program({ entry : branching1b, locals : localsFrom( env ) });
      a.program({ entry : branching2a, locals : localsFrom( env ) });
      a.program({ entry : branching2b, locals : localsFrom( env ) });
      a.program({ entry : branchingCommon, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
branching1a
branching1b
branchingCommon
branching2b
branching2a
main.after / isIncluded( Branching1 ) true
main.after / isIncluded( Branching2 ) true
Branching1
  ${a.abs( 'branching1a' )}
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
Branching2
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2a' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
orphans
  ${a.abs( 'mainBranchingAfter1' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `after, branching2, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : mainBranchingAfter2, locals : localsFrom( env ) }).filePath;
      a.program({ entry : branching1a, locals : localsFrom( env ) });
      a.program({ entry : branching1b, locals : localsFrom( env ) });
      a.program({ entry : branching2a, locals : localsFrom( env ) });
      a.program({ entry : branching2b, locals : localsFrom( env ) });
      a.program({ entry : branchingCommon, locals : localsFrom( env ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
branching2a
branching2b
branchingCommon
branching1b
branching1a
main.after / isIncluded( Branching1 ) true
main.after / isIncluded( Branching2 ) true
Branching1
  ${a.abs( 'branching1a' )}
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
Branching2
  ${a.abs( 'branching1b' )}
  ${a.abs( 'branching2a' )}
  ${a.abs( 'branching2b' )}
  ${a.abs( 'branchingCommon' )}
orphans
  ${a.abs( 'mainBranchingAfter2' )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

  }

  /* - */

  function mainSingleBefore()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    _.module.predeclare( 'Single1', __dirname + '/single1' );
    _.module.predeclare( 'Single2', __dirname + '/single2/' );

    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/single1' ) );
    console.log( `main.mid / predeclared.single1 : ${_.entity.strType( module )}` );
    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/single2' ) );
    console.log( `main.mid / predeclared.single2 : ${_.entity.strType( module )}` );

    console.log( 'main.mid / isIncluded( Single1 )', _.module.isIncluded( 'Single1' ) );
    console.log( 'main.mid / isIncluded( Single2 )', _.module.isIncluded( 'Single2' ) );

    if( includingWith === 'require' )
    require( './single1' );
    else
    _.include( 'Single1' );

    console.log( 'main.after / isIncluded( Single1 )', _.module.isIncluded( 'Single1' ) );
    console.log( 'main.after / isIncluded( Single2 )', _.module.isIncluded( 'Single2' ) );

    var files = [ ... _.module.withName( 'Single1' ).files.keys() ];
    console.log( `Single1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Single2' ).files.keys() ];
    console.log( `Single2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function single1()
  {
    const _ = _global_.wTools;
    console.log( 'single1 / isIncluded( Single1 )', _.module.isIncluded( 'Single1' ) );
    console.log( 'single1 / isIncluded( Single2 )', _.module.isIncluded( 'Single2' ) );

    if( includingWith === 'require' )
    require( './single2' );
    else
    _.include( 'Single2' );

  }

  /* - */

  function single2()
  {
    const _ = _global_.wTools;
    console.log( 'single2 / isIncluded( Single1 )', _.module.isIncluded( 'Single1' ) );
    console.log( 'single2 / isIncluded( Single2 )', _.module.isIncluded( 'Single2' ) );
  }

  /* - */

  function mainSingleAfterTopFirst()
  {

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      require( './singleAfter1' );
      _.module.predeclare( 'Single1', __dirname + '/singleAfter1' );
      _.module.predeclare( 'Single2', __dirname + '/singleAfter2/' );
    }
    else if( order === 'rtp' )
    {
      require( './singleAfter1' );
      _ = require( toolsPath );
      _.module.predeclare( 'Single1', __dirname + '/singleAfter1' );
      _.module.predeclare( 'Single2', __dirname + '/singleAfter2/' );
    }
    else if( order === 'prt' )
    {

      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Single1' ] = { entryPath : __dirname + '/singleAfter1' };
      _.module._modulesToPredeclare[ 'Single2' ] = { entryPath : __dirname + '/singleAfter2/' };
      require( './singleAfter1' );
      _ = require( toolsPath );
    }

    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/singleAfter1' ) );
    console.log( `main.after / predeclared.singleAfter1 : ${_.entity.strType( module )}` );
    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/singleAfter2' ) );
    console.log( `main.after / predeclared.singleAfter2 : ${_.entity.strType( module )}` );

    console.log( 'main.after / isIncluded( Single1 )', _.module.isIncluded( 'Single1' ) );
    console.log( 'main.after / isIncluded( Single2 )', _.module.isIncluded( 'Single2' ) );

    var files = [ ... _.module.withName( 'Single1' ).files.keys() ];
    console.log( `Single1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Single2' ).files.keys() ];
    console.log( `Single2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function mainSingleAfterBottomFirst()
  {

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      require( './singleAfter1' );
      _.module.predeclare( 'Single2', __dirname + '/singleAfter2/' );
      _.module.predeclare( 'Single1', __dirname + '/singleAfter1' );
    }
    else if( order === 'rtp' )
    {
      require( './singleAfter1' );
      _ = require( toolsPath );
      _.module.predeclare( 'Single2', __dirname + '/singleAfter2/' );
      _.module.predeclare( 'Single1', __dirname + '/singleAfter1' );
    }
    else if( order === 'prt' )
    {
      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Single2' ] = { entryPath : __dirname + '/singleAfter2/' };
      _.module._modulesToPredeclare[ 'Single1' ] = { entryPath : __dirname + '/singleAfter1' };
      require( './singleAfter1' );
      _ = require( toolsPath );
    }

    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/singleAfter1' ) );
    console.log( `main.after / predeclared.singleAfter1 : ${_.entity.strType( module )}` );
    var module = _.module.predeclaredWithEntryPathMap.get( _.path.canonize( __dirname + '/singleAfter2' ) );
    console.log( `main.after / predeclared.singleAfter2 : ${_.entity.strType( module )}` );

    console.log( 'main.after / isIncluded( Single1 )', _.module.isIncluded( 'Single1' ) );
    console.log( 'main.after / isIncluded( Single2 )', _.module.isIncluded( 'Single2' ) );

    var files = [ ... _.module.withName( 'Single1' ).files.keys() ];
    console.log( `Single1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Single2' ).files.keys() ];
    console.log( `Single2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function singleAfter1()
  {
    console.log( 'singleAfter1' );
    require( './singleAfter2' );
  }

  /* - */

  function singleAfter2()
  {
    console.log( 'singleAfter2' );
  }

  /* - */

  function mainDeepBefore()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    _.module.predeclare( 'Deep1b', __dirname + '/deep1b' );
    _.module.predeclare( 'Deep1d', __dirname + '/deep1d' );

    console.log( 'main.mid / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'main.mid / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    require( './deep1a' );

    console.log( 'main.after / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'main.after / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    var files = [ ... _.module.withName( 'Deep1b' ).files.keys() ];
    console.log( `Deep1b\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Deep1d' ).files.keys() ];
    console.log( `Deep1d\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function deep1a()
  {
    const _ = _global_.wTools;
    console.log( 'deep1a / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'deep1a / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    if( includingWith === 'require' )
    require( './deep1b' );
    else
    _.include( 'Deep1b' );

  }

  /* - */

  function deep1b()
  {
    const _ = _global_.wTools;
    console.log( 'deep1b / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'deep1b / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    require( './deep1c' );

  }

  /* - */

  function deep1c()
  {
    const _ = _global_.wTools;
    console.log( 'deep1c / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'deep1c / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    if( includingWith === 'require' )
    require( './deep1d' );
    else
    _.include( 'Deep1d' );

  }

  /* - */

  function deep1d()
  {
    const _ = _global_.wTools;
    console.log( 'deep1d / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'deep1d / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    require( './deep1e' );

  }

  /* - */

  function deep1e()
  {
    const _ = _global_.wTools;
    console.log( 'deep1e / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'deep1e / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );
  }

  /* - */

  function mainDeepAfterB()
  {

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      require( './deep11a' );
      _.module.predeclare( 'Deep1b', __dirname + '/deep11b' );
      _.module.predeclare( 'Deep1d', __dirname + '/deep11d' );
    }
    else if( order === 'rtp' )
    {
      require( './deep11a' );
      _ = require( toolsPath );
      _.module.predeclare( 'Deep1b', __dirname + '/deep11b' );
      _.module.predeclare( 'Deep1d', __dirname + '/deep11d' );
    }
    else if( order === 'prt' )
    {
      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Deep1b' ] = { entryPath : __dirname + '/deep11b' };
      _.module._modulesToPredeclare[ 'Deep1d' ] = { entryPath : __dirname + '/deep11d' };
      require( './deep11a' );
      _ = require( toolsPath );
    }

    console.log( 'main.after / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'main.after / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    var files = [ ... _.module.withName( 'Deep1b' ).files.keys() ];
    console.log( `Deep1b\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Deep1d' ).files.keys() ];
    console.log( `Deep1d\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function mainDeepAfterD()
  {

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      require( './deep11a' );
      _.module.predeclare( 'Deep1d', __dirname + '/deep11d' );
      _.module.predeclare( 'Deep1b', __dirname + '/deep11b' );
    }
    else if( order === 'rtp' )
    {
      require( './deep11a' );
      _ = require( toolsPath );
      _.module.predeclare( 'Deep1d', __dirname + '/deep11d' );
      _.module.predeclare( 'Deep1b', __dirname + '/deep11b' );
    }
    else if( order === 'prt' )
    {

      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Deep1d' ] = { entryPath : __dirname + '/deep11d' };
      _.module._modulesToPredeclare[ 'Deep1b' ] = { entryPath : __dirname + '/deep11b' };
      require( './deep11a' );
      _ = require( toolsPath );
    }

    console.log( 'main.after / isIncluded( Deep1b )', _.module.isIncluded( 'Deep1b' ) );
    console.log( 'main.after / isIncluded( Deep1d )', _.module.isIncluded( 'Deep1d' ) );

    var files = [ ... _.module.withName( 'Deep1b' ).files.keys() ];
    console.log( `Deep1b\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Deep1d' ).files.keys() ];
    console.log( `Deep1d\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function deep11a()
  {
    console.log( 'deep11a' );
    require( './deep11b' );
  }

  /* - */

  function deep11b()
  {
    console.log( 'deep11b' );
    require( './deep11c' );
  }

  /* - */

  function deep11c()
  {
    console.log( 'deep11c' );
    require( './deep11d' );
  }

  /* - */

  function deep11d()
  {
    console.log( 'deep11d' );
    require( './deep11e' );
  }

  /* - */

  function deep11e()
  {
    console.log( 'deep11e' );
  }

  /* - */

  function mainMultipleDeclare()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    _.module.predeclare( 'Common1', __dirname + '/common' );
    _.module.predeclare( 'Common2', __dirname + '/common' );

  }

  /* - */

  function mainBeforeCommonSubFile()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    _.module.predeclare( 'Common1', __dirname + '/common1' );
    _.module.predeclare( 'Common2', __dirname + '/common2' );

    if( includingWith === 'require' )
    console.log( 'Common1', require( './common1' ) );
    else
    console.log( 'Common1', _.include( 'Common1' ) );

    if( includingWith === 'require' )
    console.log( 'Common2', require( './common2' ) );
    else
    console.log( 'Common2', _.include( 'Common2' ) );

    var files = [ ... _.module.withName( 'Common1' ).files.keys() ];
    console.log( `Common1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Common2' ).files.keys() ];
    console.log( `Common2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function mainAfterCommonSubFile()
  {
    let _;

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      console.log( 'Common1', require( './common1' ) );
      console.log( 'Common2', require( './common2' ) );
      _.module.predeclare( 'Common1', __dirname + '/common1' );
      _.module.predeclare( 'Common2', __dirname + '/common2' );
    }
    else if( order === 'rtp' )
    {
      console.log( 'Common1', require( './common1' ) );
      console.log( 'Common2', require( './common2' ) );
      _ = require( toolsPath );
      _.module.predeclare( 'Common1', __dirname + '/common1' );
      _.module.predeclare( 'Common2', __dirname + '/common2' );
    }
    else if( order === 'prt' )
    {
      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Common1' ] = { entryPath : __dirname + '/common1' };
      _.module._modulesToPredeclare[ 'Common2' ] = { entryPath : __dirname + '/common2' };
      console.log( 'Common1', require( './common1' ) );
      console.log( 'Common2', require( './common2' ) );
      _ = require( toolsPath );
    }

    var file = _.module.fileWith( './common1' );
    var files = [ ... file.upFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `common1.upFiles\n  ${files.join( '\n  ' )}` );
    var file = _.module.fileWith( './common2' );
    var files = [ ... file.upFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `common2.upFiles\n  ${files.join( '\n  ' )}` );
    var file = _.module.fileWith( './common' );
    var files = [ ... file.downFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `common.downFiles\n  ${files.sort().join( '\n  ' )}` );

    var files = [ ... _.module.withName( 'Common1' ).files.keys() ];
    console.log( `Common1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Common2' ).files.keys() ];
    console.log( `Common2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function common()
  {
    console.log( 'common' );
    module.exports = 'abc';
  }

  /* - */

  function common1()
  {
    let result = require( './common' );
    console.log( 'common1', result );
    module.exports = result;
  }

  /* - */

  function common2()
  {
    let result = require( './common' );
    console.log( 'common2', result );
    module.exports = result;
  }

  /* - */

  function mainBeforeCommonSubFileDeep()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    _.module.predeclare( 'Deep2', __dirname + '/deep2a' );
    _.module.predeclare( 'Deep3', __dirname + '/deep3a' );

    if( includingWith === 'require' )
    console.log( 'Deep2', require( './deep2a' ) );
    else
    console.log( 'Deep2', _.include( 'Deep2' ) );

    if( includingWith === 'require' )
    console.log( 'Deep3', require( './deep3a' ) );
    else
    console.log( 'Deep3', _.include( 'Deep3' ) );

    var file = _.module.fileWith( './deep2b' );
    var files = [ ... file.upFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `deep2b.upFiles\n  ${files.join( '\n  ' )}` );
    var file = _.module.fileWith( './deep3b' );
    var files = [ ... file.upFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `deep3b.upFiles\n  ${files.join( '\n  ' )}` );
    var file = _.module.fileWith( './common' );
    var files = [ ... file.downFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `common.downFiles\n  ${files.sort().join( '\n  ' )}` );

    var files = [ ... _.module.withName( 'Deep2' ).files.keys() ];
    console.log( `Deep2\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Deep3' ).files.keys() ];
    console.log( `Deep3\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function mainAfterCommonSubFileDeep()
  {

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      console.log( 'Deep2', require( './deep2a' ) );
      console.log( 'Deep3', require( './deep3a' ) );
      _.module.predeclare( 'Deep2', __dirname + '/deep2a' );
      _.module.predeclare( 'Deep3', __dirname + '/deep3a' );
    }
    else if( order === 'rtp' )
    {
      console.log( 'Deep2', require( './deep2a' ) );
      console.log( 'Deep3', require( './deep3a' ) );
      _ = require( toolsPath );
      _.module.predeclare( 'Deep2', __dirname + '/deep2a' );
      _.module.predeclare( 'Deep3', __dirname + '/deep3a' );
    }
    else if( order === 'prt' )
    {
      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Deep2' ] = { entryPath : __dirname + '/deep2a' };
      _.module._modulesToPredeclare[ 'Deep3' ] = { entryPath : __dirname + '/deep3a' };
      console.log( 'Deep2', require( './deep2a' ) );
      console.log( 'Deep3', require( './deep3a' ) );
      _ = require( toolsPath );
    }

    var file = _.module.fileWith( './deep2b' );
    var files = [ ... file.upFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `deep2b.upFiles\n  ${files.join( '\n  ' )}` );
    var file = _.module.fileWith( './deep3b' );
    var files = [ ... file.upFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `deep3b.upFiles\n  ${files.join( '\n  ' )}` );
    var file = _.module.fileWith( './common' );
    var files = [ ... file.downFiles.values() ].map( ( file ) => file.sourcePath );
    console.log( `common.downFiles\n  ${files.sort().join( '\n  ' )}` );

    var files = [ ... _.module.withName( 'Deep2' ).files.keys() ];
    console.log( `Deep2\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Deep3' ).files.keys() ];
    console.log( `Deep3\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function deep2a()
  {
    let result = require( './deep2b' );
    console.log( 'deep2a', result );
    module.exports = result;
  }

  /* - */

  function deep2b()
  {
    let result = require( './common' );
    console.log( 'deep2b.1', result );
    let result2 = require( './common' );
    console.log( 'deep2b.2', result2 );
    module.exports = result;
  }

  /* - */

  function deep3a()
  {
    let result = require( './deep3b' );
    console.log( 'deep3a', result );
    module.exports = result;
  }

  /* - */

  function deep3b()
  {
    let result = require( './common' );
    console.log( 'deep3b.1', result );
    let result2 = require( './common' );
    console.log( 'deep3b.2', result2 );
    module.exports = result;
  }

  /* - */

  function mainBranchingBefore1()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    _.module.predeclare( 'Branching1', __dirname + '/branching1a' );
    _.module.predeclare( 'Branching2', __dirname + '/branching2a' );

    console.log( 'main.mid / isIncluded( Branching1 )', _.module.isIncluded( 'Branching1' ) );
    console.log( 'main.mid / isIncluded( Branching2 )', _.module.isIncluded( 'Branching2' ) );

    if( includingWith === 'require' )
    require( './branching1a' );
    else
    _.include( 'Branching1' );

    if( includingWith === 'require' )
    require( './branching2a' );
    else
    _.include( 'Branching2' );

    console.log( 'main.after / isIncluded( Branching1 )', _.module.isIncluded( 'Branching1' ) );
    console.log( 'main.after / isIncluded( Branching2 )', _.module.isIncluded( 'Branching2' ) );

    var files = [ ... _.module.withName( 'Branching1' ).files.keys() ].sort();
    console.log( `Branching1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Branching2' ).files.keys() ].sort();
    console.log( `Branching2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function mainBranchingBefore2()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    _.module.predeclare( 'Branching1', __dirname + '/branching1a' );
    _.module.predeclare( 'Branching2', __dirname + '/branching2a' );

    console.log( 'main.mid / isIncluded( Branching1 )', _.module.isIncluded( 'Branching1' ) );
    console.log( 'main.mid / isIncluded( Branching2 )', _.module.isIncluded( 'Branching2' ) );

    if( includingWith === 'require' )
    require( './branching2a' );
    else
    _.include( 'Branching2' );

    if( includingWith === 'require' )
    require( './branching1a' );
    else
    _.include( 'Branching1' );

    console.log( 'main.after / isIncluded( Branching1 )', _.module.isIncluded( 'Branching1' ) );
    console.log( 'main.after / isIncluded( Branching2 )', _.module.isIncluded( 'Branching2' ) );

    var files = [ ... _.module.withName( 'Branching1' ).files.keys() ].sort();
    console.log( `Branching1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Branching2' ).files.keys() ].sort();
    console.log( `Branching2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function mainBranchingAfter1()
  {

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      require( './branching1a' );
      require( './branching2a' );
      _.module.predeclare( 'Branching1', __dirname + '/branching1a' );
      _.module.predeclare( 'Branching2', __dirname + '/branching2a' );
    }
    else if( order === 'rtp' )
    {
      require( './branching1a' );
      require( './branching2a' );
      _ = require( toolsPath );
      _.module.predeclare( 'Branching1', __dirname + '/branching1a' );
      _.module.predeclare( 'Branching2', __dirname + '/branching2a' );
    }
    else if( order === 'prt' )
    {
      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Branching1' ] = { entryPath : __dirname + '/branching1a' };
      _.module._modulesToPredeclare[ 'Branching2' ] = { entryPath : __dirname + '/branching2a' };
      require( './branching1a' );
      require( './branching2a' );
      _ = require( toolsPath );
    }

    console.log( 'main.after / isIncluded( Branching1 )', _.module.isIncluded( 'Branching1' ) );
    console.log( 'main.after / isIncluded( Branching2 )', _.module.isIncluded( 'Branching2' ) );

    var files = [ ... _.module.withName( 'Branching1' ).files.keys() ].sort();
    console.log( `Branching1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Branching2' ).files.keys() ].sort();
    console.log( `Branching2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function mainBranchingAfter2()
  {

    if( order === 'trp' )
    {
      _ = require( toolsPath );
      require( './branching2a' );
      require( './branching1a' );
      _.module.predeclare( 'Branching1', __dirname + '/branching1a' );
      _.module.predeclare( 'Branching2', __dirname + '/branching2a' );
    }
    else if( order === 'rtp' )
    {
      require( './branching2a' );
      require( './branching1a' );
      _ = require( toolsPath );
      _.module.predeclare( 'Branching1', __dirname + '/branching1a' );
      _.module.predeclare( 'Branching2', __dirname + '/branching2a' );
    }
    else if( order === 'prt' )
    {
      let g = get();
      _ = g.wTools = g.wTools || Object.create( null );
      _.module = _.module || Object.create( null );
      _.module._modulesToPredeclare = _.module._modulesToPredeclare || Object.create( null );
      _.module._modulesToPredeclare[ 'Branching1' ] = { entryPath : __dirname + '/branching1a' };
      _.module._modulesToPredeclare[ 'Branching2' ] = { entryPath : __dirname + '/branching2a' };
      require( './branching2a' );
      require( './branching1a' );
      _ = require( toolsPath );
    }

    console.log( 'main.after / isIncluded( Branching1 )', _.module.isIncluded( 'Branching1' ) );
    console.log( 'main.after / isIncluded( Branching2 )', _.module.isIncluded( 'Branching2' ) );

    var files = [ ... _.module.withName( 'Branching1' ).files.keys() ].sort();
    console.log( `Branching1\n  ${files.join( '\n  ' )}` );
    var files = [ ... _.module.withName( 'Branching2' ).files.keys() ].sort();
    console.log( `Branching2\n  ${files.join( '\n  ' )}` );
    var orphans = [ ... _.module.filesMap.values() ].filter( ( file ) => !file.module ).map( ( file ) => file.sourcePath );
    console.log( `orphans\n  ${orphans.join( '\n  ' )}` );

  }

  /* - */

  function branching1a()
  {
    console.log( 'branching1a' );
    require( './branching1b' );
  }

  /* - */

  function branching1b()
  {
    console.log( 'branching1b' );
    require( './branchingCommon' );
    require( './branching2b' );
  }

  /* - */

  function branching2a()
  {
    console.log( 'branching2a' );
    require( './branching2b' );
  }

  /* - */

  function branching2b()
  {
    console.log( 'branching2b' );
    require( './branchingCommon' );
    require( './branching1b' );
  }

  /* - */

  function branchingCommon()
  {
    console.log( 'branchingCommon' );
  }

  /* - */

}

predeclarePrime.timeOut = 120000;

//

function predeclareRelative( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `full relative path, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainWithFullPath ).filePath;
      a.program
      ({
        entry : module1,
        dirPath : 'node_modules',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main
module1
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `require name, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainWithRequireName ).filePath;
      a.program
      ({
        entry : module1,
        dirPath : 'node_modules',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main
module1
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

  }

  /* - */

  function mainWithFullPath()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    console.log( 'main' );
    _.module.predeclare({ name : 'Mod1', entryPath : __dirname + '/node_modules/module1' } );
    _.include( 'Mod1' );
  }

  /* - */

  function mainWithRequireName()
  {
    console.log( 'main' );
    require( 'module1' );
  }

  /* - */

  function module1()
  {
    console.log( 'module1' );
  }

  /* - */

}

//

function predeclareAbsolute( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `assumption, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainAssuption ).filePath;
      a.program
      ({
        entry : file1,
        dirPath : 'node_modules',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main
file1
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainProperCasedModule ).filePath;
      a.program
      ({
        entry : file1,
        dirPath : 'node_modules',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main
file1
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `upper cased module, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainUpperCasedModule ).filePath;
      a.program
      ({
        entry : file1,
        dirPath : 'node_modules',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`= Message of Error#1
    Cant resolve module::MOD1.
    Looked at:
     - MOD1`
      test.nil( op.exitCode, 0 );
      test.true( _.strHas( op.output, exp ) );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `lower cased module, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainLowerCasedModule ).filePath;
      a.program
      ({
        entry : file1,
        dirPath : 'node_modules',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`= Message of Error#1
    Cant resolve module::mod1.
    Looked at:
     - mod1`
      test.nil( op.exitCode, 0 );
      test.true( _.strHas( op.output, exp ) );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `upper cased include, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainUpperCaseInclude ).filePath;
      a.program
      ({
        entry : file1,
        dirPath : 'node_modules',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main
file1
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `relative, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( mainRelative ).filePath;
      a.program
      ({
        entry : file1,
        dirPath : 'dir1',
      });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main
file1
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

  }

  /* - */

  function mainAssuption()
  {
    console.log( 'main' );
    require( 'file1' );
  }

  /* - */

  function mainProperCasedModule()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    console.log( 'main' );
    _.module.predeclare({ name : 'Mod1', entryPath : 'file1' } );
    _.include( 'Mod1' );
  }

  /* - */

  function mainUpperCasedModule()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    console.log( 'main' );
    _.module.predeclare({ name : 'Mod1', entryPath : 'file1' } );
    _.include( 'MOD1' );
  }

  /* - */

  function mainLowerCasedModule()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    console.log( 'main' );
    _.module.predeclare({ name : 'Mod1', entryPath : 'file1' } );
    _.include( 'mod1' );
  }

  /* - */

  function mainUpperCaseInclude()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    console.log( 'main' );
    _.include( 'FILE1' );
  }

  /* - */

  function mainRelative()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    console.log( 'main' );
    _.module.predeclare({ name : 'Mod1', entryPath : './dir1/file1' } );
    _.include( 'Mod1' );
  }

  /* - */

  function file1()
  {
    console.log( 'file1' );
  }

  /* - */

}

predeclareAbsolute.description =
`
  - assumed npm can find file with relative path without dot if such put in node_modules directory
  - it is possible to declare module with name of file ( not absolute path )
  - include with upper-cased name of module cant find the module
  - but include of upper-cased name of npm module works
  - delcaring of module with relative entry path based on path of the current module file
`

//

function predeclareRedeclaring( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( main1 ).filePath;
      a.program( file1 );
      a.program( file2 );

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
main
file1
file2
./main1 : {- ModuleFile ./main1 -}
  global : real
  modules
    {- Module Module1 -}
  upFiles
    {- ModuleFile ${_.module.toolsPathGet()} -}
    {- ModuleFile ./file1 -}
    {- ModuleFile ./file2 -}
./file1 : {- ModuleFile ./file1 -}
  global : real
  modules
    {- Module Module1 -}
  downFiles
    {- ModuleFile ./main1 -}
    {- ModuleFile ./file2 -}
./file2 : {- ModuleFile ./file2 -}
  global : real
  modules
    {- Module Module1 -}
  downFiles
    {- ModuleFile ./main1 -}
  upFiles
    {- ModuleFile ${_.module.toolsPathGet()} -}
    {- ModuleFile ./file1 -}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

  }

  /* - */

  function main1()
  {
    console.log( 'main' );
    const _ = require( toolsPath );

    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __filename,
    });

    require( './file1' );
    require( './file2' );

    log( './main1' );
    log( './file1' );
    log( './file2' );

    function log( filePath )
    {
      let moduleFile = _.module.fileWith( filePath );
      if( !moduleFile )
      return console.log( `${filePath} : ${moduleFile}` );
      let output = _.module.fileExportString( moduleFile, { it : { verbosity : 2 } } ).resultExportString();
      output = _.strReplace( output, _.path.normalize( __dirname ), '.' );
      console.log( `${filePath} : ${output}` );
    }

  }

  /* - */

  function file1()
  {
    console.log( 'file1' );
  }

  /* - */

  function file2()
  {
    console.log( 'file2' );
    const _ = require( toolsPath );

    require( './file1' );

    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __filename,
    });

  }

  /* - */

}

predeclareRedeclaring.description =
`
  - redeclaring of module does not thow any error
  - module graph is proper after redeclaring
`

//

function predeclareRedeclaringSharedFile( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({ after : 0 });
  act({ after : 1 });

  return ready;

  /* - */

  function act( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `without redeclaring, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : module1, locals : _.props.extend( null, env, { withRedeclaring : 0 } ) }).filePath;
      a.program({ entry : module2, locals : _.props.extend( null, env, { withRedeclaring : 0 } ) });
      a.program({ entry : file1, locals : _.props.extend( null, env, { withRedeclaring : 0 } ) });
      a.program({ entry : file2, locals : _.props.extend( null, env, { withRedeclaring : 0 } ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
module1
file1
module2
file2
./module1 : {- ModuleFile ./module1 -}
  global : real
  modules
    {- Module Module1 -}
  upFiles
    {- ModuleFile ${_.module.toolsPathGet()} -}
    {- ModuleFile ./file1 -}
    {- ModuleFile ./module2 -}
./module2 : {- ModuleFile ./module2 -}
  global : real
  modules
    {- Module Module2 -}
  downFiles
    {- ModuleFile ./module1 -}
  upFiles
    {- ModuleFile ./file2 -}
    {- ModuleFile ./file1 -}
./file1 : {- ModuleFile ./file1 -}
  global : real
  modules
    {- Module Module1 -}
    {- Module Module2 -}
  downFiles
    {- ModuleFile ./module1 -}
    {- ModuleFile ./module2 -}
./file2 : {- ModuleFile ./file2 -}
  global : real
  modules
    {- Module Module2 -}
  downFiles
    {- ModuleFile ./module2 -}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

    ready.then( () =>
    {
      test.case = `without redeclaring, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program({ entry : module1, locals : _.props.extend( null, env, { withRedeclaring : 1 } ) }).filePath;
      a.program({ entry : module2, locals : _.props.extend( null, env, { withRedeclaring : 1 } ) });
      a.program({ entry : file1, locals : _.props.extend( null, env, { withRedeclaring : 1 } ) });
      a.program({ entry : file2, locals : _.props.extend( null, env, { withRedeclaring : 1 } ) });
      a.program({ entry : file3, locals : _.props.extend( null, env, { withRedeclaring : 1 } ) });

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
`
module1
file1
module2
file2
file3
./module1 : {- ModuleFile ./module1 -}
  global : real
  modules
    {- Module Module1 -}
  upFiles
    {- ModuleFile ${_.module.toolsPathGet()} -}
    {- ModuleFile ./file1 -}
    {- ModuleFile ./module2 -}
    {- ModuleFile ./file3 -}
./module2 : {- ModuleFile ./module2 -}
  global : real
  modules
    {- Module Module2 -}
  downFiles
    {- ModuleFile ./module1 -}
  upFiles
    {- ModuleFile ./file2 -}
    {- ModuleFile ./file1 -}
./file1 : {- ModuleFile ./file1 -}
  global : real
  modules
    {- Module Module1 -}
    {- Module Module2 -}
  downFiles
    {- ModuleFile ./module1 -}
    {- ModuleFile ./module2 -}
    {- ModuleFile ./file3 -}
./file2 : {- ModuleFile ./file2 -}
  global : real
  modules
    {- Module Module2 -}
  downFiles
    {- ModuleFile ./module2 -}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

    /* */

  }

  /* - */

  function module1()
  {
    console.log( 'module1' );
    const _ = require( toolsPath );

    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __filename,
    });

    require( './file1' );
    require( './module2' );

    if( withRedeclaring )
    require( './file3' );

    log( './module1' );
    log( './module2' );
    log( './file1' );
    log( './file2' );

    function log( filePath )
    {
      let moduleFile = _.module.fileWith( filePath );
      if( !moduleFile )
      return console.log( `${filePath} : ${moduleFile}` );
      let output = _.module.fileExportString( moduleFile, { it : { verbosity : 2 } } ).resultExportString();
      output = _.strReplace( output, _.path.normalize( __dirname ), '.' );
      console.log( `${filePath} : ${output}` );
    }

  }

  /* - */

  function file1()
  {
    console.log( 'file1' );
  }

  /* - */

  function module2()
  {
    console.log( 'module2' );
    let _ = _global_.wTools;

    if( !after )
    _.module.predeclare
    ({
      name : 'Module2',
      entryPath : __filename,
    });

    require( './file2' );
    require( './file1' );

    if( after )
    _.module.predeclare
    ({
      name : 'Module2',
      entryPath : __filename,
    });

  }

  /* - */

  function file2()
  {
    console.log( 'file2' );

  }

  /* - */

  function file3()
  {
    console.log( 'file3' );
    const _ = require( toolsPath );

    require( './file1' );

    _.module.predeclare
    ({
      alias : [ 'Module1', 'module1' ],
      entryPath : __filename,
    });

  }

  /* - */

}

predeclareRedeclaringSharedFile.description =
`
  - redeclaring of module does not unshare shared module file
`

//

function moduleIsIncluded( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  let start = __.process.starter
  ({
    outputCollecting : 1,
    outputPiping : 1,
    inputMirroring : 1,
    throwingExitCode : 0,
    mode : 'fork',
  });

  test.true( _.module.isIncluded( 'wTesting' ) );
  test.true( !_.module.isIncluded( 'abcdef123' ) );

  act({ entry : _programWithRequire });
  act({ entry : _programWithIncludeLower });
  act({ entry : _programWithIncludeUpper });

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;

      let program = __.program.make
      ({
        entry : env.entry,
        withSubmodules : 1,
        moduleFile : _.module.fileWith( 0 ),
        tempPath : a.abs( '.' ),
      });

      console.log( _.strLinesNumber( program.entry.routineCode ) );

      return start
      ({
        execPath : program.filePath,
        currentPath : _.path.dir( program.filePath ),
      })
    })
    .then( ( op ) =>
    {
      var exp =
  `
isIncluded( wLooker ) false
isIncluded( wlooker ) false
isIncluded( wLooker ) true
isIncluded( wlooker ) true
  `
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

  }

  /* - */

  function _programWithRequire()
  {
    const _ = require( toolsPath );
    console.log( 'isIncluded( wLooker )', _.module.isIncluded( 'wLooker' ) );
    console.log( 'isIncluded( wlooker )', _.module.isIncluded( 'wlooker' ) );
    _.include( 'wLooker' );
    console.log( 'isIncluded( wLooker )', _.module.isIncluded( 'wLooker' ) );
    console.log( 'isIncluded( wlooker )', _.module.isIncluded( 'wlooker' ) );
  }

  /* - */

  function _programWithIncludeLower()
  {
    const _ = require( toolsPath );
    console.log( 'isIncluded( wLooker )', _.module.isIncluded( 'wLooker' ) );
    console.log( 'isIncluded( wlooker )', _.module.isIncluded( 'wlooker' ) );
    _.include( 'wlooker' );
    console.log( 'isIncluded( wLooker )', _.module.isIncluded( 'wLooker' ) );
    console.log( 'isIncluded( wlooker )', _.module.isIncluded( 'wlooker' ) );
  }

  /* - */

  function _programWithIncludeUpper()
  {
    const _ = require( toolsPath );
    console.log( 'isIncluded( wLooker )', _.module.isIncluded( 'wLooker' ) );
    console.log( 'isIncluded( wlooker )', _.module.isIncluded( 'wlooker' ) );
    _.include( 'WLOOKER' );
    console.log( 'isIncluded( wLooker )', _.module.isIncluded( 'wLooker' ) );
    console.log( 'isIncluded( wlooker )', _.module.isIncluded( 'wlooker' ) );
  }

  /* - */

}

//

function moduleResolveFromAnotherGlobal( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    /* */

    ready.then( () =>
    {
      test.case = `throwing, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( main1 ).filePath;

      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );

      var exp =
`
_.module.resolve( Main1 ) : ${ a.abs( 'main1' ) }
__.module.resolve( Main1 ) : undefined
_.module.resolve( Main1 ) : ${ a.abs( 'main1' ) }
__.module.resolve( Main1 ) : ${ a.abs( 'main1' ) }
`
      test.equivalent( op.output, exp );

      return op;
    });

    /* */

  }

  /* - */

  function main1()
  {
    const _ = require( toolsPath );
    let __ = _.include( 'wTesting' );

    _.module.predeclare
    ({
      name : 'Main1',
      entryPath : __filename,
    });
    console.log( `_.module.resolve( Main1 ) : ${_.module.resolve( 'Main1' )}` );
    console.log( `__.module.resolve( Main1 ) : ${__.module.resolve( 'Main1' )}` );

    __.module.predeclare
    ({
      name : 'Main1',
      entryPath : __filename,
    });
    console.log( `_.module.resolve( Main1 ) : ${_.module.resolve( 'Main1' )}` );
    console.log( `__.module.resolve( Main1 ) : ${__.module.resolve( 'Main1' )}` );
  }

  /* - */

}

moduleResolveFromAnotherGlobal.description =
`
- global namespace::testing have its own space of modules, so __.module.resolve will throw error, unless this special case is handled somehow
- if error not throwen then __.module.resolve after declaration of module should give correct path
`

//

/* xxx : move the test to introspector */
function programMakeOptionWithSubmodule( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;

      let files =
      {
        mainRegisterBefore,
        programRoutine1 : { routine : programRoutine1, dirPath : 'dir' },
        programRoutine2,
      }

      /* xxx : add test of program.make with different name of program file and name of routine and reusing the same routine */
      /* xxx : add test of program.make with different dirPaths */
      let program = __.program.make
      ({
        files,
        entry : mainRegisterBefore,
        moduleFile : module,
        withSubmodules : 1,
      });

      return program.start();
    })
    .then( ( op ) =>
    {
      var exp =
  `
  isIncluded( wTesting ) false
  isIncluded( wTesting ) true
  `
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

  }

  /* - */

  function mainRegisterBefore()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );
    console.log( 'main / before / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'main / before / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );
    require( 'dir/programRoutine1' );
    console.log( 'main / after / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'main / after / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );
  }

  /* - */

  function programRoutine1()
  {
    const _ = _global_.wTools;
    console.log( 'programRoutine1 / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'programRoutine1 / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );
    require( '../programRoutine2' );
  }

  /* - */

  function programRoutine2()
  {
    const _ = _global_.wTools;
    console.log( 'programRoutine2 / isIncluded( Program1 )', _.module.isIncluded( 'Program1' ) );
    console.log( 'programRoutine2 / isIncluded( Program2 )', _.module.isIncluded( 'Program2' ) );
  }

  /* - */

}

programMakeOptionWithSubmodule.experimental = 1; /* xxx : remove */

//

function programInheritedModuleFilePaths( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;

      var filePath = a.program( programRoutine1 ).filePath;
      a.program( programRoutine2 );
      a.program({ entry : program3, dirPath : 'dir', });
      return a.forkNonThrowing
      ({
        execPath : filePath,
        currentPath : _.path.dir( filePath ),
      })
    })
    .then( ( op ) =>
    {

      var exp =
`
programRoutine1.paths
  ${trailOf( __dirname, a.abs( '.' ) )}
programRoutine2.paths
  ${trailOf( __dirname, a.abs( '.' ) )}
program3.paths
  ${trailOf( __dirname, a.abs( 'dir' ) )}
`
      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

  }

  /* - */

  function programRoutine1()
  {
    console.log( `programRoutine1.paths\n  ${module.paths.join( '\n  ' )}` );
    require( './programRoutine2' );
  }

  /* - */

  function programRoutine2()
  {
    console.log( `programRoutine2.paths\n  ${module.paths.join( '\n  ' )}` );
    require( './dir/program3' );
  }

  /* - */

  function program3()
  {
    console.log( `program3.paths\n  ${module.paths.join( '\n  ' )}` );
  }

  /* - */

  function trailOf()
  {
    let result = [];
    for( let a = arguments.length-1 ; a >= 0 ; a-- )
    {
      let filePath = arguments[ a ];
      let trace = __.path.traceToRoot( filePath );
      if( process.platform === 'win32' )
      trace.splice( 0, 1 );
      _.arrayPrependArrayOnce( result, __.path.s.nativize( __.path.s.join( trace, 'node_modules' ) ).reverse() );
    }
    return '  ' + result.join( '\n  ' );
  }

  /* - */

}

/* xxx : duplicate test routine in module::wIntrospectorBasics */
programInheritedModuleFilePaths.description =
`
program should inherit path of parent
`

//

/* xxx : duplicate test routine in module::wIntrospectorBasics */
function programLocalsChanging( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({ tools : 'testing' });
  // act({ tools : 'real' });
  // xxx : switch on in introspector

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;
      const tools = _globals_[ env.tools ].wTools;

      var locals = { local1 : { a : 1 } };
      var program1 = tools.program.make
      ({
        entry : programRoutine1,
        tempPath : a.abs( '.' ),
        locals,
      });
      test.true( _.aux.is( program1.group.locals ) );
      test.true( program1.group.locals.a === locals.a );
      test.true( program1.group.locals === locals );
      locals.local1 = { a : 2 };
      var program2 = tools.program.make
      ({
        entry : programRoutine2,
        tempPath : a.abs( '.' ),
        locals,
      });
      test.true( _.aux.is( program2.group.locals ) );
      test.true( program2.group.locals.a === locals.a );
      test.true( program2.group.locals === locals );
      locals.local1 = { a : 3 };

      return a.forkNonThrowing
      ({
        execPath : [ program1.filePath, program2.filePath ],
        currentPath : program1.group.tempPath,
      });
    })
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );

      var exp = `programRoutine1.local1.a : 1`;
      test.equivalent( op.sessions[ 0 ].output, exp );

      var exp = `programRoutine2.local1.a : 2`;
      test.equivalent( op.sessions[ 1 ].output, exp );

      return op;
    });

  }

  /* - */

  function programRoutine1()
  {
    console.log( `programRoutine1.local1.a : ${local1.a}` );
  }

  /* - */

  function programRoutine2()
  {
    console.log( `programRoutine2.local1.a : ${local1.a}` );
  }

  /* - */

}

programLocalsChanging.description =
`
  - changing of locals after call of write does not has impact on written program
  - program.write does not clone lolcals map
`

//

/* xxx : duplicate test routine in module::wIntrospectorBasics */
/* xxx : in module::wIntrospectorBasic evolve exporting of locas and cover it.
  - make possible exporting of more complex structures
  - make possible exporting into global
*/

function programOptionLocalsRoutines( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;
      const tools = __;
      // const tools = _; /* xxx : use in introspector */

      var locals = { a : 1, routine1 };
      var program1 = tools.program.make
      ({
        entry : programRoutine1,
        tempPath : a.abs( '.' ),
        locals,
      });
      test.true( _.aux.is( program1.group.locals ) );

      return a.forkNonThrowing
      ({
        execPath : program1.filePath,
        currentPath : program1.group.tempPath,
      });
    })
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      var exp =
`
programRoutine1.a : 1
routine1.a : 1
`;
      test.equivalent( op.output, exp );
      return op;
    });

  }

  /* - */

  function programRoutine1()
  {
    console.log( `programRoutine1.a : ${a}` );
    routine1();
  }

  /* - */

  function routine1()
  {
    console.log( `routine1.a : ${a}` );
  }

  /* - */

}

programOptionLocalsRoutines.description =
`
  - basic passing routines in locals map make possible to call it in program
`

//

function selfFindAssumption( test )
{
  let context = this;
  let a = test.assetFor( false );

  programWrite( '.', programRoutine1, 'programRoutine1' );
  programWrite( 'dir1/dir2', programRoutine2, 'programRoutine2' );
  programWrite( 'dir1/node_modules', program3a, 'program3' );
  programWrite( 'node_modules', program3b, 'program3' );

  return a.fork({ execPath : a.abs( 'programRoutine1' ) })
  .then( ( op ) =>
  {
    var exp =
`
programRoutine1
programRoutine2
program3a
`
    test.equivalent( op.output, exp );
    test.identical( op.exitCode, 0 );
    return op;
  });

  /* */

  function programWrite( dirPath, program, name )
  {
    var postfix =
    `
    ${program.name}();
    `
    __.fileProvider.fileWrite( a.abs( dirPath, name ), program.toString() + postfix );
  }

  /* */

  function programRoutine1()
  {
    console.log( 'programRoutine1' );
    require( './dir1/dir2/programRoutine2' );
  }

  /* */

  function programRoutine2()
  {
    console.log( 'programRoutine2' );
    require( 'program3' );
  }

  /* */

  function program3a()
  {
    console.log( 'program3a' );
    require( 'program3' );
  }

  /* */

  function program3b()
  {
    console.log( 'program3b' );
  }

  /* */

}

selfFindAssumption.description =
`
  - include of a file with the same name as itself from node_modules dir does not find it, but find itself
`

//

function localPathAssumption( test )
{
  let context = this;
  let a = test.assetFor( false );

  programWrite( programRoutine1 );
  programWrite( programRoutine2 );
  programWrite( program3 );

  return a.fork({ execPath : a.abs( 'programRoutine1' ) })
  .then( ( op ) =>
  {
    var exp =
`
programRoutine1.before.paths
  ${trailOf( a.abs( '.' ) )}
  /pro
programRoutine2.before.paths
  ${trailOf( a.abs( '.' ) )}
  /programRoutine2/local
program3.paths
  ${trailOf( a.abs( '.' ) )}
  /program3/local
programRoutine2.after.paths
  ${trailOf( a.abs( '.' ) )}
  /programRoutine2/local
programRoutine1.after.paths
  ${trailOf( a.abs( '.' ) )}
  /pro
`
    test.equivalent( op.output, exp );
    test.identical( op.exitCode, 0 );
    return op;
  });

  /* */

  function programWrite( program )
  {
    var postfix =
    `
    ${program.name}();
    `
    __.fileProvider.fileWrite( a.abs( program.name ), program.toString() + postfix );
  }

  /* */

  function programRoutine1()
  {
    module.paths.push( '/pro' );
    console.log( `programRoutine1.before.paths\n  ${module.paths.join( '\n  ' )}` );
    require( './programRoutine2' );
    console.log( `programRoutine1.after.paths\n  ${module.paths.join( '\n  ' )}` );
  }

  /* */

  function programRoutine2()
  {
    module.paths.push( '/programRoutine2/local' );
    console.log( `programRoutine2.before.paths\n  ${module.paths.join( '\n  ' )}` );
    require( './program3' );
    console.log( `programRoutine2.after.paths\n  ${module.paths.join( '\n  ' )}` );
  }

  /* */

  function program3()
  {
    module.paths.push( '/program3/local' );
    console.log( `program3.paths\n  ${module.paths.join( '\n  ' )}` );
  }

  /* - */

  function trailOf()
  {
    let result = [];
    for( let a = arguments.length-1 ; a >= 0 ; a-- )
    {
      let filePath = arguments[ a ];
      let trace = __.path.traceToRoot( filePath );
      if( process.platform === 'win32' )
      trace.splice( 0, 1 );
      _.arrayPrependArrayOnce( result, __.path.s.nativize( __.path.s.join( trace, 'node_modules' ) ).reverse() );
    }
    return '  ' + result.join( '\n  ' );
  }

  /* */

}

localPathAssumption.description =
`
  - verify assumption about inheritance of local paths of module files of NPM
`

//

function globalPathAssumption( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ModuleFileNative = require( 'module' );

  programWrite( programRoutine1 );
  programWrite( programRoutine2 );
  programWrite( program3 );

  return a.fork({ execPath : a.abs( 'programRoutine1' ) })
  .then( ( op ) =>
  {
    var exp =
`
programRoutine1.before.globalPaths
  ${ModuleFileNative.globalPaths.join( '\n' )}
  /programRoutine1/global
programRoutine2.before.globalPaths
  ${ModuleFileNative.globalPaths.join( '\n' )}
  /programRoutine1/global
  /programRoutine2/global
program3.globalPaths
  ${ModuleFileNative.globalPaths.join( '\n' )}
  /programRoutine1/global
  /programRoutine2/global
  /program3/global
programRoutine2.after.globalPaths
  ${ModuleFileNative.globalPaths.join( '\n' )}
  /programRoutine1/global
  /programRoutine2/global
  /program3/global
programRoutine1.after.globalPaths
  ${ModuleFileNative.globalPaths.join( '\n' )}
  /programRoutine1/global
  /programRoutine2/global
  /program3/global
`
    test.equivalent( op.output, exp );
    test.identical( op.exitCode, 0 );
    return op;
  });

  /* */

  function programWrite( program )
  {
    var postfix =
    `
    ${program.name}();
    `
    __.fileProvider.fileWrite( a.abs( program.name ), program.toString() + postfix );
  }

  /* */

  function programRoutine1()
  {
    let ModuleFileNative = require( 'module' );
    ModuleFileNative.globalPaths.push( '/programRoutine1/global' );
    console.log( `programRoutine1.before.globalPaths\n  ${ModuleFileNative.globalPaths.join( '\n  ' )}` );
    require( './programRoutine2' );
    console.log( `programRoutine1.after.globalPaths\n  ${ModuleFileNative.globalPaths.join( '\n  ' )}` );
  }

  /* */

  function programRoutine2()
  {
    let ModuleFileNative = require( 'module' );
    ModuleFileNative.globalPaths.push( '/programRoutine2/global' );
    console.log( `programRoutine2.before.globalPaths\n  ${ModuleFileNative.globalPaths.join( '\n  ' )}` );
    require( './program3' );
    console.log( `programRoutine2.after.globalPaths\n  ${ModuleFileNative.globalPaths.join( '\n  ' )}` );
  }

  /* */

  function program3()
  {
    let ModuleFileNative = require( 'module' );
    ModuleFileNative.globalPaths.push( '/program3/global' );
    console.log( `program3.globalPaths\n  ${ModuleFileNative.globalPaths.join( '\n  ' )}` );
  }

  /* */

}

globalPathAssumption.description =
`
  - verify assumption about inheritance of global paths of module files of NPM
`

//

function experiment( test )
{
  test.true( true );
}

experiment.experimental = 1;

//

function requireModuleFileWithAccessor( test )
{
  let a = test.assetFor( false );
  let programRoutine1Path = a.program({ entry : programRoutine1 }).filePath;

  a.program({ entry : programRoutine2 });

  /* */

  a.fork({ execPath : programRoutine1Path })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    test.identical( _.strCount( op.output, 'programRoutine1.begin' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine1.end' ), 1 );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function programRoutine1()
  {
    console.log( 'programRoutine1.begin' )

    const _ = require( toolsPath );
    const p2 = require( './programRoutine2' );

    console.log( 'programRoutine1.end' )
  }

  /* */

  function programRoutine2()
  {
    console.log( 'programRoutine2.begin' )

    Object.defineProperty( module, 'exports',
    {
    	enumerable : true,
    	get,
    });

    console.log( 'programRoutine2.end' )

    function get()
    {
      return { a : 1 };
    }

  }

  /* */

}

requireModuleFileWithAccessor.description =
`
- exports of module file may be defined with accessor returning different entity with each attempt of getting it
- no error  should be throwen in such case
`

//

/* xxx : duplicate in module::Testing */
function testingOnL1( test )
{
  let a = test.assetFor( false );
  let locals =
  {
    toolsPath : __.path.nativize( __.path.normalize( __dirname + '/../../../../node_modules/wTools.l1' ) ),
    testingPath : __.path.nativize( _.module.resolve( 'wTesting' ) ),
    test1,
    programRoutine2,
  }
  let program = a.program({ entry : programRoutine1, locals });

  /* */

  program.start()
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    return null;
  });

  /* */

  return a.ready;

  /* */

  /* xxx : fix coloring problems
  node wtools/abase/l0/l9.test/Module.test.s n:1 v:7 s:0 r:testingOnL1
  */

  function programRoutine1()
  {
    console.log( `programRoutine1.toolsPath : ${toolsPath}` );
    console.log( `programRoutine1.testingPath : ${testingPath}` );
    require( testingPath );
    const _ = require( toolsPath );

    const Proto =
    {
      verbosity : 8,
      tests :
      {
        test1,
      }
    }
    const Self = wTestSuite( Proto );
    if( typeof module !== 'undefined' && !module.parent )
    wTester.test( Self.name );

  }

  function test1( test )
  {
    let a = test.assetFor( false );
    let program = a.program({ entry : programRoutine2 });
    test.true( true );
    console.log( 'test1!' );

    program.start()
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      return null;
    });

    return a.ready;
  }

  function programRoutine2()
  {
    console.log( `programRoutine2.toolsPath : ${toolsPath}` );
  }

}

testingOnL1.description =
`
- running of test from l1 works
`

//

function l1Environment( test )
{
  let a = test.assetFor( false );
  let tools1Path = __.path.nativize( __.path.normalize( __dirname + '/../../../../node_modules/wTools.l1' ) );
  let toolsPath = _.module.toolsPathGet();
  let locals =
  {
    tools1Path,
    toolsPath,
  }
  let program = a.program({ entry : r1, locals });
  a.program({ entry : r2, locals });
  a.program({ entry : r3, locals });
  a.program({ entry : r4, locals });

  test.true( a.fileProvider.fileExists( toolsPath ) );
  console.log( `r1.toolsPath : ${toolsPath}` );

  /* */

  program.start()
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );

    var exp =
`
r2
r3
r4
r1.real.name : real
r1.real.theStatus : r1
r1.test1.theStatus : r4
r1.real.name : real
r1.real.theStatus : r1
r1.test1.theStatus : r4
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function r1()
  {
    const _ = require( tools1Path );
    _global_.theStatus = 'r1';
    require( './r2' );
    console.log( `r1.${_global_.__GLOBAL_NAME__}.name : ${_global_.__GLOBAL_NAME__}` );
    console.log( `r1.${_globals_.real.__GLOBAL_NAME__}.theStatus : ${_globals_.real.theStatus}` );
    console.log( `r1.${_globals_.test1.__GLOBAL_NAME__}.theStatus : ${_globals_.test1.theStatus}` );
    setTimeout( () =>
    {
      includeR4();
      console.log( `r1.${_global_.__GLOBAL_NAME__}.name : ${_global_.__GLOBAL_NAME__}` );
      console.log( `r1.${_globals_.real.__GLOBAL_NAME__}.theStatus : ${_globals_.real.theStatus}` );
      console.log( `r1.${_globals_.test1.__GLOBAL_NAME__}.theStatus : ${_globals_.test1.theStatus}` );
    }
    , 100 );
  }

  function r2()
  {
    const __ = wTools;
    const _global = __.global.makeAndOpen( module, 'test1' );
    const _ = require( toolsPath );

    _global_.theStatus = 'r2';

    console.log( `r2` );
    require( './r3' );

    _.global.close( 'test1' );
  }

  function r3()
  {
    console.log( `r3` );
    require( './r4' );
    _realGlobal_.includeR4 = function()
    {
      require( './r4' );
    }
  }

  function r4()
  {
    console.log( `r4` );
    _global_.theStatus = 'r4';
  }

}

l1Environment.description =
`
- r4 should be included only once and only in namespace::test1
- if real namespace include only l1 then "ModuleFileNative._load = _loadEnvironment" should be assigned anyway in secondary namespace
`

//

function l1SecondRequire( test )
{
  let a = test.assetFor( false );
  let tools1Path = __.path.nativize( __.path.normalize( __dirname + '/../../../../node_modules/wTools.l1' ) );
  let toolsPath = _.module.toolsPathGet();
  let locals =
  {
    tools1Path,
    toolsPath,
  }
  let program = a.program({ entry : r1, locals });
  a.program({ entry : r2, locals });
  a.program({ entry : r3, locals });
  a.program({ entry : includeTools, locals });

  test.true( a.fileProvider.fileExists( toolsPath ) );
  console.log( `r1.toolsPath : ${toolsPath}` );

  /* */

  program.start()
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );

    var exp =
`
r2
r3
r2
_ = __ : true
r1.real.name : real
{- ModuleFile ${a.abs( 'r1' )} -}
  {- ModuleFile ${__.path.dir( _.module.toolsPathGet() )}/wTools.l1 -}
  {- ModuleFile ${a.abs( 'r2' )} -}
  {- ModuleFile ${a.abs( 'includeTools' )} -}
  {- ModuleFile ${_.module.toolsPathGet()} -}
  {- ModuleFile ${a.abs( 'r3' )} -}
`
    test.equivalent( op.output, exp );

    return null;
  });

  /* */

  return a.ready;

  /* */

  function r1()
  {
    let _ = require( tools1Path );
    require( './r2' );
    require( './includeTools' );
    let __ = require( toolsPath );
    console.log( `_ = __ : ${_ === __}` );
    console.log( `r1.${_global_.__GLOBAL_NAME__}.name : ${_global_.__GLOBAL_NAME__}` );
    require( './r3' );

    console.log( module.universal );
    module.universal.upFiles.forEach( ( file ) => console.log( `  ${file}` ) );

  }

  function r2()
  {
    console.log( `r2` );
    require( './r3' );
  }

  function r3()
  {
    console.log( `r3` );
  }

  function includeTools()
  {
    const __ = wTools;
    const _global = __.global.makeAndOpen( module, 'test1' );
    const _ = require( toolsPath );
    console.log( `r2` );
    _.global.close( 'test1' );
  }

}

l1SecondRequire.description =
`
- second require in the main namespace should add up even if _loadEnvironment is registered in secondary namespace
`

//

function secondaryNamespaceSecondRequire( test )
{
  let a = test.assetFor( false );
  let files =
  {
    r1,
    r2,
    r3,
    secondary1,
    secondary2,
    secondary3,
    secondary4,
    common2,
    common3,
  }
  let program = a.program({ entry : r1, files });

  /* */

  act({ method : 'require' });
  act({ method : 'include' });

  return a.ready;

  /* - */

  function act( env )
  {

    program.start({ args : [ 0, env.method ] })
    .then( ( op ) =>
    {
      test.case = `without tree, ${__.entity.exportStringSolo( env )}`;
      test.identical( op.exitCode, 0 );
      var exp =
  `
  secondary3
  secondary4
  {- ModuleFile ${ a.abs( 'secondary2' ) } -}
    secondary2  {- ModuleFile ${ a.abs( 'secondary3' ) } -}
    secondary2  {- ModuleFile ${ a.abs( 'secondary4' ) } -}
  common2
  common3
  {- ModuleFile ${ a.abs( 'secondary1' ) } -}
    secondary1  {- ModuleFile ${ _.module.toolsPathGet() } -}
    secondary1  {- ModuleFile ${ a.abs( 'secondary2' ) } -}
    secondary1  {- ModuleFile ${ a.abs( 'secondary3' ) } -}
    secondary1  {- ModuleFile ${ a.abs( 'common2' ) } -}
    secondary1  {- ModuleFile ${ a.abs( 'common3' ) } -}
  r2
  r3
  common2
  common3
  {- ModuleFile ${ a.abs( 'r1' ) } -}
    r1  {- ModuleFile ${ _.module.toolsPathGet() } -}
    r1  {- ModuleFile ${ a.abs( 'secondary1' ) } -}
    r1  {- ModuleFile ${ a.abs( 'r2' ) } -}
    r1  {- ModuleFile ${ a.abs( 'r3' ) } -}
    r1  {- ModuleFile ${ a.abs( 'common2' ) } -}
    r1  {- ModuleFile ${ a.abs( 'common3' ) } -}
  `
      test.equivalent( op.output, exp );

      return null;
    });

    /* */

    program.start({ args : [ 1, env.method ] })
    .then( ( op ) =>
    {
      test.case = `with tree, ${__.entity.exportStringSolo( env )}`;
      test.identical( op.exitCode, 0 );
      var exp =
  `
  secondary3
  secondary4

  secondary2:
  {- ModuleFile ${ a.abs( 'secondary2' ) } -}
    {- ModuleFile ${ a.abs( 'secondary3' ) } -}
      {- ModuleFile ${ a.abs( 'secondary4' ) } -}
    {- ModuleFile ${ a.abs( 'secondary4' ) } -}

  common2
  common3

  secondary1:
  {- ModuleFile ${ a.abs( 'secondary1' ) } -}
    {- ModuleFile ${ _.module.toolsPathGet() } -}
      {- ModuleFile ${ __.path.join( _.module.toolsPathGet(), '../wTools' ) } -}
    {- ModuleFile ${ a.abs( 'secondary2' ) } -}
      {- ModuleFile ${ a.abs( 'secondary3' ) } -}
      {- ModuleFile ${ a.abs( 'secondary4' ) } -}
    {- ModuleFile ${ a.abs( 'secondary3' ) } -}
      {- ModuleFile ${ a.abs( 'secondary4' ) } -}
    {- ModuleFile ${ a.abs( 'common2' ) } -}
      {- ModuleFile ${ a.abs( 'common3' ) } -}
    {- ModuleFile ${ a.abs( 'common3' ) } -}

  r2
  r3
  common2
  common3

  r1:
  {- ModuleFile ${ a.abs( 'r1' ) } -}
    {- ModuleFile ${ _.module.toolsPathGet() } -}
      {- ModuleFile ${ __.path.join( _.module.toolsPathGet(), '../wTools' ) } -}
    {- ModuleFile ${ a.abs( 'secondary1' ) } -}
      {- ModuleFile ${ _.module.toolsPathGet() } -}
      {- ModuleFile ${ a.abs( 'secondary2' ) } -}
      {- ModuleFile ${ a.abs( 'secondary3' ) } -}
      {- ModuleFile ${ a.abs( 'common2' ) } -}
      {- ModuleFile ${ a.abs( 'common3' ) } -}
    {- ModuleFile ${ a.abs( 'r2' ) } -}
      {- ModuleFile ${ a.abs( 'r3' ) } -}
    {- ModuleFile ${ a.abs( 'r3' ) } -}
    {- ModuleFile ${ a.abs( 'common2' ) } -}
      {- ModuleFile ${ a.abs( 'common3' ) } -}
    {- ModuleFile ${ a.abs( 'common3' ) } -}
  `
      test.equivalent( op.output, exp );

      return null;
    });

  }

  /* */

  return a.ready;

  /* */

  function r1()
  {
    const _ = require( toolsPath );
    _realGlobal_.withTree = Number( process.argv[ 2 ] );
    _realGlobal_.method = process.argv[ 3 ];
    require( './secondary1' );
    require( './r2' );
    if( method === 'require' )
    require( './r3' );
    else
    _.include( 'moduleR3' );
    require( './common2' );

    if( method === 'require' )
    require( './common3' );
    else
    _.include( 'moduleCommon3' );

    if( withTree )
    {
      console.log( '\nr1:\n' + _.module.fileExportString( module.universal, { it : { verbosity : 1, recursive : 3 } } ).resultExportString() + '\n' );
    }
    else
    {
      console.log( module.universal );
      module.universal.upFiles.forEach( ( file ) => console.log( `  r1  ${file}` ) );
    }

  }

  function r2()
  {
    console.log( `r2` );
    require( './r3' );
  }

  function r3()
  {
    const _ = _global_.wTools;
    _.module.predeclare
    ({
      alias : [ 'moduleR3' ],
      entryPath : __filename,
    });
    console.log( `r3` );
  }

  function secondary1()
  {
    const __ = wTools;
    const _global = __.global.makeAndOpen( module, 'test1' );
    const _ = require( toolsPath );

    require( './secondary2' );
    if( method === 'require' )
    require( './secondary3' );
    else
    _.include( 'moduleSecondary3' );

    require( './common2' );
    if( method === 'require' )
    require( './common3' );
    else
    _.include( 'moduleCommon3' );

    if( withTree )
    {
      console.log( '\nsecondary1:\n' + _.module.fileExportString( module.universal, { it : { verbosity : 1, recursive : 3 } } ).resultExportString() + '\n' );
    }
    else
    {
      console.log( module.universal );
      module.universal.upFiles.forEach( ( file ) => console.log( `  secondary1  ${file}` ) );
    }

    _.global.close( 'test1' );
  }

  function secondary2()
  {
    const _ = _global_.wTools;
    require( './secondary3' );
    if( method === 'require' )
    require( './secondary4' );
    else
    _.include( 'moduleSecondary4' );

    if( withTree )
    {
      console.log( '\nsecondary2:\n' + _.module.fileExportString( module.universal, { it : { verbosity : 1, recursive : 3 } } ).resultExportString() + '\n' );
    }
    else
    {
      console.log( module.universal );
      module.universal.upFiles.forEach( ( file ) => console.log( `  secondary2  ${file}` ) );
    }

  }

  function secondary3()
  {
    const _ = _global_.wTools;
    _.module.predeclare
    ({
      alias : [ 'moduleSecondary3' ],
      entryPath : __filename,
    });
    console.log( `secondary3` );
    require( './secondary4' );
  }

  function secondary4()
  {
    const _ = _global_.wTools;
    _.module.predeclare
    ({
      alias : [ 'moduleSecondary4' ],
      entryPath : __filename,
    });
    console.log( `secondary4` );
  }

  function common2()
  {
    console.log( `common2` );
    require( './common3' );
  }

  function common3()
  {
    const _ = _global_.wTools;
    _.module.predeclare
    ({
      alias : [ 'moduleCommon3' ],
      entryPath : __filename,
    });
    console.log( `common3` );
  }

}

secondaryNamespaceSecondRequire.description =
`
- second require in both main and secondary namespace should add element to upFiles of down module file
`

//

function requireSameModuleTwice( test )
{
  let context = this;
  let a = test.assetFor( false );
  let program;

  /* */

  a.ready.then( () =>
  {
    test.case = 'with require';
    program = a.program({ entry : withRequire, tempPath : a.abs( 'node_modules/withrequire' ) });
    var packageFile = { name : 'withrequire', main : 'withRequire' };
    a.fileProvider.fileWrite
    ({
      filePath : a.abs( `node_modules/withrequire/package.json` ),
      data : packageFile,
      encoding : 'json',
    })
    return program.start({ ready : null });
  })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    return null;
  });

  /* */

  a.ready.then( () =>
  {
    test.case = 'with include';
    program = a.program({ entry : withInclude, tempPath : a.abs( 'node_modules/withinclude' ) });
    var packageFile = { name : 'withinclude', main : 'withInclude' };
    a.fileProvider.fileWrite
    ({
      filePath : a.abs( `node_modules/withinclude/package.json` ),
      data : packageFile,
      encoding : 'json',
    })
    return program.start({ ready : null });
  })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function withRequire()
  {
    _ = Object.create( null );
    if( process.platform === 'linux' )
    require( 'withrequire' );
    else
    require( 'withRequire' );
    if( _.included )
    throw new Error( 'Module withRequire is included for the second time' );
    _.included = Object.create( null );
    module.exports = _;
  }

  /* */

  function withInclude()
  {
    const _ = require( toolsPath );
    _.include( 'withInclude' );
    if( _.included )
    throw new Error( 'Module withInclude is included for the second time' );
    _.included = Object.create( null );
    module.exports = _global_.wTools;
  }

}

requireSameModuleTwice.description =
`
- Module moduleA should not be included for the second time, cached version should be used instead
`

/*

  let _ToolsPath_ = a.path.nativize( _.module.toolsPathGet() );
  let programRoutine1Path = a.program({ routine : programRoutine1, locals : { _ToolsPath_ } }).filePath;

*/

//

function nativeModuleFileDeleting( test )
{
  let a = test.assetFor( false );
  let files =
  {
    r1,
    r2,
  }
  let locals =
  {
    log,
  }
  let program = a.program({ entry : r1, files, locals });

  /* */

  act({});

  return a.ready;

  /* - */

  function act( env )
  {

    /* */

    program.start({ args : [ 0 ] })
    .then( ( op ) =>
    {
      test.case = `without deleting, ${__.entity.exportStringSolo( env )}`;
      test.identical( op.exitCode, 0 );
      var exp =
`
r2
= f:r1
{- ModuleFile ${ a.abs( 'r1' ) } -}
  {- ModuleFile ${_.module.toolsPathGet()} -}
  {- ModuleFile ${ a.abs( 'r2' ) } -}
`

      test.equivalent( op.output, exp );
      return null;
    });

    /* */

    program.start({ args : [ 1 ] })
    .then( ( op ) =>
    {
      test.case = `with deleting, ${__.entity.exportStringSolo( env )}`;
      test.identical( op.exitCode, 0 );
      var exp =
`
r2
r2
= f:r1
{- ModuleFile ${ a.abs( 'r1' ) } -}
  {- ModuleFile ${_.module.toolsPathGet()} -}
  {- ModuleFile ${ a.abs( 'r2' ) } -}
`

      test.equivalent( op.output, exp );
      return null;
    });

    /* */

  }

  /* - */

  function log( moduleFile, verbosity )
  {
    const _ = _global_.wTools;
    let prefix = `f:${_.path.fullName( moduleFile.sourcePath )}`;
    console.log( `= ${prefix}\n${_.module.fileExportString( moduleFile, { it : { verbosity : 1, recursive : 2 } } ).resultExportString()}` );
  }

  /* - */

  function r1()
  {
    const _ = require( toolsPath );
    _realGlobal_.deleting = Number( process.argv[ 2 ] );
    require( './r2' );
    if( deleting )
    delete _.module.nativeFilesMap[ _.path.nativize( __dirname + '/r2' ) ];
    require( './r2' );
    log( _.module.fileUniversalFrom( module ) );
  }

  /* - */

  function r2()
  {
    console.log( `r2` );
  }

  /* - */

}

nativeModuleFileDeleting.description =
`
- deleting native module file is possible by user land code. it should not cause problems
`

//

function stealthyRequireIssue( test )
{
  let context = this;
  let a = test.assetFor( false );
  let programRoutine1Path = a.program( programRoutine1 ).filePath;

  /* */

  begin()
  a.appStartNonThrowing({ execPath : programRoutine1Path })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    test.identical( _.strCount( op.output, 'programRoutine1.begin' ), 1 );
    test.identical( _.strCount( op.output, 'programRoutine1.end' ), 1 );
    return null;
  });

  /* */

  return a.ready;

  function programRoutine1()
  {
    console.log( 'programRoutine1.begin' );
    const _ = require( toolsPath );
    require( 'jsdom' );
    console.log( 'programRoutine1.end' );
  }

  /* */

  function begin()
  {
    let packageFile =
    {
      dependencies :
      {
        "jsdom": "16.4.0"
      }
    }

    a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageFile, encoding : 'json' })

    a.shell( 'npm i' )
  }
}

stealthyRequireIssue.timeOut = 120000;
stealthyRequireIssue.description =
`
- That https://github.com/analog-nico/stealthy-require is working
`;

//

function fileExportBasic( test )
{
  const basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
  const baseAbs = __.routine.join( __.path, __.path.join, [ basePath ] );

  console.log( '' );
  console.log( _.module.rootFile.sourcePath );
  console.log( _.module.fileExportString( _.module.rootFile, { it : { verbosity : 2, recursive : 4 } } ).resultExportString() );
  console.log( '' );
  console.log( _.module.resolve( 'wTesting' ) );
  console.log( _.module.fileExportString( _.module.fileWith( _.module.resolve( 'wTesting' ) ), { it : { verbosity : 2, recursive : 4 } } ).resultExportString() );

  console.log( '' );
  console.log( _.module.resolve( 'wTesting' ) );
  console.log( _.module.exportString( _.module.fileWith( _.module.resolve( 'wTesting' ) ).module, { it : { verbosity : 2, recursive : 4 } } ).resultExportString() );

  // console.log( '' );
  // console.log( _globals_.testing.wTools.module.resolve( 'wTesting' ) );
  // console.log( _.module.exportString( __.module.fileWith( _globals_.testing.wTools.module.resolve( 'wTesting' ) ).module, { it : { verbosity : 2, recursive : 4 } } ).resultExportString() );

  /* */

  test.case = 'Array.s v:implicit';
  var modulePath = baseAbs( 'l0/l5/Array.s' );
  var moduleFile = _.module.fileWith( modulePath );
  var got = _.module.fileExportString( moduleFile ).resultExportString();
  var exp = `{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}`;
  test.identical( got, exp );

  /* */

  test.case = 'Array.s v:1';
  var modulePath = baseAbs( 'l0/l5/Array.s' );
  var moduleFile = _.module.fileWith( modulePath );
  var got = _.module.fileExportString( moduleFile, { it : { verbosity : 1 } } ).resultExportString();
  var exp = `{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}`;
  test.identical( got, exp );

  /* */

  test.case = 'Array.s v:2';
  var modulePath = baseAbs( 'l0/l5/Array.s' );
  var moduleFile = _.module.fileWith( modulePath );
  var got = _.module.fileExportString( moduleFile, { it : { verbosity : 2 } } ).resultExportString();
  var exp =
`{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}
  global : real
  modules
    {- Module wTools -}
  downFiles
    {- ModuleFile ${baseAbs( 'l0/Include5.s' )} -}`;
  test.identical( got, exp );

  /* */

  test.case = 'Array.s v:3';
  var modulePath = baseAbs( 'l0/l5/Array.s' );
  var moduleFile = _.module.fileWith( modulePath );
  var got = _.module.fileExportString( moduleFile, { it : { verbosity : 3 } } ).resultExportString();
  var exp =
`{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}
  global : real
  modules
    {- Module wTools -}
  downFiles
    {- ModuleFile ${baseAbs( 'l0/Include5.s' )} -}`;
  test.identical( got, exp );

  /* */

}

//

function fileExportExternal( test )
{
  let a = test.assetFor( false );
  let program1 = a.program( r1 );
  let program2 = a.program( r2 );
  let program3 = a.program( r3 );
  let program4a = a.program( r4a );
  let program4b = a.program( r4b );
  let program5 = a.program( r5 );
  let verbosityProgram = a.program( verbosityRoutine );
  let basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
  let baseAbs = __.routine.join( __.path, __.path.join, [ basePath ] );

  /* */

  verbosityProgram.start()
  .then( ( op ) =>
  {
    test.case = 'recursive:1';
    test.identical( op.exitCode, 0 );
    var exp =
`
== ${ a.abs( 'verbosityRoutine' )}

= v:undefined f:verbosityRoutine
{- ModuleFile ${ a.abs( 'verbosityRoutine' )} -}
= v:0 f:verbosityRoutine

= v:1 f:verbosityRoutine
{- ModuleFile ${ a.abs( 'verbosityRoutine' )} -}
= v:2 f:verbosityRoutine
{- ModuleFile ${ a.abs( 'verbosityRoutine' )} -}
  global : real
  upFiles
    {- ModuleFile ${ _.module.toolsPathGet() } -}
= v:3 f:verbosityRoutine
{- ModuleFile ${ a.abs( 'verbosityRoutine' )} -}
  global : real
  upFiles
    {- ModuleFile ${ _.module.toolsPathGet() } -}

== ${baseAbs( 'l0/l5/Array.s' )}

= v:undefined f:Array.s
{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}
= v:0 f:Array.s

= v:1 f:Array.s
{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}
= v:2 f:Array.s
{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}
  global : real
  modules
    {- Module wTools -}
  downFiles
    {- ModuleFile ${baseAbs( 'l0/Include5.s' )} -}
= v:3 f:Array.s
{- ModuleFile ${baseAbs( 'l0/l5/Array.s' )} -}
  global : real
  modules
    {- Module wTools -}
  downFiles
    {- ModuleFile ${baseAbs( 'l0/Include5.s' )} -}
`
    test.identical( op.output, exp );
    return null;
  });
  /* */

  program1.start({ args : [ '1', '1' ] })
  .then( ( op ) =>
  {
    test.case = 'verbosity:1 recursive:1';
    test.identical( op.exitCode, 0 );
    var basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
    var exp = `{- ModuleFile ${a.abs( 'r3' )} -}
`;
    test.identical( op.output, exp );
    return null;
  });

  /* */

  program1.start({ args : [ '1', '2' ] })
  .then( ( op ) =>
  {
    test.case = 'verbosity:1 recursive:2';
    test.identical( op.exitCode, 0 );
    var basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
    var exp = `{- ModuleFile ${a.abs( 'r3' )} -}
  {- ModuleFile ${a.abs( 'r4a' )} -}
  {- ModuleFile ${a.abs( 'r4b' )} -}
`;
    test.identical( op.output, exp );
    return null;
  });

  /* */

  program1.start({ args : [ '1', '3' ] })
  .then( ( op ) =>
  {
    test.case = 'verbosity:1 recursive:3';
    test.identical( op.exitCode, 0 );
    var basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
    var exp = `{- ModuleFile ${a.abs( 'r3' )} -}
  {- ModuleFile ${a.abs( 'r4a' )} -}
  {- ModuleFile ${a.abs( 'r4b' )} -}
    {- ModuleFile ${a.abs( 'r5' )} -}
`;
    test.identical( op.output, exp );
    return null;
  });

  /* */

  program1.start({ args : [ '1', '4' ] })
  .then( ( op ) =>
  {
    test.case = 'verbosity:1 recursive:4';
    test.identical( op.exitCode, 0 );
    var basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
    var exp = `{- ModuleFile ${a.abs( 'r3' )} -}
  {- ModuleFile ${a.abs( 'r4a' )} -}
  {- ModuleFile ${a.abs( 'r4b' )} -}
    {- ModuleFile ${a.abs( 'r5' )} -}
`;
    test.identical( op.output, exp );
    return null;
  });

  /* */

  program1.start({ args : [ '2', '1' ] })
  .then( ( op ) =>
  {
    test.case = 'verbosity:2 recursive:1';
    test.identical( op.exitCode, 0 );
    var basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
    var exp = `{- ModuleFile ${a.abs( 'r3' )} -}
  global : test1
  downFiles
    {- ModuleFile ${a.abs( 'r2' )} -}
  upFiles
    {- ModuleFile ${a.abs( 'r4a' )} -}
    {- ModuleFile ${a.abs( 'r4b' )} -}
`;
    test.identical( op.output, exp );
    return null;
  });

  /* */

  program1.start({ args : [ '2', '2' ] })
  .then( ( op ) =>
  {
    test.case = 'verbosity:2 recursive:2';
    test.identical( op.exitCode, 0 );
    var basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
    var exp = `{- ModuleFile ${a.abs( 'r3' )} -}
  global : test1
  downFiles
    {- ModuleFile ${a.abs( 'r2' )} -}
  upFiles
    {- ModuleFile ${a.abs( 'r4a' )} -}
    {- ModuleFile ${a.abs( 'r4b' )} -}
  ups
    {- ModuleFile ${a.abs( 'r4a' )} -}
      global : test1
      downFiles
        {- ModuleFile ${a.abs( 'r3' )} -}
    {- ModuleFile ${a.abs( 'r4b' )} -}
      global : test1
      downFiles
        {- ModuleFile ${a.abs( 'r3' )} -}
      upFiles
        {- ModuleFile ${a.abs( 'r5' )} -}
`;
    test.identical( op.output, exp );
    return null;
  });

  /* */

  program1.start({ args : [ '2', '3' ] })
  .then( ( op ) =>
  {
    test.case = 'verbosity:2 recursive:3';
    test.identical( op.exitCode, 0 );
    var basePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase' );
    var exp = `{- ModuleFile ${a.abs( 'r3' )} -}
  global : test1
  downFiles
    {- ModuleFile ${a.abs( 'r2' )} -}
  upFiles
    {- ModuleFile ${a.abs( 'r4a' )} -}
    {- ModuleFile ${a.abs( 'r4b' )} -}
  ups
    {- ModuleFile ${a.abs( 'r4a' )} -}
      global : test1
      downFiles
        {- ModuleFile ${a.abs( 'r3' )} -}
    {- ModuleFile ${a.abs( 'r4b' )} -}
      global : test1
      downFiles
        {- ModuleFile ${a.abs( 'r3' )} -}
      upFiles
        {- ModuleFile ${a.abs( 'r5' )} -}
      ups
        {- ModuleFile ${a.abs( 'r5' )} -}
          global : test1
          downFiles
            {- ModuleFile ${a.abs( 'r4b' )} -}
`;
    test.identical( op.output, exp );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function r1()
  {
    const _ = require( toolsPath );
    require( './r2' );
  }

  function r2()
  {
    const _ = _global_.wTools;
    const _global = _.global.makeAndOpen( module, 'test1' );
    const __ = require( toolsPath );
    require( './r3' );
    _.global.close( 'test1' );
  }

  function r3()
  {
    require( './r4a' );
    require( './r4b' );
  }

  function r4a()
  {
  }

  function r4b()
  {
    require( './r5' );
  }

  function r5()
  {
    const _ = _globals_.real.wTools;
    let verbosity = Number( process.argv[ 2 ] );
    let recursive = Number( process.argv[ 3 ] );
    let moduleFile = _.module.fileUniversalFrom( module.parent.parent );
    console.log( _.module.fileExportString( moduleFile, { it : { verbosity, recursive } } ).resultExportString() );
  }

  function verbosityRoutine()
  {
    const _ = require( toolsPath );

    var moduleFile = _.module.fileUniversalFrom( module );
    console.log( `\n== ${moduleFile.sourcePath}\n` );
    log( moduleFile, undefined );
    log( moduleFile, 0 );
    log( moduleFile, 1 );
    log( moduleFile, 2 );
    log( moduleFile, 3 );

    var modulePath = _.path.normalize( _.module.toolsPathGet() + '/../../wtools/abase/l0/l5/Array.s' );
    var moduleFile = _.module.fileWith( modulePath );
    console.log( `\n== ${moduleFile.sourcePath}\n` );
    log( moduleFile, undefined );
    log( moduleFile, 0 );
    log( moduleFile, 1 );
    log( moduleFile, 2 );
    log( moduleFile, 3 );

    function log( moduleFile, verbosity )
    {
      let prefix = `v:${verbosity} f:${_.path.fullName( moduleFile.sourcePath )}`;
      if( verbosity !== undefined )
      console.log( `= ${prefix}\n${_.module.fileExportString( moduleFile, { it : { verbosity } } ).resultExportString()}` );
      else
      console.log( `= ${prefix}\n${_.module.fileExportString( moduleFile ).resultExportString()}` );
    }

  }

}

fileExportExternal.timeOut = 120000;
fileExportExternal.description =
`
- change of option verbosity change level of verbosity of the output
`;

//

function fileInvalidateBasic( test )
{
  let context = this;
  let a = test.assetFor( false );

  var data = { a : 1 };
  __.fileProvider.fileWrite({ filePath : a.abs( 'f.json' ), data, encoding : 'json' });
  var got = require( __.path.nativize( a.abs( 'f.json' ) ) );
  test.identical( got, data );

  var deleted = _.module.fileInvalidate( a.abs( 'f.json' ) );
  test.true( deleted );

  var data = { b : 2 };
  __.fileProvider.fileWrite({ filePath : a.abs( 'f.json' ), data, encoding : 'json' });
  var got = require( __.path.nativize( a.abs( 'f.json' ) ) );
  test.identical( got, data );

}

//

function fileInvalidateProgram( test )
{
  let context = this;
  let a = test.assetFor( false );
  let programAbsolute = a.program( programRoutineAbsolute );
  let programRelative = a.program( programRoutineRelative );
  let programNpm = a.program( programRoutineNpm );

  /* */

  programAbsolute.start()
  .then( ( op ) =>
  {
    test.case = 'absolute path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{ a: 1 }
deleted : true
{ b: 2 }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  programRelative.start()
  .then( ( op ) =>
  {
    test.case = 'relative path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{ a: 1 }
deleted : true
{ b: 2 }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  programNpm.start()
  .then( ( op ) =>
  {
    test.case = 'npm path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{ a: 1 }
deleted : true
{ b: 2 }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function programRoutineAbsolute()
  {
    let _ = require( toolsPath );
    _.include( 'wFiles' );

    var data = { a : 1 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/f.json', data, encoding : 'json' });
    var got = require( __dirname + '/f.json' );
    console.log( got )

    var deleted = _.module.fileInvalidate( __dirname + '/f.json' );
    console.log( `deleted : ${deleted}` );

    var data = { b : 2 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/f.json', data, encoding : 'json' });
    var got = require( __dirname + '/f.json' );
    console.log( got )

  }

  /* */

  function programRoutineRelative()
  {
    let _ = require( toolsPath );
    _.include( 'wFiles' );

    var data = { a : 1 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/f.json', data, encoding : 'json' });
    var got = require( './f.json' );
    console.log( got )

    var deleted = _.module.fileInvalidate( './f.json' );
    console.log( `deleted : ${deleted}` );

    var data = { b : 2 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/f.json', data, encoding : 'json' });
    var got = require( './f.json' );
    console.log( got )

  }

  /* */

  function programRoutineNpm()
  {
    let _ = require( toolsPath );
    _.include( 'wFiles' );

    var data = { a : 1 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/node_modules/f.json', data, encoding : 'json' });
    var got = require( 'f.json' );
    console.log( got )

    let deleted = _.module.fileInvalidate( 'f.json' );
    console.log( `deleted : ${deleted}` );

    var data = { b : 2 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/node_modules/f.json', data, encoding : 'json' });
    var got = require( 'f.json' );
    console.log( got )

  }

  /* */

}

//

function fileWith( test )
{
  let context = this;
  let a = test.assetFor( false );
  let programAbsolute = a.program( programRoutineAbsolute );
  let programRelative = a.program( programRoutineRelative );
  let programNpm = a.program( programRoutineNpm );

  /* */

  programAbsolute.start()
  .then( ( op ) =>
  {
    test.case = 'absolute path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{ a: 1 }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  programRelative.start()
  .then( ( op ) =>
  {
    test.case = 'relative path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{ a: 1 }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  programNpm.start()
  .then( ( op ) =>
  {
    test.case = 'npm path';
    test.identical( op.exitCode, 0 );
    var exp =
`
{ a: 1 }
`
    test.equivalent( op.output, exp );
    return null;
  });

  /* */

  return a.ready;

  /* */

  function programRoutineAbsolute()
  {
    let _ = require( toolsPath );
    _.include( 'wFiles' );

    var data = { a : 1 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/f.json', data, encoding : 'json' });
    var got = require( __dirname + '/f.json' );
    console.log( got )

  }

  /* */

  function programRoutineRelative()
  {
    let _ = require( toolsPath );
    _.include( 'wFiles' );

    var data = { a : 1 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/f.json', data, encoding : 'json' });
    var got = require( './f.json' );
    console.log( got )

  }

  /* */

  function programRoutineNpm()
  {
    let _ = require( toolsPath );
    _.include( 'wFiles' );

    var data = { a : 1 }
    _.fileProvider.fileWrite({ filePath : __dirname + '/node_modules/f.json', data, encoding : 'json' });
    var got = require( 'f.json' );
    console.log( got )

  }

  /* */

}

//

function requireModuleProcess( test )
{
  let context = this;
  let a = test.assetFor( false );
  let program = a.program( program1 );

  /* */

  a.shell( `npm add wprocess@gamma` )
  program.start()
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( _.strCount( op.output, 'nhandled' ), 0 );
    test.identical( _.strCount( op.output, 'error' ), 0 );
    return null;
  });

  return a.ready;

  /* */

  function program1()
  {
    require( 'wprocess' );
  }
}

//

function moduleBinProblem( test )
{
  let a = test.assetFor( false );

  // a.shell.predefined.sync = 1;
  // a.shell.predefined.ready = null;

  /* - */

  // a.ready.then( () =>
  // {
    a.fileProvider.dirMake( a.abs( '.' ) );
    a.shell({ execPath : 'git init', sync : 1 });
    require( 'wdeasync' );
    return test.true( true );
    return null;
  // });
  // a.ready.then( () =>
  // {
  //   // test.true( a.fileProvider.fileExists( a.abs( '.git' ) ) );
  //   return test.true( true );
  //   // return null;
  // });

  /* - */

  return a.ready;
}

moduleBinProblem.description =
`
Demonstrate problem 'Module did not self-register'
Fails only on njs v10
Maybe, the reason of the problem is loading native module from hard drive, not from cache
`

//

function requireElectronProblem( test )
{
  let a = test.assetFor( false );

  a.program({ entry : program1, filePath : a.abs( 'program.js' ) });

  let noSandbox = _.process.insideTestContainer() ? '' : ' --no-sandbox';
  let packagedPath =
  {
    'win32' : `dist/win-unpacked/test.exe${ noSandbox }`,
    'darwin' : `dist/mac/test.app/Contents/MacOS/test${ noSandbox }`,
    'linux' : `dist/linux-unpacked/test${ noSandbox }`,
  };

  let packageJson =
  {
    name : 'test',
    version : '0.0.1',
    main : 'program.js',
    scripts :
    {
      postinstall : "electron-builder build --publish never --dir true"
    },
    devDependencies :
    {
      'electron' : '8.2.5',
      'electron-builder': '22.6.0'
    }
  };

  a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageJson, encoding : 'json' })

  /* */

  a.shell( 'npm i' )
  a.shell({ execPath : `${packagedPath[ process.platform ]}` })
  // a.shell( `${packagedPath[ process.platform]} --inspect-brk`) /* Vova: use to debug in chrome*/
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.false( _.strHas( op.output, 'Assertion fails' ) )
    return null;
  })

  return a.ready;

  /* */

  function program1()
  {
    const _ = require( toolsPath );
    var electron = require( 'electron' );
    process.exit();
  }
}

requireElectronProblem.routineTimeOut = 120000;

//

function requireGulpImageMin( test )
{
  if( Config.interpreter === 'njs' )
  if( _.number.from( process.versions.node.substring( 0, 2 ) ) < 14 )
  return test.true( true );

  const a = test.assetFor( false );
  const program = a.program({ entry : program1, filePath : a.abs( 'program.js' ) });

  const packageJson =
  {
    name : 'test',
    version : '0.0.1',
    main : 'program.js',
    devDependencies :
    {
      'gulp-imagemin' : "=8.0.0"
    }
  };

  a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageJson, encoding : 'json' })

  /* */

  a.shell( 'npm i' );
  program.start();
  a.ready.then( ( op ) =>
  {
    console.log( "abc3" );
    test.identical( op.exitCode, 0 );
    test.false( _.strHas( op.output, 'Assertion fails' ) )
    return null;
  })

  return a.ready;

  /* */

  function program1()
  {
    const _ = require( toolsPath );
    // var imagemin = require( 'gulp-imagemin' );
    debugger;
    var imagemin = import( 'gulp-imagemin' );
    imagemin.then( function( m )
    {
      console.log( "gifsicle :", m.gifsicle.name );
    })
    console.log( imagemin );
    console.log( "abc2" );
    /* qqq : for Dmytro : why does not print?? */
  }

}

requireGulpImageMin.routineTimeOut = 120000;

//

// function requireProductionModuleProblem( test )
// {
//   const a = test.assetFor( false );
//
//   /* */
//
//   const packageJson = { dependencies : { 'wgittools' : 'stable' } };
//   a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageJson, encoding : 'json' })
//
//   const programInclude = a.path.nativize( a.program({ entry : program1, filePath : a.abs( 'program1.js' ) }).filePath );
//   const programRequire = a.path.nativize( a.program({ entry : program2, filePath : a.abs( 'program2.js' ) }).filePath );
//
//   /* - */
//
//   a.shell( 'npm i --production' );
//
//   /* - */
//
//   a.ready.then( () =>
//   {
//     test.open( 'with unlinked module directory' );
//     return null;
//   });
//
//   a.shell( `node ${ programInclude }` )
//   .then( ( op ) =>
//   {
//     test.case = 'require file by routine `include`';
//     test.identical( op.exitCode, 0 );
//     test.identical( op.output, 'Module `GitTools` succefully loaded.\n' );
//     return null;
//   });
//
//   /* */
//
//   a.shell( `node ${ programRequire }` )
//   .then( ( op ) =>
//   {
//     test.case = 'require file directly';
//     test.identical( op.exitCode, 0 );
//     test.identical( op.output, 'Module `GitTools` succefully loaded.\n' );
//     return null;
//   });
//
//   a.ready.then( () =>
//   {
//     test.close( 'with unlinked module directory' );
//     return null;
//   });
//
//   /* - */
//
//   a.ready.then( () =>
//   {
//     test.open( 'with linked module directory' );
//     return null;
//   });
//
//   a.ready.then( () =>
//   {
//       a.fileProvider.softLink
//       ({
//         dstPath : a.abs( 'node_modules/wgittools/node_modules/wgittools' ),
//         srcPath : `hd://${ a.abs( 'node_modules/wgittools' ) }`,
//         makingDirectory : 1,
//         rewritingDirs : 1,
//       });
//     return null;
//   });
//
//   a.shell( `node ${ programInclude }` )
//   .then( ( op ) =>
//   {
//     test.case = 'require file by routine `include`';
//     test.identical( op.exitCode, 0 );
//     test.identical( op.output, 'Module `GitTools` succefully loaded.\n' );
//     return null;
//   });
//
//   a.shell( `node ${ programRequire }` )
//   .then( ( op ) =>
//   {
//     test.case = 'require file directly';
//     test.identical( op.exitCode, 0 );
//     test.identical( op.output, 'Module `GitTools` succefully loaded.\n' );
//     return null;
//   });
//
//   a.ready.then( () =>
//   {
//     test.close( 'with module directory' );
//     return null;
//   });
//
//   /* - */
//
//   return a.ready;
//
//   /* */
//
//   function program1()
//   {
//     const _ = require( toolsPath );
//     _.include( 'wGitTools' );
//     console.log( 'Module `GitTools` succefully loaded.' );
//   }
//
//   /* */
//
//   function program2()
//   {
//     const _ = require( 'wgittools' );
//     console.log( 'Module `GitTools` succefully loaded.' );
//   }
// }

//

function requireMongooseAfterTestingProblem( test )
{
  let a = test.assetFor( false );

  let filePath = a.program({ entry : program1, filePath : a.abs( 'program.js' ) }).filePath;
  let packageJson =
  {
    dependencies : { 'mongoose' : '' },
    devDependencies : { 'wTesting' : 'latest' },
  };
  a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageJson, encoding : 'json' })

  /* */

  a.shell( 'npm i' );
  a.shell({ execPath : `node ${ filePath }` });
  a.ready.then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    test.identical( op.output, 'succefully loaded modules\n' );
    return null;
  });

  /* - */

  return a.ready;

  /* */

  function program1()
  {
    const _ = require( 'wTesting' );
    const Mongoose_ = require( 'mongoose' );
    console.log( 'succefully loaded modules' );
  }
}

requireMongooseAfterTestingProblem.experimental = 1;

//

function requireProductionModuleProblemWithGitTools( test )
{
  const a = test.assetFor( false );

  const packageJson = { dependencies : { 'wgittools' : 'stable' } };
  a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageJson, encoding : 'json' })
  const programRequire = a.path.nativize( a.program({ entry : program1, filePath : a.abs( 'program1.js' ) }).filePath );

  /* - */

  a.shell( 'npm i --production' );
  a.ready.then( () =>
  {
    const directories = a.find({ filePath : a.abs( 'node_modules/*/node_modules' ) });
    console.log( __.entity.exportStringNice( directories ) );
    const filesMap = Object.create( null );
    _.each( directories, ( filePath ) =>
    {
      filesMap[ filePath ] = a.find( a.abs( 'node_modules', filePath, './*' ) );
    });
    console.log( __.entity.exportStringNice( filesMap, { levels : 2 } ) );
    return null;
  });
  a.shell( `node ${ programRequire }` )
  .then( ( op ) =>
  {
    test.case = 'require file directly';
    test.identical( op.exitCode, 0 );
    test.identical( op.output, 'Module `GitTools` succefully loaded.\n' );
    return null;
  });

  /* - */

  return a.ready;

  /* */

  function program1()
  {
    const _ = require( 'wgittools' );
    console.log( 'Module `GitTools` succefully loaded.' );
  }
}

requireProductionModuleProblemWithGitTools.experimental = 1;

//

function requireProductionModuleProblemWithGitToolsAlternative( test )
{
  const a = test.assetFor( false );

  const packageJson = { dependencies : { 'wgittools' : 'stable' } };
  a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageJson, encoding : 'json' })
  const programRequire = a.path.nativize( a.program({ entry : program1, filePath : a.abs( 'program1.js' ) }).filePath );

  /* - */

  a.shell( 'npm i --production' );
  a.ready.then( () =>
  {
    const directories = a.find({ filePath : a.abs( 'node_modules/*/node_modules' ) });
    console.log( __.entity.exportStringNice( directories ) );
    const filesMap = Object.create( null );
    _.each( directories, ( filePath ) =>
    {
      filesMap[ filePath ] = a.find( a.abs( 'node_modules', filePath, './' ) );
    });
    console.log( __.entity.exportStringNice( filesMap, { levels : 2 } ) );
    return null;
  });
  a.shell( `node ${ programRequire }` )
  .then( ( op ) =>
  {
    test.case = 'require file directly';
    test.identical( op.exitCode, 0 );
    test.identical( op.output, 'Module `GitTools` succefully loaded.\n' );
    return null;
  });

  /* - */

  return a.ready;

  /* */

  function program1()
  {
    require( 'wTools' );
    require( 'wgittools' );
    console.log( 'Module `GitTools` succefully loaded.' );
  }
}

requireProductionModuleProblemWithGitToolsAlternative.experimental = 1;

//

function requireProductionModuleProblemWithGdf( test )
{
  const a = test.assetFor( false );

  const packageJson = { dependencies : { 'wgdf' : 'stable' } };
  a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : packageJson, encoding : 'json' })
  const programRequire = a.path.nativize( a.program({ entry : program1, filePath : a.abs( 'program1.js' ) }).filePath );

  /* - */

  a.shell( 'npm i --production' );
  a.ready.then( () =>
  {
    const directories = a.find({ filePath : a.abs( 'node_modules/*/node_modules' ) });
    console.log( __.entity.exportStringNice( directories ) );
    const filesMap = Object.create( null );
    _.each( directories, ( filePath ) =>
    {
      filesMap[ filePath ] = a.find( a.abs( 'node_modules', filePath, './*' ) );
    });
    console.log( __.entity.exportStringNice( filesMap, { levels : 2 } ) );
    return null;
  });
  a.shell( `node ${ programRequire }` )
  .then( ( op ) =>
  {
    test.case = 'require file directly';
    test.identical( op.exitCode, 0 );
    test.identical( op.output, 'Module `GitTools` succefully loaded.\n' );
    return null;
  });

  /* - */

  return a.ready;

  /* */

  function program1()
  {
    const _ = require( 'wgdf' );
    console.log( 'Module `GitTools` succefully loaded.' );
  }
}

requireProductionModuleProblemWithGdf.experimental = 1;

// --
// test suite declaration
// --

const Proto =
{

  name : 'Tools.module.l0.l9.Basic',
  silencing : 1,
  routineTimeOut : 30000,

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

    modulePredeclareBasic,
    modulePredeclareBasic2,
    moduleExportsUndefined,
    resolveBasic,

    modulingNativeIncludeErrors,
    modulingSourcePathValid,
    modulingGlobalNamespaces,
    moduleRedeclare,
    preload,
    preloadIncludeModule,

    /* xxx : rename predeclare -> declare */
    predeclareBasic,
    predeclarePrime,
    predeclareRelative,
    predeclareAbsolute,
    predeclareRedeclaring,
    predeclareRedeclaringSharedFile,

    moduleIsIncluded,
    moduleResolveFromAnotherGlobal,
    programMakeOptionWithSubmodule,

    programInheritedModuleFilePaths,
    programLocalsChanging,
    programOptionLocalsRoutines,

    selfFindAssumption,
    localPathAssumption,
    globalPathAssumption,
    experiment,

    requireModuleFileWithAccessor,
    testingOnL1,
    l1Environment,
    l1SecondRequire,
    secondaryNamespaceSecondRequire,
    // requireSameModuleTwice, /* not a bug */
    nativeModuleFileDeleting,
    stealthyRequireIssue,

    fileExportBasic,
    fileExportExternal,
    fileInvalidateBasic,
    fileInvalidateProgram,

    // requireModuleProcess, /* not a bug */
    moduleBinProblem,

    requireElectronProblem,
    requireGulpImageMin,
    requireMongooseAfterTestingProblem, /* xxx : investigate */

    requireProductionModuleProblemWithGitTools,
    requireProductionModuleProblemWithGitToolsAlternative,
    requireProductionModuleProblemWithGdf,

  }

}

/*
xxx : test to include file which does not exist to reproduce problem of throwing assert:
_.assert( native === moduleFile.native );
*/

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
