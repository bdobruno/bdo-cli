use std::env;
use std::process::ExitCode;

mod scripts;

const CLI_NAME: &str = "bdo";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let result = match args.first().map(String::as_str) {
        Some("create") => match args.get(1) {
            Some(repo_name) => scripts::create::run(repo_name),
            None => {
                eprintln!("Usage: {CLI_NAME} create <repo-name>");
                Err("missing repository name".into())
            }
        },
        Some("push") => scripts::push::run(),
        Some("branch") => match args.get(1) {
            Some(branch_name) => scripts::branch::run(branch_name),
            None => {
                eprintln!("Usage: {CLI_NAME} branch <branch-name>");
                Err("missing branch name".into())
            }
        },
        Some("checkout") => scripts::checkout::run(),
        Some("pr") => scripts::pr::run(),
        Some("help" | "--help" | "-h") | None => {
            show_help();
            Ok(())
        }
        Some(command) => {
            eprintln!("Unknown command: {command}");
            eprintln!("Run '{CLI_NAME} help' to see available commands");
            Err("unknown command".into())
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn show_help() {
    println!("Available commands:");
    println!("  {CLI_NAME} create <repo-name> - Create and initialize a new repository");
    println!("  {CLI_NAME} push - Add, commit, and push changes");
    println!("  {CLI_NAME} branch <branch-name> - Create and push a new branch");
    println!("  {CLI_NAME} checkout - Reset to match remote main branch");
    println!("  {CLI_NAME} pr - Open a pull request for the current branch");
    println!("  {CLI_NAME} help - Show this help message");
}
