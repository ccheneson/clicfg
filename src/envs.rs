use strum_macros::Display;

use crate::errors::CliConfigError;
use crate::errors::CliConfigError::CliArgError;

use crate::help;

#[derive(Debug, Copy, Clone, Display)]
pub enum Environment {
    Live,
    Staging,
    Hotfix
}

impl Environment {
    pub fn from_cli(env: Option<&String>) -> Result<Environment, CliConfigError> {
        let cli_env = match env {
            Some(v) => v,
            None => {
                help::print_help();
                return Err(CliArgError("Missing environment".to_owned()));
            }
        };
        let env = match cli_env.as_str() {
            "live" => Environment::Live,
            "staging" => Environment::Staging,
            "hotfix" => Environment::Hotfix,
            _ => {
                help::print_help();
                return Err(CliArgError("Invalid environment".to_owned()));
            }
        };
        return Ok(env);
    }
}
