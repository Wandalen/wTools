use woptions::*;

Options!{ splitter< 'a >
{
  #![ derive( PartialOrd ) ]

  pub src : &'a str;
  pub delimeter : &'a str;
  #[ default( true ) ]
  pub left : bool;

  fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) >
  where
    Self : Sized,
  {
    if *self.left()
    {
      Box::new( self.src().split( self.delimeter() ) )
    }
    else
    {
      Box::new( self.src().rsplit( self.delimeter() ) )
    }
  }

}}

//

fn main()
{
  /* form options */
  let from_former = splitter().src( "abc" ).delimeter( "b" )._form();
  let from_options = splitter::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( from_former, from_options );

  /* perform methods from autotrait */
  use splitter::OptionsAdapter;
  let splitted = from_former.perform().map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( splitted, vec![ "a", "c" ] );
}
