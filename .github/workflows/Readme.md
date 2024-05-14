# healthtable

[healthtable](../../Readme.md) - in addition to information about modules, their stability contains the results of CI/CD of the master and alpha branches.

# for_pr_rust_push.yml

actions:
- install stable rust
- install nightly rust
- install willbe
- run tests with all features, but only on stable toolchain and in debug optimization mode

Groups creates by strategy:
```yml  
for_pr_rust_push_${{ inputs.module_name }}_${{ github.ref }}_  
 ${{ contains( inputs.commit_message, '+test' ) || startsWith( inputs.commit_message, 'merge' ) }}  
```  

inputs.module_name - name of module  
github.ref - name of branch  
{{ contains( inputs.commit_message, '+test' ) || startsWith( inputs.commit_message, 'merge' ) }} - returns true if commit message contains directive `+test` or starts with `merge` word.

runs if commit message contains directive `+test` or starts with `merge` word.

# standard_rust_push.yml

actions:
- install stable rust
- install nightly rust
- install cargo-udeps
- install cargo-audit
- checks crate with cargo-udeps
- checks crete with cargo-audit
- install willbe
- run tests with all features, with stable and nightly toolchain, with release and debug optimization mode

Groups creates by strategy:
```yml  
standard_rust_push_${{ inputs.module_name }}_${{ github.ref }}_  
 ${{ contains( inputs.commit_message, '+test' ) || startsWith( inputs.commit_message, 'merge' ) }}  
```  

inputs.module_name - name of module  
github.ref - name of branch  
{{ contains( inputs.commit_message, '+test' ) || startsWith( inputs.commit_message, 'merge' ) }} - returns true if commit message contains directive `+test` or starts with `merge` word.

runs if commit message contains directive `+test` or starts with `merge` word.

# standard_rust_pool_request.yml

actions:
- call [for_pr_rust_push.yml](#for_pr_rust_pushyml) for all project.

# standard_rust_schedule.yml

actions:
- call [standard_rust_push.yml](#standard_rust_pushyml) for `{module_name}` every day at 1:00 a.m.

Affects badges in the header of the workspace readme.md file, looks like this [![wTools](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/standard_rust_scheduled.yml?label=master&logo=github&branch=master)](https://github.com/Wandalen/wTools/actions/workflows/standard_rust_scheduled.yml).

# module_{module_name}_push.yml

actions:
- call [standard_rust_push.yml](#standard_rust_pushyml) for `{module_name}`.

Affects badges that are opposite to modules in the **[healthtable](#healthtable)**, as well as badges in the header of the crate readme.md files.


# appropriate_branch.yml

This workflow ensures that pull requests are opened against the correct target branches based on a predefined branching strategy (alpha -> beta -> master). It checks whether the destination branch specified in the pull request matches the expected branch according to the branching strategy. If it doesn't match, the pull request is converted to draft mode, and if it still doesn't match, the workflow fails.

# appropriate_branch_beta.yml

This workflow delegates the actual validation and actions to another workflow file (appropriate_branch.yml) located in the Wandalen/wTools repository under .github/workflows directory on the "alpha" branch. It ensures that pull requests targeting the "beta" branch are appropriately validated and processed according to the rules defined in the external workflow file.

# appropriate_branch_master.yml

Similar to the previous workflow, this one also delegates the validation and processing of pull requests to an external workflow file (appropriate_branch.yml) located in the Wandalen/wTools repository under the .github/workflows directory on the "alpha" branch.  
By specifying the "beta" branch as the source branch and dynamically referencing the base branch of the pull request as the destination branch, this workflow ensures that pull requests targeting the "main" or "master" branches are appropriately validated and processed according to the rules defined in the external workflow file.  
This setup promotes consistency and reusability of workflow logic across different branches within the repository, helping to maintain a standardized process for handling pull requests.

# auto_merge_to_beta.yml

This workflow automates the process of merging changes from the "alpha" branch into the "beta" branch after ensuring that related workflow runs for modules have completed successfully.  
It waits for the completion of workflow runs related to modules and checks their statuses before proceeding with the merge process.  
If all checks pass, it merges the changes into the "beta" branch using the provided GitHub token.

# auto_pr.yml

This workflow automates the process of opening pull requests between specified source and destination branches.  
Upon triggering, it checks out the repository and opens a pull request from the source branch (src_branch) to the destination branch (dst_branch).  
The pull request title is automatically generated to indicate that it's an automated pull request forwarding from one branch to another.  
If a pull request already exists between the specified branches and PASS_IF_EXISTS is set to true, the action will pass without creating a new pull request.

# auto_pr_to_alpha.yml

This workflow automates the process of opening pull requests from any branch except for those explicitly excluded to the "alpha" branch.  
It leverages branch filtering to include all branches and exclude specific ones such as master, main, alpha, beta, and any branches containing test or experiment in their names.  
When triggered by a push event on a qualifying branch, it calls the external workflow (auto_pr.yml) to handle the process of opening a pull request to the "alpha" branch, passing the source and destination branch information along with the GitHub bot token for authentication.

# auto_pr_to_beta.yml

This workflow automates the process of opening pull requests from the "alpha" branch to the "beta" branch.  
When triggered by a push event on the "alpha" branch, it calls the external workflow (auto_pr.yml) to handle the process of opening a pull request to the "beta" branch, passing the source and destination branch information along with the GitHub bot token for authentication.

# auto_pr_to_master.yml

This workflow automates the process of opening pull requests from the "beta" branch to the "master" branch.  
When triggered by a push event on the "beta" branch, it calls the external workflow (auto_pr.yml) to handle the process of opening a pull request to the "master" branch, passing the source and destination branch information along with the GitHub bot token for authentication.

# runs_clean.yml

This workflow allows manual triggering to clean up workflow runs in the repository.  
It first deletes any runs that have been cancelled or skipped, ensuring that they do not clutter the workflow history.  
Then, it deletes runs older than a specified number of days, while ensuring that at least 20 runs are preserved regardless of their age.  
By regularly cleaning up older workflow runs, this workflow helps maintain a clean and organized workflow history in the repository.

# standard_rust_status.yml

This workflow serves as a status monitor for the completion of specific workflows: "auto_merge_to_beta" and "rust_scheduled."  
Upon completion of any of these workflows, it checks the status of their runs.  
It employs a matrix strategy to iterate over different workflow files to check their statuses.  
If the conclusion of any checked workflow run is "failure," "cancelled," or "skipped," the workflow exits with an error, indicating a problem.

# status_checks_rules_update.yml

When a pull request is opened targeting branches "alpha" or "beta":
- If the base branch is "beta":
    - It compares the contents of the workflow directories between branches "alpha" and "beta".
    - If they are not equal, it triggers an update of branch protection rules for the "beta" branch.
- If the base branch is "alpha":
    - It directly triggers an update of branch protection rules for the "alpha" branch with specific required status checks for different contexts.