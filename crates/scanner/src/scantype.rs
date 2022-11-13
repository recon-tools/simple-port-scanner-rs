use clap::ValueEnum;
use netscan::setting::ScanType;

#[derive(ValueEnum, Debug, Clone)]
pub enum ScanTypeInput {
    TcpSync,
    TcpConnect,
    IcmpPing,
    TcpPing,
    UdpPing,
}

impl ScanTypeInput {
    pub fn convert(&self) -> ScanType {
        match self {
            ScanTypeInput::TcpSync => ScanType::TcpSynScan,
            ScanTypeInput::TcpConnect => ScanType::TcpConnectScan,
            ScanTypeInput::IcmpPing => ScanType::IcmpPingScan,
            ScanTypeInput::TcpPing => ScanType::TcpPingScan,
            ScanTypeInput::UdpPing => ScanType::UdpPingScan,
        }
    }
}
