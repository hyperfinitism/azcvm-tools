// SPDX-License-Identifier: Apache-2.0

use azcvm::types::hcl::{HclAttestationReport, HclRuntimeClaims};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use clap::Parser;
use serde::Serialize;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(
    name = "show-hcl-report",
    about = "Show a raw HCL report into a JSON representation."
)]
struct Cli {
    /// Input HCL report file (raw bytes)
    #[arg(short, long = "in", value_name = "INFILE")]
    infile: PathBuf,
    /// Output JSON file path (omit for stdout)
    #[arg(short, long = "out", value_name = "OUTFILE")]
    outfile: Option<PathBuf>,
    /// Pretty-print JSON (indent output)
    #[arg(short, long = "pretty")]
    pretty: bool,
}

#[derive(Serialize)]
struct JsonHeader {
    signature: String,
    version: u32,
    request_type: u32,
    status: u32,
    reserved: [u8; 12],
}

#[derive(Serialize)]
struct JsonReport {
    header: JsonHeader,
    report_payload: String, // base64url
    runtime_data: JsonRuntimeData,
}

#[derive(Serialize)]
struct JsonRuntimeData {
    data_size: u32,
    version: u32,
    report_type: u32,
    hash_type: u32,
    claim_size: u32,
    runtime_claims: HclRuntimeClaims,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let input = fs::read(&cli.infile)?;
    let parsed = HclAttestationReport::from_bytes(&input)?;
    let header = &parsed.header;
    let signature_bytes = header.signature.to_le_bytes(); // or to_be_bytes()
    let signature_str = match std::str::from_utf8(&signature_bytes) {
        Ok(s) => s.to_string(),
        Err(_) => "<invalid utf-8 in header.signature>".to_string(),
    };

    let header_json = JsonHeader {
        signature: signature_str,
        version: header.version,
        request_type: header.request_type,
        status: header.status,
        reserved: header.reserved,
    };

    // base64url encode payload (NO padding)
    let payload_str = URL_SAFE_NO_PAD.encode(&parsed.report_payload);

    let runtime_data = &parsed.runtime_data;
    let runtime_claims = HclRuntimeClaims::from_string(&runtime_data.runtime_claims)?;
    let runtime_json = JsonRuntimeData {
        data_size: runtime_data.data_size,
        version: runtime_data.version,
        report_type: runtime_data.report_type as u32,
        hash_type: runtime_data.hash_type as u32,
        claim_size: runtime_data.claim_size,
        runtime_claims,
    };

    let root_json = JsonReport {
        header: header_json,
        report_payload: payload_str,
        runtime_data: runtime_json,
    };

    let out = if cli.pretty {
        serde_json::to_string_pretty(&root_json)?
    } else {
        serde_json::to_string(&root_json)?
    };
    match &cli.outfile {
        Some(p) => fs::write(p, out)?,
        None => {
            io::stdout().write_all(out.as_bytes())?;
        }
    }
    Ok(())
}
