# `scrap`
<sup>**S**tate**c**harts **R**h**ap**sody</sup>

Features:
- output as:
  - [x] [xstate configuration][xstate-format]
- [x] REPL
- Shell friendly
  - [x] Colored output
  - [x] Pipe-able (e.g: `cat f.scdl | scrap eval` or `scrap code f.scdl | cat`) which disable the colored output. Useful when piping to file or running on CI.

## Usage
```console
$ scrap help

Statecharts Rhapsody

USAGE:
    scrap <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    code    Generate from scdlang file declaration to another format [aliases: generate, gen, declaration, declr]
    eval    Evaluate scdlang expression in interactive manner [aliases: repl]
    help    Prints this message or the help of the given subcommand(s)
```

### Generate from file
```console
$ scrap code --help

Generate from scdlang file declaration to another format

USAGE:
    scrap code [FLAGS] [OPTIONS] <FILE> [DIST]

FLAGS:
    -h, --help      Prints help information
        --stream    Parse the file line by line

OPTIONS:
    -f, --format <format>    Select output format [default: xstate]  [possible values: xstate]

ARGS:
    <FILE>    File to print / concatenate
    <DIST>    Output the result to this directory / file
```

Given this simple declaration:
```scl
A1 -> B @ C
A2 <- B @ D
```
then
```console
$ scrap code simple.scl --format xstate
{
  states: {
    A1: {
      on: {
        C: "B"
      }
    },
    A2: {
      on: {
        D: "B"
      }
    }
  }
}
```

### REPL
```console
$ scrap eval --help
Evaluate scdlang expression in interactive manner

USAGE:
    scrap eval [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -i, --interactive    Prints result on each expression
        --strict         Exit immediately if an error occurred

OPTIONS:
    -f, --format <format>    Select output format [default: xstate]  [possible values: xstate]
```

[xstate-format]: https://xstate.js.org/docs/guides/machines.html#configuration