use strum_macros::Display;

use crate::errors::CliConfigError;
use crate::errors::CliConfigError::CliArgError;
use crate::help;

#[derive(Debug, Display)]
pub enum Project {
    Registration,
    Authentication,
    Authorization,
    Api
}

impl Project {
    pub fn from_cli(env: Option<&String>) -> Result<Project, CliConfigError> {
        let cli_project = match env {
            Some(v) => v,
            None => {
                help::print_help();
                return Err(CliArgError("Missing project".to_owned()));
            }
        };
        let project = match cli_project.as_str() {
            "registration" => Project::Registration,
            "authentication" => Project::Authentication,
            "authorization" => Project::Authorization,
            "api" => Project::Api,
            _ => {
                help::print_help();
                return Err(CliArgError("Invalid project".to_owned()));
            }
        };
        return Ok(project);
    }
}
