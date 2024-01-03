use colored::*;
use std::{env, process};
mod config;
use config::Config;
mod git;
use git::Git;
mod github;
use github::Github;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = if args.len() > 1 { &args[1] } else { "" };
    match arg {
        "create" => {}
        "config" => {
            match Config::set_github_token() {
                Ok(set) => {
                    if set {
                        println!("{}", "✔️ Token set.".green())
                    } else {
                        println!("{}", "Skipped.".dimmed())
                    }
                }
                Err(e) => {
                    eprintln!("{}", e.to_string().red());
                    return;
                }
            };
            match Config::set_default_desc() {
                Ok(set) => {
                    if set {
                        println!("{}", "✓ Default pull request description set.".green());
                    } else {
                        println!("{}", "Skipped.".dimmed());
                    }
                }
                Err(e) => eprintln!("{}", e.to_string().red()),
            };
            return;
        }
        _ => {
            println!("{}", "Usage".bold());
            println!("  ghl [command]");
            println!();
            println!("{}", "Commands".bold());
            println!("  help        Display this message.");
            println!(
                "  config      Set the GitHub token and the default pull request description."
            );
            println!("  create      Do the following:");
            println!("                1. Create a new branch.");
            println!("                2. Create a new commit.");
            println!("                3. Push to the remote repository.");
            println!("                4. Create a new pull request.");
            println!("                5. Assign you the pull request.");
            return;
        }
    }

    let github_token = Config::get_github_token().unwrap_or_else(|_| {
        eprintln!("Please set the token with `ghl config`.");
        process::exit(1);
    });

    let config = Config::ask().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    match Config::confirm(&config).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    }) {
        true => {}
        false => return,
    }

    match Git::create_branch(&config.branch) {
        Ok(_) => println!("{}", "✔️ Branch created.".green()),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    match Git::create_commit(&config.pr_name) {
        Ok(_) => println!("{}", "✔️ Commit created.".green()),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    match Git::push(&config.branch) {
        Ok(_) => println!("{}", "✔️ Successfully pushed.".green()),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let gh = Github::new(&github_token);
    let pr_url = match Github::create_pr(&gh, config).await {
        Ok(url) => {
            println!("{}", "✔️ Pull request created.".green());
            url
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let username = match Github::get_username(&gh).await {
        Ok(username) => username,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let pr_number = pr_url.split('/').last().unwrap();
    match Github::assign_to_pr(&gh, &username, pr_number).await {
        Ok(_) => println!("{}", "✔️ Successfully assigned you.".green()),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!(
        "🎉 {} The pull request url is: {}",
        "Success!".green(),
        pr_url.bright_cyan()
    );
}
