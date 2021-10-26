function someFunction1( optionsMap )
{
  if( optionsMap.option1 >= 0 )
  console.log( 'option1' );

  if( optionsMap.option2 < 0 )
  console.log( 'option2' );
}
someFunction1( { option1 : 1, option2 : 2 } );
/* log : option1 */

