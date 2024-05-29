// use super::*;

// tests_impls!
// {


//   fn node()
//   {
//     use the_module::prelude::*;
//     let mut factory = GenerativeNodeFactory::< the_module::IdentityWithInt >::from();

//     let n1 = factory.node_making( 1 );
//     let n1b = factory.node( 1 );
//     a_id!( n1, n1b.id() );
//     dbg!( &n1 );

//     let node1a = factory.node( 1 );
//     let node1b = factory.node( 1 );
//     a_id!( node1a, node1b );

//     let node1a = factory.node( &1 );
//     let node1b = factory.node( &&1 );
//     a_id!( node1a, node1b );

//   }

//   //


//   fn make_default()
//   {
//     use the_module::prelude::*;
//     use type_constructor::from;

//     let mut factory : GenerativeNodeFactory::< the_module::IdentityWithInt > = from!();
//     let n1 = factory.node_making( 1 );
//     let n1b = factory.node( 1 );
//     a_id!( n1, n1b.id() );

//   }

//   //


//   fn basic()
//   {
//     use the_module::prelude::*;
//     use type_constructor::from;

//     let mut factory = GenerativeNodeFactory::< the_module::IdentityWithInt >::from();

//     let a = factory.node_making( 1 );
//     let b = factory.node_making( 2 );

//     factory.node_add_out_node( a, b );
//     factory.node_add_out_nodes( b, [ a, b ].into_iter() );

//     a_id!( factory.nnodes(), 2 );
//     a_id!( factory.nedges(), 3 );

//     dbg!( factory.node( a ) );
//     dbg!( factory.node( b ) );

//     let got : HashSet< _ > = factory.out_nodes_ids( a ).collect();
//     let exp = hset![ b ];
//     a_id!( got, exp );
//     let got : HashSet< _ > = factory.out_nodes_ids( b ).collect();
//     let exp = hset![ a, b ];
//     a_id!( got, exp );

//     // let got : HashSet< _ > = factory.out_nodes_ids_2( a ).collect();
//     // let exp = hset![ b ];
//     // a_id!( got, exp );
//     // let got : HashSet< _ > = factory.out_nodes_ids_2( b ).collect();
//     // let exp = hset![ a, b ];
//     // a_id!( got, exp );

//     let got : HashSet< _ > = factory.out_edges( a ).map( | el | ( el.1.in_node, el.1.out_node ) ).collect();
//     let exp = hset![ ( a, b ) ];
//     a_id!( got, exp );
//     let got : HashSet< _ > = factory.out_edges( b ).map( | el | ( el.1.in_node, el.1.out_node ) ).collect();
//     let exp = hset![ ( b, a ), ( b, b ) ];
//     a_id!( got, exp );

//     // let got = factory.out_nodes_ids_2( a ).map( | id |
//     // {
//     //   // 13_i32
//     //   ( id, factory.node( id ) )
//     // });
//     // use test_tools::inspect_type_of;
//     // inspect_type_of!( got );

//   }

//   // xxx : fix test make_with_edge_list

//   fn make_with_edge_list()
//   {
//     use the_module::prelude::*;
//     use type_constructor::from;

//     let mut factory = GenerativeNodeFactory::< the_module::IdentityWithInt >::from();

//     factory.make_with_edge_list
//     ([
//       1, 2,
//       2, 1,
//       2, 2,
//     ]);

//     dbg!( factory.node( 1 ) );
//     dbg!( factory.node( 2 ) );

//     let exp = hset![ 2 ];
//     let got : HashSet< _ > = factory.out_nodes_ids( 1 ).collect();
//     a_id!( got, exp );
//     let exp = hset![ 1, 2 ];
//     let got : HashSet< _ > = factory.out_nodes_ids( 2 ).collect();
//     a_id!( got, exp );

//     let got : HashSet< _ > = factory.out_edges( 1 ).map( | el | ( el.1.in_node, el.1.out_node ) ).collect();
//     let exp = hset![ ( factory.edge_id( 1 ), factory.edge_id( 2 ) ) ];
//     a_id!( got, exp );
//     let got : HashSet< _ > = factory.out_edges( 2 ).map( | el | ( el.1.in_node, el.1.out_node ) ).collect();
//     let exp = hset![ ( factory.edge_id( 2 ), factory.edge_id( 1 ) ), ( factory.edge_id( 2 ), factory.edge_id( 2 ) ) ];
//     // let exp = hset![ factory.edge_ids( 2, 1 ), factory.edge_ids( 2, 2 ) ];
//     // let exp : HashSet< ( the_module::IdentityWithInt, the_module::IdentityWithInt ) > = hset![ ( 2, 1 ).into(), ( 2, 2 ).into() ];
//     a_id!( got, exp );

//   }

//   //

// // xxx : fix it
// //
// //   fn make_with_edge_list_string()
// //   {
// //     use the_module::prelude::*;
// //
// //     let mut factory = ReadableNodeFactory::< the_module::IdentityWithName >::make();
// //
// //     factory.make_with_edge_list
// //     ([
// //       "A", "B",
// //       "B", "A",
// //       "B", "B",
// //     ]);
// //
// //     dbg!( factory.node( "A" ) );
// //     dbg!( factory.node( "B" ) );
// //
// //     let exp = hset![ "B" ];
// //     let got : HashSet< _ > = factory.out_nodes_ids( "A" ).collect();
// //     a_id!( got, exp );
// //
// //     let exp = hset![ "A", "B" ];
// //     let got : HashSet< _ > = factory.out_nodes_ids( "B" ).collect();
// //     a_id!( got, exp );
// //   }

//   //


//   fn graph_print()
//   {
//     use the_module::prelude::*;

//     let mut factory = GenerativeNodeFactory::< the_module::IdentityWithInt >::from();

//     factory.make_with_edge_list
//     ([
//       1, 2,
//       2, 1,
//       2, 2,
//     ]);

//     let exp = r#"GenerativeNodeFactory
//   node::1
//    - 2
//   node::2
//    - 1
//    - 2"#;
//     let got = format!( "{:?}", factory );
//     println!( "{}", got );
//     a_id!( got, exp );

//   }

// }
