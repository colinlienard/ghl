use inquire::{validator::Validation, InquireError, Text};

const PR_PREFIX: [&str; 9] = [
    "Add", "Clean", "Fix", "Improve", "Remove", "Update", "Rework", "Ignore", "Bump",
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

        Ok(Config {
            pr_name,
            branch: linear_branch,
        })
    }
}
