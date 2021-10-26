let _ = require( 'wTools' );

//

function name_pre( routine, args )
{
  let o = args[ 0 ];
  if( _.strIs( o ) )
  o = { path : o };

  _.routine.options( routine, o );
  _.assert( o && _.strIs( o.path ), 'Expects strings {-o.path-}' );
  _.assert( args.length === 1 );
  _.assert( arguments.length === 2 );

  return o;
}

function name_body( o )
{
  let i = o.path.lastIndexOf( '/' );
  if( i !== -1 )
  o.path = o.path.substr( i+1 );

  if( !o.full )
  {
    let i = o.path.lastIndexOf( '.' );
    if( i !== -1 ) o.path = o.path.substr( 0, i );
  }

  return o.path;
}

name_body.defaults =
{
  path : null,
  full : 0,
}

let name = _.routine.unite( name_pre, name_body );

let nameFull = _.routine.unite( name_pre, name_body );
nameFull.defaults.full = 1;

console.log( name( './a/b/c/File.js' ) );
console.log( nameFull( './a/b/c/File.js' ) );
