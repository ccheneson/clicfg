pub fn print_help() -> () {
    let exec_name = std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();

    let man = format!("
    usage: {exec} project env action parameter

    example: {exec} api staging ls
	     {exec} api staging get
	     {exec} api staging put 2.1.13.conf

    project choice : api, registration, authentication, authorization
    env choice : live, staging , hotfix
    action choice : get, put, ls, cat, lslo (ls local)
    parameter 1, 2, ... N : parameters for the action
    ", exec = exec_name);
    println!("{}", man)
}
