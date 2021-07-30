# read-cursor

[![Linux and Windows build](https://github.com/mjoork/cir/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/mjoork/cir/actions/workflows/build-and-test.yml)

A simple utility to format CUR and ICO files data into a human-readable format.

## Sample output

This is how reading `arrow_l.cur` looks like:

```
C:\Windows\Cursors>cir arrow_l.cur
I've read the file, here's the info:

Type of file    : CUR
Number of images: 5

┍━━━━┯━━━━━━━━━━━━┑
│ #1 │ Size       : 128x128px
┕━━━━┥ N Colors   : 2
     │ Res. Byte  : 0
     │ X Hotspot  : 0
     │ Y Hotspot  : 2
     │ Image (Kb) : 4144
     │ Img. Offset: 86
     ┕━━━━━━━━━━━━┙
┍━━━━┯━━━━━━━━━━━━┑
│ #2 │ Size       : 96x96px
┕━━━━┥ N Colors   : 2
     │ Res. Byte  : 0
     │ X Hotspot  : 0
     │ Y Hotspot  : 2
     │ Image (Kb) : 2352
     │ Img. Offset: 4230
     ┕━━━━━━━━━━━━┙
┍━━━━┯━━━━━━━━━━━━┑
│ #3 │ Size       : 64x64px
┕━━━━┥ N Colors   : 2
     │ Res. Byte  : 0
     │ X Hotspot  : 0
     │ Y Hotspot  : 1
     │ Image (Kb) : 1072
     │ Img. Offset: 6582
     ┕━━━━━━━━━━━━┙
┍━━━━┯━━━━━━━━━━━━┑
│ #4 │ Size       : 48x48px
┕━━━━┥ N Colors   : 2
     │ Res. Byte  : 0
     │ X Hotspot  : 0
     │ Y Hotspot  : 2
     │ Image (Kb) : 816
     │ Img. Offset: 7654
     ┕━━━━━━━━━━━━┙
┍━━━━┯━━━━━━━━━━━━┑
│ #5 │ Size       : 32x32px
┕━━━━┥ N Colors   : 2
     │ Res. Byte  : 0
     │ X Hotspot  : 0
     │ Y Hotspot  : 1
     │ Image (Kb) : 304
     │ Img. Offset: 8470
     ┕━━━━━━━━━━━━┙
```
