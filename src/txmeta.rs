use std::default;
use std::{
    fmt::Debug,
    fs,
    io,
    io::Cursor,
    num::ParseIntError,
    path::{Path, PathBuf},
    prelude::*,
    str::FromStr,
    str::Utf8Error,
};

use soroban_env_host::xdr::{
    Error as XdrError,
    LedgerCloseMeta,
    ReadXdr,
};

pub trait LedgerBackend {
    fn get_checkpoint(&self, checkpoint: u32) -> Result<u32, Error>;
    fn get_latest(&self) -> Result<u32, Error>;
    fn get_ledger(&self, seq: u32) -> Result<LedgerCloseMeta, Error>;
}

#[derive(Debug, PartialEq)]
pub struct FSLedgerBackend<'a> {
    root: &'a Path,
}

impl<'a> FSLedgerBackend<'a> {
    pub fn root(root: &'a Path) -> Self {
        Self { root }
    }
}

impl<'a> Default for FSLedgerBackend<'a> {
    fn default() -> Self {
        Self { root: Path::new(".") }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("xdr")]
    Xdr(#[from] XdrError),
    #[error("io")]
    Io(#[from] io::Error),
    #[error("utf8")]
    Utf8Error(#[from] Utf8Error),
    #[error("parseint")]
    ParseInt(#[from] ParseIntError),
}

// TODO: Implement these for realsies
impl<'a> LedgerBackend for FSLedgerBackend<'a> {
    fn get_checkpoint(&self, checkpoint: u32) -> Result<u32, Error> {
        panic!("TODO: Implement FSLedgerBackend.get_checkpoint");
    }

    fn get_latest(&self) -> Result<u32, Error> {
        let mut path = self.root.to_path_buf();
        path.push("latest");
        match fs::read_to_string(path) {
            Ok(r) => {
                u32::from_str_radix(&r, 10)
                    .map_err(|e| From::from(e))
            },
            Err(err) => {
                 match err.kind() {
                    std::io::ErrorKind::NotFound => Ok(2),
                    _ => Err(From::from(err))
                }
            },
        }
    }

    fn get_ledger(&self, seq: u32) -> Result<LedgerCloseMeta, Error> {
        let mut path = self.root.to_path_buf();
        path.push("ledgers");
        path.push(format!("{:?}", seq));
        let r = fs::read(path)?;
        LedgerCloseMeta::read_xdr(&mut Cursor::new(r)).map_err(|e| From::from(e))
    }
}
