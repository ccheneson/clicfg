use cli::CliConfig;

use crate::cli;

pub fn build_path_file(cli_config: &CliConfig, file: &str) -> PathsConf {
    let local_path = format!("{}/{}", &cli_config.local_path(), file);
    let s3_path = format!("{}/{}", &cli_config.s3_path(), file);
    PathsConf {
        local: local_path,
        s3: s3_path,
    }
}


pub struct PathsConf {
    pub local: String,
    pub s3: String,
}