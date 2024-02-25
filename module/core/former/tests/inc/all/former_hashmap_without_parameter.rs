use former::Former;

struct HashMap< T >
{
  f1 : T,
}

#[derive( Former )]
pub struct Struct1
{
  pub string_slice_1 : HashMap< i32 >,
}

fn main()
{
}

// qqq : find out why, explain and fix that
//
// WARNINGS:
// ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
// warning: type `HashMap<i32>` is more private than the item `Struct1::string_slice_1`
//   --> tests/inc/all/former_hashmap_without_parameter.rs:11:3
//    |
// 11 |   pub string_slice_1 : HashMap< i32 >,
//    |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `Struct1::string_slice_1` is reachable at visibility `pub`
//    |
// note: but type `HashMap<i32>` is only usable at visibility `pub(crate)`
//   --> tests/inc/all/former_hashmap_without_parameter.rs:3:1
//    |
// 3  | struct HashMap< T >
//    | ^^^^^^^^^^^^^^^^^^^
//    = note: `#[warn(private_interfaces)]` on by default
//
// warning: 1 warning emitted
// ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈