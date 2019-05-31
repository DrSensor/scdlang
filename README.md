# Scdlang
<sup>**S**tate**c**harts **D**escription **Lang**uage</sup><br>

[![current version](https://badge.fury.io/gh/drsensor%2Fscdlang.svg)](https://github.com/drsensor/scdlang/releases/latest)
[![Docker image size](https://img.shields.io/microbadger/image-size/scdlang/scrap/latest.svg)](https://hub.docker.com/r/scdlang/scrap)
![License](https://img.shields.io/github/license/drsensor/scdlang.svg)
[![gitstore](https://enjoy.gitstore.app/repositories/badge-DrSensor/scdlang.svg)](https://enjoy.gitstore.app/repositories/DrSensor/scdlang)

> 🚧 Still **Work in Progress** 🏗️

## About
Scdlang (pronounced `/ˈesˌsi:ˈdi:ˈlæŋ/`) is a description language for describing Statecharts that later can be used to generate code or just transpile it into another format. This project is more focus on how to describe Statecharts universally that can be used in another language/platform rather than drawing a Statecharts diagram. For drawing, see [State Machine cat][].

### Philosophy
- **Readable** just like you read then visualize a state diagram
- **Writeable** just like you write code which is concise, clear, and can be refactored
- **Transferable** to any implementation (e.g platform, programming language, runtime, etc)

### Key Features
- [x] Awesome [CLI](packages/cli/README.md)
- [x] Syntax is inspired from various drawing languages like [mermaid][], [Graphviz][], [PlantUML][], and many more
- [x] Decent error message
- [x] Avoid invalid and ambigous transition via semantics analysis
- Transpile into other formats:
  - [x] [XState](https://xstate.js.org/docs/)
  - [ ] [State Machine cat][]
  - [ ] [Sismic](https://sismic.readthedocs.io/en/latest/)
  - [ ] [SCXML](https://www.w3.org/TR/scxml/)
- Compile into other formats (hopefully, no promise):
  - [ ] WebAssembly (possibly via AssemblyScript CLI 😋)
- Code generation 🤔
  - [ ] Rust macro derive? (I'm still afraid if it will conflict with another crates)

> For more info, see the changelog in the [release page][]

## Getting Started
Currently, this project only have the binary CLI for each OS. Please go to the [release page][] to download then extract it. It also shipped with auto-completions script for your preffered shell.

### Installing
Just download the binary in the [release page][] or follow this instructions 👇

#### Linux
TODO: (AUR, DEB, RPM) or via shellscript just like installing rustup

#### MacOS
TODO: brew or via shellscript just like installing rustup

#### Windows
TODO: chocolatey or via msi installer just like installing rustup

#### [Docker](https://hub.docker.com/r/scdlang/scrap)
```console
docker pull scdlang/scrap
```

#### via Cargo
TODO: TBD

## Contributing
![open "help wanted" issues](https://img.shields.io/github/issues/drsensor/scdlang/help%20wanted.svg)
![open "good first issue" issues](https://img.shields.io/github/issues/drsensor/scdlang/good%20first%20issue.svg)
<!-- TODO: add proper CONTRIBUTING.md alongs with ARCHITECTURE.md and github template for issues & pull_request -->
Any contributions are welcome as long as it follow [Code of Conduct](CODE_OF_CONDUCT.md).<br>
<sup><sup>If anyone have questions or something to discuss, feel free to DM or mention me in any platform that have my profile picture 👹.</sup></sup>

## License

This project is licensed under the UPL-1.0 License - see the [LICENSE](LICENSE) file for details

## Credits

- *Statecharts in the Making: A Personal Account* by David Harel
- [Welcome to the world of Statecharts](https://statecharts.github.io/) started by @mogsie in December 2017
- [Fundamentals: why state machines?](https://gist.github.com/thure/dcffc30117b9a9800084) by @thure

[Graphviz]: https://www.graphviz.org/
[PlantUML]: http://plantuml.com/state-diagram
[mermaid]: https://mermaidjs.github.io/
[State Machine cat]: https://github.com/sverweij/state-machine-ca
[release page]: https://github.com/DrSensor/scdlang/releases