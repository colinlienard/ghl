use home::home_dir;
use std::{fs, io::Error};

use colored::*;
use inquire::{validator::Validation, Confirm, Editor, InquireError, Text};

const PR_PREFIX: [&str; 9] = [
    "Add ", "Clean ", "Fix ", "Improve ", "Remove ", "Update ", "Rework ", "Ignore ", "Bump ",
];

pub struct Config {
    pub pr_name: String,
    pub branch: String,
}

impl Config {
    fn get_paths() -> (String, String, String) {
        let home = home_dir().unwrap();
        let home = home.to_str().unwrap();
        let dir_path = home.to_owned() + "/.snp";
        let token_path = dir_path.to_owned() + "/token";
        let default_desc_path = dir_path.to_owned() + "/desc.md";
        (dir_path, token_path, default_desc_path)
    }

    pub fn set_github_token() -> Result<(), InquireError> {
        let token = Text::new("Github token:").prompt_skippable()?;
        let token = match token {
            Some(token) => {
                if token.is_empty() {
                    return Ok(());
                }
                token
            }
            None => return Ok(()),
        };

        let (dir_path, token_path, _) = Config::get_paths();

        match fs::read_dir(&dir_path) {
            Ok(_) => {}
            Err(_) => {
                fs::create_dir(&dir_path)?;
            }
        };
        match fs::read(&token_path) {
            Ok(_) => {}
            Err(_) => {
                fs::File::create(&token_path)?;
            }
        }
        fs::write(&token_path, token)?;

        Ok(())
    }

    pub fn get_github_token() -> Result<String, Error> {
        let (_, token_path, _) = Config::get_paths();
        let token = fs::read_to_string(&token_path)?;
        Ok(token)
    }

    pub fn set_default_desc() -> Result<(), InquireError> {
        let actual = match Config::get_default_desc() {
            Ok(desc) => desc,
            Err(_) => String::new(),
        };
        let desc = Editor::new("Pull request description")
            .with_predefined_text(&actual)
            .prompt()?;

        let (dir_path, _, default_desc_path) = Config::get_paths();

        match fs::read_dir(&dir_path) {
            Ok(_) => {}
            Err(_) => {
                fs::create_dir(&dir_path)?;
            }
        };
        match fs::read(&default_desc_path) {
            Ok(_) => {}
            Err(_) => {
                fs::File::create(&default_desc_path)?;
            }
        }
        fs::write(&default_desc_path, desc)?;

        Ok(())
    }

    pub fn get_default_desc() -> Result<String, Error> {
        let (_, _, default_desc_path) = Config::get_paths();
        let default_desc = fs::read_to_string(&default_desc_path)?;
        Ok(default_desc)
    }

    pub fn ask() -> Result<Config, InquireError> {
        let not_empty_validator = |value: &str| match value.is_empty() {
            true => Ok(Validation::Invalid("You must enter a value.".into())),
            false => Ok(Validation::Valid),
        };

        let pr_name_validator =
            |value: &str| match PR_PREFIX.iter().any(|current| value.starts_with(current)) {
                true => Ok(Validation::Valid),
                false => Ok(Validation::Invalid("TODO".into())),
            };

        let linear_branch = Text::new("Linear branch name:")
            .with_validator(not_empty_validator)
            .prompt()?;

        let pr_name = Text::new("Pull request name:")
            .with_validators(&[Box::new(not_empty_validator), Box::new(pr_name_validator)])
            .prompt()?;

        let prefix = pr_name.split(' ').collect::<Vec<&str>>()[0];
        let branch_prefix = match prefix {
            "Add" => "feature",
            "Clean" => "rework",
            "Fix" => "fix",
            "Improve" => "rework",
            "Remove" => "feature",
            "Update" => "feature",
            "Rework" => "rework",
            "Ignore" => "feature",
            "Bump" => "core",
            &_ => todo!(),
        };
        let branch = branch_prefix.to_owned() + "/" + &linear_branch;

        Ok(Config { pr_name, branch })
    }

    pub fn confirm(config: &Config) -> Result<bool, InquireError> {
        println!(
            "\
{}
{} {}
{}
{}
{} {}
{}",
            "This will:".bright_cyan(),
            "- Create a branch called".bright_cyan(),
            config.branch.bright_cyan().italic(),
            "- Create an empty commit".bright_cyan(),
            "- Push".bright_cyan(),
            "- Create a PR named".bright_cyan(),
            config.pr_name.bright_cyan().italic(),
            "- TODO...".bright_cyan()
        );
        Confirm::new("Confirm? (y/n)").prompt()
    }
}