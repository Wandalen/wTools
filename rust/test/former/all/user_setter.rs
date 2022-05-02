
use wtest_basic::*;

#[cfg( feature = "in_wtools" )]
use wtools::former::Former;
#[cfg( not( feature = "in_wtools" ) )]
use former::Former;

//

fn _user_setter()
{
  #[ derive( Former, PartialEq, Debug ) ]
  pub struct Book
  {
    #[ default( "" ) ]
    /* Dmytro : suggest to use attribute like: #[ setter( false ) ] */
    book_name : String,
    #[ default( "" ) ]
    url : std::path::PathBuf,
  }

  /* does not compile, duplicated method */
  // impl BookFormer
  // {
  //   /// Trivial example of user setter.
  //   pub fn url< T : AsRef< str > >( &mut self, url_short : T ) -> &mut Book
  //   {
  //     let mut url = std::path::PathBuf::from( "https://own_site/books" );
  //     url.push( url_short.as_ref().unwrap() );
  //     self.url = Some( url );
  //     self
  //   }
  // }

  let book = Book::former()
  .book_name( "Miserables" )
  .url( "name_of_book" )
  .form();

  let expected = Book
  {
    book_name : String::from( "Miserables" ),
    url : std::path::PathBuf::from( "https://own_site/books/name_of_book" ),
  };

  assert_eq!( book, expected );
}

//

test_suite!
{
  user_setter,
}

