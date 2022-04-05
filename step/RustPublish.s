
var _ = require( 'wTools' );

_.include( 'wProcess' );
_.include( 'wFiles' );

let step = function rustPublish( o )
{
  o = _.routine.optionsWithUndefined( step, o || Object.create( null ) );

  const appArgs = _.process.input();
  _.process.inputReadTo
  ({
    dst : o,
    propertiesMap : appArgs.map,
    namesMap : _.map.keys( step.defaults ),
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
      start({ currentPath : o.modulesList[ i ], execPath : 'selector get ./Cargo.toml package.publish' });
      ready.then( ( op ) =>
      {
        if( op.output.trim() === 'false' )
        _.arrayBut_( o.modulesList, o.modulesList, i );
        return null
      });
    }
    return ready;
  });

  /* bump */
  con.then( () =>
  {
    /* qqq : primitive bump, can be improved */
    for( let i = 0; i < o.modulesList.length; i++ )
    ready.then( () => bump( o, i ) );
    return ready;
  });

  /* commit */
  /* alternatively, commit each package version */
  con.then( () =>
  {
    if( o.logger && o.logger.verbosity >= 3 );
    ready.then( () =>
    {
      console.log( 'Committing changes in rust packages.' );
      return null;
    });
    if( !o.dry )
    {
      start({ currentPath, execPath : `git commit -am "publish rust packages"` });
      start({ currentPath, execPath : `git push"` });
    }
    return ready;
  });

  /* publish */
  con.then( () =>
  {
    for( let i = 0; i < o.modulesList.length; i++ )
    {
      if( o.dry )
      start({ currentPath : o.modulesList[ i ], execPath : `cargo publish --dry-run` });
      else
      start({ currentPath : o.modulesList[ i ], execPath : `cargo publish` });
    }
    return ready;
  });

  return con;
}

let defaults = step.defaults = Object.create( null );
defaults.modulesList = null;
defaults.logger = 2;
defaults.dry = 0;

//

function filesFind( o2 )
{
  o2.outputFormat = 'absolute';
  o2.mode = 'distinct';
  o2.withDirs = true;
  o2.withTerminals = false;
  o2.withStem = false;
  let files = _.fileProvider.filesFind( o2 );
  return files;
}

//

function bump( o, i )
{
  const start = _.process.starter
  ({
    currentPath : o.modulesList[ i ],
    outputCollecting : 1,
    outputPiping : o.logger ? o.logger.verbosity >= 3 : 0,
    inputMirroring : o.logger ? o.logger.verbosity >= 3 : 0,
    verbosity : o.logger ? o.logger.verbosity : 0,
    logger : o.logger,
    mode : 'shell',
  });

  const con = _.take( null );
  let appliedVersion = '0.0.0';
  con.then( () =>
  {
    /* qqq : toml reader is required */
    return start( 'selector get ./Cargo.toml package.version -f yaml' )
    .then( ( op ) =>
    {
      const splits = op.output.split( '.' );
      splits[ 2 ] = Number( splits[ 2 ] ) + 1;
      appliedVersion = splits.join( '.' );
      return null;
    });
  });
  con.then( () =>
  {
    /* qqq : toml writer is required */
    const ready = start( `selector set ./Cargo.toml package.version ${ appliedVersion }` );
    if( o.dry )
    return null;
    return ready.then( ( op ) =>
    {
      const data = op.output;
      const packagePath = _.path.join( o.modulesList[ i ], 'Cargo.toml' );
      _.fileProvider.fileWrite( packagePath, data );
      return null;
    });
  });

  return con;
}

module.exports = step;
if( !module.parent )
step();

