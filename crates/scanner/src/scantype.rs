use anyhow::anyhow;
use netscan::setting::ScanType;

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ScanTypeInput {
    TcpSync,
    TcpConnect,
}

impl ScanTypeInput {
    pub fn convert(&self) -> ScanType {
        match self {
            ScanTypeInput::TcpSync => ScanType::TcpSynScan,
            ScanTypeInput::TcpConnect => ScanType::TcpConnectScan,
        }
    }
}

impl FromStr for ScanTypeInput {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<ScanTypeInput, anyhow::Error> {
        match input {
            "tcp-sync" => Ok(ScanTypeInput::TcpSync),
            "tcp-connect" => Ok(ScanTypeInput::TcpConnect),
            _ => Err(anyhow!("input cannot be converted!")),
        }
    }
}
