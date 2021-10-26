if( typeof module !== 'undefined' )
require( 'wTools' );

let _ = wTools;

function onStrip( strip )
{
  var allowedKeys = [ 'bg', 'background', 'fg', 'foreground' ];
  var parts = strip.split( ' : ' )
  if( parts.length === 2 )
  {
    if( allowedKeys.indexOf( parts[ 0 ] ) === -1 )
    return undefined;
    return parts;
  }
}

var src = 'this #background : red#is#background : default# text1 and # text2 # and text3';
var got = _.strSplitInlined({ src, onInlined : onStrip });
console.log( src );
// log : 'this #background : red#is#background : default# text1 and # text2 # and text3'
console.log( _.entity.exportString( got, { levels : 2 } ) );
// log : 'this ,background,red,is,background,default, text1 and # text2 # and text3'

var src = 'this #background : red#is#background : default# #text1';
var got = _.strSplitInlined({ src, onInlined : onStrip });
console.log( src );
// log : 'this #background : red#is#background : default# #text1'
console.log( _.entity.exportString( got, { levels : 2 } ) );
// log : 'this ,background,red,is,background,default, #text1'

var src = '#background : red#text#background : default#';
var got = _.strSplitInlined({ src, onInlined : onStrip });
console.log( src );
// log : '#background : red#text#background : default#'
console.log( _.entity.exportString( got, { levels : 2 } ) );
// log : ',background,red,text,background,default,'

var src = '#background : red#i#s#background : default##text';
var got = _.strSplitInlined({ src, onInlined : onStrip, preservingEmpty : 1 });
console.log( src );
// log : '#background : red#i#s#background : default##text'
console.log( _.entity.exportString( got, { levels : 2 } ) );
// log : ',background,red,i#s,background,default,#text'
