use std::process;

use colored::*;
mod config;
use config::Config;
mod git;
use git::Git;

fn main() {
    let config = Config::ask().unwrap_or_else(|_| {
        eprintln!("An error occured");
        process::exit(1);
    });

    match Config::confirm(&config).unwrap_or_else(|_| {
        eprintln!("An error occured");
        process::exit(1);
    }) {
        true => {}
        false => {
            println!("{}", "Successfully aborted.".bright_green());
            return;
        }
    }

    match Git::create_branch(&config.branch) {
        Ok(_) => println!("{}", "Successfully created the branch.".bright_green()),
        Err(e) => {
            eprintln!("{}", e.to_string().red());
            return;
        }
    };

    match Git::create_commit(&config.pr_name) {
        Ok(_) => println!("{}", "Successfully created the commit.".bright_green()),
        Err(e) => {
            eprintln!("{}", e.to_string().red());
            return;
        }
    };

    match Git::push(&config.branch) {
        Ok(_) => println!("{}", "Successfully pushed.".bright_green()),
        Err(e) => {
            eprintln!("{}", e.to_string().red());
            return;
        }
    };
}
