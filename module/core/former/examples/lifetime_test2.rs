// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// See: /home/user1/pro/lib/wTools/module/core/macro_tools/task/task_issue.md
compile_error!("This example is temporarily disabled due to trailing comma issue in Former derive macro");

use former::Former;

#[derive(Debug, PartialEq, Former)]
pub struct Other<'x> {
  data: &'x str,
}

fn main() {
  let s = "hello";
  let instance = Other::former().data(s).form();
  println!("{:?}", instance);
}