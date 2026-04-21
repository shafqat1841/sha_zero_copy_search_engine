use std::fs::File;

use memmap2::Mmap;

use crate::errors::RunErr;


pub struct FileMmap {
    mmap: Mmap,
}

impl FileMmap {
    pub fn new(path: &str) -> Result<Self, RunErr> {
        let file = File::open(path)?;

        let mmap: Mmap = unsafe { Mmap::map(&file)? };

        let res = Self { mmap };

        Ok(res)
    }

    pub fn get_bytes(&self) -> &Mmap {
        &self.mmap
    }
}