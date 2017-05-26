# Recholera

A simple and safe way to change colors

```
Recholera 0.1.0
Christian Dürr <contact@christianduerr.com>
A simple and safe way to change colors

USAGE:
    recholera [FLAGS] <FILE> <CURRENT_COLOR> <NEW_COLOR>

FLAGS:
    -h, --help       Prints help information
    -r, --revert     Revert all changes from backup
    -V, --version    Prints version information

ARGS:
    <FILE>             File which will be changed
    <CURRENT_COLOR>    The current color
    <NEW_COLOR>        The new color
```

Using `recholera <file> <current_color> <new_color>` (i.e.: `recholera demo.txt "#ff00ff" "#00ff00"`) you can replace every instance of one color with another. If this color doesn’t exist, or the target color already exists, the tool will warn you and exit without any changes.

Using `recholera --revert` will restore all files inside ./backup/ to their current state. If you wish to restore only a specific file or want to exclude files from being restored, you can either restore files manually or remove unneeded files from the ./backup/ directory.
