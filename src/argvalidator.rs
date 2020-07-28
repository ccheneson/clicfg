#[macro_export]
macro_rules! validate_args {

        ($($elem: expr),+) => {{
        let mut v = Vec::new();
        $(
            let b = $elem.validate();
            v.push(b);
        )*
        let result : Vec<&bool> = v.iter().filter(|arg| **arg == false).collect();
        if result.len()  == 0 {
            Ok(())
        } else {
            help::print_help();
            Err(CliArgError("Missing argument(s)".to_owned()))
        }
        }}
    }

//https://stackoverflow.com/questions/34214136/how-do-i-match-the-type-of-an-expression-in-a-rust-macro
pub trait ValidateArg {
    fn validate(&self) -> bool;
}

impl ValidateArg for Option<&String> {
    fn validate(&self) -> bool {
        self.is_some()
    }
}

