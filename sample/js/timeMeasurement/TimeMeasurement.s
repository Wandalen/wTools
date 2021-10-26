let _ = require( 'wTools' );
let times = 10;
let size = 50000000;
let array = new U8x( size );

var result;
var counter = 0;
var time = _.time.now();
for( let i = times ; i > 0; i-- )
result = forLoop( array, () => counter += 1 );
console.log( `For loop took ${_.time.spent( time )} on Njs ${process.version}` );
console.info( `Output ${counter} to avoid unwanted optimization` );

var counter = 0;
var time = _.time.now();
for( let i = times ; i > 0; i-- )
result = forEach( array, () => counter += 1 );
console.log( `For each took ${_.time.spent( time )} on Njs ${process.version}` );
console.info( `Output ${counter} to avoid unwanted optimization` );

function forLoop( src, onEach )
{
  for( let k = 0 ; k < src.length ; k++ )
  onEach( src[ k ], k, src );
  return src
}

function forEach( src, onEach )
{
  src.forEach( ( e, k, src ) => onEach( e, k, src ) );
  return src;
}

