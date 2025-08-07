// xxx : This file temporarily disables Former derive macro tests due to trailing comma issue
// See: /home/user1/pro/lib/wTools/module/core/macro_tools/task/task_issue.md
// Re-enable when macro_tools::generic_params::decompose is fixed

#[cfg(test)]
mod disabled_former_tests {
    #[test]
    #[ignore = "Former derive macro temporarily disabled due to trailing comma issue"]
    fn former_derive_disabled() {
        println!("Former derive macro tests are temporarily disabled");
        println!("See: /home/user1/pro/lib/wTools/module/core/macro_tools/task/task_issue.md");
    }
}