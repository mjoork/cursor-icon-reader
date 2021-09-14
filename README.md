# Cursor/Icon Reader

[![Linux and Windows build](https://github.com/mjoork/cursor-icon-reader/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/mjoork/cursor-icon-reader/actions/workflows/build-and-test.yml)

Formats CUR and ICO files data into a human-readable format.

Example output reading a reference cursor looks like:

```
$ cir test.cur
File type: Cursor
Images count: 1

Width: 32
Height: 32
Colors in palette: 0
X Hotspot: 13 px
Y Hotspot: 6 px
Image size: 4264 bytes
Image offset: 22 bytes
```

## Installing

### Via `cargo install`

Cargo supports installing directly from GIT, like so:

```shell
cargo install --git https://github.com/mjoork/cursor-icon-reader.git
```

You can also [download a zip archive](https://github.com/mjoork/cursor-icon-reader/archive/master.zip), extract it and install from inside the directory via `cargo install --path .`.

### Via latest release from [releases](https://github.com/mjoork/cursor-icon-reader/releases/)

1. Download [latest release](https://github.com/mjoork/cursor-icon-reader/releases/latest/) binary for your respective OS.
2. Add it to `PATH`.
