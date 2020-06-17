use std::fs;

use crate::commands::Command;
use crate::envs::Environment;
use crate::projects::Project;


#[derive(Debug)]
pub struct CliConfig {
    project: Project,
    env: Environment,
    pub cmd: Command,
}

impl CliConfig {
    pub fn new(project: Project, env: Environment, cmd: Command) -> Self {
        Self {
            project,
            env,
            cmd,
        }
    }

    /**
    Same as s3_path, but ending with "/"
    */
    pub fn s3_dir(&self) -> String {
        format!("{}/", self.s3_path())
    }

    pub fn s3_path(&self) -> String {
        format!("{project}/{environment}", project = self.project.name().to_lowercase(), environment = self.env.to_string().to_lowercase())
    }

    pub fn local_path(&self) -> String {
        let path = format!("{home_dir}/conf/{path}", home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_lowercase(), path = self.s3_path().to_lowercase());
        fs::create_dir_all(&path).expect("Fail to create folder to store conf");
        path
    }
}

