# Foxhole Production Calculator CLI

CLI for the foxhole production calculator.

## What is this Thing?

This thing is a CLI tool that allows you to plan out the number of buildings required to produce a given output from a Foxhole factory. This will take a large amount of the guess work needed when trying to sort through dependency tree of all the factories.

### Why a CLI and not a web app?

Because I'm a bum and I suck at front end development. Leave me alone.

## Example

Below is example output for a factory that would output 100 pipes per hour, and take in components from an external source. Note that the building requirements are only for the main structures, and do not include the costs to build foundations, electric poles, etc. There really isn't a good way to calculate how much you would need for these things, as the amount you will build is entirely dependent on your factory layout.

```
foxhole-production-calculator-cli pipe 100 -u components

{
  "buildings": [
    {
      "building": "Coal Refinery",
      "upgrade": "Coal Liquefier",
      "count": 1.3333334
    },
    {
      "building": "Materials Factory",
      "upgrade": "Smelter",
      "count": 1.3888888
    },
    {
      "building": "MetalWorks Factory",
      "upgrade": null,
      "count": 3.3333333
    },
    {
      "building": "Oil Refinery",
      "upgrade": "Cracking Unit",
      "count": 0.2962963
    },
    {
      "building": "MetalWorks Factory",
      "upgrade": "Blast Furnace",
      "count": 5.0
    },
    {
      "building": "Coal Refinery",
      "upgrade": "Advanced Coal Liquefier",
      "count": 0.64102566
    }
  ],
  "power": 81.5,
  "build_cost": {
    "ConstructionMaterials": 2375,
    "BasicMaterials": 400,
    "SteelConstructionMaterials": 200,
    "ProcessedConstructionMaterials": 420
  },
  "inputs": {
    "Water": 2942.3076,
    "Components": 5500.0,
    "Coal": 8884.615,
    "Salvage": 1999.9999
  }
}
```

## Limitations

Currently this tool only calculates the most optimal path through the dependency tree. Optimal in this case means optimizing for the fastest production rate, so choosing buildings that output more of a given resource than another. There may be situations where this is not ideal for your needs. For example you may want to build a factory that minimizes power input at the cost of slower output. For now we don't support those use cases, as it's difficult to enumerate all of the possible value functions that we could use to make decisions when traversing the dependency tree.

## Installation Instructions

The suggested installation method is using [Cargo](https://doc.rust-lang.org/cargo/). If you do not have cargo installed follow the instructions for your OS at [rustup.rs](https://rustup.rs/).

After you have cargo installed run the following command:
```
cargo install foxhole-prodcution-calculator-cli
```

## Commands

Below is the help output for the currently supported commands:
```
USAGE:
    foxhole-production-calculator-cli [OPTIONS] <MATERIAL> <RATE>

ARGS:
    <MATERIAL>    Specifies the output material for the factory [possible values:
                  basic-materials, salvage, construction-materials,
                  processed-construction-materials, oil, petrol, coal, coke,
                  explosive-materials, heavy-explosive-materials, flame-ammo, components, water,
                  heavy-oil, enriched-oil, sulfur, steel-construction-materials,
                  concrete-materials, pipe, assembly-materials-i, assembly-materials-ii,
                  assembly-materials-iii, assembly-materials-iv, assembly-materials-v,
                  metal-beam, sand-bag, barbed-wire, rocket3-c-high-explosive, rocket4-c-fire,
                  shell75-mm, shell945-mm, shell120-mm, shell150-mm, shell250-mm, shell300-mm]
    <RATE>        Specifies the desired rate of output for the given material. [Unit/Hour]

OPTIONS:
    -h, --help                         Print help information
    -u, --user-inputs <USER_INPUTS>    Optional argument specifying inputs that will be brought in
                                       externally from the factory. Multiple values can be input
                                       with comma seperators [possible values: basic-materials,
                                       salvage, construction-materials,
                                       processed-construction-materials, oil, petrol, coal, coke,
                                       explosive-materials, heavy-explosive-materials, flame-ammo,
                                       components, water, heavy-oil, enriched-oil, sulfur,
                                       steel-construction-materials, concrete-materials, pipe,
                                       assembly-materials-i, assembly-materials-ii,
                                       assembly-materials-iii, assembly-materials-iv,
                                       assembly-materials-v, metal-beam, sand-bag, barbed-wire,
                                       rocket3-c-high-explosive, rocket4-c-fire, shell75-mm,
                                       shell945-mm, shell120-mm, shell150-mm, shell250-mm,
                                       shell300-mm]
    -V, --version                      Print version information
```

## Contributing

PRs and issues are always welcome. Additionally if you find any discrepancies in the factory values please feel free to open a PR with the edited toml files, or open an issue.