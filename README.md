# simple-port-scanner-rs

Simple port scanner for local and remote networks.

# Usage

```bash
$ sps -h
Simple port scanner.

Usage: sps [OPTIONS] <ADDRESS>

Arguments:
  <ADDRESS>  IPv4 CIDR to be scanned (example: 192.168.1.0/24, 10.0.0.2/32, 10.5.22.76)

Options:
  -d, --device <DEVICE>          Name of the network device to be used for scanning [default: ]
      --port-range <PORT_RANGE>  Port range to be scanned. Expects an interval delimited by a dash, for example: 1-1000 [default: 1-1000]
      --scan-type <SCAN_TYPE>    Type of the scan. Possible values are: tcp-sync, tcp-connect Default type is tcp-sync, but it might require elevated privileges. Use tcp-connect you don't have the possibility to elevate your privileges [default: tcp-sync] [possible values: tcp-sync, tcp-connect]
  -h, --help                     Print help information
  -V, --version                  Print version information
```

Example:

```bash
$ sps 192.168.100.1/32 --port-range 1-500
192.168.100.1:53 - Open
192.168.100.1:80 - Open
Scan Time: 6.329893083s
```
