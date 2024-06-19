use super::*;

use the_module::tree::ListNodeReport;
use willbe::tree::TreePrinter;

#[ test ]
fn node_with_depth_two_leaves_stop_spacer()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec!
    [
      ListNodeReport
      {
        name : "sub_node1".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![ ListNodeReport
        {
          name : "sub_sub_node1".into(),
          version : None,
          crate_dir : None,
          duplicate : false,
          normal_dependencies : vec![],
          dev_dependencies : vec![],
          build_dependencies : vec![],
        }],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      },
      ListNodeReport
      {
        name : "sub_node2".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![ ListNodeReport
        {
          name : "sub_sub_node2".into(),
          version : None,
          crate_dir : None,
          duplicate : false,
          normal_dependencies : vec![],
          dev_dependencies : vec![],
          build_dependencies : vec![],
        }],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }
    ],
    dev_dependencies : vec![],
    build_dependencies : vec![],
  };
  let expected = r#"
node
├─ sub_node1
│  └─ sub_sub_node1
└─ sub_node2
   └─ sub_sub_node2
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_depth_two_leaves()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec!
    [
      ListNodeReport
      {
        name : "sub_node1".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![ ListNodeReport
        {
          name : "sub_sub_node".into(),
          version : None,
          crate_dir : None,
          duplicate : false,
          normal_dependencies : vec![],
          dev_dependencies : vec![],
          build_dependencies : vec![],
        }],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      },
      ListNodeReport
      {
        name : "sub_node2".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }
    ],
    dev_dependencies : vec![],
    build_dependencies : vec![],
  };
  let expected = r#"
node
├─ sub_node1
│  └─ sub_sub_node
└─ sub_node2
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_depth_one_leaf()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec![ ListNodeReport
    {
      name : "sub_node".into(),
      version : None,
      crate_dir : None,
      duplicate : false,
      normal_dependencies : vec![ ListNodeReport
      {
        name : "sub_sub_node".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }],
      dev_dependencies : vec![],
      build_dependencies : vec![],
    }],
    dev_dependencies : vec![],
    build_dependencies : vec![],
  };
  let expected = r#"
node
└─ sub_node
   └─ sub_sub_node
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_build_dependencies_tree_with_two_leaves()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec![],
    dev_dependencies : vec![],
    build_dependencies : vec!
    [
      ListNodeReport
      {
        name : "build_sub_node1".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      },
      ListNodeReport
      {
        name : "build_sub_node2".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }
    ],
  };
  let expected = r#"
node
[build-dependencies]
├─ build_sub_node1
└─ build_sub_node2
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_build_dependencies_tree_with_one_leaf()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec![],
    dev_dependencies : vec![],
    build_dependencies : vec![
      ListNodeReport
      {
        name : "build_sub_node".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }
    ],
  };
  let expected = r#"
node
[build-dependencies]
└─ build_sub_node
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_dev_dependencies_tree_with_two_leaves()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec![],
    dev_dependencies : vec!
    [
      ListNodeReport
      {
        name : "dev_sub_node1".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      },
      ListNodeReport
      {
        name : "dev_sub_node2".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }
    ],
    build_dependencies : vec![],
  };
  let expected = r#"
node
[dev-dependencies]
├─ dev_sub_node1
└─ dev_sub_node2
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_dev_dependencies_tree_with_one_leaf()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec![],
    dev_dependencies : vec![
      ListNodeReport
      {
        name : "dev_sub_node".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }
    ],
    build_dependencies : vec![],
  };
  let expected = r#"
node
[dev-dependencies]
└─ dev_sub_node
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_dependencies_tree_with_two_leaves()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec!
    [
      ListNodeReport
      {
        name : "sub_node1".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      },
      ListNodeReport
      {
        name : "sub_node2".into(),
        version : None,
        crate_dir : None,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      }
    ],
    dev_dependencies : vec![],
    build_dependencies : vec![],
  };
  let expected = r#"
node
├─ sub_node1
└─ sub_node2
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn node_with_dependency_tree_with_one_leaf()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec![ ListNodeReport
    {
      name : "sub_node".into(),
      version : None,
      crate_dir : None,
      duplicate : false,
      normal_dependencies : vec![],
      dev_dependencies : vec![],
      build_dependencies : vec![],
    }],
    dev_dependencies : vec![],
    build_dependencies : vec![],
  };
  let expected = r#"
node
└─ sub_node
"#.trim();

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  let actual = actual.trim();
  println!("{actual}");

  assert_eq!( expected, actual );
}

#[ test ]
fn one_node_one_line()
{
  let node = ListNodeReport
  {
    name : "node".into(),
    version : None,
    crate_dir : None,
    duplicate : false,
    normal_dependencies : vec![],
    dev_dependencies : vec![],
    build_dependencies : vec![],
  };
  let expected = "node\n";

  let printer = TreePrinter::new( &node );
  let actual = printer.display_with_spacer( "" ).unwrap();
  println!("{actual}");

  assert_eq!( expected, actual );
}
