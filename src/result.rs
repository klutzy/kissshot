use std::error::Error;
use std::io;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SshErrorKind {
    IoFailure,
}

#[derive(Debug)]
pub struct SshError {
    pub kind: SshErrorKind,
    pub desc: String,
}

impl SshError {
    pub fn new<T>(kind: SshErrorKind, desc: String) -> SshResult<T> {
        Err(SshError {
            kind: kind,
            desc: desc,
        })
    }
}

impl Error for SshError {
    fn description(&self) -> &str {
        match self.kind {
            SshErrorKind::IoFailure => "i/o error",
        }
    }
}

impl From<io::Error> for SshError {
    fn from(err: io::Error) -> SshError {
        SshError {
            kind: SshErrorKind::IoFailure,
            desc: format!("io error: {}", err),
        }
    }
}

impl fmt::Display for SshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

pub type SshResult<T> = Result<T, SshError>;
