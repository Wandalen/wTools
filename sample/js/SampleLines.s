
if( typeof module !== 'undefined' )
require( 'wTools' );

let _ = wTools;

var srcStr = `\na\nbc\ndef\nghij\n\n`;


console.log( 'Input is a string' )
console.log( 'GOT', [ _.strLinesStrip( '  Hello \r\n\t World \n\n  ! \n\n' ) ] );
console.log( 'Input is an Array' )
console.log( 'GOT', _.strLinesStrip( [ '  Hello \r\n\t World \n\n  ! \n\n' ] ) );

let str = '\n\tHello  \r\n\t\t\r\n World \t\r\n! \r\n\t\nHow are you?  ';
let arrStr = str.split( '\n' );
console.log( arrStr )
console.log( _.strLinesStrip( str ).split( '\n' ) );
console.log( _.strLinesStrip( arrStr ) );
