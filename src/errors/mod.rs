use std::str::Utf8Error;

#[derive(Debug)]
pub enum RunErr {
    IOError(String),
    Utf8Error(String),
    EmptyQueryErr(String),
    EmptyPathErr(String),
}

impl From<std::io::Error> for RunErr {
    fn from(value: std::io::Error) -> Self {
        RunErr::IOError(value.to_string())
    }
}

impl From<Utf8Error> for RunErr {
    fn from(value: Utf8Error) -> Self {
        RunErr::Utf8Error(value.to_string())
    }
}
