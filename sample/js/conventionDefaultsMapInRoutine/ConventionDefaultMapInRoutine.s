let _ = require( 'wTools' );

//

function nameFull( path )
{
  _.assert( arguments.length === 1 );
  _.assert( _.strIs( path ), 'Expects strings {-path-}' );

  let o = { path };
  _.routine.options( nameFull, o );

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

nameFull.defaults =
{
  path : null,
  full : 1,
}

//

console.log( nameFull( '/foo/bar/baz.js' ) );

//

function name( o )
{
  _.assert( arguments.length === 1 );
  _.assert( _.mapIs( o ), 'Expects map' );

  _.routine.options( name, o );

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

name.defaults =
{
  path : null,
  full : 0,
}

//

console.log( name( { path : '/foo/bar/baz.js' } ) );
console.log( name( { path : '/foo/bar/baz.js', full : 1 } ) );

//

let o = { own : 1 };

var a = { a : 1 }
var b = { b : 2 }
Object.setPrototypeOf( a, b );

console.log( _.mapKeys( a ) );
console.log( _.mapKeys.call( o, a ) );
