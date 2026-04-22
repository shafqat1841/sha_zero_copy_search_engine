mod errors;
mod file_mmap;
mod log_searcher;
mod search_result;
mod user_args;

use memmap2::Mmap;

use crate::{errors::RunErr, file_mmap::FileMmap, log_searcher::LogSearcher, user_args::UserArgs};

pub fn run() -> Result<(), RunErr> {
    let user_args = UserArgs::new()?;

    let file_buffer: FileMmap = FileMmap::new(&user_args.path)?;

    let bytes: &Mmap = file_buffer.get_bytes();

    let log_searcher: LogSearcher = LogSearcher::new(bytes);

    let result = log_searcher.search(&user_args.query, user_args.limit)?;

    for search_result in result {
        println!("{}", search_result)
    }

    Ok(())
}
