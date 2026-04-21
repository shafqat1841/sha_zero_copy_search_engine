mod errors;
mod file_mmap;
mod log_searcher;
mod search_result;

use memmap2::Mmap;

use crate::{errors::RunErr, file_mmap::FileMmap, log_searcher::LogSearcher};

pub fn run() -> Result<(), RunErr> {
    let path: &str = "./log_files/access.log";
    let query = "uplherc.upl.com";
    let limit: usize = 1000;

    let file_buffer: FileMmap = FileMmap::new(path)?;

    let bytes: &Mmap = file_buffer.get_bytes();

    let log_searcher = LogSearcher::new(bytes);

    let result = log_searcher.search(query, limit)?;

    for search_result in result {
        println!("{}", search_result)
    }

    Ok(())
}
