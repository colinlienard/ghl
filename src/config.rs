use colored::*;
use inquire::{validator::Validation, Confirm, InquireError, Text};

const PR_PREFIX: [&str; 9] = [
    "Add ", "Clean ", "Fix ", "Improve ", "Remove ", "Update ", "Rework ", "Ignore ", "Bump ",
];

pub struct Config {
    pub pr_name: String,
    pub branch: String,
}

impl Config {
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
