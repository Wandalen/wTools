#[ allow( unused_imports ) ]
use super::*;

only_for_wtools!
{
  #[ allow( unused_imports ) ]
  use wtools::meta::*;
  #[ allow( unused_imports ) ]
  use wtools::former::Former;
}

only_for_local_module!
{
  #[ allow( unused_imports ) ]
  use meta_tools::*;
  #[ allow( unused_imports ) ]
  use former::Former;
}

//

tests_impls!
{
  #[ ignore ]
  fn user_setter()
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

    impl BookFormer
    {
      /* does not compile, duplicated method */
      // /// Trivial example of user setter.
      // pub fn url< T : AsRef< str > >( &mut self, url_short : T ) -> &mut BookFormer
      // {
      //   let mut url = std::path::PathBuf::from( "https://own_site/books" );
      //   url.push( url_short.as_ref() );
      //   self.url = Some( url );
      //   self
      // }
      /* compiles, unique method */
      pub fn url_u< T : AsRef< str > >( &mut self, url_short : T ) -> &mut BookFormer
      {
        let mut url = std::path::PathBuf::from( "https://own_site/books" );
        url.push( url_short.as_ref() );
        self.url = Some( url );
        self
      }
    }

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
}

//

tests_index!
{
  user_setter,
}

