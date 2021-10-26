if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

// returns [ 'sample,string', undefined, '' ]
let s1 = _.strIsolateLeftOrAll( { src : 'sample,string', delimeter : [ '+' ] } );
logger.log( s1 )
// returns [ 'sample ,', 'st', 'ring' ]
let s2 = _.strIsolateLeftOrAll( { src : 'sample , string', delimeter : ' st' } )
logger.log( s2 )
// returns [ 'sample', ',', ' string,name string' ]
let s3 = _.strIsolateLeftOrAll( 'sample, string,name string', ',' )
logger.log( s3 )
// returns [ 'sample', ',', 'string' ]
let s4 = _.strIsolateRightOrAll( { src : 'sample,string', delimeter : [ ',' ] } );
logger.log( s4 )
// returns [ 'sample , s', 't', 'ring' ]
let s5 = _.strIsolateRightOrAll( { src : 'sample , string', delimeter : [ 's', 't' ] } )
logger.log( s5 )
// returns [ '', undefined, 'sample, string,name string' ]
let s6 = _.strIsolateRightOrAll( 'sample, string,name string', 'x' )
logger.log( s6 )
