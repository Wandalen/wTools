if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

// Oficated routine

function mapToStr( o )
{
  if( _.strIs( o ) )
  o = { src : o }
  _.routine.options( mapToStr, o );
  let result = '';
  for( let s in o.src )
  result += s + o.keyValDelimeter + o.src[ s ] + o.entryDelimeter;
  result = result.substr( 0, result.length-o.entryDelimeter.length );
  return result
}

mapToStr.defaults =
{
  src : null,
  keyValDelimeter : ':',
  entryDelimeter : ';',
}

console.log( mapToStr( { src : { a : 1, b : 2, c : 3 } } ) ) ;
// log : a:1;b:2;c:3
