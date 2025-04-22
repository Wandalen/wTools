use super::*;

#[ test ]
fn unit_variant_constructors()
{
  // Test the Status::Pending constructor
  let got_pending = Status::pending();
  let exp_pending = Status::Pending;
  assert_eq!( got_pending, exp_pending );

  // Test the Status::Complete constructor
  let got_complete = Status::complete();
  let exp_complete = Status::Complete;
  assert_eq!( got_complete, exp_complete );
}
