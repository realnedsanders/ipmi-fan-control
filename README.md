# ipmi-fan-control

[![GitHub license](https://img.shields.io/github/license/yinheli/ipmi-fan-control)](https://github.com/yinheli/ipmi-fan-control/blob/master/LICENSE)

A tool to control the fan speed of my QCT (aka Quanta aka QuantaGrid) d51b-1u and d51pc-1ulh by monitoring the temperature of CPU via IPMI.

## Why

I wanted to be able to control the thresholds of the fan speed via the OS and not rely on the BMC's automatic control.

## Usage

Download from [release](https://github.com/yinheli/ipmi-fan-control/releases) page (prebuilt binary via github actions), or build from source code.

```bash
cargo build --release
```

Install dependency, install (debian/pve):

```bash
apt install ipmitool
```

use `ipmi-fan-control --help` to see the usage.

```bash
ipmi-fan-control --help
```

```
Usage: ipmi-fan-control [OPTIONS] <COMMAND>

Commands:
  auto   Auto adjust fan speed by interval checking CPU temperature
  fixed  Set fixed RPM percentage for fan
  info   Print CPU temperature and fan RPM
  help   Print this message or the help of the given subcommand(s)

Options:
      --verbose      Verbose output
  -f, --fans <FANS>  Number of fans [default: 4]
  -h, --help         Print help information
  -V, --version      Print version information
```

## Resource

- https://www.intel.com/content/www/us/en/servers/ipmi/ipmi-home.html
- https://github.com/ipmitool/ipmitool
- https://back2basics.io/2020/05/reduce-the-fan-noise-of-the-dell-r720xd-plus-other-12th-gen-servers-with-ipmi/
