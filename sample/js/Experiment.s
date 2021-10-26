
if( typeof module !== 'undefined' )
require( 'wTools' );

let _ = wTools;

function onStrip( part )
{
  var temp = part.split( ':' )
  if( temp.length === 2 )
  {
    return temp;
  }
  return undefined;
}

var src = 'this #background:red#is#background:default# text1 and # text2 # and text3';
var result = _.ct.strip( src, { onStrip } );

var src1 = 'this #background:red#is#background:default# #text1';
var result2 = _.ct.strip( src1, { onStrip } );

console.log( result );
console.log( result2 );
