use clap::Parser;
use scanner::scan;

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
}

fn main() -> Result<(), anyhow::Error> {
    let args: ScannerArgs = ScannerArgs::parse();
    println!("Scanning!");
    scan(args.device, args.address, args.port_range)
}
