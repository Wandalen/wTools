mod private
{
    use error_tools::for_app::Result;

    const APPROPRIATIVE_BRANCH: &str = 
    r#"

    name : appropriate_branch

    on :

      workflow_call :
        inputs :
          src_branch :
            required : true
            type : string
          dst_branch :
            required : true
            type : string
        secrets :
          PRIVATE_GITHUB_BOT_TOKEN :
            description : 'Github bot token'
            required : true

    env :

      CARGO_TERM_COLOR : always

    concurrency :

      group : appropraite_branch_${{ inputs.src_branch }}_${{ inputs.dst_branch }}
      cancel-in-progress : true

    jobs :

      check :
        runs-on : ubuntu-latest
        outputs :
          shouldSkip : ${{ steps.validation.outputs.wrong-target }}
        steps :
          - name : Check branch
            id : validation
            uses : Vankka/pr-target-branch-action@v2.1
            env :
              GITHUB_TOKEN : ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}
            with :
              target : ${{ inputs.dst_branch }}
              exclude : ${{ inputs.src_branch }}
              comment : |
                To maintain stability of the module the repository uses 3-stages system to forward changes from an unstable branch to a stable.
                The unstable branch is `alpha`. All user pull requests should be opened to this branch.
                The staging branch is `beta`. Changes to this branch are forwarded by a pull request from branch `alpha` automatically.
                The stable branch is `master`. Changes to this branch are forwarded by a pull request from branch `beta` automatically.

                The pull request was automatically converted to draft.
                Please, change base branch taking into account the described system `alpha -> beta -> master`.
          - name : Convert to draft
            if : ${{ steps.validation.outputs.wrong-target == 'true' }}
            uses: voiceflow/draft-pr@latest
            with:
              token: ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}
          - name : Failure
            if : ${{ steps.validation.outputs.wrong-target == 'true' }}
            run : exit 1
    "#;

    const APPROPRIATE_BRANCH_BETA: &str = 
    r#"
    name : appropriate_branch_beta

    on :
      pull_request_target :
        branches :
          - beta

    jobs :

    appropriate_branch :
        uses : Wandalen/wTools/.github/workflows/AppropriateBranch.yml@alpha
        with :
          src_branch : 'alpha'
          dst_branch : '${{ github.base_ref }}'
        secrets :
          PRIVATE_GITHUB_BOT_TOKEN : '${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}'
    "#;

    const AppropriateBranchMaster: &str = 
    r#"
    name : appropriate_branch_master

    on :
      pull_request_target :
        branches :
          - main
          - master

    jobs :

      appropriate_branch :
        uses : Wandalen/wTools/.github/workflows/AppropriateBranch.yml@alpha
        with :
          src_branch : 'beta'
          dst_branch : '${{ github.base_ref }}'
        secrets :
          PRIVATE_GITHUB_BOT_TOKEN : '${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}'
    "#;

    const AutoMergeToBeta: &str = 
    r#"
        
    name : auto_merge_to_beta

    on :
      push :
        branches : [ alpha ]

    concurrency :

      group : auto_merge_to_beta
      cancel-in-progress : true

    jobs :

      get_modules:
        outputs :
          workflow_files: ${{ steps.workflow_files.outputs.files }}
          workflow_names: ${{ steps.workflow_names.outputs.names }}
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v3
          - id: workflow_files
            run: |
              WORKFLOWS=$(ls .github/workflows | grep Module)
              for WORKFLOW in $WORKFLOWS ; do
                NAME=$(echo $WORKFLOW | sed 's/\(\S\+\).yml/\1/')
                NAMES="$NAMES $NAME"
              done;
              NAMES=$(sed 's/\s\+/\n/g' <<< $NAMES)
              OUTPUT=$(echo "$NAMES" | jq -R -s -c 'split("\n")[1:-1]')
              echo "files={\"modules\":$OUTPUT}" >> $GITHUB_OUTPUT
          - id: workflow_names
            run: |
              WORKFLOWS=$(ls .github/workflows | grep Module)
              for WORKFLOW in $WORKFLOWS ; do
                NAME=$(cat .github/workflows/$WORKFLOW | grep -G '^name :' | sed 's/name\s*:\s\+\(\S*\)/\1/')
                NAMES="$NAMES%0A$NAME"
              done;
              echo "names=$NAMES" >> $GITHUB_OUTPUT

      wait_for_modules :
        needs : get_modules
        runs-on : ubuntu-latest
        steps :
          - name : Waiting ...
            uses : willgarcia/workflow-wait-action@main
            with :
              timeout : 21600
              interval : 60
              initial_delay : 60
              workflows : ${{ needs.get_modules.outputs.workflow_names }}

      runs_check :
        needs :
          - get_modules
          - wait_for_modules
        strategy :
          matrix : ${{ fromJSON( needs.get_modules.outputs.workflow_files ) }}
        runs-on : ubuntu-latest
        steps :
          - name : Check workflow run status
            id : check_ci
            uses :  ronymeyer/workflow-status@v0.3.7
            with :
              token : ${{ secrets.GITHUB_TOKEN }}
              workflow : ${{ matrix.modules }}.yml
              event : push
              branch : alpha
          - name : Check failure conclusion
            if : ${{ steps.check_ci.outputs.conclusion == 'failure' }}
            run : exit 1
          - name : Check cancelled conclusion
            if : ${{ steps.check_ci.outputs.conclusion == 'cancelled' }}
            run : exit 1
          - name : Check skipped conclusion
            if : ${{ steps.check_ci.outputs.conclusion == 'skipped' }}
            run : exit 1

      merge :
        needs : runs_check
        runs-on : ubuntu-latest
        steps :
          - name : Find PR number for current commit
            uses : jwalton/gh-find-current-pr@v1
            id : find
            with :
              state: open
          - name: Automerge passed pull request
            if : ${{ success() }}
            uses: juliangruber/merge-pull-request-action@v1
            with:
              github-token: ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}
              repo: ${{ github.repository }}
              number: ${{ steps.find.outputs.number }}
              method: merge
    "#;

    const AUTO_PR: &str = 
    r#"
    name : auto_pr

    on :

    workflow_call :
      inputs :
        src_branch :
          required : true
          type : string
        dst_branch :
          required : true
          type : string
      secrets :
        PRIVATE_GITHUB_BOT_TOKEN :
          description : 'Github bot token'
          required : true

    concurrency :

      group : auto_pr_${{ inputs.src_branch }}_${{ inputs.dst_branch }}
      cancel-in-progress : true

    jobs :

    build :
      runs-on : ubuntu-latest
      steps :
        - uses : actions/checkout@v3
        - name : Open PR
          uses : vsoch/pull-request-action@1.0.18
          env :
            GITHUB_TOKEN : ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}
            PULL_REQUEST_BRANCH : ${{ inputs.dst_branch }}
            PULL_REQUEST_TITLE : 'AUTO : Forward from ${{ inputs.src_branch }} to ${{ inputs.dst_branch }}'
            PASS_IF_EXISTS : true
    "#;

    const AutoPrToAlpha: &str = 
    r#"
    
    name : auto_pr_to_alpha

    on :
      push :
        branches :
          - '*'
          - '*/*'
          - '**'
          - '!master'
          - '!main'
          - '!alpha'
          - '!beta'
          - '!*test*'
          - '!*test*/*'
          - '!*/*test*'
          - '!*experiment*'
          - '!*experiment*/*'
          - '!*/*experiment*'

    jobs :

      forward :
        uses : Wandalen/wTools/.github/workflows/AutoPr.yml@alpha
        with :
          src_branch : '${{ github.ref_name }}'
          dst_branch : 'alpha'
        secrets :
          PRIVATE_GITHUB_BOT_TOKEN : '${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}'

    "#;

    const AUTO_PR_TO_BETTA: &str = 
    r#"
    
name : auto_pr_to_beta

on :
  push :
    branches : [ alpha ]

jobs :

  forward :
    uses : Wandalen/wTools/.github/workflows/AutoPr.yml@alpha
    with :
      src_branch : 'alpha'
      dst_branch : 'beta'
    secrets :
      PRIVATE_GITHUB_BOT_TOKEN : '${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}'

    "#;

    const AUTO_PR_TO_MASTER: &str = 
    r#"
    
    "#;

    /// generate workflow
    pub fn workflow_generate() -> Result< () >
    {

        todo!()
    }
}

crate::mod_interface!
{
    prelude use workflow_generate;
}