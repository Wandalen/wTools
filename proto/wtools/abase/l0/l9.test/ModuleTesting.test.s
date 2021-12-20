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

function secondaryNamespaceOfTesting( test )
{
  let a = test.assetFor( false );
  let program = a.program({ entry : r1 });

  act({});

  return a.ready;

  /* - */

  function act( env )
  {

    /* */

    program.start()
    .then( ( op ) =>
    {
      test.case = `basic, ${__.entity.exportStringSolo( env )}`;
      test.identical( op.exitCode, 0 );
      var exp =
`
{- ModuleFile ${ _.module.resolve( 'wTesting' ) } -}
  {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/entry/Main.s' ) } -}
    {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/include/Top.s' ) } -}
      {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/abase/l0/l0/l0/Global.s' ) } -}
      {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/include/Base.s' ) } -}
      {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/l1/Namespace.s' ) } -}
      {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/l3/TesterBasic.s' ) } -}
      {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/l5/Routine.s' ) } -}
      {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/l5/Suite.s' ) } -}
      {- ModuleFile ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/l7/TesterTop.s' ) } -}
Top.ups.length : 7
Top.children.length : 7
`
      test.equivalent( op.output, exp );
      return null;
    });

    /* */

  }

  function r1()
  {
    const _ = require( toolsPath );
    _.include( 'wTesting' );
    let testingPath = _.module.resolve( 'wTesting' );
    let moduleFile = _.module.fileWith( testingPath );
    console.log( _.module.fileExportString( moduleFile, { it : { verbosity : 1, recursive : 4 } } ).resultExportString() );
    console.log( 'Top.ups.length : ' + _.module.fileWith( testingPath + '/../../wtools/atop/testing/include/Top.s' ).upFiles.size );
    console.log( 'Top.children.length : ' + _.module.fileWith( testingPath + '/../../wtools/atop/testing/include/Top.s' ).native.children.length );
  }

}

secondaryNamespaceOfTesting.description =
`
- wtools/atop/testing/include/Top.s should include wtools/atop/testing/include/Base.s and other children
`

//

function modulingLogistic( test )
{
  let context = this;
  let a = test.assetFor( false );
  let ready = __.take( null );

  var toolsPath = _.module.resolve( 'wTools' );
  var toolsDirPath = _.module.toolsDirGet();
  var testingPath = _.module.resolve( 'wTesting' );
  var moduleFile = _.module.fileWithResolvedPath( testingPath );
  test.true( _.module.fileIs( moduleFile ) );
  test.true( _.module.fileUniversalIs( moduleFile ) );
  test.true( !_.module.fileNativeIs( moduleFile ) );
  test.true( _.module.fileIs( moduleFile.native ) );
  test.true( !_.module.fileUniversalIs( moduleFile.native ) );
  test.true( _.module.fileNativeIs( moduleFile.native ) );
  test.identical( moduleFile.sourcePath, testingPath );
  test.true( _.module.is( moduleFile.module ) );

  var module = _.module.withPath( testingPath );
  test.ge( _.entity.lengthOf( module.files ), 1 );
  test.identical( _.entity.lengthOf( module.alias ), 2 );
  var exp = new Set
  ([
    'proto/wtools/atop/testing/entry/Basic.s',
    'proto/wtools/atop/testing/include/Top.s',
    'proto/wtools/abase/l0/l0/l0/Global.s',
    'proto/node_modules/wTesting',
  ]);
  var files = __.select( [ ... module.files.values() ], '*/sourcePath' );
  _.assert( files[ 0 ] !== undefined );
  test.identical( new Set( __.path.s.relative( testingPath + '/../../..', files ) ), exp ); /* yyy */
  test.true( new Set( __.path.s.relative( testingPath + '/../../..', files ) ).has( 'proto/node_modules/wTesting' ) );
  var module2 = _.module.withName( 'wTesting' );
  test.true( module === module2 );
  var module2 = _.module.withName( 'wtesting' );
  test.true( module === module2 );

  var module = _.module.withName( 'wTools' );
  test.gt( _.entity.lengthOf( module.files ), 100 );
  test.identical( _.entity.lengthOf( module.alias ), 2 );
  test.true( _.module.filesMap.has( toolsPath ) );
  test.true( module.files.has( toolsPath ) );
  test.true( _.module.filesMap.has( __.path.join( toolsDirPath, 'abase/l0/l0/l0/Global.s' ) ) );
  test.true( module.files.has( __.path.join( toolsDirPath, 'abase/l0/l0/l0/Global.s' ) ) );
  var module2 = _.module.withName( 'wTools' );
  test.true( module === module2 );
  var module2 = _.module.withName( 'wtools' );
  test.true( module === module2 );
  log( __.path.join( toolsDirPath, 'abase/l0/l0/l0/Global.s' ) );

  test.identical( _.entity.lengthOf( _.module._modulesToPredeclare ), 0 );
  test.ge( _.entity.lengthOf( _.module.modulesMap ), 4 );

  console.log( 'lengthOf( _modulesToPredeclare )', _.entity.lengthOf( _.module._modulesToPredeclare ) );
  console.log( 'lengthOf( predeclaredWithNameMap )', _.entity.lengthOf( _.module.predeclaredWithNameMap ) );
  console.log( 'lengthOf( predeclaredWithEntryPathMap )', _.entity.lengthOf( _.module.predeclaredWithEntryPathMap ) );
  console.log( 'lengthOf( modulesMap )', _.entity.lengthOf( _.module.modulesMap ) );
  console.log( 'lengthOf( filesMap )', _.entity.lengthOf( _.module.filesMap ) );
  var diff = _.arraySet.diff_( null, [ ... _.module.filesMap.keys() ], [ ... _.module.withName( 'wTools' ).files.keys() ] )
  console.log( `filesMap but tools.files\n  ${diff.join( '\n  ' )}` );

  act({});

  return ready;

  /* - */

  function act( env )
  {

    ready.then( () =>
    {
      test.case = `external program, ${__.entity.exportStringSolo( env )}`;

      var filePath/*programPath*/ = a.program( programRoutine1 ).filePath/*programPath*/;

      return a.forkNonThrowing
      ({
        execPath : filePath/*programPath*/,
        currentPath : _.path.dir( filePath/*programPath*/ ),
      })
    })
    .then( ( op ) =>
    {
      /*
      extra moddule file is program
      */
      var exp =
`
lengthOf( _modulesToPredeclare ) 0
lengthOf( predeclaredWithNameMap ) 2
lengthOf( predeclaredWithEntryPathMap ) 2
lengthOf( modulesMap ) ${_.entity.lengthOf( _.module.withName( 'wTools' ).alias )}
lengthOf( filesMap ) ${_.entity.lengthOf( _.module.withName( 'wTools' ).files ) + 1}
module.fileIs( moduleFile ) true
module.fileUniversalIs( moduleFile ) true
module.fileNativeIs( moduleFile ) false
module.fileIs( moduleFile.native ) true
module.fileUniversalIs( moduleFile.native ) false
module.fileNativeIs( moduleFile.native ) true
moduleFile.sourcePath ${testingPath}
moduleFile.downFile.sourcePath ${a.abs( 'programRoutine1' )}
moduleFile.downFile.module null
module.is( moduleFile.module ) true
filesOfTesting
  ${ __.path.join( _.module.resolve( 'wTesting' ), '.' ) }
  ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/entry/Basic.s' ) }
  ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/atop/testing/include/Top.s' ) }
  ${ __.path.join( _.module.resolve( 'wTesting' ), '../../wtools/abase/l0/l0/l0/Global.s' ) }
lengthOf( _modulesToPredeclare ) 0
lengthOf( predeclaredWithNameMap ) 4
lengthOf( predeclaredWithEntryPathMap ) 3
lengthOf( modulesMap ) ${_.entity.lengthOf( _.module.withName( 'wTools' ).alias ) + _.entity.lengthOf( _.module.withName( 'wTesting' ).alias )}
modulesMap wTools wTools wTesting wTesting
`

      test.identical( op.exitCode, 0 );
      test.equivalent( op.output, exp );
      return op;
    });

  }

  /* - */

  function programRoutine1()
  {
    const _ = require( toolsPath );
    let ModuleFileNative = require( 'module' );

    console.log( 'lengthOf( _modulesToPredeclare )', _.entity.lengthOf( _.module._modulesToPredeclare ) );
    console.log( 'lengthOf( predeclaredWithNameMap )', _.entity.lengthOf( _.module.predeclaredWithNameMap ) );
    console.log( 'lengthOf( predeclaredWithEntryPathMap )', _.entity.lengthOf( _.module.predeclaredWithEntryPathMap ) );
    console.log( 'lengthOf( modulesMap )', _.entity.lengthOf( _.module.modulesMap ) );
    console.log( 'lengthOf( filesMap )', _.entity.lengthOf( _.module.filesMap ) );

    debugger;
    let __ = _.include( 'wTesting' );

    var testingPath = _.module.resolve( 'wTesting' );
    var moduleFile = _.module.fileWithResolvedPath( testingPath );
    console.log( `module.fileIs( moduleFile )`, _.module.fileIs( moduleFile ) );
    console.log( `module.fileUniversalIs( moduleFile )`, _.module.fileUniversalIs( moduleFile ) );
    console.log( `module.fileNativeIs( moduleFile )`, _.module.fileNativeIs( moduleFile ) );
    console.log( `module.fileIs( moduleFile.native )`, _.module.fileIs( moduleFile.native ) );
    console.log( `module.fileUniversalIs( moduleFile.native )`, _.module.fileUniversalIs( moduleFile.native ) );
    console.log( `module.fileNativeIs( moduleFile.native )`, _.module.fileNativeIs( moduleFile.native ) );
    console.log( `moduleFile.sourcePath`, moduleFile.sourcePath );
    console.log( `moduleFile.downFile.sourcePath`, moduleFile.downFile.sourcePath );
    console.log( `moduleFile.downFile.module`, moduleFile.downFile.module );
    console.log( `module.is( moduleFile.module )`, _.module.is( moduleFile.module ) );

    var module = _.module.withPath( testingPath );
    var filesOfTesting = __.select( [ ... module.files.values() ], '*/sourcePath' );
    console.log( `filesOfTesting\n  ${filesOfTesting.join( '\n  ' )}` );

    console.log( 'lengthOf( _modulesToPredeclare )', _.entity.lengthOf( _.module._modulesToPredeclare ) );
    console.log( 'lengthOf( predeclaredWithNameMap )', _.entity.lengthOf( _.module.predeclaredWithNameMap ) );
    console.log( 'lengthOf( predeclaredWithEntryPathMap )', _.entity.lengthOf( _.module.predeclaredWithEntryPathMap ) );
    console.log( 'lengthOf( modulesMap )', _.entity.lengthOf( _.module.modulesMap ) );
    console.log( 'modulesMap', [ ... _.module.modulesMap.values() ].map( ( m ) => m.name ).join( ' ' ) );

  }

  /* - */

  function log( filePath )
  {
    let moduleFile = _.module.fileWith( filePath );
    if( !moduleFile )
    return console.log( `${filePath} : ${moduleFile}` );
    let output = _.module.fileExportString( moduleFile, { it : { verbosity : 2 } } ).resultExportString();
    output = _.strReplace( output, _.path.normalize( __dirname ), '.' );
    console.log( `${filePath} : ${output}` );
  }

  /* - */

}

modulingLogistic.description =
`
- relations between module files is sane
`

// --
// test suite declaration
// --

const Proto =
{

  name : 'Tools.module.l0.l9.Testing',
  silencing : 1,
  routineTimeOut : 30000,

  tests :
  {

    secondaryNamespaceOfTesting,
    modulingLogistic,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
