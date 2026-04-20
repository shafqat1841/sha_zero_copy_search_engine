mod errors;

use std::{fs::File, str::from_utf8};

use memmap2::Mmap;

use crate::errors::RunErr;

#[derive(Debug)]
struct SearchResult<'a> {
    line: &'a str,       // Reference to the full line in the NASA log
    ip_address: &'a str, // Reference to just the IP part of that line
}

pub fn run() -> Result<(), RunErr> {
    let address_to_find = "uplherc.upl.com";

    let file = File::open("./log_files/access.log")?;

    let mmap = unsafe { Mmap::map(&file)? };

    let bytes: &[u8] = &mmap;

    let mut result: Vec<SearchResult> = Vec::new();

    for (index, line_bytes) in bytes.split(|b| *b == b'\n').enumerate() {
        let line_number = index + 1;
        // for development we are just checking 1000 lines only
        if line_number >= 1000 {
            break;
        }
        match from_utf8(line_bytes) {
            Err(err) => {
                println!("Line {}, error: {}", line_number, err);
                continue;
            }
            Ok(line_str) => {
                let byte_index_option = line_str.find(" - - ");
                match byte_index_option {
                    None => {
                        continue;
                    }
                    Some(byte_index) => {
                        let ip_address = &line_str[..byte_index];
                        let search_result = SearchResult {
                            line: line_str,
                            ip_address,
                        };

                        result.push(search_result);
                    }
                }
            }
        }
    }

    for search_result in result {
        println!("ip_address : {:?} ", search_result.ip_address);
        println!("line : {:?} ", search_result.line);
    }

    Ok(())
}
