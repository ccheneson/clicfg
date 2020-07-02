use strum_macros::Display;

use crate::errors::CliConfigError;
use crate::errors::CliConfigError::CliArgError;
use crate::help;

//PartialEq : used for unit test
#[derive(Debug, Display)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Command {
    Ls,
    LsLo,
    Cat { file: String },
    Edit { file: String },
    Get { file: String },
    Put { file: String },
    Bump { file_from: String, file_to: String },
    Diff { file_a: String, file_b: String },
}

impl Command {
    //https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments
    fn exit_on_missing_arg(args: &[&Option<&String>]) -> Result<(), CliConfigError> {
        //x.clone because
        //https://users.rust-lang.org/t/why-fromiterator-string-is-not-implemented-for-vec-string/28931
        let a: Vec<&Option<&String>> = args.into_iter().filter(|arg| arg.is_none()).map(|x| x.clone()).collect();
        if a.len() > 0 {
            help::print_help();
            return Err(CliArgError("Missing argument".to_owned()));
        }
        Ok(())
    }

    pub fn from_cli(cmd: Option<&String>, arg1: Option<&String>, arg2: Option<&String>) -> Result<Command, CliConfigError> {
        let cli_cmd = cmd.ok_or_else(||{
            help::print_help();
            CliArgError("Missing command".to_owned())
        })?;
        let cmd = match cli_cmd.as_str() {
            "ls" => Command::Ls {},
            "lslo" => Command::LsLo {},
            "cat" => {
                Command::exit_on_missing_arg(&[&arg1])?;
                Command::Cat { file: arg1.unwrap().to_owned() }
            }
            "edit" => {
                Command::exit_on_missing_arg(&[&arg1])?;
                Command::Edit { file: arg1.unwrap().to_owned() }
            }
            "get" => {
                Command::exit_on_missing_arg(&[&arg1])?;
                Command::Get { file: arg1.unwrap().to_owned() }
            }
            "put" => {
                Command::exit_on_missing_arg(&[&arg1])?;
                Command::Put { file: arg1.unwrap().to_owned() }
            }
            "bump" => {
                Command::exit_on_missing_arg(&[&arg1, &arg2])?;
                Command::Bump { file_from: bump_to_conf(arg1.unwrap()), file_to: bump_to_conf(arg2.unwrap()) }
            }
            "diff" => {
                Command::exit_on_missing_arg(&[&arg1, &arg2])?;
                Command::Diff { file_a: arg1.unwrap().to_owned(), file_b: arg2.unwrap().to_owned() }
            }
            _ => {
                help::print_help();
                return Err(CliArgError("Invalid command".to_owned()));
            }
        };
        return Ok(cmd);
    }
}

fn bump_to_conf(version: &str) -> String {
    format!("{}.conf", version)
}


#[cfg(test)]
mod tests {
    use crate::commands::Command;
    use crate::errors::CliConfigError;

    #[test]
    fn check_cli_arguments() {
        let cmd = None;
        let arg1 = None;
        let arg2 = None;
        let result = Command::from_cli(cmd, arg1, arg2);
        assert_eq!(result.err().unwrap(), CliConfigError::CliArgError("Missing command".to_owned()));


        let bad_cmd = String::from("top");
        let cmd = Some(&bad_cmd);
        let result = Command::from_cli(cmd, arg1, arg2);
        assert_eq!(result.err().unwrap(), CliConfigError::CliArgError("Invalid command".to_owned()));

        let ok_cmd = String::from("cat");
        let cmd = Some(&ok_cmd);
        let result = Command::from_cli(cmd, arg1, arg2);
        assert_eq!(result.err().unwrap(), CliConfigError::CliArgError("Missing argument".to_owned()));

        let ok_cmd = String::from("cat");
        let ok_arg1 = String::from("1.10.1.conf");
        let cmd = Some(&ok_cmd);
        let arg1 = Some(&ok_arg1);
        let result = Command::from_cli(cmd, arg1, arg2);
        assert_eq!(result.ok().unwrap(), Command::Cat { file : ok_arg1 });

        let ok_cmd = String::from("bump");
        let ok_arg1 = String::from("1.10.1.conf");
        let cmd = Some(&ok_cmd);
        let arg1 = Some(&ok_arg1);
        let result = Command::from_cli(cmd, arg1, arg2);
        assert_eq!(result.err().unwrap(), CliConfigError::CliArgError("Missing argument".to_owned()));

        let ok_cmd = String::from("bump");
        let ok_arg1 = String::from("1.10.1");
        let ok_arg2 = String::from("5.5.5");
        let cmd = Some(&ok_cmd);
        let arg1 = Some(&ok_arg1);
        let arg2 = Some(&ok_arg2);
        let result = Command::from_cli(cmd, arg1, arg2);
        assert_eq!(result.ok().unwrap(), Command::Bump { file_from : "1.10.1.conf".to_owned(), file_to: "5.5.5.conf".to_owned() });
    }
}