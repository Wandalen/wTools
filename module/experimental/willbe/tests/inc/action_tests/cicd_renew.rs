use super :: *;
use assert_fs ::prelude :: *;
use the_module :: { action };

//

use std :: { fs ::File, io ::Read };
use std ::fs ::create_dir_all;

fn arrange(sample_dir: &str) -> assert_fs ::TempDir
{
  let root_path = std ::path ::Path ::new(env!("CARGO_MANIFEST_DIR"));
  let assets_relative_path = std ::path ::Path ::new(ASSET_PATH);
  let assets_path = root_path.join(assets_relative_path);

  let temp = assert_fs ::TempDir ::new().unwrap();
  temp.copy_from(assets_path.join(sample_dir), &[ "**"]).unwrap();
  create_dir_all(temp.path().join(".github").join("workflows")).unwrap();
  temp
}

#[ test ]
fn default_case()
{
  // Arrange
  let temp = arrange("single_module");
  let base_path = temp.path().join(".github").join("workflows");

  // Act
  () = action ::cicd_renew ::action(&temp).unwrap();

  // Assert — workspace_push.yml is generated with correct content
  let workspace_push_path = base_path.join("workspace_push.yml");
  assert!(workspace_push_path.exists(), "workspace_push.yml should be generated");
  let mut file = File ::open(&workspace_push_path).unwrap();
  let mut content = String ::new();
  _ = file.read_to_string(&mut content).unwrap();
  assert!(content.contains("cargo metadata"), "workspace_push.yml should use cargo metadata");
  assert!(content.contains("all-crates"), "workspace_push.yml should have all-crates fan-in job");

  // Assert — expected infrastructure workflows are generated
  assert!(base_path.join("appropriate_branch.yml").exists());
  assert!(base_path.join("appropriate_branch_master.yml").exists());
  assert!(base_path.join("auto_pr.yml").exists());
  assert!(base_path.join("auto_pr_to_alpha.yml").exists());
  assert!(base_path.join("auto_pr_to_master.yml").exists());
  assert!(base_path.join("runs_clean.yml").exists());
  assert!(base_path.join("standard_rust_pull_request.yml").exists());
  assert!(base_path.join("standard_rust_push.yml").exists());
  assert!(base_path.join("for_pr_rust_push.yml").exists());
  assert!(base_path.join("readme.md").exists());

  // Assert — beta branch workflows are NOT generated
  assert!(!base_path.join("appropriate_branch_beta.yml").exists(), "beta branch workflow should not be generated");
  assert!(!base_path.join("auto_merge_to_beta.yml").exists(), "beta merge workflow should not be generated");
  assert!(!base_path.join("auto_pr_to_beta.yml").exists(), "beta PR workflow should not be generated");

  // Assert — removed legacy workflows are NOT generated
  assert!(!base_path.join("standard_rust_scheduled.yml").exists(), "scheduled workflow should not be generated");
  assert!(!base_path.join("standard_rust_status.yml").exists(), "status workflow should not be generated");
  assert!(!base_path.join("status_checks_rules_update.yml").exists(), "rules update workflow should not be generated");

  // Assert — per-crate workflows are NOT generated (replaced by workspace_push.yml)
  assert!(!base_path.join("module_test_module_push.yml").exists(), "per-crate workflows should not be generated");
}

// aaa: for Petro: fix styles
// aaa: ✅
