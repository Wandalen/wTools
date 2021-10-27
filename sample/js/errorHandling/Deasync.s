program();

function program()
{

  const _ = require( 'wTools' );
  _.include( 'wProcedure' );
  _.include( 'wConsequence' );

  const con = _.Consequence();

  con.then( function callback1( arg )
  {
    console.log( 'sourcePath::callback1 ' + _.procedure.activeProcedure._sourcePath );
    return 'callback1';
  } )

  _.time.out( 100, function timeOut1()
  {
    console.log( 'sourcePath::timeout ' + _.procedure.activeProcedure._sourcePath );
    con.take( 'timeout1' );
  } );

  con.deasync();

  console.log( 'sourcePath::program ' + _.procedure.activeProcedure._sourcePath );
}

/*
  Deasync pause current procedure and resolve it only after consequence get a resource.
*/
