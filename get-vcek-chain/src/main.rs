// SPDX-License-Identifier: Apache-2.0

use azcvm::host::snp::get_vcek_chain_from_host;

use clap::Parser;
use pem::Pem as Pem2;
use std::path::{Path, PathBuf};
use std::{fs, io};
use x509_parser::pem::Pem;

#[derive(Parser)]
#[command(name = "get-vcek-chain", about = "Fetch VCEK/ASK/ARK certs from host")]
struct Cli {
    /// Output directory
    #[arg(short, long, value_name = "OUTDIR")]
    out: PathBuf,
}

fn write_pem(p: &Pem, dst: impl AsRef<Path>) -> io::Result<()> {
    let p2 = Pem2::new(p.label.clone(), p.contents.clone());
    let pem_str = pem::encode(&p2);
    fs::write(dst, pem_str)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    fs::create_dir_all(&cli.out)?;
    let chain = get_vcek_chain_from_host()?;

    write_pem(&chain.vcek, cli.out.join("vcek.pem"))?;
    write_pem(&chain.ask, cli.out.join("ask.pem"))?;
    write_pem(&chain.ark, cli.out.join("ark.pem"))?;

    Ok(())
}
