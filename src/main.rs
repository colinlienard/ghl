use colored::*;
use config::Config;
use github::Github;
use home::home_dir;
use std::{env, error::Error, fs, process};
use utils::process_command;

mod config;
mod git;
mod github;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let arg = if args.len() > 1 { &args[1] } else { "" };

    match arg {
        "pr" => pr_command().await?,
        "config" => config_command()?,
        "version" | "-v" => version_command().await?,
        "update" | "-up" => update_command()?,
        _ => help_command(),
    }

    Ok(())
}

async fn pr_command() -> Result<(), Box<dyn Error>> {
    let github_token = Config::get_github_token().unwrap_or_else(|_| {
        eprintln!("Please set the token with `ghl config`.");
        process::exit(1);
    });

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

    let gh = Github::new(&github_token);

    let pr_url = gh.create_pr(config).await?;
    println!("{}", "✔ Pull request created.".green());

    let username = gh.get_username().await?;

    let pr_number = pr_url.split('/').last().unwrap();
    gh.assign_to_pr(&username, pr_number).await?;
    println!("{}", "✔ Successfully assigned you.".green());

    println!(
        "🎉 Success! The pull request url is: {}",
        pr_url.replace('"', "").bright_cyan()
    );

    open::that(pr_url.replace('"', ""))?;

    Ok(())
}

fn config_command() -> Result<(), Box<dyn Error>> {
    match Config::set_github_token()? {
        true => println!("{}", "✔ Token set.".green()),
        false => println!("{}", "Skipped.".dimmed()),
    };

    match Config::set_default_desc()? {
        true => println!("{}", "✓ Default pull request description set.".green()),
        false => println!("{}", "Skipped.".dimmed()),
    };

    Ok(())
}

async fn version_command() -> Result<(), Box<dyn Error>> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version:   {}", current_version);

    let latest_version = github::get_package_latest_version().await?;
    println!("Latest version:    {}\n", latest_version);

    if latest_version == current_version {
        println!("{}", "You are using the latest version.".green());
    } else {
        println!("{}", "You can update with `ghl update`".yellow());
    }

    Ok(())
}

fn update_command() -> Result<(), Box<dyn Error>> {
    process_command(process::Command::new("curl").args([
        "-o",
        "ghl",
        "-L",
        "https://github.com/colinlienard/ghl/releases/latest/download/ghl",
    ]))?;

    process_command(process::Command::new("chmod").args(["+x", "ghl"]))?;

    let home_dir = home_dir().unwrap();
    let target_path = home_dir.join(".local/bin/ghl");
    fs::rename("ghl", target_path)?;

    println!("{}", "✔ Updated successfully.".green());

    Ok(())
}

fn help_command() {
    println!("{}", "Usage".bold());
    println!("  ghl [command]");
    println!();
    println!("{}", "Commands".bold());
    println!("  help           Display this message.");
    println!("  config         Set the GitHub token and the default pull request description.");
    println!("  create, -c     Do the following:");
    println!("                   1. Create a new branch.");
    println!("                   2. Create a new commit.");
    println!("                   3. Push to the remote repository.");
    println!("                   4. Create a new pull request.");
    println!("                   5. Assign you the pull request.");
    println!("  version, -v    Display the current and the latest version.");
    println!("  update, -up    Update the binary to the latest version.");
}
