//! Binary entrypoint for the `verdant` scene packer, asset/shader oracle, and benchmark CLI.

#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match verdant_cli::run_cli(&args) {
        Ok(report) => {
            if !report.is_empty() {
                println!("{report}");
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("verdant: {err}");
            ExitCode::FAILURE
        }
    }
}
