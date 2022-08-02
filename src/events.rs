use clap::Parser;
use crate::snapshot;
use std::{
    ffi::OsString,
    fmt::Debug,
    fs,
    io,
    io::Cursor,
    str::Utf8Error,
};
use soroban_env_host::xdr::{
    ReadXdr,
    self,
};

use crate::txmeta::{LedgerBackend, FSLedgerBackend, Error as TxMetaError};

#[derive(Parser, Debug)]
pub struct Cmd {
    /// Contract ID to filter by
    #[clap(long = "id")]
    contract_id: String,
    /// File to persist ledger state
    #[clap(long, parse(from_os_str), default_value = "ledger.json")]
    ledger_file: std::path::PathBuf,
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
    #[error("snapshot")]
    Snapshot(#[from] snapshot::Error),
}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        // let contract_id: [u8; 32] = utils::contract_id_from_str(&self.contract_id)?;

        // Initialize storage and host
        // TODO: allow option to separate input and output file
        let mut ledger_entries = snapshot::read(&self.ledger_file)?;

        // // // Connect to the ledger backend
        // let backend = FSLedgerBackend::root(
        //     &self.ledger_root.as_path()
        // );
        // Load the latest ledger seq
        let seq = backend.get_latest()?;
        let ledger = backend.get_ledger(seq)?;
        // Print the contract events for the latest ledger
        println!("{:?}", ledger);
        // Check if the ledger has any contract events. Need to update the xdr in the repo for that.
        Ok(())
    }
}
