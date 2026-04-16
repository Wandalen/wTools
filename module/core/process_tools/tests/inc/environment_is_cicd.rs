use super :: *;

#[ test ]
fn is_cicd_consistent_with_ci_env_var()
{
  // When the canonical CI env var is present, is_cicd() must return true.
  // On developer machines where CI is not set the check is skipped — no false positives.
  if std ::env ::var( "CI" ).is_ok()
  {
    assert!
    (
      the_module ::environment ::is_cicd(),
      "is_cicd() must return true when the CI env var is set",
    );
  }
}
