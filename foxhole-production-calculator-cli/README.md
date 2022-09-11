# Foxhole Production Calculator CLI

CLI for the foxhole production calculator.

## Installation Instructions

The suggested installation method is using [Cargo](). If you do not have cargo installed follow the instructions for your OS at [rustup.rs](https://rustup.rs/).

After you have cargo installed run the following command:
```
cargo install foxhole-prodcution-calculator-cli
```

## Commands

Below is the help output for the currently supported commands:
```
blake@homeserver:~$ foxhole-production-calculator-cli --help
foxhole-production-calculator-cli 0.1.0
API to calculate buildings and resources needed for building production facilities.

USAGE:
    foxhole-production-calculator-cli <MATERIAL> <RATE>

ARGS:
    <MATERIAL>    Specifies the output material for the factory [possible values:
                  basic-materials, salvage, construction-materials,
                  processed-construction-materials, refined-oil, petrol, coal, coke,
                  heavy-explosive-materials, napalm, components]
    <RATE>        Specifies the desired rate of output for the given material. [Unit/Hour]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```