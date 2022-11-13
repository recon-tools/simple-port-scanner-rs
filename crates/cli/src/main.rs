use clap::Parser;
use scanner::{scan, ScanTypeInput};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ScannerArgs {
    /// IPv4 address to be scanned
    #[clap(short, long, value_parser)]
    address: String,

    /// Name of the network device to be used for scanning
    #[clap(short, long, value_parser)]
    device: String,

    /// Port range to be scanned. Expects an interval delimited by a dash, for example: 1-1000
    #[clap(
        long,
        use_value_delimiter = true,
        value_delimiter = '-',
        default_value = "1-1000",
        value_parser
    )]
    port_range: Vec<u16>,

    /// Type of the scan. Possible values are: tcp-sync, tcp-connect, icmp-ping, tcp-ping, udp-ping
    /// Default type is tcp-sync, but it might require elevated privileges. Use tcp-connect you
    /// don't have the possibility to elevate your privileges
    #[clap(long, default_value = "tcp-sync", value_enum)]
    scan_type: ScanTypeInput,
}

fn main() -> Result<(), anyhow::Error> {
    let args: ScannerArgs = ScannerArgs::parse();
    scan(args.device, args.address, args.port_range, args.scan_type)
}
