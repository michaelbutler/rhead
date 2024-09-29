# rhead

A `head` clone written in Rust.

"head" is a command line utility which gives you the first n lines (default 10) of STDIN or a text file.

# Usage Examples

```
# Handle text from Stdin
cat /path/to/file | rhead -n 20

# Handle text in a file
rhead -n 1 /etc/passwd
```

```
Usage: rhead [OPTIONS] [FILE]

If FILE is not present, it will read from STDIN.

Arguments:
  [FILE]  The file to read from (optional)

Options:
  -n, --number <NUMBER>  The number of lines to print [default: 10]
  -h, --help             Print help
  -V, --version          Print version
```