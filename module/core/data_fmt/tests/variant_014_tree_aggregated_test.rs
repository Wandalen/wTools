//! Variant 014: Tree Aggregated spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeBuilder, TreeFormatter };

/// VT-1: directory nodes show aggregated totals
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_014_vt_01_directory_aggregated_totals()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "main.rs" ], 150i64 )
    .insert( &[ "src", "lib.rs" ], 300i64 )
    .build();

  let out = TreeFormatter::new().format_with_aggregation(
    &tree,
    450i64,
    | v | *v,
    | v | v as f64,
    | v, _total, _pct | format!( "{v}" ),
    | _name, total, _pct | format!( "{total}" ),
  );

  assert!( out.contains( "450" ), "aggregated total visible: {out}" );
  assert!( out.contains( "150" ), "leaf value 150 visible: {out}" );
  assert!( out.contains( "300" ), "leaf value 300 visible: {out}" );
}

/// VT-2: leaf values preserved alongside aggregation
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_014_vt_02_leaf_values_preserved()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "test.rs" ], 50i64 )
    .build();

  let out = TreeFormatter::new().format_with_aggregation(
    &tree,
    50i64,
    | v | *v,
    | v | v as f64,
    | v, _total, _pct | format!( "{v}" ),
    | _name, total, _pct | format!( "{total}" ),
  );

  assert!( out.contains( "50" ), "leaf value preserved: {out}" );
}

/// VT-3: aggregation function applied recursively
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_014_vt_03_recursive_aggregation()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "main.rs" ], 100i64 )
    .insert( &[ "tests", "test.rs" ], 50i64 )
    .build();

  let out = TreeFormatter::new().format_with_aggregation(
    &tree,
    150i64,
    | v | *v,
    | v | v as f64,
    | v, _total, _pct | format!( "{v}" ),
    | _name, total, _pct | format!( "{total}" ),
  );

  // src total = 100, tests total = 50 (root is not rendered by format_with_aggregation)
  assert!( out.contains( "100" ), "src subtotal: {out}" );
  assert!( out.contains( "50" ), "tests subtotal: {out}" );
}

/// VT-4: single-leaf tree shows leaf value without aggregation noise
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_014_vt_04_single_leaf()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "file.rs" ], 42i64 )
    .build();

  let out = TreeFormatter::new().format_with_aggregation(
    &tree,
    42i64,
    | v | *v,
    | v | v as f64,
    | v, _total, _pct | format!( "{v}" ),
    | _name, total, _pct | format!( "{total}" ),
  );

  assert!( out.contains( "42" ), "leaf value present: {out}" );
  assert!( out.contains( "file.rs" ), "leaf name present: {out}" );
}
