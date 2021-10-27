
let _ = require( 'wTools' );
_.include( 'wFiles' );

program();

function program()
{

  _.time.out( 100, function timeOut1()
  {
    console.log( 'sourcePath::timeout ' + _.procedure.activeProcedure._sourcePath );
    throw _.error.err( 'timeOut1' );
  } );

  console.log( 'sourcePath::program ' + _.procedure.activeProcedure._sourcePath );
}

/*
  In case of time out native async stack has impurities what decreases its readability.
  Synthetic async stack is much easier to read.
*/
