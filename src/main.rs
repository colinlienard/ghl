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
                Ok(_) => println!("{}", "Successfully set the token.".bright_green()),
                Err(e) => {
                    eprintln!("{}", e.to_string().red());
                    return;
                }
            };
            match Config::set_default_desc() {
                Ok(_) => println!(
                    "{}",
                    "Successfully set the default pull request description.".bright_green()
                ),
                Err(e) => {
                    eprintln!("{}", e.to_string().red());
                }
            };
            return;
        }
        "help" | _ => {
            println!("Usage: gitpr");
            println!("       gitpr help");
            println!("       gitpr version");
            println!("");
            println!("Options:");
            println!("  help     Display this message");
            println!("  version  Display the version");
            return;
        }
    }

    match Config::get_github_token() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("{}", "Please set the token with `snp config`.".bright_red());
            return;
        }
    };

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

    let gh = Github::new(Config::get_github_token().unwrap().as_str());
    let pr_url = match Github::create_pr(&gh, config).await {
        Ok(url) => {
            println!(
                "{}",
                "Successfully created the pull request.".bright_green()
            );
            url
        }
        Err(_) => {
            eprintln!("An error occured");
            process::exit(1);
        }
    };

    let username = match Github::get_username(&gh).await {
        Ok(username) => {
            println!("{}", "Successfully get the current user.".bright_green());
            username
        }
        Err(_) => {
            eprintln!("An error occured");
            process::exit(1);
        }
    };

    let pr_number = pr_url.split("/").last().unwrap();
    match Github::assign_to_pr(&gh, &username, pr_number).await {
        Ok(_) => println!("{}", "Successfully assigned you.".bright_green()),
        Err(_) => {
            eprintln!("An error occured");
            process::exit(1);
        }
    };

    println!("{}", pr_url);
}
