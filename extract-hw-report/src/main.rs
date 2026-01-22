// SPDX-License-Identifier: Apache-2.0

use azcvm::types::hcl::{
    HclAttestationReport,
    hwreport::{SEV_SNP_REPORT_SIZE, TDX_REPORT_SIZE},
};
use clap::{Parser, ValueEnum};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, ValueEnum)]
enum Tee {
    Snp,
    Tdx,
}

#[derive(Parser)]
#[command(
    name = "extract-hw-report",
    about = "Extract SNP or TDX report from an HCL report (raw byte file)"
)]
struct Cli {
    /// Input HCL report (raw bytes)
    #[arg(short, long = "in", value_name = "INFILE")]
    infile: PathBuf,
    /// Output hardware report (raw bytes)
    #[arg(short, long = "out", value_name = "OUTFILE")]
    outfile: PathBuf,
    /// Report type: snp | tdx
    #[arg(short, long = "tee", value_name = "TEE_TYPE", required = true)]
    tee: Tee,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let input = fs::read(&cli.infile)?;
    let hcl = HclAttestationReport::from_bytes(&input)
        .map_err(|e| anyhow::anyhow!("Failed to parse HCL report: {e}"))?;
    let tee = cli.tee;
    let content: &[u8] = match tee {
        Tee::Snp => {
            if hcl.report_payload.len() < SEV_SNP_REPORT_SIZE {
                anyhow::bail!(
                    "report_payload is less than {} bytes; got {}",
                    SEV_SNP_REPORT_SIZE,
                    hcl.report_payload.len(),
                );
            }
            &hcl.report_payload[..1184]
        }
        Tee::Tdx => {
            if hcl.report_payload.len() < TDX_REPORT_SIZE {
                anyhow::bail!(
                    "report_payload is less than {} bytes; got {}",
                    TDX_REPORT_SIZE,
                    hcl.report_payload.len(),
                );
            }
            &hcl.report_payload[..1024]
        }
    };
    fs::write(&cli.outfile, content)?;
    Ok(())
}
