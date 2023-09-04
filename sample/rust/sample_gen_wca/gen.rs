#!/bin/sh
//usr/bin/env rustc $0 -o ./gen.bin -Cdebuginfo=1 --edition 2021 -Cpanic=abort && ./gen.bin $@ && rm ./gen.bin ; exit

fn main() {
    let count = std::env::args().nth(1).unwrap().parse::<u64>().unwrap();
    let commands = (0..count)
        .map(|i| format!("command(cmd.into_builder().name(\"name_{i}\"))"))
        .collect::<Vec<_>>()
        .join("\n        .");

    let code = format!(
        "
use wca::stdx::{{cli, IntoBuilder}};

fn cmd((): (), _args: wca::Args, _props: wca::Props) -> Result<(), wca::BasicError> {{
    Ok(())
}}
    
fn main() {{
    let memory_usage = ra_ap_profile::memory_usage();
    let aggregator = cli(()).{commands}.build();
    println!(\"{{}}\", ra_ap_profile::memory_usage() - memory_usage);
    let args = std::env::args().skip(1).collect::<Vec<_>>().join(\" \");
    aggregator.perform(args).unwrap();
}}"
    );

    let _ = std::fs::create_dir("src");
    std::fs::write("src/main.rs", code.trim()).unwrap();
}
