# `scrap`
<sup>**S**tate**c**harts **R**h**ap**sody</sup>

Features:
- output as:
  - [x] [xstate configuration in json][xstate-format]
  - [x] [state-machine-cat in json][smcat]
- [x] REPL
- Shell friendly
  - [x] Colored output
  - [x] Pipe-able (e.g: `cat f.scdl | scrap eval` or `scrap code f.scdl | cat`) which disable the colored output. Useful when piping to file or running on CI.

### Hooked CLI
Scrap will automatically use this CLI below if it's available on the system host.
In Games term, think of it as an extension pack where some features will be enabled if you install it 😉

<details><summary><a href="https://github.com/sverweij/state-machine-cat#command-line-interface">smcat</a></summary>

Install
```console
npm -g state-machine-cat
```
<sup>[👆 How to install `npm`!](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)</sup>

Unlocked features:
- `smcat` become the default output when `--format smcat` is specified
- flag `--as` can accept 🗹 when `--format smcat` is specified
  - [x] `svg`    -> Scalable Vector Graphics
  - [x] `dot`    -> Alias for Graphviz format/language
  - [x] `smcat`  -> state-machine-cat language (default)
  - [x] `json`   -> AST representation of state-machine-cat in JSON (ex-default)
  - [ ] `ast`    -> AST representation of state-machine-cat
  - [ ] `html`   -> HTML
  - [x] `scxml`  -> State Chart XML (W3C standard)
  - [ ] `scjson` -> experimental JSON representation of state-machine-cat
  - [x] `xmi`    -> XML Metadata Interchange (OMG standard)
</details>
<details><summary><a href="https://metacpan.org/pod/distribution/Graph-Easy/bin/graph-easy">graph-easy</a></summary>

> ⚠️ [smcat](https://github.com/sverweij/state-machine-cat#command-line-interface) need to be installed first

Install
```console
cpanm graph-easy
```
<sup>[👆 How to install `cpanminus`!](https://metacpan.org/pod/App::cpanminus#Installing-to-system-perl)</sup>

Unlocked features:
- flag `--as` can accept 🗹 when `--format graph` is specified
  - [x] `ascii`    -> ASCII art rendering
  - [x] `boxart`   -> Unicode Boxart rendering (default)
  - [ ] `html`     -> HTML
  - [x] `svg`      -> Scalable Vector Graphics
  - [x] `dot`      -> the DOT language
  - [x] `txt`      -> Graph::Easy text
  - [ ] `vcg`      -> VCG (Visualizing Compiler Graphs - a subset of GDL) text
  - [ ] `gdl`      -> GDL (Graph Description Language) text
  - [ ] `graphml`  -> GraphML
  - [x] `bmp`      -> Windows bitmap
  - [x] `gif`      -> GIF
  - [ ] `hpgl`     -> HP-GL/2 vector graphic
  - [x] `jpg`      -> JPEG
  - [ ] `pcl`      -> PCL printer language
  - [x] `pdf`      -> PDF
  - [x] `png`      -> PNG
  - [x] `ps`       -> Postscript
  - [x] `ps2`      -> Postscript with PDF notations (see graphviz documentation)
  - [ ] `tga`      -> Targa bitmap
  - [x] `tif`      -> TIFF bitmap
</details>
<details><summary><a href="http://edutechwiki.unige.ch/en/Graphviz#Command_line_syntax">dot</a></summary>

> ⚠️ [smcat](https://github.com/sverweij/state-machine-cat#command-line-interface) need to be installed first

Unlocked features:
- flag `--as` can accept 🗹 when `--format graph` is specified
  - [x] bmp                             ->  Windows Bitmap Format             (blob)
  - [x] canon/dot/gv/xdot               ->  DOT/Graphviz language
  - [ ] cgimage                         ->  CGImage bitmap format
  - [x] eps                             ->  Encapsulated PostScript
  - [ ] exr                             ->  OpenEXR
  - [x] fig                             ->  FIG graphics language
  - [x] gd/gd2                          ->  GD/GD2 formats                    (blob)
  - [x] gif                             ->  Graphics Interchange Format       (blob)
  - [ ] gtk                             ->  GTK canvas
  - [ ] ico                             ->  Icon Image File Format
  - [ ] <!--TODO:support-->imap/cmapx   ->  Server-side and client-side imagemaps
  - [ ] imap_np/cmapx_np                ->  These are identical to the imap and cmapx formats, except they rely solely on rectangles as active areas
  - [ ] jp2                             ->  JPEG 2000
  - [x] jpg/jpeg/jpe                    ->  JPEG                              (blob)
  - [x] json/json0/dot_json/xdot_json   ->  Dot graph represented in JSON format
  - [ ] pct/pict                        ->  PICT
  - [ ] pdf                             ->  Portable Document Format (PDF). This option does not support anchors, etc. Refer to `ps2` instead
  - [x] pic                             ->  Kernighan's PIC graphics language
  - [x] plain/plain-ext                 ->  Simplified version of `dot` language.
  - [x] png                             ->  Portable Network Graphics format  (blob)
  - [x] ps                              ->  PostScript
  - [x] ps2                             ->  PostScript for PDF
  - [ ] psd                             ->  Photoshop Document
  - [ ] sgi                             ->  Silicon Graphics Image
  - [x] svg/svgz                        ->  Scalable Vector Graphics          (blob:svgz)
  - [ ] tga                             ->  Truevision TGA
  - [x] tif/tiff                        ->  TIFF (Tag Image File Format)      (blob)
  - [x] tk                              ->  TK graphics language
  - [x] vml/vmlz                        ->  Vector Markup Language (VML)      (blob:vmlz)
  - [x] vrml                            ->  Virtual Reality Modeling Language
  - [x] wbmp                            ->  Wireless BitMap format            (blob)
  - [ ] webp                            ->  Image format for the Web          (blob)
  - [ ] xlib/x11                        ->  Xlib canvas
</details>

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

<details><summary><code>scrap help code</code></summary>

```console
$ scrap code --help

Generate from scdlang file declaration to another format

USAGE:
    scrap code [FLAGS] [OPTIONS] <FILE> --format <target> [DIST]

FLAGS:
    -h, --help      Prints help information
        --stream    Parse the file line by line

OPTIONS:
        --as <format>        Select parser output [possible values: json, svg, dot, smcat, html, scxml, xmi, ascii, boxart, bmp, gif, jpg, pdf, png, ps, ps2, tif]
    -f, --format <target>    Select output format [possible values: xstate, smcat, graph]

ARGS:
    <FILE>    File to print / concatenate
    <DIST>    Output the result to this directory / file
```
</details>

<details><summary><code>scrap help repl</code></summary>

```console
$ scrap repl --help

Evaluate scdlang expression in interactive manner

USAGE:
    scrap eval [FLAGS] [OPTIONS] --format <target>

FLAGS:
    -h, --help           Prints help information
    -i, --interactive    Prints result on each expression
        --strict         Exit immediately if an error occurred

OPTIONS:
        --as <format>        Select parser output [possible values: json, svg, dot, smcat, html, scxml, xmi, ascii, boxart, bmp, gif, jpg, pdf, png, ps, ps2, tif]
    -f, --format <target>    Select output format [possible values: xstate, smcat, graph]
```
</details>

## Cheats

Some CLI and tools that can came handy:
- [`watchexec`](https://github.com/watchexec/watchexec)  ->  Executes commands in response to file modifications.
- [`live-server`](http://tapiov.net/live-server)         ->  A simple development http server with live reload capability. Useful to quickly preview the `svg` output
- [`Jusfile`](https://github.com/casey/just) or [`Makefile`](https://wikipedia.org/wiki/Makefile)  -> Task runner to automatically perform frequent tasks

<details><summary>Live preview the visual representation in terminal window</summary>

```shell
watchexec "scrap code $FILE.scl -f graph" --clear --watch $FILE.scl
```
![live preview boxart.gif](https://files.steempeak.com/file/steempeak/drsensor/xKsEZn7a-live20preview20boxart.gif)
</details>

<details><summary>Live preview the visual representation of media output (svg, jpg, png, etc)</summary>

- For SVG
```shell
watchexec "scrap code $INPUT.scl -o $OUTPUT.svg -f smcat --as svg" --clear --watch $INPUT.scl
live-server --watch=$INPUT.scl --entry-file=$OUTPUT.svg --port=2019 --wait=2020
google-chrome --app=http://localhost:2019
```
![live preview svg.gif](https://files.steempeak.com/file/steempeak/drsensor/865FJM93-live20preview20svg.compressed.gif)

- For VSCode user, you can `--output` it `--as` media file like png or jpg then open it in VSCode
```shell
watchexec "scrap code $INPUT.scl -o $OUTPUT.png -f graph --as png" --watch $INPUT.scl
code $OUTPUT.png
```
![live preview png.gif](https://files.steempeak.com/file/steempeak/drsensor/sco5lkYP-live20preview20png.gif)
</details>

<details><summary>Inspect each line</summary>

```shell
cat $FILE.scl | scrap repl --interactive --format graph
```
![Inspect each line](https://user-images.githubusercontent.com/4953069/60797897-5edba800-a19a-11e9-9e32-dd0b14e8a53c.gif)
  <details><summary>Only inspect result</summary>

```shell
cat $FILE.scl | scrap repl --interactive --format xstate 2>/dev/null
```
![Only inspect result on each line](https://user-images.githubusercontent.com/4953069/60797896-5edba800-a19a-11e9-8b1b-3ae73e0e08c0.gif)
  </details>
  <details><summary>Only inspect error</summary>

```shell
cat $FILE.scl | scrap repl --interactive --format xstate 1>/dev/null
```
![Only inspect error on each line](https://user-images.githubusercontent.com/4953069/60797894-5e431180-a19a-11e9-93c4-942f7bd6b78c.gif)
  </details>
</details>

<details><summary>Log error to file</summary>

```shell
scrap code $FILE.scl --stream 2> $OUTPUT.log
```
![Log error to file](https://user-images.githubusercontent.com/4953069/60797893-5e431180-a19a-11e9-97a4-bfe0509cd18a.gif)
</details>

<details><summary>Print in plain format (no syntax highlighter)</summary>

```shell
scrap code $FILE.scl --format xstate --stream 2>&1 | cat
```
![Print in plain format](https://user-images.githubusercontent.com/4953069/60797892-5e431180-a19a-11e9-85f5-d2347fbd1879.gif)
  <details><summary>Only print result</summary>

```shell
scrap code $FILE.scl --format xstate --stream 2>/dev/null | cat
```
![Only print result and in plain format](https://user-images.githubusercontent.com/4953069/60797891-5daa7b00-a19a-11e9-8776-22cbd33050a6.gif)
  </details>
  <details><summary>Only print error</summary>

```shell
scrap code $FILE.scl --format xstate --stream 2>&1 1>/dev/null | cat
```
![Only print error and in plain format](https://user-images.githubusercontent.com/4953069/60797889-5daa7b00-a19a-11e9-8266-d5dc4121bfd6.gif)
  </details>
</details>

<details><summary>Inspect and log each line to file</summary>
  <details><summary>Inspect and log error</summary>

```shell
cat $FILE.scl | scrap repl --interactive --format xstate 2> $OUTPUT.log
```
![Inspect and log error](https://user-images.githubusercontent.com/4953069/60797887-5d11e480-a19a-11e9-97fd-5192bae5b55f.gif)
  </details>
  <details><summary>Inspect and log result</summary>

```shell
cat $FILE.scl | scrap repl --interactive --format xstate > $OUTPUT.log
```
![Inspect and log result](https://user-images.githubusercontent.com/4953069/60797885-5c794e00-a19a-11e9-9b78-7e7be793548b.gif)
  </details>
</details>

[xstate-format]: https://xstate.js.org/docs/guides/machines.html#configuration
[smcat]: https://github.com/sverweij/state-machine-cat
