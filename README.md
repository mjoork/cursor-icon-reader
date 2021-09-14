# Cursor/Icon Reader

[![Linux and Windows build](https://github.com/mjoork/cir/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/mjoork/cir/actions/workflows/build-and-test.yml)

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
