mod private
{
    use error_tools::for_app::Result;

    fn appropriative_branch() -> String
    {
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
      
        group : appropraite_branch_${{{{ inputs.src_branch }}}}_${{{{ inputs.dst_branch }}}}
        cancel-in-progress : true
      
      jobs :
      
        check :
          runs-on : ubuntu-latest
          outputs :
            shouldSkip : ${{{{ steps.validation.outputs.wrong-target }}}}
          steps :
            - name : Check branch
              id : validation
              uses : Vankka/pr-target-branch-action@v2.1
              env :
                GITHUB_TOKEN : ${{{{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}}}
              with :
                target : ${{{{ inputs.dst_branch }}}}
                exclude : ${{{{ inputs.src_branch }}}}
                comment : |
                  To maintain stability of the module the repository uses 3-stages system to forward changes from an unstable branch to a stable.
                  The unstable branch is `alpha`. All user pull requests should be opened to this branch.
                  The staging branch is `beta`. Changes to this branch are forwarded by a pull request from branch `alpha` automatically.
                  The stable branch is `master`. Changes to this branch are forwarded by a pull request from branch `beta` automatically.
      
                  The pull request was automatically converted to draft.
                  Please, change base branch taking into account the described system `alpha -> beta -> master`.
            - name : Convert to draft
              if : ${{{{ steps.validation.outputs.wrong-target == 'true' }}}}
              uses: voiceflow/draft-pr@latest
              with:
                token: ${{{{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}}}
            - name : Failure
              if : ${{{{ steps.validation.outputs.wrong-target == 'true' }}}}
              run : exit 1
      "#.into()
    }

    fn appropraite_branch_for( branches: &str, uses_branch: &str, src_branch: &str, name: &str ) -> String
    {
        format!(r#"
        name : appropriate_branch_{name}
        
        on :
          pull_request_target :
            branches :
              {branches}
        
        jobs :
        
          appropriate_branch :
            uses : Wandalen/wTools/.github/workflows/AppropriateBranch.yml@{uses_branch}
            with :
              src_branch : '{src_branch}'
              dst_branch : '${{{{ github.base_ref }}}}'
            secrets :
              PRIVATE_GITHUB_BOT_TOKEN : '${{{{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}}}'
        "#)
    }

    fn auto_merge_to( branch: &str, group_branch: &str, name: &str ) -> String
    {
      format!(r#"
      
name : auto_merge_to_{name}

on :
  push :
    branches : [ {branch} ]

concurrency :

  group : auto_merge_to_{group_branch}
  cancel-in-progress : true

jobs :

  get_modules:
    outputs :
      workflow_files: ${{{{ steps.workflow_files.outputs.files }}}}
      workflow_names: ${{{{ steps.workflow_names.outputs.names }}}}
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
          echo "files={{\"modules\":$OUTPUT}}" >> $GITHUB_OUTPUT
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
          workflows : ${{{{ needs.get_modules.outputs.workflow_names }}}}

  runs_check :
    needs :
      - get_modules
      - wait_for_modules
    strategy :
      matrix : ${{{{ fromJSON( needs.get_modules.outputs.workflow_files ) }}}}
    runs-on : ubuntu-latest
    steps :
      - name : Check workflow run status
        id : check_ci
        uses :  ronymeyer/workflow-status@v0.3.7
        with :
          token : ${{{{ secrets.GITHUB_TOKEN }}}}
          workflow : ${{{{ matrix.modules }}}}.yml
          event : push
          branch : {branch}
      - name : Check failure conclusion
        if : ${{{{ steps.check_ci.outputs.conclusion == 'failure' }}}}
        run : exit 1
      - name : Check cancelled conclusion
        if : ${{{{ steps.check_ci.outputs.conclusion == 'cancelled' }}}}
        run : exit 1
      - name : Check skipped conclusion
        if : ${{{{ steps.check_ci.outputs.conclusion == 'skipped' }}}}
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
        if : ${{{{ success() }}}}
        uses: juliangruber/merge-pull-request-action@v1
        with:
          github-token: ${{{{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}}}
          repo: ${{{{ github.repository }}}}
          number: ${{{{ steps.find.outputs.number }}}}
          method: merge
"#)
    }

    fn auto_pr() -> String
    {
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
        
          group : auto_pr_${{{{ inputs.src_branch }}}}_${{{{ inputs.dst_branch }}}}
          cancel-in-progress : true
        
        jobs :
        
          build :
            runs-on : ubuntu-latest
            steps :
              - uses : actions/checkout@v3
              - name : Open PR
                uses : vsoch/pull-request-action@1.0.18
                env :
                  GITHUB_TOKEN : ${{{{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}}}
                  PULL_REQUEST_BRANCH : ${{{{ inputs.dst_branch }}}}
                  PULL_REQUEST_TITLE : 'AUTO : Forward from ${{{{ inputs.src_branch }}}} to ${{{{ inputs.dst_branch }}}}'
                  PASS_IF_EXISTS : true
        "#.into()
    }

    fn auto_pr_to( name: &str, branches: &str, uses: &str, src_branch: &str, dest_branch: &str ) -> String
    {
        format!
        (
          r#"
          name : auto_pr_to_{name}

          on :
            push :
              branches :
                {branches}
                  
          jobs :
                  
            forward :
              uses : Wandalen/wTools/.github/workflows/AutoPr.yml@{uses}
              with :
                src_branch : '{src_branch}'
                dst_branch : '{dest_branch}'
              secrets :
                PRIVATE_GITHUB_BOT_TOKEN : '${{{{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}}}'
          "#
        )
    }

    fn module_push( name: &str, branch: &str, manifest_path: &str ) -> String
    {
        format!
        (
          r#"
          name : {name}

          on : push
                  
          env :
            CARGO_TERM_COLOR : always
                  
          jobs :
                                    
            test :
              uses : Wandalen/wTools/.github/workflows/StandardRustPush.yml@{branch}
              with :
                manifest_path : '{manifest_path}'
                module_name : '{name}'
                commit_message : ${{{{ github.event.head_commit.message }}}}"#
        )
    }

    fn rust_clean() -> String
    {
        r#"
        
        name : runs_clean

        on :

          workflow_dispatch :
            inputs :
              days :
                description : 'Older than number of days.'
                required : true
                type : number
                default : 0

        concurrency :

          group : runs_clean
          cancel-in-progress : true

            jobs :

              del_runs :
                runs-on : ubuntu-latest
                steps :
                  - name : Delete skipped and cancelled runs
                    uses : dmvict/clean-workflow-runs@v1
                    with :
                      token : ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}
                      run_conclusions : |
                        cancelled
                        skipped
                      save_period : 0
                      save_min_runs_number : 0
                  - name : Delete older workflow runs
                    uses : dmvict/clean-workflow-runs@v1
                    with :
                      token : ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}
                      save_period : ${{ github.event.inputs.days }}
                      save_min_runs_number : 20

        "#.into()
    }

    fn standard_rust_pull_request() -> String 
    {
        r#"
        
name : rust_pull_request

on : [ pull_request ]

env :
  CARGO_TERM_COLOR : always

concurrency :
  group : standard_rust_pull_request_${{ github.event.base.ref }}_${{ github.event.number }}
  cancel-in-progress : true

jobs :

  check :
    if : ${{ github.event.pull_request.head.repo.fork }}
    runs-on : ubuntu-latest
    outputs :
      commit_message : ${{ steps.message.outputs.message }}
      should_run : ${{ steps.run.outputs.should_run }}
    steps :
      - name : List commits on the pull request
        run : |
          response=$(curl --request GET \
          --url 'https://api.github.com/repos/${{ github.repository }}/pulls/${{ github.event.pull_request.number }}/commits' \
          --header 'Authorization: token ${{ secrets.GITHUB_TOKEN }}' \
          --header 'Accept: application/vnd.github.v3+json' \
          --header 'Content-Type: application/json')
          echo $response > response.json
      - name : Get latest commit
        id : message
        run : |
          length=$(jq 'length' response.json)
          index=$(($length - 1))
          latest_commit=$(jq --argjson index $index '.[$index]' response.json)
          latest_commit_message=$(echo "$latest_commit" | jq -r '.commit.message')
          echo "message=$latest_commit_message" >> $GITHUB_OUTPUT
      - name : Set output
        id: run
        if : "!startsWith( steps.message.outputs.message, 'Merge ' )"
        run : echo "should_run=true" >> $GITHUB_OUTPUT

  tested :
    needs: check
    if : ${{ needs.check.outputs.should_run == 'true' }}
    uses : Wandalen/wTools/.github/workflows/StandardRustPush.yml@alpha
    with :
      manifest_path : './Cargo.toml'
      module_name : ${{ github.event.base.ref }}_${{ github.event.number }}
      commit_message : ${{ github.event.base.ref }}_${{ github.event.number }}

        "#.into()
    }

    fn standard_rust_push() -> String
    {
        r#"
        
name : rust_push

on :

  workflow_call :
    inputs :
      manifest_path :
        required : true
        type : string
      module_name :
        required : true
        type : string
      commit_message :
        required : true
        type : string
      with_smoke :
        required : false
        type : string
        default : true

concurrency :

  group : standard_rust_push_${{ inputs.module_name }}_${{ github.ref }}_
    ${{ contains( inputs.commit_message, '!test' ) || startsWith( inputs.commit_message, 'Merge' ) || contains( inputs.commit_message, inputs.module_name ) }}_
    ${{ !contains( inputs.commit_message, '!only_js' )}}
  cancel-in-progress : true

env :

  RUST_BACKTRACE : 1
  CARGO_TERM_COLOR : always
  WITH_SMOKE : ${{ inputs.with_smoke }}

jobs :

  fast :
    if : |
      !contains( inputs.commit_message, '!test' )
      && !startsWith( inputs.commit_message, 'Merge' )
      && contains( inputs.commit_message, inputs.module_name )
      && !contains( inputs.commit_message, '!only_js' )
    concurrency :
      group : standard_rust_push_${{ inputs.module_name }}_${{ github.ref }}_${{ matrix.os }}
      cancel-in-progress : true
    strategy :
      fail-fast : false
      matrix :
        os : [ ubuntu-latest ]
    runs-on : ${{ matrix.os }}
    steps :
      - name : Install latest stable toolchain
        uses : Wandalen/wretry.action@master
        with :
          action : actions-rs/toolchain@v1
          with : |
            toolchain : stable
            override : true
          attempt_limit : 3
          attempt_delay: 10000
      - uses : actions/checkout@v3
      - name : Run tests with default features
        run : cargo test --manifest-path ${{ inputs.manifest_path }}
      - name : Run tests without default features
        run : cargo test --manifest-path ${{ inputs.manifest_path }} --no-default-features

  full :
    if : |
      startsWith( inputs.commit_message, 'Merge' )
      || ( contains( inputs.commit_message, '!test' ) && !contains( inputs.commit_message, '!only_js' ) )
    concurrency :
      group : standard_rust_push_${{ inputs.module_name }}_${{ github.ref }}_${{ matrix.os }}
      cancel-in-progress : true
    strategy :
      fail-fast : false
      matrix :
        os : [ ubuntu-latest, windows-latest, macos-latest ]
    runs-on : ${{ matrix.os }}
    steps :
      - name : Install latest stable toolchain
        uses : Wandalen/wretry.action@master
        with :
          action : actions-rs/toolchain@v1
          with : |
            toolchain : stable
            override : true
          attempt_limit : 3
          attempt_delay: 10000
      - uses : actions/checkout@v3
      - name : Run tests in release mode
        run : cargo test --manifest-path ${{ inputs.manifest_path }} --release
      - name : Install latest nightly toolchain
        uses : Wandalen/wretry.action@master
        with :
          action : actions-rs/toolchain@v1
          with : |
            toolchain : nightly
            override : true
          attempt_limit : 3
          attempt_delay: 10000
      - name : Install cargo-hack
        run : cargo install cargo-hack
      - name : Run tests with each feature
        run : cargo hack test --manifest-path ${{ inputs.manifest_path }} --each-feature
        "#.into()
    }

    fn standard_rust_scheduled() -> String
    {
        r#"
        name : rust_scheduled

on :
  schedule :
    - cron : '0 1 * * *'

env :

  RUST_BACKTRACE : 1
  CARGO_TERM_COLOR : always
  WITH_SMOKE : ${{ inputs.with_smoke }}

jobs :

  checkmate :
    runs-on : ubuntu-latest
    steps :
      - name : Install latest nightly toolchain
        uses : Wandalen/wretry.action@master
        with :
          action : actions-rs/toolchain@v1
          with : |
            toolchain : nightly
            override : true
            components : clippy
          attempt_limit : 3
          attempt_delay: 10000
      - uses : actions/checkout@v3
        with :
          ref : alpha

      - name : Install cargo-audit
        run : cargo install cargo-audit
      - name : Install cargo-udeps
        run : cargo install cargo-udeps --locked

      - name : Audit the modules
        run : make audit
        continue-on-error : true
      - name : Generate documentation for the modules
        run : make doc open=no
        continue-on-error : true
      - name : Lint the modules
        run : make lint warnings=no
        continue-on-error : true
      - name : Check the modules
        run : make check
        continue-on-error : true
      - name : Check the modules dependencies
        run : cargo +nightly udeps --all-targets
        continue-on-error : true

  release :
    strategy :
      fail-fast : false
      matrix :
        os : [ ubuntu-latest, windows-latest, macos-latest ]
    runs-on : ${{ matrix.os }}
    steps :
      - name : Install latest stable toolchain
        uses : Wandalen/wretry.action@master
        with :
          action : actions-rs/toolchain@v1
          with : |
            toolchain : stable
            override : true
          attempt_limit : 3
          attempt_delay: 10000
      - uses : actions/checkout@v3
        with :
          ref : alpha

      - name : Make release build
        run : cargo build --release

  miri :
    runs-on : ubuntu-latest
    steps :
      - name : Install latest nightly toolchain
        uses : Wandalen/wretry.action@master
        with :
          action : actions-rs/toolchain@v1
          with : |
            toolchain : nightly
            override : true
            components : miri
          attempt_limit : 3
          attempt_delay: 10000
      - uses : actions/checkout@v3
        with :
          ref : alpha

      - name : Test with miri
        run : cargo miri test

        "#.into()
    }

    fn standard_rust_status() -> String
    {
        r#"
        
name : rust_status

on:
  workflow_run:
    workflows: [ auto_merge_to_beta, rust_scheduled ]
    types:
      - completed

concurrency :

  group : standard_rust_status
  cancel-in-progress : true

jobs :

  runs_check :
    strategy :
      matrix :
        modules : [ 'AutoPrToBeta', 'StandardRustScheduled' ]
    runs-on : ubuntu-latest
    steps :
      - name : Check workflow run status
        id : check_ci
        uses :  ronymeyer/workflow-status@v0.3.7
        with :
          token : ${{ secrets.GITHUB_TOKEN }}
          workflow : ${{ matrix.modules }}.yml
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


        "#.into()
    }

    fn status_checks_rules_update() -> String
    {
      r#"
      
name : status_checks_rules_update

on :
  pull_request :
    types : [ opened ]
    branches : [ alpha, beta ]

concurrency :

  group : projected_rules_update
  cancel-in-progress : true

jobs :

  check_workflows :
    if : ${{ github.event.pull_request.base.ref == 'beta' }}
    outputs :
      should_update : ${{ steps.compare.outputs.not_equal }}
    runs-on : ubuntu-latest
    steps :
      - name : Compare workflow directories content
        id : compare
        run : |
          files_beta=$(curl -X GET -G \
            -H "Accept: application/vnd.github+json" \
            -H "Authorization: token ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}" \
            https://api.github.com/repos/${{ github.repository }}/contents/.github/workflows \
            -d 'ref=beta')
          files_alpha=$(curl -X GET -G \
            -H "Accept: application/vnd.github+json" \
            -H "Authorization: token ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}" \
            https://api.github.com/repos/${{ github.repository }}/contents/.github/workflows \
            -d 'ref=alpha')

          if [[ "$files_beta" == "$files_alpha" ]] ; then
            echo "not_equal=false" >> $GITHUB_OUTPUT
          else
            echo "not_equal=true" >> $GITHUB_OUTPUT
          fi

  update_beta :
    needs : check_workflows
    if : ${{ needs.check_workflows.outputs.should_update == 'true' }}
    runs-on : ubuntu-latest
    steps :
      - uses: actions/checkout@v3
      - name : Get options
        id : options_get
        run : |
          WORKFLOWS=$(ls .github/workflows | grep Module)
          for WORKFLOW in $WORKFLOWS ; do
          CONTEXT=$(echo $WORKFLOW | sed 's/\(\S\+\).yml/{"context":"check (\1)","app_id":null}/')
            CONTEXTS="$CONTEXTS,$CONTEXT"
          done;
          CHECKS="[$(sed 's/^,//g' <<< $CONTEXTS),{\"context\":\"runs_check\",\"app_id\":null}]"
          echo "options={\"required_status_checks\":{\"strict\":false,\"checks\":$CHECKS},\"enforce_admins\":false,\"required_pull_request_reviews\":null,\"restrictions\":null}" >> $GITHUB_OUTPUT
      - name : Setup rules for beta
        run : |
          curl -X PUT \
            -H "Accept: application/vnd.github+json" \
            -H "Authorization: token ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}" \
            https://api.github.com/repos/${{ github.repository }}/branches/beta/protection \
            -d '${{ steps.options_get.outputs.options }}'

  update_alpha :
    if : ${{ github.event.pull_request.base.ref == 'alpha' }}
    runs-on : ubuntu-latest
    steps :
      - name : Setup rules for alpha
        run : |
          CHECKS='[{"context":"tested / fast (ubuntu-latest)","app_id":null},{"context":"tested / fast (windows-latest)","app_id":null},{"context":"tested / fast (macos-latest)","app_id":null}]'
          curl -X PUT \
            -H "Accept: application/vnd.github+json" \
            -H "Authorization: token ${{ secrets.PRIVATE_GITHUB_BOT_TOKEN }}" \
            https://api.github.com/repos/${{ github.repository }}/branches/alpha/protection \
            -d "{\"required_status_checks\":{\"strict\":false,\"checks\":$CHECKS},\"enforce_admins\":false,\"required_pull_request_reviews\":null,\"restrictions\":null}"

      "#.into()
    }

    
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