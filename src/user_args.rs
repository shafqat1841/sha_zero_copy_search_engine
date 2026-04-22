use std::path::PathBuf;

use crate::errors::RunErr;

pub struct UserArgs {
    pub path: PathBuf,
    pub query: String,
    pub limit: usize,
}

impl UserArgs {
    pub fn new() -> Result<Self, RunErr> {
        let mut args = std::env::args();
        args.next();
        let path_opt: Option<String> = args.next();
        let query_opt: Option<String> = args.next();
        let limit_opt: Option<String> = args.next();

        let mut limit: usize = 1000;

        if let Some(limit_val) = limit_opt {
            if let Ok(value) = limit_val.parse() {
                limit = value
            }
        }

        let query_val = match query_opt {
            None => {
                return Err(RunErr::EmptyQueryErr(
                    "Error: Empty query argument".to_string(),
                ));
            }
            Some(value) => value,
        };

        let query = query_val;

        let path_val = match path_opt {
            None => {
                return Err(RunErr::EmptyPathErr(
                    "Error: Empty path argument".to_string(),
                ));
            }
            Some(value) => value,
        };

        // let path: &str = "./log_files/access.log";
        let path: PathBuf = PathBuf::from(path_val);

        if !path.exists() {
            return Err(RunErr::WrongPathErr(
                "Error: Path does not exists".to_string(),
            ));
        }

        Ok(Self { path, query, limit })
    }
}
