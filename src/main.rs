use colored::*;
use config::Config;
use github::Github;
use std::{env, process};

mod config;
mod git;
mod github;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = if args.len() > 1 { &args[1] } else { "" };
    match arg {
        "create" => {}
        "config" => {
            match Config::set_github_token() {
                Ok(set) => match set {
                    true => println!("{}", "✔ Token set.".green()),
                    false => println!("{}", "Skipped.".dimmed()),
                },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            };
            match Config::set_default_desc() {
                Ok(set) => match set {
                    true => println!("{}", "✓ Default pull request description set.".green()),
                    false => println!("{}", "Skipped.".dimmed()),
                },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
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

    let pr_url = create(&github_token).await.unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    println!(
        "🎉 Success! The pull request url is: {}",
        pr_url.replace('"', "").bright_cyan()
    );
}

async fn create(github_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::ask()?;

    match Config::confirm(&config)? {
        true => {}
        false => {
            return Err("Aborted.".into());
        }
    };

    git::create_branch(&config.branch)?;
    println!("{}", "✔ Branch created.".green());

    git::create_commit(&config.pr_name)?;
    println!("{}", "✔ Commit created.".green());

    git::push(&config.branch)?;
    println!("{}", "✔ Successfully pushed.".green());

    let gh = Github::new(github_token);

    let pr_url = gh.create_pr(config).await?;
    println!("{}", "✔ Pull request created.".green());

    let username = gh.get_username().await?;

    let pr_number = pr_url.split('/').last().unwrap();
    gh.assign_to_pr(&username, pr_number).await?;
    println!("{}", "✔ Successfully assigned you.".green());

    Ok(pr_url)
}
