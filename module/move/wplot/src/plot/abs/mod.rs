::meta_tools::mod_interface!
{

  /// Describe change.
  layer change;
  /// Describe changer.
  layer changer;
  /// Describe system.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer context;

  /// Identity of resource.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer identity;
  /// Registry.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer registry;

  // exposed use Drawing;

}