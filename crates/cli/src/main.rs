use clap::Parser;
use scanner::scan;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ScannerArgs {
    /// IPv4 CIDR to be scanned (example: 192.168.1.0/24, 10.0.0.2/32, 10.5.22.76)
    #[clap(value_parser, index = 1)]
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

    /// Type of the scan. Possible values are: tcp-sync, tcp-connect
    /// Default type is tcp-sync, but it might require elevated privileges. Use tcp-connect you
    /// don't have the possibility to elevate your privileges
    #[clap(long, default_value = "tcp-sync",
    value_parser = clap::builder::PossibleValuesParser::new(["tcp-sync", "tcp-connect"]))]
    scan_type: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args: ScannerArgs = ScannerArgs::parse();
    scan(args.device, args.address, args.port_range, args.scan_type)
}
