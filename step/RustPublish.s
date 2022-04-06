
const _ = require( 'wTools' );
_.include( 'wHttp' );
_.include( 'wFiles' );
_.include( 'wProcess' );
const crypto = require( 'crypto' );

function rustPublish( o )
{
  o = _.routine.optionsWithUndefined( rustPublish, o || Object.create( null ) );

  const appArgs = _.process.input();
  _.process.inputReadTo
  ({
    dst : o,
    propertiesMap : appArgs.map,
    namesMap : _.map.keys( rustPublish.defaults ),
  });

  if( !o.modulesList )
  o.modulesList =
  [
    'module/rust/*',
    'module/rust_alias/*',
    'module/rust_move/*',
  ];

  const currentPath = _.path.current();
  for( let i = 0; i < o.modulesList.length; i++ )
  if( !_.path.isAbsolute( o.modulesList[ i ] ) )
  o.modulesList[ i ] = _.path.join( currentPath, o.modulesList[ i ] );

  _.assert( _.arrayIs( o.modulesList ), 'Expects modules list as array.' );

  for( let i = 0; i < o.modulesList.length; i++ )
  if( _.path.isGlob( o.modulesList[ i ] ) )
  {
    const paths = filesFind({ filePath : o.modulesList[ i ] });
    _.arrayBut_( o.modulesList, o.modulesList, i, paths );
  }

  const ready = _.take( null );
  o.logger = _.logger.maybe( o.logger );
  const start = _.process.starter
  ({
    outputCollecting : 1,
    outputPiping : o.logger ? o.logger.verbosity >= 3 : 0,
    inputMirroring : o.logger ? o.logger.verbosity >= 3 : 0,
    verbosity : o.logger ? o.logger.verbosity : 0,
    logger : o.logger,
    mode : 'shell',
    ready,
  });

  const con = _.take( null );
  con.then( () => start({ currentPath, execPath : 'cargo install wselector' }) );
  /* filter */
  con.then( () =>
  {
    for( let i = o.modulesList.length - 1; i >= 0; i-- )
    {
      /* qqq : toml reader is required */
      start({ currentPath : o.modulesList[ i ], execPath : 'selector get ./Cargo.toml package' });
      ready.then( ( op ) =>
      {
        const data = JSON.parse( op.output.trim() );
        if( data.publish === false )
        {
          _.arrayBut_( o.modulesList, o.modulesList, i );
        }
        else
        {
          data.localPath = o.modulesList[ i ];
          o.modulesList[ i ] = data;
          const name = o.modulesList[ i ].name;
          const version = o.modulesList[ i ].version;
          const packageCon = start({ currentPath : o.modulesList[ i ].localPath, execPath : 'cargo package', ready : null });
          const retrieveCon = _.http.retrieve
          ({
            uri : `https://static.crates.io/crates/${ name }/${ name }-${ version }.crate`,
          });
          return _.Consequence.And( packageCon, retrieveCon );
        }
        return null
      });
      ready.finally( ( err, cons ) =>
      {
        if( err )
        {
          if( _.strHas( err.originalMessage, 'Unexpected status code: 403' ) )
          _.error.attend( err );
          else
          throw _.error.brief( err );
        }
        if( cons )
        {
          const localPackageHash = crypto.createHash( 'sha1' );
          const remotePackageHash = crypto.createHash( 'sha1' );
          const packagePath = `target/package/${ o.modulesList[ i ].name }-${ o.modulesList[ i ].version }.crate`;
          const localPackageData = _.fileProvider.fileRead( _.path.join( currentPath, packagePath ) );
          localPackageHash.update( localPackageData );
          remotePackageHash.update( cons[ 1 ].response.body );

          if( localPackageHash.digest( 'hex' ) === remotePackageHash.digest( 'hex' ) )
          _.arrayBut_( o.modulesList, o.modulesList, i );
        }
        return null;
      });
    }
    return ready;
  });

  /* update and publish */
  con.then( () =>
  {
    for( let i = 0; i < o.modulesList.length; i++ )
    {
      /* bump */
      /* qqq : primitive bump, can be improved */
      ready.then( () => bump( o, i ) );

      /* commit */
      /* alternatively, commit each package version */
      if( o.logger && o.logger.verbosity >= 3 );
      ready.then( () =>
      {
        console.log( `Committing changes in package ${ o.modulesList[ i ].name }.` );
        return null;
      });
      if( !o.dry )
      {
        ready.then( () =>
        {
          return start
          ({
            currentPath,
            execPath : `git commit -am "${ o.modulesList[ i ].name } v${ o.modulesList[ i ].version }"`,
            ready : null,
          });
        });
        ready.then( () => start({ currentPath, execPath : `git push`, ready : null }) );
      }

      /* publish */
      if( o.dry )
      start({ currentPath : o.modulesList[ i ].localPath, execPath : `cargo publish --dry-run` });
      else
      start({ currentPath : o.modulesList[ i ].localPath, execPath : `cargo publish` });
    }
    return ready;
  });

  return con;
}

let defaults = rustPublish.defaults = Object.create( null );
defaults.modulesList = null;
defaults.logger = 2;
defaults.dry = 0;

//

function filesFind( o )
{
  o.outputFormat = 'absolute';
  o.mode = 'distinct';
  o.withDirs = true;
  o.withTerminals = false;
  o.withStem = false;
  let files = _.fileProvider.filesFind( o );
  return files;
}

//

function bump( o, i )
{
  const splits = o.modulesList[ i ].version.split( '.' );
  splits[ 2 ] = Number( splits[ 2 ] ) + 1;
  o.modulesList[ i ].version = splits.join( '.' );

  /* qqq : toml writer is required */
  const ready = _.process.start
  ({
    execPath : `selector set ./Cargo.toml package.version ${ o.modulesList[ i ].version }`,
    currentPath : o.modulesList[ i ].localPath,
    outputCollecting : 1,
    outputPiping : o.logger ? o.logger.verbosity >= 3 : 0,
    inputMirroring : o.logger ? o.logger.verbosity >= 3 : 0,
    verbosity : o.logger ? o.logger.verbosity : 0,
    logger : o.logger,
    mode : 'shell',
  });
  if( !o.dry )
  {
    ready.then( ( op ) =>
    {
      const data = op.output;
      const configPath = _.path.join( o.modulesList[ i ].localPath, 'Cargo.toml' );
      _.fileProvider.fileWrite( configPath, data );
      return null;
    });
  }

  return ready;
}

//

const step = rustPublish;
module.exports = step;
if( !module.parent )
step();

