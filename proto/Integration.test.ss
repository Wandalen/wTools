( function _Integration_test_ss_()
{

'use strict';

//

if( typeof module !== 'undefined' )
{
  const _ = require( 'wTools' );
  _.include( 'wTesting' );
}

//

const _ = _global_.wTools;
const __ = _globals_.testing.wTools;
const fileProvider = __.fileProvider;
const path = fileProvider.path;

// --
// context
// --

function onSuiteBegin( test )
{
  let context = this;
  context.provider = fileProvider;
  let path = context.provider.path;
  context.suiteTempPath = context.provider.path.tempOpen( path.join( __dirname, '../..' ), 'integration' );
}

//

function onSuiteEnd( test )
{
  let context = this;
  let path = context.provider.path;
  __.assert( __.strHas( context.suiteTempPath, 'integration' ), context.suiteTempPath );
  path.tempClose( context.suiteTempPath );
}

// --
// test
// --

function _production_functor( command )
{
  return function _production( test )
  {
    let context = this;
    let a = test.assetFor( 'production' );
    let runList = [];

    let mdlPath = a.abs( __dirname, '../package.json' );
    let mdl = a.fileProvider.fileRead({ filePath : mdlPath, encoding : 'json' });
    let moduleLocalPath = a.abs( __dirname, '..' );
    let trigger = __.test.workflowTriggerGet( moduleLocalPath );

    if( mdl.private || trigger === 'pull_request' )
    {
      test.true( true );
      return;
    }

    /* delay to let npm get updated */
    if( trigger === 'publish' )
    a.ready.delay( 60000 );

    console.log( `Event : ${trigger}` );
    console.log( `Env :\n${__.entity.exportString( environmentsGet() )}` );

    /* */

    let sampleDir = a.abs( __dirname, '../sample/trivial' );
    let samplePath = a.find
    ({
      filePath : sampleDir,
      filter : { filePath : { 'Sample.(s|js|ss)' : 1 } }
    });

    if( !samplePath.length )
    throw _.err( `Sample with name "Sample.(s|ss|js)" does not exist in directory ${ sampleDir }` );

    /* */

    a.fileProvider.filesReflect({ reflectMap : { [ sampleDir ] : a.abs( 'sample/trivial' ) } });

    let remotePath = null;
    if( __.git.insideRepository( moduleLocalPath ) )
    remotePath = __.git.remotePathFromLocal( moduleLocalPath );

    let isFork = false;
    let mdlRepoParsed, remotePathParsed;
    if( remotePath )
    {
      mdlRepoParsed = __.git.path.parse( mdl.repository.url );
      remotePathParsed = __.git.path.parse( remotePath );
      isFork = mdlRepoParsed.user !== remotePathParsed.user || mdlRepoParsed.repo !== remotePathParsed.repo;
    }

    let version = versionGet( isFork, remotePath );
    if( !version )
    throw _.err( 'Cannot obtain version to install' );

    let structure = { dependencies : { [ mdl.name ] : version } };
    a.fileProvider.fileWrite({ filePath : a.abs( 'package.json' ), data : structure, encoding : 'json' });
    let data = a.fileProvider.fileRead({ filePath : a.abs( 'package.json' ) });
    console.log( data );

    let moduleDir = __.path.join( a.routinePath, 'node_modules', mdl.name );

    /* */

    a.ready.then( () =>
    {
      return __.retry
      ({
        routine : () => a.shell({ execPath : command, timeOut : 120000, ready : null }),
        onError,
      });
    });
    a.ready.then( ( op ) =>
    {
      test.case = 'install module';
      test.identical( op.exitCode, 0 );

      if( _.array.is( command ) && isFork )
      return null;

      test.case = 'no test files';
      let testFiles = a.fileProvider.filesFind({ filePath : __.path.join( moduleDir, '**.test*' ), outputFormat : 'relative' });
      test.identical( testFiles, [] );
      return null;
    });
    // a.shell( `npm i --production` )
    // .catch( handleDownloadingError )
    // .then( ( op ) =>
    // {
    //   test.case = 'install module';
    //   test.identical( op.exitCode, 0 );
    //
    //   test.case = 'no test files';
    //   let testFiles = a.fileProvider.filesFind({ filePath : __.path.join( moduleDir, '**.test*' ), outputFormat : 'relative' });
    //   test.identical( testFiles, [] );
    //   return null;
    // });

    if( isFork && !_.array.is( command ) )
    a.shell({ execPath : `will .npm.install`, currentPath : moduleDir });

    run( 'Sample.s' );
    run( 'Sample.ss' );

    /* */

    return a.ready;

    /* */

    function environmentsGet()
    {
      /* object process.env is not an auxiliary element ( new implemented check ) */
      return __.filter_( null, __.props.extend( null, process.env ), ( element, key ) => /* xxx */
      {
        if( __.strBegins( key, 'PRIVATE_' ) )
        return;
        if( key === 'NODE_PRE_GYP_GITHUB_TOKEN' )
        return;
        return key;
      });
    }

    /* */

    function run( name )
    {
      let filePath = `sample/trivial/${ name }`;
      if( !a.fileProvider.fileExists( a.abs( filePath ) ) )
      return null;
      runList.push( filePath );
      a.shell
      ({
        execPath : `node ${ filePath }`,
        throwingExitCode : 0
      })
      .then( ( op ) =>
      {
        test.case = `running of sample ${filePath}`;
        test.identical( op.exitCode, 0 );
        test.ge( op.output.length, 3 );

        if( op.exitCode === 0 || !isFork )
        return null;

        test.case = 'fork is up to date with origin'
        return __.git.isUpToDate
        ({
          localPath : moduleLocalPath,
          remotePath : __.git.path.normalize( mdl.repository.url )
        })
        .then( ( isUpToDate ) =>
        {
          test.identical( isUpToDate, true );
          return null;
        })
      });
    }

    /* */

    function versionGet( isFork )
    {
      if( isFork )
      return __.git.path.nativize( remotePath );

      let devDependencies = __.npm.fileReadField
      ({
        localPath : __.npm.pathLocalFromInside( __dirname ),
        key : 'devDependencies'
      });
      if( devDependencies && devDependencies.wTesting && isNaN( devDependencies.wTesting[ 0 ] ) )
      return devDependencies.wTesting;

      return mdl.version;
    }

    /* */

    function onError( err )
    {
      const errMsg = err.originalMessage;
      if( _.str.has( errMsg, /[Nn]ot found/ ) || _.str.has( errMsg, 'SIGTERM' ) )
      {
        _.error.attend( err );
        return true;
      }
      return false;
    }
  }

  _production.timeOut = 600000;
}

//

const production = _production_functor( `npm i --production` );

//

const productionYarn = _production_functor([ `npm i -g yarn`, `yarn install --production` ])
productionYarn.rapidity = -4;

//

function samples( test )
{
  let context = this;
  let ready = __.take( null );

  let sampleDir = path.join( __dirname, '../sample' );

  let start = __.process.starter
  ({
    currentPath : sampleDir,
    outputCollecting : 1,
    outputGraying : 1,
    throwingExitCode : 0,
    ready,
    mode : 'fork'
  })

  let found = fileProvider.filesFind
  ({
    filePath : path.join( sampleDir, '**/*.(s|ss)' ),
    withStem : 0,
    withDirs : 0,
    filter :
    {
      maskTransientDirectory : { excludeAny : [ /asset/, /out/ ] }
    },
    mode : 'distinct',
    mandatory : 0,
  });

  /* */

  for( let i = 0 ; i < found.length ; i++ )
  {
    if( __.longHasAny( found[ i ].exts, [ 'browser', 'manual', 'experiment' ] ) )
    continue;

    let startTime;

    ready
    .then( () =>
    {
      test.case = found[ i ].relative;
      startTime = __.time.now();
      return null;
    })

    if( __.longHas( found[ i ].exts, 'throwing' ) )
    {
      start({ execPath : found[ i ].relative, outputPiping : 0 })
      .then( ( op ) =>
      {
        console.log( __.time.spent( startTime ) );
        test.description = 'nonzero exit code';
        test.notIdentical( op.exitCode, 0 );
        return null;
      })
    }
    else
    {
      start({ execPath : found[ i ].relative })
      .then( ( op ) =>
      {
        console.log( __.time.spent( startTime ) );
        test.description = 'good exit code';
        test.identical( op.exitCode, 0 );
        if( op.exitCode )
        return null;
        test.description = 'have no uncaught errors';
        test.identical( __.strCount( op.output, 'ncaught' ), 0 );
        test.identical( __.strCount( op.output, 'uncaught error' ), 0 );
        test.description = 'have some output';
        test.ge( op.output.split( '\n' ).length, 1 );
        test.ge( op.output.length, 3 );
        return null;
      })
    }
  }

  /* */

  return ready;
}

samples.rapidity = -1;

//

function eslint( test )
{
  let context = this;
  let rootPath = path.join( __dirname, '..' );
  let eslint = process.platform === 'win32' ? 'node_modules/eslint/bin/eslint' : 'node_modules/.bin/eslint';
  eslint = path.join( rootPath, eslint );
  let sampleDir = path.join( rootPath, 'sample' );
  let ready = __.take( null );

  if( __.process.insideTestContainer() )
  {
    let validPlatform = process.platform === 'linux';
    let validVersion = process.versions.node.split( '.' )[ 0 ] === '14';
    if( !validPlatform || !validVersion )
    {
      test.true( true );
      return;
    }
  }

  let start = __.process.starter
  ({
    execPath : eslint,
    mode : 'fork',
    currentPath : rootPath,
    args :
    [
      '-c', '.eslintrc.yml',
      '--ext', '.js,.s,.ss',
      '--ignore-pattern', '*.c',
      '--ignore-pattern', '*.ts',
      '--ignore-pattern', '*.html',
      '--ignore-pattern', '*.txt',
      '--ignore-pattern', '*.png',
      '--ignore-pattern', '*.json',
      '--ignore-pattern', '*.yml',
      '--ignore-pattern', '*.yaml',
      '--ignore-pattern', '*.md',
      '--ignore-pattern', '*.xml',
      '--ignore-pattern', '*.css',
      '--ignore-pattern', '_asset',
      '--ignore-pattern', '__*',
      '--ignore-pattern', 'out',
      '--ignore-pattern', '*.tgs',
      '--ignore-pattern', '*.bat',
      '--ignore-pattern', '*.sh',
      '--ignore-pattern', '*.jslike',
      '--ignore-pattern', '*.less',
      '--ignore-pattern', '*.hbs',
      '--ignore-pattern', '*.noeslint',
      '--quiet'
    ],
    throwingExitCode : 0,
    outputCollecting : 1,
  })

  /* */

  ready.then( () =>
  {
    test.case = 'eslint proto';
    return start( 'proto/**' );
  })
  .then( ( op ) =>
  {
    test.identical( op.exitCode, 0 );
    if( op.output.length < 1000 )
    logger.log( op.output );
    return null;
  })

  /* */

  if( fileProvider.fileExists( sampleDir ) )
  ready.then( () =>
  {
    test.case = 'eslint samples';
    return start( 'sample/**' )
    .then( ( op ) =>
    {
      test.identical( op.exitCode, 0 );
      if( op.output.length < 1000 )
      logger.log( op.output );
      return null;
    })
  })

  /**/

  return ready;
}

eslint.rapidity = -2;

//

function build( test )
{
  let context = this;
  let a = test.assetFor( false );

  let mdlPath = a.abs( __dirname, '../package.json' );
  let mdl = a.fileProvider.fileRead({ filePath : mdlPath, encoding : 'json' });

  if( !mdl.scripts.build )
  {
    test.true( true );
    return;
  }

  let remotePath = __.git.remotePathFromLocal( a.abs( __dirname, '..' ) );

  let ready = __.git.repositoryClone
  ({
    remotePath,
    localPath : a.routinePath,
    logger : 2,
    sync : 0
  })

  __.process.start
  ({
    execPath : 'npm run build',
    currentPath : a.routinePath,
    throwingExitCode : 0,
    mode : 'shell',
    outputPiping : 1,
    ready,
  })

  ready.then( ( got ) =>
  {
    test.identical( got.exitCode, 0 );
    return null;
  })

  return ready;
}

build.rapidity = -1;
build.timeOut = 900000;

// --
// declare
// --

const Proto =
{

  name : 'Integration',
  routineTimeOut : 1500000,
  silencing : 0,

  onSuiteBegin,
  onSuiteEnd,
  context :
  {
    provider : null,
    suiteTempPath : null,
    appJsPath : null
  },

  tests :
  {
    production,
    // productionYarn,
    samples,
    eslint,
    build,
  },

}

//

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
_global_.wTester.test( Self.name );

})();
