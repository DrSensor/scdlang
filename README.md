# Scdlang
<sup>**S**tate**c**harts **D**escription **Lang**uage</sup><br>

[![current version](https://badge.fury.io/gh/drsensor%2Fscdlang.svg)](https://github.com/drsensor/scdlang/releases/latest)
[![Docker image size](https://img.shields.io/microbadger/image-size/scdlang/scrap/latest.svg)](https://hub.docker.com/r/scdlang/scrap)
[![License](https://img.shields.io/github/license/drsensor/scdlang.svg)](./LICENSE)

> 🚧 Still **Experimental** 🏗️

## About
Scdlang (pronounced `/ˈesˌsi:ˈdi:ˈlæŋ/`) is a description language for describing Statecharts that later can be used to generate code or just transpile it into another format. This project is more focus on how to describe Statecharts universally that can be used in another language/platform rather than drawing a Statecharts diagram. For drawing, see [State Machine Cat][].

![quick_demo](https://user-images.githubusercontent.com/4953069/60861162-a5d1a800-a243-11e9-8dd3-b04ee3fde30c.gif)

### Philosophy
- **Readable** just like you read then visualize a state diagram
- **Writeable** just like you write code which is concise, clear, and can be refactored
- **Transferable** to any implementation (e.g platform, programming language, runtime, etc)

### Features and *Wishlist*
- [x] CLI utils (see [usage](packages/cli/README.md))
- [x] Syntax is inspired from various drawing languages like [mermaid][], [Graphviz][], [PlantUML][], and many more
- [x] Decent error message
- [x] Avoid invalid and ambigous transition via semantics analysis
- [ ] Rule definition for static analysis and formal verification
- Transpile into other formats:
  - [x] [XState](https://xstate.js.org/docs/)
  - [x] [State Machine Cat][]
  - [ ] [Kingly](https://brucou.github.io/documentation/)
  - [ ] [CSV](https://github.com/DrSensor/scdlang/issues/24)
  - [ ] [Sismic](https://sismic.readthedocs.io/en/latest/)
  - [ ] [SCXML](https://www.w3.org/TR/scxml/)
  - [ ] [WaveDrom](https://observablehq.com/@drom/wavedrom)
- Compile into other formats (need this to make it being an embeddable DSL):
  - [ ] WebAssembly (using [parity-wasm](https://github.com/paritytech/parity-wasm))
  - [ ] LLVM IR 🤔 (open the possibility to be compiled as wasm, dynlib, and static lib)
- Code generation 🤔 (not sure if I need this but why not?)
  - [ ] Julia via [`@generated`](https://docs.julialang.org/en/v1/manual/metaprogramming/#Generated-functions-1) implemented as [parametric](https://docs.julialang.org/en/v1/manual/methods/#Parametric-Methods-1) [multiple-dispatch](https://en.wikipedia.org/wiki/Multiple_dispatch#Julia) [functors](https://docs.julialang.org/en/v1/manual/methods/#Function-like-objects-1)
  - [ ] Rust via [`#[proc_macro_attribute]`](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros) implemented as [typestate programming](https://rust-embedded.github.io/book/static-guarantees/typestate-programming.html)? (I'm still afraid if it will conflict with another crates)
  - [ ] Elixir via [`use`](https://elixir-lang.org/getting-started/alias-require-and-import.html#use) macro which desugar into [gen_statem](https://andrealeopardi.com/posts/connection-managers-with-gen_statem/) 💪
  - [ ] Flutter via [`builder_factories`](https://github.com/flutter/flutter/wiki/Code-generation-in-Flutter) (waiting for the [FFI](https://github.com/dart-lang/sdk/issues/34452) to be stable)

> For more info, see the changelog in the [release page][]

## Getting Started
Currently, this project only have the binary CLI for each OS. Please go to the [release page][] to download then extract it. It also shipped with auto-completions script for your preffered shell.

### Installing
Just download the binary in the [release page][] or follow this instructions 👇

#### Linux🐧
TODO: (AUR, DEB, RPM) or via shellscript just like installing rustup

#### MacOS🍏
TODO: brew or via shellscript just like installing rustup

#### Windows🗔
TODO: chocolatey or via msi installer just like installing rustup

#### using [Docker](https://hub.docker.com/r/scdlang/scrap)🐳
> [smcat][State Machine Cat], [dot][Graphviz], and [graph-easy][] are pre-installed in this docker image
```console
docker pull scdlang/scrap
```

#### via [Cargo](https://doc.rust-lang.org/cargo)📦
```console
cargo install s-crap
```

## Contributing
[![open "help wanted" issues](https://img.shields.io/github/issues/drsensor/scdlang/help%20wanted.svg)](https://github.com/DrSensor/scdlang/labels/help%20wanted)
[![open "good first issue" issues](https://img.shields.io/github/issues/drsensor/scdlang/good%20first%20issue.svg)](https://github.com/DrSensor/scdlang/labels/good%20first%20issue)
<!-- TODO: add proper CONTRIBUTING.md alongs with ARCHITECTURE.md and github template for issues & pull_request -->
Any contributions are welcome as long as it follow [Code of Conduct](CODE_OF_CONDUCT.md).<br>
<sup><sup>If anyone have questions or something to discuss, feel free to DM or mention me in any platform that have my profile picture 👹.</sup></sup>

## License

This project is licensed under the Universal Permissive License 1.0 - see the [LICENSE](LICENSE) file for more detail.

## Resources

- [*Statecharts in the Making: A Personal Account*](http://www.wisdom.weizmann.ac.il/~harel/papers/Statecharts.History.pdf) by David Harel
- [Welcome to the world of Statecharts](https://statecharts.github.io/)

[Graphviz]: https://www.graphviz.org/
[PlantUML]: http://plantuml.com/state-diagram
[mermaid]: https://mermaidjs.github.io/
[State Machine Cat]: https://github.com/sverweij/state-machine-cat
[graph-easy]: https://metacpan.org/pod/distribution/Graph-Easy/bin/graph-easy
[release page]: https://github.com/DrSensor/scdlang/releases
