mod errors;
mod file_mmap;
mod log_searcher;
mod search_result;

use memmap2::Mmap;

use crate::{errors::RunErr, file_mmap::FileMmap, log_searcher::LogSearcher};

pub fn run() -> Result<(), RunErr> {
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

    let query: &str = &query_val;

    let path_val = match path_opt {
        None => {
            return Err(RunErr::EmptyPathErr(
                "Error: Empty path argument".to_string(),
            ));
        }
        Some(value) => value,
    };

    // let path: &str = "./log_files/access.log";
    let path: &str = &path_val;

    let file_buffer: FileMmap = FileMmap::new(path)?;

    let bytes: &Mmap = file_buffer.get_bytes();

    let log_searcher: LogSearcher = LogSearcher::new(bytes);

    let result = log_searcher.search(query, limit)?;

    for search_result in result {
        println!("{}", search_result)
    }

    Ok(())
}
