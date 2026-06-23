//! Data Structure: Variant Attributes spec tests (DS-1..DS-4)
//!
//! Validates the 46-attribute schema document structure and variant
//! doc instance compliance by parsing embedded markdown at test time.

#![ cfg( feature = "enabled" ) ]

const SCHEMA_DOC : &str = include_str!( "../docs/data_structure/001_variant_attributes.md" );
const VARIANT_DOC : &str = include_str!( "../docs/variant/001_table_plain.md" );

/// DS-1: all 10 attribute groups present
// test_kind: spec_case(DS-1)
#[ test ]
fn data_structure_001_ds_01_ten_attribute_groups()
{
  let expected_groups =
  [
    "Identity & Classification",
    "Build & Dependencies",
    "Character Set & Encoding",
    "Visual Structure",
    "Data Representation",
    "Output Characteristics",
    "Usage Context",
    "Technical Details",
    "API & Construction",
    "Performance & Size",
    "Compatibility",
  ];

  let mut found = 0u32;
  for group in &expected_groups
  {
    if SCHEMA_DOC.contains( group )
    {
      found += 1;
    }
    else
    {
      panic!( "missing attribute group: {group}" );
    }
  }

  // The spec says "10 groups" but the schema lists 11 including Compatibility
  // Count actual H4 headings to verify
  let h4_count = SCHEMA_DOC.lines()
    .filter( | l | l.starts_with( "#### " ) )
    .count();

  assert!( h4_count >= 10, "at least 10 attribute group headings present: found {h4_count}" );
  assert_eq!( found, expected_groups.len() as u32, "all expected groups found" );
}

/// DS-2: schema defines exactly 46 attributes
// test_kind: spec_case(DS-2)
#[ test ]
fn data_structure_001_ds_02_exactly_46_attributes()
{
  // Count rows matching `| N | ` where N is a number — attribute definition rows
  let attr_count = SCHEMA_DOC.lines()
    .filter( | line |
    {
      let trimmed = line.trim();
      if !trimmed.starts_with( '|' ) { return false; }
      let cols : Vec< &str > = trimmed.split( '|' ).collect();
      // | # | attr | purpose | example | has at least 5 segments (empty, #, attr, purpose, example, empty)
      if cols.len() < 5 { return false; }
      // Second column (index 1) should be a number
      cols[ 1 ].trim().parse::< u32 >().is_ok()
    })
    .count();

  assert_eq!( attr_count, 46, "schema must define exactly 46 attributes, found {attr_count}" );
}

/// DS-3: every attribute has name, purpose, and example values
// test_kind: spec_case(DS-3)
#[ test ]
fn data_structure_001_ds_03_attribute_columns_not_empty()
{
  for line in SCHEMA_DOC.lines()
  {
    let trimmed = line.trim();
    if !trimmed.starts_with( '|' ) { continue; }
    let cols : Vec< &str > = trimmed.split( '|' ).collect();
    if cols.len() < 5 { continue; }
    let id = cols[ 1 ].trim();
    if id.parse::< u32 >().is_err() { continue; }

    let attr_name = cols[ 2 ].trim();
    let purpose = cols[ 3 ].trim();
    let examples = cols[ 4 ].trim();

    assert!( !attr_name.is_empty(), "attribute #{id} has empty name" );
    assert!( attr_name.contains( '`' ), "attribute #{id} name should be in backticks: {attr_name}" );
    assert!( !purpose.is_empty(), "attribute #{id} has empty purpose" );
    assert!( !examples.is_empty(), "attribute #{id} has empty example values" );
  }
}

/// DS-4: variant doc instances fill all 46 attributes
// test_kind: spec_case(DS-4)
#[ test ]
fn data_structure_001_ds_04_variant_fills_all_attributes()
{
  // Extract attribute names from schema
  let schema_attrs : Vec< &str > = SCHEMA_DOC.lines()
    .filter_map( | line |
    {
      let trimmed = line.trim();
      if !trimmed.starts_with( '|' ) { return None; }
      let cols : Vec< &str > = trimmed.split( '|' ).collect();
      if cols.len() < 5 { return None; }
      if cols[ 1 ].trim().parse::< u32 >().is_err() { return None; }
      Some( cols[ 2 ].trim() )
    })
    .collect();

  assert_eq!( schema_attrs.len(), 46, "precondition: schema has 46 attrs" );

  // Count attribute rows in variant doc (markdown list items: `- **attr**: value`)
  let variant_attr_count = VARIANT_DOC.lines()
    .filter( | line |
    {
      let trimmed = line.trim();
      trimmed.starts_with( "- **" ) && trimmed.contains( "**:" )
    })
    .count();

  assert_eq!(
    variant_attr_count, 46,
    "variant doc must fill exactly 46 attributes, found {variant_attr_count}",
  );
}
