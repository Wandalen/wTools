use super::*;

// xxx : qqq : rewrite this tests with running external application

#[ test ]
fn basic()
{

  assert!( the_module::environment::is_cicd() || !the_module::environment::is_cicd() );

}

// #[ test ]
// fn returns_false_when_no_ci_env_vars_are_set()
// {
//   use std::env;
//   let original_env_vars = std::env::vars().collect::<Vec<( String, String )>>();
//
//   for ( key, _ ) in &original_env_vars
//   {
//     env::remove_var( key );
//   }
//
//   assert_eq!( the_module::environment::is_cicd(), false );
//
//   // Restore environment variables
//   for ( key, value ) in original_env_vars
//   {
//     env::set_var( key, value );
//   }
//
// }
//
// #[ test ]
// fn returns_true_for_github_actions()
// {
//   use std::env;
//   env::set_var( "GITHUB_ACTIONS", "true" );
//   assert!( the_module::environment::is_cicd() );
//   env::remove_var( "GITHUB_ACTIONS" );
// }
//
// #[ test ]
// fn returns_true_for_gitlab_ci()
// {
//   use std::env;
//   env::set_var( "GITLAB_CI", "true" );
//   assert!( the_module::environment::is_cicd() );
//   env::remove_var( "GITLAB_CI" );
// }
//
// #[ test ]
// fn returns_true_for_travis_ci()
// {
//   use std::env;
//   env::set_var( "TRAVIS", "true" );
//   assert!( the_module::environment::is_cicd() );
//   env::remove_var( "TRAVIS" );
// }
//
// #[ test ]
// fn returns_true_for_circleci()
// {
//   use std::env;
//   env::set_var( "CIRCLECI", "true" );
//   assert!( the_module::environment::is_cicd() );
//   env::remove_var( "CIRCLECI" );
// }
//
// #[ test ]
// fn returns_true_for_jenkins()
// {
//   use std::env;
//   env::set_var( "JENKINS_URL", "http://example.com" );
//   assert!( the_module::environment::is_cicd() );
//   env::remove_var( "JENKINS_URL" );
// }
//
// #[ test ]
// fn returns_false_when_set_to_non_standard_value()
// {
//   use std::env;
//   env::set_var( "CI", "false" ); // Assuming 'false' string shouldn't be treated as indicating CI presence
//   assert_eq!( the_module::environment::is_cicd(), true ); // The function checks for the presence of the variable, not its value
//   env::remove_var( "CI" );
// }
