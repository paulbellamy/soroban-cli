use clap::Parser;
use std::{
    ffi::OsString,
    fmt::Debug,
    fs,
    io,
    io::Cursor,
    str::Utf8Error,
};
use stellar_contract_env_host::xdr::{
    ReadXdr,
    SpecEntry,
    SpecEntryFunction,
    SpecEntryUdt,
    self,
};

use crate::txmeta::{LedgerBackend, FSLedgerBackend, Error as TxMetaError};

#[derive(Parser, Debug)]
pub struct Logs {
    #[clap(long, parse(from_os_str), default_value = ".")]
    ledger_root: std::path::PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("xdr")]
    Xdr(#[from] xdr::Error),
    #[error("io")]
    Io(#[from] io::Error),
    #[error("utf8")]
    Utf8Error(#[from] Utf8Error),
    #[error("txmeta")]
    TxMetaError(#[from] TxMetaError),
}

impl Logs {
    pub fn run(&self) -> Result<(), Error> {
        // // Connect to the ledger backend
        let backend = FSLedgerBackend::root(
            &self.ledger_root.as_path()
        );
        // Load the latest ledger seq
        let seq = backend.get_latest()?;
        let ledger = backend.get_ledger(seq)?;
        // Print the contract logs for the latest ledger
        println!("{:?}", ledger);
        // Check if the ledger has any contract logs. Need to update the xdr in the repo for that.
        Ok(())
    }
}
