// SPDX-License-Identifier: Apache-2.0

use azcvm::host::tdx::get_td_quote_from_host;
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(
    name = "get-td-quote",
    about = "Request TD quote from host SGX machine"
)]
struct Cli {
    /// Input TD report path
    #[arg(short, long = "in", value_name = "INFILE")]
    infile: PathBuf,
    /// Output TD quote path
    #[arg(short, long = "out", value_name = "OUTFILE")]
    outfile: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let td_report_bytes = fs::read(&cli.infile)?;
    let quote = get_td_quote_from_host(&td_report_bytes)?;
    fs::write(&cli.outfile, quote)?;
    Ok(())
}
