mod errors;
mod file_mmap;

use std::{str::from_utf8};

use memmap2::Mmap;

use crate::{errors::RunErr, file_mmap::FileMmap};

#[derive(Debug)]
struct SearchResult<'a> {
    line: &'a str,       // Reference to the full line in the NASA log
    ip_address: &'a str, // Reference to just the IP part of that line
}

pub fn run() -> Result<(), RunErr> {
    let address_to_find = "uplherc.upl.com";

    let file_buffer: FileMmap = FileMmap::new()?;

    let bytes: &Mmap = file_buffer.get_bytes();

    let bytes_split = bytes.split(|b| *b == b'\n');

    let filter_map_fun = |line_bytes| {
        let line_str = from_utf8(line_bytes).ok()?;

        let start_idx = line_str.find(address_to_find)?;

        let end_idx = start_idx + address_to_find.len();
        let ip_address = line_str.get(start_idx..end_idx)?;

        Some(SearchResult {
            line: line_str,
            ip_address,
        })
    };

    let result: Vec<SearchResult> = bytes_split
        // for development we are just checking 1000 lines only
        .take(1000)
        .filter_map(filter_map_fun)
        .collect();

    for search_result in result {
        println!("ip_address : {:?} ", search_result.ip_address);
        println!("line : {:?} ", search_result.line);
    }

    Ok(())
}
