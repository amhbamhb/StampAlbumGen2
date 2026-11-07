# AlbumEasy 4.5.0 stamp shapes reference fixture

`album.txt` is a frozen AlbumEasy 4.5.0 input. It exercises every documented
stamp shape in the current matrix, side-up/corner-up polygon orientation,
right-triangle options, flattened triangle apexes, a blank box, and modern and
legacy `STAMP_ADD` text forms. The only commands in the input are documented
AlbumEasy commands; no application-specific extensions are used.

The generated `album.pdf` is the output of the external AlbumEasy 4.5.0
reference renderer. This is a reference fixture, not evidence that StampAlbumGen2
supports these commands or that its native PDF output is equivalent.

The reference run and verified coverage limits are recorded in
[`docs/testing/ALBUMEASY_STAMP_COMMANDS_V1.md`](../../../../docs/testing/ALBUMEASY_STAMP_COMMANDS_V1.md).

Reproduce on Linux with the installed AlbumEasy 4.5.0 renderer and Xvfb:

```sh
cd tests/fixtures/albumeasy/stamp-shapes-v1
xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb \
  /path/to/AlbumEasy.sh -c album.txt
```

The verified run used the installed wrapper under a 60-second timeout; see the
execution report for the exact command and output evidence. The command writes
`album.pdf` in this fixture directory. Do not edit the frozen input after
reference acceptance; create a new fixture version for intentional changes.
