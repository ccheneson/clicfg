use std::{fs, io, env};

use rusoto_s3::S3Client;
use prettydiff::diff_chars;
use commands::Command;
use envs::Environment;
use errors::CliConfigError;
use projects::Project;
use s3::client;
use utils::*;
use std::process::Command as CmdShell ;

mod s3;
mod envs;
mod projects;
mod cli;
mod utils;
mod commands;
mod help;
mod errors;
mod argvalidator;


fn main() -> Result<(), CliConfigError> {
    let args: Vec<String> = std::env::args().collect();

    let project = Project::from_cli(args.get(1))?;
    let env = Environment::from_cli(args.get(2))?;

    let cmd = Command::from_cli(
        args.get(3),
        args.get(4),
        args.get(5),
    )?;

    let cli_config = cli::CliConfig::new(project, env, cmd);

    let s3_client = client::get_s3_client()?;


    match &cli_config.cmd {
        Command::Ls => {
            let s3_path = cli_config.s3_path();
            s3::cmd_ls(&s3_client, s3_path.as_str()).map(|output| for i in &output.contents.unwrap() {
                println!("{filename}", filename = str::replace(i.key.as_ref().unwrap(), cli_config.s3_dir().as_str(), ""));
            })?;
            Ok(())
        }
        Command::Get { file } => {
            let file_paths = build_path_file(&cli_config, &file);
            fetch_file(&s3_client, &file_paths.s3, &file_paths.local).map(|_| ())
        }
        Command::Cat { file } => {
            let file_paths = build_path_file(&cli_config, &file);
            fetch_file(&s3_client, &file_paths.s3, &file_paths.local).map(|target_local| {
                let content = fs::read_to_string(target_local);
                content.map(|c| println!("{}", c)).unwrap()
            })
        }
        Command::Edit { file } => {
            let file_paths = build_path_file(&cli_config, &file);
            let editor = env::var("PMCFG_EDITOR").unwrap_or_else(|_|"vi".to_owned());
            fetch_file(&s3_client, &file_paths.s3, &file_paths.local).map(|target_local| {
                CmdShell::new(editor)
                    .arg(target_local)
                    .spawn().expect("Can not open editor")
                    .wait().expect("Can not open editor");
            })?;
            s3::cmd_put(&s3_client, &file_paths.local, &file_paths.s3)?;
            Ok(())
        }
        Command::LsLo => {
            let local_path = cli_config.local_path();
            let mut paths = fs::read_dir(local_path).unwrap().map(|res| res.map(|e| e.file_name().into_string().unwrap()))
                .collect::<Result<Vec<_>, io::Error>>().unwrap();
            paths.sort();
            for file in paths {
                println!("{}", file)
            }
            Ok(())
        }
        Command::Diff { file_a, file_b } => {
            let paths_a = build_path_file(&cli_config, &file_a);
            let paths_b = build_path_file(&cli_config, &file_b);
            let _ = fetch_file(&s3_client, &paths_a.s3, &paths_a.local)?;
            let _ = fetch_file(&s3_client, &paths_b.s3, &paths_b.local)?;
            let txt_a = fs::read_to_string(paths_a.local).expect("Something went wrong reading the file");
            let txt_b = fs::read_to_string(paths_b.local).expect("Something went wrong reading the file");
            let change_set = diff_chars(txt_a.as_str(), txt_b.as_str());
            println!("{}", change_set);
            Ok(())
        }
        Command::Put { file } => {
            let file_paths = build_path_file(&cli_config, &file);
            s3::cmd_put(&s3_client, &file_paths.local, &file_paths.s3).map(|_| ())
        }
        Command::Bump { file_from, file_to } => {
            let file_from_paths = build_path_file(&cli_config, &file_from);
            let file_to_paths = build_path_file(&cli_config, &file_to);
            let _ = fetch_file(&s3_client, &file_from_paths.s3, &file_from_paths.local)?;
            println!("Bump {} to {}", file_from_paths.local, file_to_paths.local);
            fs::rename(&file_from_paths.local, &file_to_paths.local).unwrap();
            s3::cmd_put(&s3_client, &file_to_paths.local, &file_to_paths.s3).map(|_| ())
        }
    }
}


fn fetch_file<'a>(s3_client: &S3Client, s3_file: &str, save_to: &'a str) -> Result<&'a str, CliConfigError> {
    let local_file = s3::cmd_get(&s3_client, s3_file, save_to);
    local_file
}
