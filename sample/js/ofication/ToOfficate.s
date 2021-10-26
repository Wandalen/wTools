
function mapToStr( src, keyValDelimeter, entryDelimeter )
{
  let result = '';
  for( let s in src )
  result += s + keyValDelimeter + src[ s ] + entryDelimeter;
  result = result.substr( 0, result.length-entryDelimeter.length );
  return result
}

console.log( mapToStr( { a : 1, b : 2, c : 3 }, ':', ';' ) );
// log : a:1;b:2;c:3
