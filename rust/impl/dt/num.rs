#![ warn( missing_docs ) ]

/* xxx : qqq : for rust : move */

pub trait SignedOf
{
  type Unsigned;
  type Signed;
}

impl SignedOf for i8
{
  type Unsigned = u8;
  type Signed = i8;
}

impl SignedOf for i16
{
  type Unsigned = u16;
  type Signed = i16;
}

impl SignedOf for i32
{
  type Unsigned = u32;
  type Signed = i32;
}

impl SignedOf for i64
{
  type Unsigned = u64;
  type Signed = i64;
}

impl SignedOf for i128
{
  type Unsigned = u128;
  type Signed = i128;
}

impl SignedOf for isize
{
  type Unsigned = usize;
  type Signed = isize;
}

impl SignedOf for u8
{
  type Unsigned = u8;
  type Signed = i8;
}

impl SignedOf for u16
{
  type Unsigned = u16;
  type Signed = i16;
}

impl SignedOf for u32
{
  type Unsigned = u32;
  type Signed = i32;
}

impl SignedOf for u64
{
  type Unsigned = u64;
  type Signed = i64;
}

impl SignedOf for u128
{
  type Unsigned = u128;
  type Signed = i128;
}

impl SignedOf for usize
{
  type Unsigned = usize;
  type Signed = isize;
}

//

// pub trait SignedOf< Unsigned >
// {
//   type Unsigned;
//   type Signed;
// }
//
// impl SignedOf< i8 > for i8
// {
//   type Unsigned = u8;
//   type Signed = i8;
// }
//
// impl SignedOf< i16 > for i16
// {
//   type Unsigned = u16;
//   type Signed = i16;
// }
//
// impl SignedOf< i32 > for i32
// {
//   type Unsigned = u32;
//   type Signed = i32;
// }
//
// impl SignedOf< i64 > for i64
// {
//   type Unsigned = u64;
//   type Signed = i64;
// }
//
// impl SignedOf< i128 > for i128
// {
//   type Unsigned = u128;
//   type Signed = i128;
// }
//
// impl SignedOf< isize > for isize
// {
//   type Unsigned = usize;
//   type Signed = isize;
// }
//
// impl SignedOf< u8 > for u8
// {
//   type Unsigned = u8;
//   type Signed = i8;
// }
//
// impl SignedOf< u16 > for u16
// {
//   type Unsigned = u16;
//   type Signed = i16;
// }
//
// impl SignedOf< u32 > for u32
// {
//   type Unsigned = u32;
//   type Signed = i32;
// }
//
// impl SignedOf< u64 > for u64
// {
//   type Unsigned = u64;
//   type Signed = i64;
// }
//
// impl SignedOf< u128 > for u128
// {
//   type Unsigned = u128;
//   type Signed = i128;
// }
//
// impl SignedOf< usize > for usize
// {
//   type Unsigned = usize;
//   type Signed = isize;
// }
