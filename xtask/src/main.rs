//! Developer tasks for tuxtui workspace

use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "fmt" => run_fmt(),
        "lint" => run_lint(),
        "test" => run_test(),
        "doc" => run_doc(),
        "check" => run_check(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("xtask - Developer tasks for tuxtui");
    println!();
    println!("USAGE:");
    println!("    cargo xtask <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("    fmt     Format all code");
    println!("    lint    Run clippy");
    println!("    test    Run all tests");
    println!("    doc     Build documentation");
    println!("    check   Run all checks (fmt, lint, test)");
}

fn run_fmt() {
    println!("Formatting code...");
    let status = Command::new("cargo")
        .args(["fmt", "--all"])
        .status()
        .expect("Failed to run cargo fmt");
    
    if !status.success() {
        std::process::exit(1);
    }
}

fn run_lint() {
    println!("Running clippy...");
    let status = Command::new("cargo")
        .args(["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
        .status()
        .expect("Failed to run cargo clippy");
    
    if !status.success() {
        std::process::exit(1);
    }
}

fn run_test() {
    println!("Running tests...");
    let status = Command::new("cargo")
        .args(["test", "--workspace"])
        .status()
        .expect("Failed to run cargo test");
    
    if !status.success() {
        std::process::exit(1);
    }
}

fn run_doc() {
    println!("Building documentation...");
    let status = Command::new("cargo")
        .args(["doc", "--workspace", "--no-deps"])
        .status()
        .expect("Failed to run cargo doc");
    
    if !status.success() {
        std::process::exit(1);
    }
}

fn run_check() {
    println!("Running all checks...");
    run_fmt();
    run_lint();
    run_test();
    run_doc();
    println!("All checks passed!");
}
